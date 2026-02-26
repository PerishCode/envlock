# envlock

`envlock` is a config-driven environment session builder.

In v1, it reads a JSON config file and prints shell `export` statements from configured injections.
It also supports machine-readable JSON output with `--json`.

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
envlock -c <path-to-config.json>
```

- `-c, --config`: JSON config path.
- `--json`: Print merged environment values as a JSON object instead of shell `export` lines.
- `--strict`: Fail if two injections export the same key.

## Config Shape

Top-level JSON:

```json
{
  "injections": [
    { "type": "node", "enabled": true, "version": "22.11.0", "npm_registry": "https://registry.npmjs.org" },
    { "type": "kube", "enabled": true, "context": "dev-cluster", "namespace": "platform" },
    { "type": "codex", "enabled": true }
  ]
}
```

Supported injection types:

- `node`
- `kube`
- `codex` (placeholder/no-op in v1)

Notes:

- `enabled` defaults to `true`.
- Empty string values for `node.version`, `node.npm_registry`, `kube.context`, `kube.namespace` fail validation.
- Runtime order is fixed: `validate -> register -> export -> shutdown`.
- By default, duplicate export keys are resolved with last-write-wins; with `--strict`, duplicates are rejected.

## Output Contract

Current export keys:

- `node.version` -> `ENVLOCK_NODE_VERSION`
- `node.npm_registry` -> `NPM_CONFIG_REGISTRY`
- `kube.context` -> `KUBECONFIG_CONTEXT`
- `kube.namespace` -> `KUBECONFIG_NAMESPACE`

The output is shell-ready:

```bash
export ENVLOCK_NODE_VERSION='22.11.0'
export KUBECONFIG_CONTEXT='dev-cluster'
...
```

JSON mode:

```bash
cargo run --quiet -- -c examples/envlock.sample.json --json
```

## Current Scope (v1)

- Environment management only.
- No command execution orchestration.
- `codex` injection is a no-op placeholder.
