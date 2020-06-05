extern crate futures;
extern crate textplots;

pub mod chart;
pub mod openstreetmap_api;
pub mod smhi_api;
pub mod url_util;

// use chart::print_chart;
use openstreetmap_api::get_location;
use smhi_api::get_weather_for;
use std::process;

fn main() {
    let _points = [
        (1.0, 12.0),
        (2.0, 23.0),
        (3.0, 19.0),
        (4.0, 15.0),
        (5.0, 23.7),
        (6.0, 25.8),
        (7.0, 10.0),
    ];

    // match get_location("Abu Dhabi") {
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
            process::exit(1);
        }
    };

    match get_weather_for(location.lon, location.lat) {
        Some(we) => println!("{:?}", we),
        None => println!("No weatherforecast found for '{}'", location.display_name),
    };

    // println!("{}", url);

    // print_chart(&points);
    // let _data = get_weather();
}
