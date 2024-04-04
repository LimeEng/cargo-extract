use anyhow::{anyhow, Context};
use std::collections::VecDeque;

pub fn extract(pattern: &str, manifest: toml::Table) -> anyhow::Result<String> {
    let parts: VecDeque<_> = pattern.split('.').map(ToString::to_string).collect();
    handle(parts, toml::Value::Table(manifest))
}

fn handle(parts: VecDeque<String>, value: toml::Value) -> anyhow::Result<String> {
    let mut parts = parts;
    match value {
        toml::Value::String(value) => Ok(value.to_string()),
        toml::Value::Integer(value) => Ok(value.to_string()),
        toml::Value::Float(value) => Ok(value.to_string()),
        toml::Value::Boolean(value) => Ok(value.to_string()),
        toml::Value::Datetime(value) => Ok(value.to_string()),
        toml::Value::Array(value) => {
            if let Some(first) = parts.pop_front() {
                if let Ok(index) = first.parse::<usize>() {
                    handle(
                        parts,
                        value
                            .get(index)
                            .with_context(|| format!("Array index out of bounds {index}"))?
                            .clone(),
                    )
                } else {
                    Err(anyhow!("Failed to parse {first} into an array index"))
                }
            } else {
                let stringified = value
                    .iter()
                    .map(|val| handle(VecDeque::new(), val.clone()).expect("Ouch 1"))
                    .collect::<Vec<_>>()
                    .join("\n");
                Ok(stringified)
            }
        }
        toml::Value::Table(value) => {
            if let Some(first) = parts.pop_front() {
                let v = value
                    .get(&first)
                    .with_context(|| format!("No such key found {first}"))?;
                handle(parts, v.clone())
            } else {
                let stringified = value
                    .iter()
                    .map(|entry| {
                        let value = handle(VecDeque::new(), entry.1.clone()).expect("Ouch 2");
                        format!("{} = {value}", entry.0)
                    })
                    .collect::<Vec<_>>()
                    .join("\n");
                Ok(stringified)
            }
        }
    }
}
