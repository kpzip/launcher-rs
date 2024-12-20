use std::{fs, io};
use std::path::{Path, PathBuf};

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

pub fn extract_if_needed(extracted_path: &Path, internal_path: &Path, jar_path: &Path) {
    fs::create_dir_all(extracted_path.parent().unwrap()).unwrap();

    let jarfile = fs::File::open(jar_path).unwrap();
    let mut archive = zip::ZipArchive::new(jarfile).unwrap();
    let internal_name = internal_path.as_os_str().to_str().unwrap();
    println!("Extracting: {}", internal_name);
    let mut internal_file = match archive.by_name(internal_name) {
        Ok(s) => s,
        Err(e) => {
            println!("Could not open internal file {}, because of the error: {}", internal_name, e);
            return;
        }
    };
    let mut write_file = fs::File::create(extracted_path).unwrap();
    io::copy(&mut internal_file, &mut write_file).expect("TODO: panic message");
}

pub fn extract_dlls_from_jar(bin_path: &Path, jar_path: &Path) {
    let jar_file = fs::File::open(jar_path).unwrap();
    let mut archive = zip::ZipArchive::new(jar_file).unwrap();
    let mut extracted_file_path: PathBuf = bin_path.into();

    let dll_names: Vec<String> = archive.file_names().filter(|n| n.ends_with(".dll")).map(String::from).collect();
    dll_names.iter().for_each(|internal_name| {
        let external_name = match internal_name.rsplit_once('/') {
            None => internal_name,
            Some((_, rhs)) => rhs,
        };
        extracted_file_path.push(external_name);
        //println!("Extracting dll file: {} to {}", internal_name, extracted_file_path.as_os_str().to_str().unwrap());
        let mut internal_file = archive.by_name(internal_name).unwrap();
        let mut external_file = fs::File::create(&extracted_file_path).unwrap();
        io::copy(&mut internal_file, &mut external_file).expect("TODO: panic message");
        extracted_file_path.pop();
    });
}