use std::fs;

pub fn rm_command(args: Vec<&str>) {
    if args.len() < 2 {
        println!("Not enough arguments provided for 'rm' (expected at least 1: [file]).");
        return;
    }
    let file_name: &str = args[1];
    let file: &std::path::Path = std::path::Path::new(file_name);

    if !file.exists() {
        println!("'{}' does not exist.", file_name);
        return;
    }

    if !file.is_file() {
        println!("'{}' is not a file.", file_name);
        return;
    }

    match fs::remove_file(file) {
        Ok(_) => {}
        Err(err) => {
            println!("Failed to remove '{}': {}", file_name, err);
        }
    }
}
