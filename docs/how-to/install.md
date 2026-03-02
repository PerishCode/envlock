# Install

## Install Latest Release

```bash
curl -fsSL https://raw.githubusercontent.com/PerishCode/envlock/main/scripts/install.sh | sh
```

## Install a Specific Version

```bash
curl -fsSL https://raw.githubusercontent.com/PerishCode/envlock/main/scripts/install.sh | sh -s -- --version v0.2.1
```

## Installed Paths

- Binary: `~/.envlock/bin/envlock`
- Symlink: `~/.local/bin/envlock`

## Verify

```bash
envlock --version
which envlock
```

## Platform Notes

`install.sh` currently packages these targets:

- `x86_64-unknown-linux-gnu`
- `x86_64-apple-darwin`
- `aarch64-apple-darwin`

If your shell cannot find `envlock`, add `~/.local/bin` to `PATH`.
