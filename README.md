# Ferrimon

A system monitor for short-term benchmark testing on Linux servers, with NUMA support and web-based visualization.

## Overview

Ferrimon monitors system and process-level metrics during benchmark tests that run from seconds to hours. It collects CPU, memory, swap usage for both the whole server and target processes, with full NUMA-awareness.

## Features

- **System-wide metrics**: CPU, memory, swap usage for the entire server
- **Process-level metrics**: Monitor specific processes launched by ferrimon
- **NUMA support**: Per-NUMA-node CPU/memory metrics, topology display, and process allocation tracking
- **Web visualization**: Historical charts, test run comparison, future real-time monitoring
- **Flexible process management**: Start target processes via CLI or configuration file
- **Dual data formats**: CSV and NDJSON for maximum compatibility

## Metrics Collected

### Server-wide Metrics
- CPU usage (total and per-core)
- Memory usage (used, free, cached, buffers)
- Swap usage

### Per-Process Metrics
- CPU usage
- Memory usage
- Per-NUMA-node CPU/memory allocation

### NUMA Metrics
- Per-NUMA-node CPU usage
- Per-NUMA-node memory usage
- NUMA topology information
- Process NUMA allocation

## Usage

### Basic CLI

```bash
ferrimon --workdir /path/to/data -- your-command --with args
```

### Configuration File

```bash
ferrimon --config benchmark.yaml
```

Example configuration:

```yaml
workdir: /path/to/data
interval_ms: 100
command:
  - ./my-benchmark
  - --test-case
  - workload-a
```

### Options

- `--workdir <DIR>` - Directory to store metrics data (CSV/NDJSON)
- `--config <FILE>` - Load configuration from YAML file
- `--interval <MS>` - Metrics collection interval (default: 100ms)
- `--format <FORMAT>` - Output format: csv, ndjson, or both (default: both)

## Data Storage

Metrics are stored in the working directory during execution:

- `metrics.csv` - CSV format for spreadsheet compatibility
- `metrics.ndjson` - Newline-delimited JSON for programmatic access
- `meta.json` - Test run metadata (start time, command, configuration)

## Web Interface

After a test completes, launch the visualization server:

```bash
ferrimon serve --workdir /path/to/data --port 8080
```

Features:
- Historical charts for all metrics
- Compare multiple test runs side-by-side
- Statistical summaries (min/max/avg/percentiles)

## Requirements

- Linux with NUMA support (optional, falls back gracefully on non-NUMA systems)
- Root or CAP_SYS_ADMIN may be required for detailed process metrics

## Building

```bash
cargo build --release
```

## Development

See AGENTS.md for development guidelines and project structure.

## License

[To be determined]