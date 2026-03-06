# First-Star Trigger Page (0 Social)

Get this in 10 seconds: `envlock` injects deterministic env vars from a JSON profile and can scope them to one child command without polluting your parent shell.

Get this in 60 seconds: run the 3 commands below and verify expected output.

## Use This / Do Not Use This

Use this when:

- You want one command to run with fixed env vars.
- You need local and CI to share the same profile input.
- You want explicit profile paths (`--profile`) for predictable runs.

Do not use this when:

- You need interactive secret rotation.
- You expect envlock to mutate system-wide env permanently.
- You need dynamic runtime logic in profiles.

## 60-Second Verify (3 Commands)

```bash
# 1) create a minimal profile
mkdir -p ./.tmp && cat > ./.tmp/envlock.first-star.json <<'JSON'
{
  "injections": [
    {
      "type": "env",
      "vars": {
        "ENVLOCK_STAR": "first-star",
        "ENVLOCK_SCOPE": "child-only"
      }
    }
  ]
}
JSON

# 2) preview keys (read-only)
envlock preview --profile ./.tmp/envlock.first-star.json --output json

# 3) run one child command with injected env
envlock --profile ./.tmp/envlock.first-star.json -- bash -lc 'echo "$ENVLOCK_STAR:$ENVLOCK_SCOPE"'
```

## Expected Output

- Step 2 output includes `ENVLOCK_STAR` and `ENVLOCK_SCOPE`.
- Step 3 prints exactly:

```text
first-star:child-only
```

- After step 3, your parent shell remains unchanged.

## Next Entry Points

- Install: [/how-to/install](/how-to/install)
- CLI: [/reference/cli](/reference/cli)
- CI: [/how-to/ci-integration](/how-to/ci-integration)
- Troubleshooting: [/explanation/troubleshooting](/explanation/troubleshooting)
