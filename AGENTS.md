# AGENTS.md

Linux-only Rust workspace for short benchmark monitoring. Keep this file compact and factual.

## High-Signal Facts

- Toolchain: Rust edition `2024`, workspace resolver `2`, `Cargo.lock` format `4`.
- Requires Rust `1.87+` (`is_multiple_of` is used in `crates/common/src/collector_loop.rs`).
- There are currently no tests; `cargo test` builds crates but runs 0 tests.

## Workspace Entrypoints

- `crates/collector` (`ferrimon-collector`) builds binary `ferrimon`.
- `crates/web` (`ferrimon-web`) builds binary `ferrimon-web`.
- `crates/common` contains the real collection loop and shared logic.
- Collection scheduling logic is in `crates/common/src/collector_loop.rs`, not in collector `main.rs`.

## Commands You’ll Actually Use

```bash
cargo build
cargo build --release
cargo fmt --all
cargo clippy --all
cargo test

cargo run --bin ferrimon -- --workdir ./data --interval-ms 100 --format ndjson
cargo run --bin ferrimon -- --workdir ./data --format both
cargo run --bin ferrimon-web -- --workdir ./data --port 8080
```

## Behavior/Implementation Gotchas

- Collector reads only aggregate `cpu` from `/proc/stat` (not per-core yet).
- CPU usage math in `crates/common/src/cpu.rs` treats `guest`/`guest_nice` as sub-counts (excluded from total) and counts `steal` as used.
- Web routes are manually implemented in `crates/web/src/main.rs`: `GET /`, `GET /metrics.csv`, `GET /metrics.ndjson`.
- `tower-http` is present with `fs` feature, but static serving uses manual handlers (no `ServeDir`).
- Web server has no graceful shutdown path; collector handles `SIGINT`/`SIGTERM` via `CancellationToken`.

## Smoke Test Rule

- Use `timeout` for short collector runs; do not use background `&` + `kill $PID` in this environment.

## Git Workflow In This Repo

- Start work on `feature/<description>`.
- Commit at task end on the feature branch (create one first if currently on `master`).
- Merge with `--no-ff` and write a meaningful merge summary message (why + key changes), not just branch name.

## Style Convention

- Add comments only when needed to explain non-obvious logic.
