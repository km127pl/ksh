use super::directories::create_directory_if_not_exists;

#[cfg(windows)]
use dirs::config_dir;
#[cfg(not(windows))]
use dirs::home_dir;

use std::path::PathBuf;

#[cfg(windows)]
pub fn get_config_dir() -> Option<PathBuf> {
    let config_dir = config_dir().map(|path| path.join(".kterm"))?;
    create_directory_if_not_exists(&config_dir);
    Some(config_dir)
}

#[cfg(not(windows))]
pub fn get_config_dir() -> Option<PathBuf> {
    let config_dir = home_dir().map(|path| path.join(".kterm"))?;
    create_directory_if_not_exists(&config_dir);
    Some(config_dir)
}
