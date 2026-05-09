use std::path::Path;
use std::time::Duration;
use tokio_util::sync::CancellationToken;
use tracing::{error, info, warn};

use crate::{CpuMetrics, MetricsWriter, OutputFormat, Result, calculate_cpu_usage, read_cpu_stats};

pub async fn run_collection_loop(
    workdir: &Path,
    interval: Duration,
    format: OutputFormat,
    cancel_token: CancellationToken,
) -> Result<u64> {
    let mut writer = MetricsWriter::new(workdir, format)?;
    let mut prev_metrics: Option<CpuMetrics> = None;
    let mut counter = 0u64;

    loop {
        tokio::select! {
            _ = cancel_token.cancelled() => {
                break;
            }
            _ = tokio::time::sleep(interval) => {
                match read_cpu_stats() {
                    Ok(metrics) => {
                        if let Err(e) = writer.write(&metrics) {
                            error!(error = %e, "Failed to write metrics");
                        } else {
                            counter += 1;
                            if counter.is_multiple_of(10) && let Some(prev) = &prev_metrics {
                                let usage = calculate_cpu_usage(prev, &metrics);
                                info!(count = counter, usage_percent = %usage, "Collected metrics");
                            }
                        }
                        prev_metrics = Some(metrics);
                    }
                    Err(e) => {
                        warn!(error = %e, "Failed to collect CPU metrics");
                    }
                }
            }
        }
    }

    info!(total_samples = counter, "Collector stopped");
    Ok(counter)
}
