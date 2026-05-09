use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use crate::error::{FerrimonError, Result};
use crate::types::CpuMetrics;

type FieldSetter = fn(&mut CpuMetrics, u64);

const CPU_FIELDS: [(&str, usize, FieldSetter); 10] = [
    ("user", 1, |m, v| m.user = v),
    ("nice", 2, |m, v| m.nice = v),
    ("system", 3, |m, v| m.system = v),
    ("idle", 4, |m, v| m.idle = v),
    ("iowait", 5, |m, v| m.iowait = v),
    ("irq", 6, |m, v| m.irq = v),
    ("softirq", 7, |m, v| m.softirq = v),
    ("steal", 8, |m, v| m.steal = v),
    ("guest", 9, |m, v| m.guest = v),
    ("guest_nice", 10, |m, v| m.guest_nice = v),
];

pub fn read_cpu_stats() -> Result<CpuMetrics> {
    let path = Path::new("/proc/stat");
    let file = File::open(path).map_err(FerrimonError::CpuStatsRead)?;
    let reader = BufReader::new(file);

    for line in reader.lines().map_while(std::result::Result::ok) {
        if line.starts_with("cpu ") {
            let parts: Vec<&str> = line.split_whitespace().collect();

            let mut metrics = CpuMetrics {
                timestamp: chrono::Utc::now(),
                ..Default::default()
            };

            for (field_name, parts_idx, setter) in CPU_FIELDS.iter() {
                let value = parts
                    .get(*parts_idx)
                    .ok_or(FerrimonError::CpuStatsMissingFields)?
                    .parse()
                    .map_err(|_| FerrimonError::CpuStatsParse { field: field_name })?;
                setter(&mut metrics, value);
            }

            return Ok(metrics);
        }
    }

    Err(FerrimonError::CpuStatsMissingFields)
}

pub fn calculate_cpu_usage(prev: &CpuMetrics, curr: &CpuMetrics) -> f64 {
    let prev_idle = prev.idle + prev.iowait;
    let curr_idle = curr.idle + curr.iowait;

    let prev_total: u64 = prev.user
        + prev.nice
        + prev.system
        + prev.idle
        + prev.iowait
        + prev.irq
        + prev.softirq
        + prev.steal;
    let curr_total: u64 = curr.user
        + curr.nice
        + curr.system
        + curr.idle
        + curr.iowait
        + curr.irq
        + curr.softirq
        + curr.steal;

    let total_delta = curr_total.saturating_sub(prev_total) as f64;
    let idle_delta = curr_idle.saturating_sub(prev_idle) as f64;

    if total_delta == 0.0 {
        return 0.0;
    }

    let usage = ((total_delta - idle_delta) / total_delta) * 100.0;
    usage.clamp(0.0, 100.0)
}
