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
envlock self-update [--check] [--version <x.y.z|vX.Y.Z>] [-y|--yes]
```

- `-p, --profile`: JSON profile file path.
- `--use <profile>`: load profile from `ENVLOCK_PROFILE_HOME/profiles/<profile>.json`.
  If `ENVLOCK_PROFILE_HOME` is unset, default is `~/.envlock`.
- `--output <shell|json>`: choose output mode (`shell` by default).
- `--strict`: fail on duplicate exported keys.
- `-- <cmd...>`: run a command with injected env in-process, and return the child exit code.
- `--log-level <error|warn|info|debug|trace>`: set log verbosity (default: `warn`).
- `--log-format <text|json>`: set log format (default: `text`).
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

If envlock is installed via Homebrew, use:

```bash
brew upgrade envlock
```

## Release Process (Maintainers)

- `CI` workflow (`.github/workflows/ci.yml`) runs on pull requests and pushes to `main`.
- `Release` workflow (`.github/workflows/release.yml`) runs on tag push `v*`.
- Multi-binary packaging is controlled by `TOOLS` in `release.yml`.
  Add another binary name to `TOOLS` to include it in release artifacts.
- Homebrew tap target repository is configured by `HOMEBREW_TAP_REPO` in `release.yml`.
- Release workflow builds archives for:
  - `x86_64-unknown-linux-gnu`
  - `x86_64-apple-darwin`
  - `aarch64-apple-darwin`
- Artifacts, `checksums.txt`, and `dist/tap/*.env` metadata are published to GitHub Release.
- If repository secret `HOMEBREW_TAP_TOKEN` is configured, release workflow also syncs formulas into the tap repository automatically.

Typical release steps:

1. Merge to `main` after CI passes.
2. Create and push tag: `git tag vX.Y.Z && git push origin vX.Y.Z`.
3. Wait for `Release` workflow to publish artifacts.

Required secret for tap sync:

- `HOMEBREW_TAP_TOKEN`: Fine-grained PAT with `Contents: Read and Write` on `PerishCode/homebrew-tap`.

## Tap Formula Script

Use `scripts/update-tap-formula.sh` to generate/update a Homebrew formula in a tap repository:

```bash
scripts/update-tap-formula.sh \
  --formula /path/to/homebrew-tap/Formula/envlock.rb \
  --tool envlock \
  --desc "Build environment sessions from JSON profile" \
  --homepage "https://github.com/PerishCode/envlock" \
  --version v0.1.0 \
  --macos-arm-url "https://github.com/PerishCode/envlock/releases/download/v0.1.0/envlock-v0.1.0-aarch64-apple-darwin.tar.gz" \
  --macos-arm-sha256 "<sha256>" \
  --macos-amd-url "https://github.com/PerishCode/envlock/releases/download/v0.1.0/envlock-v0.1.0-x86_64-apple-darwin.tar.gz" \
  --macos-amd-sha256 "<sha256>" \
  --linux-amd-url "https://github.com/PerishCode/envlock/releases/download/v0.1.0/envlock-v0.1.0-x86_64-unknown-linux-gnu.tar.gz" \
  --linux-amd-sha256 "<sha256>"
```
