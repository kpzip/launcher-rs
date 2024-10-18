use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};
use crate::launcher_rewrite::authentication::LOGGED_IN_ACCOUNT_DATA;
use crate::launcher_rewrite::installed_versions::InstalledVersions;
use crate::launcher_rewrite::path_handler::{INSTALLED_VERSIONS_FILE_PATH, TOKENS_FILE_PATH};

pub fn load_from_file<T>(path: &Path, pretty: bool) -> T
where
    T: for<'a> Deserialize<'a> + Serialize + Default
{
    let exists = fs::exists(path);
    match exists {
        Ok(true) => {
            let read_file = fs::read_to_string(path).expect("Error reading file: ");
            let deserialized = serde_json::from_str(read_file.as_str()).expect("Invalid syntax in config file: ");
            deserialized
        },
        Ok(false) => {
            let ret = T::default();
            let serialized = if pretty {
                serde_json::to_string_pretty(&ret).expect("Error Serializing data: ")
            } else {
                serde_json::to_string(&ret).expect("Error Serializing data: ")
            };
            fs::create_dir_all(path.parent().unwrap_or("".as_ref())).expect("Unable to create directories");
            fs::write(path, serialized).expect("Error writing file: ");
            ret
        },
        Err(e) => {
            panic!("Unable to access file location.\n{e}")
        }
    }
}

pub fn save_to_file<T>(value: &T, path: &Path, pretty: bool)
where
    T: Serialize
{
    let serialized = if pretty {
        serde_json::to_string_pretty(value).expect("Error Serializing config data: ")
    } else {
        serde_json::to_string(value).expect("Error Serializing config data: ")
    };
    fs::write(path, serialized).expect("Error writing to file: ");
}