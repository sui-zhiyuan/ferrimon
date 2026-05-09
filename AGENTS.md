# AGENTS.md

System monitor for short-term benchmark testing with NUMA support and web visualization.

## Project Architecture

**Purpose**: Monitor system and process metrics during benchmark tests (seconds to hours).

**Current Implementation**: Minimum viable CPU monitor with separate collector and web server processes.

**Key Components**:
- **crates/common**: Shared library (types, CPU reading, storage)
- **crates/collector**: Binary that collects CPU metrics and writes to CSV/NDJSON
- **crates/web**: Binary that serves collected data via HTTP

**Data Flow**:
1. Ferrimon collector reads `/proc/stat` at configurable interval
2. Metrics calculated and written to working directory (CSV + NDJSON)
3. Web server serves files for download

**Planned Components**:
- Process manager (starts and monitors target processes)
- Memory/swap metrics collector
- NUMA stats collector
- Web visualization with charts

## Commands

- `cargo build` - build all workspace members
- `cargo build --release` - optimized build
- `cargo test` - run tests
- `cargo run --bin ferrimon-collector -- --workdir ./data` - collect CPU metrics
- `cargo run --bin ferrimon-web -- --workdir ./data --port 8080` - launch web server
- `cargo clippy --all` - lint all workspace members
- `cargo fmt --all` - format all workspace members

## Git Workflow

Before making any changes to the repository:

1. Create a feature branch: `git checkout -b feature/<description>`
2. Make changes and commit incrementally

When ready to merge:

1. Rebase to latest master: `git fetch origin && git rebase origin/master`
2. Merge with no fast-forward: `git checkout master && git merge --no-ff feature/<description>`
   - Review commits being merged and use a meaningful summary message as the merge commit message

## Development Notes

**Technology Stack**:
- Web framework: axum
- Data formats: CSV + NDJSON
- Config format: YAML (planned)
- Metrics interval: 100ms or lower (configurable)
- Workspace: 3 crates (common, collector, web)

**NUMA Support** (Planned):
- Reads from `/sys/devices/system/node/` and `/proc/` filesystems
- Falls back gracefully on non-NUMA systems
- Tracks per-NUMA-node CPU/memory for both server and processes

**Process Management** (Planned):
- Target processes started by ferrimon (not monitoring existing PIDs)
- Supports both CLI arguments after `--` and YAML config files

**Data Storage**:
- All metrics stored to working directory on disk
- CSV for spreadsheet import, NDJSON for programmatic access
- No in-memory buffering beyond collection interval