# CLI Reference

## Command Forms

```bash
envlock (-p <path> | --use <name>) [--output <shell|json>] [--strict] [-- <cmd...>]
envlock preview --profile <path> [--output <text|json>]
envlock self-update [--check] [--version <x.y.z|vX.Y.Z>] [-y|--yes]
```

## Run Command Options

| Option | Description |
| --- | --- |
| `-p, --profile <path>` | Explicit JSON profile path. |
| `--use <name>` | Named profile under `ENVLOCK_PROFILE_HOME/profiles`. |
| `--output <shell|json>` | Output mode, default `shell`. |
| `--strict` | Fail on duplicate keys in final output. |
| `--log-level <error|warn|info|debug|trace>` | Logging level, default `warn`. |
| `--log-format <text|json>` | Logging format, default `text`. |
| `-- <cmd...>` | Run child command with injected env and return child exit code. |

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

## Exit Behavior

- Shell/JSON output mode: exits `0` on success.
- Command mode: exits with child exit code.
- Validation/parsing failures: non-zero exit.
