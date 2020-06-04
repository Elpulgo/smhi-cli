extern crate url;

use std::collections::HashMap;
use url::Url;

pub fn build_encoded_url<'a>(
    base_url: &str,
    parameters: HashMap<&str, &str>,
) -> Result<String, String> {
    let mut url = base_url.to_string();

    match parameters.capacity() {
        0 => {
            let parsed = parse(&url).unwrap();
            return Ok(parsed);
        }
        _ => {
            url.push_str("?");
            let mut first = true;
            for (key, value) in parameters {
                match first {
                    true => {
                        url.push_str(&format!("{}={}", key, value));
                        first = false;
                    }
                    false => {
                        url.push_str(&format!("&{}={}", key, value));
                    }
                }
            }
        }
    }
    return parse(&url);
}

fn parse(url: &str) -> Result<String, String> {
    match Url::parse(&url) {
        Ok(value) => return Ok(value.as_str().to_string()),
        Err(e) => {
            return Err(format!(
                "{}, {}",
                "Failed to parse url".to_string(),
                e.to_string()
            ))
        }
    };
}
