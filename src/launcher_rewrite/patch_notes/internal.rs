use serde::{Deserialize, Serialize};
use crate::launcher_rewrite::version_type::VersionType;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub (in crate::launcher_rewrite::patch_notes) struct PatchNotesInternal<'file> {
    pub version: u64,
    #[serde(borrow)]
    pub entries: Vec<PatchNotesEntry<'file>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub (in crate::launcher_rewrite::patch_notes) struct PatchNotesEntry<'file> {
    #[serde(borrow)]
    pub title: &'file str,
    #[serde(rename = "type")]
    pub version_type: VersionType,
    #[serde(borrow)]
    pub version: &'file str,
    pub body: String,
    #[serde(borrow)]
    pub id: &'file str,
    #[serde(borrow)]
    pub content_path: Option<&'file str>,
    #[serde(default, skip)]
    pub image: (), // TODO
}

