# Troubleshooting

## `resource://` Delimiter Behavior

`resource://` token parsing stops at `:` or `;` delimiters.
This is useful for PATH-like composition, but URL-like literals that include `:` can split unexpectedly.

Use plain string literals when you need `:` preserved as-is.

## Missing Resource File Timing

- `resource://` expands to absolute paths during export.
- Existence checks happen when downstream tooling reads the path.
- `resource-content://` reads at export time and fails early if missing.

## `HOME` Missing Fallback

If `HOME` is not available, fallback defaults are literal:

- `~/.envlock`
- `~/.envlock/resources`

These are not shell-expanded by envlock.

## Debugging Checklist

1. Run `envlock ... --output json` to inspect final map.
2. Add `--log-level debug` for lifecycle logs.
3. Confirm profile path resolution (`--profile` vs default profile under `ENVLOCK_HOME`).
4. Confirm resource root via `ENVLOCK_RESOURCE_HOME`.
