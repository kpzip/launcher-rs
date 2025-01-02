use std::fs;
use std::sync::{LazyLock, RwLock};

use serde::{Deserialize, Serialize, Serializer};
use serde::ser::{SerializeSeq, SerializeStruct};

use crate::launcher_rewrite::path_handler::INSTALLED_VERSIONS_FILE_PATH;
use crate::launcher_rewrite::profiles::ModLoader;
use crate::launcher_rewrite::util::config_file::{load_from_file, save_to_file};
use crate::util::option_comparison;

pub static INSTALLED_VERSIONS: LazyLock<RwLock<InstalledVersions>> = LazyLock::new(|| RwLock::new(get_installed_versions()));

fn get_installed_versions() -> InstalledVersions {
    load_from_file(INSTALLED_VERSIONS_FILE_PATH.as_path(), true)
}

pub fn save_installed_versions() {
    save_to_file(&*INSTALLED_VERSIONS.read().unwrap(), INSTALLED_VERSIONS_FILE_PATH.as_path(), true);
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct InstalledVersionInfo  {
    pub game_version: String,
    pub loader: ModLoader,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub loader_version: Option<String>
}

impl InstalledVersionInfo {
    fn new(game_version: String, loader: ModLoader, loader_version: Option<String>) -> Self {
        Self { game_version, loader, loader_version }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InstalledVersions {
    #[serde(skip_deserializing, default = "comment", rename = "__comment")]
    comment: &'static str,
    installed: Vec<InstalledVersionInfo>,
}

impl InstalledVersions {

    fn new() -> Self {
        Self::default()
    }

    pub fn contains(&self, version: &str, loader: ModLoader, loader_version: Option<&str>) -> bool {
        self.installed.iter().find(|inf| loader == inf.loader && version == inf.game_version && option_comparison(inf.loader_version.as_ref(), loader_version)).is_some()
    }

    pub fn add(&mut self, version: &str, loader: ModLoader, loader_version: Option<&str>) {
        self.installed.push(InstalledVersionInfo::new(version.to_owned(), loader, loader_version.map(|s| s.to_owned())));
    }

}

impl Default for InstalledVersions {
    fn default() -> Self {
        Self {
            comment: comment(),
            installed: Vec::new(),
        }
    }
}

fn comment() -> &'static str {
    "be advised that modifying this file may break the launcher and require you to manually verify game files!"
}