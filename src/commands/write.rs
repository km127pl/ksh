use std::fs;
use text_io::read;
use crate::modules::command::CommandResult;

pub fn write_command(args: Vec<&str>) -> CommandResult {
    if args.len() < 2 {
        println!("Not enough arguments provided for 'write' (expected 1: [file name]).");
        return CommandResult::Failure;
    }

    println!("Enter lines of content (type 'EOL' to finish):");
    let mut content: String = String::new();
    let file_name: &str = args[1];

    loop {
        let line: String = read!("{}\n");
        if line.trim() == "EOL" {
            break;
        }
        content.push_str(&line);
        content.push('\n');
    }

    match fs::write(file_name, content) {
        Ok(_) => println!("Successfully wrote to file '{}'.", file_name),
        Err(err) => {
            println!("Unable to write to file '{}': {}", file_name, err);
            return CommandResult::Failure;
        }
    }

    CommandResult::Success
}
