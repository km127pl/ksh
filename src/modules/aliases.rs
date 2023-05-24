use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Write},
};

use super::config::get_config_dir;

pub fn load_aliases() -> Option<HashMap<String, String>> {
    let mut aliases: HashMap<String, String> = HashMap::new();

    let config_dir = get_config_dir()?;
    let aliases_file = config_dir.join("aliases");

    if !aliases_file.exists() {
        return None;
    }

    let file = File::open(aliases_file).ok()?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        if let Ok(line) = line {
            let parts: Vec<&str> = line.split('=').collect();
            if parts.len() == 2 {
                let alias = parts[0].trim().to_string();
                let command = parts[1].trim().to_string();
                aliases.insert(alias, command);
            }
        }
    }

    Some(aliases)
}

pub fn save_aliases(aliases: &HashMap<String, String>) {
    let config_dir = get_config_dir().expect("Failed to get config directory");
    let aliases_file = config_dir.join("aliases");

    let mut file = File::create(aliases_file).expect("Failed to create aliases file");

    for (alias, command) in aliases {
        let line = format!("{}={}\n", alias, command);
        file.write_all(line.as_bytes())
            .expect("Failed to write to aliases file");
    }
}
