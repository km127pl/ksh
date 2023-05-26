use commands::cd::cd_command;
use commands::clear::clear_command;
use commands::edit::edit_command;
use commands::exec::exec_command;
use commands::exit::exit_command;
use commands::head::head_command;
use commands::help::help_command;
use commands::ls::ls_command;
use commands::mkdir::mkdir_command;
use commands::neofetch::neofetch_command;
use commands::pwd::pwd_command;
use commands::read::read_command;
use commands::rm::rm_command;
use commands::rmdir::rmdir_command;
use commands::run::run_command;
use commands::tail::tail_command;
use commands::touch::touch_command;
use commands::wc::wc_command;
use commands::write::write_command;

use modules::aliases::load_aliases;

use core::time;

use std::io::Write;
use std::{collections::HashMap, thread};
use text_io::read;

use crossterm::style::Stylize;

use crate::modules::aliases::save_aliases;
use crate::modules::directories::parse_directory_path;
pub mod commands {
    pub mod cd;
    pub mod clear;
    pub mod edit;
    pub mod exec;
    pub mod exit;
    pub mod head;
    pub mod help;
    pub mod ls;
    pub mod mkdir;
    pub mod neofetch;
    pub mod pwd;
    pub mod read;
    pub mod rm;
    pub mod run;
    pub mod tail;
    pub mod touch;
    pub mod wc;
    pub mod write;
    pub mod rmdir;
}

pub mod modules {
    pub mod aliases;
    pub mod config;
    pub mod directories;
    pub mod display;
    pub mod systeminfo;
}

fn main() {
    let env_args: Vec<String> = std::env::args().collect();
    let mut aliases: HashMap<String, String> = load_aliases().unwrap_or_else(HashMap::new);
    if env_args.len() > 1 {
        match &env_args[1].as_ref() {
            &"-c" => {
                if env_args.len() < 3 {
                    println!("{}", "You must supply a command to execute!".red());
                    return;
                }
                let command = &env_args[2..].join(" ");
                println!("{:?}", aliases);
                execute_command(command, &aliases);
                return;
            }
            _ => run_command(["", &env_args[1], "env"].to_vec())
        }
    }
    ctrlc::set_handler(|| {
        // Handle Ctrl-C signal
    })
    .expect("Error setting Ctrl-C handler");

    loop {
        let username: String = whoami::username();

        let prompt: String;
        if let Ok(current_dir) = std::env::current_dir() {
            prompt = String::from(format!(
                "{} {} {} ",
                username.cyan(),
                parse_directory_path(current_dir.display().to_string()).blue(),
                "#".cyan().bold()
            ));
        } else {
            prompt = String::from(format!("{}#", &username));
        }
        print!("{prompt}");
        std::io::stdout().flush().unwrap(); // Flush stdout to ensure prompt is printed immediately

        thread::sleep(time::Duration::from_millis(10));
        let command: String = read!("{}\n");
        let args: Vec<&str> = command.split_whitespace().collect();
        if args.len() == 0 || args[0].trim().is_empty() {
            continue;
        };
        if command.starts_with("alias") {
            // Process alias command
            let parts: Vec<&str> = command.split('=').collect();
            let split: Vec<&str> = command.split(" ").collect();
            if parts.len() != 2 {
                if let Some(alias_value) = aliases.get(split[1].trim()) {
                    // show aliases info;
                    println!("Alias of {:?} is {:?}", split[1].trim(), alias_value);
                    continue;
                }
                println!("{} {}", split[1], "is not a valid alias.".red());
                continue;
            }

            let alias = parts[0].trim().replace("alias ", "");
            let cmd = parts[1].trim().to_string();

            aliases.insert(alias, cmd);
            save_aliases(&aliases);
            continue;
        } else if command.starts_with("unalias") {
            // Process alias command
            let parts: Vec<&str> = command.split(' ').collect();
            let _cmd = parts[1].trim().to_string();
            let cmd = _cmd.trim();
            drop(&_cmd);

            if !aliases.contains_key(cmd) {
                println!("{} {}", cmd, "is not a valid alias.".red());
                continue;
            }

            aliases.remove(cmd);
            println!("{} {}", "Removed the alias".red(), &cmd);
            continue;
        }
    }
}

fn execute_command(command: &String, aliases: &HashMap<String, String>) {
    let mut args: Vec<&str> = command.split_whitespace().collect();
    
    if let Some(alias_cmd) = aliases.get(args[0]) { // check for aliases
        // a bit wasteful, but will work for now
        args = alias_cmd.split_whitespace().collect();
    }
    
    match args[0] {
        "read" | "cat" => read_command(args),
        "write" => write_command(args),
        "pwd" => pwd_command(),
        "mkdir" => mkdir_command(args),
        "rmdir" => rmdir_command(args),
        "clear" => clear_command(),
        "touch" => touch_command(args),
        "rm" => rm_command(args),
        "ls" => ls_command(args),
        "head" => head_command(args),
        "cd" => cd_command(args),
        "exec" => exec_command(args),
        "tail" => tail_command(args),
        "neofetch" => neofetch_command(),
        "run" => run_command(args),
        "wc" => wc_command(args),
        "edit" | "nano" => edit_command(args),
        "exit" | "quit" => exit_command(&aliases),
        "help" => help_command(),
        _ => {
            //TODO: using a hashmap of all executables in $path, run that executable if it matches our command
            println!(
                "{} {}",
                "Invalid command:".red(),
                String::from(args[0]).yellow()
            );
        }
    }
}
