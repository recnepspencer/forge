#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

CAP=400
ALLOWLIST="scripts/ci/workspace_rust_line_cap_allowlist.txt"

echo "[workspace-rust-line-caps] enforcing ${CAP}-line cap for tracked Rust files"

violations=0
while IFS= read -r file; do
  [[ -n "$file" ]] || continue

  line_count="$(wc -l < "$file" | tr -d ' ')"
  if (( line_count > CAP )); then
    if [[ -f "$ALLOWLIST" ]] && grep -Fxq "$file" "$ALLOWLIST"; then
      echo "[workspace-rust-line-caps] allowlisted: ${file} (${line_count} lines)"
    else
      echo "FAIL: ${file} is ${line_count} lines (cap ${CAP})"
      violations=1
    fi
  fi
done < <(git ls-files 'crates/**/*.rs' | sort)

if (( violations != 0 )); then
  exit 1
fi

echo "[workspace-rust-line-caps] PASS"
