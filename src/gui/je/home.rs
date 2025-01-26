use crate::gui::je::{JeGuiInteraction, JeGuiState};
use crate::gui::{LauncherMessage, LauncherRenderer, LauncherTheme, MC_FONT};
use iced::alignment::{Horizontal, Vertical};
use iced::futures::future::select;
use iced::widget::{button, column, container, image, row, text, Column, PickList, Scrollable, Space, markdown, rich_text, scrollable};
use iced::{Element, Font, Length, padding, Pixels, Theme};
use iced_aw::DropDown;
use std::fmt::Display;
use std::sync::Arc;
use iced::widget::markdown::{Catalog, Item, Settings, Style, Url};
use crate::gui::general::nice_header;
use crate::gui::style::{dark_container_style, play_button_style};
use crate::launcher_rewrite::launch_properties::Rule;
use crate::launcher_rewrite::manifest::GAME_VERSION_MANIFEST;
use crate::launcher_rewrite::patch_notes::JAVA_EDITION_PATCH_NOTES;
use crate::launcher_rewrite::profiles::{LauncherProfile, PROFILES};

#[derive(Clone, PartialEq)]
pub struct ProfileSelectorElement {
    name: String,
    version: String,
    id: u128,
}

impl ProfileSelectorElement {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn id(&self) -> u128 {
        self.id
    }
}

impl Display for ProfileSelectorElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {}", &self.name, &self.version)
    }
}

impl From<&LauncherProfile> for ProfileSelectorElement {
    fn from(value: &LauncherProfile) -> Self {
        Self { name: value.name().into(), version: GAME_VERSION_MANIFEST.sanitize_version_name(value.version_name(), value.mod_loader()).into(), id: value.id() }
    }
}

pub fn home_tab_content(state: &JeGuiState) -> Element<'static, LauncherMessage, LauncherTheme, LauncherRenderer> {
    let play_button = button(text("Play").align_y(Vertical::Center).align_x(Horizontal::Center).size(40)).width(300).height(100).style(play_button_style).on_press(LauncherMessage::JavaEditionInteraction(JeGuiInteraction::LaunchGame));

    let profiles_lock = PROFILES.read().unwrap();
    let profiles = profiles_lock.je_client_profiles();
    //let profile_picker = Column::with_children(profiles.iter().map(get_list_profile_display));
    let list: Vec<ProfileSelectorElement> = profiles.iter().map(|p| ProfileSelectorElement::from(p)).collect();
    let selected_element = list.iter().find(|p| p.id() == state.selected_profile_id()).map(Clone::clone);

    let dropdown = PickList::new(list, selected_element, |p: ProfileSelectorElement| LauncherMessage::JavaEditionInteraction(JeGuiInteraction::SelectedProfileChanged(p.id())));

    //let profile = get_list_profile_display(profiles.iter().find(|p| p.id() == state.selected_profile_id()).unwrap());
    drop(profiles_lock);

    //let dropdown = DropDown::new(profile, profile_picker, true);

    let play_button_row = container(row![container(dropdown).center_y(Length::Fill).center_x(Length::Fill), play_button, Space::new(Length::Fill, Length::Shrink),].width(Length::Fill).height(100)).style(dark_container_style);

    let patch_notes_bar = nice_header("Patch Notes", 30f32);

    let patch_notes = row![
        Space::new(10, Length::Fill),
        Column::with_children(map_patch_notes(5)).width(Length::Fill).height(Length::Shrink),
        Space::new(10, Length::Fill),
    ].width(Length::Fill).height(Length::Shrink);

    column![patch_notes_bar, Scrollable::new(patch_notes).height(Length::Fill), play_button_row,].width(Length::Fill).height(Length::Fill).into()
}

fn get_list_profile_display(profile: &LauncherProfile) -> Element<'static, LauncherMessage, LauncherTheme, LauncherRenderer> {
    row![container(image(profile.icon())).width(Length::FillPortion(1)).center_y(Length::Fill).align_x(Horizontal::Left), container(text(profile.name().to_owned())).width(Length::FillPortion(3)).center_y(Length::Fill).align_x(Horizontal::Left),].height(50).width(240).into()
}

