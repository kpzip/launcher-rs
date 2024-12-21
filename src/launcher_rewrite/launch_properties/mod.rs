mod internal;

use crate::launcher_rewrite::assets::AssetsIndex;
use crate::launcher_rewrite::jar_utils::extractor::extract_dlls_from_jar;
use crate::launcher_rewrite::installer::Downloadable;
use crate::launcher_rewrite::launch_properties::internal::{Arg, AssetIndexInfo, ClientJson, LibraryFormat, LoggingInfo, RuleAction};
use crate::launcher_rewrite::path_handler::{get_assets_index_dir, get_bin_path, get_log_configs_folder, get_vanilla_client_json_path, BIN_PATH};
use crate::launcher_rewrite::profiles::ModLoader;
use crate::launcher_rewrite::util::hash;
use crate::launcher_rewrite::util::hash::{sha1_from_base64_str, FileHash};
use crate::launcher_rewrite::version_type::VersionType;
use crate::util::{flip_result_option, unpack_option};
use chrono::{DateTime, Utc};
use reqwest::Url;
use serde::de::{Error, MapAccess, Unexpected, Visitor};
use serde::{de, Deserialize, Deserializer};
use std::env::consts::{ARCH, OS};
use std::iter::Map;
use std::num::NonZeroU64;
use std::ops::BitAnd;
use std::path::PathBuf;
use std::slice::Iter;
use std::str::FromStr;
use std::{fs, vec};
use regex::Regex;

#[derive(Debug, Clone)]
pub struct Version {
    id: String,
    game_version: String,
    time: DateTime<Utc>,
    main_class: String,
    version_type: VersionType,
    arguments: Arguments,
    libs: Vec<LibraryInfo>,
    assets: AssetsIndexInfo,
    log_info: LogConfigInfo,
}

impl Version {
    pub fn from_file() -> Self {
        todo!()
    }

    pub fn install(&self) {
        let version_name = self.game_version.as_str();

        // Libraries
        self.libs.iter().for_each(|lib| lib.download(version_name));

        // Assets
        self.assets.download(version_name);
        let index_file = fs::read_to_string(self.assets.get_file_path(version_name)).unwrap();
        let assets_index: AssetsIndex = serde_json::from_str(index_file.as_str()).unwrap();
        assets_index.download_all(version_name);

        // Log configs
        self.log_info.download(version_name);

        // Extract dlls
        let extract_path = get_bin_path(version_name);
        self.libs.iter().for_each(|lib| {
            let path = lib.get_file_path(version_name);
            extract_dlls_from_jar(&extract_path, &path);
        })
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn game_version(&self) -> &str {
        &self.game_version
    }

    pub fn time(&self) -> DateTime<Utc> {
        self.time
    }

    pub fn main_class(&self) -> &str {
        &self.main_class
    }

    pub fn version_type(&self) -> VersionType {
        self.version_type
    }

    pub fn arguments(&self) -> &Arguments {
        &self.arguments
    }

    pub fn libs(&self) -> &Vec<LibraryInfo> {
        &self.libs
    }

    pub fn assets(&self) -> &AssetsIndexInfo {
        &self.assets
    }

    pub fn log_info(&self) -> &LogConfigInfo {
        &self.log_info
    }
}

impl<'de> Deserialize<'de> for Version {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let json: ClientJson = ClientJson::deserialize(deserializer)?;
        let mut file: Option<String> = None;
        let mut inherited: Option<ClientJson> = None;

        if let Some(inheritance) = json.inherits_from {
            // TODO think of a good way to not deserialize some files twice
            // TODO download inherited file if needed

            file = Some(fs::read_to_string(get_vanilla_client_json_path(inheritance, ModLoader::Vanilla, "")).map_err(|e| Error::custom(e))?); // Only allow inheriting from vanilla for now
            inherited = Some(serde_json::from_str(file.as_ref().unwrap().as_str()).map_err(|e| Error::custom(e))?);
            if inherited.as_ref().unwrap().inherits_from.is_some() {
                return Err(Error::custom(">2-level client json inheritance is currently unsupported."));
            }
        }

