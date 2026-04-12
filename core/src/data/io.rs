use anyhow::{Context, Result};
use csv::{ReaderBuilder, WriterBuilder};
use ron::de::from_str;
use ron::ser::{PrettyConfig, to_string_pretty};
use serde::{Serialize, de::DeserializeOwned};
use std::fs;
use std::path::Path;

/// Load all records from a CSV file.
///
/// The file must contain a header row. Each row is deserialized into `T`
/// using `serde::Deserialize`. If any row fails to deserialize the whole
/// call fails.
pub fn load_csv<T>(path: &Path) -> Result<Vec<T>>
where
    T: DeserializeOwned,
{
    ReaderBuilder::new()
        .has_headers(true)
        .from_path(path)
        .with_context(|| format!("Failed to open CSV file: {}", path.display()))?
        .deserialize::<T>()
        .collect::<Result<Vec<T>, _>>()
        .context("Failed to deserialize CSV rows into target type")
}

/// Save a slice of records to a CSV file.
///
/// The first record written triggers the header row (derived from `T`'s field
/// names). Each call to `serialize` may fail; the first failure aborts the
/// whole operation.
pub fn save_csv<T>(path: &Path, rows: &[T]) -> Result<()>
where
    T: Serialize,
{
    let mut wtr = WriterBuilder::new()
        .has_headers(true)
        .from_path(path)
        .with_context(|| format!("Failed to open CSV file for writing: {}", path.display()))?;

    for (i, row) in rows.iter().enumerate() {
        wtr.serialize(row)
            .with_context(|| format!("Failed to serialize row {}", i))?;
    }

    // Flush ensures the internal buffer is written to the OS.
    wtr.flush().context("Failed to flush CSV writer")?;

    Ok(())
}

pub fn load_ron<T: DeserializeOwned>(path: &Path) -> Result<T> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read RON file at {}", path.display()))?;

    let value = from_str(&content)
        .with_context(|| format!("Failed to parse RON file at {}", path.display()))?;

    Ok(value)
}

pub fn save_ron<T: Serialize>(path: &Path, value: &T) -> Result<()> {
    // Pretty RON output (optional)
    let pretty = PrettyConfig::new();

    let content = to_string_pretty(value, pretty)
        .with_context(|| format!("Failed to serialize value to RON for {}", path.display()))?;

    fs::write(path, content)
        .with_context(|| format!("Failed to write RON file to {}", path.display()))?;

    Ok(())
}
