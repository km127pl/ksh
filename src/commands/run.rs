use std::collections::HashMap;
use std::process::exit;
use std::{fmt, fs};

use crate::modules::command::CommandResult;

#[derive(Debug)]
enum CommandType {
    PRINT,
    INPUT,
    GOTO,
    READ,
    WRITE,
    BREAK,
    UNDEFINED,
    VAR, // New command: VAR
    IF,  // New command: IF
    INC, // New command: INC
}

impl fmt::Display for CommandType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CommandType::BREAK => write!(f, "BREAK"),
            CommandType::PRINT => write!(f, "PRINT"),
            CommandType::GOTO => write!(f, "GOTO"),
            CommandType::WRITE => write!(f, "WRITE"),
            CommandType::UNDEFINED => write!(f, "UNDEFINED"),
            CommandType::READ => write!(f, "READ"),
            CommandType::INPUT => write!(f, "INPUT"),
            CommandType::VAR => write!(f, "VAR"),
            CommandType::IF => write!(f, "IF"),
            CommandType::INC => write!(f, "INC"),
        }
    }
}

#[derive(Debug)]
struct CommandOutput {
    line: usize,
    exit: bool,
}

#[derive(Debug)]
struct Command {
    id: u8,
    function: CommandType,
    args: String,
}

pub fn run_command(args: Vec<&str>) -> CommandResult {
    if args.len() < 2 {
        println!("Not enough arguments provided for 'run' (expected 1: [file name]).");
        return CommandResult::Failure;
    }
    let contents: Result<String, std::io::Error> = std::fs::read_to_string(args[1]);

    match contents {
        Ok(file_contents) => {
            exec_script(file_contents);
        }
        Err(err) => println!("Unable to read file '{}': {}", args[1], err),
    }

    if args.len() > 2 {
        if &args[2] == &"env" {
            // we are being called from the executable alone
            std::process::exit(0)
        }
    }

    CommandResult::Success
}

fn exec_script(script: String) {
    let lines: Vec<&str> = script.split("\n").map(|line| line.trim()).collect();
    let mut line_number: usize = 0;
    let mut variables: HashMap<String, String> = HashMap::new();
    let mut should_break = false;

    while line_number < lines.len() {
        let line = lines[line_number];
        if line.starts_with(";") || line.starts_with("#") {
            continue; // a comment
        };
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() >= 2 {
            let id: Result<u8, std::num::ParseIntError> = parts[0].parse::<u8>();
            let command: CommandType = get_command_type(parts[1]);
            let args: String = parts[2..].join(" ");
            match id {
                Ok(id) => {
                    let cmd = Command {
                        id,
                        function: command,
                        args: args.to_string(),
                    };
                    let output: CommandOutput =
                        execute_command(cmd, line_number, &lines, &mut variables);
                    line_number = output.line;
                    should_break = output.exit;
                }
                Err(_) => println!("Invalid command ID on line: {}", line),
            }
        }

        if should_break {
            return;
        };

        line_number += 1;
    }
}

fn get_command_type(cmd: &str) -> CommandType {
    match cmd {
        "PRINT" => CommandType::PRINT,
        "INPUT" => CommandType::INPUT,
        "GOTO" => CommandType::GOTO,
        "READ" => CommandType::READ,
        "WRITE" => CommandType::WRITE,
        "BREAK" => CommandType::BREAK,
        "VAR" => CommandType::VAR,
        "IF" => CommandType::IF,
        "INC" => CommandType::INC,
        _ => CommandType::UNDEFINED,
    }
}

