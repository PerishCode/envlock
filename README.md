# envlock

`envlock` reads a JSON profile and prepares environment variables for your shell or child command.

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
envlock (-p <path-to-profile.json> | --use <profile>) [--output <shell|json>] [--strict] [-- <cmd...>]
```

- `-p, --profile`: JSON profile file path.
- `--use <profile>`: load profile from `ENVLOCK_PROFILE_HOME/profiles/<profile>.json`.
  If `ENVLOCK_PROFILE_HOME` is unset, default is `~/.envlock`.
- `--output <shell|json>`: choose output mode (`shell` by default).
- `--strict`: fail on duplicate exported keys.
- `-- <cmd...>`: run a command with injected env in-process, and return the child exit code.
- `--log-level <error|warn|info|debug|trace>`: set log verbosity (default: `warn`).
- `--log-format <text|json>`: set log format (default: `text`).

`env` injections support `ops` for non-destructive env composition:
- `set`
- `set_if_absent`
- `prepend`
- `append`
- `unset`

`env` values also support `resource://` URI expansion:
- `resource://...` resolves against `ENVLOCK_RESOURCE_HOME`
- default resource home is `~/.envlock/resources` when `ENVLOCK_RESOURCE_HOME` is unset

Boundary:
- Use `env` for static values and simple composition.
- Use `command` for dynamic environment bootstrapping (for example `fnm env --shell bash`).

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
      },
      "ops": [
        {
          "op": "prepend",
          "key": "PATH",
          "value": "~/.local/bin",
          "separator": "os",
          "dedup": true
        },
        {
          "op": "set_if_absent",
          "key": "NPM_CONFIG_REGISTRY",
          "value": "https://registry.npmjs.org"
        },
        {
          "op": "set",
          "key": "KUBECONFIG",
          "value": "resource://kubeconfig/xx.yaml:resource://kubeconfig/yy.yaml"
        }
      ]
    },
    {
      "type": "command",
      "enabled": false,
      "program": "fnm",
      "args": ["env", "--shell", "bash"]
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
