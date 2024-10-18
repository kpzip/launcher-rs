use crate::gui::macros::gradient;
use iced::advanced::graphics::image::image_rs::imageops::vertical_gradient;
use iced::gradient::Linear;
use iced::widget::{button, container, text};
use iced::{color, Background, Gradient, Radians, Theme, Vector, Shadow, Color};

pub fn sidebar_container_style<Theme>(_: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Gradient(gradient!(0.0; (0x2a, 0x2a, 0x2a), (0x3a, 0x3a, 0x3a)))),
        ..Default::default()
    }
}

pub fn play_button_style<Theme>(_: &Theme, status: button::Status) -> button::Style {

    const ACTIVE_COLOR: Color = color!(0xff, 0xff, 0xff);
    const ACTIVE_BG: Option<Background> = None;

    match status {
        button::Status::Active => button::Style { text_color: ACTIVE_COLOR, background: ACTIVE_BG, ..Default::default() },
        button::Status::Hovered => button::Style { shadow: Shadow { offset: Vector::new(0.0, 1.0), ..Default::default() }, text_color: ACTIVE_COLOR, background: ACTIVE_BG, ..Default::default() },
        button::Status::Pressed => button::Style { text_color: ACTIVE_COLOR, background: ACTIVE_BG, ..Default::default() },
        button::Status::Disabled => button::Style {
            background: ACTIVE_BG.map(|background| match background {
                Background::Color(color) => Background::Color(Color { a: color.a * 0.5, ..color }),
                Background::Gradient(gradient) => Background::Gradient(gradient.scale_alpha(0.5)),
            }),
            text_color: Color { a: ACTIVE_COLOR.a * 0.5, ..ACTIVE_COLOR },
            ..Default::default()
        },
    }
}

pub fn dark_container_style<Theme>(_: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(color!(0x1a, 0x1a, 0x1a))),
        ..Default::default()
    }
}

pub fn generic_button_style<Theme>(_: &Theme, status: button::Status) -> button::Style {

    const ACTIVE_COLOR: Color = color!(0xff, 0xff, 0xff);
    const HOVERED_COLOR: Color = color!(0xb0, 0xb0, 0xff);
    const PRESSED_COLOR: Color = color!(0xa0, 0xa0, 0xf0);
    const ACTIVE_BG: Option<Background> = None;

    match status {
        button::Status::Active => button::Style { text_color: ACTIVE_COLOR, background: ACTIVE_BG, ..Default::default() },
        button::Status::Hovered => button::Style { shadow: Shadow { offset: Vector::new(0.0, 1.0), ..Default::default() }, text_color: HOVERED_COLOR, background: ACTIVE_BG, ..Default::default() },
        button::Status::Pressed => button::Style { text_color: PRESSED_COLOR, background: ACTIVE_BG, ..Default::default() },
        button::Status::Disabled => button::Style { text_color: ACTIVE_COLOR, background: ACTIVE_BG, ..Default::default() },
    }
}
