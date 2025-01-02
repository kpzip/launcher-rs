pub mod account;
pub mod general;
pub mod je;
mod je_server;
pub mod macros;
pub mod settings;
mod style;
pub mod game_output;
//pub mod launcher;
pub mod threading;

use crate::gui::settings::settings_gui;
use crate::gui::je::edit_installation::JeProfileChanged;
use crate::gui::je::{JavaEditionTab, JeGuiInteraction, JeGuiState};
use crate::gui::macros::button_text;
use crate::gui::settings::{on_message, SettingsGuiState, SettingsMessage};
use crate::gui::style::{dark_container_style, generic_button_style, sidebar_container_style};
use account::{AccountInteraction, AccountTabState};
use iced::futures::StreamExt;
use iced::widget::image::FilterMethod;
use iced::widget::{button, column, container, image, row, text};
use iced::{Element, Font, Length, Renderer, Size, Subscription, Task, Theme, window};
use std::cell::RefCell;
use std::panic::set_hook;
use std::sync::atomic::AtomicBool;
use std::sync::Mutex;
use native_dialog::{MessageDialog, MessageType};
use tokio::sync::mpsc::UnboundedReceiver;
use crate::gui::threading::WorkerThread;

pub const MC_FONT: Font = Font::with_name("Minecraft");
const MC_FONT_BYTES: &[u8] = include_bytes!("../../assets/minecraft_font.ttf");

pub static IS_ONLINE: AtomicBool = AtomicBool::new(true);
pub static WORKER_THREAD_HANDLE: Mutex<Option<WorkerThread>> = Mutex::new(None);



pub type LauncherExecutor = iced::executor::Default;
pub type LauncherMessage = GuiMessage;
pub type LauncherTheme = Theme;
pub type LauncherRenderer = Renderer;

pub struct LauncherGui {
    selected_menu: GameMenu,
    je_gui_state: JeGuiState,
    account_gui_state: AccountTabState,
    receiver: RefCell<Option<UnboundedReceiver<LauncherMessage>>>,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum GameMenu {
    Accounts,
    #[default]
    JavaEdition,
    JavaEditionServer,
    BedrockEdition,
    BedrockEditionServer,
    Settings(SettingsGuiState),
}

#[derive(Debug, Clone)]
pub enum GuiMessage {
    SelectMainMenu(GameMenu),
    JavaEditionSelectTab(JavaEditionTab),
    JavaEditionProfileChanged(JeProfileChanged),
    JavaEditionInteraction(JeGuiInteraction),
    AccountTabInteraction(AccountInteraction),
    SettingsTabInteraction(SettingsMessage),
}



pub struct Flags {
    receiver: UnboundedReceiver<LauncherMessage>,
}

impl Flags {
    pub fn new(receiver: UnboundedReceiver<LauncherMessage>) -> Self {
        Self { receiver }
    }
}

impl LauncherGui {

    pub fn new(flags: Flags) -> (Self, Task<LauncherMessage>) {
        (
            Self {
                selected_menu: Default::default(),
                je_gui_state: Default::default(),
                account_gui_state: Default::default(),
                receiver: RefCell::new(Some(flags.receiver)),
            },
            Task::none(),
        )
    }

    pub fn title(&self) -> String {
        String::from("launcher-rs")
    }

    pub fn update(&mut self, message: LauncherMessage) -> Task<LauncherMessage> {
        match message {
            LauncherMessage::SelectMainMenu(selection) => {
                self.selected_menu = selection;
            }
            LauncherMessage::JavaEditionSelectTab(selection) => {
                self.je_gui_state.set_current_tab(selection);
            }
            LauncherMessage::JavaEditionProfileChanged(val) => {
                self.je_gui_state.je_profile_changed(val);
            }
            LauncherMessage::JavaEditionInteraction(action) => {
                self.je_gui_state.interact(action);
            }
            LauncherMessage::AccountTabInteraction(action) => {
                self.account_gui_state.on_message(action);
            }
            LauncherMessage::SettingsTabInteraction(action) => {
                if let GameMenu::Settings(ref mut s) = self.selected_menu {
                    on_message(s, action);
                }
            }
            _ => {}
        };

        Task::none()
    }

    pub fn view(&self) -> Element<'_, LauncherMessage, LauncherTheme, LauncherRenderer> {
        let sidebar = container(column![self.sidebar_accounts_button(), row![].height(40), self.sidebar_je_button(), self.sidebar_je_server_button(), self.sidebar_be_button(), self.sidebar_be_server_button(), row![].height(Length::Fill), self.sidebar_settings_button(),].height(Length::Fill).width(Length::Fill))
            .height(Length::Fill)
            .width(270)
            .style(sidebar_container_style);

