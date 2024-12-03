use std::io::Read;
use serde::Deserialize;
use crate::launcher_rewrite::installer::DEFAULT_DOWNLOADER_CLIENT;
use crate::launcher_rewrite::manifest::GAME_VERSION_MANIFEST;
use crate::launcher_rewrite::mod_loader_version_manifest::{ModLoaderVersionInfo, ModLoaderVersionType};

const FABRIC_COMPATIBLE_VERSIONS_URL: &'static str = "https://meta.fabricmc.net/versions/loader/";

type FabricCompatibleVersionsResponse = Vec<FabricCompatibleVersionInfo>;

#[derive(Debug, Deserialize)]
pub struct FabricCompatibleVersionInfo {
    build: usize,
    version: String,
    stable: bool,
}

impl From<FabricCompatibleVersionInfo> for ModLoaderVersionInfo {
    fn from(value: FabricCompatibleVersionInfo) -> Self {
        ModLoaderVersionInfo::new(value.version, value.stable.into(), "TODO".to_owned())
    }
}

pub fn get_compatible_versions(mc_version: &str) -> Vec<ModLoaderVersionInfo> {
    // Sanitize just in case
    // Can probably be commented out
    let mc_version = GAME_VERSION_MANIFEST.sanitize_version_name(mc_version);
    let url = format!("{}{}", FABRIC_COMPATIBLE_VERSIONS_URL, mc_version);
    if let Ok(response_json) = DEFAULT_DOWNLOADER_CLIENT.get(url).send() {
        if let Ok(deserialized_vec) = serde_json::from_reader::<_, FabricCompatibleVersionsResponse>(response_json) {
            let converted = deserialized_vec.into_iter().map(Into::into).collect();
            return converted
        }
    }
    return Vec::new()
}
