//! Extract values from a parsed TOML document using dotted table keys and array indices.

pub mod cli;

use toml::Value;
use unicode_width::UnicodeWidthStr;

/// Result returned by [`extract`], containing either a value or an error message.
pub type ExtractResult<T> = Result<T, String>;

/// Extract a TOML value using a period-separated path.
///
/// Table keys are path components, and array indices are zero-based integers.
/// Selecting a complete table or array returns its contents as
/// newline-separated text. Scalar strings are returned without quotes.
///
/// # Errors
///
/// Returns an error if a table key is missing, an array index is invalid or out
/// of bounds, or the path continues after reaching a scalar value. Errors
/// messages include the access pattern, a caret, and a description of the
/// problem.
pub fn extract(pattern: &str, value: &Value) -> ExtractResult<String> {
    let parts: Vec<&str> = pattern.split('.').collect();
    handle(pattern, &parts, value, 0)
}

/// Recursively walk the TOML value along the remaining access path, dispatching by value type.
fn handle(pattern: &str, parts: &[&str], value: &Value, offset: usize) -> ExtractResult<String> {
    match value {
        // If included in the below "v @ "-binding pattern
        // it produces strings with extra quotes
        Value::String(value) => check_primitive(pattern, parts, value.clone(), offset),
        value @ (Value::Integer(_) | Value::Float(_) | Value::Boolean(_) | Value::Datetime(_)) => {
            check_primitive(pattern, parts, value.to_string(), offset)
        }
        Value::Array(value) => handle_array(pattern, parts, value, offset),
        Value::Table(value) => handle_table(pattern, parts, value, offset),
    }
}

/// Use the next path component to select an array element; when the path ends, recursively render
/// each element.
fn handle_array(
    pattern: &str,
    parts: &[&str],
    value: &[Value],
    offset: usize,
) -> ExtractResult<String> {
    match parts.split_first() {
        Some((first, rest)) => {
            let idx = first.parse::<usize>().map_err(|_| {
                construct_error(pattern, offset, &format!("Not an array index [{first}]"))
            })?;

            match value.get(idx) {
                Some(v) => handle(pattern, rest, v, offset + first.len() + 1),
                None => Err(construct_error(
                    pattern,
                    offset,
                    &format!("Array index out of bounds [{first}]"),
                )),
            }
        }
        None => value
            .iter()
            .map(|v| handle(pattern, &[], v, offset))
            .collect::<ExtractResult<Vec<_>>>()
            .map(|v| v.join("\n")),
    }
}

/// Use the next path component to select a table entry; when the path ends, recursively render
/// each entry with its key.
fn handle_table(
    pattern: &str,
    parts: &[&str],
    value: &toml::map::Map<String, Value>,
    offset: usize,
) -> ExtractResult<String> {
    match parts.split_first() {
        Some((first, rest)) => value
            .get(*first)
            .ok_or_else(|| construct_error(pattern, offset, &format!("No such property [{first}]")))
            .and_then(|v| handle(pattern, rest, v, offset + first.len() + 1)),
        None => value
            .iter()
            .map(|(k, v)| handle(pattern, &[], v, offset).map(|val| format!("{k} = {val}")))
            .collect::<ExtractResult<Vec<_>>>()
            .map(|v| v.join("\n")),
    }
}

/// Build an error message with a caret beneath the failing path component.
fn construct_error(pattern: &str, index: usize, msg: &str) -> String {
    let offset = " ".repeat(UnicodeWidthStr::width(&pattern[..index]));
    format!("{pattern}\n{offset}^ {msg}")
}

/// Return a scalar value only when no path components remain.
fn check_primitive(
    pattern: &str,
    parts: &[&str],
    value: String,
    offset: usize,
) -> ExtractResult<String> {
    if parts.is_empty() {
        Ok(value)
    } else {
        Err(construct_error(
            pattern,
            offset,
            &format!("No such property [{}]", parts[0]),
        ))
    }
}
