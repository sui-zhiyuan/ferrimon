# Uncompleted tasks

1. Time-based collection loop (compensate for overhead, use `tokio::time::interval`)
2. Parse per-CPU core stats (cpu0..cpuN)
3. Parse `/proc/stat` additional lines: `intr`, `ctxt`, `btime`, `processes`, `procs_running`, `procs_blocked`, `softirq`
