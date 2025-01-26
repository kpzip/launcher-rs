use std::fmt::format;
use std::{fs, io};
use std::fs::File;
use std::num::NonZeroU64;
use std::path::{Path, PathBuf};
use std::process::Command;
use const_format::concatcp;
use reqwest::Url;
use crate::launcher_rewrite::error::LauncherError;
use crate::launcher_rewrite::jar_utils::extractor::extract_if_needed;
use crate::launcher_rewrite::installer::Downloadable;
use crate::launcher_rewrite::manifest::GAME_VERSION_MANIFEST;
use crate::launcher_rewrite::mod_loader_version_manifest::ModLoaderVersionInfo;
use crate::launcher_rewrite::path_handler::{DUMMY_INSTALL_DIR_NAME, DUMMY_LAUNCHER_PROFILES_JSON_NAME, get_bin_path, get_vanilla_client_json_path, PATH_SEP, temp_file_path};
use crate::launcher_rewrite::profiles::ModLoader;
use crate::launcher_rewrite::util::hash::FileHash;

const CLIENT_JSON_INTERNAL_PATH: &str = "version.json";

struct ForgeJarDownloadable<'a> {
    loader_info: &'a ModLoaderVersionInfo,
    file_path: &'a Path,
}

impl Downloadable for ForgeJarDownloadable<'_> {
    fn get_download_url(&self) -> &Url {
        self.loader_info.get_download_url()
    }

    fn get_file_path(&self, game_version: &str) -> PathBuf {
        self.file_path.to_owned()
    }

    fn get_hash(&self) -> Option<FileHash> {
        None
    }

    fn get_size(&self) -> Option<NonZeroU64> {
        None
    }
}

pub fn download(loader_info: &ModLoaderVersionInfo, game_version: &str) -> Result<(), LauncherError> {

    let loader_version = loader_info.version_name();
    let game_version = GAME_VERSION_MANIFEST.sanitize_version_name(game_version, ModLoader::Forge);

    // Download installer jar and extract version json and version jar

    // Paths
    let client_jar_name = format!("forge-{}-{}-client.jar", game_version, loader_version);
    let temp_path = temp_file_path(format!("forge-{}-{}.jar.tmp", game_version, loader_version).as_str());
    let dummy_profiles_json_path = temp_file_path(concatcp!(DUMMY_INSTALL_DIR_NAME, PATH_SEP, DUMMY_LAUNCHER_PROFILES_JSON_NAME));
    let install_dir = dummy_profiles_json_path.parent().unwrap();
    let forge_client_path = {
        let mut p = get_bin_path(game_version);
        p.push(client_jar_name.as_str());
        p
    };
    let generated_client_path = {
        let mut p = install_dir.to_owned();
        p.push("libraries");
        p.push("net");
        p.push("minecraftforge");
        p.push("forge");
        p.push(format!("{}-{}", game_version, loader_version));
        p.push(client_jar_name.as_str());
        p
    };
    let forge_shim_path = {
        let mut p = get_bin_path(game_version);
        p.push(format!("forge-{}-{}-shim.jar", game_version, loader_version));
        p
    };
    if let Some(p) = temp_path.parent() { if let Err(e) = fs::create_dir_all(p) { eprintln!("Error creating directory! {}", e); return Err(e.into()) } };

    let client_json_internal_path = Path::new(CLIENT_JSON_INTERNAL_PATH);
    let client_json_external_path = get_vanilla_client_json_path(game_version, ModLoader::Forge, loader_info.version_name());

    // Installer Jar
    let downloadable = ForgeJarDownloadable { loader_info, file_path: temp_path.as_path() };
    downloadable.download(game_version)?;

    // Extract client json
    extract_if_needed(client_json_external_path.as_path(), client_json_internal_path, temp_path.as_path())?;

    // Patch forge client jar ourselves
    // TODO

    // Just run the forge installer so that way we don't have to patch the jar ourselves (For now)
    // Fake it till you make it. -Kyle Schmerge 2024
    fs::write(dummy_profiles_json_path.as_path(), [])?;
    let _ = Command::new("java").current_dir(temp_path.as_path()).args(["-jar", temp_path.to_string_lossy().as_ref(), "--installClient", install_dir.to_string_lossy().as_ref()]).output();
    //println!("path: {:?}", generated_client_path.as_path());
    let mut generated = File::open(generated_client_path.as_path())?;
    let mut bin_file = File::create(forge_client_path.as_path())?;
    io::copy(&mut generated, &mut bin_file)?;
    Ok(())
}