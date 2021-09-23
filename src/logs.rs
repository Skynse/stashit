use std::fs::{self, File};
use std::io::{prelude::*, BufReader};

fn local_path_exists(name: &str) -> bool{
    std::path::Path::new(name).exists()
}

pub static LOG_TEMPLATE: &str = "stashit_log.txt";

pub fn make_logs() -> Result<(), std::io::Error> {
    let write_str = LOG_TEMPLATE.to_string()+"\n";
    if !local_path_exists(LOG_TEMPLATE) {
        File::create(LOG_TEMPLATE)?;
        std::fs::write(LOG_TEMPLATE, write_str)?;
    }


    Ok(())
}

pub fn write_logs(name: &str) -> Result<(), std::io::Error> {
    let content = name.to_string() + "\n";
    let mut file = fs::OpenOptions::new()
        .append(true)
        .open(LOG_TEMPLATE)
        .unwrap();

    file.write_all(content.as_bytes())?;
    Ok(())
}   
pub fn read_logs() -> Vec<String> {
    //let mut buf: Vec<String> = Vec::new();
    let stream = File::open(LOG_TEMPLATE).expect("Could not open the log file");
    let buf = BufReader::new(stream);
    buf.lines()
    .map(|l| l.expect("Could not parse the line"))
    .collect()
}
