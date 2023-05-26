use chrono::Local;

pub fn clock_command() {
    let current_datetime: chrono::DateTime<Local> = Local::now();
    let formatted_datetime: String = current_datetime.format("%Y-%m-%d %H:%M:%S").to_string();
    
    println!("{}", formatted_datetime);
}
