# Why envlock

`envlock` focuses on one job: produce deterministic environment output from one JSON profile.

## Positioning

- Explicit execution: you choose when to print, evaluate, or run in child command mode.
- Deterministic output: the same profile yields the same output shape for repeatable workflows.
- Preview safety: `envlock preview` lets you inspect profile metadata before applying anything.

## Tradeoffs

- Less magic: convention-first defaults reduce flags, but explicit `--profile` is still preferred for project-local flows.
- Scope stays narrow: `envlock` does not manage your whole shell startup strategy.
- Predictability over flexibility: strict profile schema and clear modes beat implicit behavior.

## When It Fits

- You want reproducible shell setup across teammates and CI.
- You want one profile artifact that is easy to review and version.
- You want read-only preview before changing session state.

See also: [Quick Start](/tutorials/quick-start), [Use Profiles](/how-to/use-profiles), and [Migrate to v0.2](/how-to/migrate-to-v0.2).
