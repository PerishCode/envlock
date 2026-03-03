# Quick Reference

Most-used commands for v0.2+.

## Run Flow

```bash
# use default profile (~/.envlock/profiles/default.json)
envlock

# use explicit profile path
envlock --profile ./profiles/dev.json

# print resolved env as JSON
envlock --profile ./profiles/dev.json --output json
```

## Preview (Read-Only)

```bash
envlock preview --profile ./profiles/dev.json
envlock preview --profile ./profiles/dev.json --output json
```

## Command Mode

```bash
# run one command with injected env
envlock --profile ./profiles/dev.json -- pnpm run build

# child exit code is returned
envlock --profile ./profiles/dev.json -- bash -lc 'exit 17'
echo $?
```

## Self-Update

```bash
envlock self-update --check
envlock self-update
envlock self-update --yes
envlock self-update --version v0.2.1 --yes
```

## Install and Uninstall

```bash
# install
curl -fsSL https://raw.githubusercontent.com/PerishCode/envlock/main/scripts/install.sh | sh

# uninstall
curl -fsSL https://raw.githubusercontent.com/PerishCode/envlock/main/scripts/uninstall.sh | sh
```

## Next Pages

- CI usage: [/how-to/ci-integration](/how-to/ci-integration)
- Command mode details: [/how-to/command-mode](/how-to/command-mode)
- Full CLI options: [/reference/cli](/reference/cli)
