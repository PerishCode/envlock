# envlock

`envlock` reads a JSON profile and prints environment exports for your shell.

## Quick Start

Run with sample profile:

```bash
cargo run -- -p examples/envlock.sample.json
```

Apply variables to current shell:

```bash
eval "$(cargo run --quiet -- -p examples/envlock.sample.json)"
```

## CLI

```bash
envlock (-p <path-to-profile.json> | --use <profile>) [--output <shell|json>] [--strict]
```

- `-p, --profile`: JSON profile file path.
- `--use <profile>`: load profile from `ENVLOCK_PROFILE_HOME/profiles/<profile>.json`.
  If `ENVLOCK_PROFILE_HOME` is unset, default is `~/.envlock`.
- `--output <shell|json>`: choose output mode (`shell` by default).
- `--strict`: fail on duplicate exported keys.
- `--log-level <error|warn|info|debug|trace>`: set log verbosity (default: `warn`).
- `--log-format <text|json>`: set log format (default: `text`).

## Profile Format

```json
{
  "injections": [
    {
      "type": "env",
      "enabled": true,
      "vars": {
        "ENVLOCK_PROFILE": "dev",
        "ENVLOCK_NODE_VERSION": "22.11.0",
        "NPM_CONFIG_REGISTRY": "https://registry.npmjs.org",
        "KUBECONFIG_CONTEXT": "dev-cluster",
        "KUBECONFIG_NAMESPACE": "platform"
      }
    },
    {
      "type": "symlink",
      "enabled": false,
      "source": "~/.config/codex-agents.md",
      "target": "~/.codex/AGENTS.md",
      "on_exist": "error",
      "cleanup": true
    }
  ]
}
```

## Output

Default output is shell exports:

```bash
export ENVLOCK_NODE_VERSION='22.11.0'
export NPM_CONFIG_REGISTRY='https://registry.npmjs.org'
export KUBECONFIG_CONTEXT='dev-cluster'
export KUBECONFIG_NAMESPACE='platform'
```

JSON output:

```bash
cargo run --quiet -- -p examples/envlock.sample.json --output json
```
