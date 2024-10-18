use crate::authentication::account_data::AccountData;
use crate::launcher::asset_index::AssetIndex;
use crate::launcher::launch_properties::{LaunchProperties, Library};
use crate::launcher::util::sha1_of_file;
use crate::launcher::version::{Version, VersionManifest};
use serde_json::from_str;
use std::collections::HashMap;
use std::num::NonZeroU64;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, RwLock};
use std::{fs, io};
use crate::launcher::settings::profiles::ModLoader;

pub(crate) const MANIFEST_URL: &str = "https://launchermeta.mojang.com/mc/game/version_manifest_v2.json";

pub fn verify_and_download_if_needed(path: &Path, url: &str, sha1: Option<&str>, _size: Option<NonZeroU64>) {
    // TODO check size
    if path.try_exists().unwrap() {
        if let Some(sha1) = sha1 {
            let hash_string = sha1_of_file(path);
            //println!("File {:?} Exists and has hash of {}, while the reported hash is {}.", path, hash_string, sha1);
            if hash_string == sha1 {
                // File exists and hash matches
                //println!("Files have identical hashes. Skipping...");
                return;
            }
        }
    }
    // File either doesn't exist or has incorrect hash. Time to download!

    fs::create_dir_all(path.parent().unwrap()).unwrap();

    let data = reqwest::blocking::get(url).unwrap().bytes().unwrap();
    fs::write(path, data).unwrap();
}

pub fn get_assets_dir(launcher_dir: &Path) -> PathBuf {
    let mut assets_dir: PathBuf = launcher_dir.into();
    assets_dir.push("assets");
    assets_dir
}

pub fn get_indexes_dir(launcher_dir: &Path) -> PathBuf {
    let mut indexes_dir: PathBuf = get_assets_dir(launcher_dir);
    indexes_dir.push("indexes");
    indexes_dir
}

pub fn get_logging_config_dir(launcher_dir: &Path) -> PathBuf {
    let mut path = get_assets_dir(launcher_dir);
    path.push("log_configs");
    path
}

pub fn download_version_manifest_v2(launcher_main_path: &Path) {
    let mut path: PathBuf = launcher_main_path.into();
    path.push("versions");
    path.push("version_manifest_v2.json");
    verify_and_download_if_needed(path.as_path(), MANIFEST_URL, None, None);
}

pub fn get_version_manifest(launcher_main_path: &Path) -> VersionManifest {
    let mut manifest_path: PathBuf = launcher_main_path.into();
    manifest_path.push("versions");
    manifest_path.push("version_manifest_v2.json");
    let manifest_v2 = fs::read_to_string(manifest_path).unwrap();
    from_str(manifest_v2.as_str()).unwrap()
}

#[cfg(all(target_os = "windows", target_arch = "x86_64"))]
pub const NATIVE_JARS_SUFFIX: &str = "natives-windows.jar";
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub const NATIVE_JARS_SUFFIX: &str = "natives-windows-x86.jar";
#[cfg(all(target_os = "windows", target_arch = "aarch64"))]
pub const NATIVE_JARS_SUFFIX: &str = "natives-windows-arm64.jar";
#[cfg(all(target_os = "macos", target_arch = "x86_64"))]
pub const NATIVE_JARS_SUFFIX: &str = "natives-macos.jar";
#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
pub const NATIVE_JARS_SUFFIX: &str = "natives-macos-arm64.jar";
#[cfg(target_os = "linux")]
pub const NATIVE_JARS_SUFFIX: &str = "natives-linux.jar";

pub fn extract_if_needed(extracted_path: &Path, internal_path: &Path, jar_path: &Path, _hash: Option<&str>, _size: u64) {
    // TODO check existence and hash
    fs::create_dir_all(extracted_path.parent().unwrap()).unwrap();

    let jarfile = fs::File::open(jar_path).unwrap();
    let mut archive = zip::ZipArchive::new(jarfile).unwrap();
    let internal_name = internal_path.as_os_str().to_str().unwrap();
    println!("Extracting: {}", internal_name);
    let mut internal_file = match archive.by_name(internal_name) {
        Ok(s) => s,
        Err(e) => {
            println!("Could not open internal file {}, because of the error: {}", internal_name, e);
            return;
        }
    };
    let mut write_file = fs::File::open(extracted_path).unwrap();
    io::copy(&mut internal_file, &mut write_file).expect("TODO: panic message");
}

