use std::{
    fs,
    io::{self, BufRead, BufReader},
};

pub fn head_command(args: Vec<&str>) {
    if args.len() < 2 {
        println!("Not enough arguments provided for 'head' (expected 1: [file name]).");
        return;
    }

    let file_path = &args[1];
    let num_lines = 10; // Default number of lines to display

    if let Err(err) = head(file_path, num_lines) {
        println!("Error: {}", err);
    }
}

fn head(file_path: &str, num_lines: usize) -> io::Result<()> {
    let file = fs::File::open(file_path)?;
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        if index >= num_lines {
            break;
        }

        println!("{}", line?);
    }

    Ok(())
}
