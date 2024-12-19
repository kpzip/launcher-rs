use std::borrow::Cow;
use std::env::current_dir;
use std::ffi::OsString;
use std::path::{MAIN_SEPARATOR_STR, Path, PathBuf};
use std::str::FromStr;
use std::sync::LazyLock;
use const_format::concatcp;
use crate::launcher_rewrite::profiles::ModLoader;

#[cfg(debug_assertions)]
pub static LAUNCHER_DIR: LazyLock<PathBuf> = LazyLock::new(|| PathBuf::from_str(concatcp!(env!("CARGO_MANIFEST_DIR"), PATH_SEP, "run", PATH_SEP, "launcher")).unwrap());
#[cfg(not(debug_assertions))]
pub static LAUNCHER_DIR: LazyLock<PathBuf> = LazyLock::new(|| current_dir().unwrap());

#[cfg(debug_assertions)]
pub static DEV_GAME_DIR: LazyLock<PathBuf> = LazyLock::new(|| PathBuf::from_str(concatcp!(env!("CARGO_MANIFEST_DIR"), PATH_SEP, "run", PATH_SEP, "game")).unwrap());

pub const PATH_SEP: &'static str = MAIN_SEPARATOR_STR;

pub const VERSIONS_FOLDER: &'static str = "versions";
pub const ASSETS_FOLDER: &'static str = "assets";
pub const INDEXES_FOLDER: &'static str = concatcp!(ASSETS_FOLDER, PATH_SEP, "indexes");
pub const LOG_CONFIGS_FOLDER: &'static str = concatcp!(ASSETS_FOLDER, PATH_SEP, "log_configs");
pub const OBJECTS_FOLDER: &'static str = concatcp!(ASSETS_FOLDER, PATH_SEP, "objects");
pub const SKINS_FOLDER: &'static str = concatcp!(ASSETS_FOLDER, PATH_SEP, "skins");

pub const BIN_PATH: &'static str = "bin";

pub const VANILLA_MANIFEST_LOCATION: &str = concatcp!(VERSIONS_FOLDER, PATH_SEP, "vanilla_mainifest_v2.json");
pub const FABRIC_MANIFEST_LOCATION: &str = concatcp!(VERSIONS_FOLDER, PATH_SEP, "fabric_manifest_v2.json");
pub const QUILT_MANIFEST_LOCATION: &str = concatcp!(VERSIONS_FOLDER, PATH_SEP, "quilt_manifest_v2.json");

pub const LAUNCHER_CFG_LOCATION: &str = "profiles.json";
pub const TOKENS_FILE_LOCATION: &str = "tokens.json";
pub const INSTALLED_VERSIONS_FILE_LOCATION: &str = "installed.json";

// Vanilla is special and doesn't need a folder
pub const VANILLA_CLIENT_JSON_NAME: &str = "vanilla.json";

pub const FABRIC_CLIENT_JSON_FOLDER_NAME: &str = "fabric";
pub const QUILT_CLIENT_JSON_FOLDER_NAME: &str = "quilt";
pub const FORGE_CLIENT_JSON_FOLDER_NAME: &str = "forge";
pub const NEO_FORGE_CLIENT_JSON_FOLDER_NAME: &str = "neo_forge";

pub static GAME_VERSION_MANIFEST_PATH: LazyLock<PathBuf> = LazyLock::new(game_version_manifest_path);
pub static LAUNCHER_CFG_PATH: LazyLock<PathBuf> = LazyLock::new(launcher_cfg_path);
pub static TOKENS_FILE_PATH: LazyLock<PathBuf> = LazyLock::new(token_file_path);
pub static INSTALLED_VERSIONS_FILE_PATH: LazyLock<PathBuf> = LazyLock::new(installed_versions_file_path);

fn client_json_name(mod_loader: ModLoader, loader_version: &str) -> Cow<'static, str> {
    match mod_loader {
        ModLoader::Vanilla => VANILLA_CLIENT_JSON_NAME.into(),
        ModLoader::Fabric => format!("{}{}{}.json", FABRIC_CLIENT_JSON_FOLDER_NAME, PATH_SEP, loader_version).into(),
        ModLoader::Quilt => format!("{}{}{}.json", QUILT_CLIENT_JSON_FOLDER_NAME, PATH_SEP, loader_version).into(),
        ModLoader::Forge => format!("{}{}{}.json", FORGE_CLIENT_JSON_FOLDER_NAME, PATH_SEP, loader_version).into(),
        ModLoader::NeoForge => format!("{}{}{}.json", NEO_FORGE_CLIENT_JSON_FOLDER_NAME, PATH_SEP, loader_version).into(),
    }
}

#[inline(always)]
pub fn from_launcher_dir<I, P>(elements: I) -> PathBuf
where
    P: AsRef<Path>,
    I: IntoIterator<Item = P>
{
    let mut buf: OsString = LAUNCHER_DIR.clone().into_os_string();
    elements.into_iter().for_each(|p| {
        buf.push(PATH_SEP);
        buf.push(p.as_ref());
    });
    buf.into()
}

pub fn get_assets_root() -> PathBuf {
    from_launcher_dir([ASSETS_FOLDER])
}

pub fn get_vanilla_client_json_path(version_name: &str, mod_loader: ModLoader, mut loader_version: &str) -> PathBuf {
    if let Some(manifest) = mod_loader.get_manifest() {
        loader_version = manifest.sanitize_loader_version_name(version_name)
    }
    from_launcher_dir([VERSIONS_FOLDER, version_name, client_json_name(mod_loader, loader_version).as_ref()])
}

pub fn get_assets_index_dir(index_name: &str) -> PathBuf {
    from_launcher_dir([INDEXES_FOLDER, index_name])
}

pub fn get_log_configs_folder(config_name: &str) -> PathBuf {
    from_launcher_dir([LOG_CONFIGS_FOLDER, config_name])
}

pub fn get_objects_dir() -> PathBuf {
    from_launcher_dir([OBJECTS_FOLDER])
}

pub fn get_bin_path(version_name: &str) -> PathBuf {
    from_launcher_dir([VERSIONS_FOLDER, version_name, BIN_PATH])
}

fn game_version_manifest_path() -> PathBuf {
    from_launcher_dir([VANILLA_MANIFEST_LOCATION])
}

fn launcher_cfg_path() -> PathBuf {
    from_launcher_dir([LAUNCHER_CFG_LOCATION])
}

fn token_file_path() -> PathBuf {
    from_launcher_dir([TOKENS_FILE_LOCATION])
}

fn installed_versions_file_path() -> PathBuf {
    from_launcher_dir([INSTALLED_VERSIONS_FILE_LOCATION])
}

