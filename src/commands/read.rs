pub fn read_command(args: Vec<&str>) {
    if args.len() < 2 {
        println!("Not enough arguments provided for 'read' (expected 1: [file name]).");
        return;
    }
    let contents: Result<String, std::io::Error> = std::fs::read_to_string(args[1]);

    match contents {
        Ok(file_contents) => println!("{}", file_contents),
        Err(err) => println!("Unable to read file '{}': {}", args[1], err),
    }
}
