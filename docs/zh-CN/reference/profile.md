# Profile 格式参考

## 顶层结构

```json
{
  "injections": [
    { "type": "env", "enabled": true, "vars": {}, "ops": [] },
    { "type": "command", "enabled": false, "program": "fnm", "args": ["env", "--shell", "bash"] },
    { "type": "symlink", "enabled": false, "source": "./src", "target": "~/.target", "on_exist": "error", "cleanup": true }
  ]
}
```

## 注入类型

## `env`

字段：

- `enabled`（默认 `true`）
- `vars` 键值对
- `ops` 操作数组

支持的 `ops`：

- `set`
- `set_if_absent`
- `prepend`
- `append`
- `unset`

`prepend`/`append` 额外字段：

- `separator`：自定义分隔符或 `"os"`。
- `dedup`：去重分段。

## `command`

字段：

- `enabled`（默认 `true`）
- `program`
- `args`（默认空）

输出必须可解析为 `export KEY=value` / `KEY=value` 行。

## `symlink`

字段：

- `enabled`（默认 `true`）
- `source`
- `target`
- `on_exist`：`error` 或 `replace`（默认 `error`）
- `cleanup`（默认 `true`）

`source` 与 `target` 在加载时会归一化为绝对路径。

## 资源 URI 规则

在 `env` 值中：

- `resource://x/y` 解析为 `<resource_home>/x/y`。
- `resource-content://x/y` 解析为文件内容。

`resource_home` 为 `ENVLOCK_RESOURCE_HOME`，未设置时默认 `~/.envlock/resources`。
