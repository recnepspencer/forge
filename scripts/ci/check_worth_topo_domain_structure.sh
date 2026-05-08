#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

echo "[worth-topo-domain-structure] cargo fmt"
cargo fmt --package worth-topo --check

echo "[worth-topo-domain-structure] cargo check"
cargo check -p worth-topo --quiet

echo "[worth-topo-domain-structure] structure guards"
cargo test -p worth-topo certification::structure_guard --quiet

echo "[worth-topo-domain-structure] facade privacy compile-fail contracts"
cargo test -p worth-topo --test ui --quiet

echo "[worth-topo-domain-structure] worth-topo full suite"
cargo test -p worth-topo --quiet

echo "[worth-topo-domain-structure] worth-topo rust line caps"
violations=0
while IFS= read -r file; do
  line_count="$(wc -l < "$file" | tr -d ' ')"
  if (( line_count > 400 )); then
    echo "FAIL: ${file} is ${line_count} lines (cap 400)"
    violations=1
  fi
done < <(git ls-files 'crates/worth-topo/**/*.rs' | sort)

if (( violations != 0 )); then
  exit 1
fi

echo "[worth-topo-domain-structure] PASS"
