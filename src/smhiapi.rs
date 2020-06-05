extern crate isahc;
extern crate serde;

use futures::executor::block_on;
use isahc::prelude::*;
use serde::{Serialize, Deserialize};
use serde_json::Result;

static SMHI_BASE_URL: &str = "https://opendata-download-metobs.smhi.se/api/version/1.0/parameter/1/station-set/all/period/latest-hour/data.json";

pub fn get_weather() -> Result<()> {
    block_on(async {
        let mut response = isahc::get_async(SMHI_BASE_URL).await.unwrap();
        let body = response.text_async().await.unwrap();
        let data: WeatherData = serde_json::from_str(&body).unwrap();
        println!("{:?}", data);
      
        Ok(())
    })
}

#[derive(Serialize, Deserialize, Debug)]
struct WeatherData {
    updated: i64
}