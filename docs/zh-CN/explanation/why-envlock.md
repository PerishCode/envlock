# 为什么选择 envlock

`envlock` 只聚焦一件事：从一个 JSON profile 产出确定性的环境输出。

## 定位

- 显式执行：何时打印、何时 `eval`、何时走 command mode，都由你控制。
- 输出确定性：同一个 profile 产出同样的输出形态，便于重复执行。
- 预览安全：`envlock preview` 可在应用前检查 profile 元信息。

## 取舍

- 更少魔法：约定优先减少参数，但项目内场景仍推荐显式 `--profile`。
- 范围收敛：`envlock` 不负责管理整个 shell 启动策略。
- 可预测优先于灵活：严格 schema 与清晰模式优先于隐式行为。

## 适用场景

- 你希望在团队与 CI 之间复现一致的 shell 环境。
- 你希望把环境配置收敛为一个易审阅、可版本化的 profile。
- 你希望在改变会话状态前先做只读预览。

另见：[快速开始](/zh-CN/tutorials/quick-start)、[使用 Profiles](/zh-CN/how-to/use-profiles)、[迁移到 v0.3](/zh-CN/how-to/migrate-to-v0.3)。
