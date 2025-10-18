use std::path::Path;
use shortcuts_rs::{MSLinkError, ShellLink};
use crate::launcher_rewrite::launch_properties::Version;

fn create_link(exe_path: &Path, version: Version) -> Result<(), MSLinkError> {
    let _ = ShellLink::new(exe_path, None, None, None)?;
    return Ok(())
}