        let id: String = json.version_id.into();
        let game_version: String = if let Some(i) = &inherited { i.version_id.into() } else { json.version_id.into() };
        if json.time != json.release_time {
            return Err(Error::custom("Time and release time do not match!"));
        }
        let time = json.time;
        let main_class: String = first_or_second_or_missing(json.main_class.map(|s| String::from(s)), inherited.as_ref(), |j| Ok(j.main_class.map(|s| String::from(s))), "mainClass")?; //json.main_class.into();
        let version_type = first_or_second_or_missing(json.release_type, inherited.as_ref(), |j| Ok(j.release_type), "type")?;

        let (game_args_internal, jvm_args_internal) = json.arguments.into_parts();
        // TODO optimize to reduce allocations
        let a1 = Argument::without_rules(map_unconditional_args(game_args_internal.iter()));
        let mut game_args: Vec<Argument> = map_args(game_args_internal)?;
        game_args.push(a1);

        let a2 = Argument::without_rules(map_unconditional_args(jvm_args_internal.iter()));
        let mut jvm_args: Vec<Argument> = map_args(jvm_args_internal)?;
        jvm_args.push(a2);

        let mut libs = json.libraries.into_iter().map(map_library).filter_map(flip_result_option).collect::<Result<Vec<LibraryInfo>, D::Error>>()?;
        // Add Main Jar as a library since its easier that way
        let downloads = first_or_second_or_missing(json.downloads, inherited.as_ref(), |j| Ok(j.downloads), "downloads")?;
        libs.push(LibraryInfo::new(
            Url::parse(downloads.client.url).map_err(|e| Error::custom(e))?,
            match downloads.client.sha1 {
                None => None,
                Some(h) => Some(FileHash::Sha1(sha1_from_base64_str(h)?)),
            },
            downloads.client.size,
            String::from("client.jar"),
            String::from("client"),
        ));

        let assets = first_or_second_or_missing(unpack_assets_index(json.asset_index)?, inherited.as_ref(), |j| unpack_assets_index(j.asset_index), "assetIndex")?;

        let log_info = first_or_second_or_missing(unpack_log_config(json.logging)?, inherited.as_ref(), |j| unpack_log_config(j.logging), "logging")?;
        let log_arg = first_or_second_or_missing(unpack_option(json.logging, |l| l.client), inherited.as_ref(), |j| Ok(unpack_option(j.logging, |l| l.client)), "logging")?.argument.replace("${path}", "${logging_path}");
        jvm_args.push(Argument::without_rules(vec![log_arg]));

        if let Some(inherit) = inherited {
            let a1 = Argument::without_rules(map_unconditional_args(inherit.arguments.game.iter()));
            game_args.extend(map_args(inherit.arguments.game)?);
            game_args.push(a1);
            let a2 = Argument::without_rules(map_unconditional_args(inherit.arguments.jvm.iter()));
            jvm_args.extend(map_args(inherit.arguments.jvm)?);
            jvm_args.push(a2);
            let extend_by = inherit.libraries.into_iter().map(map_library).filter_map(flip_result_option).filter(|l| match l {
                Ok(lib) => {
                    libs.iter().filter(|l| l.name == lib.name).next().is_none()
                }
                Err(_) => { true }
            }).collect::<Result<Vec<LibraryInfo>, D::Error>>()?;
            libs.extend(extend_by);
        }

        // TODO resolve references and consolidate structs

        Ok(Self {
            id,
            game_version,
            time,
            main_class,
            version_type,
            arguments: Arguments::new(game_args, jvm_args),
            libs,
            assets,
            log_info,
        })
    }
}

