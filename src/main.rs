use clap::Command;
use std::{fs::read_dir, os::unix::prelude::FileTypeExt};
struct AppInfo {
    name: &'static str,
    version: &'static str,
    about: &'static str,
    author: &'static str,
}

fn main() {
    let app_info = AppInfo {
        name: "syze",
        version: "0.0.1",
        about: "view the size of files",
        author: "Jim Ryan",
    };
    let _matches = Command::new(app_info.name)
        .version(app_info.version)
        .about(app_info.about)
        .author(app_info.author)
        .get_matches();

    let paths = read_dir("./").unwrap();

    for path in paths {
        let file_type = path.as_ref().unwrap().file_type().unwrap();

        println!(
            "{:?}, type: {:?}",
            path.as_ref().unwrap().file_name(),
            //TODO are these file types mutually exclusive
            match file_type {
                _ if file_type.is_dir() => "directory",
                _ if file_type.is_file() => "file",
                _ if file_type.is_symlink() => "symlink",
                _ if file_type.is_char_device() => "char device",
                _ if file_type.is_block_device() => "block device",
                _ if file_type.is_fifo() => "fifo",
                _ if file_type.is_socket() => "socket",
                _ => "idk",
            }
        );
    }
}
