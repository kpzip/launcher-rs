use std::{fs, io};
use std::fs::File;
use std::num::NonZeroU64;
use std::path::{Path, PathBuf};
use std::process::Command;
use const_format::concatcp;
use reqwest::Url;
use crate::launcher_rewrite::installer::Downloadable;
use crate::launcher_rewrite::jar_utils::extractor::extract_if_needed;
use crate::launcher_rewrite::manifest::GAME_VERSION_MANIFEST;
use crate::launcher_rewrite::mod_loader_version_manifest::ModLoaderVersionInfo;
use crate::launcher_rewrite::path_handler::{get_bin_path, get_vanilla_client_json_path, temp_file_path};
use crate::launcher_rewrite::profiles::ModLoader;
use crate::launcher_rewrite::util::hash::FileHash;

const CLIENT_JSON_INTERNAL_PATH: &str = "version.json";

// TODO checksum / hash
struct NeoForgeJarDownloadable<'a> {
    loader_info: &'a ModLoaderVersionInfo,
    file_path: &'a Path,
}

impl Downloadable for NeoForgeJarDownloadable<'_> {
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
    let game_version = GAME_VERSION_MANIFEST.sanitize_version_name(game_version);

    // Download installer jar and extract version json and version jar

    // Paths
    let temp_path = temp_file_path(format!("neoforge-{}-{}.jar.tmp", game_version, loader_version).as_str());
    if let Some(p) = temp_path.parent() { if let Err(e) = fs::create_dir_all(p) { eprintln!("Error creating directory! {}", e) } };

    let client_json_internal_path = Path::new(CLIENT_JSON_INTERNAL_PATH);
    let client_json_external_path = get_vanilla_client_json_path(game_version, ModLoader::NeoForge, loader_info.version_name());

    // Installer Jar
    let downloadable = NeoForgeJarDownloadable { loader_info, file_path: temp_path.as_path() };
    downloadable.download(game_version);

    // Extract client json
    extract_if_needed(client_json_external_path.as_path(), client_json_internal_path, temp_path.as_path());

}