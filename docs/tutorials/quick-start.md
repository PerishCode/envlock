# Quick Start

This tutorial gets `envlock` working end-to-end in a few minutes.

## Prerequisites

- You can run shell commands in `bash`, `zsh`, or compatible shells.
- You have either:
  - the `envlock` binary installed, or
  - the source repository and Rust toolchain.

## Step 1: Create a Profile

Create `quickstart.json`:

```json
{
  "injections": [
    {
      "type": "env",
      "vars": {
        "ENVLOCK_PROFILE": "quickstart",
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

## Step 2: Preview Output

```bash
envlock -p quickstart.json --output shell
```

Expected shape:

```bash
export ENVLOCK_PROFILE='quickstart'
export KUBECONFIG_CONTEXT='dev-cluster'
```

## Step 3: Apply in Current Shell

```bash
eval "$(envlock -p quickstart.json)"
echo "$ENVLOCK_PROFILE"
```

You should see `quickstart`.

## Step 4: Run in Command Mode

Command mode injects variables only into the child process:

```bash
envlock -p quickstart.json -- bash -lc 'echo "$ENVLOCK_PROFILE"'
```

This prints `quickstart` without mutating your parent shell.

## Next

- Learn profile resolution with `--use` in [Use Profiles](/how-to/use-profiles).
- Review all options in [CLI Reference](/reference/cli).
