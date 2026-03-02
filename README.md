# envlock

`envlock` reads a JSON profile and prepares environment variables for your shell or child command.

## Install

Install latest release:

```bash
curl -fsSL https://raw.githubusercontent.com/PerishCode/envlock/main/scripts/install.sh | sh
```

Install layout:

- Binary: `~/.envlock/bin/envlock`
- Symlink: `~/.local/bin/envlock`

## Quick Start (Source)

Run with sample profile from source:

```bash
cargo run -- -p examples/envlock.sample.json
```

Apply variables to current shell:

```bash
eval "$(cargo run --quiet -- -p examples/envlock.sample.json)"
```

## Linux Container Smoke

Use one-shot Linux container smoke checks (low-cost robustness validation):

```bash
scripts/e2e-smoke.sh smoke
```

## CLI

```bash
envlock [--profile <path-to-profile.json>] [--output <shell|json>] [--strict] [-- <cmd...>]
envlock preview --profile <path-to-profile.json> [--output <text|json>]
envlock self-update [--check] [--version <x.y.z|vX.Y.Z>] [-y|--yes]
```

- `-p, --profile`: JSON profile file path.
- default profile resolution (when `--profile` is omitted):
  - `ENVLOCK_HOME/profiles/default.json` if `ENVLOCK_HOME` is set
  - `~/.envlock/profiles/default.json` otherwise
- `--output <shell|json>`: choose output mode (`shell` by default).
- `--strict`: fail on duplicate exported keys.
- `-- <cmd...>`: run a command with injected env in-process, and return the child exit code.
- `--log-level <error|warn|info|debug|trace>`: set log verbosity (default: `warn`).
- `--log-format <text|json>`: set log format (default: `text`).
- `preview`: read-only profile inspection without executing injections.
  - `--profile`: profile file to inspect.
  - `--output <text|json>`: preview rendering mode (`text` by default).
  - security boundary: preview only exposes metadata (for example env keys, command arg count), not sensitive values.
- `self-update`: built-in updater for GitHub Release binaries.
  - `--check`: only check whether an update is available.
  - `--version`: upgrade to a specific release tag/version.
  - `-y, --yes`: non-interactive confirmation.

`env` injections support `ops` for non-destructive env composition:
- `set`
- `set_if_absent`
- `prepend`
- `append`
- `unset`

`env` values also support `resource://` URI expansion:
- `resource://...` resolves against `ENVLOCK_RESOURCE_HOME`
- `resource-content://...` reads file content from `ENVLOCK_RESOURCE_HOME` and injects it as the variable value
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

## Update

Check for updates:

```bash
envlock self-update --check
```

Upgrade interactively:

```bash
envlock self-update
```

Upgrade without prompt:

```bash
envlock self-update --yes
```

Uninstall:

```bash
curl -fsSL https://raw.githubusercontent.com/PerishCode/envlock/main/scripts/uninstall.sh | sh
```

## Docs

Full documentation (VitePress + GitHub Pages):

- https://perishcode.github.io/envlock/

## Release Process (Maintainers)

- `CI` workflow (`.github/workflows/ci.yml`) runs on pull requests and pushes to `main`.
- `Release` workflow (`.github/workflows/release.yml`) runs on tag push `v*`.
- Multi-binary packaging is controlled by `TOOLS` in `release.yml`.
  Add another binary name to `TOOLS` to include it in release artifacts.
- Release workflow builds archives for:
  - `x86_64-unknown-linux-gnu`
  - `x86_64-apple-darwin`
  - `aarch64-apple-darwin`
- Artifacts and `checksums.txt` are published to GitHub Release.

Typical release steps:

1. Merge to `main` after CI passes.
2. Bump `Cargo.toml` version to `X.Y.Z` and commit.
3. Create and push matching tag: `git tag vX.Y.Z && git push origin vX.Y.Z`.
4. Wait for `Release` workflow to publish artifacts.

Release guardrails:

- `release.yml` validates `github.ref_name == v$(Cargo.toml version)` and fails on mismatch.
