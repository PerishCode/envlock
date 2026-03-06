# CLI Reference

## Command Forms

```bash
envlock [--profile <path>] [--output <shell|json>] [--strict] [-- <cmd...>]
envlock preview --profile <path> [--output <text|json>]
envlock self-update [--check] [--version <x.y.z|vX.Y.Z>] [-y|--yes]
envlock profiles status
envlock profiles init --type <minimal|sample> [--name <name>] [--force]
envlock alias list
envlock alias append <name> --profile <path>
envlock <alias> [-- <cmd...>]
```

## Run Command Options

| Option | Description |
| --- | --- |
| `-p, --profile <path>` | Explicit JSON profile path. |
| `--output <shell|json>` | Output mode, default `shell`. |
| `--strict` | Fail on duplicate keys in final output. |
| `--log-level <error|warn|info|debug|trace>` | Logging level, default `warn`. |
| `--log-format <text|json>` | Logging format, default `text`. |
| `-- <cmd...>` | Run child command with injected env and return child exit code. |

When `--profile` is omitted, envlock resolves:

- `$ENVLOCK_HOME/profiles/default.json` if `ENVLOCK_HOME` is set.
- `~/.envlock/profiles/default.json` otherwise.

## `self-update` Options

| Option | Description |
| --- | --- |
| `--check` | Check availability only; no install. |
| `--version <x.y.z|vX.Y.Z>` | Install exact release version. |
| `-y, --yes` | Skip confirmation prompt. |

## `preview` Options

| Option | Description |
| --- | --- |
| `-p, --profile <path>` | Explicit JSON profile path to inspect. |
| `--output <text|json>` | Preview format, default `text`. |

`preview` is read-only and does not execute injections. It exposes metadata only:

- `env`: key names only.
- `command`: program and argument count only.
- `symlink`: path metadata only.

## `profiles` Commands

- `profiles status`: show `$ENVLOCK_HOME/profiles` health, default profile presence, and JSON parse status.
- `profiles init --type <minimal|sample>`: create a starter profile at `$ENVLOCK_HOME/profiles/default.json`.
- `profiles init --name <name>`: write to `$ENVLOCK_HOME/profiles/<name>.json`.
- `profiles init --force`: overwrite existing target file.

## `alias` Commands

- `alias list`: show alias to profile mappings from `$ENVLOCK_HOME/aliases.json`.
- `alias append <name> --profile <path>`: append one alias mapping (fails on duplicate name).
- `envlock <alias>`: fallback to alias profile when `<alias>` is not a built-in command.

## Exit Behavior

- Shell/JSON output mode: exits `0` on success.
- Command mode: exits with child exit code.
- Validation/parsing failures: non-zero exit.
