#!/usr/bin/env bash
set -euo pipefail

METHOD="${1:-}"
if [[ -z "$METHOD" ]]; then
  echo "usage: node.sh <init|validate|preview|apply>" >&2
  exit 64
fi
shift

STATE_DIR_ARG=""
NODE_BIN_ARG=""
NPM_BIN_ARG=""
PNPM_BIN_ARG=""
YARN_BIN_ARG=""

while [[ $# -gt 0 ]]; do
  case "$1" in
    --state-dir)
      [[ $# -ge 2 ]] || { echo "missing value for --state-dir" >&2; exit 64; }
      STATE_DIR_ARG="$2"
      shift 2
      ;;
    --node-bin)
      [[ $# -ge 2 ]] || { echo "missing value for --node-bin" >&2; exit 64; }
      NODE_BIN_ARG="$2"
      shift 2
      ;;
    --npm-bin)
      [[ $# -ge 2 ]] || { echo "missing value for --npm-bin" >&2; exit 64; }
      NPM_BIN_ARG="$2"
      shift 2
      ;;
    --pnpm-bin)
      [[ $# -ge 2 ]] || { echo "missing value for --pnpm-bin" >&2; exit 64; }
      PNPM_BIN_ARG="$2"
      shift 2
      ;;
    --yarn-bin)
      [[ $# -ge 2 ]] || { echo "missing value for --yarn-bin" >&2; exit 64; }
      YARN_BIN_ARG="$2"
      shift 2
      ;;
    --force)
      shift
      ;;
    -h|--help)
      echo "usage: node.sh <init|validate|preview|apply> [--state-dir <path>] [--node-bin <path>] [--npm-bin <path>] [--pnpm-bin <path>] [--yarn-bin <path>]" >&2
      exit 0
      ;;
    *)
      echo "unsupported option: $1" >&2
      exit 64
      ;;
  esac
done

STATE_DIR="${STATE_DIR_ARG:-${ENVLOCK_PLUGIN_NODE_STATE_DIR:-${ENVLOCK_HOME:-$HOME/.envlock}/plugin-node}}"
NODE_BIN_OVERRIDE="${NODE_BIN_ARG:-${ENVLOCK_PLUGIN_NODE_BIN:-}}"
NPM_BIN_OVERRIDE="${NPM_BIN_ARG:-${ENVLOCK_PLUGIN_NPM_BIN:-}}"
PNPM_BIN_OVERRIDE="${PNPM_BIN_ARG:-${ENVLOCK_PLUGIN_PNPM_BIN:-}}"
YARN_BIN_OVERRIDE="${YARN_BIN_ARG:-${ENVLOCK_PLUGIN_YARN_BIN:-}}"

CURRENT_BIN_DIR="$STATE_DIR/current/bin"
LOCK_DIR="$STATE_DIR/locks/apply.lock"
LOCK_PID_FILE="$LOCK_DIR/pid"
STATE_FILE="$STATE_DIR/state.v2.json"

state_error() {
  local action="$1"
  local detail="${2:-unknown error}"
  log_error "state action=$action state_dir=$STATE_DIR detail=$detail"
  echo "failed to $action in state dir $STATE_DIR: $detail" >&2
  exit 74
}

run_state_op() {
  local action="$1"
  shift
  local output
  if ! output="$("$@" 2>&1)"; then
    state_error "$action" "$output"
  fi
}

log_line() {
  local level="$1"
  shift
  [[ -n "${ENVLOCK_LOG_FILE:-}" ]] || return 0
  mkdir -p "$(dirname "$ENVLOCK_LOG_FILE")" 2>/dev/null || true
  printf '%s %s plugin.node %s\n' "$(date -u +'%Y-%m-%dT%H:%M:%SZ')" "$level" "$*" >> "$ENVLOCK_LOG_FILE" 2>/dev/null || true
}

log_info() {
  log_line INFO "$*"
}

log_warn() {
  log_line WARN "$*"
}

log_error() {
  log_line ERROR "$*"
}

json_escape() {
  printf '%s' "$1" | sed 's/\\/\\\\/g; s/"/\\"/g'
}

normalize_version() {
  local raw="$1"
  raw="${raw#v}"
  printf '%s' "$raw"
}

resolve_tool_bin() {
  local __resultvar="$1"
  local tool="$2"
  local override="$3"
  if [[ -n "$override" ]]; then
    log_info "resolve tool=$tool source=override bin=$override"
    if [[ -L "$override" && ! -e "$override" ]]; then
      log_error "resolve tool=$tool invalid_symlink=$override"
      echo "configured $tool binary symlink has invalid or looped target: $override" >&2
      return 2
    fi
    [[ -e "$override" ]] || {
      log_error "resolve tool=$tool missing_bin=$override"
      echo "configured $tool binary does not exist or cannot be read: $override" >&2
      return 2
    }
    [[ -x "$override" ]] || {
      log_error "resolve tool=$tool non_executable=$override"
      echo "configured $tool binary is not executable: $override" >&2
      return 2
    }
    printf -v "$__resultvar" '%s' "$override"
    return 0
  fi

  if command -v "$tool" >/dev/null 2>&1; then
    log_info "resolve tool=$tool source=path bin=$(command -v "$tool")"
    printf -v "$__resultvar" '%s' "$(command -v "$tool")"
    return 0
  fi

  log_warn "resolve tool=$tool source=path result=not_found"
  return 1
}

require_resolved_tool() {
  local __resultvar="$1"
  local tool="$2"
  local override="$3"
  local status=0

  resolve_tool_bin "$__resultvar" "$tool" "$override"
  status=$?
  if [[ "$status" -eq 0 ]]; then
    return 0
  fi

  case "$status" in
    1)
      log_error "resolve tool=$tool result=not_found"
      echo "$tool binary not found (set ENVLOCK_PLUGIN_${tool^^}_BIN or ensure $tool on PATH)" >&2
      exit 3
      ;;
    2)
      exit 2
      ;;
    *)
      exit "$status"
      ;;
  esac
}

resolve_tool_version() {
  local bin="$1"
  local raw
  local node_bin_dir=""
  if [[ -n "${NODE_BIN:-}" ]]; then
    node_bin_dir="$(dirname "$NODE_BIN")"
  fi
  if ! raw="$(PATH="${node_bin_dir}${node_bin_dir:+:}$PATH" "$bin" --version 2>/dev/null)"; then
    return 1
  fi
  log_info "version bin=$bin raw=$raw"
  normalize_version "$raw"
}

tool_version_dir() {
  local tool="$1"
  local version="$2"
  printf '%s/versions/%s/v%s' "$STATE_DIR" "$tool" "$version"
}

tool_cache_dir() {
  local tool="$1"
  local version="$2"
  printf '%s/cache/%s/v%s' "$STATE_DIR" "$tool" "$version"
}

write_wrapper() {
  local path="$1"
  local real_bin="$2"
  local extra_env="$3"
  local node_dir="$(tool_version_dir node "$NODE_VERSION")/bin"
  local npm_global_bin="$(tool_version_dir npm "$NPM_VERSION")/global/bin"

  run_state_op "prepare wrapper dir" mkdir -p "$(dirname "$path")"
  cat > "$path" <<EOF
#!/usr/bin/env bash
set -euo pipefail
export PATH="$(json_escape "$node_dir"):$(json_escape "$CURRENT_BIN_DIR"):$(json_escape "$npm_global_bin")\${PATH:+:\$PATH}"
$extra_env
exec "$(json_escape "$real_bin")" "\$@"
EOF
  run_state_op "set wrapper executable" chmod 755 "$path"
}

write_state() {
  local node_bin="$1" node_version="$2"
  local npm_bin="$3" npm_version="$4"
  local pnpm_bin="$5" pnpm_version="$6"
  local yarn_bin="$7" yarn_version="$8"

  mkdir -p "$(dirname "$STATE_FILE")"
  cat > "$STATE_FILE" <<EOF
{
  "schema": "envlock.plugin-node.state.v2",
  "resolved": {
    "node": { "bin": "$(json_escape "$node_bin")", "version": "$(json_escape "$node_version")" },
    "npm": { "bin": "$(json_escape "$npm_bin")", "version": "$(json_escape "$npm_version")" },
    "pnpm": { "bin": "$(json_escape "$pnpm_bin")", "version": "$(json_escape "$pnpm_version")" },
    "yarn": { "bin": "$(json_escape "$yarn_bin")", "version": "$(json_escape "$yarn_version")" }
  },
  "paths": {
    "current_bin": "$(json_escape "$CURRENT_BIN_DIR")"
  }
}
EOF
  log_info "state wrote file=$STATE_FILE"
}

ensure_layout() {
  run_state_op "prepare layout" mkdir -p "$CURRENT_BIN_DIR" "$STATE_DIR/locks"
  log_info "layout state_dir=$STATE_DIR current_bin=$CURRENT_BIN_DIR"
}

read_lock_pid() {
  local pid=""
  if ! pid="$(cat "$LOCK_PID_FILE" 2>/dev/null)"; then
    return 1
  fi
  case "$pid" in
    ''|*[!0-9]*)
      return 1
      ;;
  esac
  printf '%s' "$pid"
}

