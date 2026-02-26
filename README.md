# envlock

`envlock` is a config-driven environment session builder.

In v1, it reads a JSON config file and prints shell `export` statements from configured injections.
It also supports machine-readable JSON output with `--output json`.

## Quick Start

1. Build or run directly:

```bash
cargo run -- -c examples/envlock.sample.json
```

2. Apply exports in your shell:

```bash
eval "$(cargo run --quiet -- -c examples/envlock.sample.json)"
```

## CLI

```bash
envlock (-c <path-to-config.json> | --use <profile>)
```

- `-c, --config`: JSON config path.
- `--use <profile>`: Load config from `ENVLOCK_CONFIG_HOME/configs/<profile>.json`.
  - If `ENVLOCK_CONFIG_HOME` is not set, default is `~/.envlock`.
  - If `-c` and `--use` are both provided, `-c` takes priority.
- `--output <shell|json>`: Select output mode (default: `shell`).
- `--strict`: Fail if two injections export the same key.
- `--log-level <error|warn|info|debug|trace>`: Set log verbosity (default: `warn`).
- `--log-format <text|json>`: Set log output format (default: `text`).

Logging is emitted to `stderr` only, while env outputs remain on `stdout`.

## Config Shape

Top-level JSON:

```json
{
  "injections": [
    { "type": "env", "enabled": true, "vars": { "ENVLOCK_PROFILE": "dev", "ENVLOCK_NODE_VERSION": "22.11.0", "NPM_CONFIG_REGISTRY": "https://registry.npmjs.org", "KUBECONFIG_CONTEXT": "dev-cluster", "KUBECONFIG_NAMESPACE": "platform" } },
    { "type": "symlink", "enabled": false, "source": "~/.config/codex-agents.md", "target": "~/.codex/AGENTS.md", "on_exist": "error", "cleanup": true }
  ]
}
```

Supported injection types:

- `env`
- `symlink`

Notes:

- `enabled` defaults to `true`.
- Empty keys in `env.vars` fail validation.
- Empty string values for `symlink.source`, `symlink.target` fail validation.
- Path parsing strategy for `symlink.source` and `symlink.target`:
  - `~/...` expands to `HOME`
  - absolute paths are used as-is
  - relative paths are resolved from config file directory
- Runtime order is fixed: `validate -> register -> export -> shutdown`.
- By default, duplicate export keys are resolved with last-write-wins; with `--strict`, duplicates are rejected.
- `symlink` register behavior:
  - creates `target -> source`
  - `on_exist: "error"` fails if target exists
  - `on_exist: "replace"` replaces existing non-directory target
  - if `cleanup: true`, shutdown removes symlink created by this injection

## Output Contract

Current export keys:

- `env.vars.<KEY>` -> `<KEY>`

Common profile keys (enumerable convention):

- `ENVLOCK_NODE_VERSION`
- `NPM_CONFIG_REGISTRY`
- `KUBECONFIG_CONTEXT`
- `KUBECONFIG_NAMESPACE`

The output is shell-ready:

```bash
export ENVLOCK_NODE_VERSION='22.11.0'
export KUBECONFIG_CONTEXT='dev-cluster'
...
```

JSON mode:

```bash
cargo run --quiet -- -c examples/envlock.sample.json --output json
```

## Current Scope (v1)

- Environment management only.
- No command execution orchestration.
- `symlink` injection manages generic symlink lifecycle.

## Architecture Map

- CLI entry: `src/bin/envlock.rs`
- Runtime library: `src/lib.rs`
- Config schema/parser: `src/config.rs`
- Injection implementations: `src/injections/env.rs`, `src/injections/symlink.rs`
- Integration tests: `tests/`
- Example config: `examples/envlock.sample.json`

See `TROUBLESHOOTING.md` for decision-chain notes and common traps observed during bootstrap.

## Local Quality Checks

```bash
cargo fmt --check
cargo lint
cargo test
```
