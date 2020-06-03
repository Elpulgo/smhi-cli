extern crate url;

use std::collections::HashMap;
use url::Url;

pub fn build_encoded_url<'a>(
    base_url: &str,
    parameters: HashMap<&str, &str>,
) -> Result<String, String> {
    let mut url = base_url.to_string();

    if parameters.len() < 1 {
        return Ok(url);
    }
    if parameters.len() > 0 {
        url.push_str("?");
        let mut first = true;
        for (key, value) in parameters {
            match first {
                true => {
                    let p: &str = &format!("{}={}", key, value);
                    url.push_str(p);
                    first = false;
                }
                false => {
                    let p: &str = &format!("&{}={}", key, value);
                    url.push_str(p);
                }
            }
        }
    }

    let parsed = match Url::parse(&url) {
        Ok(value) => value,
        Err(e) => {
            return Err(format!(
                "{}, {}",
                "Failed to parse url".to_string(),
                e.to_string()
            ))
        }
    };

    return Ok(parsed.as_str().to_string());
}

// pub struct Parameter<'a> {
//     pub key: &'a str,
//     pub value: &'a str,
// }
