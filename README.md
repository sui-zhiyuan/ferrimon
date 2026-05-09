# Ferrimon

A system monitor for short-term benchmark testing on Linux servers, with NUMA support and web-based visualization.

## Overview

Ferrimon monitors system and process-level metrics during benchmark tests that run from seconds to hours. It collects CPU, memory, swap usage for both the whole server and target processes, with full NUMA-awareness.

## Current Implementation

This is the minimum viable implementation focusing on CPU monitoring:

- **System-wide CPU metrics**: Overall CPU usage percentage
- **Dual data formats**: CSV and NDJSON output
- **Collector binary**: Standalone process for metrics collection
- **Web server**: Serves collected data for download

## Project Structure

The project is organized as a Cargo workspace:

- `crates/common` - Shared library for CPU metrics, storage
- `crates/collector` - Metrics collector binary (builds `ferrimon` executable)
- `crates/web` - Web server binary (builds `ferrimon-web` executable)

## Features

- **System-wide metrics**: CPU, memory, swap usage for the entire server
- **Process-level metrics**: Monitor specific processes launched by ferrimon
- **NUMA support**: Per-NUMA-node CPU/memory metrics, topology display, and process allocation tracking
- **Web visualization**: Historical charts, test run comparison, future real-time monitoring
- **Flexible process management**: Start target processes via CLI or configuration file
- **Dual data formats**: CSV and NDJSON for maximum compatibility

## Metrics Collected

### Current (Minimum Implementation)
- Overall CPU usage percentage

### Planned
- CPU usage (total and per-core)
- Memory usage (used, free, cached, buffers)
- Swap usage
- Per-process CPU/memory
- NUMA metrics

## Usage

### Collect Metrics

Run the collector to capture CPU metrics:

```bash
# Build first
cargo build --release

# Collect metrics to ./data directory (default interval: 100ms)
./target/release/ferrimon --workdir ./data

# Collect with custom interval (200ms) and format (csv only)
./target/release/ferrimon --workdir ./data --interval-ms 200 --format csv

# The collector runs until you press Ctrl+C
```

### View Metrics

Start the web server to view/download collected metrics:

```bash
# Start web server on port 8080 (default)
./target/release/ferrimon-web --workdir ./data

# Or specify a custom port
./target/release/ferrimon-web --workdir ./data --port 9000
```

Then open http://localhost:8080 in your browser to:
- Download `metrics.csv` (CSV format)
- Download `metrics.ndjson` (NDJSON format)

### Collector Options

- `--workdir <DIR>` - Directory to store metrics data (default: ./data)
- `--interval-ms <MS>` - Collection interval in milliseconds (default: 100)
- `--format <FORMAT>` - Output format: csv, ndjson, or both (default: ndjson)

### Web Server Options

- `--workdir <DIR>` - Directory containing metrics data (default: ./data)
- `--port <PORT>` - Port to listen on (default: 8080)

## Planned Features

### Basic CLI (Planned)

```bash
ferrimon --workdir /path/to/data -- your-command --with args
```

### Configuration File (Planned)

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

### Web Interface (Planned)

Features:
- Historical charts for all metrics
- Compare multiple test runs side-by-side
- Statistical summaries (min/max/avg/percentiles)

## Data Storage

Metrics are stored in the working directory:

- `metrics.csv` - CSV format for spreadsheet compatibility
  - Columns: timestamp, user, nice, system, idle, iowait, irq, softirq, steal, guest, guest_nice, usage_percent
- `metrics.ndjson` - Newline-delimited JSON for programmatic access
  - Each line is a JSON object with the same fields

## Requirements

- Linux (reads from `/proc/stat`)
- Rust 2024 edition

## Building

```bash
cargo build --release
```

## Development

See AGENTS.md for development guidelines and project structure.

## License

[To be determined]