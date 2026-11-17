use crate::ExtractResult;
use clap::builder::PossibleValue;
use serde_json::{Number, Value as JsonValue};
use toml::Value;

/// Format used to render an extracted TOML value.
#[derive(Clone, Copy, Debug, Default)]
pub enum OutputFormat {
    /// Unquoted strings and newline-separated collections.
    #[default]
    Plain,
    /// JSON values, with TOML dates and times rendered as strings.
    Json,
}

impl clap::ValueEnum for OutputFormat {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Plain, Self::Json]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(PossibleValue::new(match self {
            Self::Plain => "plain",
            Self::Json => "json",
        }))
    }
}

impl OutputFormat {
    /// Render a TOML value in this format.
    ///
    /// # Errors
    ///
    /// JSON rendering fails if the value contains a non-finite float or
    /// serialization fails.
    pub fn render(self, value: &Value) -> ExtractResult<String> {
        match self {
            Self::Plain => Ok(format_plain(value)),
            Self::Json => {
                let json = to_json(value)?;
                serde_json::to_string(&json)
                    .map_err(|err| format!("Failed to serialize JSON: {err}"))
            }
        }
    }
}

fn format_plain(value: &Value) -> String {
    match value {
        Value::String(value) => value.clone(),
        Value::Array(value) => value
            .iter()
            .map(format_plain)
            .collect::<Vec<_>>()
            .join("\n"),
        Value::Table(value) => value
            .iter()
            .map(|(k, v)| format!("{k} = {}", format_plain(v)))
            .collect::<Vec<_>>()
            .join("\n"),
        value => value.to_string(),
    }
}

fn to_json(value: &Value) -> ExtractResult<JsonValue> {
    match value {
        Value::String(value) => Ok(JsonValue::String(value.clone())),
        Value::Integer(value) => Ok(JsonValue::Number((*value).into())),
        Value::Float(value) => Number::from_f64(*value)
            .map(JsonValue::Number)
            .ok_or_else(|| format!("Cannot represent non-finite float {value} in JSON")),
        Value::Boolean(value) => Ok(JsonValue::Bool(*value)),
        Value::Datetime(value) => Ok(JsonValue::String(value.to_string())),
        Value::Array(values) => values
            .iter()
            .map(to_json)
            .collect::<ExtractResult<Vec<_>>>()
            .map(JsonValue::Array),
        Value::Table(values) => values
            .iter()
            .map(|(key, value)| Ok((key.clone(), to_json(value)?)))
            .collect::<ExtractResult<serde_json::Map<_, _>>>()
            .map(JsonValue::Object),
    }
}
