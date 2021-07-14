mod utils;
use utils::*;
use std::io;
use std::{fs, env, path, process};

fn move_files() -> Result<(), io::Error> {
    let current_dir = env::current_dir()?;
    if current_dir == "/home/" {
        println!("Cannot stash in this directory");
        process::exit(0);
    }
    for entry in fs::read_dir(current_dir)? {
        let entry = entry?;
        let path = entry.path();
        let f_name =  path::Path::new(&path).file_name().unwrap().to_str().unwrap();

        let metadata = fs::metadata(&path)?;

        if  !f_name.starts_with("stashit") {
            println!("Moved: {}", f_name);
            //fs::rename(f_name, get_folder_name()+"/"+f_name);

        }
    }
    Ok(())

}
fn main()   {
    if !path::Path::new(&get_date()).exists() {
        create_folder();
        move_files();
    }
    else {
        move_files();
    }
    
}

