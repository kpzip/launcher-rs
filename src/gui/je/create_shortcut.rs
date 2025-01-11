use crate::launcher_rewrite::profiles::ModLoader;

#[derive(Default)]
pub struct ShortcutInfo {
    profile_id: u128,
    game_version: String,
    loader_version: String,
    loader: ModLoader,
    memory: u16,
}