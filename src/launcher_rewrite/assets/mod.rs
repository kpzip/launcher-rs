use std::num::NonZeroU64;
use std::path::PathBuf;
use reqwest::Url;
use serde::{Deserialize, Deserializer};
use serde::de::Error;
use crate::launcher_rewrite::installer::Downloadable;
use crate::launcher_rewrite::path_handler::get_objects_dir;
use crate::launcher_rewrite::util::hash::{FileHash, Sha1, sha1_from_base64_str};

mod internal;

pub const ASSETS_URL: &'static str = "https://resources.download.minecraft.net/";

#[derive(Clone)]
pub struct AssetsIndex<'file> {
    objects: Vec<Object<'file>>,
}

impl<'file> AssetsIndex<'file> {
    pub fn download_all(&self, version_name: &str) {
        self.objects.iter().for_each(|o| o.download(version_name));
    }
}

impl<'de: 'file, 'file> Deserialize<'de> for AssetsIndex<'file> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        let internal = internal::AssetsIndexJson::deserialize(deserializer)?;
        let objects: Result<Vec<Object<'file>>, D::Error> = internal.objects.iter().map(|(name, data)| Ok(Object::new(*name, data.hash, sha1_from_base64_str(data.hash)?, data.size, Url::parse(format!("{}{}/{}", ASSETS_URL, folder_name(data.hash), data.hash).as_str()).map_err(|e| Error::custom(e))?))).collect();
        let objects= objects?;
        Ok(Self {
            objects,
        })
    }
}

#[derive(Clone)]
pub struct Object<'file> {
    name: &'file str,
    sha1_str: &'file str,
    sha1: Sha1,
    size: Option<NonZeroU64>,
    url: Url,
}

impl<'file> Object<'file> {
    pub fn new(name: &'file str, sha1_str: &'file str, sha1: Sha1, size: Option<NonZeroU64>, url: Url) -> Self {
        Self { name, sha1_str, sha1, size, url }
    }
}

#[inline(always)]
fn folder_name(hash: &str) -> &str {
    &hash[..2]
}

impl<'file> Downloadable for Object<'file> {
    fn get_download_url(&self) -> &Url {
        &self.url
    }

    fn get_file_path(&self, _version: &str) -> PathBuf {
        let mut path = get_objects_dir();
        path.push(folder_name(self.sha1_str));
        path.push(self.sha1_str);
        path
    }

    fn get_hash(&self) -> Option<FileHash> {
        Some(FileHash::Sha1(self.sha1))
    }

    fn get_size(&self) -> Option<NonZeroU64> {
        self.size
    }
}