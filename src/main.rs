use clap::{Arg, Command};
use std::{
    fs::{read_dir, DirEntry},
    io,
    path::Path,
};

use colored::*;
struct AppInfo {
    name: &'static str,
    version: &'static str,
    about: &'static str,
    author: &'static str,
}

struct Argument {
    id: &'static str,
    required: bool,
    index: usize,
}

fn main() {
    let app_info = AppInfo {
        name: "syze",
        version: "0.0.1",
        about: "view the size of files",
        author: "Jim Ryan",
    };

    let file_or_folder_argument = Argument {
        id: "file_or_folder",
        required: false,
        index: 1,
    };

    let matches = Command::new(app_info.name)
        .version(app_info.version)
        .about(app_info.about)
        .author(app_info.author)
        .arg(
            Arg::new(file_or_folder_argument.id)
                .required(file_or_folder_argument.required)
                .index(file_or_folder_argument.index),
        )
        .get_matches();

    let folder_if_none = "./".to_string();

    let file_or_folder = matches
        .get_one::<String>(file_or_folder_argument.id)
        .unwrap_or(&folder_if_none);

    let is_folder = Path::new(file_or_folder).is_dir();

    match is_folder {
        true => {
            print_info_from_folder(&file_or_folder, 0);
            print_total_size(&file_or_folder);
        }
        false => {
            print_file_size(&file_or_folder);
        }
    }
}

fn print_file_size(path: &str) {
    let file_name = Path::new(path).file_name().unwrap().to_string_lossy();
    let file_size = get_dir_size(Path::new(path)).unwrap();
    println!(
        "{}: {}",
        file_name.blue().bold(),
        display_size(file_size).cyan().bold()
    );
}

fn print_total_size(path: &str) {
    let total_size = get_dir_size(Path::new(path)).unwrap();
    println!(
        "{} {}",
        "Total size:".blue().bold(),
        display_size(total_size).cyan().bold()
    );
}

fn print_info_from_folder(path: &str, level: u8) {
    let paths: Vec<_> = read_dir(path).unwrap().collect();

    let mut folders: Vec<&DirEntry> = vec![];
    let mut files: Vec<&DirEntry> = vec![];

    for path in &paths {
        let entry = path.as_ref().unwrap();
        let file_type = entry.file_type().unwrap();
        if file_type.is_dir() {
            folders.push(entry)
        } else {
            files.push(entry)
        }
    }

    folders.sort_by_key(|entry| entry.file_name().to_owned());
    files.sort_by_key(|entry| entry.file_name().to_owned());

    for folder in &folders {
        let entry = *folder;

        let size = get_dir_size(entry.path().as_path()).unwrap();

        let folder_path = entry.path();
        let folder_name = folder_path.file_name().unwrap().to_string_lossy();

        if folder_name.starts_with(".") {
            continue;
        }

        let spaces = " ".repeat((level).into());
        print!("{}", spaces);

        println!("📁{} {}", folder_name, display_size(size));
        print_info_from_folder(entry.path().to_str().unwrap(), level + 1);
    }
    for file in &files {
        let entry = *file;

        let size = get_dir_size(entry.path().as_path()).unwrap();

        let file_path = entry.path();
        let file_name = file_path.file_name().unwrap().to_string_lossy();

        if file_name.starts_with(".") {
            continue;
        }

        let spaces = " ".repeat((level).into());
        print!("{}", spaces);

        match is_executable(entry) {
            Ok(executable) => match executable {
                true => {
                    println!("📄{} {}", file_name.green(), display_size(size).cyan());
                }
                false => {
                    println!("📄{} {}", file_name, display_size(size).cyan());
                }
            },
            Err(_) => {
                println!("📄{} {}", file_name, display_size(size));
            }
        }
    }
}

fn get_dir_size(path: &Path) -> io::Result<u64> {
    let mut size = 0;

    if path.is_file() {
        size += path.metadata()?.len();
    } else {
        for entry in read_dir(path)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                size += path.metadata()?.len();
            } else {
                size += get_dir_size(&path)?;
            }
        }
    }

    Ok(size)
}

fn display_size(size: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if size < KB {
        return format!("{} B", size);
    } else if size < MB {
        return format!("{:.2} KB", (size as f64) / (KB as f64));
    } else if size < GB {
        return format!("{:.2} MB", (size as f64) / (MB as f64));
    } else {
        return format!("{:.2} GB", (size as f64) / (GB as f64));
    }
}

fn is_executable(entry: &DirEntry) -> io::Result<bool> {
    let metadata = entry.metadata()?;
    let permisisons = metadata.permissions();

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        return Ok(permisisons.mode() & 0o111 != 0);
    }

    #[cfg(not(unix))]
    {
        return Ok(!permisisons.readonly());
    }
}
