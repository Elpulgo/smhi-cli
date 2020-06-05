extern crate url;

use url::Url;

pub fn build_encoded_url<'a>(base_url: &str, parameters: Vec<Parameter>) -> Result<String, String> {
    let mut url = base_url.to_string();

    match parameters.capacity() {
        0 => {
            let parsed = parse(&url).unwrap();
            return Ok(parsed);
        }
        _ => {
            let mut first = true;
            for param in parameters {
                url.push_str(&build_param_string(param, first));
                first = false;
            }
        }
    }
    return parse(&url);
}

fn build_param_string(param: Parameter, first: bool) -> String {
    let first_char = match first {
        true => "&",
        false => "",
    };

    match param.Type {
        ParameterType::QueryType => format!(
            "{First}{Key}={Value}",
            First = first_char,
            Key = param.Key,
            Value = param.Value
        ),
        ParameterType::PathTypeKeyAndValue => {
            format!("/{Key}/{Value}", Key = param.Key, Value = param.Value)
        }
        ParameterType::PathTypeOnlyValue => format!("/{Value}", Value = param.Value),
    }
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

pub fn slice_params(take: usize, value: String) -> String {
    value.chars().into_iter().take(take).collect()
}

pub struct Parameter {
    Key: String,
    Value: String,
    Type: ParameterType,
}

pub enum ParameterType {
    QueryType,
    PathTypeKeyAndValue,
    PathTypeOnlyValue,
}
