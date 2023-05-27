use std::{io, process::Command};

use crossterm::style::Stylize;

pub fn exec_command(args: Vec<&str>) {
    if args.len() < 2 {
        println!("Not enough arguments provided for 'exec' (expected at least 1: [command]).");
        return;
    }

    let command: String = args[1..].join(" ");

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
                    eprintln!("{}", stderr);
                }
            } else {
                eprintln!(
                    "{}\n{}",
                    format!(
                        "Command '{}' exited with code {}.",
                        args[1],
                        output.status.code().unwrap_or(1)
                    ).red(),
                    String::from_utf8_lossy(&output.stderr).trim_end()
                    .red()
                );
            }
        }
        Err(err) => {
            eprintln!("{} {}", "Failed to execute command:".red(), err);
        }
    }
}
