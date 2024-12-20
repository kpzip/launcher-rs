use std::collections::HashMap;
use std::env::consts::{ARCH, OS};
use std::num::NonZeroU64;
use std::ops::{BitAnd, Deref};
use std::path::Path;
use std::slice::from_ref;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Map;
use crate::launcher_rewrite::version_type::VersionType;



#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub (in crate::launcher_rewrite::launch_properties) struct ClientJson<'file> {
    pub inherits_from: Option<&'file str>,
    pub arguments: Arguments<'file>,
    pub asset_index: Option<AssetIndexInfo<'file>>,
    pub assets: Option<&'file str>,
    pub compliance_level: Option<u8>,
    pub downloads: Option<MainDownloads<'file>>,
    #[serde(rename = "id")]
    pub version_id: &'file str,
    pub java_version: Option<JavaInfo<'file>>,
    #[serde(default)]
    pub libraries: Vec<Library<'file>>,
    pub logging: Option<LoggingInfo<'file>>,
    pub main_class: Option<&'file str>,
    pub minimum_launcher_version: Option<u8>,
    pub release_time: DateTime<Utc>,
    pub time: DateTime<Utc>,
    #[serde(rename = "type")]
    pub release_type: Option<VersionType>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub (in crate::launcher_rewrite::launch_properties) struct Arguments<'file> {
    #[serde(borrow, default)]
    pub game: Vec<Arg<'file>>,
    #[serde(borrow, default)]
    pub jvm: Vec<Arg<'file>>,
}

impl<'file> Arguments<'file> {
    pub fn game(&self) -> &Vec<Arg> {
        &self.game
    }

    pub fn jvm(&self) -> &Vec<Arg> {
        &self.jvm
    }

    pub fn into_parts(self) -> (Vec<Arg<'file>>, Vec<Arg<'file>>) {
        (self.game, self.jvm)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub (in crate::launcher_rewrite::launch_properties) struct AssetIndexInfo<'file> {
    pub id: &'file str,
    pub total_size: Option<NonZeroU64>,
    #[serde(flatten)]
    pub download_info: DownloadInfo<'file>,
}

/*impl<'file> AssetIndexInfo<'file> {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn total_size(&self) -> Option<NonZeroU64> {
        self.total_size
    }
}

impl<'file> Deref for AssetIndexInfo<'file> {
    type Target = DownloadInfo<'file>;

    fn deref(&self) -> &Self::Target {
        &self.download_info
    }
}*/

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub (in crate::launcher_rewrite::launch_properties) struct MainDownloads<'file> {
    #[serde(borrow)]
    pub client: DownloadInfo<'file>,
    #[serde(borrow)]
    pub client_mappings: Option<DownloadInfo<'file>>,
    #[serde(borrow)]
    pub server: DownloadInfo<'file>,
    #[serde(borrow)]
    pub server_mappings: Option<DownloadInfo<'file>>,
}

impl<'file> MainDownloads<'file> {
    pub fn client(&self) -> &DownloadInfo {
        &self.client
    }

    pub fn client_mappings(&self) -> Option<&DownloadInfo> {
        self.client_mappings.as_ref()
    }

    pub fn server(&self) -> &DownloadInfo {
        &self.server
    }

    pub fn server_mappings(&self) -> Option<&DownloadInfo> {
        self.server_mappings.as_ref()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub (in crate::launcher_rewrite::launch_properties) struct JavaInfo<'file> {
    component: &'file str,
    major_version: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub (in crate::launcher_rewrite::launch_properties) struct DownloadInfo<'file> {
    pub sha1: Option<&'file str>,
    pub size: Option<NonZeroU64>,
    pub url: &'file str,
}

impl<'file> DownloadInfo<'file> {
    pub fn sha1(&self) -> Option<&str> {
        self.sha1
    }

    pub fn size(&self) -> Option<NonZeroU64> {
        self.size
    }

    pub fn url(&self) -> &str {
        &self.url
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub (in crate::launcher_rewrite::launch_properties) struct Library<'file> {
    #[serde(flatten)]
    pub format: LibraryFormat<'file>,
    pub name: &'file str,
    #[serde(default)]
    pub rules: Rules<'file>,
}

impl<'file> Library<'file> {
    pub fn name(&self) -> &str {
        self.name
    }

