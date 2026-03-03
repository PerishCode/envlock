# 文档维护

在提交 PR 前，先运行这些简单且可验证的文档检查。
标准顺序是：`verify-doc-alignment` -> `verify-doc-links` -> `docs:build`。

## 运行文档对齐检查

```bash
bash scripts/verify-doc-alignment.sh
```

## 运行文档链接完整性检查

```bash
bash scripts/verify-doc-links.sh
```

## 标准文档检查顺序

```bash
bash scripts/verify-doc-alignment.sh
bash scripts/verify-doc-links.sh
pnpm run docs:build
```

## 运行完整收敛检查

```bash
bash scripts/converge-check.sh
```
