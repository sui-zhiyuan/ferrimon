# AGENTS.md

System monitor for short-term benchmark testing with NUMA support and web visualization.

## Toolchain Requirements

- Edition **2024**, Cargo resolver **2**, Cargo.lock version **4**
- Requires Rust **1.87+** (`is_multiple_of` on integers, used in `collector_loop.rs`)

## Crate Map

| Crate              | Package name         | Binary             | Role                                                               |
|--------------------|----------------------|--------------------|--------------------------------------------------------------------|
| `crates/collector` | `ferrimon-collector` | **`ferrimon`**     | CLI: reads `/proc/stat`, writes CSV/NDJSON, handles SIGTERM/SIGINT |
| `crates/web`       | `ferrimon-web`       | **`ferrimon-web`** | HTTP: serves collected files for download via axum                 |
| `crates/common`    | `ferrimon-common`    | _(library)_        | Shared types, CPU reader, storage writer, collection loop          |

The collection loop logic lives in **`crates/common`**, not in the collector binary.

## Commands

```bash
cargo build                                                          # build all
cargo build --release                                                # optimized
cargo test                                                           # compiles but no tests exist yet
cargo clippy --all                                                   # lint
cargo fmt --all                                                      # format

cargo run --bin ferrimon -- --workdir ./data                         # collect (default: 100ms, ndjson)
cargo run --bin ferrimon -- --workdir ./data --interval-ms 200 --format csv
cargo run --bin ferrimon -- --workdir ./data --format both           # csv + ndjson

cargo run --bin ferrimon-web -- --workdir ./data --port 8080         # web server
```

Collector CLI args: `--workdir` (default `./data`), `--interval-ms` (default `100`), `--format csv|ndjson|both` (default `ndjson`).

Web server CLI args: `--workdir` (default `./data`), `--port` (default `8080`).

## Data Flow

1. Collector reads aggregate `cpu` line from `/proc/stat` at each interval
2. Writes to `<workdir>/metrics.csv` and/or `<workdir>/metrics.ndjson` (append mode, flushed per sample)
3. Web server reads those files from disk on each request and serves them as downloads
4. Web routes: `GET /` (HTML index), `GET /metrics.csv`, `GET /metrics.ndjson`

## Code Style

- Add comments only when necessary — if the code can speak for itself, omit the comment.

## Non-Obvious Implementation Details

- **CPU usage calculation** (`crates/common/src/cpu.rs`): excludes `guest`/`guest_nice` from total (they are sub-counts of `user`/`nice`); `steal` is counted as used. This is the correct Linux interpretation.
- **`tower-http` fs feature** is declared as a dependency in `crates/web` but routes are implemented manually — `ServeDir` is not used.
- **`ferrimon-common` is imported** by `crates/web` but currently unused there in practice.
- **No graceful shutdown** in the web server (unlike the collector which handles SIGTERM/SIGINT via `CancellationToken`).
- **No tests** exist anywhere in the workspace (`cargo test` compiles but runs nothing).
- Planned work tracked in `doc/TODO.md`: per-CPU core stats, additional `/proc/stat` lines, interval drift compensation.

## Smoke Test

Always use `timeout` to run the collector — do not use `&` + `kill $PID` (unreliable in persistent shell sessions).

## Git Workflow

Before making changes:
1. `git checkout -b feature/<description>`
2. Commit incrementally

At the end of every task:
- Check the current branch with `git branch --show-current`
- If already on a `feature/*` branch: commit all changes there
- If on any other branch (e.g. `master`): create `git checkout -b feature/<description>` first, then commit

To merge:
1. `git fetch origin && git rebase origin/master`
2. `git checkout master && git merge --no-ff feature/<description>` — use a meaningful summary as the merge commit message

## Planned Components (Not Yet Implemented)

- Per-CPU core metrics (currently only aggregate `cpu` line)
- Memory/swap metrics
- NUMA stats (`/sys/devices/system/node/`, `/proc/`)
- Process manager (starts target processes, not monitoring existing PIDs)
- YAML config file support
- Web visualization with charts