    /*pub fn rules_match(&self) -> bool {
        if !self.rules.is_empty() {
            for rule in &self.rules {
                if rule.matches() {
                    return true;
                }
            }
            return false;
        }
        return true;
    }*/
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub (in crate::launcher_rewrite::launch_properties) enum LibraryFormat<'file> {
    Artifact {
        #[serde(borrow)]
        downloads: LibDownload<'file>
    },
    Plain {
        #[serde(borrow)]
        #[serde(flatten)]
        info: DownloadInfo<'file>,
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub (in crate::launcher_rewrite::launch_properties) struct LibDownload<'file> {
    #[serde(borrow)]
    pub artifact: ArtifactDownload<'file>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub (in crate::launcher_rewrite::launch_properties) struct ArtifactDownload<'file> {
    #[serde(flatten)]
    #[serde(borrow)]
    pub info: DownloadInfo<'file>,
    #[serde(borrow)]
    pub path: &'file str,
}

type Rules<'file> = Vec<Rule<'file>>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub (in crate::launcher_rewrite::launch_properties) struct Rule<'file> {
    pub action: RuleAction,

    #[serde(borrow)]
    #[serde(default)]
    pub features: HashMap<&'file str, bool>,
    #[serde(borrow)]
    #[serde(default)]
    pub os: HashMap<&'file str, String>,
    //#[serde(default)]
    //#[serde(borrow)]
    //pub os: Option<OsMatcher<'file>>,
    //#[serde(default)]
    //pub features: Option<Features>,
}



/*
impl<'file> Rule<'file> {
    fn matches(&self) -> bool {
        let init = match self.action {
            RuleAction::Allow => false,
            RuleAction::Disallow => true,
        };
        let mut secondary = true;
        if let Some(os_matcher) = &self.os {
            secondary = os_matcher.matches();
        }
        if let Some(feature_matcher) = &self.features {
            secondary &= feature_matcher.matches();
        }
        init ^ secondary
    }
}*/

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Hash, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub (in crate::launcher_rewrite::launch_properties) enum RuleAction {
    Allow,
    Disallow,
}

impl RuleAction {
    pub fn as_str(self) -> &'static str {
        match self {
            RuleAction::Allow => "allow",
            RuleAction::Disallow => "disallow",
        }
    }
}
/*
#[derive(Serialize, Deserialize, Debug, Clone)]
pub (in crate::launcher_rewrite::launch_properties) struct OsMatcher<'file> {
    pub name: Option<&'file str>,
    pub arch: Option<&'file str>,
}

impl OsMatcher<'_> {
    fn matches(&self) -> bool {
        if let Some(os_name) = self.name {
            // config refers to macOS as "osx" so we need an extra check for that
            if os_name != OS || (OS == "macos" && os_name != "osx") {
                return false;
            }
        }
        if let Some(arch_name) = self.arch {
            if arch_name != ARCH {
                return false;
            }
        }
        return true;
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub (in crate::launcher_rewrite::launch_properties) struct Features {
    pub is_demo_user: Option<bool>,
    pub has_custom_resolution: Option<bool>,
    pub has_quick_plays_support: Option<bool>,
    pub is_quick_play_singleplayer: Option<bool>,
    pub is_quick_play_multiplayer: Option<bool>,
    pub is_quick_play_realms: Option<bool>,
}

impl Features {
    pub fn matches(&self) -> bool {
        return false; // TODO - no features for now :(
    }
}*/

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub (in crate::launcher_rewrite::launch_properties) enum Arg<'file> {
    Always(&'file str),
    Conditional { rules: Rules<'file>, value: ValueType<'file> },
}

impl<'file> Arg<'file> {
    /* pub fn get_args(&self) -> &[&'file str] {
        match self {
            Arg::Always(str) => from_ref(str),
            Arg::Conditional { rules, value } => {
                if rules.iter().map(Rule::matches).fold(true, bool::bitand) {
                    value.as_slice()
                } else {
                    &[]
                }
            }
        }
    }*/

    pub fn is_conditional(&self) -> bool {
        match self {
            Arg::Always(_) => false,
            Arg::Conditional { .. } => true,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub (in crate::launcher_rewrite::launch_properties) enum ValueType<'file> {
    Single(&'file str),
    Multiple(Vec<String>),
}

impl<'file> ValueType<'file> {
    /*pub fn as_slice(&self) -> &[&'file str] {
        match self {
            ValueType::Single(s) => from_ref(s),
            ValueType::Multiple(v) => v.as_slice(),
        }
    }*/

    pub fn into_vec(self) -> Vec<String> {
        match self {
            ValueType::Single(s) => { vec![s.to_owned().replace(' ', "")] }
            ValueType::Multiple(v) => { v.into_iter().map(|s| s.replace(' ', "")).collect() }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub (in crate::launcher_rewrite::launch_properties) struct LoggingInfo<'file> {
    #[serde(borrow)]
    pub client: ClientLoggingInfo<'file>,
}

impl<'file> Deref for LoggingInfo<'file> {
    type Target = ClientLoggingInfo<'file>;

    fn deref(&self) -> &Self::Target {
        &self.client
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub (in crate::launcher_rewrite::launch_properties) struct ClientLoggingInfo<'file> {
    pub argument: &'file str,
    pub file: LoggingFileData<'file>,
    #[serde(rename = "type")]
    pub log_type: &'file str,
}

impl<'file> ClientLoggingInfo<'file> {
    pub fn log_type(&self) -> &str {
        &self.log_type
    }

    pub fn argument(&self) -> &str {
        &self.argument
    }
}

impl<'file> Deref for ClientLoggingInfo<'file> {
    type Target = LoggingFileData<'file>;

    fn deref(&self) -> &Self::Target {
        &self.file
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub (in crate::launcher_rewrite::launch_properties) struct LoggingFileData<'file> {
    #[serde(flatten)]
    pub download_info: DownloadInfo<'file>,
    pub id: &'file str,
}

impl<'file> LoggingFileData<'file> {
    pub fn id(&self) -> &str {
        &self.id
    }
}

impl<'file> Deref for LoggingFileData<'file> {
    type Target = DownloadInfo<'file>;

    fn deref(&self) -> &Self::Target {
        &self.download_info
    }
}
