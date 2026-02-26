# Phase: 1.1 - Define UX optimization scope and baseline

## Objective
Translate core goals into an executable implementation plan: stabilize versioning and package release policy, prepare Homebrew onboarding path, and define a built-in update command UX and acceptance checks.

## Exit Criteria
- [x] Version management and release/publish strategy is documented with clear conventions and constraints.
- [x] Homebrew integration approach is selected (tap/release artifact flow) with concrete implementation tasks.
- [x] Built-in update command requirements are specified, including expected behavior and safety boundaries.

## Work Log
- [2026-02-26 15:22:28 +0800] @ZQXY123deMacBook-Pro.local: Phase initialized.
- [2026-02-26 15:22:28 +0800] Captured user core goals:
  - solidify version management and package publishing strategy
  - integrate Homebrew distribution
  - add built-in update command
- [2026-02-26 15:33:17 +0800] Documented version/release strategy in `README.md`: CI on PR/main, release on `v*` tags, and maintainer release steps.
- [2026-02-26 15:33:17 +0800] Landed implementation baseline:
  - Added `envlock self-update` command in CLI and `src/self_update.rs`.
  - Added release integrity checks (asset checksum verification) and atomic binary replacement workflow.
  - Added GitHub Actions workflows for CI and release artifact publishing.
- [2026-02-26 15:33:17 +0800] Verified local checks: `cargo fmt --check` and `cargo test -q` passed.
- [2026-02-26 15:33:17 +0800] Phase complete; follow-up Homebrew tap initialization moved to Phase 1.2.

## Technical Notes
- **Files Touched:** `.task/MAIN.md`, `.task/PHASE_1.1.md`, `.github/workflows/ci.yml`, `.github/workflows/release.yml`, `src/bin/envlock.rs`, `src/self_update.rs`, `src/lib.rs`, `Cargo.toml`, `README.md`
- **New Dependencies:** `reqwest`, `semver`, `sha2`, `flate2`, `tar`, `tempfile` (runtime)
- **Blockers:** none

---
*Archived to .task/archive/ when closed or reverted.*
