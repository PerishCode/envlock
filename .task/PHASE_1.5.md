# Phase: 1.5 - JSON Output Mode

## Objective
Add a machine-readable output mode (`--json`) while preserving existing shell export output as default.

## Exit Criteria
- [x] CLI supports `--json` flag.
- [x] Output switches to JSON object when `--json` is enabled.
- [x] Unit tests cover both shell and JSON render paths.
- [x] `cargo test` passes.

## Work Log
- [2026-02-26 10:47:30 +0800] @ZQXY123deMacBook-Pro.local: Phase initialized.
- [2026-02-26 10:47:52 +0800] Added `--json` CLI flag and unified output path in `src/main.rs`.
- [2026-02-26 10:47:52 +0800] Added `main.rs` tests for shell escaping and env map merge behavior.
- [2026-02-26 10:48:02 +0800] Updated `README.md` with JSON mode usage.
- [2026-02-26 10:48:10 +0800] Validation: `cargo fmt`, `cargo test`, and both runtime modes passed.

## Technical Notes
- **Files Touched:** `src/main.rs`, `README.md`, `.task/MAIN.md`, `.task/PHASE_1.5.md`
- **New Dependencies:** none
- **Blockers:** none

---
*Archived to .task/archive/ when closed or reverted.*
