use chrono::{DateTime, Local};
use chrono::Datelike;

pub fn get_day() -> u32{
    // Get the current date in the local time zone
    let local: DateTime<Local> = Local::now();
    return local.day()
}