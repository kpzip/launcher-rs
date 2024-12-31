use std::{fs, io};
use std::fs::File;
use std::num::NonZeroU64;
use std::path::PathBuf;
use std::sync::LazyLock;
use reqwest::blocking::Client;
use reqwest::{redirect, Url};
use crate::launcher_rewrite::error::LauncherError;
use crate::launcher_rewrite::util::hash::{FileHash, sha1_matches};

pub const APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

pub static DEFAULT_DOWNLOADER_CLIENT: LazyLock<Client> = LazyLock::new(init_client);

pub const ACCEPT_HEADER_NAME: &str = "Accept";
pub const APPLICATION_JSON: &str = "application/json";

fn init_client() -> Client {
    Client::builder().user_agent(APP_USER_AGENT).redirect(redirect::Policy::limited(10)).build().expect("Failed to start web client")
}

pub trait Downloadable {

    fn get_download_url(&self) -> &Url;

    // fn get_download_url_in_place(&self, out: &mut Url);

    fn get_file_path(&self, game_version: &str) -> PathBuf;

    // fn get_file_path_in_place(&self, out: &mut PathBuf);

    fn get_hash(&self) -> Option<FileHash>;

    fn get_size(&self) -> Option<NonZeroU64>;

    fn requires_custom_download_fn(&self) -> bool {
        false
    }

    fn custom_download_fn(&self, _game_version: &str) -> Result<(), LauncherError> { Ok(()) }

    // For convenience
    fn download(&self, game_version: &str) -> Result<(), LauncherError> where Self: Sized {
        download(self, game_version)
    }

}


pub fn download<D: Downloadable>(download: &D, game_version: &str) -> Result<(), LauncherError> {
    if download.requires_custom_download_fn() {
        return download.custom_download_fn(game_version);
    }
    let path = download.get_file_path(game_version);
    if let Ok(file) = File::open(&path) {
        if let Some(hash) = download.get_hash() {
            if sha1_matches(file, hash.as_slice()) {
                return Ok(());
            }
        }
    }
    // TODO find a good way to have the url be moved by the `Downloadable` Trait but also have urls be verified at deserialize time
    let url = download.get_download_url();
    if url.scheme() == "about" && url.path() == "blank" {
        return Ok(());
    }
    let mut file = DEFAULT_DOWNLOADER_CLIENT.get(url.clone()).send()?;
    //println!("Path: {:?}", &path);
    let dir = path.parent().unwrap();
    //println!("Path: {:?}", dir);
    fs::create_dir_all(dir)?;
    let mut write_file = File::create(&path)?;
    io::copy(&mut file, &mut write_file)?;
    return Ok(());
}
