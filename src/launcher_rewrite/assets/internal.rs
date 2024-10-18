use std::collections::HashMap;
use std::num::NonZeroU64;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub (in crate::launcher_rewrite::assets) struct AssetsIndexJson<'file> {
    #[serde(borrow)]
    pub objects: HashMap<&'file str, ObjectInfo<'file>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub (in crate::launcher_rewrite::assets) struct ObjectInfo<'file> {
    #[serde(borrow)]
    pub hash: &'file str,
    pub size: Option<NonZeroU64>,
}