fn first_or_second_or_missing<T, S, F, E>(first: Option<T>, second_container: Option<S>, unpacker: F, error_msg: &'static str) -> Result<T, E>
where
    E: de::Error,
    F: FnOnce(S) -> Result<Option<T>, E>,
{
    match first {
        None => unpacker(second_container.ok_or(Error::missing_field(error_msg))?)?.ok_or(Error::missing_field(error_msg)),
        Some(s) => Ok(s),
    }
}

fn map_args<E: Error>(args_in: Vec<internal::Arg>) -> Result<Vec<Argument>, E> {
    args_in
        .into_iter()
        .map(|a| match a {
            Arg::Always(v) => { Ok(None) }
            Arg::Conditional {rules, value} => { Ok(Some(Argument::with_rules(value.into_vec(), rules.into_iter().map(Rule::try_from_internal).collect::<Result<Vec<Rule>, E>>()?))) }
        })
        .filter_map(|r| {
            if let Ok(arg) = r {
                if let Some(inner) = arg {
                    Some(Ok(inner))
                } else {
                    None
                }
            } else {
                Some(Err(r.err().unwrap()))
            }
        })
        .collect::<Result<Vec<Argument>, E>>()
}

fn map_unconditional_args<'a>(args_in: impl Iterator<Item = &'a internal::Arg<'a>>) -> Vec<String> {
    args_in.map(|a| if let internal::Arg::Always(s) = a { Some(String::from(*s).replace(' ', "")) } else { None }).filter_map(|s| s).collect()
}

fn map_library<E: Error>(lib: internal::Library) -> Result<Option<LibraryInfo>, E> {
    const INVALID_MAVEN_NAME_TEXT: &'static str = "Valid maven Identifier: <groupId>:<artifactId>:<version>";

    let name = lib.name;

    // if name.chars().filter(|c| *c == ':').count() != 2 { return Err(E::invalid_value(Unexpected::Str(name), &INVALID_MAVEN_NAME_TEXT)) }

    for internal_rule in lib.rules {
        let parsed = Rule::try_from_internal(internal_rule)?;
        if !parsed.matches(false, false, false, false, false, false) {
            return Ok(None);
        }
    }

    let (first, version) = name.rsplit_once(':').ok_or_else(|| E::invalid_value(Unexpected::Str(name), &INVALID_MAVEN_NAME_TEXT))?;
    let (group_id, artifact_id) = first.split_once(':').ok_or_else(|| E::invalid_value(Unexpected::Str(name), &INVALID_MAVEN_NAME_TEXT))?;

    let group_id_url = group_id.replace('.', "/");

    let url = match lib.format {
        LibraryFormat::Artifact { ref downloads } => match downloads.artifact.info.url {
            "" => {
                Url::parse("about:blank").map_err(E::custom)?
            },
            url => {
                Url::parse(url).map_err(E::custom)?
            },
        },
        LibraryFormat::Plain { ref info } => Url::parse(format!("{}{}/{}/{}/{}-{}.jar", info.url, group_id_url.as_str(), artifact_id, version, artifact_id, version).as_str()).map_err(E::custom)?,
    };
    let name = match lib.format {
        LibraryFormat::Artifact { ref downloads } => String::from(downloads.artifact.path.rsplit_once('/').map(|s| s.1).unwrap_or(downloads.artifact.path)),
        LibraryFormat::Plain { ref info } => format!("{}-{}.jar", artifact_id, version),
    };
    let size = match lib.format {
        LibraryFormat::Artifact { ref downloads } => downloads.artifact.info.size,
        LibraryFormat::Plain { ref info } => info.size,
    };
    let check = match lib.format {
        LibraryFormat::Artifact { ref downloads } => match downloads.artifact.info.sha1 {
            None => None,
            Some(s) => Some(FileHash::Sha1(hash::sha1_from_base64_str(s)?)),
        },
        LibraryFormat::Plain { ref info } => match info.sha1 {
            None => None,
            Some(s) => Some(FileHash::Sha1(hash::sha1_from_base64_str(s)?)),
        },
    };

    Ok(Some(LibraryInfo::new(url, check, size, name, first.to_owned())))
}

