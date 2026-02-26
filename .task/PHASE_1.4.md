# Phase: 1.4 - README and Usage Docs

## Objective
Create baseline repository documentation so users can run `envlock`, understand the JSON format, and know current v1 limitations.

## Exit Criteria
- [x] Add `README.md` with quick start and usage command.
- [x] Document `injections` schema for `node`, `kube`, `codex`.
- [x] Align docs with actual CLI behavior via a verification run.

## Work Log
- [2026-02-26 10:34:59 +0800] @ZQXY123deMacBook-Pro.local: Phase initialized.
- [2026-02-26 10:35:20 +0800] Added `README.md` with quick start, CLI usage, config schema, and output contract.
- [2026-02-26 10:35:28 +0800] Validation: `cargo run --quiet -- -c examples/envlock.sample.json` output matched docs.

## Technical Notes
- **Files Touched:** `README.md`, `.task/MAIN.md`, `.task/PHASE_1.4.md`
- **New Dependencies:** none
- **Blockers:** none

---
*Archived to .task/archive/ when closed or reverted.*
