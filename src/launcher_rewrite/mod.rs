use std::fs;
use std::path::Path;
use std::sync::atomic::AtomicUsize;
use crate::launcher_rewrite::authentication::LOGGED_IN_ACCOUNT_DATA;
use crate::launcher_rewrite::installed_versions::INSTALLED_VERSIONS;
use crate::launcher_rewrite::installer::Downloadable;
use crate::launcher_rewrite::launch_properties::Version;
use crate::launcher_rewrite::manifest::GAME_VERSION_MANIFEST;
use crate::launcher_rewrite::path_handler::get_vanilla_client_json_path;
use crate::launcher_rewrite::profiles::{convert_width_height, ModLoader, PROFILES};

pub mod launch_properties;
pub mod version_type;
pub mod path_handler;
pub mod assets;
pub mod util;
pub mod installer;
pub mod extractor;
pub mod launch_cmd;
pub mod manifest;
pub mod urls;
pub mod profiles;
pub mod authentication;
pub mod patch_notes;
pub mod installed_versions;
mod fabric;
mod mod_loader_version_manifest;

// Number of game instances open. Know this so that way we can refrain from exiting the launcher process until all game instances were closed by the user.
pub static GAME_INSTANCE_COUNT: AtomicUsize = AtomicUsize::new(0);

pub fn launch_game(profile_id: u128) {
    let profile_lock = PROFILES.read().unwrap();
    let profile = profile_lock.find_profile(profile_id);
    if let Some(profile) = profile {
        let ver = GAME_VERSION_MANIFEST.get_version_from_str(profile.version_name());
        if let Some(version_info) = ver {
            let need_to_install = !INSTALLED_VERSIONS.read().unwrap().contains(version_info.id(), profile.mod_loader(), None);

            if need_to_install {
                // Download vanilla json
                version_info.download(version_info.id());
                // Download modded version json

            }

            // TODO handle installing modded versions
            match profile.mod_loader() {
                ModLoader::Vanilla => {}
                ModLoader::Fabric => {

                }
                ModLoader::Quilt => {}
                ModLoader::Forge => {

                }
                ModLoader::NeoForge => {}
            }


            // TODO Error Handling
            let json_path = get_vanilla_client_json_path(version_info.id(), profile.mod_loader(), profile.mod_loader_version());
            let client_file_contents = fs::read_to_string(json_path).expect("Failed to read client json");
            let version: Version = serde_json::from_str(client_file_contents.as_str()).expect("Invalid Client Json!");

            if need_to_install {
                version.install();
            }

            if need_to_install {
                INSTALLED_VERSIONS.write().unwrap().add(version_info.id(), profile.mod_loader(), None);
            }

            let acc_lock = LOGGED_IN_ACCOUNT_DATA.read().unwrap();
            let current_account = acc_lock.active_account().unwrap();
            let username = current_account.minecraft_account_info().name();
            let uuid = current_account.minecraft_account_info().id();
            let token = current_account.minecraft_token();
            let width = profile.width();
            let height = profile.height();
            let res = convert_width_height(width, height);

            version.launch(username, uuid, token, res, Path::new(profile.mc_directory()));

        }
        else {
            eprintln!("Attempted to launch profile with illegal version name {}!", profile.version_name());
        }
    }
    else {
        eprintln!("Attempted to launch nonexistent profile with id {}!", profile_id);
    }
}