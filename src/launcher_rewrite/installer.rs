use std::fs;
use std::fs::File;
use std::num::NonZeroU64;
use std::path::PathBuf;
use std::sync::LazyLock;
use reqwest::blocking::Client;
use reqwest::{redirect, Url};
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

    fn get_file_path(&self, version_name: &str) -> PathBuf;

    // fn get_file_path_in_place(&self, out: &mut PathBuf);

    fn get_hash(&self) -> Option<FileHash>;

    fn get_size(&self) -> Option<NonZeroU64>;

    // For convenience
    fn download(&self, version_name: &str) where Self: Sized {
        download(self, version_name);
    }

}


// TODO Error Handling + Client and reduce allocations
pub fn download<D: Downloadable>(download: &D, version_name: &str) {
    let path = download.get_file_path(version_name);
    if let Ok(file) = File::open(&path) {
        if let Some(hash) = download.get_hash() {
            if sha1_matches(file, hash.as_slice()) {
                return;
            }
        }
    }
    // TODO find a good way to have the url be moved by the `Downloadable` Trait but also have urls be verified at deserialize time
    let file = DEFAULT_DOWNLOADER_CLIENT.get(Url::clone(download.get_download_url())).send().unwrap().bytes().unwrap();
    //println!("Path: {:?}", &path);
    let dir = path.parent().unwrap();
    //println!("Path: {:?}", dir);
    fs::create_dir_all(dir).unwrap();
    fs::write(&path, file).unwrap();
}
