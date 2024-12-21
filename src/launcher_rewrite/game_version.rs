use crate::launcher_rewrite::manifest::GAME_VERSION_MANIFEST;

pub struct GameVersion<'a> {
    unsanitized: &'a str,
}

impl<'a> GameVersion<'a> {

    pub fn from_str(game_version: &'a str) -> Self {
        Self {
            unsanitized: game_version,
        }
    }

    pub fn as_str(&self) -> &str {
        GAME_VERSION_MANIFEST.sanitize_version_name(self.unsanitized)
    }

}