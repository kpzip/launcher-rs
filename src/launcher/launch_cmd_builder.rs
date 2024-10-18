use crate::launcher::file_handler::get_assets_dir;
use crate::launcher::launch_properties::LaunchProperties;
use crate::launcher::version::Version;
use crate::LOGGED_IN_ACCOUNT_DATA;
use std::collections::HashMap;
use std::path::Path;
use std::process::Command;
use std::{fs, str};
use crate::launcher::settings::profiles::ModLoader;

#[cfg(target_os = "windows")]
pub const CLASSPATH_SEPARATOR: &str = ";";
#[cfg(not(target_os = "windows"))]
pub const CLASSPATH_SEPARATOR: &str = ":";

pub fn generate_args_and_launch_game(minecraft_directory: impl AsRef<Path>, launch_cmd_builder: LaunchCmdBuilder) {
    println!("Command Builder: {:?}", launch_cmd_builder);
    let cmd = Command::new("java").current_dir(minecraft_directory).args(launch_cmd_builder.get_args()).output().expect("failed to execute process");
    println!("Stdout:\n{}", str::from_utf8(cmd.stdout.as_slice()).unwrap());
    println!("Stderr:\n{}", str::from_utf8(cmd.stderr.as_slice()).unwrap());
}

pub fn build_launch_command(minecraft_directory: &Path, minecraft_launcher_directory: &Path, version: &Version, loader: ModLoader) -> LaunchCmdBuilder {
    let launch_properties = version.launch_properties(loader).unwrap();
    let mut cmd_builder = LaunchCmdBuilder::new();
    cmd_builder.build(&launch_properties, minecraft_directory, minecraft_launcher_directory, version, loader).expect("failed building command");
    cmd_builder
}

#[derive(Debug)]
pub struct LaunchCmdBuilder {
    java_cmd: String,
    jvm_args: Vec<String>,
    main_class_name: String,
    game_args: Vec<String>,
}

impl LaunchCmdBuilder {
    fn new() -> Self {
        Self {
            java_cmd: String::new(),
            jvm_args: Vec::new(),
            main_class_name: String::new(),
            game_args: Vec::new(),
        }
    }

    fn build(&mut self, launch_properties: &LaunchProperties, minecraft_directory: &Path, minecraft_launcher_directory: &Path, version: &Version, loader: ModLoader) -> Result<(), ()> {
        self.java_cmd.push_str("java");
        self.main_class_name.push_str(launch_properties.main_class());
        launch_properties.arguments().game().iter().for_each(|arg| {
            arg.get_args().iter().for_each(|s| {
                self.game_args.push(s.clone());
            });
        });
        launch_properties.arguments().jvm().iter().for_each(|arg| {
            arg.get_args().iter().for_each(|s| {
                self.jvm_args.push(s.clone());
            });
        });
        self.jvm_args.push(launch_properties.logging().argument().into());

        let session_lock = LOGGED_IN_ACCOUNT_DATA.read().unwrap();

        //let natives_directory: String = r"C:\Users\kpzip\AppData\Roaming\.minecraft\bin\6453699edf181d1c3967da820c72e49a2f653e89".into();
        let natives_directory: String = version.get_bin_path(minecraft_launcher_directory).as_os_str().to_str().unwrap().into();
        let launcher_name: String = "launcher-rs".into();
        let launcher_version: String = "2.1".into();
        let classpath: String = get_classpath(minecraft_launcher_directory, version);

        let auth_player_name: String = session_lock.active_account().ok_or(())?.minecraft_account_info().name().into();
        let version_name: String = version.id().into();
        let game_directory: String = minecraft_directory.to_str().ok_or(())?.into();
        let assets_root: String = get_assets_dir(minecraft_launcher_directory).to_str().ok_or(())?.into();
        let assets_index_name: String = launch_properties.asset_index().id().into();
        let auth_uuid: String = session_lock.active_account().ok_or(())?.minecraft_account_info().id().into();
        let auth_access_token: String = session_lock.active_account().ok_or(())?.minecraft_token().into();
        let client_id: String = "a".into(); // TODO Telemetry (add option to disable?)
        let auth_xuid: String = "1".into(); // TODO Telemetry (add option to disable?)
        let user_type: String = "msa".into();
        let version_type: String = version.version_type().as_str().into();
        let resolution_width: String = "".into();
        let resolution_height: String = "".into();
        let quick_play_path: String = "".into();
        let quick_play_singleplayer: String = "".into();
        let quick_play_multiplayer: String = "".into();
        let quick_play_realms: String = "".into();

        let log_configuration_path: String = version.get_logging_config_file_path(&minecraft_launcher_directory, loader).as_os_str().to_str().unwrap().into();

        self.format(Self::build_format_map(
            natives_directory,
            launcher_name,
            launcher_version,
            classpath,
            auth_player_name,
            version_name,
            game_directory,
            assets_root,
            assets_index_name,
            auth_uuid,
            auth_access_token,
            client_id,
            auth_xuid,
            user_type,
            version_type,
            resolution_width,
            resolution_height,
            quick_play_path,
            quick_play_singleplayer,
            quick_play_multiplayer,
            quick_play_realms,
            log_configuration_path,
        ));

        Ok(())
    }

