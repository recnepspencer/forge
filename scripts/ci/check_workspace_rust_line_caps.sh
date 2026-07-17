#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

CAP=400
ALLOWLIST="scripts/ci/workspace_rust_line_cap_allowlist.txt"

echo "[workspace-rust-line-caps] enforcing ${CAP}-line cap for tracked Rust files"

violations=0
while read -r line_count file; do
  [[ -n "$file" ]] || continue
  [[ "$file" != "total" ]] || continue

  if (( line_count > CAP )); then
    if [[ -f "$ALLOWLIST" ]] && grep -Fxq "$file" "$ALLOWLIST"; then
      echo "[workspace-rust-line-caps] allowlisted: ${file} (${line_count} lines)"
    else
      echo "FAIL: ${file} is ${line_count} lines (cap ${CAP})"
      violations=1
    fi
  fi
done < <(
  git ls-files -z \
    'crates/**/*.rs' \
    'workspaces/worth-ui/crates/**/*.rs' \
    | while IFS= read -r -d '' file; do
        [[ -f "$file" ]] && printf '%s\0' "$file"
      done \
    | sort -zu \
    | xargs -0 -r -n 128 wc -l
)

if (( violations != 0 )); then
  exit 1
fi

echo "[workspace-rust-line-caps] PASS"
