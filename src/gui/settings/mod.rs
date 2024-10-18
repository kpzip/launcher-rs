use crate::gui::general::nice_header;
use crate::gui::{LauncherMessage, LauncherRenderer, LauncherTheme};
use iced::alignment::Horizontal;
use iced::widget::{column, toggler};
use iced::widget::Space;
use iced::widget::{container, row, scrollable, text, Scrollable, Toggler};
use iced::{Element, Length};
use std::ops::{Deref, DerefMut};
use crate::launcher_rewrite::profiles::{LauncherSettings, PROFILES};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SettingsGuiState {
    settings: LauncherSettings,
}

impl Deref for SettingsGuiState {
    type Target = LauncherSettings;

    fn deref(&self) -> &Self::Target {
        &self.settings
    }
}

impl DerefMut for SettingsGuiState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.settings
    }
}

impl Default for SettingsGuiState {
    fn default() -> Self {
        Self { settings: *PROFILES.read().unwrap().settings() }
    }
}

#[derive(Debug, Clone)]
pub enum SettingsMessage {
    HistoricalChanged(bool),
    SnapshotsChanged(bool),
    Save,
}

pub fn settings_gui(settings: &SettingsGuiState) -> Element<'static, LauncherMessage, LauncherTheme, LauncherRenderer> {
    let scroll = Scrollable::new(
        column![
            Space::new(Length::Fill, 75),
            setting_center_aligned("Show Historical Versions", toggler(settings.enable_historical).on_toggle(|b| LauncherMessage::SettingsTabInteraction(SettingsMessage::HistoricalChanged(b)))),
            Space::new(Length::Fill, 30),
            setting_center_aligned("Show Snapshot Versions", Toggler::new(settings.enable_snapshots).on_toggle(|b| LauncherMessage::SettingsTabInteraction(SettingsMessage::SnapshotsChanged(b)))),
        ]
        .width(Length::Fill),
    );

    let main_column = column![nice_header("Settings", 50f32), scroll].width(Length::Fill);

    main_column.into()
}

pub fn setting_center_aligned(name: &str, element: impl Into<Element<'static, LauncherMessage, LauncherTheme, LauncherRenderer>>) -> Element<'static, LauncherMessage, LauncherTheme, LauncherRenderer> {
    container(row![container(text(name.to_owned())).width(200).center_y(Length::Shrink).align_x(Horizontal::Left), container(element).width(200).center_y(Length::Shrink).align_x(Horizontal::Right),]).center_x(Length::Fill).into()
}

pub fn on_message(settings: &mut SettingsGuiState, message: SettingsMessage) {
    match message {
        SettingsMessage::HistoricalChanged(b) => {
            settings.enable_historical = b;
        }
        SettingsMessage::SnapshotsChanged(b) => {
            settings.enable_snapshots = b;
        }
        SettingsMessage::Save => {
            PROFILES.write().unwrap().settings_mut().set_settings(settings.settings);
        }
    }
}
