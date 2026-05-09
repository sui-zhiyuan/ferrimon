use anyhow::Result;
use clap::{Parser, ValueEnum};
use std::path::PathBuf;
use std::time::Duration;
use tokio::signal;
use tokio_util::sync::CancellationToken;
use tracing::info;

use ferrimon_common::{OutputFormat, run_collection_loop};

#[derive(Parser, Debug)]
#[command(name = "ferrimon-collector")]
#[command(about = "Collect CPU metrics and write to CSV/NDJSON")]
struct Args {
    #[arg(short, long, default_value = "./data")]
    workdir: PathBuf,

    #[arg(short = 'i', long, default_value = "100", value_name = "MS")]
    interval_ms: u64,

    #[arg(short, long, default_value = "ndjson", value_enum)]
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
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let args = Args::parse();
    let format = OutputFormat::from(args.format);
    let interval = Duration::from_millis(args.interval_ms);
    let cancel_token = CancellationToken::new();

    info!(
        workdir = %args.workdir.display(),
        interval_ms = args.interval_ms,
        format = ?format,
        "Starting ferrimon collector"
    );

    let cancel_token_clone = cancel_token.clone();

    let mut sigterm = signal::unix::signal(signal::unix::SignalKind::terminate())?;
    let mut sigint = signal::unix::signal(signal::unix::SignalKind::interrupt())?;

    let signal_handler = async move {
        tokio::select! {
            _ = sigterm.recv() => {
                info!("Received SIGTERM, shutting down");
            }
            _ = sigint.recv() => {
                info!("Received SIGINT, shutting down");
            }
        }
        cancel_token_clone.cancel();
    };

    tokio::spawn(signal_handler);

    run_collection_loop(&args.workdir, interval, format, cancel_token)
        .await
        .map_err(anyhow::Error::msg)?;
    Ok(())
}
