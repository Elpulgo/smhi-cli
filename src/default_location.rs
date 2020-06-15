extern crate serde;
extern crate serde_json;

use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::process;
use std::env::current_exe;

static DEFAULT_FILE_NAME: &str = "/default_location.dat";

#[derive(Debug, Serialize, Deserialize)]
pub struct DefaultLocation {
    pub lat: String,
    pub lon: String,
    pub display_name: String,
}

pub fn persist_default_location(location: &String, lat: &String, lon: &String) {
    let default_location = DefaultLocation {
        lat: lat.to_string(),
        lon: lon.to_string(),
        display_name: location.to_string(),
    };

    let current_dir = current_exe().unwrap();
    let current_path = current_dir.to_str().unwrap();
    let location_file_path = current_path.to_owned() + DEFAULT_FILE_NAME;

    if serde_json::to_writer(&File::create(location_file_path).unwrap(), &default_location).is_err()
    {
        eprintln!("Failed to persist default location!");
        process::exit(0);
    }
}

pub fn read_default_location() -> Option<DefaultLocation>{
    let file = match File::open(DEFAULT_FILE_NAME){
        Ok(file) => file,
        Err(_e) => return None
    };

    let reader = BufReader::new(file);

    match serde_json::from_reader(reader){
        Ok(location) => return location,
        Err(_e) => return None
    };
}
