use serde_json::{Result, Value};
use std::fs::{File};
use std::io::{prelude::*, BufReader};

struct LogItem {
    name: String,
}

fn local_path_exists(name: &str) -> bool{
    std::path::Path::new(name).exists()
}

static LOG_TEMPLATE: &str = "stashit_log.json";

pub fn make_logs() -> Result<()> {
    if !local_path_exists(LOG_TEMPLATE) {
        File::create(LOG_TEMPLATE);
    }

    Ok(())
}

pub fn write_logs(name: &str) -> Result<()> {
    let item = LogItem {name: name.to_string()};
    let out = serde_json::to_string(&item.name).unwrap();
    serde_json::to_writer_pretty(File::open(LOG_TEMPLATE).unwrap(), &out);
    std::fs::write(LOG_TEMPLATE, &out).expect("Could not open file");

    Ok(())
}
pub fn read_logs() -> Vec<String> {
    let mut buf: Vec<String> = Vec::new();
    let stream = File::open(LOG_TEMPLATE);
    for i in BufReader::new(stream.unwrap()).lines() {
       // println!("{:?}", i);
        buf.push(i.unwrap());
    }
    buf
}