release_lock() {
  log_info "lock released dir=$LOCK_DIR"
  rm -f "$LOCK_PID_FILE" 2>/dev/null || true
  rmdir "$LOCK_DIR" 2>/dev/null || true
}

recover_stale_lock() {
  local pid=""

  if pid="$(read_lock_pid)"; then
    if kill -0 "$pid" 2>/dev/null; then
      log_warn "lock busy dir=$LOCK_DIR pid=$pid"
      return 1
    fi
    log_warn "lock stale dir=$LOCK_DIR pid=$pid"
    rm -rf "$LOCK_DIR"
    return 0
  fi

  sleep 1
  if pid="$(read_lock_pid)"; then
    if kill -0 "$pid" 2>/dev/null; then
      log_warn "lock busy dir=$LOCK_DIR pid=$pid"
      return 1
    fi
    log_warn "lock stale dir=$LOCK_DIR pid=$pid"
    rm -rf "$LOCK_DIR"
    return 0
  fi

  if rmdir "$LOCK_DIR" 2>/dev/null; then
    log_warn "lock stale dir=$LOCK_DIR pid=missing"
    return 0
  fi

  log_warn "lock busy dir=$LOCK_DIR pid=unknown"
  return 1
}

acquire_lock() {
  local lock_output=""

  if lock_output="$(mkdir "$LOCK_DIR" 2>&1)"; then
    :
  elif [[ ! -e "$LOCK_DIR" ]]; then
    state_error "create lock directory" "$lock_output"
  else
    recover_stale_lock || {
      log_warn "lock rejected dir=$LOCK_DIR"
      echo "node plugin apply is locked by another process" >&2
      exit 73
    }

    if lock_output="$(mkdir "$LOCK_DIR" 2>&1)"; then
      :
    elif [[ -e "$LOCK_DIR" ]]; then
      log_warn "lock rejected dir=$LOCK_DIR"
      echo "node plugin apply is locked by another process" >&2
      exit 73
    else
      state_error "create lock directory" "$lock_output"
    fi
  fi

  printf '%s\n' "$$" > "$LOCK_PID_FILE"
  log_info "lock acquired dir=$LOCK_DIR pid=$$"
  trap 'release_lock' EXIT
}

