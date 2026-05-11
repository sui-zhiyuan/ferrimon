# Ferrimon

Ferrimon is a Linux benchmark monitor with a Rust collector/web backend and a Vue frontend workspace.

## Current Scope

- Collects aggregate CPU usage from `/proc/stat` on a fixed interval.
- Writes metrics as CSV and/or NDJSON in a work directory.
- Serves raw metrics files over HTTP from the Rust web server.
- Includes a Vue app scaffold in `web-app/` for frontend integration.

## Repository Layout

- `crates/common` - shared collection loop, CPU math, storage logic.
- `crates/collector` - collector binary package (`ferrimon`).
- `crates/web` - web server binary package (`ferrimon-web`).
- `web-app` - Vue + Vite frontend source (config files stay in this directory).
- `package.json` / `pnpm-lock.yaml` at repo root - frontend scripts and dependencies.

## Rust Commands

```bash
cargo build
cargo build --release
cargo fmt --all
cargo clippy --all
cargo test
```

Run collector:

```bash
cargo run --bin ferrimon -- --workdir ./data --interval-ms 100 --format ndjson
cargo run --bin ferrimon -- --workdir ./data --format both
```

Run web server:

```bash
cargo run --bin ferrimon-web -- --workdir ./data --port 8080
```

## Frontend Commands

Run from repository root:

```bash
pnpm dev
pnpm build
pnpm test:unit
pnpm test:e2e:dev
pnpm test:e2e
pnpm lint
pnpm format
```

Notes:

- Vite config and Vitest config live in `web-app/` and are referenced via script flags.
- Frontend production build output is `target/web-app/dist`.
- ESLint cache is written to `target/web-app/eslintcache`.

## Data Output

Collector writes into `--workdir` (default `./data`):

- `metrics.csv`
- `metrics.ndjson`

Web routes in `crates/web/src/main.rs`:

- `GET /`
- `GET /metrics.csv`
- `GET /metrics.ndjson`

## Requirements

- Linux (`/proc/stat` required).
- Rust `1.87+`.
- Node.js `^20.19.0 || >=22.12.0` for frontend tooling.

## Development Notes

- Use `timeout` for collector smoke tests (avoid background `&` + manual kill).
- Project workflow and guardrails are documented in `AGENTS.md`.
