# Quick Start

This tutorial gets `envlock` working end-to-end in a few minutes.

## Prerequisites

- You can run shell commands in `bash`, `zsh`, or compatible shells.
- You have either:
  - the `envlock` binary installed, or
  - the source repository and Rust toolchain.

## Step 1: Create the Convention Default Profile

Create `${ENVLOCK_HOME:-$HOME/.envlock}/profiles/default.json`:

```bash
mkdir -p "${ENVLOCK_HOME:-$HOME/.envlock}/profiles"
```

```json
{
  "injections": [
    {
      "type": "env",
      "vars": {
        "ENVLOCK_PROFILE": "default",
        "KUBECONFIG_CONTEXT": "dev-cluster"
      },
      "ops": [
        {
          "op": "prepend",
          "key": "PATH",
          "value": "~/.local/bin",
          "separator": "os",
          "dedup": true
        }
      ]
    }
  ]
}
```

Save the JSON as `${ENVLOCK_HOME:-$HOME/.envlock}/profiles/default.json`.

## Step 2: Preview Output

```bash
envlock preview
envlock --output shell
```

Expected shape:

```bash
export ENVLOCK_PROFILE='default'
export KUBECONFIG_CONTEXT='dev-cluster'
```

## Step 3: Apply in Current Shell

```bash
eval "$(envlock)"
echo "$ENVLOCK_PROFILE"
```

You should see `default`.

## Step 4: Run in Command Mode

Command mode injects variables only into the child process:

```bash
envlock -- bash -lc 'echo "$ENVLOCK_PROFILE"'
```

This prints `default` without mutating your parent shell.

For project-local profiles, keep explicit path mode:

```bash
envlock --profile ./profiles/dev.json
```

## Next

- Learn default profile resolution in [Use Profiles](/how-to/use-profiles).
- Review migration notes in [Migrate to v0.2](/how-to/migrate-to-v0.2).
- Review all options in [CLI Reference](/reference/cli).
