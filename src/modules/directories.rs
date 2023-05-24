use std::{
    fs,
    path::{Path, PathBuf},
};

#[cfg(not(windows))]
use dirs::home_dir;

pub fn change_directory(directory: &str) {
    let new_dir: PathBuf;

    if directory == "-" {
        // Switch to the previous directory
        if let Some(previous_dir) = std::env::var("OLDPWD").ok() {
            new_dir = PathBuf::from(previous_dir);
        } else {
            println!("Previous directory not found.");
            return;
        }
    } else if directory == "~" {
        // switch to user directory
        new_dir = PathBuf::from(format!("C:\\Users\\{}", whoami::username()));
    } else {
        // change to the specified directory
        new_dir = PathBuf::from(directory);
    }

    // Attempt to change the current directory
    if let Err(err) = std::env::set_current_dir(&new_dir) {
        println!("Failed to change directory: {}", err);
        return;
    }
}

#[cfg(windows)]
pub fn parse_directory_path(path: String) -> String {
    let home_dir = format!("C:\\Users\\{}", whoami::username());
    if path.starts_with(&home_dir) {
        let relative_path = path.trim_start_matches(&home_dir);
        format!("~{}", relative_path)
    } else {
        path
    }
}

pub fn create_directory_if_not_exists(path: &Path) {
    if !path.exists() {
        if let Err(err) = fs::create_dir_all(path) {
            eprintln!("Failed to create directory: {:?}", err);
        }
    }
}

pub fn create_directory(directory: &str) {
    if let Err(err) = fs::create_dir_all(directory) {
        println!("Failed to create directory '{}': {}", directory, err);
    } else {
        println!("Directory '{}' created successfully.", directory);
    }
}

pub fn convert_to_human_readable(size: u64) -> String {
    let units = ["B", "KB", "MB", "GB", "TB"];
    let mut size = size as f64;
    let mut index = 0;

    while size >= 1024.0 && index < units.len() - 1 {
        size /= 1024.0;
        index += 1;
    }

    format!("{:.1} {}", size, units[index])
}
