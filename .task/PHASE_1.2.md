# Phase: 1.2 - Initialize Runtime Skeleton

## Objective
Build the first runnable `envlock` scaffold: CLI entry, JSON config loader, and injection runtime loop with lifecycle order `validate -> register -> export -> shutdown`.

## Exit Criteria
- [x] CLI accepts `-c <path>` and loads JSON config.
- [x] Injections are modeled as `node`, `kube`, `codex` with a common lifecycle contract.
- [x] Runtime executes lifecycle in deterministic order and prints exported env pairs.
- [x] Project compiles cleanly.

## Work Log
- [2026-02-26 10:23:30 +0800] @ZQXY123deMacBook-Pro.local: Phase initialized.
- [2026-02-26 10:23:30 +0800] Scope for this phase: scaffolding only, no real external integrations.
- [2026-02-26 10:24:30 +0800] Added dependencies: `anyhow`, `clap`, `serde`, `serde_json`.
- [2026-02-26 10:24:30 +0800] Implemented JSON config model and loader in `src/config.rs`.
- [2026-02-26 10:24:30 +0800] Implemented injection lifecycle executor with `node`, `kube`, `codex` in `src/injection.rs`.
- [2026-02-26 10:24:30 +0800] Wired CLI and export rendering in `src/main.rs`.
- [2026-02-26 10:24:45 +0800] Validation: `cargo fmt` + `cargo check` passed.
- [2026-02-26 10:24:58 +0800] Validation: `cargo run -- -c /tmp/envlock.sample.json` passed and printed expected exports.

## Technical Notes
- **Files Touched:** `Cargo.toml`, `Cargo.lock`, `src/main.rs`, `src/config.rs`, `src/injection.rs`
- **New Dependencies:** `anyhow`, `clap`, `serde`, `serde_json`
- **Blockers:** none

---
*Archived to .task/archive/ when closed or reverted.*
