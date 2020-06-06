extern crate isahc;
extern crate serde;

use crate::url_util::{build_encoded_url, Parameter, ParameterType};

use futures::executor::block_on;
use isahc::prelude::*;
use serde::{Deserialize, Serialize};

static OPEN_STREET_MAP_BASE_URL: &str = "https://nominatim.openstreetmap.org/search";

pub fn get_location(location: &str) -> Option<Location> {

    let url = match build_encoded_url(OPEN_STREET_MAP_BASE_URL, get_params(location)) {
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

fn get_params(location: &str) -> Vec<Parameter>{
    return vec![
        Parameter {
            key: "q".to_string(),
            value: location.to_string(),
            param_type: ParameterType::QueryType,
        },
        Parameter {
            key: "format".to_string(),
            value: "json".to_string(),
            param_type: ParameterType::QueryType,
        },
        Parameter {
            key: "countrycodes".to_string(),
            value: "se".to_string(),
            param_type: ParameterType::QueryType,
        },
    ];
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Location {
    pub lat: String,
    pub lon: String,
    pub display_name: String,
}
