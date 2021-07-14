use chrono::{Utc, Datelike};
use std::{fs};

pub fn get_date() -> String{
    let now = Utc::now();
    let date : String = format!("{year}-{month}-{day}", 
                                year=now.year(), month=now.month(), day=now.day());

    return date;
}

pub fn get_folder_name() -> String {
    "stashit-".to_string()+get_date().as_str()
}

pub fn debug(job: &str) -> &str {
//! Stupid debug message to make the script look fun
    match job {
        "createdir" => return "job: creating new folder",
        "error" => return "cannot make folder, already exists",
        _ => "None"
    }
}

pub fn create_folder() {
    fs::create_dir(get_folder_name());
}