empty_patch() {
  cat <<'EOF'
{
  "schema": "envlock.patch.v1",
  "env": [],
  "symlink": []
}
EOF
}

emit_patch() {
  local node_bin="$1" node_version="$2"
  local npm_bin="$3" npm_version="$4"
  local pnpm_bin="$5" pnpm_version="$6"
  local yarn_bin="$7" yarn_version="$8"

  local node_wrapper="$(tool_version_dir node "$node_version")/bin/node"
  local npm_wrapper="$(tool_version_dir npm "$npm_version")/bin/npm"
  local pnpm_wrapper="$(tool_version_dir pnpm "$pnpm_version")/bin/pnpm"
  local yarn_wrapper="$(tool_version_dir yarn "$yarn_version")/bin/yarn"
  local current_node="$CURRENT_BIN_DIR/node"
  local current_npm="$CURRENT_BIN_DIR/npm"
  local current_pnpm="$CURRENT_BIN_DIR/pnpm"
  local current_yarn="$CURRENT_BIN_DIR/yarn"

  cat <<EOF
{
  "schema": "envlock.patch.v1",
  "env": [
    { "op": "set", "key": "ENVLOCK_NODE_BIN", "value": "$(json_escape "$current_node")" },
    { "op": "set", "key": "ENVLOCK_NODE_VERSION", "value": "$(json_escape "$node_version")" },
    { "op": "set", "key": "NPM_CONFIG_CACHE", "value": "$(json_escape "$(tool_cache_dir npm "$npm_version")")" },
    { "op": "set", "key": "NPM_CONFIG_PREFIX", "value": "$(json_escape "$(tool_version_dir npm "$npm_version")/global")" },
    { "op": "set", "key": "PNPM_HOME", "value": "$(json_escape "$CURRENT_BIN_DIR")" },
    { "op": "set", "key": "YARN_CACHE_FOLDER", "value": "$(json_escape "$(tool_cache_dir yarn "$yarn_version")")" },
    { "op": "prepend_path", "key": "PATH", "value": "$(json_escape "$(tool_version_dir npm "$npm_version")/global/bin")", "separator": ":" },
    { "op": "prepend_path", "key": "PATH", "value": "$(json_escape "$CURRENT_BIN_DIR")", "separator": ":" }
  ],
  "symlink": [
    { "op": "ensure", "source": "$(json_escape "$node_wrapper")", "target": "$(json_escape "$current_node")", "on_exist": "replace" },
    { "op": "ensure", "source": "$(json_escape "$npm_wrapper")", "target": "$(json_escape "$current_npm")", "on_exist": "replace" },
    { "op": "ensure", "source": "$(json_escape "$pnpm_wrapper")", "target": "$(json_escape "$current_pnpm")", "on_exist": "replace" },
    { "op": "ensure", "source": "$(json_escape "$yarn_wrapper")", "target": "$(json_escape "$current_yarn")", "on_exist": "replace" }
  ]
}
EOF
  log_info "patch emitted env_count=8 symlink_count=4"
}

