# Run Command Mode

Use command mode to run a child process with injected environment.

## Basic Usage

```bash
envlock -p profile.json -- bash -lc 'echo "$ENVLOCK_PROFILE"'
```

The parent shell remains unchanged.

## Exit Code Propagation

`envlock` returns the child exit code directly:

```bash
envlock -p profile.json -- bash -lc 'exit 17'
echo $?  # 17
```

## Typical Pattern for Tooling

```bash
envlock -p profile.json -- npm run build
```

This is useful in CI and local scripts where environment scope should stay process-local.
