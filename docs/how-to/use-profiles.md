# Use Profiles

`envlock` supports two profile selection modes.

## Mode A: Explicit Path

```bash
envlock -p ./profiles/dev.json
```

Use this when your profile lives next to a project.

## Mode B: Named Profile with `--use`

```bash
envlock --use dev
```

Lookup behavior:

1. If `ENVLOCK_PROFILE_HOME` is set, resolve from `$ENVLOCK_PROFILE_HOME/profiles/<name>.json`.
2. Otherwise resolve from `~/.envlock/profiles/<name>.json`.

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
