use std::ffi::OsString;
use std::path::Path;
use std::sync::atomic::Ordering;
use std::thread::sleep;
use std::time::Duration;
use clap::Parser;
use crate::launcher_rewrite::{GAME_INSTANCE_COUNT, launch_game};
use crate::launcher_rewrite::profiles::ModLoader;
use crate::launcher_rewrite::mod_loader_version_manifest::LATEST_STABLE_TEXT;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = ("latest-release".to_owned()))]
    game_version: String,
    #[arg(short, long, value_enum, default_value_t = ModLoader::Vanilla)]
    loader: ModLoader,
    #[arg(short = 'v', long, default_value_t = LATEST_STABLE_TEXT.to_owned())]
    loader_version: String,
    #[arg(short, long)]
    profile_name: Option<String>,
    #[arg(long)]
    width: Option<u32>,
    #[arg(long)]
    height: Option<u32>,
    #[arg(short, long, default_value_t = ("%appdata%/.minecraft/".to_owned()))]
    dir: String,
}

impl Args {
    pub fn game_version(&self) -> &str {
        &self.game_version
    }

    pub fn loader(&self) -> ModLoader {
        self.loader
    }

    pub fn loader_version(&self) -> &str {
        &self.loader_version
    }

    pub fn profile_name(&self) -> Option<&str> {
        self.profile_name.as_ref().map(|a| a.as_str())
    }

    pub fn width(&self) -> Option<u32> {
        self.width
    }

    pub fn height(&self) -> Option<u32> {
        self.height
    }

    pub fn dir(&self) -> &Path {
        Path::new(&self.dir)
    }
}

pub fn cli_main() {
    let args = Args::parse();

    let launched = launch_game(args.game_version(), args.loader(), args.loader_version(), args.width(), args.height(), args.dir());

    loop {
        if GAME_INSTANCE_COUNT.load(Ordering::SeqCst) == 0 {
            break;
        }
        // Sleep just to let the os run other tasks
        sleep(Duration::from_millis(1000));
    }
    println!("Launcher Exiting!");
}