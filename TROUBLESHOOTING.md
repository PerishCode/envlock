# Troubleshooting and Decision Chain

This file captures implementation traps and design reasons from bootstrap phases.

## Why These Decisions

- Output separation: logs must stay on `stderr` so shell export streams on `stdout` remain eval-safe.
- Injection scope: v1 is environment management only; command execution orchestration is intentionally out of scope.
- Output contract: `--output <shell|json>` is the single selector to avoid dual-flag drift.
- Extensibility: `symlink` replaced codex-specific logic and `env` replaced node/kube-specific env writers to keep runtime generic.
- Config discovery: `--use <profile>` defaults to `~/.envlock/configs/<profile>.json` and keeps `-c` as explicit override.

## Traps Encountered

- Mixing logs and exports can break `eval "$(envlock ...)"`; always keep lifecycle logs off `stdout`.
- Duplicate export keys hide bugs in composed profiles; use `--strict` to force explicit conflict handling.
- Symlink cleanup must only remove links created by the current lifecycle to avoid deleting user-managed files.
- Relative symlink paths are ambiguous unless resolved from the config file directory.

## Promotion Audit (Lighthouse)

- Decision chain promoted here from `.task` phase history.
- Reusable assets audit: no `.task/resources/` scripts were created, so no script promotion is needed.
- System mapping promoted to README (`Architecture Map` section).
