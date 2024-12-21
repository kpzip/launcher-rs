use std::borrow::Cow;
use std::collections::HashMap;
use std::num::NonZeroU64;
use std::path::PathBuf;
use std::sync::{Arc, LazyLock, Mutex, OnceLock};
use iced::widget::markdown::Url;
use crate::launcher_rewrite::{fabric, forge, neo_forge, quilt};
use crate::launcher_rewrite::installer::Downloadable;
use crate::launcher_rewrite::path_handler::get_vanilla_client_json_path;
use crate::launcher_rewrite::profiles::ModLoader;
use crate::launcher_rewrite::util::hash::FileHash;

pub static FABRIC_MANIFEST: LazyLock<ModLoaderVersionManifest> = LazyLock::new(|| ModLoaderVersionManifest::new(ModLoader::Fabric, fabric::get_compatible_versions));
pub static QUILT_MANIFEST: LazyLock<ModLoaderVersionManifest> = LazyLock::new(|| ModLoaderVersionManifest::new(ModLoader::Quilt, quilt::get_compatible_versions));
pub static FORGE_MANIFEST: LazyLock<ModLoaderVersionManifest> = LazyLock::new(|| ModLoaderVersionManifest::new(ModLoader::Forge, forge::get_compatible_versions));
pub static NEO_FORGE_MANIFEST: LazyLock<ModLoaderVersionManifest> = LazyLock::new(|| ModLoaderVersionManifest::new(ModLoader::NeoForge, neo_forge::get_compatible_versions));

#[derive(Debug, Default, Clone)]
pub struct ModLoaderVersionManifest {
    loader: ModLoader,
    versions_map: ModLoaderVersionMap,
}

impl ModLoaderVersionManifest {

    pub fn get_loader_versions(&self, game_version_name: &str) -> Arc<[ModLoaderVersionInfo]> {
        self.versions_map.get(game_version_name)
    }

    pub fn has_loader_for_game_version(&self, game_version_name: &str) -> bool {
        self.versions_map.contains(game_version_name)
    }

    pub fn has_stable_loader_version_for_game_version(&self, game_version_name: &str) -> bool {
        self.versions_map.get(game_version_name).iter().filter(|loader_version| loader_version.is_stable()).next().is_some()
    }

    pub fn contains(&self, game_version_name: &str, loader_version_name: &str) -> bool {
        match loader_version_name {
            "latest-stable" => {
                self.has_stable_loader_version_for_game_version(game_version_name)
            },
            "latest-beta" => {
                self.get_loader_versions(game_version_name).is_empty()
            }
            loader_version_name => {
                self.get_loader_versions(game_version_name).iter().find(|e| e.version_name == loader_version_name).is_some()
            }
        }
    }

    pub fn sanitize_loader_version_name<'a>(&'a self, game_version_name: &str, loader_version_name: &'a str) -> Cow<'a, str> {
        match loader_version_name {
            "latest-stable" => {
                if let Some(v) = self.get_loader_versions(game_version_name).iter().find(|v| v.is_stable()) {
                    v.version_name.clone().into()
                }
                else {
                    panic!("No stable loader version found for loader `{:?}` for game version `{}`.", self.loader, game_version_name);
                }
            },
            "latest-beta" => {
                if let Some(v) = self.get_loader_versions(game_version_name).get(0) {
                    v.version_name.clone().into()
                }
                else {
                    panic!("No loader version found for loader `{:?}` for game version `{}`.", self.loader, game_version_name);
                }
            }
            n => {
                n.into()
            }
        }
    }

    // TODO find a way to do this without cloning?
    pub fn get_loader_version_info(&self, game_version: &str, loader_version: &str) -> ModLoaderVersionInfo {
        let loader_version = self.sanitize_loader_version_name(game_version, loader_version);
        if let Some(ver) = self.get_loader_versions(game_version).iter().find(|v| v.version_name == loader_version) {
            ver.clone()
        }
        else {
            panic!("Unable to find loader version `{}` for mod loader `{:?}` and game version `{}`", loader_version, self.loader, game_version)
        }
    }

    pub fn new(loader: ModLoader, version_func: fn(&str) -> Vec<ModLoaderVersionInfo>) -> Self {
        Self { loader, versions_map: ModLoaderVersionMap::new(version_func, Mutex::new(HashMap::new())) }
    }
}

