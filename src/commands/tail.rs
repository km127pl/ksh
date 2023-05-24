use std::io::{self, BufRead, BufReader};

pub fn tail_command(args: Vec<&str>) {
    if args.len() < 2 {
        println!("Not enough arguments provided for 'tail' (expected 1: [file name]).");
        return;
    }

    let file_path = &args[1];
    let num_lines = 10; // Default number of lines to display

    if let Err(err) = tail(file_path, num_lines) {
        println!("Error: {}", err);
    }
}

fn tail(file_path: &str, num_lines: usize) -> io::Result<()> {
    let file = std::fs::File::open(file_path)?;
    let reader = BufReader::new(file);
    let lines: Vec<_> = reader.lines().collect();
    let total_lines = lines.len();

    if total_lines <= num_lines {
        for line in lines {
            println!("{}", line?);
        }
    } else {
        for line in lines.iter().skip(total_lines - num_lines) {
            println!("{:?}", line);
        }
    }

    Ok(())
}
