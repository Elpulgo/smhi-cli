extern crate serde;
extern crate serde_json;

use serde::{Deserialize, Serialize};
use std::env::current_exe;
use std::fs::File;
use std::io::{BufReader, ErrorKind};
use std::process;

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

    let file = match File::create(build_file_path()) {
        Ok(file) => file,
        Err(e) => match e.kind() {
            ErrorKind::PermissionDenied => {
                println!("Insufficient permissions to save default location. Try with sudo.");
                process::exit(0);
            }
            ErrorKind::NotFound => {
                println!("Directory to save default location not found.");
                process::exit(0);
            }
            _ => {
                eprintln!("Failed to create default location file '{}'", e);
                process::exit(0);
            }
        },
    };

    if serde_json::to_writer(&file, &default_location).is_err() {
        eprintln!("Failed to persist default location!");
        process::exit(0);
    }
}

pub fn read_default_location() -> Option<DefaultLocation> {
    let file = match File::open(build_file_path()) {
        Ok(file) => file,
        Err(e) => match e.kind() {
            ErrorKind::PermissionDenied => {
                println!("Insufficient permissions to read default location file. Try with sudo.");
                process::exit(0);
            }
            ErrorKind::NotFound => {
                return None;
            }
            _ => {
                eprintln!("Failed to open default location file '{}'", e);
                process::exit(0);
            }
        },
    };

    let reader = BufReader::new(file);

    match serde_json::from_reader(reader) {
        Ok(location) => return location,
        Err(e) => {
            eprintln!("Failed to parse filecontent for default location '{}'", e);
            process::exit(0);
        }
    };
}

fn build_file_path() -> String {
    let mut current_dir = current_exe().unwrap();

    // Remove the actual binary to get the directory
    current_dir.pop();
    let current_path = current_dir.to_str().unwrap();
    return current_path.to_owned() + DEFAULT_FILE_NAME;
}
