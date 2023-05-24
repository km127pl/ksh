pub fn pwd_command() {
    if let Ok(current_dir) = std::env::current_dir() {
        println!("Current directory: {}", current_dir.display());
    } else {
        println!("Unable to determine the current directory.");
    }
}
