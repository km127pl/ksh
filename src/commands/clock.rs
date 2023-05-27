use chrono::Local;
use crate::modules::command::CommandResult;

pub fn clock_command() -> CommandResult {
    let current_datetime: chrono::DateTime<Local> = Local::now();
    let formatted_datetime: String = current_datetime.format("%Y-%m-%d %H:%M:%S").to_string();

    println!("{}", formatted_datetime);

    CommandResult::Success
}
