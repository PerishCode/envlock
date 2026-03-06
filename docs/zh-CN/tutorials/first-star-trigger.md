# 首星触发页（0 社媒）

10 秒理解：`envlock` 用 JSON profile 注入可复现环境变量，并且可以只作用于单个子命令，不污染父 shell。

60 秒验证：运行下面 3 条命令并核对预期输出。

## 适用 / 不适用

适用：

- 你要让某个命令在固定环境变量下运行。
- 你要让本地与 CI 使用同一份 profile 输入。
- 你希望用显式路径（`--profile`）保证行为可预测。

不适用：

- 你需要交互式密钥轮换。
- 你希望永久修改系统级环境变量。
- 你需要在 profile 里写动态逻辑。

## 60 秒验证（三步命令）

```bash
# 1) 创建最小 profile
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

# 2) 只读预览 key
envlock preview --profile ./.tmp/envlock.first-star.json --output json

# 3) 用注入环境运行单个子命令
envlock --profile ./.tmp/envlock.first-star.json -- bash -lc 'echo "$ENVLOCK_STAR:$ENVLOCK_SCOPE"'
```

## 预期输出

- 第 2 步输出可看到 `ENVLOCK_STAR` 和 `ENVLOCK_SCOPE`。
- 第 3 步应精确输出：

```text
first-star:child-only
```

- 第 3 步结束后，父 shell 不会被持久修改。

## 下一步入口

- 安装：[/zh-CN/how-to/install](/zh-CN/how-to/install)
- CLI：[/zh-CN/reference/cli](/zh-CN/reference/cli)
- CI：[/zh-CN/how-to/ci-integration](/zh-CN/how-to/ci-integration)
- 排障：[/zh-CN/explanation/troubleshooting](/zh-CN/explanation/troubleshooting)
