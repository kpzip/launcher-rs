use std::collections::HashMap;
use std::fs;
use std::num::NonZeroU64;
use std::path::PathBuf;
use std::sync::LazyLock;
use chrono::{DateTime, Utc};
use reqwest::Url;
use serde::{de, Deserialize, Deserializer};
use serde::de::Error;
use crate::launcher_rewrite::installer::{DEFAULT_DOWNLOADER_CLIENT, Downloadable};
use crate::launcher_rewrite::path_handler::{GAME_VERSION_MANIFEST_PATH, get_vanilla_client_json_path};
use crate::launcher_rewrite::profiles::ModLoader;
use crate::launcher_rewrite::urls::GAME_VERSION_MANIFEST_URL;
use crate::launcher_rewrite::util::hash::{FileHash, Sha1, sha1_from_base64_str};
use crate::launcher_rewrite::version_type::VersionType;

mod internal;

pub const LATEST_RELEASE_TEXT: &str = "latest-release";
pub const LATEST_SNAPSHOT_TEXT: &str = "latest-snapshot";

pub static GAME_VERSION_MANIFEST: LazyLock<GameVersionManifest> = LazyLock::new(init_game_version_manifest);

fn init_game_version_manifest() -> GameVersionManifest {
    let manifest_response = DEFAULT_DOWNLOADER_CLIENT.get(GAME_VERSION_MANIFEST_URL).send().map(|r| r.text().expect("Invalid response from Mojang!"));

    match manifest_response {
        Ok(json_data) => {
            fs::create_dir_all(GAME_VERSION_MANIFEST_PATH.as_path().parent().unwrap_or("".as_ref())).expect("Unable to create directories");
            fs::write(GAME_VERSION_MANIFEST_PATH.as_path(), json_data.as_str()).unwrap();
            serde_json::from_str(json_data.as_str()).expect("Invalid manifest JSON supplied by Mojang!")
        }
        Err(_) => {

            // Assume we're offline, but there are other possible error cases
            let offline_json_data = fs::read_to_string(GAME_VERSION_MANIFEST_PATH.as_path()).unwrap();
            serde_json::from_str(offline_json_data.as_str()).expect("Invalid manifest JSON!")
        }
    }
}

// INVARIANT: `latest` and `latest-snapshot` must be present as keys in `version_info`
#[derive(Debug, Clone)]
pub struct GameVersionManifest {
    latest: String,
    latest_snapshot: String,
    version_info: HashMap<String, GameVersionInfo>,
}

impl GameVersionManifest {

    ///
    /// Converts `latest-release` and `latest-snapshot` into their respective actual version names based on the manifest info.
    /// Returns `name` if `name` is an invalid version!
    ///
    pub fn sanitize_version_name<'a>(&'a self, name: &'a str) -> &'a str {
        match name {
            LATEST_RELEASE_TEXT => self.latest.as_str(),
            LATEST_SNAPSHOT_TEXT => self.latest_snapshot.as_str(),
            other => other,
        }
    }

    ///
    /// Gets the version, resolving `latest-release` and `latest-snapshot` to their respective versions
    ///
    pub fn get_version_from_str(&self, name: &str) -> Option<&GameVersionInfo> {
        match name {
            LATEST_RELEASE_TEXT => Some(self.latest_version()),
            LATEST_SNAPSHOT_TEXT => Some(self.latest_snapshot()),
            other => self.get_version_by_name(other),
        }
    }

    ///
    /// Gets the version by name only, not resolving `latest-release` and `latest-snapshot` to their respective versions
    ///
    pub fn get_version_by_name(&self, name: &str) -> Option<&GameVersionInfo> {
        self.version_info.get(name)
    }

    pub fn latest_version(&self) -> &GameVersionInfo {
        self.version_info.get(self.latest.as_str()).unwrap()
    }

    pub fn latest_snapshot(&self) -> &GameVersionInfo {
        self.version_info.get(self.latest_snapshot.as_str()).unwrap()
    }

