use clap::{Parser, ValueEnum};
use std::path::PathBuf;
use std::time::Duration;
use tokio::signal;
use tracing::{error, info, warn};

use ferrimon_common::{MetricsWriter, OutputFormat, collect_cpu_metrics};

#[derive(Parser, Debug)]
#[command(name = "ferrimon-collector")]
#[command(about = "Collect CPU metrics and write to CSV/NDJSON")]
struct Args {
    #[arg(short, long, default_value = "./data")]
    workdir: PathBuf,

    #[arg(short = 'i', long, default_value = "100", value_name = "MS")]
    interval_ms: u64,

    #[arg(short, long, default_value = "both", value_enum)]
    format: FormatArg,
}

#[derive(Debug, Clone, ValueEnum)]
enum FormatArg {
    Csv,
    Ndjson,
    Both,
}

impl From<FormatArg> for OutputFormat {
    fn from(arg: FormatArg) -> Self {
        match arg {
            FormatArg::Csv => OutputFormat::Csv,
            FormatArg::Ndjson => OutputFormat::Ndjson,
            FormatArg::Both => OutputFormat::Both,
        }
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();

    let args = Args::parse();
    let format = OutputFormat::from(args.format);

    info!(
        workdir = %args.workdir.display(),
        interval_ms = args.interval_ms,
        format = ?format,
        "Starting ferrimon collector"
    );

    let mut writer = MetricsWriter::new(&args.workdir, format)?;
    let interval = Duration::from_millis(args.interval_ms);
    let mut prev_stats = None;
    let mut counter = 0u64;

    let mut sigterm = signal::unix::signal(signal::unix::SignalKind::terminate())?;
    let mut sigint = signal::unix::signal(signal::unix::SignalKind::interrupt())?;

    loop {
        tokio::select! {
            _ = sigterm.recv() => {
                info!("Received SIGTERM, shutting down");
                break;
            }
            _ = sigint.recv() => {
                info!("Received SIGINT, shutting down");
                break;
            }
            _ = tokio::time::sleep(interval) => {
                if let Some((metrics, curr_stats)) = collect_cpu_metrics(prev_stats) {
                    if let Err(e) = writer.write(&metrics) {
                        error!(error = %e, "Failed to write metrics");
                    } else {
                        counter += 1;
                        if counter.is_multiple_of(10) {
                            info!(count = counter, usage_percent = %metrics.usage_percent, "Collected metrics");
                        }
                    }
                    prev_stats = Some(curr_stats);
                } else {
                    warn!("Failed to collect CPU metrics");
                }
            }
        }
    }

    if let Err(e) = writer.flush() {
        error!(error = %e, "Failed to flush metrics");
    }

    info!(total_samples = counter, "Collector stopped");
    Ok(())
}
