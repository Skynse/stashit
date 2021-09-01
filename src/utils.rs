use chrono::{Datelike, Utc};
use std::fs;

pub fn get_date() -> String {
    let now = Utc::now();
    let date: String = format!(
        "{year}-{month}-{day}",
        year = now.year(),
        month = now.month(),
        day = now.day()
    );
    date
}
pub fn get_folder_name(name: String) -> String {
    if !name.is_empty() {
        name
    }
    else {
    "stashit-".to_string() + get_date().as_str()
}
}

pub fn create_folder(name: String) {
    fs::create_dir(get_folder_name(name));
}