#[derive(Debug, Clone)]
pub struct ModLoaderVersionInfo {
    version_name: String,
    version_type: ModLoaderVersionType,
    version_client_url: Url,
    // Annoying that we have to store this
    loader: ModLoader,
}

impl Default for ModLoaderVersionInfo {
    fn default() -> Self {
        Self {
            version_name: Default::default(),
            version_type: Default::default(),
            version_client_url: Url::parse("about:blank").expect("Invalid default url"),
            loader: Default::default(),
        }
    }
}

impl ModLoaderVersionInfo {

    pub fn is_stable(&self) -> bool {
        return self.version_type == ModLoaderVersionType::Stable
    }

    pub fn version_name(&self) -> &str {
        &self.version_name
    }

    pub fn version_type(&self) -> ModLoaderVersionType {
        self.version_type
    }

    pub fn version_client_url(&self) -> &Url {
        &self.version_client_url
    }

    pub fn new(version_name: String, version_type: ModLoaderVersionType, version_client_url: Url, loader: ModLoader) -> Self {
        Self { version_name, version_type, version_client_url, loader }
    }
}

impl Downloadable for ModLoaderVersionInfo {
    fn get_download_url(&self) -> &Url {
        &self.version_client_url
    }

    fn get_file_path(&self, version_name: &str) -> PathBuf {
        get_vanilla_client_json_path(version_name, self.loader, self.version_name.as_str())
    }

    fn get_hash(&self) -> Option<FileHash> {
        None
    }

    fn get_size(&self) -> Option<NonZeroU64> {
        None
    }

    fn requires_custom_download_fn(&self) -> bool {
        match self.loader {
            ModLoader::Vanilla => false,
            ModLoader::Fabric => false,
            ModLoader::Quilt => false,
            ModLoader::Forge => true,
            ModLoader::NeoForge => true,
        }
    }

    fn custom_download_fn(&self, game_version: &str) {
        match self.loader {
            ModLoader::Vanilla => unreachable!(),
            ModLoader::Fabric => unreachable!(),
            ModLoader::Quilt => unreachable!(),
            ModLoader::Forge => {
                forge::installer::download(&self, game_version);
            }
            ModLoader::NeoForge => {
                todo!()
            }
        }
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum ModLoaderVersionType {
    #[default]
    Stable,
    Beta,
}

impl From<bool> for ModLoaderVersionType {
    fn from(is_stable: bool) -> Self {
        match is_stable {
            true => ModLoaderVersionType::Stable,
            false => ModLoaderVersionType::Beta,
        }
    }
}

#[derive(Debug)]
pub struct ModLoaderVersionMap {
    version_getter: fn(&str) -> Vec<ModLoaderVersionInfo>,
    versions_map: Mutex<HashMap<String, Arc<[ModLoaderVersionInfo]>>>,
}

impl ModLoaderVersionMap {

    pub fn get(&self, game_version: &str) -> Arc<[ModLoaderVersionInfo]> {
        let mut versions_lock = self.versions_map.lock().unwrap();
        if let Some(val) = versions_lock.get(game_version) {
            val.clone()
        }
        else {
            let compatible_versions = (self.version_getter)(game_version);
            versions_lock.insert(game_version.to_owned(), compatible_versions.into());
            versions_lock.get(game_version).unwrap().clone()
        }

    }

    pub fn contains(&self, game_version: &str) -> bool {
        let mut versions_lock = self.versions_map.lock().unwrap();
        let value = versions_lock.get(game_version);
        if let Some(val) = value {
            val.is_empty()
        }
        else {
            let compatible_versions = (self.version_getter)(game_version);
            versions_lock.insert(game_version.to_owned(), compatible_versions.into());
            versions_lock.get(game_version).unwrap().is_empty()
        }
    }

    pub fn new(version_getter: fn(&str) -> Vec<ModLoaderVersionInfo>, versions_map: Mutex<HashMap<String, Arc<[ModLoaderVersionInfo]>>>) -> Self {
        Self { version_getter, versions_map }
    }
}

impl Default for ModLoaderVersionMap {
    fn default() -> Self {
        Self {
            version_getter: |_| Vec::new(),
            versions_map: Mutex::new(HashMap::new()),
        }
    }
}

impl Clone for ModLoaderVersionMap {
    fn clone(&self) -> Self {
        Self {
            version_getter: self.version_getter,
            versions_map: Mutex::new(self.versions_map.lock().unwrap().clone()),
        }
    }
}