use std::io;
use std::io::Read;
use serde::de::Error;
use sha1::Digest;

pub type Sha1 = [u8; 20];
pub type Sha256 = [u8; 32];
pub type Sha512 = [u8; 64];
pub type Md5 = [u8; 16];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileHash {
    Sha1(Sha1),
    Sha256(Sha256),
    Sha512(Sha512),
    Md5(Md5),
    Checksum(), // TODO
}

impl FileHash {
    pub fn as_slice(&self) -> &[u8] {
        match self {
            FileHash::Sha1(ref data) => data,
            FileHash::Sha256(ref data) => data,
            FileHash::Sha512(ref data) => data,
            FileHash::Md5(ref data) => data,
            FileHash::Checksum() => &[],
        }
    }
}

pub fn sha1_from_base64_str<E: Error>(base16: &str) -> Result<Sha1, E> {
    let mut encode_to = [0_u8; 20];
    if base16.len() != 20 * 2 { return Err(E::custom("Invalid Sha1 hash; must be 40 characters long")) }
    base16::decode_slice(&base16, &mut encode_to).map_err(E::custom).map(|_| encode_to)
}

pub fn sha256_from_base64_str<E: Error>(base16: &str) -> Result<Sha256, E> {
    let mut encode_to = [0_u8; 32];
    if base16.len() != 32 * 2 { return Err(E::custom("Invalid Sha256 hash; must be 64 characters long")) }
    base16::decode_slice(&base16, &mut encode_to).map_err(E::custom).map(|_| encode_to)
}

pub fn sha512_from_base64_str<E: Error>(base16: &str) -> Result<Sha512, E> {
    let mut encode_to = [0_u8; 64];
    if base16.len() != 64 * 2 { return Err(E::custom("Invalid Sha512 hash; must be 128 characters long")) }
    base16::decode_slice(&base16, &mut encode_to).map_err(E::custom).map(|_| encode_to)
}

pub fn md5_from_base64_str<E: Error>(base16: &str) -> Result<Md5, E> {
    let mut encode_to = [0_u8; 16];
    if base16.len() != 16 * 2 { return Err(E::custom("Invalid Md5 hash; must be 16 characters long")) }
    base16::decode_slice(&base16, &mut encode_to).map_err(E::custom).map(|_| encode_to)
}

// TODO abstract this to work with multiple hashers
pub fn sha1_matches<R: Read>(mut file: R, expected: &[u8]) -> bool {
    let mut hasher = sha1::Sha1::new();
    io::copy(&mut file, &mut hasher).unwrap();
    hasher.finalize().as_slice() == expected
}