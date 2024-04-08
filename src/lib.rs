use std::collections::VecDeque;

pub type ExtractResult<T> = Result<T, String>;

pub fn extract(pattern: &str, value: toml::Value) -> ExtractResult<String> {
    let parts: VecDeque<_> = pattern.split('.').map(ToString::to_string).collect();
    handle(pattern, parts, value)
}

fn handle(pattern: &str, parts: VecDeque<String>, value: toml::Value) -> ExtractResult<String> {
    let mut parts = parts;
    match value {
        toml::Value::String(value) => check_primitive(pattern, parts, value.to_string()),
        toml::Value::Integer(value) => check_primitive(pattern, parts, value.to_string()),
        toml::Value::Float(value) => check_primitive(pattern, parts, value.to_string()),
        toml::Value::Boolean(value) => check_primitive(pattern, parts, value.to_string()),
        toml::Value::Datetime(value) => check_primitive(pattern, parts, value.to_string()),
        toml::Value::Array(value) => {
            if let Some(first) = parts.pop_front() {
                if let Ok(index) = first.parse::<usize>() {
                    if let Some(value) = value.get(index) {
                        handle(pattern, parts, value.clone())
                    } else {
                        let error_msg = format!("Array index out of bounds [{first}]");
                        Err(construct_error(pattern, &first, &error_msg))
                    }
                } else {
                    let error_msg = format!("Not an array index [{first}]");
                    Err(construct_error(pattern, &first, &error_msg))
                }
            } else {
                let mut aggregated = Vec::new();
                for val in value {
                    aggregated.push(handle(pattern, VecDeque::new(), val.clone())?);
                }
                Ok(aggregated.join("\n"))
            }
        }
        toml::Value::Table(value) => {
            if let Some(first) = parts.pop_front() {
                if let Some(v) = value.get(&first) {
                    handle(pattern, parts, v.clone())
                } else {
                    let error_msg = format!("No such property [{first}]");
                    Err(construct_error(pattern, &first, &error_msg))
                }
            } else {
                let mut aggregated = Vec::new();
                for entry in value {
                    let value = handle(pattern, VecDeque::new(), entry.1.clone())?;
                    aggregated.push(format!("{} = {value}", entry.0));
                }
                Ok(aggregated.join("\n"))
            }
        }
    }
}

fn construct_error(pattern: &str, part: &str, msg: &str) -> String {
    let index = pattern.find(part).unwrap_or(0);
    let offset = " ".repeat(index);
    format!("{pattern}\n{offset}^ {msg}")
}

fn check_primitive(pattern: &str, parts: VecDeque<String>, value: String) -> ExtractResult<String> {
    let mut parts = parts;
    if let Some(first) = parts.pop_front() {
        let error_msg = format!("No such property [{first}]");
        Err(construct_error(pattern, &first, &error_msg))
    } else {
        Ok(value)
    }
}
