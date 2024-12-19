use std::io::Read;
use iced::widget::markdown::Url;
use serde::Deserialize;
use crate::launcher_rewrite::installer::DEFAULT_DOWNLOADER_CLIENT;
use crate::launcher_rewrite::manifest::GAME_VERSION_MANIFEST;
use crate::launcher_rewrite::mod_loader_version_manifest::{ModLoaderVersionInfo, ModLoaderVersionType};
use crate::launcher_rewrite::profiles::ModLoader;

const FABRIC_VERSIONS_URL: &'static str = "https://meta.fabricmc.net/v2/versions/loader/";
const PROFILE_JSON_PATH: &'static str = "/profile/json";

type FabricCompatibleVersionsResponse = Vec<FabricCompatibleVersionInfo>;

#[derive(Debug, Deserialize)]
pub struct FabricCompatibleVersionInfo {
    #[serde(rename = "loader")]
    loader_info: LoaderInfo,
}

#[derive(Debug, Deserialize)]
pub struct LoaderInfo {
    build: usize,
    version: String,
    stable: bool,
}

impl ModLoaderVersionInfo {
    fn from_deserialized(value: FabricCompatibleVersionInfo, game_version: &str) -> Self {
        let url = format!("{}{}/{}{}", FABRIC_VERSIONS_URL, game_version, value.loader_info.version.as_str(), PROFILE_JSON_PATH);
        ModLoaderVersionInfo::new(value.loader_info.version, value.loader_info.stable.into(), Url::parse(&url).expect("Failed to parse URL!"), ModLoader::Fabric)
    }
}

pub fn get_compatible_versions(game_version: &str) -> Vec<ModLoaderVersionInfo> {
    // Sanitize just in case
    // Can probably be commented out
    let game_version = GAME_VERSION_MANIFEST.sanitize_version_name(game_version);
    let url = format!("{}{}", FABRIC_VERSIONS_URL, game_version);
    if let Ok(response_json) = DEFAULT_DOWNLOADER_CLIENT.get(url).send() {
        // println!("Sent Request! Response: {:?}", response_json.text());
        if let Ok(deserialized_vec) = serde_json::from_reader::<_, FabricCompatibleVersionsResponse>(response_json) {
            // println!("Deserialized: {:?}", deserialized_vec);
            let converted = deserialized_vec.into_iter().map(|vi| ModLoaderVersionInfo::from_deserialized(vi, game_version)).collect();
            return converted
        }
    }
    return Vec::new()
}
