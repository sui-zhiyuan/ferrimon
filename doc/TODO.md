# Uncompleted tasks

1. Parse per-CPU core stats (cpu0..cpuN)
2. Parse `/proc/stat` additional lines: `intr`, `ctxt`, `btime`, `processes`, `procs_running`, `procs_blocked`,
   `softirq`
3. lscpu to get info of CPU.
4. cat /etc/os-release to get OS info (how about core info)
5. cat /proc/cmdline get start info
6. cpupower idel-stat to get cpu idel config