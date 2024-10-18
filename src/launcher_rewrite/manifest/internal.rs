use chrono::{DateTime, Utc};
use serde::Deserialize;
use crate::launcher_rewrite::version_type::VersionType;

#[derive(Deserialize, Debug, Clone)]
pub (in crate::launcher_rewrite::manifest) struct GameVersionManifest<'file> {
    #[serde(borrow)]
    pub latest: LatestGameVersions<'file>,
    #[serde(borrow)]
    pub versions: Vec<GameVersionInfo<'file>>,
}

#[derive(Deserialize, Debug, Clone)]
pub (in crate::launcher_rewrite::manifest) struct LatestGameVersions<'file> {
    #[serde(borrow)]
    pub release: &'file str,
    #[serde(borrow)]
    pub snapshot: &'file str,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub (in crate::launcher_rewrite::manifest) struct GameVersionInfo<'file> {
    #[serde(borrow)]
    pub id: &'file str,
    #[serde(rename = "type")]
    pub version_type: VersionType,
    #[serde(borrow)]
    pub url: &'file str,
    pub time: DateTime<Utc>,
    pub release_time: DateTime<Utc>,
    #[serde(borrow)]
    pub sha1: &'file str,
    pub compliance_level: u64,
}