# 迁移到 v0.2

本页提供 v0.2.1 的最小迁移路径，优先保证可运行。

## 1) 统一默认 profile 位置

把默认 profile 放到以下路径之一：

- `ENVLOCK_HOME/profiles/default.json`（推荐：项目或团队有统一目录时）
- `~/.envlock/profiles/default.json`（本地单机默认）

## 2) 更新调用方式

日常默认运行：

```bash
envlock
```

需要临时切换 profile 时：

```bash
envlock --profile ./profiles/dev.json
```

## 3) 验证迁移结果

```bash
envlock preview --profile ./profiles/dev.json
envlock --profile ./profiles/dev.json --output json
```

如果以上命令可稳定输出，迁移即可视为完成。
