use std::env;
use std::path::{PathBuf};
use std::collections::HashMap;

pub fn get_path_env() -> HashMap<String, PathBuf> {
    let path_env: String = env::var("PATH").unwrap_or_else(|_| String::new());
    let mut executables: HashMap<String, PathBuf> = HashMap::new();
    let paths: Vec<_> = env::split_paths(&path_env).collect();

    // iterate over each path and check if it's an exec
    for path in paths {
        if let Ok(entries) = path.read_dir() {
            for entry in entries {
                if let Ok(entry) = entry {
                    let file_path: PathBuf = entry.path();
                    if !file_path.is_file() || !is_executable(&file_path) {
                        continue;
                    }

                    if let Some(file_name) = file_path.file_stem() {
                        if let Some(name) = file_name.to_str() {
                            let _name = name.to_owned().to_ascii_lowercase();
                            executables.insert(_name, file_path.clone());
                        }
                    }
                }
            }
        }
    }


    executables
}

pub fn is_executable(path: &PathBuf) -> bool {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if let Ok(metadata) = path.metadata() {
            let permissions = metadata.permissions();
            return permissions.mode() & 0o111 != 0;
        }
    }

    #[cfg(windows)]
    {
        if let Some(extension) = path.extension() {
            if let Some(extension_str) = extension.to_str() {
                return extension_str.eq_ignore_ascii_case("exe");
            }
        }
    }

    false
}