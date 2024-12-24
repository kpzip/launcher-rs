use std::error::Error;
use std::fmt::{Display, Formatter, Write};

type LauncherResult<T> = Result<T, LauncherError>;

#[derive(Debug, Clone)]
pub enum LauncherError {
    ProfilesDeserializeError,
    ClientDeserializeError,
    DownloadError,
    AccountError,
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