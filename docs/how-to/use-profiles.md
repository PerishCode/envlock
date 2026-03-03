# Use Profiles

`envlock` supports convention-first resolution with explicit override.

If you are upgrading from older invocation patterns, check [Migrate to v0.2](/how-to/migrate-to-v0.2).

## Mode A: Explicit Path

```bash
envlock -p ./profiles/dev.json
```

Use this when your profile lives next to a project.

## Mode B: Convention Default Profile

```bash
envlock
```

Default profile file:

- `${ENVLOCK_HOME:-$HOME/.envlock}/profiles/default.json`

Lookup behavior:

1. If `ENVLOCK_HOME` is set, resolve from `$ENVLOCK_HOME/profiles/default.json`.
2. Otherwise resolve from `~/.envlock/profiles/default.json`.

## Useful Flags

- `--output shell`: print shell exports.
- `--output json`: print JSON object.
- `--strict`: fail on duplicate keys in final output.

## Resource URI Expansion

`env` values support URI expansion with `ENVLOCK_RESOURCE_HOME`:

- `resource://path/to/file` -> absolute path under resource home.
- `resource-content://path/to/file` -> file contents under resource home.

Default `ENVLOCK_RESOURCE_HOME` when unset:

- `~/.envlock/resources`
