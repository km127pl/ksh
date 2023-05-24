use crate::modules::directories::remove_directory;

pub fn rmdir_command(args: Vec<&str>) {
    if args.len() < 2 {
        println!("Not enough arguments provided for 'rmdir' (expected at least 1: [directory]).");
        return;
    }
    let directory = args[1];
    remove_directory(&directory);
}
