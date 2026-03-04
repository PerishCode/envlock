# 故障排查

## `resource://` 分隔符行为

`resource://` 的 token 解析会在 `:` 或 `;` 分隔符处停止。
这对 PATH 类拼接有帮助，但包含 `:` 的 URL 类字面量可能被意外拆分。

当你需要保留 `:` 原样时，请使用普通字符串字面量。

## 资源文件缺失的触发时机

- `resource://` 在导出阶段扩展为绝对路径。
- 存在性检查发生在下游工具读取该路径时。
- `resource-content://` 在导出阶段读取，文件缺失会立即失败。

## `HOME` 缺失时的兜底行为

当 `HOME` 不可用时，默认值是字面路径：

- `~/.envlock`
- `~/.envlock/resources`

这些路径不会被 envlock 做 shell 展开。

## 排查清单

1. 运行 `envlock ... --output json` 检查最终环境映射。
2. 加上 `--log-level debug` 查看生命周期日志。
3. 确认 profile 解析路径（显式 `--profile` 或 `ENVLOCK_HOME` 下默认 profile）。
4. 确认资源根目录 `ENVLOCK_RESOURCE_HOME`。
