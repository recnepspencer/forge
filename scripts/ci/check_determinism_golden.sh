#!/usr/bin/env bash
set -euo pipefail

# Tier-0 deterministic cache/runtime gates.
# These are intentionally small and targeted so they are safe for CI check runs.

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "${repo_root}"

echo "[determinism-golden] forge-signal checkpoint runtime"
cargo test -p forge-signal --lib \
  logic::checkpoint_runtime::tests::flush_respects_barrier_policy_and_order \
  -- --test-threads=1

echo "[determinism-golden] forge-topo cache runtime policy/parity"
cargo test -p forge-topo --lib \
  b_rep::data::storage::cache_runtime::tests \
  -- --test-threads=1

echo "[determinism-golden] forge-topo replay determinism"
cargo test -p forge-topo --lib \
  transactions::tests::determinism_golden_pipeline_roundtrip_preserves_replay_hash_and_lineage_ordering \
  -- --test-threads=1

echo "[determinism-golden] forge-topo rollback contract lock"
cargo test -p forge-topo --lib \
  transactions::tests::rollback_contract_is_locked_to_snapshot_restore_v1 \
  -- --test-threads=1

echo "[determinism-golden] all gates passed"
