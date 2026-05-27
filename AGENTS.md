# AGENTS.md

Operating manual for AI agents working in this repo. Keep it compact, factual, and current.
**If you discover a fact that contradicts this file, fix the file in the same change.**

---

## 1. What This Repo Is

Ferrimon — a Linux-only short-duration benchmark monitor. Three concerns:

1. **Collect** aggregate CPU stats from `/proc/stat` on a fixed interval.
2. **Persist** them as CSV and/or NDJSON to a work directory.
3. **Serve** the raw files over HTTP, with a Vue frontend scaffold for future visualization.

A fourth crate (`burner`) provides CPU-bound workloads used as collection targets — this is benchmark fodder, not product code.

Linux only: `/proc/stat` is required. macOS / Windows are not supported and not in scope.

---

## 2. Workspace Map

```
ferrimon/
├── Cargo.toml                  # workspace root (resolver = "2"), 4 members
├── rust-toolchain.toml         # channel = "nightly"  (REQUIRED, see §3)
├── package.json                # frontend scripts; manifest at REPO ROOT
├── pnpm-lock.yaml              # pnpm is the package manager (not npm/yarn)
├── crates/
│   ├── common/                 # lib  ferrimon-common  — collection loop, CPU math, storage
│   ├── collector/              # bin  ferrimon         — CLI that runs the loop
│   ├── web/                    # bin  ferrimon-web     — axum server, manual handlers
│   └── burner/                 # lib + bin + bench     — Fibonacci/SIMD CPU burners
├── web-app/                    # Vue 3 frontend source + ALL its config files
├── doc/TODO.md                 # product backlog (per-core stats, more /proc/stat lines)
├── data/                       # collector output, GITIGNORED
└── .opencode/skills/           # project-level OMO skills (see §8)
```

### Crate responsibilities

| Crate              | Binary / artifact         | Owns                                                                 |
|--------------------|---------------------------|----------------------------------------------------------------------|
| `ferrimon-common`  | lib                       | `run_collection_loop`, `read_cpu_stats`, `calculate_cpu_usage`, `MetricsWriter`, `CpuMetrics`, `OutputFormat`, `FerrimonError`. **The scheduling logic lives here, not in the collector binary.** |
| `ferrimon-collector` | bin `ferrimon`         | CLI parsing (`clap`), signal handling (`SIGINT`/`SIGTERM` → `CancellationToken`), invokes `run_collection_loop`. |
| `ferrimon-web`     | bin `ferrimon-web`        | axum router with three routes (see §6). No CORS, no graceful shutdown, no `ServeDir`. |
| `ferrimon-burner`  | lib + bin `ferrimon-burner` + bench `burner_bench` | Fibonacci via matrix exponentiation: scalar, SIMD-dot, SIMD-dot-large variants. Bench harness uses `criterion`. The `main.rs` is currently a stub `println!("Hello, world!")`. |

---

## 3. Toolchain — READ THIS

- **Rust**: pinned to `nightly` via `rust-toolchain.toml`. **Do not switch to stable.**
  - `crates/burner/src/lib.rs` uses `#![feature(portable_simd)]`. This feature is unstable; stable will not compile.
  - `crates/common/src/collector_loop.rs` uses `u64::is_multiple_of` (stabilized in Rust 1.87) — works on nightly trivially.
- **Edition**: `2024` on every crate.
- **Workspace resolver**: `"2"`.
- **Node.js**: `^20.19.0 || >=22.12.0` (declared in `package.json` `engines`).
- **Package manager**: `pnpm`. Lockfile is `pnpm-lock.yaml`. **Do not run `npm install` or `yarn`** — it will create a competing lockfile and diverge from CI.

---

## 4. Commands You'll Actually Use

### Rust

```bash
cargo build                                # debug build, all crates
cargo build --release
cargo fmt --all
cargo clippy --all
cargo test                                 # 17 tests in ferrimon-burner; 0 in common/collector/web
cargo bench -p ferrimon-burner             # criterion benches; HTML reports under target/criterion/
```

### Run the collector

```bash
# default: interval=100ms, format=ndjson, workdir=./data
cargo run --bin ferrimon -- --workdir ./data --interval-ms 100 --format ndjson
cargo run --bin ferrimon -- --workdir ./data --format both    # write both CSV and NDJSON
```