resolve_all_tools() {
  require_resolved_tool NODE_BIN node "$NODE_BIN_OVERRIDE"
  [[ -x "$NODE_BIN" ]] || {
    echo "node binary is not executable: $NODE_BIN" >&2
    exit 2
  }
  NODE_VERSION="$(resolve_tool_version "$NODE_BIN")" || {
    log_error "version tool=node bin=$NODE_BIN result=failed"
    echo "failed to resolve node version from: $NODE_BIN" >&2
    exit 4
  }

  require_resolved_tool NPM_BIN npm "$NPM_BIN_OVERRIDE"
  require_resolved_tool PNPM_BIN pnpm "$PNPM_BIN_OVERRIDE"
  require_resolved_tool YARN_BIN yarn "$YARN_BIN_OVERRIDE"

  NPM_VERSION="$(resolve_tool_version "$NPM_BIN")" || { log_error "version tool=npm bin=$NPM_BIN result=failed"; echo "failed to resolve npm version" >&2; exit 4; }
  PNPM_VERSION="$(resolve_tool_version "$PNPM_BIN")" || { log_error "version tool=pnpm bin=$PNPM_BIN result=failed"; echo "failed to resolve pnpm version" >&2; exit 4; }
  YARN_VERSION="$(resolve_tool_version "$YARN_BIN")" || { log_error "version tool=yarn bin=$YARN_BIN result=failed"; echo "failed to resolve yarn version" >&2; exit 4; }
  log_info "resolved node=$NODE_VERSION npm=$NPM_VERSION pnpm=$PNPM_VERSION yarn=$YARN_VERSION"
}

prepare_version_dirs() {
  run_state_op "prepare version directories" mkdir -p \
    "$(tool_version_dir node "$NODE_VERSION")/bin" \
    "$(tool_version_dir npm "$NPM_VERSION")/bin" \
    "$(tool_version_dir npm "$NPM_VERSION")/global" \
    "$(tool_version_dir npm "$NPM_VERSION")/global/bin" \
    "$(tool_version_dir npm "$NPM_VERSION")/global/lib/node_modules" \
    "$(tool_version_dir pnpm "$PNPM_VERSION")/bin" \
    "$(tool_version_dir yarn "$YARN_VERSION")/bin" \
    "$(tool_version_dir yarn "$YARN_VERSION")/global" \
    "$(tool_cache_dir npm "$NPM_VERSION")" \
    "$(tool_cache_dir pnpm "$PNPM_VERSION")/store" \
    "$(tool_cache_dir yarn "$YARN_VERSION")"
  log_info "dirs prepared state_dir=$STATE_DIR"
}

