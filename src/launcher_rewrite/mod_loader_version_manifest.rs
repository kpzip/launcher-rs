use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::{Arc, LazyLock, Mutex, OnceLock};
use crate::launcher_rewrite::fabric;
use crate::launcher_rewrite::profiles::ModLoader;

pub static FABRIC_MANIFEST: LazyLock<ModLoaderVersionManifest> = LazyLock::new(|| ModLoaderVersionManifest::new(ModLoader::Fabric, fabric::get_compatible_versions));
pub static QUILT_MANIFEST: LazyLock<ModLoaderVersionManifest> = LazyLock::new(Default::default);
pub static FORGE_MANIFEST: LazyLock<ModLoaderVersionManifest> = LazyLock::new(Default::default);
pub static NEO_FORGE_MANIFEST: LazyLock<ModLoaderVersionManifest> = LazyLock::new(Default::default);

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

    pub fn new(loader: ModLoader, version_func: fn(&str) -> Vec<ModLoaderVersionInfo>) -> Self {
        Self { loader, versions_map: ModLoaderVersionMap::new(version_func, Mutex::new(HashMap::new())) }
    }
}

#[derive(Debug, Default, Clone)]
pub struct ModLoaderVersionInfo {
    version_name: String,
    version_type: ModLoaderVersionType,
    version_client_url: String,
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

    pub fn version_client_url(&self) -> &str {
        &self.version_client_url
    }

    pub fn new(version_name: String, version_type: ModLoaderVersionType, version_client_url: String) -> Self {
        Self { version_name, version_type, version_client_url }
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