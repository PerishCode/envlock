# Release

Release trigger is tag-based:

1. Bump `Cargo.toml` package version.
2. Push matching tag `vX.Y.Z`.
3. Release workflow validates tag/version consistency.
4. Assets and checksums are published to GitHub Release.

Maintainer command example:

```bash
git tag v0.1.7
git push origin v0.1.7
```
