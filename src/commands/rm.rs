use std::fs;

pub fn rm_command(args: Vec<&str>) {
    if args.len() < 2 {
        println!("Not enough arguments provided for 'rm' (expected at least 1: [file]).");
        return;
    }
    let file = args[1];
    if fs::metadata(file).unwrap().is_file() {
        fs::remove_dir(file).expect("Failed to remove file");
    } else {
        println!("'{}' is not a file.", file);
    }
}