fn map_patch_notes(n: usize) -> impl Iterator<Item = Element<'static, LauncherMessage, LauncherTheme, LauncherRenderer>> {
    let first: [Element<'static, LauncherMessage, LauncherTheme, LauncherRenderer>; 1] = [Space::new(Length::Fill, 10).into()];
    let last: [Element<'static, LauncherMessage, LauncherTheme, LauncherRenderer>; 1] = [container(button(text("Load more"))).center_x(Length::Fill).center_y(50).into()];
    first.into_iter().chain(JAVA_EDITION_PATCH_NOTES.n_segments(n).map(|s| {

        column![
            view(&markdown::parse(s).collect::<Vec<markdown::Item>>(), markdown::Settings::default(), markdown::Style::from_palette(Theme::Dark.palette())).map(|url| LauncherMessage::JavaEditionInteraction(JeGuiInteraction::ClickLink(url))),
            Space::new(Length::Fill, 10),
            iced::widget::Rule::horizontal(5),
            Space::new(Length::Fill, 10),
        ].into()
    })).chain(last.into_iter())
}

pub fn view<'a, 'b, Theme, Renderer>(
    items: impl IntoIterator<Item=&'b Item>,
    settings: Settings,
    style: Style,
) -> iced::advanced::graphics::core::Element<'a, Url, Theme, Renderer>
where
    Theme: Catalog + 'a,
    Renderer: iced::advanced::graphics::core::text::Renderer<Font=Font> + 'a,
{
    let Settings {
        text_size,
        h1_size,
        h2_size,
        h3_size,
        h4_size,
        h5_size,
        h6_size,
        code_size,
    } = settings;

    let spacing = text_size * 0.625;

    let blocks = items.into_iter().enumerate().map(|(i, item)| match item {
        Item::Heading(level, heading) => {
            container(rich_text(heading.spans(style)).size(match level {
                pulldown_cmark::HeadingLevel::H1 => h1_size,
                pulldown_cmark::HeadingLevel::H2 => h2_size,
                pulldown_cmark::HeadingLevel::H3 => h3_size,
                pulldown_cmark::HeadingLevel::H4 => h4_size,
                pulldown_cmark::HeadingLevel::H5 => h5_size,
                pulldown_cmark::HeadingLevel::H6 => h6_size,
            }))
                .padding(padding::top(if i > 0 {
                    text_size / 2.0
                } else {
                    Pixels::ZERO
                }))
                .into()
        }
        Item::Paragraph(paragraph) => {
            rich_text(paragraph.spans(style)).size(text_size).into()
        }
        Item::List { start: None, items } => {
            column(items.iter().map(|items| {
                row![text("•").size(text_size), view(items, settings, style)]
                    .spacing(spacing)
                    .into()
            }))
                .spacing(spacing)
                .into()
        }
        Item::List {
            start: Some(start),
            items,
        } => column(items.iter().enumerate().map(|(i, items)| {
            row![
                text!("{}.", i as u64 + *start).size(text_size),
                view(items, settings, style)
            ]
                .spacing(spacing)
                .into()
        }))
            .spacing(spacing)
            .into(),
        Item::CodeBlock(code) => container(
            scrollable(
                container(
                    rich_text(code.spans(style))
                        .font(Font::MONOSPACE)
                        .size(code_size),
                )
                    .padding(spacing.0 / 2.0),
            )
                .direction(scrollable::Direction::Horizontal(
                    scrollable::Scrollbar::default()
                        .width(spacing.0 / 2.0)
                        .scroller_width(spacing.0 / 2.0),
                )),
        )
            .width(Length::Fill)
            .padding(spacing.0 / 2.0)
            .class(Theme::code_block())
            .into(),
    });

    Element::new(column(blocks).width(Length::Fill).spacing(text_size))
}
