use std::io::{self, BufRead, BufReader};

use crate::modules::command::CommandResult;

pub fn tail_command(args: Vec<&str>) -> CommandResult {
    if args.len() < 2 {
        println!("Not enough arguments provided for 'tail' (expected 1: [file name]).");
        return CommandResult::Failure;
    }

    let file_path: &&str = &args[1];
    let num_lines: usize = 10; // Default number of lines to display

    if let Err(err) = tail(file_path, num_lines) {
        println!("Error: {}", err);
        return CommandResult::Failure;
    }

    CommandResult::Success
}

fn tail(file_path: &str, num_lines: usize) -> io::Result<()> {
    let file = std::fs::File::open(file_path)?;
    let reader = BufReader::new(file);
    let lines: Vec<_> = reader.lines().collect();
    let total_lines = lines.len();

    if total_lines <= num_lines {
        for line in lines {
            println!("{}", line?);
        }
    } else {
        for line in lines.iter().skip(total_lines - num_lines) {
            println!("{:?}", line);
        }
    }

    Ok(())
}
