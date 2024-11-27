use std::collections::HashMap;
use std::sync::{LazyLock, OnceLock};
use crate::launcher_rewrite::profiles::ModLoader;

pub static FABRIC_MANIFEST: LazyLock<ModLoaderVersionManifest> = LazyLock::new(Default::default);
pub static QUILT_MANIFEST: LazyLock<ModLoaderVersionManifest> = LazyLock::new(Default::default);
pub static FORGE_MANIFEST: LazyLock<ModLoaderVersionManifest> = LazyLock::new(Default::default);
pub static NEO_FORGE_MANIFEST: LazyLock<ModLoaderVersionManifest> = LazyLock::new(Default::default);

#[derive(Debug, Default, Clone)]
pub struct ModLoaderVersionManifest {
    loader: ModLoader,
    versions_map: HashMap<String, Vec<ModLoaderVersionInfo>>,
}

impl ModLoaderVersionManifest {

    pub fn get_loader_versions(&self, game_version_name: &str) -> &[ModLoaderVersionInfo] {
        self.versions_map.get(game_version_name).map(|v| v.as_ref()).unwrap_or(&[])
    }

    pub fn has_loader_for_game_version(&self, game_version_name: &str) -> bool {
        self.versions_map.contains_key(game_version_name)
    }

    pub fn has_stable_loader_version_for_game_version(&self, game_version_name: &str) -> bool {
        if let Some(loader_versions) = self.versions_map.get(game_version_name) {
            return loader_versions.iter().filter(|loader_version| loader_version.is_stable()).next().is_some()
        }
        false
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

}

#[derive(Debug, Default, Clone)]
pub struct ModLoaderVersionInfo {
    version_name: String,
    version_type: ModLoaderVersionType,
    version_client_url: String,
}

impl ModLoaderVersionInfo {

    pub fn is_stable(&self) -> bool {
        return self.version_type == ModLoaderVersionType::STABLE
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
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum ModLoaderVersionType {
    #[default]
    STABLE,
    BETA
}