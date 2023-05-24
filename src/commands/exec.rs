use std::{io, process::Command};

use crossterm::style::Stylize;

pub fn exec_command(args: Vec<&str>) {
    if args.len() < 2 {
        println!("Not enough arguments provided for 'exec' (expected at least 1: [command]).");
        return;
    }

    let command = args[1..].join(" ");

    let output: Result<std::process::Output, io::Error> =
        Command::new("cmd").args(&["/C", &command]).output();

    match output {
        Ok(output) => {
            if output.status.success() {
                let stdout: std::borrow::Cow<str> = String::from_utf8_lossy(&output.stdout);
                let stderr: std::borrow::Cow<str> = String::from_utf8_lossy(&output.stderr);

                if !stdout.is_empty() {
                    println!("{}", stdout);
                }

                if !stderr.is_empty() {
                    eprintln!("{}", stderr.red());
                }
            } else {
                eprintln!("{} {}", "Command failed:".red(), output.status);
            }
        }
        Err(err) => {
            eprintln!("{} {}", "Failed to execute command:".red(), err);
        }
    }
}
