use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ops::Deref;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AssetIndex {
    objects: HashMap<String, AssetData>,
}

impl Deref for AssetIndex {
    type Target = HashMap<String, AssetData>;

    fn deref(&self) -> &Self::Target {
        &self.objects
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AssetData {
    hash: String,
    size: u64,
}

impl AssetData {
    pub fn hash(&self) -> &str {
        &self.hash
    }

    pub fn size(&self) -> u64 {
        self.size
    }
}
