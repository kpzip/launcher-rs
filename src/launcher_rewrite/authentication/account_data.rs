use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;
use std::ops::Deref;
use std::path::Path;
use crate::launcher_rewrite::authentication::LOGGED_IN_ACCOUNT_DATA;
use crate::launcher_rewrite::path_handler::TOKENS_FILE_PATH;
use crate::launcher_rewrite::util::config_file::{load_from_file, save_to_file};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct AccountData {
    logged_in_accounts: Vec<LoggedInAccount>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    active_account_id: Option<usize>,
}

impl AccountData {
    pub const fn new() -> Self {
        Self { logged_in_accounts: Vec::new(), active_account_id: None }
    }

    pub fn active_account(&self) -> Option<&LoggedInAccount> {
        self.logged_in_accounts.get(self.active_account_id?)
    }

    pub fn add_account_and_set_active(&mut self, acc: LoggedInAccount) {
        self.active_account_id = Some(self.logged_in_accounts.len());
        self.logged_in_accounts.push(acc);
    }

    pub fn add_account(&mut self, acc: LoggedInAccount) {
        if self.logged_in_accounts.len() == 0 {
            self.active_account_id = Some(0)
        }
        self.logged_in_accounts.push(acc);
    }

    pub fn is_empty(&self) -> bool {
        self.logged_in_accounts.is_empty()
    }

    pub fn logout_all(&mut self) {
        self.logged_in_accounts.clear();
        self.active_account_id = None;
    }

    pub fn get_by_uuid(&self, uuid: &str) -> Option<&LoggedInAccount> {
        self.logged_in_accounts.iter().find(|a| a.minecraft_account_info.id == uuid)
    }

    fn index_of(&self, uuid: &str) -> Option<usize> {
        let mut i = 0;
        for acc in &self.logged_in_accounts {
            if acc.minecraft_account_info.id == uuid {
                return Some(i);
            }
            i += 1;
        }
        None
    }

    pub fn remove_by_uuid(&mut self, uuid: &str) {
        if let Some(index) = self.index_of(uuid) {
            if let Some(active_index) = self.active_account_id {
                match active_index.cmp(&index) {
                    Ordering::Less => {}
                    Ordering::Equal => self.active_account_id = None,
                    Ordering::Greater => self.active_account_id = Some(active_index - 1),
                }
            }
            self.logged_in_accounts.remove(index);
        }
    }

    pub fn set_active_by_uuid(&mut self, uuid: &str) {
        if let Some(index) = self.index_of(uuid) {
            self.active_account_id = Some(index);
        }
    }
}

impl Deref for AccountData {
    type Target = Vec<LoggedInAccount>;

