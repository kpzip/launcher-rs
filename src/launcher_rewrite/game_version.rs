use crate::launcher_rewrite::manifest::GAME_VERSION_MANIFEST;

pub struct SanitizedGameVersion<'a> {
    sanitized: &'a str,
}

impl<'a> SanitizedGameVersion<'a> {

    pub fn from_str(game_version: &'a str) -> Self {
        Self {
            sanitized: game_version,
        }
    }

}