use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use iced::{Element, Size, Subscription, Task, time, window};
use iced::widget::text;
use crate::gui::{LauncherGui, LauncherMessage, LauncherRenderer, LauncherTheme, MC_FONT};
use crate::MC_FONT_BYTES;

#[must_use]
pub fn open_game_output_window() -> Arc<Mutex<String>> {
    let window = GameOutputGuiState::new();
    let handle = window.get_output();
    thread::Builder::new().name("Game Output GUI Thread".to_owned()).spawn(move || {
        let window_settings = window::Settings {
            size: Size::new(1280_f32, 720_f32),
            resizable: true,
            decorations: true,
            ..Default::default()
        };


        iced::application(GameOutputGuiState::title, GameOutputGuiState::update, GameOutputGuiState::view)
            .window(window_settings)
            .font(MC_FONT_BYTES)
            .default_font(MC_FONT)
            .subscription(GameOutputGuiState::subscription)
            .run()
            .expect("Failed to load game output gui")
    }).expect("Failed to start game output gui thread");
    handle
}

#[derive(Debug, Clone)]
pub struct GameOutputGuiState {
    output: Arc<Mutex<String>>,
}

#[derive(Debug, Clone)]
pub enum GameOutputMessage {
    OutputReceived,
}

impl GameOutputGuiState {

    fn new() -> Self {
        Self {
            output: Arc::new(Mutex::new(String::new()))
        }
    }

    fn get_output(&self) -> Arc<Mutex<String>> {
        self.output.clone()
    }

    fn title(&self) -> String {
        String::from("Minecraft Game Output")
    }

    pub fn update(&mut self, message: GameOutputMessage) -> Task<GameOutputMessage> {
        Task::none()
    }

    fn view(&self) -> Element<'_, GameOutputMessage, LauncherTheme, LauncherRenderer> {
        text(self.output.lock().unwrap().as_str().to_owned()).into()
    }

    pub fn subscription(&self) -> Subscription<GameOutputMessage> {
        time::every(Duration::from_millis(100)).map(|_i| GameOutputMessage::OutputReceived)
    }

}

impl Default for GameOutputGuiState {
    fn default() -> Self {
        Self::new()
    }
}