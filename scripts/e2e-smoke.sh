#!/usr/bin/env bash
set -euo pipefail

IMAGE="${ENVLOCK_E2E_IMAGE:-ubuntu:24.04}"
PLATFORM="${ENVLOCK_E2E_PLATFORM:-linux/amd64}"
CPU_LIMIT="${ENVLOCK_E2E_CPUS:-1}"
MEM_LIMIT="${ENVLOCK_E2E_MEMORY:-1g}"
REPO_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

usage() {
  cat <<'EOF'
Usage: e2e-smoke.sh <command>

Commands:
  smoke     Run one-shot install/run/uninstall smoke in Linux container
  exec ...  Run one-shot custom shell command in Linux container

Environment overrides:
  ENVLOCK_E2E_IMAGE     Container image (default: ubuntu:24.04)
  ENVLOCK_E2E_PLATFORM  Docker platform (default: linux/amd64)
  ENVLOCK_E2E_CPUS      CPU limit (default: 1)
  ENVLOCK_E2E_MEMORY    Memory limit (default: 1g)
  ENVLOCK_E2E_VERSION   Optional release version passed to install.sh
EOF
}

ensure_docker() {
  if ! command -v docker >/dev/null 2>&1; then
    echo "missing required command: docker" >&2
    exit 1
  fi
}

docker_shell() {
  ensure_docker
  docker run --rm \
    --platform "${PLATFORM}" \
    --cpus "${CPU_LIMIT}" \
    --memory "${MEM_LIMIT}" \
    -v "${REPO_DIR}:/workspace/envlock" \
    -w /workspace/envlock \
    "${IMAGE}" \
    sh -lc "$*"
}

run_smoke() {
  local install_version_arg=""
  if [[ -n "${ENVLOCK_E2E_VERSION:-}" ]]; then
    install_version_arg="--version ${ENVLOCK_E2E_VERSION}"
  fi

  docker_shell "set -eu
apt-get update >/dev/null
apt-get install -y --no-install-recommends bash curl ca-certificates tar coreutils >/dev/null

rm -rf /root/.envlock /root/.local/bin/envlock /tmp/envlock-profile.json /tmp/envlock-preview.txt
bash /workspace/envlock/scripts/install.sh ${install_version_arg}

cat > /tmp/envlock-profile.json <<'JSON'
{\"injections\":[{\"type\":\"env\",\"vars\":{\"ENVLOCK_E2E\":\"ok\"}}]}
JSON

/root/.local/bin/envlock --output json -p /tmp/envlock-profile.json > /tmp/envlock-out.json
grep -q '\"ENVLOCK_E2E\": \"ok\"' /tmp/envlock-out.json

if /root/.local/bin/envlock preview --help >/dev/null 2>&1; then
  /root/.local/bin/envlock preview --profile /tmp/envlock-profile.json --output text > /tmp/envlock-preview.txt
  grep -q 'ENVLOCK_E2E' /tmp/envlock-preview.txt
fi

bash /workspace/envlock/scripts/uninstall.sh
test ! -e /root/.local/bin/envlock
echo 'smoke passed in Linux container'"
}

run_exec() {
  if [[ $# -eq 0 ]]; then
    echo "exec requires a command string" >&2
    exit 1
  fi
  docker_shell "$*"
}

main() {
  local cmd="${1:-}"
  case "${cmd}" in
    smoke)
      run_smoke
      ;;
    exec)
      shift
      run_exec "$@"
      ;;
    -h|--help|help|"")
      usage
      ;;
    *)
      echo "unknown command: ${cmd}" >&2
      usage >&2
      exit 1
      ;;
  esac
}

main "$@"
