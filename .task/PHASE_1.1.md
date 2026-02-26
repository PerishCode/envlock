# Phase: 1.1 - Bootstrap kubectl effect validation

## Objective
Define practical local kubectl verification scenarios and execute a first baseline validation pass through envlock profile usage.

## Exit Criteria
- [x] kubectl validation scope and command checklist are documented for this task.
- [x] At least one baseline kubectl read-only verification run is executed with captured results.

## Work Log
- [2026-02-26 14:39:51 +0800] @ZQXY123deMacBook-Pro.local: Phase initialized.
- [2026-02-26 14:44:31 +0800] Exported two minimal kubeconfig files from `~/.kube/config` to `~/.envlock/resources/kubeconfig/xx.yaml` and `~/.envlock/resources/kubeconfig/yy.yaml`.
- [2026-02-26 14:44:31 +0800] Created `~/.envlock/profiles/kube.json` with `type=env` and `KUBECONFIG=xx.yaml:yy.yaml`.
- [2026-02-26 14:44:31 +0800] Verified isolation: baseline `kubectl config current-context` is `.../refly-test-eks`, while `envlock --use kube -- kubectl config current-context` is `.../refly-prod-eks`.
- [2026-02-26 14:44:31 +0800] Confirmed child process receives injected `KUBECONFIG` value under `~/.envlock/resources/kubeconfig/*`.
- [2026-02-26 14:50:05 +0800] Added `resource://` prefix resolution in `env` injection values (including `ops` values), with default base `~/.envlock/resources` and optional `ENVLOCK_RESOURCE_HOME` override.
- [2026-02-26 14:50:05 +0800] Updated `~/.envlock/profiles/kube.json` to `resource://kubeconfig/xx.yaml:resource://kubeconfig/yy.yaml` and verified runtime resolution plus kubectl context isolation still works.
- [2026-02-26 14:51:38 +0800] Created composite profile `~/.envlock/profiles/node-kube-combo.json` by combining node (`type=command`, fnm env) and kube (`type=env`, KUBECONFIG resource URIs) behaviors.
- [2026-02-26 14:51:38 +0800] Executed temporary shell script via `envlock --use node-kube-combo -- /tmp/envlock_combo_verify.sh` and passed assertions for node/npm/pnpm versions, resolved KUBECONFIG, and isolated kubectl current-context (`.../refly-prod-eks`).
- [2026-02-26 14:57:56 +0800] Refactored startup configuration flow by introducing `RuntimeConfig` in `src/config.rs`, unifying CLI args + env defaults (`ENVLOCK_PROFILE_HOME`, `ENVLOCK_RESOURCE_HOME`) at process start.
- [2026-02-26 14:57:56 +0800] Updated runtime pipeline to consume explicit config context (including `resource_home`) instead of direct env reads in injection execution path.
- [2026-02-26 14:57:56 +0800] Verified no regressions with `cargo fmt && cargo test -q` (all tests passing).
- [2026-02-26 15:04:03 +0800] Reworked runtime architecture to `AppContext` trait pattern: added `src/app.rs` and switched run/injection flow to read config and process services through app context.
- [2026-02-26 15:04:03 +0800] Updated `command` and `env` injections to consume app-provided environment and command execution dependencies, eliminating direct runtime coupling to global process env within injection logic.
- [2026-02-26 15:04:03 +0800] Verified post-refactor stability via `cargo fmt && cargo test -q` (all tests passing).
- [2026-02-26 15:08:03 +0800] Session handoff checkpoint prepared with updated `.task` context and pending WIP commit.

## Technical Notes
- **Files Touched:** `.task/MAIN.md`, `.task/PHASE_1.1.md`, `~/.envlock/resources/kubeconfig/xx.yaml`, `~/.envlock/resources/kubeconfig/yy.yaml`, `~/.envlock/profiles/kube.json`, `~/.envlock/profiles/node-kube-combo.json`, `src/app.rs`, `src/config.rs`, `src/bin/envlock.rs`, `src/lib.rs`, `src/injections/mod.rs`, `src/injections/env.rs`, `src/injections/command.rs`, `README.md`
- **New Dependencies:** none
- **Blockers:** none

---
*Archived to .task/archive/ when closed or reverted.*
