use crate::launcher_rewrite::authentication::account_data::LoggedInAccount;
use crate::gui::general::nice_header;
use crate::gui::je::installations::{horizontal_separator, SIDE_SPACER};
use crate::gui::{LauncherMessage, LauncherRenderer, LauncherTheme};
use crate::gui::threading::WorkerThreadTask;
use crate::util::ref_comparison;
use crate::gui::WORKER_THREAD_HANDLE;
use iced::alignment::Horizontal;
use iced::widget::horizontal_rule;
use iced::widget::{button, column, container, row, text, Column, Scrollable, Space, TextInput};
use iced::{color, Element, Length, theme};
use std::sync::mpsc::Receiver;
use std::sync::{Arc, mpsc, Mutex};
use crate::launcher_rewrite::authentication::LOGGED_IN_ACCOUNT_DATA;
use crate::launcher_rewrite::error::LauncherError;

#[derive(Clone, Debug)]
pub enum AccountInteraction {
    UsernameChanged(String),
    PasswordChanged(String),
    LoginSubmit,
    LoginSuccess,
    _2FARequired,
    InvalidCreds,
    LoginError(Arc<LauncherError>),
    Logout(String),
    LogoutAll,
    UseAccount(String),
    ChangeMenu(AccountMenu),
}

#[derive(Debug, Clone)]
pub enum AccountMenu {
    List,
    Login { username: String, password: String, waiting_for_sign_in: bool, show_invalid_creds_text: bool },
    _2FA { text: String },
}

impl AccountMenu {
    pub fn login() -> Self {
        Self::Login {
            username: Default::default(),
            password: Default::default(),
            waiting_for_sign_in: false,
            show_invalid_creds_text: false,
        }
    }

    pub fn _2fa() -> Self {
        Self::_2FA { text: Default::default() }
    }
}

impl Default for AccountMenu {
    fn default() -> Self {
        if LOGGED_IN_ACCOUNT_DATA.read().unwrap().is_empty() {
            Self::login()
        } else {
            AccountMenu::List
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct AccountTabState {
    search_query: String,
    menu: AccountMenu,
}

impl AccountTabState {
    pub fn account_data_tab(&self) -> Element<'static, LauncherMessage, LauncherTheme, LauncherRenderer> {
        match &self.menu {
            AccountMenu::List => self.accounts_list(),
            AccountMenu::Login { username, password, waiting_for_sign_in, show_invalid_creds_text } => self.sign_in(username.as_str(), password.as_str(), *waiting_for_sign_in, *show_invalid_creds_text),
            AccountMenu::_2FA { text } => self.sign_in_2fa(text.as_str()),
        }
    }

    pub fn accounts_list(&self) -> Element<'static, LauncherMessage, LauncherTheme, LauncherRenderer> {
        let lock = LOGGED_IN_ACCOUNT_DATA.read().unwrap();
        let active_acc = lock.active_account();
        let account_elements: Vec<Element<'static, LauncherMessage, LauncherTheme, LauncherRenderer>> = lock.iter().map(|a| self.account_display(a, active_acc.map(|r| ref_comparison(r, a)).unwrap_or(false))).collect();
        drop(lock);

        let list = Scrollable::new(Column::with_children(account_elements).width(Length::Fill).height(Length::Shrink)).width(Length::Fill).height(Length::Fill);

        column![nice_header("Logged In Accounts", 50f32), Space::new(Length::Fill, 8), container(button("Add Account").padding(5).on_press(LauncherMessage::AccountTabInteraction(AccountInteraction::ChangeMenu(AccountMenu::login())))).center_x(Length::Fill), Space::new(Length::Fill, 8), horizontal_separator(), list,].into()
    }

    pub fn account_display(&self, acc: &LoggedInAccount, is_active: bool) -> Element<'static, LauncherMessage, LauncherTheme, LauncherRenderer> {
        let uuid = acc.minecraft_account_info().id();

        let active_text: Element<'static, LauncherMessage, LauncherTheme, LauncherRenderer> = if is_active { text("Active").into() } else { Space::new(Length::Fill, Length::Fill).into() };

        let mut use_button = button("Use").padding(5).padding(5);

        if !is_active {
            use_button = use_button.on_press(LauncherMessage::AccountTabInteraction(AccountInteraction::UseAccount(String::from(uuid))));
        }

        let row = row![
            Space::new(SIDE_SPACER, Length::Fill),
            container(text(acc.minecraft_account_info().name().to_owned())).width(Length::FillPortion(1)).center_y(Length::Fill).align_x(Horizontal::Left),
            container(active_text).width(Length::FillPortion(1)).center_y(Length::Fill).align_x(Horizontal::Left),
            container(use_button).width(Length::Shrink).center_y(Length::Fill),
            Space::new(5, Length::Fill),
            container(button("Log Out").on_press(LauncherMessage::AccountTabInteraction(AccountInteraction::Logout(String::from(uuid)))).padding(5)).width(Length::Shrink).center_y(Length::Fill),
            Space::new(SIDE_SPACER, Length::Fill),
        ]
        .width(Length::Fill)
        .height(Length::Fill);

        column![row, horizontal_separator(),].width(Length::Fill).height(50).into()
    }

