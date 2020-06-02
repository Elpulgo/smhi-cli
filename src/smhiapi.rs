extern crate isahc;

use isahc::prelude::*;

static SMHI_BASE_URL: &str = "https://opendata-download-metobs.smhi.se/api/version/1.0/parameter/1/station-set/all/period/latest-hour/data.json";

pub async fn get_weather() {
    // futures::executor::block_on(async {
    let mut response = isahc::get_async(SMHI_BASE_URL).await.unwrap();
    let body = response.text_async().await.unwrap();
    println!("{}", body);

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
