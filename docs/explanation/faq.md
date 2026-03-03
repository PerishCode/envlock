# FAQ

## Where is the default profile in v0.2+?

When `--profile` is omitted, envlock resolves:

- `$ENVLOCK_HOME/profiles/default.json` if `ENVLOCK_HOME` is set.
- `~/.envlock/profiles/default.json` otherwise.

## When should I use `--profile`?

Use `--profile` when you need a non-default profile for one command or one session. For daily local usage, keep a stable `default.json` and run `envlock` directly.

## Is `envlock preview` safe to run in production?

`preview` is read-only. It validates and inspects profile metadata but does not execute injections, create symlinks, or run child commands.

## How do I choose shell vs JSON vs command mode?

- Shell mode (default): `eval "$(envlock)"` for interactive shells.
- JSON mode: `envlock --output json` for automation that reads structured output.
- Command mode: `envlock -- <cmd...>` to isolate injection scope to one process.

## What is the difference between `ENVLOCK_HOME` and `ENVLOCK_RESOURCE_HOME`?

- `ENVLOCK_HOME` controls envlock home paths, including default profile lookup.
- `ENVLOCK_RESOURCE_HOME` controls where `resource://` and `resource-content://` read files.

Set one or both when profile location and resource storage should be separated.

## How do I migrate from the old `--use` flow?

In v0.2+, use either default profile execution (`envlock`) or explicit profile path (`envlock --profile ./profiles/dev.json`).

Migration note: `--use` and `ENVLOCK_PROFILE_HOME` are old v0.1 behavior names.

## I get "default profile not found". What should I do first?

Create the default profile and retry:

```bash
mkdir -p "${ENVLOCK_HOME:-$HOME/.envlock}/profiles"
printf '%s\n' '{"injections":[]}' > "${ENVLOCK_HOME:-$HOME/.envlock}/profiles/default.json"
envlock preview --profile "${ENVLOCK_HOME:-$HOME/.envlock}/profiles/default.json"
```