fn unpack_assets_index<E: Error>(info: Option<internal::AssetIndexInfo>) -> Result<Option<AssetsIndexInfo>, E> {
    match info {
        None => Ok(None),
        Some(a) => Ok(Some(AssetsIndexInfo::new(
            format!("{}.json", a.id),
            match a.download_info.sha1 {
                None => None,
                Some(hash_str) => Some(FileHash::Sha1(sha1_from_base64_str(hash_str)?)),
            },
            a.download_info.size,
            a.total_size,
            Url::parse(a.download_info.url).map_err(|e| Error::custom(e))?,
        ))),
    }
}

fn unpack_log_config<E: Error>(info: Option<internal::LoggingInfo>) -> Result<Option<LogConfigInfo>, E> {
    match info {
        None => Ok(None),
        Some(l) => {
            if let Some(client) = l.client {
                Ok(Some(LogConfigInfo::new(

                String::from(client.file.id),
                match client.file.sha1 {
                    None => None,
                    Some(hash_str) => Some(FileHash::Sha1(sha1_from_base64_str(hash_str)?)),
                },
                client.file.size,
            Url::parse(client.file.url).map_err(|e| Error::custom(e))?,
                )))
            } else {
                Ok(None)
            }
        },
    }
}

#[derive(Debug, Clone)]
pub struct Arguments {
    game_args: Vec<Argument>,
    jvm_args: Vec<Argument>,
}

impl Arguments {
    pub fn new(game_args: Vec<Argument>, jvm_args: Vec<Argument>) -> Self {
        Self { game_args, jvm_args }
    }

    pub fn game_args(&self) -> &Vec<Argument> {
        &self.game_args
    }

    pub fn jvm_args(&self) -> &Vec<Argument> {
        &self.jvm_args
    }
}

#[derive(Debug, Clone)]
pub struct Argument {
    values: Vec<String>,
    rules: Vec<Rule>,
}

impl Argument {
    fn without_rules(values: Vec<String>) -> Self {
        Self { values, rules: Vec::new() }
    }

    fn with_rules(values: Vec<String>, rules: Vec<Rule>) -> Self {
        Self { values, rules }
    }

    pub fn values(&self) -> &Vec<String> {
        &self.values
    }

    pub fn matches(&self, is_demo_user: bool, has_custom_resolution: bool, has_quick_play_support: bool, has_quick_play_singleplayer: bool, has_quick_play_multiplayer: bool, has_quick_play_realms: bool) -> bool {
        self.rules.iter().map(|r| r.matches(is_demo_user, has_custom_resolution, has_quick_play_support, has_quick_play_singleplayer, has_quick_play_multiplayer, has_quick_play_realms)).fold(true, bool::bitand)
    }
}

#[derive(Debug, Clone)]
pub struct Rule {
    action: RuleAction,
    condition: Option<RuleCondition>,
}

impl Rule {
    fn try_from_internal<T: Error>(value: internal::Rule) -> Result<Self, T> {
        let action = value.action;
        let mut condition = None;
        if value.os.len() + value.features.len() > 1 {
            return Err(Error::custom("Only one condition per rule is currently supported."));
        }
        for (k, v) in value.os {
            condition = Some(match k {
                "name" => RuleCondition::Os(Os::from_str(v.as_str()).ok_or(Error::invalid_value(Unexpected::Str(v.as_str()), &"`windows`, `osx`, or `linux`"))?),
                "arch" => RuleCondition::Arch(Architecture::from_str(v.as_str()).ok_or(Error::invalid_value(Unexpected::Str(v.as_str()), &"`x86`, `x64`, or `arm64`"))?),
                "version" => RuleCondition::OsVersion(Regex::new(v.as_str()).map_err(Error::custom)?),
                unknown => return Err(Error::unknown_field(unknown, &["name", "arch", "version"])),
            });
            break;
        }
        for (k, v) in value.features {
            condition = Some(match k {
                "is_demo_user" => RuleCondition::IsDemoUser(v),
                "has_custom_resolution" => RuleCondition::HasCustomResolution(v),
                "has_quick_plays_support" => RuleCondition::HasQuickPlaySupport(v),
                "is_quick_play_singleplayer" => RuleCondition::IsQuickPlaySingleplayer(v),
                "is_quick_play_multiplayer" => RuleCondition::IsQuickPlayMultiplayer(v),
                "is_quick_play_realms" => RuleCondition::IsQuickPlayRealms(v),
                unknown => return Err(Error::unknown_field(unknown, &["is_demo_user", "has_custom_resolution", "has_quick_plays_support", "is_quick_play_singleplayer", "is_quick_play_multiplayer", "is_quick_play_realms"])),
            });
            break;
        }
        Ok(Self { action, condition })
    }

