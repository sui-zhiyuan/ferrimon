use std::path::Path;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
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

    let (mut tick, _, mut next_dura) = get_current_tick(SystemTime::now(), interval)?;

    loop {
        tokio::select! {
            _ = cancel_token.cancelled() => {
                break;
            }
            _ = tokio::time::sleep(next_dura) => {
                let now_after_wake = SystemTime::now();
                let mut tick_after_wake ;
                let mut target_tick_time;
                (tick_after_wake , target_tick_time , next_dura) = get_current_tick(now_after_wake, interval)?;

                let mut next_tick = tick +1;
                if  tick_after_wake < next_tick{
                    warn!(tick_after_wake , next_tick , ?now_after_wake ,"wake too early");
                    target_tick_time += (next_tick - tick_after_wake) * interval;
                    next_dura += (next_tick - tick_after_wake) * interval;
                    tick_after_wake = next_tick;
                }

                if tick_after_wake > next_tick {
                    let missed = tick_after_wake - next_tick;
                    warn!(missed, tick, "Collection loop overran interval; skipping missed ticks");
                    next_tick = tick_after_wake;
                }

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

                tick = next_tick;
            }
        }
    }

    info!(total_samples = counter, "Collector stopped");
    Ok(counter)
}

fn get_current_tick(
    curr_time: SystemTime,
    interval: Duration,
) -> Result<(u32, SystemTime, Duration)> {
    let ticks =
        (curr_time.duration_since(UNIX_EPOCH).unwrap().as_millis() / interval.as_millis()) as u32; // TODO error
    let curr_tick_time = UNIX_EPOCH + interval * ticks;
    let next_tick_time = curr_tick_time + interval;
    let duration_for_next_tick = next_tick_time
        .duration_since(curr_time)
        .expect("next_tick_time should large then curr_time");
    Ok((ticks, curr_tick_time, duration_for_next_tick))
}
