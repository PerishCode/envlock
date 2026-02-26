# Phase: 1.3 - Sample Config and Baseline Tests

## Objective
Improve usability and confidence by adding a reusable JSON sample and baseline unit tests for config parsing and lifecycle behavior.

## Exit Criteria
- [x] Add a sample JSON config file in repository.
- [x] Add tests for JSON parsing and core lifecycle behavior.
- [x] `cargo test` passes.

## Work Log
- [2026-02-26 10:28:35 +0800] @ZQXY123deMacBook-Pro.local: Phase initialized.
- [2026-02-26 10:29:00 +0800] Added `examples/envlock.sample.json`.
- [2026-02-26 10:29:10 +0800] Added config parser tests in `src/config.rs`.
- [2026-02-26 10:29:15 +0800] Added lifecycle tests in `src/injection.rs`.
- [2026-02-26 10:29:22 +0800] Validation: `cargo fmt` + `cargo test` passed (4 tests).

## Technical Notes
- **Files Touched:** `examples/envlock.sample.json`, `src/config.rs`, `src/injection.rs`, `.task/MAIN.md`, `.task/PHASE_1.3.md`
- **New Dependencies:** none
- **Blockers:** none

---
*Archived to .task/archive/ when closed or reverted.*