pub fn extract_dlls_from_jar(bin_path: &Path, jar_path: &Path, _overwrite: bool) {
    let jar_file = fs::File::open(jar_path).unwrap();
    let mut archive = zip::ZipArchive::new(jar_file).unwrap();
    let mut extracted_file_path: PathBuf = bin_path.into();

    let dll_names: Vec<String> = archive.file_names().filter(|n| n.ends_with(".dll")).map(String::from).collect();
    dll_names.iter().for_each(|internal_name| {
        let external_name = match internal_name.rsplit_once('/') {
            None => internal_name,
            Some((_, rhs)) => rhs,
        };
        extracted_file_path.push(external_name);
        //println!("Extracting dll file: {} to {}", internal_name, extracted_file_path.as_os_str().to_str().unwrap());
        let mut internal_file = archive.by_name(internal_name).unwrap();
        let mut external_file = fs::File::create(&extracted_file_path).unwrap();
        io::copy(&mut internal_file, &mut external_file).expect("TODO: panic message");
        extracted_file_path.pop();
    });
}

pub fn get_objects_path(launcher_path: &Path) -> PathBuf {
    let mut dir = get_assets_dir(launcher_path);
    dir.push("objects");
    dir
}

impl Version {
    // Installs the version, and populates the launch properties field
    pub fn install_and_get_launch_properties(&self, launcher_path: &Path, loader: ModLoader) -> LaunchProperties {
        // If launch properties is populated, everything has already been downloaded
        let launch_properties = self.launch_properties(loader);
        if let Some(properties) = launch_properties {
            return properties;
        }
        self.download_version_json(launcher_path, loader);

        let launch_properties = self.get_launch_properties(launcher_path, loader).propagate_inheritance(launcher_path);
        let assets_index = self.get_assets_index(launcher_path, &launch_properties);

        INSTALLED_VERSION_INFO.lock().unwrap().insert((self.id().into(), loader), (assets_index, launch_properties.clone()));

        self.download_main_jar(launcher_path, loader);
        self.download_jar_dependencies(launcher_path, loader);

        self.download_assets_index(launcher_path, loader);

        self.download_assets(launcher_path, loader);
        self.extract_natives(launcher_path);
        self.download_logging_config(launcher_path, loader);

        launch_properties
    }

    fn get_path(&self, launcher_main_path: &Path) -> PathBuf {
        let mut version_folder_path: PathBuf = launcher_main_path.into();
        version_folder_path.push("versions");
        version_folder_path.push(self.id());
        fs::create_dir_all(&version_folder_path).expect("bruh");
        version_folder_path
    }

    pub(crate) fn get_bin_path(&self, launcher_main_path: &Path) -> PathBuf {
        let mut path = self.get_path(launcher_main_path);
        path.push("bin");
        path
    }

    fn get_version_json_path(&self, launcher_main_path: &Path, loader: ModLoader) -> PathBuf {
        let mut path = self.get_path(launcher_main_path);
        path.push(format!("{}.json", loader.as_str_non_pretty()));
        path
    }

    fn download_version_json(&self, launcher_main_path: &Path, loader: ModLoader) {
        verify_and_download_if_needed(self.get_version_json_path(launcher_main_path, loader).as_path(), self.url(loader).as_str(), if loader == ModLoader::Vanilla { Some(self.sha1()) } else { None } /* TODO */, None);
    }

    fn get_launch_properties(&self, launcher_main_path: &Path, loader: ModLoader) -> LaunchProperties {
        //println!("Path: {:?}", self.get_version_json_path(launcher_main_path, loader));
        from_str(fs::read_to_string(self.get_version_json_path(launcher_main_path, loader)).unwrap().as_str()).unwrap()
    }

    fn get_version_jar_path(&self, launcher_main_path: &Path) -> PathBuf {
        let mut path = self.get_bin_path(launcher_main_path);
        path.push(format!("{}.jar", self.id()));
        path
    }

