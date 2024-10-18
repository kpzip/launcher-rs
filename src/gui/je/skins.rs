use crate::gui::{LauncherMessage, LauncherRenderer, LauncherTheme};
use iced::widget::text;
use iced::Element;

pub fn skins_tab_content() -> Element<'static, LauncherMessage, LauncherTheme, LauncherRenderer> {
    text("Skins").into()
}
