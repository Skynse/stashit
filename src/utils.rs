use chrono::{Datelike, Utc};
use std::io;
use std::{env, fs, path, process};
use std::cfg;
use crate::logs::*;

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
pub fn get_folder_name(name: &str) -> String {
    if !name.is_empty() {
        name.to_owned().to_string()
    } else {
        "stashit-".to_string() + get_date().as_str()
    }
}

pub fn create_folder(name: &str) -> Result<(), std::io::Error> {
    make_logs()?;
    write_logs(&get_folder_name(name))?;
    fs::create_dir(get_folder_name(name)).unwrap_or(());
    Ok(())
}

pub fn move_files(name: &str) -> Result<(), io::Error> {

    let log = read_logs();
    let mut counter: i32 = 0;
    let current_dir = env::current_dir()?;
    if current_dir == home::home_dir().unwrap() {
        println!("Cannot stash in this directory");
        process::exit(0);
    } else {
        for entry in fs::read_dir(current_dir)? {
            counter += 1;
            let entry = match entry {
                Ok(entry) => entry,
                Err(e) => return Err(e),
            };

            let path = entry.path();
            let f_name = path::Path::new(&path)
                .file_name()
                .unwrap()
                .to_str()
                .unwrap();
            
            if !log.contains(&f_name.to_string()) {
                #[cfg(target_os = "linux")] 
                {
                    println!("Moved ➟ \x1b[0;32m{}\x1b[0m", f_name);
                }
                
                #[cfg(target_os = "macos")] 
                {
                    println!("Moved ➟ \x1b[0;32m{}\x1b[0m", f_name);
                }

                #[cfg(target_os = "windows")]
                {
                    println!("Moved -> {}", f_name);
                }
                fs::rename(f_name, get_folder_name(name) + "/" + f_name).unwrap_or(());
            }
        }
        println!("\n{} item(s) moved!", counter-1);
    }
    Ok(())
}
