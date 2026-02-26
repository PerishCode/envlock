#!/usr/bin/env bash
set -euo pipefail

INSTALL_ROOT="${HOME}/.envlock"
BIN_PATH="${INSTALL_ROOT}/bin/envlock"
LINK_PATH="${HOME}/.local/bin/envlock"

if [[ -L "${LINK_PATH}" ]]; then
  target="$(readlink "${LINK_PATH}" || true)"
  if [[ "${target}" == "${BIN_PATH}" ]]; then
    rm -f "${LINK_PATH}"
    echo "Removed symlink ${LINK_PATH}"
  fi
fi

if [[ -d "${INSTALL_ROOT}" ]]; then
  rm -rf "${INSTALL_ROOT}"
  echo "Removed ${INSTALL_ROOT}"
else
  echo "Nothing to remove at ${INSTALL_ROOT}"
fi

echo "envlock uninstalled."
