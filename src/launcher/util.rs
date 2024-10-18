use sha1::{Digest, Sha1};
use std::path::Path;
use std::{fs, io};

pub fn sha1_of_file(path: &Path) -> String {
    let mut file = fs::File::open(&path).unwrap();
    let mut hasher = Sha1::new();
    io::copy(&mut file, &mut hasher).unwrap();
    let hash = hasher.finalize();
    hash.iter().map(|b| format!("{:02x}", b)).collect::<Vec<String>>().join("")
}

