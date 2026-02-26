# Troubleshooting

## `resource://` Delimiter Behavior

`env` injection resolves `resource://` tokens until it hits `:` or `;`.
This is intentional for PATH-like values, but it also means URL-like literals that include `:` may be split unexpectedly.

## Missing Resource File Detection Timing

`resource://` resolution converts relative paths to absolute paths during export, but it does not check file existence at parse/validate time.
If a resource file is missing, the failure usually appears later when downstream tools read that path.

## `HOME` Missing Fallback Semantics

When `HOME` is unavailable, envlock falls back to literal strings:

- `~/.envlock`
- `~/.envlock/resources`

envlock does not shell-expand `~` itself in this fallback path.
