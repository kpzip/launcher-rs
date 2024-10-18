use crate::gui::settings::SettingsGuiState;
use base64::prelude::BASE64_STANDARD;
use base64::{DecodeError, Engine};
use iced::advanced::graphics::text::cosmic_text::rustybuzz::ttf_parser::RasterImageFormat::PNG;
use rand::random;
use serde::de::{Error, Visitor};
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use serde_json::from_str;
use std::fmt::{Display, Formatter, Write};
use std::fs;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::str;

const fn memory_default() -> u16 {
    2
}

fn memory_is_default(mem: &u16) -> bool {
    *mem == memory_default()
}

pub fn get_launcher_profiles(launcher_directory: &Path) -> LauncherProfiles {
    let mut profiles_json_dir: PathBuf = launcher_directory.into();
    profiles_json_dir.push("profiles");
    profiles_json_dir.set_extension("json");

    // TODO create if it doesn't exist
    let profiles_str = fs::read_to_string(profiles_json_dir).unwrap();
    from_str(profiles_str.as_str()).unwrap()
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LauncherProfiles {
    je_client_profiles: Vec<LauncherProfile>,
    settings: LauncherPersistentState,
}

impl LauncherProfiles {
    pub fn je_client_profiles(&self) -> &Vec<LauncherProfile> {
        &self.je_client_profiles
    }

    pub fn je_client_profiles_mut(&mut self) -> &mut Vec<LauncherProfile> {
        &mut self.je_client_profiles
    }

    pub fn settings(&self) -> LauncherPersistentState {
        self.settings
    }

    pub fn settings_mut(&mut self) -> &mut LauncherPersistentState {
        &mut self.settings
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct LauncherProfile {
    name: String,
    #[serde(rename = "id")]
    uuid: u128,
    mod_loader: ModLoader,
    version_name: String,
    mc_directory: String,
    icon: LauncherProfileIcon,
    additional_args: Option<String>,
    // in GB
    #[serde(default = "memory_default")]
    #[serde(skip_serializing_if = "memory_is_default")]
    memory: u16,
    #[serde(default)]
    width: Option<u32>,
    #[serde(default)]
    height: Option<u32>,
}

impl LauncherProfile {
    pub fn new() -> Self {
        Self {
            name: "Unnamed Profile".to_string(),
            uuid: random(),
            mod_loader: Default::default(),
            version_name: "latest-release".to_string(),
            mc_directory: "%appdata%/.minecraft/".to_string(), // TODO default path is OS dependent
            icon: Default::default(),
            additional_args: None,
            memory: 2,
            width: None,
            height: None,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn mod_loader(&self) -> ModLoader {
        self.mod_loader
    }

    pub fn version_name(&self) -> &str {
        &self.version_name
    }

    pub fn mc_directory(&self) -> &str {
        &self.mc_directory
    }

    pub fn icon(&self) -> &LauncherProfileIcon {
        &self.icon
    }

    pub fn additional_args(&self) -> &Option<String> {
        &self.additional_args
    }

    pub fn memory(&self) -> u16 {
        self.memory
    }

    pub fn width(&self) -> Option<u32> {
        self.width
    }

    pub fn height(&self) -> Option<u32> {
        self.height
    }

    pub fn id(&self) -> u128 {
        self.uuid
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_id(&mut self, id: u128) {
        self.uuid = id;
    }

    pub fn set_mod_loader(&mut self, mod_loader: ModLoader) {
        self.mod_loader = mod_loader;
    }

    pub fn set_version_name(&mut self, version_name: String) {
        self.version_name = version_name;
    }

    pub fn set_mc_directory(&mut self, mc_directory: String) {
        self.mc_directory = mc_directory;
    }

    pub fn set_icon(&mut self, icon: LauncherProfileIcon) {
        self.icon = icon;
    }

    pub fn set_additional_args(&mut self, additional_args: Option<String>) {
        self.additional_args = additional_args;
    }

    pub fn set_memory(&mut self, memory: u16) {
        self.memory = memory;
    }

    pub fn set_width(&mut self, width: Option<u32>) {
        self.width = width;
    }

    pub fn set_height(&mut self, height: Option<u32>) {
        self.height = height;
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Hash, Clone, Copy, Default)]
#[serde(rename_all = "snake_case")]
pub enum ModLoader {
    #[default]
    Vanilla,
    Fabric,
    Quilt,
    Forge,
    NeoForge,
}

pub fn fabric_version() -> String {
    "0.16.2".into() // TODO
}

impl ModLoader {

    pub fn as_str_non_pretty(&self) -> &'static str {
        match self {
            ModLoader::Vanilla => "vanilla",
            ModLoader::Fabric => "fabric",
            ModLoader::Quilt => "quilt",
            ModLoader::Forge => "forge",
            ModLoader::NeoForge => "neo_forge",
        }
    }

}

impl Display for ModLoader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            ModLoader::Vanilla => "Vanilla",
            ModLoader::Fabric => "Fabric",
            ModLoader::Quilt => "Quilt",
            ModLoader::Forge => "Forge",
            ModLoader::NeoForge => "NeoForge",
        };
        write!(f, "{}", str)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct LauncherPersistentState {
    #[serde(flatten)]
    settings: LauncherSettings,
    selected_profile_id: u128,
}

impl Deref for LauncherPersistentState {
    type Target = LauncherSettings;

    fn deref(&self) -> &Self::Target {
        &self.settings
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default, PartialEq, Eq)]
#[serde(default)]
pub struct LauncherSettings {
    pub enable_historical: bool,
    pub enable_snapshots: bool,
    pub keep_launcher_open: bool,
    pub re_open_launcher: bool,
}

impl LauncherPersistentState {
    pub fn set_settings(&mut self, settings: LauncherSettings) {
        self.settings = settings;
    }

    pub fn selected_profile_id(&self) -> u128 {
        self.selected_profile_id
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum LauncherProfileIcon {
    #[default]
    Grass,
    CraftingTable,
    Dirt,
    Bedrock,
    Bookshelf,
    Brick,
    Cake,
    CarvedPumpkin,
    Chest,
    Clay,
    CoalBlock,
    CoalOre,
    Cobblestone,
    CreeperHead,
    DiamondBlock,
    DiamondOre,
    DirtPodzol,
    DirtSnow,
    EmeraldBlock,
    EmeraldOre,
    EnchantingTable,
    EndStone,
    Farmland,
    Furnace,
    FurnaceOn,
    Glass,
    GlazedTerracottaLightBlue,
    GlazedTerracottaOrange,
    GlazedTerracottaWhite,
    Glowstone,
    GoldBlock,
    GoldOre,
    Gravel,
    HardenedClay,
    IcePacked,
    IronBlock,
    IronOre,
    LapisOre,
    LeavesBirch,
    LeavesJungle,
    LeavesOak,
    LeavesSpruce,
    LecternBook,
    LogAcacia,
    LogBirch,
    LogDarkOak,
    LogJungle,
    LogOak,
    LogSpruce,
    Mycelium,
    NetherBrick,
    Netherrack,
    Obsidian,
    PlanksAcacia,
    PlanksBirch,
    PlanksDarkOak,
    PlanksJungle,
    PlanksOak,
    PlanksSpruce,
    QuartzOre,
    RedSand,
    RedSandstone,
    RedstoneBlock,
    RedstoneOre,
    Sand,
    Sandstone,
    SkeletonSkull,
    Snow,
    SoulSand,
    Stone,
    StoneAndesite,
    StoneDiorite,
    StoneGranite,
    TNT,
    Water,
    Wool,
    Png(Vec<u8>),
    Svg(Vec<u8>),
}

impl Display for LauncherProfileIcon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                LauncherProfileIcon::Grass => "Grass",
                LauncherProfileIcon::CraftingTable => "Crafting Table",
                LauncherProfileIcon::Dirt => "Dirt",
                LauncherProfileIcon::Bedrock => "Bedrock",
                LauncherProfileIcon::Bookshelf => "Bookshelf",
                LauncherProfileIcon::Brick => "Brick",
                LauncherProfileIcon::Cake => "Cake",
                LauncherProfileIcon::CarvedPumpkin => "Carved Pumpkin",
                LauncherProfileIcon::Chest => "Chest",
                LauncherProfileIcon::Clay => "Clay",
                LauncherProfileIcon::CoalBlock => "Block of Coal",
                LauncherProfileIcon::CoalOre => "Coal Ore",
                LauncherProfileIcon::Cobblestone => "Cobblestone",
                LauncherProfileIcon::CreeperHead => "Creeper Head",
                LauncherProfileIcon::DiamondBlock => "Block of Diamond",
                LauncherProfileIcon::DiamondOre => "Diamond Ore",
                LauncherProfileIcon::DirtPodzol => "Podzol",
                LauncherProfileIcon::DirtSnow => "Snowy Grass",
                LauncherProfileIcon::EmeraldBlock => "Block of Emerald",
                LauncherProfileIcon::EmeraldOre => "Emerald Ore",
                LauncherProfileIcon::EnchantingTable => "Enchanting Table",
                LauncherProfileIcon::EndStone => "End Stone",
                LauncherProfileIcon::Farmland => "Farmland",
                LauncherProfileIcon::Furnace => "Furnace",
                LauncherProfileIcon::FurnaceOn => "Lit Furnace",
                LauncherProfileIcon::Glass => "Glass",
                LauncherProfileIcon::GlazedTerracottaLightBlue => "Light Blue Glazed Terracotta",
                LauncherProfileIcon::GlazedTerracottaOrange => "Orange Glazed Terracotta",
                LauncherProfileIcon::GlazedTerracottaWhite => "White Glazed Terracotta",
                LauncherProfileIcon::Glowstone => "Glowstone",
                LauncherProfileIcon::GoldBlock => "Block of Gold",
                LauncherProfileIcon::GoldOre => "Gold Ore",
                LauncherProfileIcon::Gravel => "Gravel",
                LauncherProfileIcon::HardenedClay => "Terracotta",
                LauncherProfileIcon::IcePacked => "Packed Ice",
                LauncherProfileIcon::IronBlock => "Block of Iron",
                LauncherProfileIcon::IronOre => "Iron Ore",
                LauncherProfileIcon::LapisOre => "Lapis Ore",
                LauncherProfileIcon::LeavesBirch => "Birch Leaves",
                LauncherProfileIcon::LeavesJungle => "Jungle Leaves",
                LauncherProfileIcon::LeavesOak => "Oak Leaves",
                LauncherProfileIcon::LeavesSpruce => "Spruce Leaves",
                LauncherProfileIcon::LecternBook => "Lectern",
                LauncherProfileIcon::LogAcacia => "Acacia Log",
                LauncherProfileIcon::LogBirch => "Birch Log",
                LauncherProfileIcon::LogDarkOak => "Dark Oak Log",
                LauncherProfileIcon::LogJungle => "Jungle Log",
                LauncherProfileIcon::LogOak => "Oak Log",
                LauncherProfileIcon::LogSpruce => "Spruce Log",
                LauncherProfileIcon::Mycelium => "Mycelium",
                LauncherProfileIcon::NetherBrick => "Nether Bricks",
                LauncherProfileIcon::Netherrack => "Netherrack",
                LauncherProfileIcon::Obsidian => "Obsidian",
                LauncherProfileIcon::PlanksAcacia => "Acacia Planks",
                LauncherProfileIcon::PlanksBirch => "Birch Planks",
                LauncherProfileIcon::PlanksDarkOak => "Dark Oak Planks",
                LauncherProfileIcon::PlanksJungle => "Jungle Planks",
                LauncherProfileIcon::PlanksOak => "Oak Planks",
                LauncherProfileIcon::PlanksSpruce => "Spruce Planks",
                LauncherProfileIcon::QuartzOre => "Quartz Ore",
                LauncherProfileIcon::RedSand => "Red Sand",
                LauncherProfileIcon::RedSandstone => "Red Sandstone",
                LauncherProfileIcon::RedstoneBlock => "Block of Redstone",
                LauncherProfileIcon::RedstoneOre => "Redstone Ore",
                LauncherProfileIcon::Sand => "Sand",
                LauncherProfileIcon::Sandstone => "Sandstone",
                LauncherProfileIcon::SkeletonSkull => "Skeleton Skull",
                LauncherProfileIcon::Snow => "Snow",
                LauncherProfileIcon::SoulSand => "Soul Sand",
                LauncherProfileIcon::Stone => "Stone",
                LauncherProfileIcon::StoneAndesite => "Andesite",
                LauncherProfileIcon::StoneDiorite => "Diorite",
                LauncherProfileIcon::StoneGranite => "Granite",
                LauncherProfileIcon::TNT => "TNT",
                LauncherProfileIcon::Water => "Water",
                LauncherProfileIcon::Wool => "Wool",
                LauncherProfileIcon::Png(ref arr) => "Custom Png",
                LauncherProfileIcon::Svg(ref arr) => "Custom Svg",
            }
        )
    }
}

impl LauncherProfileIcon {
    pub fn as_string(&self) -> String {
        match self {
            LauncherProfileIcon::Grass => "Grass".into(),
            LauncherProfileIcon::CraftingTable => "Crafting_Table".into(),
            LauncherProfileIcon::Dirt => "Dirt".into(),
            LauncherProfileIcon::Bedrock => "Bedrock".into(),
            LauncherProfileIcon::Bookshelf => "Bookshelf".into(),
            LauncherProfileIcon::Brick => "Brick".into(),
            LauncherProfileIcon::Cake => "Cake".into(),
            LauncherProfileIcon::CarvedPumpkin => "Carved_Pumpkin".into(),
            LauncherProfileIcon::Chest => "Chest".into(),
            LauncherProfileIcon::Clay => "Clay".into(),
            LauncherProfileIcon::CoalBlock => "Coal_Block".into(),
            LauncherProfileIcon::CoalOre => "Coal_Ore".into(),
            LauncherProfileIcon::Cobblestone => "Cobblestone".into(),
            LauncherProfileIcon::CreeperHead => "Creeper_Head".into(),
            LauncherProfileIcon::DiamondBlock => "Diamond_Block".into(),
            LauncherProfileIcon::DiamondOre => "Diamond_Ore".into(),
            LauncherProfileIcon::DirtPodzol => "Dirt_Podzol".into(),
            LauncherProfileIcon::DirtSnow => "Dirt_Snow".into(),
            LauncherProfileIcon::EmeraldBlock => "Emerald_Block".into(),
            LauncherProfileIcon::EmeraldOre => "Emerald_Ore".into(),
            LauncherProfileIcon::EnchantingTable => "Enchanting_Table".into(),
            LauncherProfileIcon::EndStone => "End_Stone".into(),
            LauncherProfileIcon::Farmland => "Farmland".into(),
            LauncherProfileIcon::Furnace => "Furnace".into(),
            LauncherProfileIcon::FurnaceOn => "Furnace_On".into(),
            LauncherProfileIcon::Glass => "Glass".into(),
            LauncherProfileIcon::GlazedTerracottaLightBlue => "Glazed_Terracotta_Light_Blue".into(),
            LauncherProfileIcon::GlazedTerracottaOrange => "Glazed_Terracotta_Orange".into(),
            LauncherProfileIcon::GlazedTerracottaWhite => "Glazed_Terracotta_White".into(),
            LauncherProfileIcon::Glowstone => "Glowstone".into(),
            LauncherProfileIcon::GoldBlock => "Gold_Block".into(),
            LauncherProfileIcon::GoldOre => "Gold_Ore".into(),
            LauncherProfileIcon::Gravel => "Gravel".into(),
            LauncherProfileIcon::HardenedClay => "Hardened_Clay".into(),
            LauncherProfileIcon::IcePacked => "Ice_Packed".into(),
            LauncherProfileIcon::IronBlock => "Iron_Block".into(),
            LauncherProfileIcon::IronOre => "Iron_Ore".into(),
            LauncherProfileIcon::LapisOre => "Lapis_Ore".into(),
            LauncherProfileIcon::LeavesBirch => "Leaves_Birch".into(),
            LauncherProfileIcon::LeavesJungle => "Leaves_Jungle".into(),
            LauncherProfileIcon::LeavesOak => "Leaves_Oak".into(),
            LauncherProfileIcon::LeavesSpruce => "Leaves_Spruce".into(),
            LauncherProfileIcon::LecternBook => "Lectern_Book".into(),
            LauncherProfileIcon::LogAcacia => "Log_Acacia".into(),
            LauncherProfileIcon::LogBirch => "Log_Birch".into(),
            LauncherProfileIcon::LogDarkOak => "Log_DarkOak".into(),
            LauncherProfileIcon::LogJungle => "Log_Jungle".into(),
            LauncherProfileIcon::LogOak => "Log_Oak".into(),
            LauncherProfileIcon::LogSpruce => "Log_Spruce".into(),
            LauncherProfileIcon::Mycelium => "Mycelium".into(),
            LauncherProfileIcon::NetherBrick => "Nether_Brick".into(),
            LauncherProfileIcon::Netherrack => "Netherrack".into(),
            LauncherProfileIcon::Obsidian => "Obsidian".into(),
            LauncherProfileIcon::PlanksAcacia => "Planks_Acacia".into(),
            LauncherProfileIcon::PlanksBirch => "Planks_Birch".into(),
            LauncherProfileIcon::PlanksDarkOak => "Planks_DarkOak".into(),
            LauncherProfileIcon::PlanksJungle => "Planks_Jungle".into(),
            LauncherProfileIcon::PlanksOak => "Planks_Oak".into(),
            LauncherProfileIcon::PlanksSpruce => "Planks_Spruce".into(),
            LauncherProfileIcon::QuartzOre => "Quartz_Ore".into(),
            LauncherProfileIcon::RedSand => "Red_Sand".into(),
            LauncherProfileIcon::RedSandstone => "Red_Sandstone".into(),
            LauncherProfileIcon::RedstoneBlock => "Redstone_Block".into(),
            LauncherProfileIcon::RedstoneOre => "Redstone_Ore".into(),
            LauncherProfileIcon::Sand => "Sand".into(),
            LauncherProfileIcon::Sandstone => "Sandstone".into(),
            LauncherProfileIcon::SkeletonSkull => "Skeleton_Skull".into(),
            LauncherProfileIcon::Snow => "Snow".into(),
            LauncherProfileIcon::SoulSand => "Soul_Sand".into(),
            LauncherProfileIcon::Stone => "Stone".into(),
            LauncherProfileIcon::StoneAndesite => "Stone_Andesite".into(),
            LauncherProfileIcon::StoneDiorite => "Stone_Diorite".into(),
            LauncherProfileIcon::StoneGranite => "Stone_Granite".into(),
            LauncherProfileIcon::TNT => "TNT".into(),
            LauncherProfileIcon::Water => "Water".into(),
            LauncherProfileIcon::Wool => "Wool".into(),
            LauncherProfileIcon::Png(s) => {
                let mut ret = String::with_capacity(PNG_PREFIX.len() + s.len());
                ret.push_str(PNG_PREFIX);
                ret.push_str(BASE64_STANDARD.encode(s).as_str());
                ret
            },
            LauncherProfileIcon::Svg(s) => {
                let mut ret = String::with_capacity(SVG_PREFIX.len() + s.len());
                ret.push_str(SVG_PREFIX);
                ret.push_str(BASE64_STANDARD.encode(s).as_str());
                ret
            },
        }
    }

    pub fn is_custom(&self) -> bool {
        match self {
            LauncherProfileIcon::Png(_) => true,
            LauncherProfileIcon::Svg(_) => true,
            _ => false,
        }
    }
}

const PNG_PREFIX: &str = "data:image/png;base64,";
const SVG_PREFIX: &str = "data:image/svg;base64,";
const ICON_EXPECTED_STRING: &[&str] = &["`<Icon name>`", "`data:image/<png|svg>;base64,<base64 string>`"];

impl<'de> Deserialize<'de> for LauncherProfileIcon {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ImageStringVisitor;

        impl<'de> Visitor<'de> for ImageStringVisitor {
            type Value = LauncherProfileIcon;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str(format!("{} or {}", ICON_EXPECTED_STRING[0], ICON_EXPECTED_STRING[1]).as_str())
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                match value {
                    "Grass" => Ok(LauncherProfileIcon::Grass),
                    "Dirt" => Ok(LauncherProfileIcon::Dirt),
                    "Crafting_Table" => Ok(LauncherProfileIcon::CraftingTable),
                    "Bedrock" => Ok(LauncherProfileIcon::Bedrock),
                    "Bookshelf" => Ok(LauncherProfileIcon::Bookshelf),
                    "Brick" => Ok(LauncherProfileIcon::Brick),
                    "Cake" => Ok(LauncherProfileIcon::Cake),
                    "Carved_Pumpkin" => Ok(LauncherProfileIcon::CarvedPumpkin),
                    "Chest" => Ok(LauncherProfileIcon::Chest),
                    "Clay" => Ok(LauncherProfileIcon::Clay),
                    "Coal_Block" => Ok(LauncherProfileIcon::CoalBlock),
                    "Coal_Ore" => Ok(LauncherProfileIcon::CoalOre),
                    "Cobblestone" => Ok(LauncherProfileIcon::Cobblestone),
                    "Creeper_Head" => Ok(LauncherProfileIcon::CreeperHead),
                    "Diamond_Block" => Ok(LauncherProfileIcon::DiamondBlock),
                    "Diamond_Ore" => Ok(LauncherProfileIcon::DiamondOre),
                    "Dirt_Podzol" => Ok(LauncherProfileIcon::DirtPodzol),
                    "Dirt_Snow" => Ok(LauncherProfileIcon::DirtSnow),
                    "Emerald_Block" => Ok(LauncherProfileIcon::EmeraldBlock),
                    "Emerald_Ore" => Ok(LauncherProfileIcon::EmeraldOre),
                    "Enchanting_Table" => Ok(LauncherProfileIcon::EnchantingTable),
                    "End_Stone" => Ok(LauncherProfileIcon::EndStone),
                    "Farmland" => Ok(LauncherProfileIcon::Farmland),
                    "Furnace" => Ok(LauncherProfileIcon::Furnace),
                    "Furnace_On" => Ok(LauncherProfileIcon::FurnaceOn),
                    "Glass" => Ok(LauncherProfileIcon::Glass),
                    "Glazed_Terracotta_Light_Blue" => Ok(LauncherProfileIcon::GlazedTerracottaLightBlue),
                    "Glazed_Terracotta_Orange" => Ok(LauncherProfileIcon::GlazedTerracottaOrange),
                    "Glazed_Terracotta_White" => Ok(LauncherProfileIcon::GlazedTerracottaWhite),
                    "Glowstone" => Ok(LauncherProfileIcon::Glowstone),
                    "Gold_Block" => Ok(LauncherProfileIcon::GoldBlock),
                    "Gold_Ore" => Ok(LauncherProfileIcon::GoldOre),
                    "Gravel" => Ok(LauncherProfileIcon::Gravel),
                    "Hardened_Clay" => Ok(LauncherProfileIcon::HardenedClay),
                    "Ice_Packed" => Ok(LauncherProfileIcon::IcePacked),
                    "Iron_Block" => Ok(LauncherProfileIcon::IronBlock),
                    "Iron_Ore" => Ok(LauncherProfileIcon::IronOre),
                    "Lapis_Ore" => Ok(LauncherProfileIcon::LapisOre),
                    "Leaves_Birch" => Ok(LauncherProfileIcon::LeavesBirch),
                    "Leaves_Jungle" => Ok(LauncherProfileIcon::LeavesJungle),
                    "Leaves_Oak" => Ok(LauncherProfileIcon::LeavesOak),
                    "Leaves_Spruce" => Ok(LauncherProfileIcon::LeavesSpruce),
                    "Lectern_Book" => Ok(LauncherProfileIcon::LecternBook),
                    "Log_Acacia" => Ok(LauncherProfileIcon::LogAcacia),
                    "Log_Birch" => Ok(LauncherProfileIcon::LogBirch),
                    "Log_DarkOak" => Ok(LauncherProfileIcon::LogDarkOak),
                    "Log_Jungle" => Ok(LauncherProfileIcon::LogJungle),
                    "Log_Oak" => Ok(LauncherProfileIcon::LogOak),
                    "Log_Spruce" => Ok(LauncherProfileIcon::LogSpruce),
                    "Mycelium" => Ok(LauncherProfileIcon::Mycelium),
                    "Nether_Brick" => Ok(LauncherProfileIcon::NetherBrick),
                    "Netherrack" => Ok(LauncherProfileIcon::Netherrack),
                    "Obsidian" => Ok(LauncherProfileIcon::Obsidian),
                    "Planks_Acacia" => Ok(LauncherProfileIcon::PlanksAcacia),
                    "Planks_Birch" => Ok(LauncherProfileIcon::PlanksBirch),
                    "Planks_DarkOak" => Ok(LauncherProfileIcon::PlanksDarkOak),
                    "Planks_Jungle" => Ok(LauncherProfileIcon::PlanksJungle),
                    "Planks_Oak" => Ok(LauncherProfileIcon::PlanksOak),
                    "Planks_Spruce" => Ok(LauncherProfileIcon::PlanksSpruce),
                    "Quartz_Ore" => Ok(LauncherProfileIcon::QuartzOre),
                    "Red_Sand" => Ok(LauncherProfileIcon::RedSand),
                    "Red_Sandstone" => Ok(LauncherProfileIcon::RedSandstone),
                    "Redstone_Block" => Ok(LauncherProfileIcon::RedstoneBlock),
                    "Redstone_Ore" => Ok(LauncherProfileIcon::RedstoneOre),
                    "Sand" => Ok(LauncherProfileIcon::Sand),
                    "Sandstone" => Ok(LauncherProfileIcon::Sandstone),
                    "Skeleton_Skull" => Ok(LauncherProfileIcon::SkeletonSkull),
                    "Snow" => Ok(LauncherProfileIcon::Snow),
                    "Soul_Sand" => Ok(LauncherProfileIcon::SoulSand),
                    "Stone" => Ok(LauncherProfileIcon::Stone),
                    "Stone_Andesite" => Ok(LauncherProfileIcon::StoneAndesite),
                    "Stone_Diorite" => Ok(LauncherProfileIcon::StoneDiorite),
                    "Stone_Granite" => Ok(LauncherProfileIcon::StoneGranite),
                    "TNT" => Ok(LauncherProfileIcon::TNT),
                    "Water" => Ok(LauncherProfileIcon::Water),
                    "Wool" => Ok(LauncherProfileIcon::Wool),
                    custom => {
                        if custom.starts_with("data:image/png;base64,") {
                            let base64str = String::from(custom.split_once(',').unwrap().1);
                            let base64data = match BASE64_STANDARD.decode(&base64str) {
                                Ok(d) => d,
                                Err(e) => return Err(propagate_b64_error(e)),
                            };
                            Ok(LauncherProfileIcon::Png(base64data))
                        } else if custom.starts_with("data:image/svg;base64") {
                            let base64str = String::from(custom.split_once(',').unwrap().1);
                            let base64data = match BASE64_STANDARD.decode(&base64str) {
                                Ok(d) => d,
                                Err(e) => return Err(propagate_b64_error(e)),
                            };
                            Ok(LauncherProfileIcon::Svg(base64data))
                        } else {
                            Err(Error::unknown_field(custom, ICON_EXPECTED_STRING))
                        }
                    },
                }
            }
        }
        deserializer.deserialize_string(ImageStringVisitor)
    }
}

fn propagate_b64_error<E>(e: DecodeError) -> E
where
    E: Error,
{
    E::custom(e)
}

impl Serialize for LauncherProfileIcon {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_string().as_str())
    }
}

impl From<&LauncherProfileIcon> for iced::advanced::image::Handle {
    fn from(value: &LauncherProfileIcon) -> Self {
        match value {
            LauncherProfileIcon::Png(d) => iced::advanced::image::Handle::from_memory(d.clone()),
            LauncherProfileIcon::Svg(d) => iced::advanced::image::Handle::from_memory(d.clone()),
            _ => iced::advanced::image::Handle::from_path(format!("{}/assets/icons/{}.png", env!("CARGO_MANIFEST_DIR"), value.as_string())),
        }
    }
}
