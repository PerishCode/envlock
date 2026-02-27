# Design Boundaries

`envlock` is intentionally narrow.

## What It Does Well

- Build deterministic environment sessions from JSON.
- Compose static values and simple env operations.
- Bridge dynamic bootstrap tools through `command` injection.
- Apply configuration to shell output or process-local command execution.

## What It Intentionally Does Not Do

- Persist global shell mutation automatically.
- Manage package versions or runtime installers directly.
- Replace full task runners or shell framework managers.

## Why This Boundary Exists

A narrow boundary keeps behavior auditable:

- Profiles are explicit and versionable.
- Output can be reviewed before application.
- Runtime behavior is predictable across CI and local shells.

This is also why advanced routing logic (for example tool-specific agent selection) is better handled in caller scripts or aliases, while `envlock` stays focused on environment assembly.
