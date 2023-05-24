use crate::modules::directories::create_directory;

pub fn mkdir_command(args: Vec<&str>) {
    if args.len() < 2 {
        println!("Not enough arguments provided for 'mkdir' (expected at least 1: [directory]).");
        return;
    }
    let directory = args[1];
    create_directory(&directory);
}
