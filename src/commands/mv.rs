use crate::modules::command::CommandResult;

pub fn mv_command(args: Vec<&str>) -> CommandResult {
    if args.len() < 3 {
        println!("Not enough arguments provided for 'mv' (expected 2: [source] [destination]).");
        return CommandResult::Failure;
    }

    let source: &std::path::Path = std::path::Path::new(args[1]);
    let destination: &std::path::Path = std::path::Path::new(args[2]);

    if !source.exists() {
        println!("Source '{}' does not exist.", source.display());
        return CommandResult::Failure;
    }

    if !source.is_file() {
        println!("Source '{}' is not a file.", source.display());
        return CommandResult::Failure;
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
        println!(
            "Destination file '{}' already exists.",
            destination.display()
        );
        return CommandResult::Failure;
    }

    match std::fs::rename(source, destination_file) {
        Ok(_) => {}
        Err(err) => {
            println!("Failed to move '{}': {}", source.display(), err);
            return CommandResult::Failure;
        }
    }

    CommandResult::Success
}
