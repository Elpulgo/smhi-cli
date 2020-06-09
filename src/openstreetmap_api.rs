extern crate isahc;
extern crate serde;
use crate::rest_util;
use crate::url_util;

use serde::{Deserialize, Serialize};
use url_util::{build_encoded_url, Parameter, ParameterType};

static OPEN_STREET_MAP_BASE_URL: &str = "https://nominatim.openstreetmap.org/search";

#[derive(Serialize, Deserialize, Debug)]
pub struct Location {
    pub lat: String,
    pub lon: String,
    pub display_name: String,
}

pub fn get_location(location: &str) -> Option<Location> {
    let url = match build_encoded_url(OPEN_STREET_MAP_BASE_URL, get_params(location)) {
        Ok(url) => url,
        Err(e) => {
            eprintln!("Failed to build url for open street map API: {:?}", e);
            return None;
        }
    };

    match rest_util::get_async::<Vec<Location>>(url, String::from("Open Street Map API")) {
        Some(locations) => match locations.into_iter().nth(0) {
            Some(loc) => return Some(loc),
            None => return None,
        },
        None => return None,
    };
}

fn get_params(location: &str) -> Vec<Parameter> {
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
