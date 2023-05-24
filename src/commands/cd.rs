use crate::modules::directories::change_directory;

pub fn cd_command(args: Vec<&str>) {
    if args.len() < 2 {
        println!("Not enough arguments provided for 'cd' (expected at least 1: [directory]).");
        return;
    }
    let directory = args[1];
    change_directory(directory);
}
