use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::error::{FerrimonError, Result};
use crate::types::{CpuMetrics, OutputFormat};

pub struct MetricsWriter {
    csv_file: Option<csv::Writer<File>>,
    ndjson_file: Option<File>,
    workdir: PathBuf,
}

impl MetricsWriter {
    pub fn new(workdir: &Path, format: OutputFormat) -> Result<Self> {
        std::fs::create_dir_all(workdir).map_err(FerrimonError::DirectoryCreate)?;

        let csv_file = if format == OutputFormat::Csv || format == OutputFormat::Both {
            let path = workdir.join("metrics.csv");
            let file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(&path)
                .map_err(FerrimonError::FileOpen)?;
            Some(csv::Writer::from_writer(file))
        } else {
            None
        };

        let ndjson_file = if format == OutputFormat::Ndjson || format == OutputFormat::Both {
            let path = workdir.join("metrics.ndjson");
            Some(
                OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(&path)
                    .map_err(FerrimonError::FileOpen)?,
            )
        } else {
            None
        };

        Ok(Self {
            csv_file,
            ndjson_file,
            workdir: workdir.to_path_buf(),
        })
    }

    pub fn write(&mut self, metrics: &CpuMetrics) -> Result<()> {
        if let Some(ref mut csv_writer) = self.csv_file {
            csv_writer
                .serialize(metrics)
                .map_err(FerrimonError::CsvSerialize)?;
            csv_writer.flush().map_err(FerrimonError::FileWrite)?;
        }

        if let Some(ref mut file) = self.ndjson_file {
            let json = serde_json::to_string(metrics).map_err(FerrimonError::JsonSerialize)?;
            writeln!(file, "{}", json).map_err(FerrimonError::FileWrite)?;
        }

        Ok(())
    }

    pub fn csv_path(&self) -> PathBuf {
        self.workdir.join("metrics.csv")
    }

    pub fn ndjson_path(&self) -> PathBuf {
        self.workdir.join("metrics.ndjson")
    }
}