    pub fn matches(&self, is_demo_user: bool, has_custom_resolution: bool, has_quick_play_support: bool, has_quick_play_singleplayer: bool, has_quick_play_multiplayer: bool, has_quick_play_realms: bool) -> bool {
        let modifier = match self.action {
            RuleAction::Allow => false,
            RuleAction::Disallow => true,
        };
        modifier ^ self.condition.as_ref().map(|c| c.matches(is_demo_user, has_custom_resolution, has_quick_play_support, has_quick_play_singleplayer, has_quick_play_multiplayer, has_quick_play_realms)).unwrap_or(true)
    }
}

impl Rule {
    pub fn new(action: RuleAction, condition: Option<RuleCondition>) -> Self {
        Self { action, condition }
    }
}

#[derive(Debug, Clone)]
pub enum RuleCondition {
    IsDemoUser(bool),
    HasCustomResolution(bool),
    HasQuickPlaySupport(bool),
    IsQuickPlaySingleplayer(bool),
    IsQuickPlayMultiplayer(bool),
    IsQuickPlayRealms(bool),
    Arch(Architecture),
    Os(Os),
    OsVersion(Regex),
}

impl RuleCondition {
    pub fn matches(&self, is_demo_user: bool, has_custom_resolution: bool, has_quick_play_support: bool, has_quick_play_singleplayer: bool, has_quick_play_multiplayer: bool, has_quick_play_realms: bool) -> bool {
        match self {
            RuleCondition::IsDemoUser(b) => *b == is_demo_user,
            RuleCondition::HasCustomResolution(b) => *b == has_custom_resolution,
            RuleCondition::HasQuickPlaySupport(b) => *b == has_quick_play_support,
            RuleCondition::IsQuickPlaySingleplayer(b) => *b == has_quick_play_singleplayer,
            RuleCondition::IsQuickPlayMultiplayer(b) => *b == has_quick_play_multiplayer,
            RuleCondition::IsQuickPlayRealms(b) => *b == has_quick_play_realms,
            RuleCondition::Arch(a) => *a == Architecture::current(),
            RuleCondition::Os(os) => *os == Os::current(),
            RuleCondition::OsVersion(regex) => if let Some(ver) = get_os_version() { regex.is_match(ver.as_str()) } else { true },
        }
    }
}

fn get_os_version() -> Option<String> {
    let version =  match os_version::detect() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error getting os version!: {}", e);
            return None
        },
    };
    match version {
        os_version::OsVersion::Linux(lv) => lv.version_name,
        os_version::OsVersion::MacOS(mv) => Some(mv.version),
        os_version::OsVersion::Windows(wv) => Some(wv.version),
        _ => None
    }

}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Architecture {
    X86,
    X64,
    Aarch64,
}

impl Architecture {
    fn from_str(name: &str) -> Option<Self> {
        Some(match name {
            "x86" => Self::X86,
            "x64" => Self::X64,
            "x86_64" => Self::X64,
            "aarch64" => Self::Aarch64,
            "arm64" => Self::Aarch64,
            _ => return None,
        })
    }

