extern crate url;

use core::cmp::Ordering;
use url::Url;

pub fn build_encoded_url<'a>(base_url: &str, mut parameters: Vec<Parameter>) -> Result<String, String> {
    let mut url = base_url.to_string();

    match parameters.capacity() {
        0 => {
            let parsed = parse(&url).unwrap();
            return Ok(parsed);
        }
        _ => {
            let mut first_query_param_executed = false;
            parameters.sort_by_key(|a| a.param_type); //(|a, b| a.param_type.cmp(&b.param_type));
            // parameters.sort_by(|a, b| a.param_type.cmp(&b.param_type));
            for param in parameters {
                match param.param_type {
                    ParameterType::QueryType if first_query_param_executed == false => {
                        url.push_str(&build_param_string(param, true));
                        first_query_param_executed = false;
                    }
                    _ => url.push_str(&build_param_string(param, false)),
                }
            }
        }
    }
    return parse(&url);
}

fn build_param_string(param: Parameter, first_query_param: bool) -> String {
    match param.param_type {
        ParameterType::PathTypeOnlyValue => format!("/{Value}", Value = param.value),
        ParameterType::PathTypeKeyAndValue => {
            format!("/{Key}/{Value}", Key = param.key, Value = param.value)
        }
        ParameterType::QueryType => {
            let first_char = match first_query_param {
                true => "?",
                false => "&",
            };
            format!(
                "{First}{Key}={Value}",
                First = first_char,
                Key = param.key,
                Value = param.value
            )
        }
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
    let p = value[0..take].to_string();
    let x: String = value.chars().into_iter().take(take).collect();
    println!("Into iter collection: {:?}", x);
    println!("Value slice: {:?}", p);

    return p;
}

pub struct Parameter {
    pub key: String,
    pub value: String,
    pub param_type: ParameterType,
}

#[derive(Eq, Copy, Clone)]
pub enum ParameterType {
    PathTypeOnlyValue,
    PathTypeKeyAndValue,
    QueryType,
}

impl Ord for ParameterType {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            ParameterType::PathTypeOnlyValue => match other {
                ParameterType::QueryType => Ordering::Greater,
                ParameterType::PathTypeKeyAndValue => Ordering::Greater,
                ParameterType::PathTypeOnlyValue => Ordering::Equal,
            },
            ParameterType::PathTypeKeyAndValue => match other {
                ParameterType::QueryType => Ordering::Greater,
                ParameterType::PathTypeKeyAndValue => Ordering::Equal,
                ParameterType::PathTypeOnlyValue => Ordering::Less,
            },
            ParameterType::QueryType => match other {
                ParameterType::QueryType => Ordering::Equal,
                ParameterType::PathTypeKeyAndValue => Ordering::Less,
                ParameterType::PathTypeOnlyValue => Ordering::Less,
            },
        };
        self.cmp(&other)
    }
}

impl PartialOrd for ParameterType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for ParameterType {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}
