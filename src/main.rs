mod utils;
use std::{path};
use utils::*;
use clap::{Arg, App};

fn path_exists(path_name: &str) -> bool {
    path::Path::new(&get_folder_name(path_name)).exists()
}
fn main() {
    let app = App::new("Stashit")
    .about("Mass file mover")
    .author("Skynse")
    .version("0.2.0")
    .arg(Arg::with_name("name") 
        .value_name("name")
        .takes_value(true)
        .required(false)
        .help("name of the folder to move files to")
        .short("n"))
    .get_matches();

    if app.is_present("name") {
        //Check if name argument used, if not, use the default stashit datetime folder
        let arg = String::from(app.value_of("name").unwrap());
        if !path_exists(arg.as_str()) {
            create_folder(arg.as_str());
            move_files(arg.as_str()).unwrap_or_default();
        }
        else {
            //If the folder with the argument name already exists, just move files to it
            move_files(arg.as_str()).unwrap_or_default();
        }
    }
    
    else if !path_exists(&get_folder_name("")) {
        //check if a stashit folder exists
            create_folder("");
            move_files(&get_folder_name("")).unwrap_or_default();
    }

    else {
        move_files(&get_folder_name("")).unwrap_or_default();
    }
} 