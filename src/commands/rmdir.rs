use crate::modules::directories::remove_directory;

use crate::modules::command::CommandResult;

pub fn rmdir_command(args: Vec<&str>) -> CommandResult {
    if args.len() < 2 {
        println!("Not enough arguments provided for 'rmdir' (expected at least 1: [directory]).");
        return CommandResult::Failure;
    }
    let directory: &str = args[1];
    remove_directory(&directory);

    CommandResult::Success
}
