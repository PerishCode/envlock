# Docs Maintenance

Keep docs checks simple and deterministic before opening a PR.
The standard sequence is: `verify-doc-alignment` -> `verify-doc-links` -> `docs:build`.

## Run doc alignment

```bash
bash scripts/verify-doc-alignment.sh
```

## Run doc link integrity

```bash
bash scripts/verify-doc-links.sh
```

## Standard docs check sequence

```bash
bash scripts/verify-doc-alignment.sh
bash scripts/verify-doc-links.sh
pnpm run docs:build
```

## Run full convergence checks

```bash
bash scripts/converge-check.sh
```
