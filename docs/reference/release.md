# Release Pipeline

## Triggers

- CI: pull requests and pushes to `main`.
- Release: tag push matching `v*`.
- Docs deploy: pushes to `main` affecting `docs/**` or docs workflow files.

## Release Workflow

1. `release.yml` validates tag/version consistency (`vX.Y.Z` vs `Cargo.toml`).
2. Build runs per target:
   - `x86_64-unknown-linux-gnu`
   - `x86_64-apple-darwin`
   - `aarch64-apple-darwin`
3. Binary archives and `checksums.txt` are generated.
4. Artifacts are published to GitHub Release.

## Maintainer Steps

```bash
# after merging changes and bumping Cargo.toml version
git tag v0.1.8
git push origin v0.1.8
```
