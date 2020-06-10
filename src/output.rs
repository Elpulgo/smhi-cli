
use crate::smhi_api;

use smhi_api::WeatherData;
use std::iter;

pub fn print_weather(weather_data: WeatherData){
    let mut line_length = 50;
    let mut first = true;

    println!("Time\t\t\t Temp (°C)\t Wind (ms/s)\t Rain (mm/h)\t Visibility (hm)\t ");

    for point in weather_data.points.into_iter() {
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