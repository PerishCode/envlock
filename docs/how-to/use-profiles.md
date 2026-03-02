# Use Profiles

`envlock` supports convention-first resolution with explicit override.

## Mode A: Explicit Path

```bash
envlock -p ./profiles/dev.json
```

Use this when your profile lives next to a project.

## Mode B: Convention Default Profile

```bash
envlock
```

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