    fn build_format_map(
        natives_directory: String,
        launcher_name: String,
        launcher_version: String,
        classpath: String,
        auth_player_name: String,
        version_name: String,
        game_directory: String,
        assets_root: String,
        assets_index_name: String,
        auth_uuid: String,
        auth_access_token: String,
        client_id: String,
        auth_xuid: String,
        user_type: String,
        version_type: String,
        resolution_width: String,
        resolution_height: String,
        quick_play_path: String,
        quick_play_singleplayer: String,
        quick_play_multiplayer: String,
        quick_play_realms: String,
        log_configuration_path: String,
    ) -> HashMap<String, String> {
        let mut map = HashMap::with_capacity(30);

        map.insert("natives_directory".into(), natives_directory);
        map.insert("launcher_name".into(), launcher_name);
        map.insert("launcher_version".into(), launcher_version);
        map.insert("classpath".into(), classpath);
        map.insert("auth_player_name".into(), auth_player_name);
        map.insert("version_name".into(), version_name);
        map.insert("game_directory".into(), game_directory);
        map.insert("assets_root".into(), assets_root);
        map.insert("assets_index_name".into(), assets_index_name);
        map.insert("auth_uuid".into(), auth_uuid);
        map.insert("auth_access_token".into(), auth_access_token);
        map.insert("client_id".into(), client_id);
        map.insert("auth_xuid".into(), auth_xuid);
        map.insert("user_type".into(), user_type);
        map.insert("version_type".into(), version_type);
        map.insert("resolution_width".into(), resolution_width);
        map.insert("resolution_height".into(), resolution_height);
        map.insert("quickPlayPath".into(), quick_play_path);
        map.insert("quickPlaySingleplayer".into(), quick_play_singleplayer);
        map.insert("quickPlayMultiplayer".into(), quick_play_multiplayer);
        map.insert("quickPlayRealms".into(), quick_play_realms);
        map.insert("path".into(), log_configuration_path);

        return map;
    }

    fn format(&mut self, vars: HashMap<String, String>) {
        vars.iter().for_each(|(name, value)| {
            self.jvm_args.iter_mut().for_each(|s| {
                *s = s.replace(format!("${{{name}}}").as_str(), value.as_str());
            });
            self.game_args.iter_mut().for_each(|s| {
                *s = s.replace(format!("${{{name}}}").as_str(), value.as_str());
            });
        });
    }

    fn get_args(&self) -> Vec<&str> {
        let mut args = Vec::new();
        self.jvm_args.iter().for_each(|arg| {
            args.push(arg.as_str());
        });
        args.push(self.main_class_name.as_str());
        self.game_args.iter().for_each(|arg| {
            args.push(arg.as_str());
        });

        args
    }
}

fn get_classpath(launcher_directory: &Path, version: &Version) -> String {
    let mut classpath = String::new();

    let bin_dir = version.get_bin_path(launcher_directory);
    fs::read_dir(bin_dir).unwrap().for_each(|f| {
        let entry = f.unwrap();
        if entry.path().extension().is_some_and(|i| i == "jar") {
            if !classpath.is_empty() {
                classpath.push_str(CLASSPATH_SEPARATOR);
            }
            classpath.push_str(entry.path().to_str().unwrap());
        }
    });

    println!("Classpath: {}", classpath);

    classpath
}
