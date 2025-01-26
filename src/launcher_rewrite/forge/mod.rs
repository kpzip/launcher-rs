pub(crate) mod installer;

use std::fmt::format;
use std::sync::LazyLock;
use regex::Regex;
use reqwest::Url;
use crate::launcher_rewrite::installer::{ACCEPT_HEADER_NAME, APPLICATION_JSON, DEFAULT_DOWNLOADER_CLIENT};
use crate::launcher_rewrite::manifest::GAME_VERSION_MANIFEST;
use crate::launcher_rewrite::mod_loader_version_manifest::{ModLoaderLatestVersionData, ModLoaderVersionInfo, ModLoaderVersionType};
use crate::launcher_rewrite::profiles::ModLoader;

const FORGE_INDEX_URL_PREFIX: &str = "https://files.minecraftforge.net/net/minecraftforge/forge/index_";
const FORGE_INDEX_URL_SUFFIX: &str = ".html";

const FORGE_INSTALLER_URL_PREFIX: &str = "https://maven.minecraftforge.net/net/minecraftforge/forge/";
const FORGE_INSTALLER_URL_INFIX: &str = "/forge-";
const FORGE_INSTALLER_URL_SUFFIX: &str = "-installer.jar";


static TABLE_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"(?s)<table.*?class="download-list".*?>.*?</table>"#).expect("Failed to compile regex!"));
static TABLE_ENTRY_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"(?s)<tr.*?>.*?</tr>"#).expect("Failed to compile regex!"));

static VERSION_TABLE_ENTRY_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"(?s)<td.*?class="download-version".*?>.*?</td>"#).expect("Failed to compile regex!"));
static VERSION_NAME_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"\d+\.\d+\.\d+"#).expect("Failed to compile regex!"));

//static INSTALLER_TABLE_ENTRY_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"(?s)<a.*?>.*?Installer.*?</a>"#).expect("Failed to compile regex!"));
//static INSTALLER_LINK_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"(?s)https://.*?installer\.jar"#).expect("Failed to compile regex!"));


pub fn get_compatible_versions(game_version: &str) -> Vec<ModLoaderVersionInfo> {
    let game_version = GAME_VERSION_MANIFEST.sanitize_version_name(game_version, ModLoader::Forge);

    // We do a little web scraping
    let url = format!("{}{}{}", FORGE_INDEX_URL_PREFIX, game_version, FORGE_INDEX_URL_SUFFIX);
    // println!("Getting forge data...");
    if let Ok(response) = DEFAULT_DOWNLOADER_CLIENT.get(url).header(ACCEPT_HEADER_NAME, APPLICATION_JSON).send() {
        // println!("Got response {:?}", response);
        if let Ok(response_html_text) = response.text() {
            // println!("Response Text {}", response_html_text);
            if let Some(versions_table) = TABLE_REGEX.find(response_html_text.as_str()) {
                // println!("Versions Table {}", versions_table.as_str());
                let versions = TABLE_ENTRY_REGEX.find_iter(versions_table.as_str());
                let mapped = versions.filter_map(|m| {
                    let match_str = m.as_str();
                    let version_tag = VERSION_TABLE_ENTRY_REGEX.find(match_str)?;
                    let loader_version = VERSION_NAME_REGEX.find(version_tag.as_str())?.as_str();

                    //let installer_tag = INSTALLER_TABLE_ENTRY_REGEX.find(match_str)?;
                    //let installer_link = Url::parse(INSTALLER_LINK_REGEX.find(installer_tag.as_str())?.as_str()).ok()?;
                    let installer_link = Url::parse(format!("{}{}-{}{}{}-{}{}", FORGE_INSTALLER_URL_PREFIX, game_version, loader_version, FORGE_INSTALLER_URL_INFIX, game_version, loader_version, FORGE_INSTALLER_URL_SUFFIX).as_str()).ok()?;

                    Some(ModLoaderVersionInfo::new(loader_version.to_owned(), ModLoaderVersionType::Beta, installer_link, ModLoader::Forge))
                });
                let collected = mapped.collect();
                // println!("Scraped Forge Data: {:?}", collected);
                return collected;
            }
        }
    }
    return Vec::new()
}

pub fn get_latest_supported_game_version() -> ModLoaderLatestVersionData {
    // TODO web scraping
    return ModLoaderLatestVersionData::new("".to_owned(), "".to_owned())
}