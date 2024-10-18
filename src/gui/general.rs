use crate::gui::je::installations::SIDE_SPACER;
use crate::gui::{LauncherMessage, LauncherRenderer, LauncherTheme};
use iced::widget::{container, horizontal_rule, row, text, Space};
use iced::{Element, Length};
use crate::gui::style::dark_container_style;

pub fn nice_header(content: &str, height: f32) -> Element<'static, LauncherMessage, LauncherTheme, LauncherRenderer> {
    container(
        row![
            container(horizontal_rule(5)).center_x(50).center_y(Length::Fill),
            Space::new(SIDE_SPACER, Length::Fill),
            container(text(content.to_owned())).center_x(Length::Shrink).center_y(Length::Fill),
            Space::new(SIDE_SPACER, Length::Fill),
            container(horizontal_rule(5)).center_x(50).center_y(Length::Fill),
        ]
        .width(Length::Shrink)
        .height(height),
    )
    .style(dark_container_style)
    .center_x(Length::Fill)
    .into()
}
