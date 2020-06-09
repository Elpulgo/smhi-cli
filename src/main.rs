extern crate futures;
extern crate textplots;

pub mod chart;
pub mod openstreetmap_api;
pub mod smhi_api;
pub mod url_util;
pub mod rest_util;

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

    match get_weather_for(location.lat, location.lon) {
        Some(we) => {
            for point in we.points.into_iter(){
                println!("Ref time: {Ref}\t temp: {Temp}\t wind: {Wind}\t rain: {Rain}\t Visibility: {Vis} \t Desc: {Desc}", 
                    Ref = point.time, 
                    Temp = point.temperature,
                    Wind = point.wind,
                    Rain = point.min_rain,
                    Vis = point.visibility,
                    Desc = point.weather_description
                );
            }
        },
        None => println!("No weatherforecast found for '{}'", location.display_name),
    };

    // println!("{}", url);

    // print_chart(&points);
    // let _data = get_weather();
}
