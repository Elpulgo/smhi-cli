extern crate url;

use std::process;
use url::Url;

#[derive(Debug, Clone)]
pub struct Parameter {
    pub key: String,
    pub value: String,
    pub param_type: ParameterType,
}

#[derive(Debug, Clone)]
pub enum ParameterType {
    PathTypeOnlyValue = 1,
    PathTypeKeyAndValue = 2,
    QueryType = 3,
    PathEndingType = 4,
}

pub fn build_encoded_url<'a>(base_url: &str, parameters: Vec<Parameter>) -> Result<String, String> {
    let mut url = base_url.to_string();

    match parameters.capacity() {
        0 => {
            let parsed = parse(&url).unwrap();
            return Ok(parsed);
        }
        _ => {
            let mut first_query_param_executed = false;
            for param in ensure_parameter_order(&parameters) {
                match param.param_type {
                    ParameterType::QueryType if first_query_param_executed == false => {
                        url.push_str(&build_param_string(&param, true));
                        first_query_param_executed = true;
                    }
                    _ => url.push_str(&build_param_string(&param, false)),
                }
            }
        }
    }
    return parse(&url);
}

fn ensure_parameter_order(parameters: &Vec<Parameter>) -> Vec<&Parameter> {
    let filtered_query_params: Vec<&Parameter> = parameters
        .into_iter()
        .filter(|par| match par.param_type {
            ParameterType::QueryType => true,
            _ => false,
        })
        .collect();

    let rest_of_params: Vec<&Parameter> = parameters
        .into_iter()
        .filter(|par| match par.param_type {
            ParameterType::QueryType => false,
            ParameterType::PathEndingType => false,
            _ => true,
        })
        .collect();

    let ending_params: Vec<&Parameter> = parameters
        .into_iter()
        .filter(|par| match par.param_type {
            ParameterType::PathEndingType => true,
            _ => false,
        })
        .collect();

    if ending_params.capacity() > 1 {
        eprintln!(
            "Only a single ending path parameter is allowed in URL '{:?}'. Please report bug! Bye bye.", ending_params
        );
        process::exit(1);
    }

    let ordered_parameters = [
        &rest_of_params[..],
        &filtered_query_params[..],
        &ending_params[..],
    ]
    .concat();

    return ordered_parameters;
}

fn build_param_string(param: &Parameter, first_query_param: bool) -> String {
    match param.param_type {
        ParameterType::PathTypeOnlyValue => format!("/{Value}", Value = param.value),
        ParameterType::PathEndingType => format!("&{Value}", Value = param.value),
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
    return value[0..take].to_string();
}