    fn current() -> Self {
        match ARCH {
            "x86" => Self::X86,
            "x64" => Self::X64,
            "x86_64" => Self::X64,
            _ => panic!("Invalid OS!"),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Os {
    Windows,
    Osx,
    Linux,
}

impl Os {
    fn from_str(name: &str) -> Option<Self> {
        Some(match name {
            "windows" => Self::Windows,
            "osx" => Self::Osx,
            "macos" => Self::Osx,
            "darwin" => Self::Osx,
            "linux" => Self::Linux,
            _ => return None,
        })
    }

    fn current() -> Self {
        match OS {
            "windows" => Self::Windows,
            "macos" => Self::Osx,
            "linux" => Self::Linux,
            _ => panic!("Invalid OS!"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct LibraryInfo {
    download_url: Url,
    verifier: Option<FileHash>,
    size: Option<NonZeroU64>,
    file_name: String,
    name: String,
}

impl LibraryInfo {
    pub fn new(download_url: Url, verifier: Option<FileHash>, size: Option<NonZeroU64>, file_name: String, name: String) -> Self {
        Self { download_url, verifier, size, file_name, name }
    }
}

impl Downloadable for LibraryInfo {
    fn get_download_url(&self) -> &Url {
        &self.download_url
    }

    fn get_file_path(&self, version_name: &str) -> PathBuf {
        let mut buf = get_bin_path(version_name);
        buf.push(self.file_name.as_str());
        buf
    }

    fn get_hash(&self) -> Option<FileHash> {
        self.verifier
    }

    fn get_size(&self) -> Option<NonZeroU64> {
        self.size
    }
}

#[derive(Debug, Clone)]
pub struct AssetsIndexInfo {
    // No file extension
    id: String,
    hash: Option<FileHash>,
    size: Option<NonZeroU64>,
    total_size: Option<NonZeroU64>,
    url: Url,
}

impl AssetsIndexInfo {
    pub fn new(id: String, hash: Option<FileHash>, size: Option<NonZeroU64>, total_size: Option<NonZeroU64>, url: Url) -> Self {
        Self { id, hash, size, total_size, url }
    }

    pub fn id(&self) -> &str {
        &self.id
    }
}

impl Downloadable for AssetsIndexInfo {
    fn get_download_url(&self) -> &Url {
        &self.url
    }

    fn get_file_path(&self, _version: &str) -> PathBuf {
        get_assets_index_dir(self.id.as_str())
    }

    fn get_hash(&self) -> Option<FileHash> {
        self.hash
    }

    fn get_size(&self) -> Option<NonZeroU64> {
        self.size
    }
}

#[derive(Debug, Clone)]
pub struct LogConfigInfo {
    id: String,
    hash: Option<FileHash>,
    size: Option<NonZeroU64>,
    url: Url,
}

impl LogConfigInfo {
    pub fn new(id: String, hash: Option<FileHash>, size: Option<NonZeroU64>, url: Url) -> Self {
        Self { id, hash, size, url }
    }
}

impl Downloadable for LogConfigInfo {
    fn get_download_url(&self) -> &Url {
        &self.url
    }

    fn get_file_path(&self, _version: &str) -> PathBuf {
        get_log_configs_folder(self.id.as_str())
    }

    fn get_hash(&self) -> Option<FileHash> {
        self.hash
    }

    fn get_size(&self) -> Option<NonZeroU64> {
        self.size
    }
}

/*
#[cfg(test)]
mod tests {
    use std::fs;
    use std::time::Instant;
    use const_format::concatcp;
    use crate::launcher_rewrite::launch_properties::Version;
    use crate::launcher_rewrite::path_handler::{from_launcher_dir, PATH_SEP, VERSIONS_FOLDER};

    #[test]
    fn version_install_test() {
        let path = from_launcher_dir([VERSIONS_FOLDER, concatcp!("24w36a", PATH_SEP, "vanilla.json")]);
        println!("File Path: {:?}", path);
        let version: Version = serde_json::from_str(fs::read_to_string(path).unwrap().as_str()).unwrap();
        let start_time = Instant::now();
        //version.install();
        let elapsed = start_time.elapsed();
        println!("Time to install: {:?}", elapsed);
        version.launch("kpzip", "9858326aae20476b878e750106b2f9bd", "", None, "".as_ref());
        loop {}
        panic!();
    }

}*/
