# Environment Variables

## Consumed by envlock

| Variable | Purpose |
| --- | --- |
| `ENVLOCK_PROFILE_HOME` | Base directory for `--use` profile lookup (`profiles/<name>.json`). |
| `ENVLOCK_RESOURCE_HOME` | Base directory for `resource://` and `resource-content://`. |
| `HOME` | Fallback base for default profile/resource directories. |

## Default Paths

When `ENVLOCK_PROFILE_HOME` is unset:

- profile home: `~/.envlock`

When `ENVLOCK_RESOURCE_HOME` is unset:

- resource home: `~/.envlock/resources`

If `HOME` is unavailable, envlock falls back to literal strings:

- `~/.envlock`
- `~/.envlock/resources`

These literal fallback paths are not shell-expanded by envlock.