link_versions() {
  local npm_prefix="$(tool_version_dir npm "$NPM_VERSION")/global"
  local pnpm_store_dir="$(tool_cache_dir pnpm "$PNPM_VERSION")/store"
  local yarn_cache_dir="$(tool_cache_dir yarn "$YARN_VERSION")"
  local yarn_global_dir="$(tool_version_dir yarn "$YARN_VERSION")/global"

  run_state_op "link node version" ln -sfn "$NODE_BIN" "$(tool_version_dir node "$NODE_VERSION")/bin/node"
  write_wrapper "$(tool_version_dir npm "$NPM_VERSION")/bin/npm" "$NPM_BIN" "export NPM_CONFIG_CACHE=\"$(json_escape "$(tool_cache_dir npm "$NPM_VERSION")")\"; export NPM_CONFIG_PREFIX=\"$(json_escape "$npm_prefix")\""
  write_wrapper "$(tool_version_dir pnpm "$PNPM_VERSION")/bin/pnpm" "$PNPM_BIN" "export PNPM_HOME=\"$(json_escape "$CURRENT_BIN_DIR")\"; export npm_config_store_dir=\"$(json_escape "$pnpm_store_dir")\""
  write_wrapper "$(tool_version_dir yarn "$YARN_VERSION")/bin/yarn" "$YARN_BIN" "export YARN_CACHE_FOLDER=\"$(json_escape "$yarn_cache_dir")\"; export YARN_GLOBAL_FOLDER=\"$(json_escape "$yarn_global_dir")\"; export PREFIX=\"$(json_escape "$npm_prefix")\"; export npm_config_prefix=\"$(json_escape "$npm_prefix")\""

  run_state_op "refresh current node link" ln -sfn "$(tool_version_dir node "$NODE_VERSION")/bin/node" "$CURRENT_BIN_DIR/node"
  run_state_op "refresh current npm link" ln -sfn "$(tool_version_dir npm "$NPM_VERSION")/bin/npm" "$CURRENT_BIN_DIR/npm"
  run_state_op "refresh current pnpm link" ln -sfn "$(tool_version_dir pnpm "$PNPM_VERSION")/bin/pnpm" "$CURRENT_BIN_DIR/pnpm"
  run_state_op "refresh current yarn link" ln -sfn "$(tool_version_dir yarn "$YARN_VERSION")/bin/yarn" "$CURRENT_BIN_DIR/yarn"
  log_info "symlinks refreshed current_bin=$CURRENT_BIN_DIR"
}

do_init() {
  ensure_layout
  log_info "method=init"
  empty_patch
}

do_validate_or_preview() {
  ensure_layout
  log_info "method=$METHOD"
  resolve_all_tools
  prepare_version_dirs
  emit_patch "$NODE_BIN" "$NODE_VERSION" "$NPM_BIN" "$NPM_VERSION" "$PNPM_BIN" "$PNPM_VERSION" "$YARN_BIN" "$YARN_VERSION"
}

do_apply() {
  ensure_layout
  acquire_lock

  resolve_all_tools
  prepare_version_dirs
  link_versions
  write_state "$NODE_BIN" "$NODE_VERSION" "$NPM_BIN" "$NPM_VERSION" "$PNPM_BIN" "$PNPM_VERSION" "$YARN_BIN" "$YARN_VERSION"
  emit_patch "$NODE_BIN" "$NODE_VERSION" "$NPM_BIN" "$NPM_VERSION" "$PNPM_BIN" "$PNPM_VERSION" "$YARN_BIN" "$YARN_VERSION"
}

case "$METHOD" in
  init)
    do_init
    ;;
  validate|preview)
    do_validate_or_preview
    ;;
  apply)
    do_apply
    ;;
  *)
    echo "unsupported method: $METHOD" >&2
    exit 64
    ;;
esac
