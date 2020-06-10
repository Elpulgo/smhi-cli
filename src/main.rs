extern crate futures;
extern crate textplots;

pub mod chart;
pub mod openstreetmap_api;
pub mod rest_util;
pub mod smhi_api;
pub mod url_util;

// use chart::print_chart;
use openstreetmap_api::get_location;
use smhi_api::get_weather_for;
use std::iter;
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
            let mut line_length = 50;
            let mut first = true;

            println!("Time\t\t\t Temp (°C)\t Wind (ms/s)\t Rain (mm/h)\t Visibility (hm)\t ");

            for point in we.points.into_iter() {
                let line = format!(
                    "{Ref}\t {Temp}\t\t {Wind}\t\t {Rain}\t\t {Vis} \t\t {Desc}",
                    Ref = point.time.format("%Y-%m-%d %H:%M").to_string(),
                    Temp = point.temperature,
                    Wind = point.wind,
                    Rain = point.min_rain,
                    Vis = point.visibility,
                    Desc = point.weather_description
                );
                line_length = line.chars().count() * 2 + 3;
                let divider = iter::repeat("-").take(line_length).collect::<String>();
                match first {
                    true => {
                        println!("{}", String::from(divider));
                        println!("{}", line);
                        first = false;
                    }
                    false => {
                        println!("{}", line);
                    }
                }
            }
            println!(
                "{}",
                String::from(iter::repeat("-").take(line_length).collect::<String>())
            );
        }
        None => println!("No weatherforecast found for '{}'", location.display_name),
    };

    // println!("{}", url);

    // print_chart(&points);
    // let _data = get_weather();
}