CLI flags (see `crates/collector/src/main.rs`):

| Flag                | Default     | Notes                                    |
|---------------------|-------------|------------------------------------------|
| `-w`, `--workdir`   | `./data`    | created if missing                       |
| `-i`, `--interval-ms` | `100`     | tick spacing in milliseconds             |
| `-f`, `--format`    | `ndjson`    | `csv` \| `ndjson` \| `both`              |

### Run the web server

```bash
cargo run --bin ferrimon-web -- --workdir ./data --port 8080
# defaults: workdir=./data, port=8080
```

### Frontend (from repo root)

```bash
pnpm dev                # vite dev on web-app/
pnpm build              # run-p type-check + build-only (parallel)
pnpm type-check         # vue-tsc --build web-app/tsconfig.json
pnpm test:unit          # vitest --config web-app/vitest.config.ts
pnpm test:e2e           # start-server-and-test + cypress (production preview)
pnpm test:e2e:dev       # start-server-and-test + cypress (dev server)
pnpm lint               # run-s lint:oxlint lint:eslint (SEQUENTIAL)
pnpm format             # oxfmt (NOT prettier)
```

---

## 5. Behavior & Implementation Gotchas

These are non-obvious facts. Reading the source confirms each one — they are flagged here so agents don't have to rediscover them.

### Collector loop (`crates/common/src/collector_loop.rs`)

- Uses **absolute wall-clock ticks** based on `SystemTime::now() / interval`, not relative sleeps. This means restarts re-align to the global tick grid, not to "now + interval".
- **Wake-too-early** (clock skew, scheduler quirks) → logs `warn!("wake too early")` and pushes the target forward by the missing ticks.
- **Wake-too-late** (overran the interval) → logs `warn!("Collection loop overran interval; skipping missed ticks")` and **skips** the missed ticks rather than catching up. Sample density drops, no backfill.
- Logs an `info!` line with `usage_percent` every 10th successful sample (`counter.is_multiple_of(10)`).
- **Known bug**: `get_current_tick` at `crates/common/src/collector_loop.rs:82` calls `next_tick_time.duration_since(curr_time).expect(...)`. On clock-edge races (observed on WSL2 with `--interval-ms 100`) `next_tick_time < curr_time` and the collector panics with `SystemTimeError` before writing a single sample. Tracked informally — fix is to handle the `Err` (e.g. retry next tick) instead of `.expect`.

### CPU math (`crates/common/src/cpu.rs`)

- Reads **only the aggregate `cpu ` line** from `/proc/stat`. Per-core (`cpu0..cpuN`) is on the TODO list (`doc/TODO.md`), not yet implemented.
- `total = user + nice + system + idle + iowait + irq + softirq + steal`. `guest` and `guest_nice` are **excluded** — they are already counted inside `user` and `nice` per the Linux kernel docs.
- `idle_for_usage = idle + iowait`. `iowait` is treated as idle. `steal` is treated as used (VM stolen time counts against you).
- Result is clamped to `[0.0, 100.0]`.

### Storage (`crates/common/src/storage.rs`)

- `MetricsWriter` opens files in **`create(true).append(true)`** mode. **Restarting the collector continues appending** to existing `metrics.csv` / `metrics.ndjson`. There is no truncation, no rotation, no header dedup. If you need a clean run, delete the workdir first.
- CSV writer flushes after every record. NDJSON is `writeln!` per record (line-buffered by the OS, no explicit flush).

### Web server (`crates/web/src/main.rs`)

- Routes are hand-rolled: `GET /` (inline HTML index), `GET /metrics.csv`, `GET /metrics.ndjson`. **No `ServeDir`** even though `tower-http` is pulled in with the `fs` feature.
- Reads the file fresh on every request via `tokio::fs::read` (full read into memory, not streamed).
- **No CORS headers.** Frontend must hit the server same-origin or via Vite's dev proxy.
- **No graceful shutdown.** Unlike the collector, the web server does not wire `CancellationToken` or signal handling — it just runs `axum::serve` until the process is killed.
- Missing file → `404` with a `text/plain` error body. Not a JSON error envelope.

### Frontend

