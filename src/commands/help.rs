use crossterm::style::Stylize;

use crate::{modules::command::{CommandResult, Command, COMMANDS, self}, commands};


// todo, this should be either dynamic, or a hashmap of COMMAND, ARGS, DESCRIPTION
// like a struct
pub fn help_command() -> CommandResult {
    
    println!("{}", "Available commands:".bold().cyan());

    for command in COMMANDS.iter() {
        print_command(&command);
    }

    CommandResult::Success
}

fn print_command(command: &Command) {
    let arguments: String = command
        .arguments
        .iter()
        .map(|arg| format!("[{}]", arg))
        .collect::<Vec<String>>()
        .join(" ");
    println!(
        "  {:<19} [{}] - {}",
        command.name.to_owned().cyan().bold(),
        arguments.blue(),
        command.description.to_owned()
    );
}
