extern crate isahc;
extern crate serde;

use crate::url_util::build_encoded_url;

use futures::executor::block_on;
use isahc::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;


static OPEN_STREET_MAP_BASE_URL: &str = "https://nominatim.openstreetmap.org/search";

pub fn get_location(location: &str) -> Option<Vec<Location>> {
    let parameters: HashMap<&str, &str> =
        [("q", location), ("countrycode", "se"), ("format", "json")]
            .iter()
            .cloned()
            .collect();

    let url = match build_encoded_url(OPEN_STREET_MAP_BASE_URL, parameters) {
        Ok(url) => url,
        Err(e) => {
            eprintln!("{:?}", e);
            return None;
        }
    };

    println!("{}", url);

    block_on(async {

        // let response = isahc::get(url).unwrap();
        // let body = response.body.text();// into_body().;
        // let mut buf: String;
        // let data: Location = serde_json::from_str((buf)).unwrap();
        // // println!("{:?}", data);
        // return Some(Cow::from(&data));
        let mut response = isahc::get_async(url).await.unwrap();
        let body = response.text_async().await.unwrap();
        
        // println!("{}", body);
        let data: Vec<Location> = serde_json::from_str(&body).unwrap();
        // println!("{:?}", data);
      
        return Some(data);
    })
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Location {
    lat: String,
    lon: String,
    display_name: String,
}