        let mc_logo = container(row![image(format!("{}/assets/mc_logo.png", env!("CARGO_MANIFEST_DIR"))).width(Length::Fill),].width(Length::Fill)).width(Length::Fill).style(dark_container_style);

        //let settings = PROFILES.lock().unwrap().as_ref().unwrap().settings();

        let content = column![
            mc_logo,
            match self.selected_menu {
                GameMenu::Accounts => self.account_gui_state.account_data_tab(),
                GameMenu::JavaEdition => self.je_gui_state.get_element(),
                GameMenu::JavaEditionServer => column![].into(),
                GameMenu::BedrockEdition => column![].into(),
                GameMenu::BedrockEditionServer => column![].into(),
                GameMenu::Settings(ref s) => settings_gui(s),
            },
        ]
        .height(Length::Fill)
        .width(Length::Fill);

        row![sidebar, content].into()
    }

    pub fn theme(&self) -> LauncherTheme {
        Theme::Dark
    }

    pub fn subscription(&self) -> Subscription<LauncherMessage> {
        //Subscription::run_with_id("led changes", self.receiver.borrow_mut())
        Subscription::run_with_id("led changes", futures_util::stream::unfold(
            self.receiver.take(),
            move |mut receiver| async move {
                let m = receiver.as_mut().unwrap().recv().await.unwrap();
                Some((m, receiver))
            })
        )
    }

    fn sidebar_accounts_button(&self) -> Element<'static, LauncherMessage, LauncherTheme, LauncherRenderer> {
        button(container(text("Account")).center_x(Length::Fill)).style(generic_button_style).on_press(LauncherMessage::SelectMainMenu(GameMenu::Accounts)).padding(10).width(Length::Fill).into()
    }

    fn sidebar_je_button(&self) -> Element<'static, LauncherMessage, LauncherTheme, LauncherRenderer> {
        button(row![
            container(image(format!("{}/assets/grass_block_side.png", env!("CARGO_MANIFEST_DIR"))).width(32).filter_method(FilterMethod::Nearest)).center_x(Length::FillPortion(1)),
            row![container(text(button_text!("Minecraft: Java Edition", self.selected_menu == GameMenu::JavaEdition))).center_x(Length::Fill)].width(Length::FillPortion(11)),
        ])
        .style(generic_button_style)
        .on_press(LauncherMessage::SelectMainMenu(GameMenu::JavaEdition))
        .padding(10)
        .width(Length::Fill)
        .into()
    }

    fn sidebar_je_server_button(&self) -> Element<'static, LauncherMessage, LauncherTheme, LauncherRenderer> {
        container(button(container(text("Minecraft: Java Edition Server")).center_x(Length::Fill)).style(generic_button_style).on_press(LauncherMessage::SelectMainMenu(GameMenu::JavaEditionServer)).padding(10).width(Length::Fill)).center_x(Length::Fill).into()
    }

    fn sidebar_be_button(&self) -> Element<'static, LauncherMessage, LauncherTheme, LauncherRenderer> {
        button(row![
            container(image(format!("{}/assets/bedrock.png", env!("CARGO_MANIFEST_DIR"))).filter_method(FilterMethod::Nearest)).center_x(Length::FillPortion(1)),
            row![container(text(button_text!("Minecraft: Bedrock Edition", self.selected_menu == GameMenu::BedrockEdition))).center_x(Length::Fill)].width(Length::FillPortion(11)),
        ])
        .style(generic_button_style)
        .on_press(LauncherMessage::SelectMainMenu(GameMenu::BedrockEdition))
        .padding(10)
        .width(Length::Fill)
        .into()
    }

    fn sidebar_be_server_button(&self) -> Element<'static, LauncherMessage, LauncherTheme, LauncherRenderer> {
        container(button(container(text("Minecraft: Bedrock Edition Server")).center_x(Length::Fill)).style(generic_button_style).on_press(LauncherMessage::SelectMainMenu(GameMenu::BedrockEditionServer)).padding(10).width(Length::Fill)).center_x(Length::Fill).into()
    }

    fn sidebar_settings_button(&self) -> Element<'static, LauncherMessage, LauncherTheme, LauncherRenderer> {
        container(button("Settings").style(generic_button_style).on_press(LauncherMessage::SelectMainMenu(GameMenu::Settings(Default::default()))).padding(10).width(Length::Fill)).center_x(Length::Fill).into()
    }
}

pub(crate) fn gui_main() {
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
        .expect("Failed to initialize gui application.");
}
