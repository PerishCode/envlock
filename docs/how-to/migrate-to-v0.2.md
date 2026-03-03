# Migrate to v0.2

This page gives the minimal migration path for v0.2.1 with a run-first focus.

## 1) Standardize the default profile location

Place the default profile at one of these paths:

- `ENVLOCK_HOME/profiles/default.json` (recommended when your team shares a custom home)
- `~/.envlock/profiles/default.json` (single-machine default)

## 2) Update invocation style

Use default execution for daily usage:

```bash
envlock
```

Use explicit profile path when switching context temporarily:

```bash
envlock --profile ./profiles/dev.json
```

## 3) Verify migration output

```bash
envlock preview --profile ./profiles/dev.json
envlock --profile ./profiles/dev.json --output json
```

If these commands produce stable output, migration is complete.
