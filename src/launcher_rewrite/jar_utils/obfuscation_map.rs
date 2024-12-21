use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Debug)]
pub struct ObfuscationMap {
    classes: HashMap<String, ClassFile>,
}

#[derive(Clone, Debug)]
pub struct ClassFile {
    deobfuscated_name: String,
    members: HashMap<String, String>,
}


pub fn from_map_file(file_path: &Path) -> ObfuscationMap {
    let file = File::open(file_path).expect("Failed to open File.");
    let mut lines = BufReader::new(file).lines().flatten();
    loop {
        if let Some(line) = lines.next() {
            if line.starts_with('#') || line.starts_with("    #") {
                continue
            }
            else {

            }
        } else {
            break
        }
    }

}