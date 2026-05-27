# Ferrimon

Linux-only short-duration benchmark monitor. Collects CPU stats from `/proc/stat`, persists them as CSV/NDJSON, and serves them over HTTP. A Vue frontend scaffold is in place for future visualization.

> **Agents**: read [`AGENTS.md`](./AGENTS.md) first. It is the source of truth for behavior, gotchas, and workflow. This README is for humans.

---

## Status (current truth)

- Backend: working. Collector + web server both run.
- Frontend: scaffold only. Default Vue template. No metrics UI yet.
- Tests: `cargo test` runs **17 tests in `ferrimon-burner`** (green). `common`, `collector`, `web` have **0** tests yet, so a green `cargo test` proves nothing about them.
- Per-core CPU stats and extra `/proc/stat` lines (`intr`, `ctxt`, `softirq`, etc.) are on the backlog — see [`doc/TODO.md`](./doc/TODO.md).
- Linux only. macOS / Windows are out of scope (`/proc/stat` required).

---

## Repository Layout

```
crates/common/      shared collection loop, CPU math, storage
crates/collector/   binary `ferrimon`        — runs the collection loop
crates/web/         binary `ferrimon-web`    — axum server for raw files
crates/burner/      lib + bin + criterion bench — Fibonacci/SIMD CPU burners (benchmark targets)
web-app/            Vue 3 frontend (source AND all its config files)
doc/                product notes (currently just TODO.md)
package.json        frontend scripts/deps at repo root
pnpm-lock.yaml      pnpm is the only supported package manager
rust-toolchain.toml channel = nightly (REQUIRED — see below)
```

---

## Requirements

- **Linux** with `/proc/stat`.
- **Rust nightly** — pinned in `rust-toolchain.toml`. Required because `crates/burner` uses `#![feature(portable_simd)]`. Stable will not compile.
- **Node.js** `^20.19.0 || >=22.12.0` (declared in `package.json` `engines`).
- **pnpm** for the frontend. Do not use `npm` or `yarn` — they will create a competing lockfile.

---

## Quickstart

### Build everything

```bash
cargo build
pnpm install
```

### Run the collector

```bash
# defaults: interval=100ms, format=ndjson, workdir=./data
cargo run --bin ferrimon -- --workdir ./data --interval-ms 100 --format ndjson

# write both CSV and NDJSON
cargo run --bin ferrimon -- --workdir ./data --format both
```

The collector creates `--workdir` if missing, then writes (and **appends to on restart**):

- `metrics.csv` — CSV with header
- `metrics.ndjson` — one JSON object per line, no header

Stop with `Ctrl-C` (`SIGINT`) or `SIGTERM`. Both are handled gracefully via `tokio_util::CancellationToken`.

### Run the web server

```bash
cargo run --bin ferrimon-web -- --workdir ./data --port 8080
```

Routes:

| Method | Path              | Returns                                                    |
|--------|-------------------|------------------------------------------------------------|
| GET    | `/`               | Inline HTML index with links to the two files              |
| GET    | `/metrics.csv`    | `text/csv` (Content-Disposition: attachment) or 404        |
| GET    | `/metrics.ndjson` | `application/x-ndjson` (Content-Disposition: attachment) or 404 |

The server has **no CORS headers** and **no graceful shutdown**. For local use only.

### Run the frontend (from repo root)

```bash
pnpm dev                # vite dev server
pnpm build              # type-check + vite build (parallel)
pnpm test:unit          # vitest
pnpm test:e2e:dev       # cypress against dev server
pnpm test:e2e           # cypress against production preview
pnpm lint               # oxlint then eslint (sequential)
pnpm format             # oxfmt (NOT prettier)
```

Frontend artifacts land in `target/web-app/dist`; ESLint cache in `target/web-app/eslintcache`.

### Benchmarks (burner crate)

```bash
cargo bench -p ferrimon-burner
# HTML reports: target/criterion/
```

Useful as CPU-bound load while exercising the collector against itself.

---

## Data Schema

Each sample is a `CpuMetrics` struct (`crates/common/src/types.rs`):

`timestamp` (RFC3339 UTC), then `u64` clock ticks for `user`, `nice`, `system`, `idle`, `iowait`, `irq`, `softirq`, `steal`, `guest`, `guest_nice`.

Usage math (`crates/common/src/cpu.rs`):

- `total = user + nice + system + idle + iowait + irq + softirq + steal` — `guest`/`guest_nice` are excluded because the kernel already counts them inside `user`/`nice`.
- `idle_for_usage = idle + iowait`. `steal` counts as used. Result is clamped to `[0, 100]`.

---

## Development Notes

- Smoke-test the collector with `timeout` rather than `&` + `kill`:
  ```bash
  timeout 2 cargo run --bin ferrimon -- --workdir /tmp/ferrimon-smoke --interval-ms 100 --format both
  ls /tmp/ferrimon-smoke && wc -l /tmp/ferrimon-smoke/metrics.ndjson
  ```
- Restarting the collector **appends** to existing metrics files. Delete the workdir for a clean run.
- Workspace dependencies are declared once in the root `Cargo.toml` and referenced by member crates via `.workspace = true` — keep that pattern.
- Formatting: `cargo fmt --all` for Rust, `oxfmt` (via `pnpm format`) for the frontend.

---

## Git Workflow

- Work on `feature/<short-description>` branches off `master`.
- Commit at task end. Never commit directly to `master`.
- Merge back with `--no-ff` and a meaningful merge summary (why + key changes).
- Squash WIP noise before merge.

---

## For AI Agents

Operating manual and behavioral guardrails: [`AGENTS.md`](./AGENTS.md).
Project-level OMO skills: [`.opencode/skills/`](./.opencode/skills/).
