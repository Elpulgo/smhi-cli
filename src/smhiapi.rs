extern crate isahc;

use futures::executor::block_on;
use isahc::prelude::*;

static SMHI_BASE_URL: &str = "https://opendata-download-metobs.smhi.se/api/version/1.0/parameter/1/station-set/all/period/latest-hour/data.json";

pub fn get_weather() -> Result<(), isahc::Error> {
    block_on(async {
        let mut response = isahc::get_async(SMHI_BASE_URL).await?;
        let body = response.text_async().await?;
        println!("{}", body);
      
        Ok(())
    })
}

// #[derive(Serialize, Deserialize, Debug)]
// struct WeatherData {
//     updated: i64
// }

// impl RestPath<()> for WeatherData {
//     fn get_path(_: ()) -> Result<String, Error> {
//         Ok(String::from("/parameter/1/station-set/all/period/latest-hour/data.json"))
//         // Ok(String::from(
//         //     "data.json",
//         // ))
//     }
// }
