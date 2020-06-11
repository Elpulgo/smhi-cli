extern crate chrono;
extern crate colored;
extern crate serde;
extern crate serde_json;

use crate::smhi_api;

use chrono::{DateTime, Utc};
use colored::*;
use serde::{Deserialize, Serialize};
use smhi_api::{WeatherData, WeatherPoint};
use std::iter;

const LINE_LENGTH: usize = 75;

#[derive(Debug, Serialize, Deserialize)]
struct WeatherDataGroup {
    points: Vec<WeatherPoint>,
    day: String,
    key: String,
}

pub fn print(weather_data: WeatherData) {
    let mut first = true;

    let groups = build_groups(weather_data);
    println!("");
    println!("{}", "Time\t Temp (°C)\t Wind (ms/s)\t Rain (mm)\t Visibility (km)\t ".blue().bold());
    print_divider("=");

    for group in groups {
        match first {
            true => first = false,
            false => print_divider("-"),
        }
        println!("{}", group.day.green().bold());
        print_group(group.points);
    }
    print_divider("=");
}

fn print_group(points: Vec<WeatherPoint>) {
    for point in points {
        println!(
            "{}",
            format!(
                "{Ref}\t {Temp}\t\t {Wind}\t\t {Rain}\t\t {Vis} \t\t {Desc}",
                Ref = point.time.format("%H:%M").to_string(),
                Temp = point.temperature.to_string().bold().bright_yellow(),
                Wind = point.wind,
                Rain = point.min_rain,
                Vis = point.visibility,
                Desc = point.weather_description
            )
        );
    }
}

fn print_divider(divider_symbol: &str) {
    println!(
        "{}",
        String::from(
            iter::repeat(divider_symbol)
                .take(LINE_LENGTH)
                .collect::<String>()
        ).bright_red().bold()
    );
}

fn build_groups(mut weather_data: WeatherData) -> Vec<WeatherDataGroup> {
    let mut groups = Vec::<WeatherDataGroup>::new();
    let mut datetime = weather_data.points.first_mut().unwrap().time;
    let mut comparable_key = datetime.format("%m-%d").to_string();

    let mut remaining_points = weather_data.points;
    loop {
        match group_points_per_day(&comparable_key, remaining_points.to_vec()) {
            Some((group, remaining)) => {
                groups.push(group);
                datetime = increment_date_by_days(datetime, 1);
                comparable_key = datetime.format("%m-%d").to_string();
                remaining_points = remaining;
            }
            None => break,
        }
    }

    return groups;
}

fn increment_date_by_days(datetime: DateTime<Utc>, increment: i64) -> DateTime<Utc> {
    return datetime
        .checked_add_signed(chrono::Duration::days(increment))
        .unwrap();
}

fn group_points_per_day(
    comparable_key: &String,
    points: Vec<WeatherPoint>,
) -> Option<(WeatherDataGroup, Vec<WeatherPoint>)> {
    let matched_points: Vec<WeatherPoint> = filter_points(points.to_vec(), comparable_key, true);
    let non_matched_points: Vec<WeatherPoint> =
        filter_points(points.to_vec(), comparable_key, false);

    match matched_points.len() {
        0 => return None,
        _ => {
            return Some((
                WeatherDataGroup {
                    points: matched_points.to_vec(),
                    key: comparable_key.to_owned(),
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

fn filter_points(
    points: Vec<WeatherPoint>,
    key: &String,
    should_match_predicate: bool,
) -> Vec<WeatherPoint> {
    let iterable_points = points.into_iter();

    match should_match_predicate {
        true => {
            return iterable_points
                .filter(|x| x.time.format("%m-%d").to_string() == key.to_owned())
                .collect()
        }
        false => {
            return iterable_points
                .filter(|x| x.time.format("%m-%d").to_string() != key.to_owned())
                .collect()
        }
    }
}

