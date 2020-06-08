extern crate chrono;
extern crate isahc;
extern crate serde;
use crate::url_util;

use chrono::{DateTime, Utc};
use futures::executor::block_on;
use isahc::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use url_util::{build_encoded_url, slice_params, Parameter, ParameterType};

static SMHI_BASE_URL: &str =
    "https://opendata-download-metfcst.smhi.se/api/category/pmp3g/version/2/geotype/point";

#[derive(Serialize, Deserialize, Debug)]
pub struct WeatherData {
    #[serde(alias = "referenceTime")]
    reference_time: DateTime<Utc>,
    #[serde(alias = "timeSeries")]
    pub points: Vec<WeatherPoint>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WeatherPoint {
    #[serde(alias = "validTime")]
    pub time: DateTime<Utc>,
    parameters: Vec<WeatherParameter>,
    #[serde(skip)]
    pub temperature: f64,
    #[serde(skip)]
    pub wind: f64,
    #[serde(skip)]
    pub min_rain: f64,
    #[serde(skip)]
    pub humidity: f64,
    #[serde(skip)]
    pub visibility: f64,
    #[serde(skip)]
    pub weather_description: String,
}

impl WeatherPoint {
    fn build_values(&mut self) {
        for param in self.parameters.iter_mut() {
            let value = match param.values.first() {
                Some(s) => *s,
                None => continue,
            };

            match &param.name.as_ref() {
                &"t" => self.temperature = value,
                &"ws" => self.wind = value,
                &"pmin" => self.min_rain = value,
                &"r" => self.humidity = value,
                &"vis" => self.visibility = value,
                &"Wsymb2" => self.weather_description = get_smhi_symbol(value.round() as i64),
                _ => {}
            }
        }

        self.parameters.clear();
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct WeatherParameter {
    name: String,
    unit: String,
    values: Vec<f64>,
}

pub fn get_weather_for(lat: String, lon: String) -> Option<WeatherData> {
    let url = match build_encoded_url(SMHI_BASE_URL, get_params(lat, lon)) {
        Ok(url) => url,
        Err(e) => {
            eprintln!("{:?}", e);
            return None;
        }
    };

    block_on(async {
        // TODO: Match pattern so we know what type of error..
        let mut response = isahc::get_async(url).await.unwrap();
        let body = response.text_async().await.unwrap();
        let mut data: WeatherData = serde_json::from_str(&body).unwrap();
        post_process_response(&mut data);
        return Some(data);
    })
}

fn post_process_response(data: &mut WeatherData) {
    for point in data.points.iter_mut() {
        point.build_values();
    }
}

fn get_params(lat: String, lon: String) -> Vec<Parameter> {
    return vec![
        Parameter {
            key: "lon".to_string(),
            value: slice_params(6, lon),
            param_type: ParameterType::PathTypeKeyAndValue,
        },
        Parameter {
            key: "lat".to_string(),
            value: slice_params(6, lat),
            param_type: ParameterType::PathTypeKeyAndValue,
        },
        Parameter {
            key: "format".to_string(),
            value: "data.json".to_string(),
            param_type: ParameterType::PathEndingType,
        },
    ];
}

fn get_smhi_symbol(symbol: i64) -> String {
    let smhi_symbols: HashMap<i64, &'static str> = vec![
        (1, "☀️"),
        (2, "🌤️"),
        (3, "⛅"),
        (4, "🌥️"),
        (5, "☁️"),
        (6, "☁️"),
        (7, "🌫"),
        (8, "🌦️"),
        (9, "🌦️"),
        (10, "🌦️"),
        (11, "⛈️"),
        (12, "🌨️"),
        (13, "🌨️"),
        (14, "🌨️"),
        (15, "❄️"),
        (16, "❄️"),
        (17, "❄️"),
        (18, "🌧️"),
        (19, "🌧️"),
        (20, "🌧️"),
        (21, "🌩️"),
        (22, "🌨️"),
        (23, "🌨️"),
        (24, "🌨️"),
        (25, "❄️"),
        (26, "❄️"),
        (27, "❄️"),
    ]
    .into_iter()
    .collect();

    match smhi_symbols.get(&symbol) {
        Some(symbol) => return symbol.to_string(),
        None => return "".to_string(),
    };
}

// fn weather_param_name_is(weather_param: &String) -> bool {
//     return weather_param != "t";
// }
