use crossterm::style::Stylize;

use crate::modules::{
    directories::convert_to_human_readable,
    systeminfo::{get_system_info, SystemInfo},
};

pub fn neofetch_command() {
    let info: SystemInfo = get_system_info();

    // println!("OS: {}", info.os);
    println!(
        "{}{}{}",
        info.hostname.clone().blue().bold(),
        "@".blue().bold(),
        info.username.clone().blue().bold()
    );
    display_stat("OS", info.os);
    display_stat("Hostname", info.hostname);
    display_stat("Username", info.username);
    display_stat("Shell", "kshell".to_owned());
    println!(
        "{}: {}/{}",
        "Memory".blue(),
        convert_to_human_readable(info.memory_usage),
        convert_to_human_readable(info.max_memory)
    );
    display_stat("CPU", format!("{}", info.cpu));
}

fn display_stat(key: &str, value: String) {
    println!("{}: {}", key.blue(), value);
}
