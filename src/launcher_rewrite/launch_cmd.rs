use std::os::windows::process::CommandExt;
use std::path::Path;
use std::process::{Child, Command};
use std::sync::atomic::Ordering;
use std::{fs, thread};
use aho_corasick::AhoCorasick;
use crate::gui::game_output::open_game_output_window;
use crate::launcher_rewrite::GAME_INSTANCE_COUNT;
use crate::launcher_rewrite::installer::Downloadable;
use crate::launcher_rewrite::launch_properties::Version;
use crate::launcher_rewrite::path_handler::{DEV_GAME_DIR, get_assets_root, get_bin_path};

#[cfg(target_os = "windows")]
pub const CLASSPATH_SEPARATOR: char = ';';
#[cfg(not(target_os = "windows"))]
pub const CLASSPATH_SEPARATOR: char = ':';

impl Version {

    pub fn launch(&self, username: &str, uuid: &str, token: &str, resolution: Option<(u32, u32)>, game_dir: &Path) {
        #[cfg(debug_assertions)]
        let game_dir = DEV_GAME_DIR.as_path();
        fs::create_dir_all(game_dir).expect("Failed to create game directory");
        let mut cmd = Command::new("java");
        cmd.current_dir(game_dir).raw_arg(get_jvm_args(&self, resolution).as_str()).raw_arg(self.main_class()).raw_arg(get_game_args(&self, username, uuid, token, resolution, game_dir).as_str());
        println!("Main Class: {}", self.main_class());
        println!("Command: {:?}", cmd);
        let _ = thread::Builder::new().name("Game Process Thread".to_owned()).spawn(move || {
            GAME_INSTANCE_COUNT.fetch_add(1, Ordering::SeqCst);
            match cmd.status() {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Failed to start game. {e}");
                }
            }
            GAME_INSTANCE_COUNT.fetch_sub(1, Ordering::SeqCst);
        });
    }

}

fn get_classpath(version: &Version) -> String {
    let mut classpath = String::new();
    version.libs().iter().for_each(|lib| {
        classpath.push_str(lib.get_file_path(version.game_version()).to_str().unwrap());
        classpath.push(CLASSPATH_SEPARATOR);
    });
    if classpath.ends_with(CLASSPATH_SEPARATOR) {
        classpath.pop();
    }
    println!("Classpath: {}", classpath);
    classpath
}

fn get_game_args(version: &Version, username: &str, uuid: &str, token: &str, resolution: Option<(u32, u32)>, game_dir: &Path) -> String {

    #[cfg(debug_assertions)]
    let binding = DEV_GAME_DIR.to_string_lossy();
    #[cfg(debug_assertions)]
    let game_dir = binding.as_ref();
    #[cfg(not(debug_assertions))]
    let game_dir = game_dir.to_string_lossy().as_ref();


    let binding = get_assets_root();
    let assets_root = binding.to_str().unwrap();
    let assets_name = version.assets().id().rsplit_once('.').map(|split| split.0).unwrap_or(version.assets().id());

    let has_custom_resolution = resolution.is_some();
    let disp = resolution.map(|res| (res.0.to_string(), res.1.to_string()));
    let (width, height) = disp.as_ref().map(|res| (res.0.as_str(), res.1.as_str())).unwrap_or(("1920", "1080"));
    let quick_play = false;
    let quick_play_singleplayer = false;
    let quick_play_multiplayer = false;
    let quick_play_realms = false;
    let owns_game = true;

    let unformatted: String = version.arguments().game_args().iter().filter(|a| a.matches(!owns_game, has_custom_resolution, quick_play, quick_play_singleplayer, quick_play_multiplayer, quick_play_realms)).map(|a| a.values()).flatten().map(|s| s.as_str()).intersperse(" ").collect();
    const PLACEHOLDERS: &[&str] = &["${auth_player_name}", "${version_name}", "${game_directory}", "${assets_root}", "${assets_index_name}", "${auth_uuid}", "${auth_access_token}", "${clientid}", "${auth_xuid}", "${user_type}", "${version_type}", "${resolution_width}", "${resolution_height}", "${quickPlayPath}", "${quickPlaySingleplayer}", "${quickPlayMultiplayer}", "${quickPlayRealms}"];
    let replace = [username, version.game_version(), game_dir, assets_root, assets_name, uuid, token, "telemetry", "asdf", "msa", version.version_type().as_str(), width, height, "placeholder", "placeholder", "placeholder", "placeholder"];
    let ac = AhoCorasick::new(PLACEHOLDERS).unwrap();
    let formatted = ac.replace_all(unformatted.as_str(), &replace);
    formatted
}

fn get_jvm_args(version: &Version, resolution: Option<(u32, u32)>) -> String {

    let has_custom_resolution = resolution.is_some();
    let quick_play = false;
    let quick_play_singleplayer = false;
    let quick_play_multiplayer = false;
    let quick_play_realms = false;
    let owns_game = true;

    let natives_dir = get_bin_path(version.id());
    let classpath = get_classpath(version);
    let log_config_file_path = version.log_info().get_file_path(version.id());

    let unformatted: String = version.arguments().jvm_args().iter().filter(|a| a.matches(!owns_game, has_custom_resolution, quick_play, quick_play_singleplayer, quick_play_multiplayer, quick_play_realms)).map(|a| a.values()).flatten().map(|s| s.as_str()).intersperse(" ").collect();
    const PLACEHOLDERS: &[&str] = &["${natives_directory}", "${launcher_name}", "${launcher_version}", "${classpath}", "${logging_path}"];
    let binding = natives_dir.to_string_lossy();
    let binding2 = log_config_file_path.to_string_lossy();
    let replace = [binding.as_ref(), env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"), classpath.as_str(), binding2.as_ref()];
    let ac = AhoCorasick::new(PLACEHOLDERS).unwrap();
    let formatted = ac.replace_all(unformatted.as_str(), &replace);
    formatted
}