    pub fn versions_vec(&self, include_snapshots: bool, include_historical: bool) -> Vec<&str> {
        self.version_info.iter().filter(|(name, info)| (!info.is_snapshot() || include_snapshots) && (!info.is_historical() || include_historical)).map(|(name, info)| name.as_str()).collect()
    }

    pub fn versions_vec_with_latest(&self, include_snapshots: bool, include_historical: bool) -> Vec<&str> {
        let mut ret = self.versions_vec(include_snapshots, include_historical);
        ret.insert(0, LATEST_SNAPSHOT_TEXT);
        if include_snapshots { ret.insert(0, LATEST_RELEASE_TEXT); }
        ret
    }

    pub fn versions(&self) -> &HashMap<String, GameVersionInfo> {
        &self.version_info
    }

}

impl<'de> Deserialize<'de> for GameVersionManifest {

    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        let internal = internal::GameVersionManifest::deserialize(deserializer)?;
        let latest = internal.latest.release.to_owned();
        let latest_snapshot = internal.latest.release.to_owned();
        let version_info = internal.versions.into_iter().map(|internal_info| {
            let binding = GameVersionInfo::from_internal(internal_info)?;
            Ok((binding.id.to_owned(), binding))
        }).collect::<Result<HashMap<String, GameVersionInfo>, D::Error>>()?;
        if !version_info.contains_key(latest.as_str()) {
            return Err(Error::custom("`latest` version id not found in version list!"));
        }
        if !version_info.contains_key(latest_snapshot.as_str()) {
            return Err(Error::custom("`latest_snapshot` version id not found in version list!"));
        }

        Ok(Self {
            latest,
            latest_snapshot,
            version_info,
        })

    }
}

#[derive(Debug, Clone)]
pub struct GameVersionInfo {
    id: String,
    version_type: VersionType,
    url: Url,
    release_time: DateTime<Utc>,
    sha1: Sha1,
}

impl GameVersionInfo {

    fn from_internal<E: Error>(internal: internal::GameVersionInfo) -> Result<Self, E> {
        let id = internal.id.to_owned();
        let version_type = internal.version_type;
        let url = Url::parse(internal.url).map_err(|e| de::Error::custom(e))?;
        let release_time = internal.time;
        let sha1 = sha1_from_base64_str(internal.sha1)?;
        Ok(Self {
            id,
            version_type,
            url,
            release_time,
            sha1,
        })
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn version_type(&self) -> VersionType {
        self.version_type
    }

    pub fn url(&self) -> &Url {
        &self.url
    }

    pub fn release_time(&self) -> DateTime<Utc> {
        self.release_time
    }

    pub fn sha1(&self) -> Sha1 {
        self.sha1
    }

    pub fn is_snapshot(&self) -> bool {
        self.version_type == VersionType::Snapshot
    }

    pub fn is_historical(&self) -> bool {
        self.version_type == VersionType::OldAlpha || self.version_type == VersionType::OldBeta
    }
}

impl Downloadable for GameVersionInfo {

    fn get_download_url(&self) -> &Url {
        &self.url
    }

    fn get_file_path(&self, version_name: &str) -> PathBuf {
        debug_assert!(version_name == self.id.as_str());
        get_vanilla_client_json_path(version_name, ModLoader::Vanilla, "")
    }

    fn get_hash(&self) -> Option<FileHash> {
        Some(FileHash::Sha1(self.sha1))
    }

    fn get_size(&self) -> Option<NonZeroU64> {
        None
    }

}

#[cfg(test)]
mod tests {
    use std::fs;
    use crate::launcher_rewrite::manifest::GameVersionManifest;
    use crate::launcher_rewrite::path_handler::GAME_VERSION_MANIFEST_PATH;

    #[test]
    fn game_version_manifest_deserialize_test() {
        let manifest: GameVersionManifest = serde_json::from_str(fs::read_to_string(GAME_VERSION_MANIFEST_PATH.as_path()).unwrap().as_str()).unwrap();
        println!("{:?}", manifest);
        panic!()
    }

}