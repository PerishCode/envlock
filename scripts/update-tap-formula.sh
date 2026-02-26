#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'EOF'
Usage:
  update-tap-formula.sh \
    --formula <path/to/Formula/tool.rb> \
    --tool <tool-name> \
    --desc <description> \
    --homepage <url> \
    --version <vX.Y.Z> \
    --macos-arm-url <url> \
    --macos-arm-sha256 <sha256> \
    --macos-amd-url <url> \
    --macos-amd-sha256 <sha256> \
    --linux-amd-url <url> \
    --linux-amd-sha256 <sha256>

Notes:
  - This script writes a deterministic Homebrew formula file.
  - It is repository-agnostic and can be reused for multiple binaries.
EOF
}

require_arg() {
  local name="$1"
  local value="$2"
  if [[ -z "${value}" ]]; then
    echo "missing required argument: ${name}" >&2
    usage >&2
    exit 1
  fi
}

to_ruby_class_name() {
  local raw="$1"
  local part
  local out=""
  local cleaned
  local first
  local rest
  cleaned="$(echo "${raw}" | tr -cs '[:alnum:]' ' ')"
  for part in ${cleaned}; do
    first="$(printf '%s' "${part}" | cut -c1 | tr '[:lower:]' '[:upper:]')"
    rest="$(printf '%s' "${part}" | cut -c2-)"
    out+="${first}${rest}"
  done
  if [[ -z "${out}" ]]; then
    out="Tool"
  fi
  if [[ "${out}" =~ ^[0-9] ]]; then
    out="Tool${out}"
  fi
  printf '%s\n' "${out}"
}

formula=""
tool=""
desc=""
homepage=""
version=""
macos_arm_url=""
macos_arm_sha256=""
macos_amd_url=""
macos_amd_sha256=""
linux_amd_url=""
linux_amd_sha256=""

while [[ $# -gt 0 ]]; do
  case "$1" in
    --formula) formula="${2:-}"; shift 2 ;;
    --tool) tool="${2:-}"; shift 2 ;;
    --desc) desc="${2:-}"; shift 2 ;;
    --homepage) homepage="${2:-}"; shift 2 ;;
    --version) version="${2:-}"; shift 2 ;;
    --macos-arm-url) macos_arm_url="${2:-}"; shift 2 ;;
    --macos-arm-sha256) macos_arm_sha256="${2:-}"; shift 2 ;;
    --macos-amd-url) macos_amd_url="${2:-}"; shift 2 ;;
    --macos-amd-sha256) macos_amd_sha256="${2:-}"; shift 2 ;;
    --linux-amd-url) linux_amd_url="${2:-}"; shift 2 ;;
    --linux-amd-sha256) linux_amd_sha256="${2:-}"; shift 2 ;;
    -h|--help) usage; exit 0 ;;
    *)
      echo "unknown argument: $1" >&2
      usage >&2
      exit 1
      ;;
  esac
done

require_arg "--formula" "${formula}"
require_arg "--tool" "${tool}"
require_arg "--desc" "${desc}"
require_arg "--homepage" "${homepage}"
require_arg "--version" "${version}"
require_arg "--macos-arm-url" "${macos_arm_url}"
require_arg "--macos-arm-sha256" "${macos_arm_sha256}"
require_arg "--macos-amd-url" "${macos_amd_url}"
require_arg "--macos-amd-sha256" "${macos_amd_sha256}"
require_arg "--linux-amd-url" "${linux_amd_url}"
require_arg "--linux-amd-sha256" "${linux_amd_sha256}"

class_name="$(to_ruby_class_name "${tool}")"
formula_dir="$(dirname "${formula}")"
mkdir -p "${formula_dir}"

cat > "${formula}" <<EOF
class ${class_name} < Formula
  desc "${desc}"
  homepage "${homepage}"
  version "${version#v}"
  license "MIT"

  on_macos do
    if Hardware::CPU.arm?
      url "${macos_arm_url}"
      sha256 "${macos_arm_sha256}"
    else
      url "${macos_amd_url}"
      sha256 "${macos_amd_sha256}"
    end
  end

  on_linux do
    url "${linux_amd_url}"
    sha256 "${linux_amd_sha256}"
  end

  def install
    bin.install "${tool}"
  end
end
EOF

echo "updated formula: ${formula}"