- Vue 3 + Vite + Pinia + Vue Router scaffold. **No metrics UI yet.** Default template.
- `web-app/` holds all frontend config: `vite.config.ts`, `vitest.config.ts`, `cypress.config.ts`, `tsconfig.json`, `eslint.config.ts`, `.oxlintrc.json`, `.oxfmtrc.json`.
- Build output: `target/web-app/dist`. ESLint cache: `target/web-app/eslintcache`. Both inside `target/` so `cargo clean` wipes them too.
- Linter stack: **oxlint then eslint, sequential** (`run-s`). Formatter is **oxfmt**, not prettier.

---

## 6. HTTP Surface

| Method | Path              | Returns                                                  |
|--------|-------------------|----------------------------------------------------------|
| GET    | `/`               | Inline HTML index linking to the two file routes         |
| GET    | `/metrics.csv`    | `text/csv` with `Content-Disposition: attachment` or 404 |
| GET    | `/metrics.ndjson` | `application/x-ndjson` with `Content-Disposition: attachment` or 404 |

---

## 7. Data Schema

`CpuMetrics` (see `crates/common/src/types.rs`) — fields in order, all `u64` clock ticks except `timestamp`:

`timestamp` (RFC3339 UTC), `user`, `nice`, `system`, `idle`, `iowait`, `irq`, `softirq`, `steal`, `guest`, `guest_nice`.

CSV header is written once on file create (csv crate default). NDJSON has no header — one JSON object per line.

---

## 8. OMO / Agent Workflow Hints

### Available skills (in `.opencode/skills/`)

| Skill                         | When to load                                                            |
|-------------------------------|-------------------------------------------------------------------------|
| `review-and-squash-commits`   | User asks to review the feature branch's commits and squash them into fewer logical commits before merge. Load via `task(load_skills=["review-and-squash-commits"], ...)`. |

If you add a project skill, drop it under `.opencode/skills/<name>/SKILL.md` and list it here.

### Built-in skills that are particularly relevant here

- `git-master` — for any commit / rebase / squash / blame work in this repo.
- `ai-slop-remover` — useful before merging, since this codebase has had multiple AI authors.

### Product backlog

`doc/TODO.md` lists collector gaps (per-core stats, additional `/proc/stat` lines like `intr`, `ctxt`, `softirq`). Consult it before proposing new collector features so you don't duplicate or contradict planned work.

---

## 9. Running, Smoke-Testing, Verifying

- For short collector runs in smoke tests, use `timeout`:
  ```bash
  timeout 2 cargo run --bin ferrimon -- --workdir /tmp/ferrimon-smoke --interval-ms 100 --format both
  ls /tmp/ferrimon-smoke && wc -l /tmp/ferrimon-smoke/metrics.ndjson
  ```
  **Do not** background with `&` and `kill $PID` in this shell environment — it's unreliable here.
- The web server has no built-in shutdown. Smoke-test it with `timeout` + `curl`:
  ```bash
  (timeout 3 cargo run --bin ferrimon-web -- --workdir /tmp/ferrimon-smoke --port 18080 &)
  sleep 1 && curl -sI http://127.0.0.1:18080/metrics.ndjson
  ```
- `cargo test` runs **17 tests in `ferrimon-burner`** (all green) and **0 tests in `common`/`collector`/`web`**. A green `cargo test` therefore proves nothing about the collector or web binary. Do not claim "tests pass" as functional verification for those crates. Add real tests if you change their behavior.

---

## 10. Git Workflow For This Repo

- Branch from `master` as `feature/<short-description>`.
- Commit on the feature branch at task end. Never commit directly to `master`.
- Merge back into `master` with `--no-ff`. Write a real merge summary (why + key changes), not just the branch name.
- Squash noisy WIP commits before merge — use the `review-and-squash-commits` skill (§8).

---

## 11. Style Convention

- Add comments only when explaining non-obvious logic. The CPU-field counting rule in `cpu.rs` is the canonical "deserves a comment" example.
- Match the surrounding code. Workspace dependencies live in root `Cargo.toml`; per-crate `Cargo.toml` references them via `.workspace = true` — keep that pattern.
- Rust: `cargo fmt --all` is authoritative. Frontend: `oxfmt` is authoritative.
