extern crate futures;
extern crate structopt;

pub mod openstreetmap_api;
pub mod rest_util;
pub mod smhi_api;
pub mod url_util;
pub mod weather_printer;

use openstreetmap_api::get_location;
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
    pub location: String,

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
}

fn main() {
    let args = Cli::from_args();
    execute(args);
}

fn execute(cli_args: Cli) {
    let location = match get_location(&cli_args.location) {
        Some(loc) => loc,
        None => {
            println!("The provided location was not found within the boundaries of Swedish territory. Try another term! Bye bye!");
            process::exit(0);
        }
    };

    let mut range_of_days = 1;
    if cli_args.range >= 1 {
        range_of_days = cli_args.range;
    }

    match get_weather_for(location.lat, location.lon, range_of_days) {
        Some(weather) => {
            print(weather);
        }
        None => println!("No weatherforecast found for '{}'", location.display_name),
    };
}
