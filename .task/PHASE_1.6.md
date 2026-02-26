# Phase: 1.6 - Strict Duplicate-Key Validation

## Objective
Add a strict mode that fails when multiple injections export the same environment key, preventing silent overrides.

## Exit Criteria
- [x] CLI supports `--strict`.
- [x] Duplicate export keys return an error in strict mode.
- [x] Existing non-strict behavior remains backward-compatible.
- [x] Tests and docs are updated; `cargo test` passes.

## Work Log
- [2026-02-26 10:53:59 +0800] @ZQXY123deMacBook-Pro.local: Phase initialized.
- [2026-02-26 10:54:25 +0800] Added `--strict` CLI flag and strict duplicate-key detection in `src/main.rs`.
- [2026-02-26 10:54:25 +0800] Added tests for non-strict overwrite and strict duplicate rejection.
- [2026-02-26 10:54:30 +0800] Updated `README.md` with `--strict` behavior notes.
- [2026-02-26 10:54:42 +0800] Validation: `cargo fmt`, `cargo test`, and runtime checks with `--strict` passed.
- [2026-02-26 10:59:41 +0800] Created handoff checkpoint commit `c080a23`.

## Technical Notes
- **Files Touched:** `src/main.rs`, `README.md`, `.task/MAIN.md`, `.task/PHASE_1.6.md`
- **New Dependencies:** none
- **Blockers:** none

---
*Archived to .task/archive/ when closed or reverted.*
