extern crate isahc;
extern crate serde;

use futures::executor::block_on;
use isahc::prelude::*;
use serde::Deserialize;

pub fn get_async<'a, T>(url: String, api_type: String) -> Option<T>
where
    for<'de> T: Deserialize<'de> + 'a,
{
    return block_on(async {
        let mut resp = match isahc::get_async(url).await {
            Ok(resp) => resp,
            Err(e) => {
                eprintln!(
                    "Failed to get response from {Api}: {Error}",
                    Api = api_type,
                    Error = e
                );
                return None;
            }
        };

        let body = match resp.text_async().await {
            Ok(body) => body,
            Err(e) => {
                eprintln!(
                    "Failed to read response from {Api}: {Error}",
                    Api = api_type,
                    Error = e
                );
                return None;
            }
        };

        match serde_json::from_str::<T>(&body) {
            Ok(data) => return Some(data),
            Err(e) => {
                eprintln!(
                    "Failed to parse data from {Api} response: {Error}",
                    Api = api_type,
                    Error = e
                );
                return None;
            }
        };
    });
}
