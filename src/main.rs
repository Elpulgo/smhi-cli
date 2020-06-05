extern crate futures;
extern crate textplots;

pub mod chart;
pub mod openstreetmap_api;
pub mod smhiapi;
pub mod url_util;

// use chart::print_chart;
// use smhiapi::get_weather;
use openstreetmap_api::get_location;

// use std::collections::HashMap;
// use url_util::build_encoded_url;

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


    // let parameters: HashMap<&str, &str> = [("q", "Stora Nygatan 64 Malmö"), ("format", "json")]
    //     .iter()
    //     .cloned()
    //     .collect();

    // let url = match build_encoded_url("https://nominatim.openstreetmap.org/search", parameters) {
    //     Ok(url) => url,
    //     Err(e) => {
    //         eprintln!("{:?}", e);
    //         return;
    //     }
    // };

    match get_location("Stora Nygatan 64 Malmö") {
        Some(loc) => println!("{:?}", loc),
        None => println!("Nothing found!"),
    };
    match get_location("Abu Dhabi") {
        Some(loc) => println!("{:?}", loc),
        None => println!("Nothing found!"),
    };

    // println!("{}", url);

    // print_chart(&points);
    // let _data = get_weather();
}
