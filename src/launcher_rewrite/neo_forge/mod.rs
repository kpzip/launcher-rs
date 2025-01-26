pub(crate) mod installer;

use std::fmt::format;
use std::sync::LazyLock;
use regex::Regex;
use reqwest::Url;
use crate::launcher_rewrite::installer::{ACCEPT_HEADER_NAME, APPLICATION_JSON, DEFAULT_DOWNLOADER_CLIENT};
use crate::launcher_rewrite::manifest::GAME_VERSION_MANIFEST;
use crate::launcher_rewrite::mod_loader_version_manifest::{ModLoaderLatestVersionData, ModLoaderVersionInfo};
use crate::launcher_rewrite::profiles::ModLoader;

const NEO_FORGE_INDEX_URL: &str = "https://maven.neoforged.net/releases/net/neoforged/neoforge/";
const NEO_FORGE_INSTALLER_PREFIX: &str = "/neoforge-";
const NEO_FORGE_INSTALLER_SUFFIX: &str = "-installer.jar";

static LIST_ELEMENT_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"(?s)<li.*?class="directory".*?>.*?</li>"#).expect("Failed to compile regex!"));

static LOADER_VERSION_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"(?s)\d+\.\d+\.\d+(-beta)?"#).expect("Failed to compile regex!"));

pub fn get_compatible_versions(game_version: &str) -> Vec<ModLoaderVersionInfo> {
    let game_version = GAME_VERSION_MANIFEST.sanitize_version_name(game_version);
    let truncated_game_version = game_version.split_once('.').expect("invalid version name").1;

    // We do a little web scraping
    // TODO find a way to cache this if possible
    let resp = DEFAULT_DOWNLOADER_CLIENT.get(NEO_FORGE_INDEX_URL).send().expect("web request failed").text().expect("expected text");
    let matches = LIST_ELEMENT_REGEX.find_iter(resp.as_str()).filter_map(|li| LOADER_VERSION_REGEX.find(li.as_str()).map(|m| m.as_str()));
    let filtered_by_game_version = matches.filter(|s| s.starts_with(truncated_game_version));
    filtered_by_game_version.map(|n| {
        let loader_version = n.to_owned();
        let is_beta = n.ends_with("-beta");
        let url = Url::parse(format!("{}{}/{}{}{}", NEO_FORGE_INDEX_URL, n, NEO_FORGE_INSTALLER_PREFIX, n, NEO_FORGE_INSTALLER_SUFFIX).as_str()).expect("Failed to parse URL!");
        ModLoaderVersionInfo::new(loader_version, is_beta.into(), url, ModLoader::NeoForge)
    }).collect()
}

pub fn get_latest_supported_game_version() -> ModLoaderLatestVersionData {
    // TODO web scraping
    return ModLoaderLatestVersionData::new("".to_owned(), "".to_owned())
}