use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum FerrimonError {
    #[error("Failed to read CPU stats from /proc/stat: {0}")]
    CpuStatsRead(#[source] io::Error),

    #[error("Failed to parse CPU stats field '{field}': invalid value")]
    CpuStatsParse { field: &'static str },

    #[error("Failed to parse CPU stats: missing required fields")]
    CpuStatsMissingFields,

    #[error("Failed to create metrics directory: {0}")]
    DirectoryCreate(#[source] io::Error),

    #[error("Failed to open metrics file: {0}")]
    FileOpen(#[source] io::Error),

    #[error("Failed to write metrics: {0}")]
    FileWrite(#[source] io::Error),

    #[error("Failed to serialize metrics to JSON: {0}")]
    JsonSerialize(#[source] serde_json::Error),

    #[error("Failed to serialize metrics to CSV: {0}")]
    CsvSerialize(#[source] csv::Error),
}

pub type Result<T> = std::result::Result<T, FerrimonError>;
