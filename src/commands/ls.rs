use std::fs;

use chrono::{DateTime, Local};
use crossterm::style::Stylize;

use crate::modules::directories::convert_to_human_readable;

use crate::modules::command::CommandResult;

pub fn ls_command(args: Vec<&str>) -> CommandResult {
    let mut show_hidden = false;
    let mut show_long_format = false;
    let mut human_readable = false;

    for arg in &args[1..] {
        if arg.starts_with('-') {
            // Process multiple flags combined together
            for flag in arg.chars().skip(1) {
                match flag {
                    'a' => show_hidden = true,
                    'l' => show_long_format = true,
                    'h' => human_readable = true,
                    _ => {
                        println!("Invalid flag: -{}", flag);
                        return CommandResult::Failure;
                    }
                }
            }
        } else {
            println!("Invalid argument: {}", arg);
            return CommandResult::Failure;
        }
    }

    let current_dir = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(err) => {
            println!("Unable to determine the current directory: {}", err);
            return CommandResult::Failure;
        }
    };

    if let Ok(entries) = fs::read_dir(&current_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let file_name = entry.file_name();
                let metadata = entry.metadata().unwrap();

                // Skip hidden files if -a flag is not provided
                if !show_hidden && file_name.to_string_lossy().starts_with(".") {
                    continue;
                }

                if show_long_format {
                    let size = metadata.len();
                    let modified = metadata.modified().unwrap();
                    let modified: DateTime<Local> = modified.into();

                    if metadata.is_dir() {
                        print!("{}", "d".cyan());
                    } else if metadata.is_symlink() {
                        print!("{}", "l".magenta());
                    } else {
                        print!("{}", "f".yellow());
                    }

                    print!("  {}", modified.format("%b %d %H:%M"));

                    if human_readable {
                        let size = convert_to_human_readable(size);
                        print!(" {:>9}", size);
                    } else {
                        print!("{:<10} ", size);
                    }

                    if metadata.is_dir() {
                        println!(" {}", file_name.to_string_lossy().cyan());
                    } else if metadata.is_symlink() {
                        if let Some(_sym) = fs::canonicalize(entry.path()).ok() {
                            /*
                            TODO:This should display the symlink path as relative to the file itself
                            println!(" {} -> {:?}", file_name.to_string_lossy().magenta(), sym.to_owned().as_path());
                            */

                            println!(" {} -> ?", file_name.to_string_lossy().magenta());
                        }
                    } else {
                        println!(" {}", file_name.to_string_lossy().yellow());
                    }
                } else {
                    println!("{}", file_name.to_string_lossy().yellow());
                }
            }
        }
    } else {
        println!("{}", "Unable to read directory contents.".red());
        return CommandResult::Failure;
    }

    CommandResult::Success
}
