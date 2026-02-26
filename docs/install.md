# Install

Default install is user-scoped and does not require `sudo`.

```bash
curl -fsSL https://raw.githubusercontent.com/PerishCode/envlock/main/scripts/install.sh | sh
```

Install output paths:

- Binary: `~/.envlock/bin/envlock`
- Command link: `~/.local/bin/envlock`

Install a specific version:

```bash
curl -fsSL https://raw.githubusercontent.com/PerishCode/envlock/main/scripts/install.sh | sh -s -- --version v0.1.7
```
