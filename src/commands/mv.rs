pub fn mv_command(args: Vec<&str>) {
    if args.len() < 3 {
        println!("Not enough arguments provided for 'mv' (expected 2: [source] [destination]).");
        return;
    }

    let source: &std::path::Path = std::path::Path::new(args[1]);
    let destination: &std::path::Path = std::path::Path::new(args[2]);

    if !source.exists() {
        println!("Source '{}' does not exist.", source.display());
        return;
    }

    if !source.is_file() {
        println!("Source '{}' is not a file.", source.display());
        return;
    }

    let source_file_name = source.file_name().unwrap();
    let destination_file: std::path::PathBuf;

    if !destination.exists() {
        // we assume that the user wants to rename the file
        destination_file = std::path::Path::new(destination).to_path_buf();
    } else {
        // we assume that the user wants to move the file
        destination_file = destination.join(source_file_name);
    }

    if destination.exists() {
        println!("Destination file '{}' already exists.", destination.display());
        return;
    }

    match std::fs::rename(source, destination_file) {
        Ok(_) => {},
        Err(err) => {
            println!("Failed to move '{}': {}", source.display(), err);
        }
    }
}