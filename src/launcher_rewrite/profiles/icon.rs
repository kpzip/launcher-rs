use std::fmt::{Display, Formatter};
use base64::{DecodeError, Engine};
use base64::prelude::BASE64_STANDARD;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::{Error, Visitor};

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
            LauncherProfileIcon::Png(s) => format!("{}{}", PNG_PREFIX, BASE64_STANDARD.encode(s).as_str()),
            LauncherProfileIcon::Svg(s) => format!("{}{}", SVG_PREFIX, BASE64_STANDARD.encode(s).as_str()),
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
            LauncherProfileIcon::Png(d) => iced::advanced::image::Handle::from_bytes(d.clone()),
            LauncherProfileIcon::Svg(d) => iced::advanced::image::Handle::from_bytes(d.clone()),
            _ => iced::advanced::image::Handle::from_path(format!("{}/assets/icons/{}.png", env!("CARGO_MANIFEST_DIR"), value.as_string())), // TODO reconcile this to use the path handler
        }
    }
}