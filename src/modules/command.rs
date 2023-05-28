use lazy_static::lazy_static;

#[derive(PartialEq, Debug)]
pub enum CommandResult {
    Success,
    Failure,
    Exit,
}

pub struct Command {
    pub name: String,
    pub description: String,
    pub arguments: Vec<String>,
}

lazy_static! {
    pub static ref COMMANDS: Vec<Command> = vec![
        Command {
            name: "alias".to_owned(),
            description: "Creates an alias".to_owned(),
            arguments: vec!["alias".to_string(), "command".to_string()],
        },
        Command {
            name: "unalias".to_owned(),
            description: "Removes an alias".to_owned(),
            arguments: vec!["alias".to_string()],
        },
        Command {
            name: "clear".to_owned(),
            description: "Clear the screen".to_owned(),
            arguments: vec![],
        },
        Command {
            name: "cd".to_owned(),
            description: "Changes the directory".to_owned(),
            arguments: vec!["~|directory".to_string()],
        },
        Command {
            name: "read".to_owned(),
            description: "Read the contents of a file".to_owned(),
            arguments: vec!["file".to_string()],
        },
        Command {
            name: "write".to_owned(),
            description: "Write content to a file".to_owned(),
            arguments: vec!["file".to_string()],
        },
        Command {
            name: "ls".to_owned(),
            description: "List files in the current directory".to_owned(),
            arguments: vec![],
        },
        Command {
            name: "pwd".to_owned(),
            description: "Print the current directory".to_owned(),
            arguments: vec![],
        },
        Command {
            name: "wc".to_owned(),
            description: "Print the word count of a file".to_owned(),
            arguments: vec!["file".to_string()],
        },
        Command {
            name: "head".to_owned(),
            description: "Get the first 10 lines of a file".to_owned(),
            arguments: vec!["file".to_string()],
        },
        Command {
            name: "tail".to_owned(),
            description: "Get the last 10 lines of a file".to_owned(),
            arguments: vec!["file".to_string()],
        },
        Command {
            name: "exec".to_owned(),
            description: "Execute a command in the upper shell".to_owned(),
            arguments: vec!["command".to_string()],
        },
        Command {
            name: "touch".to_owned(),
            description: "Creates a new file".to_owned(),
            arguments: vec!["file".to_string()],
        },
        Command {
            name: "exit".to_owned(),
            description: "Exit the program".to_owned(),
            arguments: vec![],
        },
        Command {
            name: "neofetch".to_owned(),
            description: "Displays system information".to_owned(),
            arguments: vec![],
        },
        Command {
            name: "run".to_owned(),
            description: "Runs a script from a ksh file".to_owned(),
            arguments: vec!["file".to_string()],
        },
        Command {
            name: "edit".to_owned(),
            description: "Opens a file in the editor".to_owned(),
            arguments: vec!["file".to_string()],
        },
        Command {
            name: "mkdir".to_owned(),
            description: "Creates a new directory".to_owned(),
            arguments: vec!["directory".to_string()],
        },
        Command {
            name: "rmdir".to_owned(),
            description: "Removes a directory".to_owned(),
            arguments: vec!["directory".to_string()],
        },
        Command {
            name: "rm".to_owned(),
            description: "Removes a file".to_owned(),
            arguments: vec!["file".to_string()],
        },
        Command {
            name: "mv".to_owned(),
            description: "Moves a file".to_owned(),
            arguments: vec!["file".to_string(), "destination".to_string()],
        },
        Command {
            name: "cp".to_owned(),
            description: "Copies a file".to_owned(),
            arguments: vec!["file".to_string(), "destination".to_string()],
        },
        Command {
            name: "clock".to_owned(),
            description: "Shows the current time".to_owned(),
            arguments: vec![],
        },
    ];
}