fn execute_command(
    cmd: Command,
    line_number: usize,
    lines: &[&str],
    variables: &mut HashMap<String, String>,
) -> CommandOutput {
    match cmd.function {
        CommandType::PRINT => {
            let value = process_print_arguments(&cmd.args, variables);
            println!("{}", value);
            CommandOutput {
                exit: false,
                line: line_number,
            }
        }
        CommandType::INPUT => {
            let mut input = String::new();
            match std::io::stdin().read_line(&mut input) {
                Ok(_) => {
                    let var_name = cmd.args.trim().to_string();
                    variables.insert(var_name, input.trim().to_string());
                    CommandOutput {
                        exit: false,
                        line: line_number,
                    }
                }
                Err(error) => {
                    println!("Error reading input: {}", error);
                    CommandOutput {
                        exit: true,
                        line: line_number,
                    }
                }
            }
        }
        CommandType::GOTO => {
            if let Ok(target_line) = cmd.args.parse::<usize>() {
                if target_line > 0 && target_line <= lines.len() {
                    CommandOutput {
                        exit: false,
                        line: target_line - 1,
                    }
                } else {
                    println!("Invalid line number: {}", cmd.args);
                    CommandOutput {
                        exit: false,
                        line: line_number,
                    }
                }
            } else {
                println!("Invalid line number: {}", cmd.args);
                CommandOutput {
                    exit: false,
                    line: line_number,
                }
            }
        }
        CommandType::READ => match fs::read_to_string(&cmd.args) {
            Ok(file_contents) => {
                variables.insert("$READ".to_owned(), file_contents.trim().to_string());
                CommandOutput {
                    exit: false,
                    line: line_number,
                }
            }
            Err(err) => {
                println!("Unable to read file '{}': {}", cmd.args, err);
                CommandOutput {
                    exit: true,
                    line: line_number,
                }
            }
        },
        CommandType::WRITE => {
            let args: Vec<&str> = cmd.args.splitn(2, ' ').collect();
            if args.len() == 2 {
                let file_name = args[0];
                let content = process_print_arguments(args[1], variables);
                match fs::write(file_name, content) {
                    Ok(_) => CommandOutput {
                        exit: false,
                        line: line_number,
                    },
                    Err(err) => {
                        println!("Unable to write to file '{}': {}", file_name, err);
                        CommandOutput {
                            exit: true,
                            line: line_number,
                        }
                    }
                }
            } else {
                println!("Invalid arguments for WRITE command: {}", cmd.args);
                CommandOutput {
                    exit: true,
                    line: line_number,
                }
            }
        }
        CommandType::BREAK => CommandOutput {
            exit: true,
            line: line_number,
        },
        CommandType::VAR => {
            let args: Vec<&str> = cmd.args.splitn(2, ' ').collect();
            if args.len() == 2 {
                let var_name = args[0].to_string();
                let var_value = process_print_arguments(args[1], variables);
                variables.insert(var_name, var_value);
                CommandOutput {
                    exit: false,
                    line: line_number,
                }
            } else {
                println!("Invalid arguments for VAR command: {}", cmd.args);
                CommandOutput {
                    exit: true,
                    line: line_number,
                }
            }
        }
        CommandType::IF => {
            let condition_parts: Vec<&str> = cmd.args.split_whitespace().collect();
            if condition_parts.len() >= 4 {
                let left_operand = condition_parts[0];
                let operator = condition_parts[1];
                let right_operand = condition_parts[2];
                let goto_line = condition_parts[3];

                let left_value = variables.get(left_operand);
                let right_value = variables.get(right_operand);

                if let (Some(left_value), Some(right_value)) = (left_value, right_value) {
                    let condition_met = match operator {
                        ">" => left_value > right_value,
                        "<" => left_value < right_value,
                        ">=" => left_value >= right_value,
                        "<=" => left_value <= right_value,
                        "==" => left_value == right_value,
                        "!=" => left_value != right_value,
                        _ => false,
                    };

                    if condition_met {
                        if let Ok(target_line) = goto_line.parse::<usize>() {
                            if target_line > 0 && target_line <= lines.len() {
                                return CommandOutput {
                                    exit: false,
                                    line: target_line - 1,
                                };
                            } else {
                                println!("Invalid line number: {}", cmd.args);
                            }
                        } else {
                            println!("Invalid line number: {}", cmd.args);
                        }
                    } else {
                        return CommandOutput {
                            exit: false,
                            line: line_number + 1,
                        };
                    }
                } else {
                    println!("Invalid operands in IF command: {}", cmd.args);
                }
            } else {
                println!("Invalid arguments for IF command: {}", cmd.args);
            }

            CommandOutput {
                exit: true,
                line: line_number,
            }
        }
        CommandType::INC => {
            let var_name = cmd.args.trim().to_string();
            let var_value = match variables.get(&var_name) {
                Some(value) => {
                    if let Ok(mut int_value) = value.parse::<i32>() {
                        int_value += 1;
                        int_value.to_string()
                    } else {
                        println!("Cannot increment non-integer variable: {}", var_name);
                        return CommandOutput {
                            exit: true,
                            line: line_number,
                        };
                    }
                }
                None => {
                    println!("Variable not found: {}", var_name);
                    return CommandOutput {
                        exit: true,
                        line: line_number,
                    };
                }
            };
            variables.insert(var_name, var_value);
            CommandOutput {
                exit: false,
                line: line_number,
            }
        }
        CommandType::UNDEFINED => {
            println!("Undefined command, line: {} args: {}", cmd.id, cmd.args);
            CommandOutput {
                exit: true,
                line: line_number,
            }
        }
    }
}

fn process_print_arguments(args: &str, variables: &HashMap<String, String>) -> String {
    let mut output = String::new();
    let parts: Vec<&str> = args.split_whitespace().collect();

    for part in parts {
        if part.starts_with('"') && part.ends_with('"') && part.len() >= 2 {
            // Print literal string
            output.push_str(&part[1..part.len() - 1]);
        } else if let Some(value) = variables.get(part) {
            // Print variable value
            output.push_str(value);
        } else {
            // Print unrecognized value as-is
            output.push_str(part);
        }
        output.push(' '); // Add space between arguments
    }

    output.replace("\"", "").trim().to_string()
}
