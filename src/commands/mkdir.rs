use crate::modules::directories::create_directory;

use crate::modules::command::CommandResult;

pub fn mkdir_command(args: Vec<&str>) -> CommandResult {
    if args.len() < 2 {
        println!("Not enough arguments provided for 'mkdir' (expected at least 1: [directory]).");
        return CommandResult::Failure;
    }
    let directory: &str = args[1];
    create_directory(&directory);

    CommandResult::Success
}
