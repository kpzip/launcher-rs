pub mod edit_installation;
mod home;
pub mod installations;
mod skins;

use crate::gui::je::edit_installation::{edit_installations_tab_content, JeProfileChanged};
use crate::gui::je::home::home_tab_content;
use crate::gui::je::installations::installations_tab_content;
use crate::gui::je::skins::skins_tab_content;
use crate::gui::{GuiMessage, LauncherGui, LauncherMessage, LauncherRenderer, LauncherTheme, MC_FONT};
use crate::threading::WorkerThreadTask;
use crate::WORKER_THREAD_HANDLE;
use iced::alignment::{Horizontal, Vertical};
use iced::widget::{button, column, container, row, text, Space, markdown};
use iced::{Element, Length};
use crate::gui::style::{dark_container_style, generic_button_style};
use crate::launcher_rewrite::profiles::{LauncherProfile, PROFILES};

pub struct JeGuiState {
    current_tab: JavaEditionTab,
    profile_edit: LauncherProfile,
    selected_profile_id: u128,
    is_launching: bool,
    profile_search_content: String,
}

impl Default for JeGuiState {
    fn default() -> Self {
        Self {
            current_tab: Default::default(),
            profile_edit: Default::default(),
            selected_profile_id: PROFILES.read().unwrap().settings().selected_profile_id(),
            is_launching: false,
            profile_search_content: Default::default(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum JeGuiInteraction {
    SelectedProfileChanged(u128),
    SearchProfiles(String),
    ClickLink(markdown::Url),
    LaunchGame,
}

#[derive(Default, Debug, Clone, Copy)]
pub enum JavaEditionTab {
    #[default]
    Home,
    Skins,
    Installations,
    EditProfile(Option<u128>),
}

impl JeGuiState {
    pub fn get_element(&self) -> Element<'_, LauncherMessage, LauncherTheme, LauncherRenderer> {
        let top_bar = container(
            row![
                button("Home").padding(10).style(generic_button_style).on_press(GuiMessage::JavaEditionSelectTab(JavaEditionTab::Home)),
                Space::new(10, Length::Fill),
                button("Installations").padding(10).style(generic_button_style).on_press(GuiMessage::JavaEditionSelectTab(JavaEditionTab::Installations)),
                Space::new(10, Length::Fill),
                button("Skins").padding(10).style(generic_button_style).on_press(GuiMessage::JavaEditionSelectTab(JavaEditionTab::Skins)),
            ]
            .width(Length::Shrink)
            .height(Length::Shrink),
        )
        .width(Length::Fill)
        .height(Length::Shrink)
        .align_x(Horizontal::Center)
        .style(dark_container_style);
        column![top_bar, self.get_content(),].width(Length::Fill).height(Length::Fill).into()
    }

    fn get_content(&self) -> Element<'_, LauncherMessage, LauncherTheme, LauncherRenderer> {
        match self.current_tab {
            JavaEditionTab::Home => home_tab_content(&self),
            JavaEditionTab::Skins => skins_tab_content(),
            JavaEditionTab::Installations => installations_tab_content(self),
            JavaEditionTab::EditProfile(id) => edit_installations_tab_content(&self.profile_edit),
        }
    }

    pub fn interact(&mut self, interaction: JeGuiInteraction) {
        match interaction {
            JeGuiInteraction::SelectedProfileChanged(id) => {
                self.selected_profile_id = id;
            }
            JeGuiInteraction::LaunchGame => {
                WORKER_THREAD_HANDLE.lock().unwrap().as_ref().unwrap().send(WorkerThreadTask::LaunchGame(self.selected_profile_id)).expect("TODO: panic message");
            }
            JeGuiInteraction::SearchProfiles(s) => {
                self.profile_search_content = s;
            }
            JeGuiInteraction::ClickLink(url) => {
                //println!("Link Clicked: {}", url)
                let _ = open::that(url.to_string()).inspect_err(|e| {
                    eprintln!("Failed to open link: {e}");
                });
            }
        }
    }

    pub(crate) fn je_profile_changed(&mut self, change: JeProfileChanged) {
        match change {
            JeProfileChanged::VersionChanged(ver) => {
                self.profile_edit.set_version_name(ver);
            }
            JeProfileChanged::ModLoaderChanged(loader) => {
                self.profile_edit.set_mod_loader(loader);
            }
            JeProfileChanged::NameChanged(name) => {
                self.profile_edit.set_name(name);
            }
            JeProfileChanged::MemoryChanged(mem) => {
                self.profile_edit.set_memory(mem);
            }
            JeProfileChanged::DirectoryChanged(dir) => {
                self.profile_edit.set_mc_directory(dir);
            }
            JeProfileChanged::WidthChanged(w) => {
                if w.is_empty() {
                    self.profile_edit.set_width(None);
                } else if let Ok(width) = w.parse() {
                    self.profile_edit.set_width(Some(width))
                }
            }
            JeProfileChanged::HeightChanged(h) => {
                if h.is_empty() {
                    self.profile_edit.set_height(None);
                } else if let Ok(height) = h.parse() {
                    self.profile_edit.set_height(Some(height))
                }
            }
            JeProfileChanged::JvmArgsChanged(args) => {
                self.profile_edit.set_additional_args(if args.is_empty() { None } else { Some(args) });
            }
            JeProfileChanged::Save => {
                if let JavaEditionTab::EditProfile(id) = self.current_tab {
                    if let Some(id) = id {
                        let mut lock = PROFILES.write().unwrap();

                        match lock.je_client_profiles_mut().iter_mut().find(|p| p.id() == id) {
                            Some(profile_ref) => {
                                *profile_ref = self.profile_edit.clone();
                            }
                            None => {
                                lock.je_client_profiles_mut().push(self.profile_edit.clone());
                            }
                        };

                        self.current_tab = JavaEditionTab::Installations;
                    }
                }
            }
            JeProfileChanged::IconChanged(i) => {
                self.profile_edit.set_icon(i);
            }
            JeProfileChanged::LoaderVersionChanged(new_version) => {
                self.profile_edit.set_mod_loader_version(new_version);
            }
        }
    }

    pub fn current_tab(&self) -> JavaEditionTab {
        self.current_tab
    }

    pub fn set_current_tab(&mut self, current_tab: JavaEditionTab) {

        if let JavaEditionTab::EditProfile(id) = current_tab {

            if let Some(id) = id {
                let profiles = PROFILES.read().unwrap();
                let profile = profiles.je_client_profiles().iter().find(|p| p.id() == id).unwrap();
                self.profile_edit = profile.clone();
                self.current_tab = current_tab;
            }
            else {
                // New Profile
                let profile = LauncherProfile::default();
                let id = profile.id();
                self.profile_edit = profile;
                self.current_tab = JavaEditionTab::EditProfile(Some(id));

            }

        }
        else {
            self.current_tab = current_tab;
        }
    }

    pub fn selected_profile_id(&self) -> u128 {
        self.selected_profile_id
    }
}
