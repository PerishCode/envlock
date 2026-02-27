# Profile Format Reference

## Top-Level Shape

```json
{
  "injections": [
    { "type": "env", "enabled": true, "vars": {}, "ops": [] },
    { "type": "command", "enabled": false, "program": "fnm", "args": ["env", "--shell", "bash"] },
    { "type": "symlink", "enabled": false, "source": "./src", "target": "~/.target", "on_exist": "error", "cleanup": true }
  ]
}
```

## Injection Types

## `env`

Fields:

- `enabled` (default `true`)
- `vars` key/value pairs
- `ops` operation array

Supported `ops`:

- `set`
- `set_if_absent`
- `prepend`
- `append`
- `unset`

`prepend`/`append` extra fields:

- `separator`: custom separator or `"os"`.
- `dedup`: remove duplicate segments.

## `command`

Fields:

- `enabled` (default `true`)
- `program`
- `args` (default empty)

Output must be parseable `export KEY=value`/`KEY=value` lines.

## `symlink`

Fields:

- `enabled` (default `true`)
- `source`
- `target`
- `on_exist`: `error` or `replace` (default `error`)
- `cleanup` (default `true`)

`source` and `target` are normalized to absolute paths during load.

## Resource URI Rules

Inside `env` values:

- `resource://x/y` resolves to `<resource_home>/x/y`.
- `resource-content://x/y` resolves to the file contents.

`resource_home` is `ENVLOCK_RESOURCE_HOME` or `~/.envlock/resources`.
