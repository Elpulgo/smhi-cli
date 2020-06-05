extern crate isahc;
extern crate serde;

use crate::url_util::{build_encoded_url, slice_params, Parameter, ParameterType};

use futures::executor::block_on;
use isahc::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::collections::HashMap;

static SMHI_BASE_URL: &str =
    "https://opendata-download-metfcst.smhi.se/api/category/pmp3g/version/2/geotype/point";

pub fn get_weather(lat: String, lon: String) -> Option<WeatherData> {
    let params = vec![
        Parameter {
            Key: "lon".to_string(),
            Value: slice_params(6, lon),
            Type: ParameterType::PathTypeKeyAndValue,
        },
        Parameter {
            Key: "lat".to_string(),
            Value: slice_params(6, lat),
            Type: ParameterType::PathTypeKeyAndValue,
        },
        Parameter {
            Key: "format".to_string(),
            Value: "data.json".to_string(),
            Type: ParameterType::PathTypeOnlyValue,
        },
    ];

    let url = match build_encoded_url(SMHI_BASE_URL, params) {
        Ok(url) => url,
        Err(e) => {
            eprintln!("{:?}", e);
            return None;
        }
    };
    block_on(async {
        let mut response = isahc::get_async(url).await.unwrap();
        let body = response.text_async().await.unwrap();
        let data: WeatherData = serde_json::from_str(&body).unwrap();
        println!("{:?}", data);
        Some(WeatherData)
    })
}

#[derive(Serialize, Deserialize, Debug)]
struct WeatherData {
    updated: i64,
}
