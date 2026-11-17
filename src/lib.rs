//! Extract values from a parsed TOML document using dotted table keys and array indices.

use toml::Value;
use unicode_width::UnicodeWidthStr;

mod format;

pub use format::OutputFormat;

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
pub fn extract(pattern: &str, value: &Value, format: OutputFormat) -> ExtractResult<String> {
    let value = extract_value(pattern, value)?;
    format.render(value)
}

/// Extract a TOML value using a period-separated path.
fn extract_value<'a>(pattern: &str, value: &'a Value) -> ExtractResult<&'a Value> {
    let mut current = value;
    let mut offset = 0;
    for part in pattern.split('.') {
        current = match current {
            Value::Table(table) => table.get(part).ok_or_else(|| {
                construct_error(pattern, offset, &format!("No such property [{part}]"))
            })?,
            Value::Array(array) => {
                let idx = part.parse::<usize>().map_err(|_| {
                    construct_error(pattern, offset, &format!("Not an array index [{part}]"))
                })?;
                array.get(idx).ok_or_else(|| {
                    construct_error(
                        pattern,
                        offset,
                        &format!("Array index out of bounds [{part}]"),
                    )
                })?
            }
            _ => {
                return Err(construct_error(
                    pattern,
                    offset,
                    &format!("No such property [{part}]"),
                ));
            }
        };
        offset += part.len() + 1;
    }
    Ok(current)
}

/// Build an error message with a caret beneath the failing path component.
fn construct_error(pattern: &str, index: usize, msg: &str) -> String {
    let offset = " ".repeat(UnicodeWidthStr::width(&pattern[..index]));
    format!("{pattern}\n{offset}^ {msg}")
}