    fn download_main_jar(&self, launcher_main_path: &Path, loader: ModLoader) {
        let path = self.get_version_jar_path(launcher_main_path);
        let launch_properties = self.launch_properties(loader).unwrap();
        verify_and_download_if_needed(path.as_path(), launch_properties.downloads().client().url(), launch_properties.downloads().client().sha1(), launch_properties.downloads().client().size());
    }

    fn download_jar_dependencies(&self, launcher_main_path: &Path, loader: ModLoader) {
        let mut path = self.get_bin_path(launcher_main_path);
        self.launch_properties(loader).unwrap().libraries().iter().filter(|arg| Library::rules_match(*arg)).for_each(|lib| {
            let name = lib.filename();
            //println!("Name: {:#?}", name);
            path.push(name);
            verify_and_download_if_needed(path.as_path(), lib.get_url().as_str(), lib.sha1(), lib.size());
            //println!("Path: {:#?}", path);
            path.pop();
        });
    }

    fn download_assets_index(&self, launcher_dir: &Path, loader: ModLoader) {
        let mut index_path = get_indexes_dir(launcher_dir);
        let binding = self.launch_properties(loader).unwrap();
        let assets_index_info = binding.asset_index();
        index_path.push(assets_index_info.id());
        index_path.set_extension("json");
        verify_and_download_if_needed(index_path.as_path(), assets_index_info.url(), assets_index_info.sha1(), assets_index_info.size());
    }

    fn get_assets_index(&self, launcher_dir: &Path, launch_properties: &LaunchProperties) -> AssetIndex {
        let mut index_path = get_indexes_dir(launcher_dir);
        let assets_index_info = launch_properties.asset_index();
        index_path.push(assets_index_info.id());
        index_path.set_extension("json");
        from_str(fs::read_to_string(index_path).unwrap().as_str()).unwrap()
    }

    fn download_assets(&self, launcher_dir: &Path, loader: ModLoader) {
        let objects_path = get_objects_path(launcher_dir);
        self.assets_index(loader).unwrap().iter().for_each(|(_, data)| {
            let mut path = objects_path.clone();

            let folder_name = &data.hash()[..2];
            let file_name = data.hash();

            // Object folder begins with first 2 digits of hash
            path.push(folder_name);
            // File name == File hash
            path.push(file_name);

            verify_and_download_if_needed(&path, format!("https://resources.download.minecraft.net/{}/{}", folder_name, file_name).as_str(), Some(data.hash()), NonZeroU64::new(data.size()));
        })
    }

    fn extract_natives(&self, launcher_dir: &Path) {
        let bin_path = self.get_bin_path(launcher_dir);
        fs::read_dir(&bin_path).unwrap().filter(|f| f.as_ref().unwrap().file_name().to_str().unwrap().ends_with(NATIVE_JARS_SUFFIX)).for_each(|f| {
            extract_dlls_from_jar(&bin_path, &f.as_ref().unwrap().path(), false);
        });
    }

    pub fn get_logging_config_file_path(&self, launcher_dir: &Path, loader: ModLoader) -> PathBuf {
        let mut path = get_logging_config_dir(launcher_dir);
        path.push(self.launch_properties(loader).unwrap().logging().id());
        path
    }

    fn download_logging_config(&self, launcher_dir: &Path, loader: ModLoader) {
        let binding = self.launch_properties(loader).unwrap();
        let logging = binding.logging();
        let path = self.get_logging_config_file_path(launcher_dir, loader);
        verify_and_download_if_needed(&path, logging.url(), logging.sha1(), logging.size());
    }
}

pub fn get_account_data_dir(launcher_dir: &Path) -> PathBuf {
    let mut path: PathBuf = launcher_dir.into();
    path.push("tokens");
    path.set_extension("json");
    path
}

pub fn get_account_data(launcher_dir: &Path) -> AccountData {
    let path = get_account_data_dir(launcher_dir);
    let string = fs::read_to_string(path).unwrap();
    from_str(string.as_str()).unwrap()
}

pub fn save_account_data(launcher_dir: &Path, acc: &AccountData) {
    let path = get_account_data_dir(launcher_dir);
    let string = serde_json::to_string_pretty(acc).unwrap();
    fs::write(path, string.as_str()).expect("TODO: panic message");
}
