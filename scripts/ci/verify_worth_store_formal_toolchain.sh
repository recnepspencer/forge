#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
crate_root="$repo_root/workspaces/worth-store/crates/worth-store-formal-models"
certification_root="$repo_root/workspaces/worth-store/crates/worth-store-certification"
toolchain="$crate_root/formal-toolchain.toml"

read_value() {
  local name="$1"
  sed -n "s/^${name}[[:space:]]*=[[:space:]]*\"\([^\"]*\)\"$/\1/p" "$toolchain" | head -n 1
}

version="$(read_value version)"
download_url="$(read_value download_url)"
expected_sha256="$(read_value sha256)"
main_class="$(read_value main_class)"
model_relative="$(read_value model)"
configuration_relative="$(read_value configuration)"

tool_cache="${WORTH_STORE_FORMAL_TOOL_CACHE:-$repo_root/workspaces/worth-store/target/formal-tools}"
mkdir -p "$tool_cache"
jar_path="$tool_cache/tla2tools-$version.jar"

if [[ ! -f "$jar_path" ]]; then
  curl --fail --location --silent --show-error --output "$jar_path" "$download_url"
fi

actual_sha256="$(sha256sum "$jar_path" | awk '{print $1}')"
if [[ "$actual_sha256" != "$expected_sha256" ]]; then
  echo "TLC digest mismatch: expected $expected_sha256, got $actual_sha256" >&2
  exit 1
fi

run_model() {
  local model_path="$1"
  local configuration_path="$2"
  local state_name="$3"
  if [[ ! -f "$configuration_path" ]]; then
    echo "missing TLC configuration for $model_path" >&2
    exit 1
  fi
  local state_directory="$tool_cache/states/$state_name"
  mkdir -p "$state_directory"
  java -cp "$jar_path" "$main_class" \
    -deadlock \
    -workers auto \
    -metadir "$state_directory" \
    -config "$configuration_path" \
    "$model_path"
}

run_model \
  "$crate_root/$model_relative" \
  "$crate_root/$configuration_relative" \
  "toolchain-smoke"

protocol_count="$(find "$crate_root/src/protocols" -type f -name '*.tla' -print | wc -l | tr -d ' ')"
cargo run --quiet \
  --manifest-path "$certification_root/Cargo.toml" \
  --bin worth_store_protocol_closeout -- \
  "$(command -v java)" "$jar_path" "$tool_cache/states"

if [[ -n "${WORTH_STORE_FORMAL_EVIDENCE_PATH:-}" ]]; then
  toolchain_sha256="$(sha256sum "$toolchain" | awk '{print $1}')"
  python3 - \
    "$WORTH_STORE_FORMAL_EVIDENCE_PATH" \
    "$version" \
    "$actual_sha256" \
    "$toolchain_sha256" \
    "$main_class" \
    "$model_relative" \
    "$configuration_relative" \
    "$(command -v java)" \
    "$protocol_count" <<'PY'
import json
import pathlib
import sys

(
    output_path,
    version,
    jar_sha256,
    toolchain_sha256,
    main_class,
    model,
    configuration,
    java_executable,
    protocol_count,
) = sys.argv[1:]
receipt = {
    "schema_version": 1,
    "tool": "tlc",
    "version": version,
    "jar_sha256": jar_sha256,
    "toolchain_sha256": toolchain_sha256,
    "main_class": main_class,
    "model": model,
    "configuration": configuration,
    "java_executable": java_executable,
    "protocol_count": int(protocol_count),
    "verdict": "passed",
}
path = pathlib.Path(output_path)
path.parent.mkdir(parents=True, exist_ok=True)
with path.open("x", encoding="utf-8", newline="\n") as handle:
    json.dump(receipt, handle, sort_keys=True, indent=2)
    handle.write("\n")
PY
fi

echo "verified TLC $version ($actual_sha256) across $protocol_count protocol models"
