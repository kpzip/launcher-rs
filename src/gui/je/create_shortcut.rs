use iced::Element;
use iced::widget::text;
use crate::gui::{LauncherMessage, LauncherRenderer, LauncherTheme};
use crate::launcher_rewrite::profiles::ModLoader;

#[derive(Default)]
pub struct ShortcutInfo {
    profile_id: u128,
    game_version: String,
    loader_version: String,
    loader: ModLoader,
    memory: u16,
}

pub fn create_shortcut_gui() -> Element<'static, LauncherMessage, LauncherTheme, LauncherRenderer> {
    text("Create Shortcut").into()
}