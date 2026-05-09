use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::types::{CpuMetrics, OutputFormat};

pub struct MetricsWriter {
    csv_file: Option<File>,
    ndjson_file: Option<File>,
    workdir: PathBuf,
    csv_initialized: bool,
}

impl MetricsWriter {
    pub fn new(workdir: &Path, format: OutputFormat) -> std::io::Result<Self> {
        std::fs::create_dir_all(workdir)?;

        let csv_file = if format == OutputFormat::Csv || format == OutputFormat::Both {
            let path = workdir.join("metrics.csv");
            Some(OpenOptions::new().create(true).append(true).open(&path)?)
        } else {
            None
        };

        let ndjson_file = if format == OutputFormat::Ndjson || format == OutputFormat::Both {
            let path = workdir.join("metrics.ndjson");
            Some(OpenOptions::new().create(true).append(true).open(&path)?)
        } else {
            None
        };

        Ok(Self {
            csv_file,
            ndjson_file,
            workdir: workdir.to_path_buf(),
            csv_initialized: false,
        })
    }

    pub fn write(&mut self, metrics: &CpuMetrics) -> std::io::Result<()> {
        if let Some(ref mut file) = self.csv_file {
            if !self.csv_initialized {
                writeln!(
                    file,
                    "timestamp,user,nice,system,idle,iowait,irq,softirq,steal,guest,guest_nice,usage_percent"
                )?;
                self.csv_initialized = true;
            }
            writeln!(
                file,
                "{},{},{},{},{},{},{},{},{},{},{},{:.2}",
                metrics.timestamp.to_rfc3339(),
                metrics.user,
                metrics.nice,
                metrics.system,
                metrics.idle,
                metrics.iowait,
                metrics.irq,
                metrics.softirq,
                metrics.steal,
                metrics.guest,
                metrics.guest_nice,
                metrics.usage_percent
            )?;
        }

        if let Some(ref mut file) = self.ndjson_file {
            let json = serde_json::to_string(metrics).map_err(std::io::Error::other)?;
            writeln!(file, "{}", json)?;
        }

        Ok(())
    }

    pub fn flush(&mut self) -> std::io::Result<()> {
        if let Some(ref mut file) = self.csv_file {
            file.flush()?;
        }
        if let Some(ref mut file) = self.ndjson_file {
            file.flush()?;
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
