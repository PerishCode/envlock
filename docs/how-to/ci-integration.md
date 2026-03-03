# CI Integration

Use command mode in CI so env injection stays process-local and reproducible.

## Minimal GitHub Actions Example

```yaml
name: ci

on:
  push:
  pull_request:

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Install envlock
        run: curl -fsSL https://raw.githubusercontent.com/PerishCode/envlock/main/scripts/install.sh | sh

      - name: Create CI profile
        run: |
          mkdir -p ./.ci
          cat > ./.ci/envlock.ci.json <<'JSON'
          {
            "injections": [
              {
                "type": "env",
                "vars": {
                  "NODE_ENV": "test",
                  "CI": "true"
                }
              }
            ]
          }
          JSON

      - name: Run tests with envlock command mode
        run: envlock --profile ./.ci/envlock.ci.json -- pnpm test
```

## Why This Pattern

- `--profile` makes profile selection explicit in each job.
- `-- <cmd...>` keeps environment changes scoped to the child process.
- Child exit code is returned directly, so CI status stays accurate.
