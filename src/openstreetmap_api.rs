extern crate isahc;
extern crate serde;

use crate::url_util::build_encoded_url;

use futures::executor::block_on;
use isahc::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

static OPEN_STREET_MAP_BASE_URL: &str = "https://nominatim.openstreetmap.org/search";

pub fn get_location(location: &str) -> Option<Location> {
    let parameters: HashMap<&str, &str> =
        [("q", location), ("countrycodes", "se"), ("format", "json")]
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

    block_on(async {
        let mut response = isahc::get_async(url).await.unwrap();
        let body = response.text_async().await.unwrap();
        let data: Vec<Location> = serde_json::from_str(&body).unwrap();

        match data.into_iter().nth(0) {
            Some(loc) => return Some(loc),
            None => return None,
        };
    })
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Location {
    lat: String,
    lon: String,
    display_name: String,
}
