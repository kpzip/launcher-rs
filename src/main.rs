#![feature(iter_intersperse)]
#![feature(panic_payload_as_str)]

use crate::launcher_rewrite::authentication::save_account_data;
use crate::launcher_rewrite::installed_versions::save_installed_versions;
use crate::launcher_rewrite::profiles::save_launcher_profiles;
use crate::launcher_rewrite::GAME_INSTANCE_COUNT;
use std::str::FromStr;
use std::sync::atomic::Ordering;
use std::thread::sleep;
use std::time::Duration;
use util::StripCanonicalization;

pub mod cli;
pub mod gui;
mod launcher_rewrite;
pub mod util;
mod link;

fn main() {
    let gui = cli::cli_main();

    if gui {
        gui::gui_main();
    }

    // TODO when feature `lazy_get` is implemented, make this save only if the values were initialized
    save_installed_versions();
    save_account_data();
    save_launcher_profiles();

    loop {
        if GAME_INSTANCE_COUNT.load(Ordering::SeqCst) == 0 {
            break;
        }
        // Sleep just to let the os run other tasks
        sleep(Duration::from_millis(1000));
    }
    println!("Launcher Exiting!");
}
