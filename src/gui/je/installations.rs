use crate::gui::je::{JavaEditionTab, JeGuiInteraction, JeGuiState};
use crate::gui::{GuiMessage, LauncherMessage, LauncherRenderer, LauncherTheme};
use iced::advanced::image::{FilterMethod, Handle};
use iced::alignment::Horizontal;
use iced::widget::{button, column, container, horizontal_rule, image, row, text, text_input, vertical_rule, Column, Image, Scrollable, Space};
use iced::{ContentFit, Element, Length};
use std::fmt::format;
use crate::launcher_rewrite::manifest::GAME_VERSION_MANIFEST;
use crate::launcher_rewrite::profiles::{LauncherProfile, PROFILES};

pub const SIDE_SPACER: u16 = 7;

pub fn installations_tab_content(state: &JeGuiState) -> Element<'static, LauncherMessage, LauncherTheme, LauncherRenderer> {
    let lock = PROFILES.read().unwrap();
    let elements: Vec<Element<'static, LauncherMessage, LauncherTheme, LauncherRenderer>> = lock.je_client_profiles().iter().map(map_profile_to_widget).collect();
    drop(lock);
    let scrollable: Scrollable<'static, LauncherMessage, LauncherTheme, LauncherRenderer> = Scrollable::new(Column::with_children(elements));

    let search_filter_bar = container(
        row![
            Space::new(SIDE_SPACER, Length::Fill),
            Space::new(Length::Fill, Length::Fill),
            container(row![
                image(format!("{}/assets/characters/search.png", env!("CARGO_MANIFEST_DIR"))).filter_method(FilterMethod::Linear).width(64).height(Length::Fill),
            container(text_input("Search for installations", state.profile_search_content.as_str()).width(255).on_input(|s| LauncherMessage::JavaEditionInteraction(JeGuiInteraction::SearchProfiles(s)))).center_y(Length::Fill).center_x(Length::Shrink),
            ].width(Length::Fill).height(Length::Fill)).height(Length::Fill).center_x(Length::Fill),
            container(button("+ New").padding(5).on_press(GuiMessage::JavaEditionSelectTab(JavaEditionTab::EditProfile(None)))).width(Length::Fill).align_x(Horizontal::Right).center_y(Length::Fill),
            Space::new(SIDE_SPACER, Length::Fill),
        ]
            .height(50),
    )
        .center_x(Length::Fill);

    column![search_filter_bar, horizontal_separator(), scrollable,].height(Length::Fill).width(Length::Fill).into()
}

fn map_profile_to_widget(profile: &LauncherProfile) -> Element<'static, LauncherMessage, LauncherTheme, LauncherRenderer> {
    let image = container(image(profile.icon()).content_fit(ContentFit::Contain).filter_method(FilterMethod::Linear).width(48).height(48)).center_x(Length::FillPortion(1)).center_y(Length::Fill);

    let name_text = container(text(profile.name().to_owned())).center_x(Length::FillPortion(3)).center_y(Length::Fill);

    let version_text = container(text(GAME_VERSION_MANIFEST.get_version_from_str(profile.version_name()).expect("profile has bad version id!").id())).center_x(Length::FillPortion(3)).center_y(Length::Fill);

    let edit_button = container(button("Edit").on_press(GuiMessage::JavaEditionSelectTab(JavaEditionTab::EditProfile(Some(profile.id()))))).center_y(Length::Fill).align_x(Horizontal::Right).width(Length::FillPortion(1));

    let profile_info = row![image, name_text, version_text, edit_button, Space::new(SIDE_SPACER, Length::Fill),].width(Length::Fill).height(130);
    column![profile_info, horizontal_separator(),].height(Length::Shrink).width(Length::Fill).into()
}

pub fn horizontal_separator() -> Element<'static, LauncherMessage, LauncherTheme, LauncherRenderer> {
    row![Space::new(SIDE_SPACER, Length::Fill), horizontal_rule(5), Space::new(SIDE_SPACER, Length::Fill),].height(Length::Shrink).into()
}
