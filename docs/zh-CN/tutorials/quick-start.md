# 快速开始

本教程会在几分钟内让你从零跑通 `envlock`。

## 前置条件

- 你可以在 `bash`、`zsh` 或兼容 shell 中执行命令。
- 你具备以下其中之一：
  - 已安装 `envlock` 二进制；
  - 已拉取源码仓库并安装 Rust 工具链。

## 第一步：创建约定默认 profile

创建 `${ENVLOCK_HOME:-$HOME/.envlock}/profiles/default.json`：

```bash
mkdir -p "${ENVLOCK_HOME:-$HOME/.envlock}/profiles"
```

```json
{
  "injections": [
    {
      "type": "env",
      "vars": {
        "ENVLOCK_PROFILE": "default",
        "KUBECONFIG_CONTEXT": "dev-cluster"
      },
      "ops": [
        {
          "op": "prepend",
          "key": "PATH",
          "value": "~/.local/bin",
          "separator": "os",
          "dedup": true
        }
      ]
    }
  ]
}
```

将 JSON 保存到 `${ENVLOCK_HOME:-$HOME/.envlock}/profiles/default.json`。

## 第二步：预览输出

```bash
envlock preview
envlock --output shell
```

预期输出形态：

```bash
export ENVLOCK_PROFILE='default'
export KUBECONFIG_CONTEXT='dev-cluster'
```

## 第三步：在当前 shell 生效

```bash
eval "$(envlock)"
echo "$ENVLOCK_PROFILE"
```

你应该看到 `default`。

## 第四步：使用 command mode

command mode 只对子进程注入变量：

```bash
envlock -- bash -lc 'echo "$ENVLOCK_PROFILE"'
```

这会打印 `default`，但不会修改父 shell。

对于项目内 profile，建议保持显式路径模式：

```bash
envlock --profile ./profiles/dev.json
```

## 下一步

- 查看默认 profile 解析规则：[使用 Profiles](/zh-CN/how-to/use-profiles)
- 查看迁移说明：[迁移到 v0.3](/zh-CN/how-to/migrate-to-v0.3)
- 查看完整参数：[CLI 参考](/zh-CN/reference/cli)
