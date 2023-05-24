use std::{
    fs,
    io::{self, BufRead, BufReader},
};

pub fn wc_command(args: Vec<&str>) {
    if args.len() < 2 {
        println!("Not enough arguments provided for 'wc' (expected 1: [file name]).");
        return;
    }

    let file_path = &args[1];

    if let Err(err) = wc(file_path) {
        println!("Error: {}", err);
    }
}

fn wc(file_path: &str) -> io::Result<()> {
    let file = fs::File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut line_count = 0;
    let mut word_count = 0;
    let mut char_count = 0;

    for line in reader.lines() {
        let line = line?;
        line_count += 1;
        word_count += line.split_whitespace().count();
        char_count += line.chars().count();
    }

    let output = format!("{} {} {}", line_count, word_count, char_count);
    let column_widths: Vec<_> = output.split_whitespace().map(|s| s.len()).collect();

    println!(
        "{: >width$} {: >width$} {: >width$}",
        line_count,
        word_count,
        char_count,
        width = column_widths.iter().max().cloned().unwrap_or(0)
    );

    Ok(())
}
