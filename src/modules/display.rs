use std::io::stdout;

use crossterm::{
    style::{PrintStyledContent, Stylize},
    ExecutableCommand,
};

pub fn draw_table(table: &[Vec<&str>]) {
    let mut max_lengths: Vec<usize> = vec![0; table[0].len()];

    for row in table {
        for (i, cell) in row.iter().enumerate() {
            if cell.len() > max_lengths[i] {
                max_lengths[i] = cell.len();
            }
        }
    }

    let mut output = Vec::new();

    for row in table {
        for (i, cell) in row.iter().enumerate() {
            let styled_cell = cell.bold();
            let padded_cell = format!("{:<width$}", styled_cell, width = max_lengths[i]);

            output.push(padded_cell);
            output.push("|".to_owned());
        }

        output.pop(); // Remove the trailing "|"
        output.push("\n".to_owned());
        output.push("-".repeat(max_lengths.iter().sum::<usize>() + row.len() - 1));
        output.push("\n".to_owned());
    }

    output.pop(); // Remove the trailing horizontal line

    let stdout = stdout();
    let mut handle = stdout.lock();
    for line in output {
        handle.execute(PrintStyledContent(line.green())).unwrap();
    }
}
