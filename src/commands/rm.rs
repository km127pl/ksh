use std::fs;
use crate::modules::command::CommandResult;

pub fn rm_command(args: Vec<&str>) -> CommandResult {
    if args.len() < 2 {
        println!("Not enough arguments provided for 'rm' (expected at least 1: [file]).");
        return CommandResult::Failure;
    }
    let file_name: &str = args[1];
    let file: &std::path::Path = std::path::Path::new(file_name);

    if !file.exists() {
        println!("'{}' does not exist.", file_name);
        return CommandResult::Failure;
    }

    if !file.is_file() {
        println!("'{}' is not a file.", file_name);
        return CommandResult::Failure;
    }

    match fs::remove_file(file) {
        Ok(_) => {}
        Err(err) => {
            println!("Failed to remove '{}': {}", file_name, err);
            return CommandResult::Failure;
        }
    }

    CommandResult::Success
}
