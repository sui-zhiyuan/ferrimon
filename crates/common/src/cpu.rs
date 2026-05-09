use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use tracing::warn;

use crate::types::CpuMetrics;

pub type CpuStats = (u64, u64, u64, u64, u64, u64, u64, u64, u64, u64);

pub fn read_cpu_stats() -> Option<CpuStats> {
    let path = Path::new("/proc/stat");
    let file = File::open(path).ok()?;
    let reader = BufReader::new(file);

    for line in reader.lines().map_while(Result::ok) {
        if line.starts_with("cpu ") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 11 {
                let user = parts[1].parse().ok()?;
                let nice = parts[2].parse().ok()?;
                let system = parts[3].parse().ok()?;
                let idle = parts[4].parse().ok()?;
                let iowait = parts[5].parse().ok()?;
                let irq = parts[6].parse().ok()?;
                let softirq = parts[7].parse().ok()?;
                let steal = parts[8].parse().ok()?;
                let guest = parts[9].parse().ok()?;
                let guest_nice = parts[10].parse().ok()?;

                return Some((
                    user, nice, system, idle, iowait, irq, softirq, steal, guest, guest_nice,
                ));
            }
        }
    }

    warn!("Could not parse /proc/stat");
    None
}

pub fn calculate_cpu_usage(prev: CpuStats, curr: CpuStats) -> f64 {
    let prev_idle = prev.3 + prev.4;
    let curr_idle = curr.3 + curr.4;

    let prev_total: u64 = prev.0 + prev.1 + prev.2 + prev.3 + prev.4 + prev.5 + prev.6 + prev.7;
    let curr_total: u64 = curr.0 + curr.1 + curr.2 + curr.3 + curr.4 + curr.5 + curr.6 + curr.7;

    let total_delta = curr_total.saturating_sub(prev_total) as f64;
    let idle_delta = curr_idle.saturating_sub(prev_idle) as f64;

    if total_delta == 0.0 {
        return 0.0;
    }

    let usage = ((total_delta - idle_delta) / total_delta) * 100.0;
    usage.clamp(0.0, 100.0)
}

pub fn collect_cpu_metrics(prev_stats: Option<CpuStats>) -> Option<(CpuMetrics, CpuStats)> {
    let curr_stats = read_cpu_stats()?;

    let usage_percent = if let Some(prev) = prev_stats {
        calculate_cpu_usage(prev, curr_stats)
    } else {
        0.0
    };

    let metrics = CpuMetrics {
        timestamp: chrono::Utc::now(),
        user: curr_stats.0,
        nice: curr_stats.1,
        system: curr_stats.2,
        idle: curr_stats.3,
        iowait: curr_stats.4,
        irq: curr_stats.5,
        softirq: curr_stats.6,
        steal: curr_stats.7,
        guest: curr_stats.8,
        guest_nice: curr_stats.9,
        usage_percent,
    };

    Some((metrics, curr_stats))
}
