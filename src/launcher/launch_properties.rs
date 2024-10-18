use std::env::consts::{ARCH, OS};
use std::num::NonZeroU64;
use std::ops::{BitAnd, Deref};
use std::path::Path;
use std::slice::from_ref;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::{INSTALLED_VERSION_INFO, VERSION_MANIFEST};
use crate::launcher::version::{Version, VersionType};
use crate::launcher::settings::profiles::ModLoader;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LaunchProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    inherits_from: Option<String>,
    arguments: Arguments,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    asset_index: Option<AssetIndexInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    assets: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    compliance_level: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    downloads: Option<MainDownloads>,
    #[serde(rename = "id")]
    version_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    java_version: Option<JavaInfo>,
    libraries: Vec<Library>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    logging: Option<LoggingInfo>,
    main_class: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    minimum_launcher_version: Option<u8>,
    release_time: DateTime<Utc>,
    time: DateTime<Utc>,
    #[serde(rename = "type")]
    release_type: VersionType,
}

impl LaunchProperties {

    pub fn propagate_inheritance(mut self, launcher_path: &Path) -> Self {
        if let Some(id) = self.inherits_from.take() {
            // TODO ignoring the id for now
            //println!("before");
            let lock = VERSION_MANIFEST.read().unwrap();
            //println!("post before");
            let parent = lock.as_ref().unwrap().get_version(id.as_str()).unwrap().install_and_get_launch_properties(launcher_path, ModLoader::Vanilla);
            //println!("after");
            self.arguments.coalesce(parent.arguments);
            if self.asset_index.is_none() { self.asset_index = parent.asset_index }
            if self.compliance_level.is_none() { self.compliance_level = parent.compliance_level }
            if self.downloads.is_none() { self.downloads = parent.downloads }
            if self.java_version.is_none() { self.java_version = parent.java_version }
            self.libraries.extend(parent.libraries);
            if self.logging.is_none() { self.logging = parent.logging }
            if self.minimum_launcher_version.is_none() { self.minimum_launcher_version = parent.minimum_launcher_version }
        }
        return self
    }

    pub fn downloads(&self) -> &MainDownloads {
        self.downloads.as_ref().unwrap()
    }

    pub fn java_version(&self) -> Option<&JavaInfo> {
        self.java_version.as_ref()
    }

    pub fn libraries(&self) -> &Vec<Library> {
        &self.libraries
    }

    pub fn main_class(&self) -> &str {
        &self.main_class
    }

    pub fn arguments(&self) -> &Arguments {
        &self.arguments
    }

    pub fn asset_index(&self) -> &AssetIndexInfo {
        self.asset_index.as_ref().unwrap()
    }