    fn deref(&self) -> &Self::Target {
        &self.logged_in_accounts
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoggedInAccount {
    microsoft_token_info: MicrosoftTokenInfo,
    xbox_live_token_info: XboxLiveTokenInfo,
    xsts_token_info: XboxLiveTokenInfo,
    minecraft_token_info: MinecraftTokenInfo,
    #[serde(flatten)]
    minecraft_account_info: MinecraftAccountInfo,
}

impl LoggedInAccount {
    #[inline(always)]
    pub fn new(microsoft_token_info: MicrosoftTokenInfo, xbox_live_token_info: XboxLiveTokenInfo, xsts_token_info: XboxLiveTokenInfo, minecraft_token_info: MinecraftTokenInfo, minecraft_account_info: MinecraftAccountInfo) -> Self {
        Self {
            microsoft_token_info,
            xbox_live_token_info,
            xsts_token_info,
            minecraft_token_info,
            minecraft_account_info,
        }
    }

    pub fn microsoft_token_info(&self) -> &MicrosoftTokenInfo {
        &self.microsoft_token_info
    }

    pub fn xbox_live_token_info(&self) -> &XboxLiveTokenInfo {
        &self.xbox_live_token_info
    }

    pub fn xsts_token_info(&self) -> &XboxLiveTokenInfo {
        &self.xsts_token_info
    }

    pub fn minecraft_token_info(&self) -> &MinecraftTokenInfo {
        &self.minecraft_token_info
    }

    pub fn minecraft_account_info(&self) -> &MinecraftAccountInfo {
        &self.minecraft_account_info
    }

    pub fn minecraft_token(&self) -> &str {
        &self.minecraft_token_info.access_token
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MicrosoftTokenInfo {
    access_token: String,
    token_type: String,
    expires_in: u64,
    scope: String,
    refresh_token: String,
    user_id: String,
}

impl MicrosoftTokenInfo {
    pub fn from_map(map: HashMap<&str, &str>) -> Self {
        let access_token = *map.get("access_token").unwrap();
        let token_type = *map.get("token_type").unwrap();
        let expires_in: u64 = map.get("expires_in").unwrap().parse::<u64>().unwrap();
        let scope = *map.get("scope").unwrap();
        let refresh_token = *map.get("refresh_token").unwrap();
        let user_id = *map.get("user_id").unwrap();

        Self {
            access_token: access_token.into(),
            token_type: token_type.into(),
            expires_in,
            scope: scope.into(),
            refresh_token: refresh_token.into(),
            user_id: user_id.into(),
        }
    }

    pub fn access_token(&self) -> &str {
        &self.access_token
    }

    pub fn token_type(&self) -> &str {
        &self.token_type
    }

    pub fn expires_in(&self) -> u64 {
        self.expires_in
    }

    pub fn scope(&self) -> &str {
        &self.scope
    }

    pub fn refresh_token(&self) -> &str {
        &self.refresh_token
    }

    pub fn user_id(&self) -> &str {
        &self.user_id
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct XboxLiveTokenInfo {
    issue_instant: String,
    not_after: String,
    token: String,
    display_claims: DisplayClaims,
}

impl XboxLiveTokenInfo {
    pub fn issue_instant(&self) -> &str {
        &self.issue_instant
    }

    pub fn not_after(&self) -> &str {
        &self.not_after
    }

    pub fn token(&self) -> &str {
        &self.token
    }
}

impl Deref for XboxLiveTokenInfo {
    type Target = DisplayClaims;

    fn deref(&self) -> &Self::Target {
        &self.display_claims
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DisplayClaims {
    xui: Vec<UserHash>,
}

impl Deref for DisplayClaims {
    type Target = Vec<UserHash>;

    fn deref(&self) -> &Self::Target {
        &self.xui
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserHash {
    uhs: String,
}

impl Deref for UserHash {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.uhs
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MinecraftTokenInfo {
    username: String,
    access_token: String,
    token_type: String,
    expires_in: u64,
}

impl MinecraftTokenInfo {
    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn access_token(&self) -> &str {
        &self.access_token
    }

    pub fn token_type(&self) -> &str {
        &self.token_type
    }

    pub fn expires_in(&self) -> u64 {
        self.expires_in
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MinecraftAccountInfo {
    name: String,
    id: String,
    skins: Vec<SkinData>,
    capes: Vec<CapeData>,
}

impl MinecraftAccountInfo {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn skins(&self) -> &Vec<SkinData> {
        &self.skins
    }

    pub fn capes(&self) -> &Vec<CapeData> {
        &self.capes
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SkinData {
    id: String,
    state: String,
    url: String,
    variant: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    alias: Option<String>,
}

impl SkinData {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn state(&self) -> &str {
        &self.state
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn variant(&self) -> &str {
        &self.variant
    }

    pub fn alias(&self) -> Option<&str> {
        match self.alias {
            None => None,
            Some(ref s) => Some(s.as_str()),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CapeData {
    id: String,
    state: String,
    url: String,
    alias: String,
}

impl CapeData {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn state(&self) -> &str {
        &self.state
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn alias(&self) -> &str {
        &self.alias
    }
}

pub fn get_account_data() -> AccountData {
    #[cfg(debug_assertions)]
    return load_from_file(TOKENS_FILE_PATH.as_path(), true);
    #[cfg(not(debug_assertions))]
    return load_from_file(TOKENS_FILE_PATH.as_path(), false);
}
