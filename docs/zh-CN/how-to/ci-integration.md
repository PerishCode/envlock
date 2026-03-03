# CI 集成

在 CI 中使用子命令模式，可将环境注入限制在当前任务进程内，结果可复现。

## GitHub Actions 最小示例

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

## 为什么推荐这个模式

- `--profile` 让每个 job 的 profile 选择显式且可审计。
- `-- <cmd...>` 只影响子进程，不污染 runner 的后续步骤。
- `envlock` 直接返回子进程退出码，CI 状态判断准确。
