mod utils;
use home;
use std::io;
use std::{env, fs, path, process};
use utils::*;

fn get_args() -> String {
    let flags: Vec<String> = env::args().collect();
    println!("{:?}", flags[1].to_owned().to_string());
    if flags.len() == 1 {
        String::from("stashit")
    }
    else {
    flags[1].to_owned().to_string()
}
}

fn move_files() -> Result<(), io::Error> {
    let current_dir = env::current_dir()?;
    if current_dir == home::home_dir().unwrap() {
        println!("Cannot stash in this directory");
        process::exit(0);
    } else {
        for entry in fs::read_dir(current_dir)? {
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

            if !f_name.starts_with("stashit") {
                println!("Moved: \x1b[0;32m{}\x1b[0m", f_name);
                fs::rename(f_name, get_folder_name(get_args()) + "/" + f_name);
            }
        }
    }
    Ok(())
}
fn main() {

    if get_args() != "stashit" {
        if !path::Path::new(&get_folder_name(get_args())).exists() {
                create_folder(get_args());
                move_files();
            }
        }

    else {
    if !path::Path::new(&get_folder_name(String::from(""))).exists() {
        create_folder(get_args());
        move_files();
    } else {
        move_files();
    }
}
}