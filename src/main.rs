extern crate colored;
extern crate futures;
extern crate structopt;

pub mod default_location;
pub mod openstreetmap_api;
pub mod rest_util;
pub mod smhi_api;
pub mod url_util;
pub mod weather_printer;

use colored::*;
use default_location::{persist_default_location, read_default_location};
use openstreetmap_api::{get_location, Location};
use smhi_api::get_weather_for;
use std::process;
use structopt::StructOpt;
use weather_printer::print;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "SMHI-CLI",
    about = "A CLI to show the weather forecast for the coming days.

Example usage: 
'Malmö', 'Regementsgatan 37', 'Stora Nyvägen 37, Stockholm'.

Will only work with locations within Swedish territory since SMHI is used as API. 
Make use of SMHI API under CC 4.0 SE."
)]
struct Cli {
    #[structopt(short = "l", long = "location", help = "Location to get forecast for.")]
    pub location: Option<String>,

    #[structopt(
        short = "r",
        long = "range",
        help = "A value between 1-10 days.",
        default_value = "1"
    )]
    pub range: i64,

    #[structopt(
        short = "d",
        long = "default",
        help = "Use this flag to set location as default. \nDefault location will be used if no location argument is passed."
    )]
    pub set_as_default_location: bool,
    #[structopt(
        short = "s",
        long = "show-default",
        help = "Use this flag to show the default location, if any."
    )]
    pub show_default_location: bool,
}

fn main() {
    let args = Cli::from_args();
    let (lat, lon, range, location) = handle_cli_args(args);
    execute(lat, lon, range, location);
}

fn handle_cli_args(cli_args: Cli) -> (String, String, i64, String) {
    if cli_args.show_default_location {
        match read_default_location(){
            Some(location) => {
                println!("Default location is set to '{}'.", location.display_name.yellow().bold());
            },
            None => {
                println!("No default location is set. {} to set a default location.", "Use '-d / --default'".yellow().bold());
            }
        }
        process::exit(0);
    }

    let location = match cli_args.location {
        Some(location_arg) => match get_location(&location_arg) {
            Some(loc) => loc,
            None => {
                println!("The provided location was not found. Try another location!");
                process::exit(0);
            }
        },
        None => match read_default_location() {
            Some(default_location) => Location {
                lat: default_location.lat,
                lon: default_location.lon,
                display_name: default_location.display_name,
            },
            None => {
                println!(
                    "No location argument was found, use {} or set a default location with {}.",
                    "'-l / --location'".yellow().bold(),
                    "'-d / --default'".yellow().bold()
                );
                process::exit(0);
            }
        },
    };

    let mut range_of_days = 1;
    if cli_args.range >= 1 {
        range_of_days = cli_args.range;
    }
    match cli_args.set_as_default_location {
        true => persist_default_location(&location.display_name, &location.lat, &location.lon),
        false => {}
    };

    return (
        location.lat,
        location.lon,
        range_of_days,
        location.display_name,
    );
}

fn execute(lat: String, lon: String, range: i64, location_name: String) {
    match get_weather_for(lat, lon, range) {
        Some(weather) => {
            println!(
                "Weather forecast the upcoming {} days for '{}'",
                range, location_name
            );
            print(weather);
        }
        None => println!("No weatherforecast found for '{}'. Try a location within Swedish terriory.", location_name),
    };
}
