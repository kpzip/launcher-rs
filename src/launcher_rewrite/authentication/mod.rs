use std::fs;
use std::sync::{LazyLock, RwLock};
use crate::launcher_rewrite::authentication::account_data::{AccountData, get_account_data};
use crate::launcher_rewrite::installed_versions::INSTALLED_VERSIONS;
use crate::launcher_rewrite::path_handler::{INSTALLED_VERSIONS_FILE_PATH, TOKENS_FILE_PATH};
use crate::launcher_rewrite::util::config_file::save_to_file;

pub mod account_data;

pub static LOGGED_IN_ACCOUNT_DATA: LazyLock<RwLock<AccountData>> = LazyLock::new(|| RwLock::new(get_account_data()));

pub fn save_account_data() {
    #[cfg(debug_assertions)]
    save_to_file(&*LOGGED_IN_ACCOUNT_DATA.read().unwrap(), TOKENS_FILE_PATH.as_path(), true);
    #[cfg(not(debug_assertions))]
    save_to_file(&*LOGGED_IN_ACCOUNT_DATA.read().unwrap(), TOKENS_FILE_PATH.as_path(), false);
}