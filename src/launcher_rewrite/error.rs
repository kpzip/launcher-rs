use std::error::Error;
use std::fmt::{Display, Formatter, Write};
use std::io;

type LauncherResult<T> = Result<T, LauncherError>;

#[derive(Debug)]
pub enum LauncherError {
    DeserializeError(serde_json::Error),
    FsError(io::Error),
    DownloadError(reqwest::Error),
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
        todo!()
    }
    
}

impl From<serde_json::Error> for LauncherError {
    fn from(e: serde_json::Error) -> Self {
        Self::DeserializeError(e)
    }
}