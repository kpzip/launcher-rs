use std::error::Error;
use std::fmt::{Display, Formatter, Write};
use std::io;
use crate::launcher_rewrite::error::LauncherError::ProfileError;

type LauncherResult<T> = Result<T, LauncherError>;

#[derive(Debug)]
pub enum LauncherError {
    DeserializeError(serde_json::Error),
    FsError(io::Error),
    DownloadError(reqwest::Error),
    ExtractError(zip::result::ZipError),
    AccountError,
    ProfileError,
}

impl Display for LauncherError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self))
    }
}

impl Error for LauncherError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        use LauncherError::*;
        match self {
            DeserializeError(e) => Some(e),
            FsError(e) => Some(e),
            DownloadError(e) => Some(e),
            ExtractError(e) => Some(e),
            AccountError | ProfileError => None,
        }
    }
    
}

impl From<serde_json::Error> for LauncherError {
    fn from(e: serde_json::Error) -> Self {
        Self::DeserializeError(e)
    }
}

impl From<reqwest::Error> for LauncherError {
    fn from(e: reqwest::Error) -> Self {
        Self::DownloadError(e)
    }
}

impl From<io::Error> for LauncherError {
    fn from(e: io::Error) -> Self {
        Self::FsError(e)
    }
}

impl From<zip::result::ZipError> for LauncherError {
    fn from(e: zip::result::ZipError) -> Self {
        Self::ExtractError(e)
    }
}