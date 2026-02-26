# Phase: 1.2 - Homebrew tap initialization planning

## Objective
Finalize the tap repository initialization and integration handoff between this repo release workflow and `homebrew-tap` formula update automation.

## Exit Criteria
- [ ] Tap repository bootstrap checklist is documented (repository layout, formula naming, tokens/permissions).
- [x] Release-to-tap update contract is documented (artifact naming, checksum source, update trigger).
- [x] Follow-up implementation tasks are confirmed with user.

## Work Log
- [2026-02-26 15:33:17 +0800] @ZQXY123deMacBook-Pro.local: Phase initialized after main repository baseline changes completed.
- [2026-02-26 15:34:32 +0800] Corrected phase split: moved Phase 1.2 content into dedicated `.task/PHASE_1.2.md`.
- [2026-02-26 15:40:48 +0800] Implemented extensible release contract:
  - `TOOLS`-driven multi-binary packaging in `.github/workflows/release.yml`
  - deterministic artifact naming: `${tool}-${version}-${target}.tar.gz`
  - release-level tap metadata export: `dist/tap/${tool}.env`
- [2026-02-26 15:40:48 +0800] Added reusable tap formula generator script: `scripts/update-tap-formula.sh`.
- [2026-02-26 15:40:48 +0800] Validation completed:
  - `cargo fmt --check`
  - `cargo test -q`
  - `bash -n scripts/update-tap-formula.sh`
  - sample formula generation run
- [2026-02-26 15:44:45 +0800] Added tap sync automation script: `scripts/sync-homebrew-tap.sh` (metadata -> formula update).
- [2026-02-26 15:44:45 +0800] Extended `release.yml` to auto-sync tap repository after release when `HOMEBREW_TAP_TOKEN` is set.
- [2026-02-26 15:44:45 +0800] Validation completed:
  - `bash -n scripts/sync-homebrew-tap.sh`
  - end-to-end local metadata-to-formula sync simulation
  - `cargo fmt --check`
  - `cargo test -q`

## Technical Notes
- **Files Touched:** `.task/MAIN.md`, `.task/PHASE_1.2.md`, `.github/workflows/release.yml`, `scripts/update-tap-formula.sh`, `scripts/sync-homebrew-tap.sh`, `README.md`
- **New Dependencies:** none
- **Blockers:** none

---
*Archived to .task/archive/ when closed or reverted.*
