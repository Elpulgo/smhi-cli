extern crate futures;
extern crate textplots;

pub mod openstreetmap_api;
pub mod rest_util;
pub mod smhi_api;
pub mod url_util;
pub mod weather_printer;

use openstreetmap_api::get_location;
use smhi_api::get_weather_for;
use std::process;
use weather_printer::print;

fn main() {
    // let location = match get_location("Abu Dhabi") {
    //     Some(loc) => println!("{:?}", loc),
    //     None => {
    //         println!("Provided location not found! Bye bye!");
    //         process::exit(1);
    //     }
    // };
    let location = match get_location("Stora Nygatan 64 Malmö") {
        Some(loc) => loc,
        None => {
            println!("The provided location was not found within the boundaries of Swedish territory. Try another term! Bye bye!");
            process::exit(0);
        }
    };


    // NOTE! Can't be 0, set to 1 then
    let range_of_days = 10;

    match get_weather_for(location.lat, location.lon, range_of_days) {
        Some(weather) => {
            print(weather);
        },        
        None => println!("No weatherforecast found for '{}'", location.display_name),
    };
}
