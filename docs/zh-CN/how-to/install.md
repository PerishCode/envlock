# 安装

## 安装最新版本

```bash
curl -fsSL https://raw.githubusercontent.com/PerishCode/envlock/main/scripts/install.sh | sh
```

## 安装指定版本

```bash
curl -fsSL https://raw.githubusercontent.com/PerishCode/envlock/main/scripts/install.sh | sh -s -- --version v0.3.0
```

## 安装后路径

- 二进制：`~/.envlock/bin/envlock`
- 软链接：`~/.local/bin/envlock`

## 验证

```bash
envlock --version
which envlock
```
