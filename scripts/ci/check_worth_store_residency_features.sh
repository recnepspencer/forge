#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
manifest="$repo_root/workspaces/worth-store/Cargo.toml"

tree="$(cargo tree --manifest-path "$manifest" -p worth-store -e normal,build -f '{p} [{f}]')"
ordinary_pool="$(printf '%s\n' "$tree" | grep 'worth-store-buffer-pool ' || true)"

if [[ -z "$ordinary_pool" ]]; then
  echo '[worth-store-residency-features] missing canonical buffer-pool dependency' >&2
  exit 1
fi

if printf '%s\n' "$ordinary_pool" | grep -q 'legacy-s2-models'; then
  echo '[worth-store-residency-features] ordinary Store activates legacy-s2-models' >&2
  printf '%s\n' "$ordinary_pool" >&2
  exit 1
fi

echo '[worth-store-residency-features] PASS: ordinary Store uses canonical residency only'
