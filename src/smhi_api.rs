extern crate isahc;
extern crate serde;

use crate::url_util::{build_encoded_url, slice_params, Parameter, ParameterType};

use futures::executor::block_on;
use isahc::prelude::*;
use serde::{Deserialize, Serialize};

static SMHI_BASE_URL: &str = "https://opendata-download-metfcst.smhi.se/api/category/pmp3g/version/2/geotype/point";

pub fn get_weather_for(lat: String, lon: String) -> Option<WeatherData> {

    let url = match build_encoded_url(SMHI_BASE_URL, get_params(lat, lon)) {
        Ok(url) => url,
        Err(e) => {
            eprintln!("{:?}", e);
            return None;
        }
    };

    println!("{:?}", url);

    block_on(async {
        let mut response = isahc::get_async(url).await.unwrap();
        let body = response.text_async().await.unwrap();
        let data: WeatherData = serde_json::from_str(&body).unwrap();
        println!("{:?}", data);
        return Some(data);
    })
}

fn get_params(lat: String, lon: String) -> Vec<Parameter>{
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

#[derive(Serialize, Deserialize, Debug)]
pub struct WeatherData {
    updated: i64,
}
