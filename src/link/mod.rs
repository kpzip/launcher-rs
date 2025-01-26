use std::{fs, io};
use std::os::windows;
use std::path::Path;
use chrono::{TimeZone, Utc};

struct ShellLinkHeader<'a> {
    icon_index: i32,
    file_to_link_to: &'a Path,
}

impl ShellLinkHeader<'_> {

    fn write(&self, buf: &mut Vec<u8>) -> io::Result<()> {
        buf.reserve(76);
        const HEADER_SIZE: u32 = 0x0000004C;
        buf.extend(HEADER_SIZE.to_ne_bytes());
        const CLASS_ID: u128 = 0x00021401_0000_0000_C000_000000000046;
        buf.extend(CLASS_ID.to_ne_bytes());
        let link_flags: u32 = 0b0000_0000__0000_0000___0000_0000__0111_0000;
        buf.extend(link_flags.to_ne_bytes());
        let file_attributes: u32 = 0b0000_0000__0000_0000___0000_0000__1000_0000;
        buf.extend(file_attributes.to_ne_bytes());
        let creation_time: u64 = Utc::now().signed_duration_since(Utc.with_ymd_and_hms(1601, 1, 1, 0, 0, 0).unwrap()).abs().num_nanoseconds().unwrap() as u64/100;
        buf.extend(creation_time.to_ne_bytes());
        let access_time: u64 = creation_time;
        buf.extend(access_time.to_ne_bytes());
        let write_time = creation_time;
        buf.extend(write_time.to_ne_bytes());
        let file_size: u32 = fs::metadata(self.file_to_link_to)?.len() as u32;
        buf.extend(file_size.to_ne_bytes());
        buf.extend(self.icon_index.to_ne_bytes());
        let show_cmd: u32 = 0x00000001;
        buf.extend(show_cmd.to_ne_bytes());
        let hotkey_flags: u16 = 0x0000;
        buf.extend(hotkey_flags.to_ne_bytes());
        let res_1: u16 = 0x0000;
        buf.extend(res_1.to_ne_bytes());
        let res_2: u32 = 0x00000000;
        buf.extend(res_2.to_ne_bytes());
        let res_3: u32 = 0x00000000;
        buf.extend(res_3.to_ne_bytes());
        Ok(())
    }

}

fn create_link() {

}