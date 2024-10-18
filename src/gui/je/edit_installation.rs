use crate::gui::je::installations::horizontal_separator;
use crate::gui::je::JavaEditionTab;
use crate::gui::{GuiMessage, LauncherMessage, LauncherRenderer, LauncherTheme};
use iced::alignment::Horizontal;
use iced::widget::image::FilterMethod;
use iced::widget::{button, column, container, image, row, text, PickList, Scrollable};
use iced::widget::{text_input, Space};
use iced::{Element, Font, Length};
use iced_aw::{helpers, number_input};
use sha1::digest::typenum::Mod;
use crate::launcher_rewrite::manifest::GAME_VERSION_MANIFEST;
use crate::launcher_rewrite::profiles::{LauncherProfile, ModLoader};
use crate::launcher_rewrite::profiles::icon::LauncherProfileIcon;

#[derive(Debug, Clone)]
pub enum JeProfileChanged {
    VersionChanged(String),
    ModLoaderChanged(ModLoader),
    NameChanged(String),
    MemoryChanged(u16),
    DirectoryChanged(String),
    WidthChanged(String),
    HeightChanged(String),
    JvmArgsChanged(String),
    IconChanged(LauncherProfileIcon),
    LoaderVersionChanged(String),
    Save,
}

pub fn edit_installations_tab_content(profile: &LauncherProfile) -> Element<'static, LauncherMessage, LauncherTheme, LauncherRenderer> {


    let icons = [LauncherProfileIcon::Grass, LauncherProfileIcon::Dirt, LauncherProfileIcon::CraftingTable, LauncherProfileIcon::Bedrock, LauncherProfileIcon::Bookshelf, LauncherProfileIcon::Brick, LauncherProfileIcon::Cake, LauncherProfileIcon::CarvedPumpkin, LauncherProfileIcon::Chest, LauncherProfileIcon::Clay, LauncherProfileIcon::CoalBlock, LauncherProfileIcon::CoalOre, LauncherProfileIcon::Cobblestone, LauncherProfileIcon::CreeperHead, LauncherProfileIcon::DiamondBlock, LauncherProfileIcon::DiamondOre, LauncherProfileIcon::DirtPodzol, LauncherProfileIcon::DirtSnow, LauncherProfileIcon::EmeraldBlock, LauncherProfileIcon::EmeraldOre, LauncherProfileIcon::EnchantingTable, LauncherProfileIcon::EndStone, LauncherProfileIcon::Farmland, LauncherProfileIcon::Furnace, LauncherProfileIcon::FurnaceOn, LauncherProfileIcon::Glass, LauncherProfileIcon::GlazedTerracottaLightBlue, LauncherProfileIcon::GlazedTerracottaOrange, LauncherProfileIcon::GlazedTerracottaWhite, LauncherProfileIcon::Glowstone, LauncherProfileIcon::GoldBlock, LauncherProfileIcon::GoldOre, LauncherProfileIcon::Gravel, LauncherProfileIcon::HardenedClay, LauncherProfileIcon::IcePacked, LauncherProfileIcon::IronBlock, LauncherProfileIcon::IronOre, LauncherProfileIcon::LapisOre, LauncherProfileIcon::LeavesBirch, LauncherProfileIcon::LeavesJungle, LauncherProfileIcon::LeavesOak, LauncherProfileIcon::LeavesSpruce, LauncherProfileIcon::LecternBook, LauncherProfileIcon::LogAcacia, LauncherProfileIcon::LogBirch, LauncherProfileIcon::LogDarkOak, LauncherProfileIcon::LogJungle, LauncherProfileIcon::LogOak, LauncherProfileIcon::LogSpruce, LauncherProfileIcon::Mycelium, LauncherProfileIcon::NetherBrick, LauncherProfileIcon::Netherrack, LauncherProfileIcon::Obsidian, LauncherProfileIcon::PlanksAcacia, LauncherProfileIcon::PlanksBirch, LauncherProfileIcon::PlanksDarkOak, LauncherProfileIcon::PlanksJungle, LauncherProfileIcon::PlanksOak, LauncherProfileIcon::PlanksSpruce, LauncherProfileIcon::QuartzOre, LauncherProfileIcon::RedSand, LauncherProfileIcon::RedSandstone, LauncherProfileIcon::RedstoneBlock, LauncherProfileIcon::RedstoneOre, LauncherProfileIcon::Sand, LauncherProfileIcon::Sandstone, LauncherProfileIcon::SkeletonSkull, LauncherProfileIcon::Snow, LauncherProfileIcon::SoulSand, LauncherProfileIcon::Stone, LauncherProfileIcon::StoneAndesite, LauncherProfileIcon::StoneDiorite, LauncherProfileIcon::StoneGranite, LauncherProfileIcon::TNT, LauncherProfileIcon::Water, LauncherProfileIcon::Wool];

    let icons_dropdown = container(PickList::new(icons, if profile.icon().is_custom() { None } else { Some(profile.icon().clone()) }, |i| LauncherMessage::JavaEditionProfileChanged(JeProfileChanged::IconChanged(i))).width(200)).center_x(Length::Fill);


    let mut version_list: Vec<String> = Vec::with_capacity(60);
    version_list.push("latest-release".into());
    version_list.push("latest-snapshot".into());

    version_list.extend(GAME_VERSION_MANIFEST.versions().iter().map(|(n, v)| v.id().into()));

    let mod_loader_list = [ModLoader::Vanilla, ModLoader::Fabric];

    let header = container(text(format!("Editing Profile:   `{}`", profile.name()))).center_x(Length::Fill);

    let icon = container(image(profile.icon()).filter_method(FilterMethod::Linear).width(128).height(128)).center_x(Length::Fill);

    let name_selector = container(row![container(text("Installation Name: ")).center_y(Length::Fill).width(240), container(text_input("Profile Name", profile.name()).width(220).on_input(|s| GuiMessage::JavaEditionProfileChanged(JeProfileChanged::NameChanged(s)))).center_y(Length::Fill).width(240),].height(40))
        .center_x(Length::Fill);
    let mod_loader_picker = container(row![container(text("Mod Loader: ")).center_y(Length::Fill).width(240), container(PickList::new(mod_loader_list, Some(profile.mod_loader()), |loader| GuiMessage::JavaEditionProfileChanged(JeProfileChanged::ModLoaderChanged(loader)))).center_y(Length::Fill).width(240),].height(40))
        .center_x(Length::Fill);

    let loader_version_list = ["latest-stable".to_owned(), "latest-beta".to_owned()];

    let mod_loader_version_picker = container(row![container(text("Loader Version: ")).center_y(Length::Fill).width(240), container(PickList::new(loader_version_list, Some(profile.mod_loader_version().to_owned()), |loader| GuiMessage::JavaEditionProfileChanged(JeProfileChanged::LoaderVersionChanged(loader.to_owned())))).center_y(Length::Fill).width(240),].height(40))
        .center_x(Length::Fill);

    let version_picker = container(
        row![
            container(text("Game Version: ")).center_y(Length::Fill).width(240),
            container(PickList::new(version_list, Some(String::from(profile.version_name())), |version_id| GuiMessage::JavaEditionProfileChanged(JeProfileChanged::VersionChanged(version_id))).width(220)).center_y(Length::Fill).width(240),
        ]
        .height(40),
    )
    .center_x(Length::Fill);

    let memory_selector = container(row![container(text("Memory (GB): ")).center_y(Length::Fill).width(240), container(number_input(profile.memory(), 2..=10, |i| { GuiMessage::JavaEditionProfileChanged(JeProfileChanged::MemoryChanged(i)) }).step(1)).center_y(Length::Fill).width(240),].height(40))
        .center_x(Length::Fill);

    let mc_dir_selector = container(row![container(text("Minecraft Directory: ")).center_y(Length::Fill).width(240), container(text_input("Minecraft Directory", profile.mc_directory()).width(220).on_input(|s| GuiMessage::JavaEditionProfileChanged(JeProfileChanged::DirectoryChanged(s)))).center_y(Length::Fill).width(240),].height(40))
        .center_x(Length::Fill);

    let resolution_selector = container(
        row![
            container(row![
                container(text_input("Width", profile.width().map(|w| w.to_string()).unwrap_or("".into()).as_str()).width(220).on_input(|s| GuiMessage::JavaEditionProfileChanged(JeProfileChanged::WidthChanged(s)))).center_y(Length::Fill),
                Space::new(Length::Fill, Length::Fill),
                container(text("x")).center_y(Length::Fill),
                Space::new(Length::Fill, Length::Fill),
            ])
            .center_y(Length::Fill)
            .width(240),
            container(text_input("Height", profile.height().map(|w| w.to_string()).unwrap_or("".into()).as_str()).width(220).on_input(|s| GuiMessage::JavaEditionProfileChanged(JeProfileChanged::HeightChanged(s)))).center_y(Length::Fill).width(240),
        ]
        .height(40),
    )
    .center_x(Length::Fill);

    let additional_jvm_args = container(container(text_input("<JVM Arguments>", profile.additional_args().as_ref().unwrap_or(&String::default()).as_str()).width(460).on_input(|s| GuiMessage::JavaEditionProfileChanged(JeProfileChanged::JvmArgsChanged(s)))).width(480)).center_x(Length::Fill);

    let save_cancel_buttons = container(
        container(
            row![
                container(button("Cancel").padding(10).on_press(GuiMessage::JavaEditionSelectTab(JavaEditionTab::Installations))).width(Length::Fill).align_x(Horizontal::Left),
                container(button("Save").padding(10).on_press(GuiMessage::JavaEditionProfileChanged(JeProfileChanged::Save))).width(Length::Fill).align_x(Horizontal::Right),
            ]
            .width(460),
        )
        .width(480),
    )
    .center_x(Length::Fill);

    let content: Element<'_, _, _, _> = column![
        Space::new(Length::Fill, 13),
        header,
        Space::new(Length::Fill, 13),
        horizontal_separator(),
        Space::new(Length::Fill, 25),
        icon,
        Space::new(Length::Fill, 25),
        icons_dropdown,
        Space::new(Length::Fill, 35),
        name_selector,
        Space::new(Length::Fill, 13),
        mod_loader_picker,
        Space::new(Length::Fill, 13),
        mod_loader_version_picker,
        Space::new(Length::Fill, 13),
        version_picker,
        Space::new(Length::Fill, 13),
        memory_selector,
        Space::new(Length::Fill, 13),
        mc_dir_selector,
        Space::new(Length::Fill, 15),
        container(text("Resolution:")).height(40).center_x(Length::Fill),
        Space::new(Length::Fill, 11),
        resolution_selector,
        Space::new(Length::Fill, 15),
        container(text("Additional Jvm Args:")).height(40).center_x(Length::Fill),
        Space::new(Length::Fill, 11),
        additional_jvm_args,
        Space::new(Length::Fill, 60),
        save_cancel_buttons,
        Space::new(Length::Fill, 150),
    ]
    .into();

    Scrollable::new(content).into()
}
