use crate::commands;

use super::command::CommandResult;
use commands::cd::cd_command;
use commands::clear::clear_command;
use commands::clock::clock_command;
use commands::cp::cp_command;
use commands::edit::edit_command;
use commands::exec::exec_command;
use commands::exit::exit_command;
use commands::head::head_command;
use commands::help::help_command;
use commands::ls::ls_command;
use commands::mkdir::mkdir_command;
use commands::mv::mv_command;
use commands::neofetch::neofetch_command;
use commands::pwd::pwd_command;
use commands::read::read_command;
use commands::rm::rm_command;
use commands::rmdir::rmdir_command;
use commands::run::{run_command};
use commands::tail::tail_command;
use commands::touch::touch_command;
use commands::wc::wc_command;
use commands::write::write_command;

#[cfg(test)]

#[test]
fn test_commands() {
    // dummy data

    use std::collections::HashMap;

    use crossterm::style::Stylize;
    let mut aliases: HashMap<String, String> = HashMap::new();
    let mut env_args: Vec<&str> = Vec::new();
    let test_directory: &str = "tests";
    let mut i : u8 = 0;
    let commands: [&str; 22] = ["cd", "clear", "clock", "cp", "edit", "exec", "exit", "head", "help", "ls", "mkdir", "mv", "neofetch", "pwd", "read", "rm", "rmdir", "run", "tail", "touch", "wc", "write"];
    let mut result : CommandResult;
    env_args.push("");
    env_args.push(test_directory);
    
    // delete test directory
    println!("ℹ Test {}/{} - {}", i, commands.len(), "delete test directory".blue());
    result = rmdir_command(env_args.clone());
    assert_eq!(result, CommandResult::Success);

    // mkdir
    i += 1;
    println!("ℹ Test {}/{} - {}", i, commands.len(), "mkdir".blue());
    result = mkdir_command(env_args.clone());
    assert_eq!(result, CommandResult::Success);
    println!("✔ Test {}/{} - {} passed", i, commands.len(), "mkdir".green());

    // cd
    i += 1;
    println!("ℹ Test {}/{} - {}", i, commands.len(), "cd".blue());
    result = cd_command(env_args.clone());
    assert_eq!(result, CommandResult::Success);
    println!("✔ Test {}/{} - {} passed", i, commands.len(), "cd".green());

    // clear
    i += 1;
    println!("ℹ Test {}/{} - {}", i, commands.len(), "clear".blue());
    result = clear_command();
    assert_eq!(result, CommandResult::Success);
    println!("✔ Test {}/{} - {} passed", i, commands.len(), "clear".green());

    // clock
    i += 1;
    println!("ℹ Test {}/{} - {}", i, commands.len(), "clock".blue());
    result = clock_command();
    assert_eq!(result, CommandResult::Success);
    println!("✔ Test {}/{} - {} passed", i, commands.len(), "clock".green());

    // touch
    i += 1;
    println!("ℹ Test {}/{} - {}", i, commands.len(), "touch".blue());
    result = touch_command(vec!["", "test.txt"]);
    assert_eq!(result, CommandResult::Success);
    println!("✔ Test {}/{} - {} passed", i, commands.len(), "touch".green());

    // write (skipped, interactive)
    i += 1;
    println!("ℹ Test {}/{} - {}", i, commands.len(), "write".blue());
    println!("✔ Test {}/{} - {} skipped", i, commands.len(), "write".yellow());
    
}
