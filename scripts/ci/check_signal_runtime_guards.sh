#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

echo "[signal-guards] Enforcing deterministic containers in worth-signal..."
if rg --type rust "HashMap|HashSet" crates/worth-signal/src >/dev/null 2>&1; then
  echo "FAIL: HashMap/HashSet found in worth-signal/src; use deterministic containers."
  rg --type rust "HashMap|HashSet" crates/worth-signal/src || true
  exit 1
fi

echo "[signal-guards] Enforcing no committed-graph bypass API..."
if rg --type rust "pub fn graph_mut\\(" crates/worth-signal/src >/dev/null 2>&1; then
  echo "FAIL: committed graph bypass API found in worth-signal."
  rg --type rust "pub fn graph_mut\\(" crates/worth-signal/src || true
  exit 1
fi

echo "[signal-guards] Enforcing no node-scale BTreeMap hot-path storage..."
if rg --type rust "BTreeMap<NodeId," crates/worth-signal/src/logic crates/worth-signal/src/data >/dev/null 2>&1; then
  echo "FAIL: node-scale BTreeMap storage found in worth-signal hot/runtime paths."
  rg --type rust "BTreeMap<NodeId," crates/worth-signal/src/logic crates/worth-signal/src/data || true
  exit 1
fi

echo "[signal-guards] Enforcing no vector materialization in signal hot paths..."
if rg --type rust "collect::<Vec|to_vec\\(|marked_indices\\(|current_indices\\(" \
  crates/worth-signal/src/logic/evaluation \
  crates/worth-signal/src/logic/invalidation.rs \
  crates/worth-signal/src/logic/transaction/runtime.rs >/dev/null 2>&1; then
  echo "FAIL: hot-path vector materialization found in worth-signal runtime paths."
  rg --type rust "collect::<Vec|to_vec\\(|marked_indices\\(|current_indices\\(" \
    crates/worth-signal/src/logic/evaluation \
    crates/worth-signal/src/logic/invalidation.rs \
    crates/worth-signal/src/logic/transaction/runtime.rs || true
  exit 1
fi

echo "[signal-guards] Enforcing no GC alive snapshot allocation pattern..."
if rg --type rust "alive_snapshot" crates/worth-signal/src/data/graph >/dev/null 2>&1; then
  echo "FAIL: GC alive snapshot allocation pattern found."
  rg --type rust "alive_snapshot" crates/worth-signal/src/data/graph || true
  exit 1
fi

echo "[signal-guards] Enforcing transaction ownership of committed runtime writes..."
if rg --type rust "parent\\.graph\\s*=|parent\\.checkpoint\\s*=|parent\\.event_bus\\s*=" \
  crates/worth-signal/src/logic \
  --glob '!**/transaction/**' >/dev/null 2>&1; then
  echo "FAIL: committed runtime assignment found outside transaction module."
  rg --type rust "parent\\.graph\\s*=|parent\\.checkpoint\\s*=|parent\\.event_bus\\s*=" \
    crates/worth-signal/src/logic \
    --glob '!**/transaction/**' || true
  exit 1
fi

echo "[signal-guards] Running determinism-critical worth-signal tests..."
cargo test -p worth-signal tests::determinism::kv64_parallel_branches_deterministic --quiet
cargo test -p worth-signal logic::events::tests::deterministic_order_independent_of_registration --quiet
cargo test -p worth-signal logic::events::tests::rollback_runs_reverse_order --quiet
cargo test -p worth-signal logic::transaction::tests::begin_commit_applies_staged_state_once --quiet
cargo test -p worth-signal logic::transaction::tests::begin_rollback_preserves_committed_state --quiet
cargo test -p worth-signal logic::transaction::tests::failure_during_event_begin_rewinds_graph --quiet
cargo test -p worth-signal tests::node_conditions::ondemand_blocks_default_evaluate --quiet
cargo test -p worth-signal tests::node_conditions::aspect_filter_skips_unmatched_dirty_aspect --quiet
cargo test -p worth-signal tests::reentrancy::nested_scratch_acquire_returns_structured_error --quiet
cargo test -p worth-signal logic::transaction::tests::node_tier_metadata_is_generation_safe_on_slot_reuse --quiet

echo "[signal-guards] PASS"
