use std::fmt::format;
use std::num::NonZeroU64;
use std::path::{Path, PathBuf};
use iced::widget::markdown::Url;
use crate::launcher_rewrite::jar_utils::extractor::extract_if_needed;
use crate::launcher_rewrite::installer::Downloadable;
use crate::launcher_rewrite::mod_loader_version_manifest::ModLoaderVersionInfo;
use crate::launcher_rewrite::path_handler::{get_bin_path, get_vanilla_client_json_path, temp_file_path};
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

pub fn download(loader_info: &ModLoaderVersionInfo, game_version: &str) {

    let loader_version = loader_info.version_name();

    // Download installer jar and extract version json and version jar

    // Paths
    let temp_path = temp_file_path(format!("forge-{}-{}.jar.tmp", game_version, loader_version).as_str());
    let forge_client_path = {
        let mut p = get_bin_path(game_version);
        p.push(format!("forge-{}-{}-client.jar", game_version, loader_version));
        p
    };
    let forge_shim_path = {
        let mut p = get_bin_path(game_version);
        p.push(format!("forge-{}-{}-shim.jar", game_version, loader_version));
        p
    };

    let client_json_internal_path = Path::new(CLIENT_JSON_INTERNAL_PATH);
    let client_json_external_path = get_vanilla_client_json_path(game_version, ModLoader::Forge, loader_info.version_name());

    // Installer Jar
    let downloadable = ForgeJarDownloadable { loader_info, file_path: temp_path.as_path() };
    downloadable.download(game_version);

    // Extract client json
    extract_if_needed(client_json_external_path.as_path(), client_json_internal_path, temp_path.as_path());

    // Patch forge client jar
    // TODO
}