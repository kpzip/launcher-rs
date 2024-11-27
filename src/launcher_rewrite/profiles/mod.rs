use std::fmt::{Display, Formatter};
use std::fs;
use std::ops::Deref;
use std::sync::{LazyLock, RwLock};

use base64::Engine;
use rand::random;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::{Error, Visitor};
use crate::launcher_rewrite::authentication::LOGGED_IN_ACCOUNT_DATA;
use crate::launcher_rewrite::manifest::GameVersionManifest;
use crate::launcher_rewrite::mod_loader_version_manifest::{FABRIC_MANIFEST, FORGE_MANIFEST, ModLoaderVersionManifest, NEO_FORGE_MANIFEST, QUILT_MANIFEST};
use crate::launcher_rewrite::path_handler::{LAUNCHER_CFG_PATH, TOKENS_FILE_PATH};
use crate::launcher_rewrite::profiles::icon::LauncherProfileIcon;
use crate::launcher_rewrite::util::config_file::{load_from_file, save_to_file};

pub mod icon;

pub static PROFILES: LazyLock<RwLock<LauncherProfiles>> = LazyLock::new(|| RwLock::new(init_launcher_profiles()));

fn init_launcher_profiles() -> LauncherProfiles {
    load_from_file(LAUNCHER_CFG_PATH.as_path(), true)
}

pub fn save_launcher_profiles() {
    save_to_file(&*PROFILES.read().unwrap(), LAUNCHER_CFG_PATH.as_path(), true);
}

const fn memory_default() -> u16 {
    2
}

