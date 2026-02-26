# AGENTS

## Directory Conventions

- `src/bin/envlock.rs`: CLI entrypoint.
- `src/profile.rs`: JSON profile schema and parsing.
- `src/injections/`: injection implementations.
- `examples/`: runnable sample profiles.
- `target/`: local build outputs (generated, do not hand-edit).
- `.task/`: branch-bound task state for development workflow, must not stay on `main`.

## Development Workflow

1. Create or switch to a feature branch before changes.
2. Implement changes in `src/` and keep `examples/` aligned when profile/CLI behavior changes.
3. Run local checks before commit:
   - `cargo fmt --check`
   - `cargo test`
4. Keep `README.md` focused on user-facing usage.
5. Before merging to `main`, ensure `.task/` is cleaned up.

## Commit and Merge Rules

- Prefer small, focused commits with clear messages.
- Open PRs against `main`.
- Use squash merge to keep `main` history clean.
