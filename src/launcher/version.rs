use crate::launcher::asset_index::AssetIndex;
use crate::launcher::launch_properties::LaunchProperties;
use crate::INSTALLED_VERSION_INFO;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::{LazyLock, Mutex, RwLock};
use crate::launcher::settings::profiles::{fabric_version, ModLoader};

#[derive(Serialize, Deserialize, Debug)]
pub struct VersionManifest {
    #[serde(rename = "latest")]
    latest_versions: LatestVersionData,
    versions: Vec<Version>,
}

impl Deref for VersionManifest {
    type Target = LatestVersionData;

    fn deref(&self) -> &Self::Target {
        &self.latest_versions
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LatestVersionData {
    #[serde(rename = "release")]
    latest_release: String,
    #[serde(rename = "snapshot")]
    latest_snapshot: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Version {
    id: String,
    #[serde(rename = "type")]
    version_type: VersionType,
    #[serde(rename = "url")]
    vanilla_url: String,
    #[serde(rename = "time")]
    updated_time: String,
    #[serde(rename = "releaseTime")]
    release_time: String,
    sha1: String,
    #[serde(rename = "complianceLevel")]
    compliance_level: u8,
}

impl Version {
    // Getters

    #[inline(always)]
    pub(crate) fn launch_properties(&self, loader: ModLoader) -> Option<LaunchProperties> {
        INSTALLED_VERSION_INFO.lock().unwrap().get(&(self.id.clone(), loader)).map(|tup| tup.1.clone())
    }

    #[inline(always)]
    pub(crate) fn assets_index(&self, loader: ModLoader) -> Option<AssetIndex> {
        INSTALLED_VERSION_INFO.lock().unwrap().get(&(self.id.clone(), loader)).map(|tup| tup.0.clone())
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn version_type(&self) -> VersionType {
        self.version_type
    }

    pub fn url(&self, loader: ModLoader) -> String {
        match loader {
            ModLoader::Vanilla => self.vanilla_url.clone(),
            ModLoader::Fabric => format!("https://meta.fabricmc.net/v2/versions/loader/{}/{}/profile/json", self.id.as_str(), fabric_version()),
            ModLoader::Quilt => todo!(),
            ModLoader::Forge => todo!(),
            ModLoader::NeoForge => todo!(),
        }
    }

    pub fn updated_time(&self) -> &str {
        &self.updated_time
    }

    pub fn release_time(&self) -> &str {
        &self.release_time
    }

    pub fn sha1(&self) -> &str {
        &self.sha1
    }

    pub fn compliance_level(&self) -> u8 {
        self.compliance_level
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Hash, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum VersionType {
    Release,
    Snapshot,
    OldBeta,
    OldAlpha,
}

impl VersionType {
    pub fn as_str(self) -> &'static str {
        match self {
            VersionType::Release => "release",
            VersionType::Snapshot => "snapshot",
            VersionType::OldBeta => "old_beta",
            VersionType::OldAlpha => "old_alpha",
        }
    }
}

impl VersionManifest {
    pub fn get_version(&self, name: &str) -> Option<&Version> {
        if name == "latest-release" {
            return Some(self.get_latest_release());
        }

        if name == "latest-snapshot" {
            return Some(self.get_latest_snapshot());
        }

        for version in &self.versions {
            if version.id == name {
                return Some(version);
            }
        }
        None
    }

    pub fn get_latest_release(&self) -> &Version {
        self.get_version(self.latest_release.as_str()).unwrap()
    }

    pub fn get_latest_snapshot(&self) -> &Version {
        self.get_version(self.latest_snapshot.as_str()).unwrap()
    }

    pub fn versions(&self) -> &Vec<Version> {
        &self.versions
    }

    // Converts "latest-release" and "latest-snapshot" into their respective versions
    // Also clones input str
    pub fn sanitize_version_name(&self, unsanitized_name: &str) -> String {
        if unsanitized_name == "latest-release" {
            return self.latest_release.as_str().into();
        }
        if unsanitized_name == "latest-snapshot" {
            return self.latest_snapshot.as_str().into();
        }
        return unsanitized_name.into();
    }
}
