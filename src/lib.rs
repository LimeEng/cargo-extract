use toml::Value;

pub type ExtractResult<T> = Result<T, String>;

pub fn extract(pattern: &str, value: &Value) -> ExtractResult<String> {
    let parts: Vec<&str> = pattern.split('.').collect();
    handle(pattern, &parts, value)
}

fn handle(pattern: &str, parts: &[&str], value: &Value) -> ExtractResult<String> {
    match value {
        // If included in the below "v @ "-binding pattern
        // it produces strings with extra quotes
        Value::String(value) => check_primitive(pattern, parts, value.to_string()),
        value @ (Value::Integer(_) | Value::Float(_) | Value::Boolean(_) | Value::Datetime(_)) => {
            check_primitive(pattern, parts, value.to_string())
        }
        Value::Array(value) => handle_array(pattern, parts, value),
        Value::Table(value) => handle_table(pattern, parts, value),
    }
}

fn handle_array(pattern: &str, parts: &[&str], value: &[Value]) -> ExtractResult<String> {
    match parts.split_first() {
        Some((first, rest)) if first.parse::<usize>().is_ok() => value
            .get(first.parse::<usize>().unwrap())
            .map(|v| handle(pattern, rest, v))
            .unwrap_or_else(|| {
                Err(construct_error(
                    pattern,
                    first,
                    &format!("Array index out of bounds [{first}]"),
                ))
            }),
        Some((first, _)) => Err(construct_error(
            pattern,
            first,
            &format!("Not an array index [{first}]"),
        )),
        None => value
            .iter()
            .map(|v| handle(pattern, &[], v))
            .collect::<ExtractResult<Vec<_>>>()
            .map(|v| v.join("\n")),
    }
}

fn handle_table(
    pattern: &str,
    parts: &[&str],
    value: &toml::map::Map<String, Value>,
) -> ExtractResult<String> {
    match parts.split_first() {
        Some((first, rest)) => value
            .get(*first)
            .map(|v| handle(pattern, rest, v))
            .unwrap_or_else(|| {
                Err(construct_error(
                    pattern,
                    first,
                    &format!("No such property [{first}]"),
                ))
            }),
        None => value
            .iter()
            .map(|(k, v)| handle(pattern, &[], v).map(|val| format!("{k} = {val}")))
            .collect::<ExtractResult<Vec<_>>>()
            .map(|v| v.join("\n")),
    }
}

fn construct_error(pattern: &str, part: &str, msg: &str) -> String {
    let index = pattern.find(part).unwrap_or(0);
    let offset = " ".repeat(index);
    format!("{pattern}\n{offset}^ {msg}")
}

fn check_primitive(pattern: &str, parts: &[&str], value: String) -> ExtractResult<String> {
    if parts.is_empty() {
        Ok(value)
    } else {
        Err(construct_error(
            pattern,
            parts[0],
            &format!("No such property [{}]", parts[0]),
        ))
    }
}
