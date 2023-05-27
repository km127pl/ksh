use std::fs::File;

use crate::modules::command::CommandResult;

pub fn touch_command(args: Vec<&str>) -> CommandResult {
    if args.len() < 2 {
        println!("Not enough arguments provided for 'touch' (expected at least 1: [file]).");
        return CommandResult::Failure;
    }
    let mut file = File::create(args[1]).expect("Error encountered while creating file!");
    std::io::Write::write(&mut file, b"").expect("Error while writing to file");

    CommandResult::Success
}
