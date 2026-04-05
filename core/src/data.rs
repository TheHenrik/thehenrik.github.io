use anyhow::{Context, Result};
use csv::{ReaderBuilder, WriterBuilder};
use serde::{Serialize, de::DeserializeOwned};
use std::path::Path;
pub mod teams;

use teams::Team;

pub struct Competition {
    teams: Vec<Team>,
}

/// Load all records from a CSV file.
///
/// The file must contain a header row. Each row is deserialized into `T`
/// using `serde::Deserialize`. If any row fails to deserialize the whole
/// call fails.
pub fn load_csv<T>(path: &Path) -> Result<Vec<T>>
where
    T: serde::de::DeserializeOwned,
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
    T: serde::Serialize,
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
