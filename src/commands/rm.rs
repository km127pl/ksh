use std::fs;

pub fn rm_command(args: Vec<&str>) {
    if args.len() < 2 {
        println!("Not enough arguments provided for 'rm' (expected at least 1: [file]).");
        return;
    }
    fs::remove_file(args[1]).expect("Error while deleting file")
}
