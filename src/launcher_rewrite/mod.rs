use crate::launcher_rewrite::authentication::LOGGED_IN_ACCOUNT_DATA;
use crate::launcher_rewrite::error::LauncherError;
use crate::launcher_rewrite::installed_versions::INSTALLED_VERSIONS;
use crate::launcher_rewrite::installer::Downloadable;
use crate::launcher_rewrite::launch_properties::Version;
use crate::launcher_rewrite::manifest::GAME_VERSION_MANIFEST;
use crate::launcher_rewrite::path_handler::get_vanilla_client_json_path;
use crate::launcher_rewrite::profiles::{convert_width_height, ModLoader, PROFILES};
use std::fs;
use std::path::Path;
use std::sync::atomic::AtomicUsize;

pub mod assets;
pub mod authentication;
pub mod error;
mod fabric;
mod forge;
mod game_version;
pub mod installed_versions;
pub mod installer;
mod jar_utils;
pub mod launch_cmd;
pub mod launch_properties;
pub mod manifest;
pub mod mod_loader_version_manifest;
mod neo_forge;
pub mod patch_notes;
pub mod path_handler;
pub mod profiles;
mod quilt;
pub mod urls;
pub mod util;
pub mod version_type;

// Number of game instances open. Know this so that way we can refrain from exiting the launcher process until all game instances were closed by the user.
pub static GAME_INSTANCE_COUNT: AtomicUsize = AtomicUsize::new(0);

pub fn launch_game_from_profile(profile_id: u128) -> Result<(), LauncherError> {
    let profile_lock = PROFILES.read().unwrap();
    let profile = profile_lock.find_profile(profile_id);
    if let Some(profile) = profile {
        launch_game(profile.version_name(), profile.mod_loader(), profile.mod_loader_version(), profile.width(), profile.height(), Path::new(profile.mc_directory()), profile.memory())
    } else {
        eprintln!("Attempted to launch nonexistent profile with id {}!", profile_id);
        Err(LauncherError::ProfileError)
    }
}

pub fn launch_game(game_version: &str, mod_loader: ModLoader, loader_version: &str, width: Option<u32>, height: Option<u32>, dir: &Path, memory: u16) -> Result<(), LauncherError> {
    let version_info = GAME_VERSION_MANIFEST.get_version_from_str(game_version).ok_or_else(|| {
        eprintln!("Attempted to launch profile with illegal version name {}!", game_version);
        LauncherError::ProfileError
    })?;

    let game_version = GAME_VERSION_MANIFEST.sanitize_version_name(game_version, mod_loader);
    let loader_version_c = mod_loader.get_manifest().map(|m| m.sanitize_loader_version_name(game_version, loader_version));
    let loader_version = loader_version_c.as_ref().map(|c| c.as_ref());

    let need_to_install = !INSTALLED_VERSIONS.read().unwrap().contains(version_info.id(), mod_loader, loader_version);

    if need_to_install {
        // Download vanilla json
        version_info.download(version_info.id())?;
        // Download modded version json if needed
        if let Some(manifest) = mod_loader.get_manifest() {
            manifest.get_loader_version_info(game_version, loader_version.unwrap()).download(game_version)?;
        }
    }

    let json_path = get_vanilla_client_json_path(version_info.id(), mod_loader, loader_version.unwrap_or(""));
    let client_file_contents = fs::read_to_string(json_path)?;
    let version: Version = serde_json::from_str(client_file_contents.as_str())?;

    if need_to_install {
        version.install()?;
    }

    if need_to_install {
        INSTALLED_VERSIONS.write().unwrap().add(version_info.id(), mod_loader, loader_version);
    }

    let acc_lock = LOGGED_IN_ACCOUNT_DATA.read().unwrap();
    let current_account = acc_lock.active_account().unwrap();
    let username = current_account.minecraft_account_info().name();
    let uuid = current_account.minecraft_account_info().id();
    let token = current_account.minecraft_token();
    let res = convert_width_height(width, height);

    version.launch(username, uuid, token, res, memory, dir);
    Ok(())
}
