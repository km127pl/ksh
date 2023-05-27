use std::io::stdout;

use crossterm::{
    cursor::{self, MoveTo},
    event::{read, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    style::Stylize,
    terminal::{
        disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};

// TODO
/*
   enter key
   up/down arrows
   saving
   a command palette at the bottom
   backspace actually removing the char
   reading files with newlines
*/

pub fn edit_command(args: Vec<&str>) {
    if args.len() < 2 {
        println!("Not enough arguments provided for 'edit' (expected 1: [file name]).");
        return;
    }

    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen).unwrap();

    enable_raw_mode().expect("Failed to enable raw mode");

    let contents: Result<String, std::io::Error> = std::fs::read_to_string(args[1]);
    let mut buffer: Vec<String> = Vec::new();

    let mut cursor_x: u16 = 0;
    let mut cursor_y: u16 = 0;

    let width = match term_size::dimensions() {
        Some((w, _)) => w,
        None => 80, // default
    };

    let text = "text editor [".to_owned() + args[1] + "]";
    let text_width = text.chars().count();
    let padding = (width - text_width) / 2;
    let mut key_pressed = false; // Track if a key is being pressed

    clearscreen::clear().expect("failed to clear screen");
    execute!(stdout, cursor::MoveTo(0, 0)).unwrap();
    let white_bar = format!("{}{}{}\n", " ".repeat(padding), text, " ".repeat(padding));
    print!("{}", white_bar.on_white());

    execute!(stdout, cursor::MoveTo(cursor_x, cursor_y + 1)).unwrap();

    // init an empty buffer
    for _ in 0..10 {
        buffer.push(String::new());
    }

    match contents {
        Ok(file_contents) => {
            for c in file_contents.chars() {
                print!("{}", c);
            }
            buffer[0] = file_contents; // load the file contents into the first buffer
                                       //@todo: this will break on %nl files
        }
        Err(err) => println!("Unable to read file '{}': {}", args[1], err),
    }

    loop {
        execute!(stdout, cursor::MoveTo(cursor_x, cursor_y + 1)).unwrap();

        if let Ok(event) = read() {
            match event {
                Event::Key(KeyEvent {
                    code: KeyCode::Char('x'),
                    modifiers: KeyModifiers::CONTROL,
                    ..
                }) => {
                    disable_raw_mode().expect("Failed to disable raw mode");

                    execute!(stdout, LeaveAlternateScreen).unwrap();
                    break;
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char(c),
                    ..
                }) => {
                    key_pressed = !key_pressed;
                    if key_pressed {
                        buffer[cursor_y as usize].push(c);
                        print!("{}", c);
                        cursor_x = cursor_x + 1;
                    }
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Backspace,
                    ..
                }) => {
                    key_pressed = !key_pressed;
                    if key_pressed && cursor_x != 0 {
                        cursor_x = cursor_x - 1;
                        if !buffer[cursor_y as usize].is_empty() {
                            // Remove the char from the buffer
                            buffer[cursor_y as usize].pop();
                            // Remove the character under the cursor
                            execute!(stdout, Clear(ClearType::CurrentLine)).unwrap();
                            execute!(stdout, MoveTo(0, cursor_y + 1)).unwrap();
                            for c in buffer[cursor_y as usize].chars() {
                                print!("{}", c);
                            }
                        }
                    } else {
                        // go back a line
                        if cursor_y != 0 {
                            cursor_y = cursor_y - 1;
                            cursor_x = buffer[cursor_y as usize].len() as u16;
                        }
                    }
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Right,
                    ..
                }) => {
                    key_pressed = !key_pressed;
                    if key_pressed && cursor_x != 120 && cursor_x < buffer.len() as u16 {
                        cursor_x = cursor_x + 1;
                    }
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Left,
                    ..
                }) => {
                    key_pressed = !key_pressed;
                    if key_pressed && cursor_x != 0 {
                        cursor_x = cursor_x - 1;
                    }
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Enter,
                    ..
                }) => {
                    key_pressed = !key_pressed;
                    if key_pressed {
                        cursor_x = 0;
                        cursor_y = cursor_y + 1;
                    }
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Up, ..
                }) => {
                    key_pressed = !key_pressed;
                    if key_pressed && cursor_y != 0 {
                        cursor_y = cursor_y - 1;
                    }
                }
                _ => {}
            }
        }
    }
}
