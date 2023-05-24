use std::fs::File;

pub fn touch_command(args: Vec<&str>) {
    if args.len() < 2 {
        println!("Not enough arguments provided for 'touch' (expected at least 1: [file]).");
        return;
    }
    let mut file = File::create(args[1]).expect("Error encountered while creating file!");
    std::io::Write::write(&mut file, b"").expect("Error while writing to file");
}