    pub fn logging(&self) -> &LoggingInfo {
        self.logging.as_ref().unwrap()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Arguments {
    game: Vec<Arg>,
    jvm: Vec<Arg>,
}

impl Arguments {
    pub fn game(&self) -> &Vec<Arg> {
        &self.game
    }

    pub fn jvm(&self) -> &Vec<Arg> {
        &self.jvm
    }
    
    pub fn coalesce(&mut self, other: Arguments) {
        self.game.extend(other.game);
        self.jvm.extend(other.jvm);
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AssetIndexInfo {
    id: String,
    total_size: u64,
    #[serde(flatten)]
    download_info: DownloadInfo,
}

impl AssetIndexInfo {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn total_size(&self) -> u64 {
        self.total_size
    }
}

impl Deref for AssetIndexInfo {
    type Target = DownloadInfo;

    fn deref(&self) -> &Self::Target {
        &self.download_info
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MainDownloads {
    client: DownloadInfo,
    client_mappings: DownloadInfo,
    server: DownloadInfo,
    server_mappings: DownloadInfo,
}

impl MainDownloads {
    pub fn client(&self) -> &DownloadInfo {
        &self.client
    }

    pub fn client_mappings(&self) -> &DownloadInfo {
        &self.client_mappings
    }

    pub fn server(&self) -> &DownloadInfo {
        &self.server
    }

    pub fn server_mappings(&self) -> &DownloadInfo {
        &self.server_mappings
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct JavaInfo {
    component: String,
    major_version: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DownloadInfo {
    sha1: Option<String>,
    size: Option<NonZeroU64>,
    url: String,
}

impl DownloadInfo {
    pub fn sha1(&self) -> Option<&str> {
        self.sha1.as_ref().map(|s| s.as_str())
    }

    pub fn size(&self) -> Option<NonZeroU64> {
        self.size
    }

    pub fn url(&self) -> &str {
        &self.url
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Library {
    #[serde(flatten)]
    format: LibraryFormat,
    name: String,
    #[serde(default)]
    rules: Rules,
}

impl Library {

    pub fn filename(&self) -> String {
        match self.format {
            LibraryFormat::Artifact { ref downloads } => {
                downloads.path.rsplit_once('/').unwrap().1.into()
            },
            LibraryFormat::Plain { .. } => {
                let (namespace, version) = get_namespace_and_version_from_name(self.name.as_str());
                //println!("Namespace: {namespace}, Version: {version}");
                format!("{}-{}.jar", namespace, version)
            },
        }
    }

    pub fn get_url(&self) -> String {
        match self.format {
            LibraryFormat::Artifact { ref downloads } => {
                downloads.url.clone()
            }
            LibraryFormat::Plain { ref info } => {

                let (first, last) = self.name.rsplit_once(':').unwrap();
                let mut ret = first.replace(':', "/").replace('.', "/");
                ret.push_str("/");
                ret.push_str(last);
                ret.push_str("/");
                ret.insert_str(0, info.url.as_str());
                ret.push_str(self.filename().as_str());
                //println!("Url: {}", ret.as_str());
                ret
            }
        }
    }

}

fn get_namespace_and_version_from_name(name: &str) -> (&str, &str) {
    let split_name = name.rsplit_once(':').unwrap();
    let second_split = split_name.0.rsplit_once(':').unwrap();
    let namespace = second_split.1;
    let version = split_name.1;
    (namespace, version)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum LibraryFormat {
    Artifact {
        downloads: LibDownload
    },
    Plain {
        #[serde(flatten)]
        info: DownloadInfo,
    }
}

impl Deref for Library {
    type Target = DownloadInfo;

    fn deref(&self) -> &Self::Target {
        match self.format {
            LibraryFormat::Artifact { ref downloads } => { downloads }
            LibraryFormat::Plain{ ref info } => { info }
        }
    }
}

impl Library {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn rules_match(&self) -> bool {
        if !self.rules.is_empty() {
            for rule in &self.rules {
                if rule.matches() {
                    return true;
                }
            }
            return false;
        }
        return true;
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LibDownload {
    artifact: ArtifactDownload,
}

impl Deref for LibDownload {
    type Target = ArtifactDownload;

    fn deref(&self) -> &Self::Target {
        &self.artifact
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ArtifactDownload {
    #[serde(flatten)]
    info: DownloadInfo,
    path: String,
}

impl Deref for ArtifactDownload {
    type Target = DownloadInfo;

    fn deref(&self) -> &Self::Target {
        &self.info
    }
}

impl ArtifactDownload {
    pub fn path(&self) -> &str {
        &self.path
    }
}

type Rules = Vec<Rule>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Rule {
    action: RuleAction,
    #[serde(default)]
    os: Option<OsMatcher>,
    #[serde(default)]
    features: Option<Features>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Hash, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum RuleAction {
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

impl Rule {
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
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OsMatcher {
    name: Option<String>,
    arch: Option<String>,
}

impl OsMatcher {
    fn matches(&self) -> bool {
        if let Some(os_name) = &self.name {
            // config refers to macOS as "osx" so we need an extra check for that
            if os_name != OS || (OS == "macos" && os_name != "osx") {
                return false;
            }
        }
        if let Some(arch_name) = &self.arch {
            if arch_name != ARCH {
                return false;
            }
        }
        return true;
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Features {
    is_demo_user: Option<bool>,
    has_custom_resolution: Option<bool>,
    has_quick_plays_support: Option<bool>,
    is_quick_play_singleplayer: Option<bool>,
    is_quick_play_multiplayer: Option<bool>,
    is_quick_play_realms: Option<bool>,
}

impl Features {
    pub fn matches(&self) -> bool {
        return false; // TODO - no features for now :(
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Arg {
    Always(String),
    Conditional { rules: Rules, value: ValueType },
}

impl Arg {
    pub fn get_args(&self) -> &[String] {
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
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ValueType {
    Single(String),
    Multiple(Vec<String>),
}

impl ValueType {
    pub fn as_slice(&self) -> &[String] {
        match self {
            ValueType::Single(s) => from_ref(s),
            ValueType::Multiple(v) => v.as_slice(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoggingInfo {
    client: ClientLoggingInfo,
}

impl Deref for LoggingInfo {
    type Target = ClientLoggingInfo;

    fn deref(&self) -> &Self::Target {
        &self.client
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClientLoggingInfo {
    argument: String,
    file: LoggingFileData,
    #[serde(rename = "type")]
    log_type: String,
}

impl ClientLoggingInfo {
    pub fn log_type(&self) -> &str {
        &self.log_type
    }

    pub fn argument(&self) -> &str {
        &self.argument
    }
}

impl Deref for ClientLoggingInfo {
    type Target = LoggingFileData;

    fn deref(&self) -> &Self::Target {
        &self.file
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoggingFileData {
    #[serde(flatten)]
    download_info: DownloadInfo,
    id: String,
}

impl LoggingFileData {
    pub fn id(&self) -> &str {
        &self.id
    }
}

impl Deref for LoggingFileData {
    type Target = DownloadInfo;

    fn deref(&self) -> &Self::Target {
        &self.download_info
    }
}