    pub fn sign_in(&self, username: &str, password: &str, waiting_for_sign_in: bool, show_wrong_creds_text: bool) -> Element<'static, LauncherMessage, LauncherTheme, LauncherRenderer> {
        let login_text = container(text("Log in to Microsoft Account")).center_x(400);
        let wrong_creds_text: Element<'static, LauncherMessage, LauncherTheme, LauncherRenderer> = if show_wrong_creds_text {
            column![container(
                text("Wrong username or password!").color(color!(255, 0, 0))
                    .width(300),
            )
            .width(400),
            Space::new(Length::Shrink, 9),
            ]
            .into()
        } else {
            Space::new(0, 0).into()
        };
        let username_input = TextInput::new("Email", username).width(400).on_input(|s| LauncherMessage::AccountTabInteraction(AccountInteraction::UsernameChanged(s)));
        let password_input = TextInput::new("Password", password).secure(true).width(400).on_input(|s| LauncherMessage::AccountTabInteraction(AccountInteraction::PasswordChanged(s)));
        let mut raw_login_button = button("Submit").padding(5);
        if !waiting_for_sign_in {
            raw_login_button = raw_login_button.on_press(LauncherMessage::AccountTabInteraction(AccountInteraction::LoginSubmit));
        }
        let submit_button = container(raw_login_button).center_x(400);

        let bottom_row = container(button("X").on_press(LauncherMessage::AccountTabInteraction(AccountInteraction::ChangeMenu(AccountMenu::List)))).width(Length::Fill).align_x(Horizontal::Left);

        column![
            container(Into::<Element<'static, LauncherMessage, LauncherTheme, LauncherRenderer>>::into(column![login_text, Space::new(Length::Shrink, 7), wrong_creds_text, username_input, Space::new(Length::Shrink, 10), password_input, Space::new(Length::Shrink, 5), submit_button,]))
                .center_x(Length::Fill)
                .center_y(Length::Fill),
            bottom_row,
        ]
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }

    pub fn sign_in_2fa(&self, text_field: &str) -> Element<'static, LauncherMessage, LauncherTheme, LauncherRenderer> {
        text("2FA WIP").into()
    }

    pub fn on_message(&mut self, action: AccountInteraction) {
        match action {
            AccountInteraction::UsernameChanged(u) => {
                if let AccountMenu::Login { username, password: _, .. } = &mut self.menu {
                    *username = u;
                }
            }
            AccountInteraction::PasswordChanged(p) => {
                if let AccountMenu::Login { username: _, password, .. } = &mut self.menu {
                    *password = p;
                }
            }
            AccountInteraction::LoginSubmit => 'login_submit: {
                if let AccountMenu::Login { username, password, waiting_for_sign_in, .. } = &mut self.menu {
                    if *waiting_for_sign_in {
                        break 'login_submit;
                    }

                    *waiting_for_sign_in = true;

                    //println!("Username: {},\nPassword: {}", username, password);

                    WORKER_THREAD_HANDLE.lock().unwrap().as_ref().unwrap().send(WorkerThreadTask::MicrosoftLogin { username: username.clone(), password: password.clone() }).unwrap()
                }
            }
            AccountInteraction::LoginSuccess => {
                self.menu = AccountMenu::List;
            }
            AccountInteraction::Logout(s) => {
                let mut lock = LOGGED_IN_ACCOUNT_DATA.write().unwrap();
                lock.remove_by_uuid(&s);
                if lock.is_empty() {
                    self.menu = AccountMenu::login();
                }
            }
            AccountInteraction::LogoutAll => {
                LOGGED_IN_ACCOUNT_DATA.write().unwrap().logout_all();
                self.menu = AccountMenu::login();
            }
            AccountInteraction::UseAccount(s) => {
                LOGGED_IN_ACCOUNT_DATA.write().unwrap().set_active_by_uuid(&s);
            }
            AccountInteraction::ChangeMenu(m) => {
                self.menu = m;
            }
            AccountInteraction::_2FARequired => {
                self.menu = AccountMenu::_2fa();
            }
            AccountInteraction::InvalidCreds => {
                if let AccountMenu::Login { username, password: pass, waiting_for_sign_in, show_invalid_creds_text } = &mut self.menu {
                    *waiting_for_sign_in = false;
                    *show_invalid_creds_text = true;
                }
            }
            AccountInteraction::LoginError(e) => {
                // Nothing for now
            }
        }
    }
}
