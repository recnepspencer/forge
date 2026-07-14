#!/usr/bin/env bash
set -euo pipefail

# Tier-0 deterministic cache/runtime gates.
# These are intentionally small and targeted so they are safe for CI check runs.

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "${repo_root}"

echo "[determinism-golden] worth-signal checkpoint runtime"
cargo test -p worth-signal --lib \
  logic::checkpoint_runtime::tests::flush_respects_barrier_policy_and_order \
  -- --test-threads=1

echo "[determinism-golden] all gates passed"
