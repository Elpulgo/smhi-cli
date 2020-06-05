extern crate isahc;
extern crate serde;

use crate::url_util::build_encoded_url;

use futures::executor::block_on;
use isahc::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

static OPEN_STREET_MAP_BASE_URL: &str = "https://nominatim.openstreetmap.org/search?";

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
        match isahc::get_async(url).await {
            Ok(mut resp) => {
                match resp.text_async().await {
                    Ok(body) => match serde_json::from_str::<Vec<Location>>(&body) {
                        Ok(data) => match data.into_iter().nth(0) {
                            Some(loc) => return Some(loc),
                            None => return None,
                        },
                        Err(e) => {
                            eprintln!("{:?}", e);
                            return None;
                        }
                    },
                    Err(e) => {
                        eprintln!("{:?}", e);
                        return None;
                    }
                };
            }
            Err(e) => {
                eprintln!("{:?}", e);
                return None;
            }
        };
    })
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Location {
    pub lat: String,
    pub lon: String,
    pub display_name: String,
}
