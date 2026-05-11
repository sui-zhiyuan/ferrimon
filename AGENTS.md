# AGENTS.md

Linux-only Rust workspace for short benchmark monitoring (`/proc/stat` required). Keep this file compact and factual.

## High-Signal Facts

- Toolchain: Rust edition `2024`, workspace resolver `2`.
- Requires Rust `1.87+` (`is_multiple_of` is used in `crates/common/src/collector_loop.rs`).
- Package manager is **pnpm** (not npm/yarn); lockfile is `pnpm-lock.yaml`.
- Node.js `^20.19.0 || >=22.12.0` required for frontend tooling.
- `cargo test` builds crates but runs 0 tests (no `#[test]` functions exist).
- Frontend package manifest is at repo root (`package.json`), while frontend config/source stays in `web-app/`.
- Frontend is still scaffold (default Vue template, no metrics UI yet).
- `data/` is gitignored — collector output is not committed.

## Workspace Entrypoints

- `crates/collector` (`ferrimon-collector`) builds binary `ferrimon`.
- `crates/web` (`ferrimon-web`) builds binary `ferrimon-web`.
- `crates/common` contains the real collection loop and shared logic.
- Collection scheduling logic is in `crates/common/src/collector_loop.rs`, not in collector `main.rs`.

## Commands You'll Actually Use

```bash
cargo build
cargo build --release
cargo fmt --all
cargo clippy --all
cargo test

cargo run --bin ferrimon -- --workdir ./data --interval-ms 100 --format ndjson
cargo run --bin ferrimon -- --workdir ./data --format both
cargo run --bin ferrimon-web -- --workdir ./data --port 8080

pnpm dev
pnpm build         # runs vue-tsc type-check + vite build in parallel (run-p)
pnpm type-check    # vue-tsc --build web-app/tsconfig.json
pnpm test:unit     # vitest --config web-app/vitest.config.ts
pnpm lint          # oxlint then eslint, sequential (run-s)
pnpm format        # oxfmt (not prettier)
```

## Frontend Layout/Tooling Notes

- Vue app source and config are under `web-app/`.
- Vite config: `web-app/vite.config.ts`; Vitest config: `web-app/vitest.config.ts`; Cypress config: `web-app/cypress.config.ts`.
- Frontend build output goes to `target/web-app/dist`.
- ESLint cache is configured at `target/web-app/eslintcache`.
- Formatter is `oxfmt` (`web-app/.oxfmtrc.json`), linter is `oxlint` (`web-app/.oxlintrc.json`) + eslint (`web-app/eslint.config.ts`).

## Behavior/Implementation Gotchas

- Collector reads only aggregate `cpu` line from `/proc/stat` (not per-core yet).
- CPU usage math in `crates/common/src/cpu.rs` treats `guest`/`guest_nice` as sub-counts (excluded from total) and counts `steal` as used.
- `MetricsWriter` opens files in **append mode** — restarting the collector continues appending to existing `metrics.csv`/`metrics.ndjson` without clearing old data.
- Web routes are manually implemented in `crates/web/src/main.rs`: `GET /`, `GET /metrics.csv`, `GET /metrics.ndjson`.
- `tower-http` is present with `fs` feature, but static serving uses manual handlers (no `ServeDir`).
- Web server has no CORS headers — frontend dev server proxy or same-origin required for API access.
- Web server has no graceful shutdown path; collector handles `SIGINT`/`SIGTERM` via `CancellationToken`.

## Smoke Test Rule

- Use `timeout` for short collector runs; do not use background `&` + `kill $PID` in this environment.

## Git Workflow In This Repo

- Start work on `feature/<description>`.
- Commit at task end on the feature branch (create one first if currently on `master`).
- Merge with `--no-ff` and write a meaningful merge summary message (why + key changes), not just branch name.

## Style Convention

- Add comments only when needed to explain non-obvious logic.