fn memory_is_default(mem: &u16) -> bool {
    *mem == memory_default()
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LauncherProfiles {
    je_client_profiles: Vec<LauncherProfile>,
    settings: LauncherPersistentState,
}

impl LauncherProfiles {
    pub fn je_client_profiles(&self) -> &Vec<LauncherProfile> {
        &self.je_client_profiles
    }

    pub fn je_client_profiles_mut(&mut self) -> &mut Vec<LauncherProfile> {
        &mut self.je_client_profiles
    }

    pub fn find_profile(&self, id: u128) -> Option<&LauncherProfile> {
        self.je_client_profiles.iter().find(|profile| profile.id() == id)
    }

    pub fn settings(&self) -> LauncherPersistentState {
        self.settings
    }

    pub fn settings_mut(&mut self) -> &mut LauncherPersistentState {
        &mut self.settings
    }
}

impl Default for LauncherProfiles {
    fn default() -> Self {
        Self {
            je_client_profiles: vec![
                LauncherProfile::new("Latest Release".to_owned(), ModLoader::Vanilla, "latest-release".to_owned(), LauncherProfileIcon::Grass),
                LauncherProfile::new("Latest Snapshot".to_owned(), ModLoader::Vanilla, "latest-snapshot".to_owned(), LauncherProfileIcon::CraftingTable),
                LauncherProfile::new("Fabric".to_owned(), ModLoader::Fabric, "latest-release".to_owned(), LauncherProfileIcon::Bookshelf),
            ],
            settings: LauncherPersistentState::new(1),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LauncherProfile {
    name: String,
    #[serde(rename = "id")]
    uuid: u128,
    mod_loader: ModLoader,
    // INVARIANCE: Should always be a valid version for the selected mod loader and should be an empty string when mod_loader is VANILLA. Can also be latest-stable or latest-beta if the specified minecraft version has such a mod loader version
    mod_loader_version: String,
    version_name: String,
    mc_directory: String,
    icon: LauncherProfileIcon,
    #[serde(skip_serializing_if = "Option::is_none")]
    additional_args: Option<String>,
    // in GB
    #[serde(default = "memory_default")]
    #[serde(skip_serializing_if = "memory_is_default")]
    memory: u16,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    width: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    height: Option<u32>,
}

impl LauncherProfile {
    pub fn new(name: String, mod_loader: ModLoader, version_name: String, icon: LauncherProfileIcon) -> Self {
        Self {
            name,
            mod_loader,
            version_name,
            icon,
            ..Default::default()
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn mod_loader(&self) -> ModLoader {
        self.mod_loader
    }

    pub fn version_name(&self) -> &str {
        &self.version_name
    }

    pub fn mc_directory(&self) -> &str {
        &self.mc_directory
    }

    pub fn icon(&self) -> &LauncherProfileIcon {
        &self.icon
    }

    pub fn additional_args(&self) -> &Option<String> {
        &self.additional_args
    }

    pub fn memory(&self) -> u16 {
        self.memory
    }

    pub fn width(&self) -> Option<u32> {
        self.width
    }

    pub fn height(&self) -> Option<u32> {
        self.height
    }

    pub fn id(&self) -> u128 {
        self.uuid
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_id(&mut self, id: u128) {
        self.uuid = id;
    }

    pub fn set_mod_loader(&mut self, mod_loader: ModLoader) {
        self.mod_loader = mod_loader;
    }

    pub fn set_version_name(&mut self, version_name: String) {
        self.version_name = version_name;
    }

    pub fn set_mc_directory(&mut self, mc_directory: String) {
        self.mc_directory = mc_directory;
    }

    pub fn set_icon(&mut self, icon: LauncherProfileIcon) {
        self.icon = icon;
    }

    pub fn set_additional_args(&mut self, additional_args: Option<String>) {
        self.additional_args = additional_args;
    }

    pub fn set_memory(&mut self, memory: u16) {
        self.memory = memory;
    }

    pub fn set_width(&mut self, width: Option<u32>) {
        self.width = width;
    }

    pub fn set_height(&mut self, height: Option<u32>) {
        self.height = height;
    }

    pub fn mod_loader_version(&self) -> &str {
        &self.mod_loader_version
    }

    pub fn set_mod_loader_version(&mut self, mod_loader_version: String) {
        self.mod_loader_version = mod_loader_version;
    }
}

impl Default for LauncherProfile {
    fn default() -> Self {
        Self {
            name: "Unnamed Profile".to_string(),
            uuid: random(),
            mod_loader: Default::default(),
            mod_loader_version: String::new(),
            version_name: "latest-release".to_string(),
            mc_directory: "%appdata%/.minecraft/".to_string(), // TODO default path is OS dependent
            icon: Default::default(),
            additional_args: None,
            memory: memory_default(),
            width: None,
            height: None,
        }
    }
}

pub fn convert_width_height(width: Option<u32>, height: Option<u32>) -> Option<(u32, u32)> {
    Some((width?, height?))
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Hash, Clone, Copy, Default)]
#[serde(rename_all = "snake_case")]
pub enum ModLoader {
    #[default]
    Vanilla,
    Fabric,
    Quilt,
    Forge,
    NeoForge,
}

pub fn fabric_version() -> String {
    "0.16.2".into() // TODO
}

impl ModLoader {

    pub fn get_manifest(&self) -> Option<&ModLoaderVersionManifest> {
        match self {
            ModLoader::Vanilla => None,
            ModLoader::Fabric => Some(&FABRIC_MANIFEST),
            ModLoader::Quilt => Some(&QUILT_MANIFEST),
            ModLoader::Forge => Some(&FORGE_MANIFEST),
            ModLoader::NeoForge => Some(&NEO_FORGE_MANIFEST),
        }
    }

    pub fn as_str_non_pretty(&self) -> &'static str {
        match self {
            ModLoader::Vanilla => "vanilla",
            ModLoader::Fabric => "fabric",
            ModLoader::Quilt => "quilt",
            ModLoader::Forge => "forge",
            ModLoader::NeoForge => "neo_forge",
        }
    }

}

impl Display for ModLoader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            ModLoader::Vanilla => "Vanilla",
            ModLoader::Fabric => "Fabric",
            ModLoader::Quilt => "Quilt",
            ModLoader::Forge => "Forge",
            ModLoader::NeoForge => "NeoForge",
        };
        write!(f, "{}", str)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct LauncherPersistentState {
    #[serde(flatten)]
    settings: LauncherSettings,
    selected_profile_id: u128,
}

impl Deref for LauncherPersistentState {
    type Target = LauncherSettings;

    fn deref(&self) -> &Self::Target {
        &self.settings
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(default)]
pub struct LauncherSettings {
    pub enable_historical: bool,
    pub enable_snapshots: bool,
    pub keep_launcher_open: bool,
    pub re_open_launcher: bool,
}

impl Default for LauncherSettings {
    fn default() -> Self {
        Self {
            enable_historical: false,
            enable_snapshots: true,
            keep_launcher_open: false,
            re_open_launcher: false,
        }
    }
}

impl LauncherPersistentState {

    pub fn new(selected_profile_id: u128) -> Self {
        Self {
            settings: Default::default(),
            selected_profile_id,
        }
    }

    pub fn set_settings(&mut self, settings: LauncherSettings) {
        self.settings = settings;
    }

    pub fn selected_profile_id(&self) -> u128 {
        self.selected_profile_id
    }
}

