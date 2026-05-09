# AGENTS.md

System monitor for short-term benchmark testing with NUMA support and web visualization.

## Project Architecture

**Purpose**: Monitor system and process metrics during benchmark tests (seconds to hours).

**Key Components**:
- Metrics collector (CPU, memory, swap, NUMA stats)
- Process manager (starts and monitors target processes)
- Data storage (CSV + NDJSON dual format)
- Web server (axum-based visualization)

**Data Flow**:
1. Ferrimon starts target process via CLI or config file
2. Collector samples metrics at configurable interval (default 100ms)
3. Data written to working directory in CSV and/or NDJSON
4. Web server serves historical charts and comparison views

## Commands

- `cargo build` - build the project
- `cargo test` - run tests
- `cargo run -- --workdir ./data -- <command>` - run with target process
- `cargo run -- serve --workdir ./data` - launch visualization server
- `cargo clippy` - lint with clippy
- `cargo fmt` - format code

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
- Config format: YAML
- Metrics interval: 100ms or lower (configurable)

**NUMA Support**:
- Reads from `/sys/devices/system/node/` and `/proc/` filesystems
- Falls back gracefully on non-NUMA systems
- Tracks per-NUMA-node CPU/memory for both server and processes

**Process Management**:
- Target processes started by ferrimon (not monitoring existing PIDs)
- Supports both CLI arguments after `--` and YAML config files

**Data Storage**:
- All metrics stored to working directory on disk
- CSV for spreadsheet import, NDJSON for programmatic access
- No in-memory buffering beyond collection interval