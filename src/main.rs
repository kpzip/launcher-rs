#![feature(iter_intersperse)]
#![feature(panic_payload_as_str)]

use std::panic::set_hook;
use crate::gui::{Flags, LauncherGui, LauncherMessage, MC_FONT};
use crate::threading::WorkerThread;
use iced::{Size, window};
//use iced_aw::BOOTSTRAP_FONT_BYTES;
use util::StripCanonicalization;
use std::str::FromStr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use std::thread::sleep;
use std::time::Duration;
use native_dialog::{MessageDialog, MessageType};
use crate::launcher_rewrite::authentication::save_account_data;
use crate::launcher_rewrite::GAME_INSTANCE_COUNT;
use crate::launcher_rewrite::installed_versions::save_installed_versions;
use crate::launcher_rewrite::profiles::save_launcher_profiles;

pub mod gui;
//pub mod launcher;
mod threading;
pub mod util;
mod launcher_rewrite;
// Global State
//pub static INSTALLED_VERSION_INFO: LazyLock<Mutex<HashMap<(String, ModLoader), (AssetIndex, LaunchProperties)>>> = LazyLock::new(|| Mutex::new(HashMap::new()));
//pub static PROFILES: Mutex<Option<LauncherProfiles>> = Mutex::new(None);
//pub static VERSION_MANIFEST: RwLock<Option<VersionManifest>> = RwLock::new(None);


pub static IS_ONLINE: AtomicBool = AtomicBool::new(true);

pub static WORKER_THREAD_HANDLE: Mutex<Option<WorkerThread>> = Mutex::new(None);

// Consts
const MC_FONT_BYTES: &[u8] = include_bytes!("../assets/minecraft_font.ttf");

fn main() {

    set_hook(Box::new(|p| {
        let mut panic_message_str = "";
        let location_str = p.location().map(|loc| loc.to_string()).unwrap_or(String::new());
        if let Some(pstr) = p.payload_as_str() {
            panic_message_str = pstr;
        }

        let err_dialog = MessageDialog::new().set_type(MessageType::Error).set_title("Launcher Error").set_text(format!("Launcher panicked: {}\nPanic Location: {}", panic_message_str, location_str).as_str()).show_alert();
    }));

    let (message_send, message_receive) = tokio::sync::mpsc::unbounded_channel::<LauncherMessage>();

    let mut lock = WORKER_THREAD_HANDLE.lock().unwrap();
    lock.replace(WorkerThread::new(message_send));
    drop(lock);

    let window_settings = window::Settings {
        size: Size::new(1280_f32, 720_f32),
        resizable: true,
        decorations: true,
        ..Default::default()
    };

    iced::application(LauncherGui::title, LauncherGui::update, LauncherGui::view)
        .window(window_settings)
        .font(MC_FONT_BYTES)
        .default_font(MC_FONT)
        .subscription(LauncherGui::subscription)
        .run_with(move || LauncherGui::new(Flags::new(message_receive)))
        .expect("GG");

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
