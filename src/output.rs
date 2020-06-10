extern crate chrono;
extern crate serde;
extern crate serde_json;

use crate::smhi_api;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use smhi_api::{WeatherData, WeatherPoint};
use std::iter;

pub fn print_weather(weather_data: WeatherData) {
    let mut line_length = 50;
    let mut first = true;

    let _p = group_per_day(weather_data);

    // println!("Time\t\t\t Temp (°C)\t Wind (ms/s)\t Rain (mm/h)\t Visibility (hm)\t ");

    // for point in weather_data.points.into_iter() {
    //     let line = format!(
    //         "{Ref}\t {Temp}\t\t {Wind}\t\t {Rain}\t\t {Vis} \t\t {Desc}",
    //         // Day = point.time.format("%a %H").to_string(),
    //         Ref = point.time.format("%Y-%m-%d %H:%M").to_string(),
    //         Temp = point.temperature,
    //         Wind = point.wind,
    //         Rain = point.min_rain,
    //         Vis = point.visibility,
    //         Desc = point.weather_description
    //     );
    //     line_length = line.chars().count() * 2 + 3;
    //     let divider = iter::repeat("-").take(line_length).collect::<String>();
    //     match first {
    //         true => {
    //             println!("{}", String::from(divider));
    //             println!("{}", line);
    //             first = false;
    //         }
    //         false => {
    //             println!("{}", line);
    //         }
    //     }
    // }
    println!(
        "{}",
        String::from(iter::repeat("-").take(line_length).collect::<String>())
    );
}

fn group_per_day(mut weather_data: WeatherData) -> Vec<WeatherDataGroup> {
    let mut groups = Vec::<WeatherDataGroup>::new();

    let mut weekday = weather_data.points.first_mut().unwrap().time;
    let mut key_comparer = weather_data
        .points
        .first_mut()
        .unwrap()
        .time
        .format("%m-%d")
        .to_string();

    println!("{:?}", weekday);

    let mut found = true;
    let mut remaining_points = weather_data.points;
    while found == true {
        match find_for_weekday(&key_comparer, remaining_points.to_vec()) {
            Some((group, remaining)) => {
                groups.push(group);
                println!("key comparer AGAIN : {:?}", weekday);
                weekday = weekday
                    .checked_add_signed(chrono::Duration::days(1))
                    .unwrap();
                key_comparer = weekday.format("%m-%d").to_string();
                remaining_points = remaining;
            }
            None => found = false,
        }
    }

    let seralized = serde_json::to_string(&groups).unwrap();
    println!("{}", seralized);
    return groups;
}

fn find_for_weekday(
    key_comparer: &String,
    points: Vec<WeatherPoint>,
) -> Option<(WeatherDataGroup, Vec<WeatherPoint>)> {
    let matched_points: Vec<WeatherPoint> = points
        .to_vec()
        .into_iter()
        .filter(|x| x.time.format("%m-%d").to_string() == key_comparer.to_owned())
        .collect();

    let non_matched_points: Vec<WeatherPoint> = points
        .to_vec()
        .into_iter()
        .filter(|x| x.time.format("%m-%d").to_string() != key_comparer.to_owned())
        .collect();

    println!("matched: {:?}", matched_points.len());
    println!("NON matched: {:?}", non_matched_points.len());

    match matched_points.len() {
        0 => return None,
        _ => {
            return Some((
                WeatherDataGroup {
                    points: matched_points.to_vec(),
                    key: key_comparer.to_owned(),
                    day: String::from(
                        matched_points
                            .first()
                            .unwrap()
                            .time
                            .format("%a")
                            .to_string(),
                    ),
                },
                non_matched_points,
            ))
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct WeatherDataGroup {
    points: Vec<WeatherPoint>,
    day: String,
    key: String,
}
