use std::{fs, io};
use std::path::{Path, PathBuf};
use crate::launcher_rewrite::error::LauncherError;

#[cfg(all(target_os = "windows", target_arch = "x86_64"))]
pub const NATIVE_JARS_SUFFIX: &str = "natives-windows.jar";
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub const NATIVE_JARS_SUFFIX: &str = "natives-windows-x86.jar";
#[cfg(all(target_os = "windows", target_arch = "aarch64"))]
pub const NATIVE_JARS_SUFFIX: &str = "natives-windows-arm64.jar";
#[cfg(all(target_os = "macos", target_arch = "x86_64"))]
pub const NATIVE_JARS_SUFFIX: &str = "natives-macos.jar";
#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
pub const NATIVE_JARS_SUFFIX: &str = "natives-macos-arm64.jar";
#[cfg(target_os = "linux")]
pub const NATIVE_JARS_SUFFIX: &str = "natives-linux.jar";

pub fn extract_if_needed(extracted_path: &Path, internal_path: &Path, jar_path: &Path) -> Result<(), LauncherError> {
    fs::create_dir_all(extracted_path.parent().unwrap())?;

    let jar_file = fs::File::open(jar_path)?;
    let mut archive = zip::ZipArchive::new(jar_file)?;
    let internal_name = internal_path.as_os_str().to_string_lossy();
    // println!("Extracting: {}", internal_name);
    let mut internal_file = match archive.by_name(internal_name.as_ref()) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Could not open internal file {}, because of the error: {}", internal_name.as_ref(), e);
            return Err(e.into());
        }
    };
    let mut write_file = fs::File::create(extracted_path)?;
    io::copy(&mut internal_file, &mut write_file)?;
    Ok(())
}

pub fn extract_dlls_from_jar(bin_path: &Path, jar_path: &Path) -> Result<(), LauncherError> {
    // println!("Jar Path: {}", jar_path.display());
    let jar_file = fs::File::open(jar_path)?;
    let mut archive = zip::ZipArchive::new(jar_file)?;
    let mut extracted_file_path: PathBuf = bin_path.into();

    let dll_names: Vec<String> = archive.file_names().filter(|n| n.ends_with(".dll")).map(String::from).collect();
    dll_names.iter().map(|internal_name| {
        let external_name = match internal_name.rsplit_once('/') {
            None => internal_name,
            Some((_, rhs)) => rhs,
        };
        extracted_file_path.push(external_name);
        //println!("Extracting dll file: {} to {}", internal_name, extracted_file_path.as_os_str().to_str().unwrap());
        let mut internal_file = archive.by_name(internal_name)?;
        let mut external_file = fs::File::create(&extracted_file_path)?;
        io::copy(&mut internal_file, &mut external_file)?;
        extracted_file_path.pop();
        Ok(())
    }).collect::<Result<Vec<()>, LauncherError>>()?;
    Ok(())
}