use std::sync::LazyLock;
use iced::widget::markdown;
use mdka::from_html;
use serde::{Deserialize, Deserializer};
use serde::de::Error;
use crate::launcher_rewrite::patch_notes::internal::{PatchNotesEntry, PatchNotesInternal};

mod internal;

pub static JAVA_EDITION_PATCH_NOTES: LazyLock<JavaEditionPatchNotes> = LazyLock::new(init_je_patch_notes);

fn init_je_patch_notes() -> JavaEditionPatchNotes {
    let retrieved = reqwest::blocking::get("https://launchercontent.mojang.com/javaPatchNotes.json").unwrap().text().unwrap();
    serde_json::from_str(retrieved.as_str()).unwrap()
}

#[derive(Clone, Debug)]
pub struct JavaEditionPatchNotes {
    text_segments: Vec<String>,
}

impl JavaEditionPatchNotes {

    pub fn first(&self) -> &str {
        self.text_segments.first().map(|s| s.as_str()).unwrap_or("")
    }

    pub fn n_segments(&self, n: usize) -> impl Iterator<Item = &str> {
        self.text_segments.iter().map(|s| s.as_str()).take(n)
    }

}

impl<'de> Deserialize<'de> for JavaEditionPatchNotes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        let internal = PatchNotesInternal::deserialize(deserializer)?;

        let text_segments = internal.entries.into_iter().map(map_version_segments).collect::<Result<Vec<String>, D::Error>>()?;

        Ok(Self{ text_segments })

    }
}

fn map_version_segments<E: Error>(entry: PatchNotesEntry) -> Result<String, E> {
    //println!("{}", entry.body);
    Ok(from_html(entry.body.as_str()))
}

#[cfg(test)]
mod tests {
    use crate::launcher_rewrite::patch_notes::JAVA_EDITION_PATCH_NOTES;

    #[test]
    fn patch_notes_de_test() {
        let test = &*JAVA_EDITION_PATCH_NOTES;
        println!("{}", test.text_segments.first().unwrap().as_str());
        panic!()
    }

}