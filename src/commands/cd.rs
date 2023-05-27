use crate::modules::directories::change_directory;
use crate::modules::command::CommandResult;

pub fn cd_command(args: Vec<&str>) -> CommandResult {
    if args.len() < 2 {
        println!("Not enough arguments provided for 'cd' (expected at least 1: [directory]).");
        return CommandResult::Failure;
    }
    let directory = args[1];
    change_directory(directory);

    CommandResult::Success
}
