#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

echo "[signal-guards] Enforcing deterministic containers in forge-signal..."
if rg --type rust "HashMap|HashSet" crates/forge-signal/src >/dev/null 2>&1; then
  echo "FAIL: HashMap/HashSet found in forge-signal/src; use deterministic containers."
  rg --type rust "HashMap|HashSet" crates/forge-signal/src || true
  exit 1
fi

echo "[signal-guards] Enforcing no committed-graph bypass API..."
if rg --type rust "pub fn graph_mut\\(" crates/forge-signal/src >/dev/null 2>&1; then
  echo "FAIL: committed graph bypass API found in forge-signal."
  rg --type rust "pub fn graph_mut\\(" crates/forge-signal/src || true
  exit 1
fi

echo "[signal-guards] Enforcing no node-scale BTreeMap hot-path storage..."
if rg --type rust "BTreeMap<NodeId," crates/forge-signal/src/logic crates/forge-signal/src/data >/dev/null 2>&1; then
  echo "FAIL: node-scale BTreeMap storage found in forge-signal hot/runtime paths."
  rg --type rust "BTreeMap<NodeId," crates/forge-signal/src/logic crates/forge-signal/src/data || true
  exit 1
fi

echo "[signal-guards] Enforcing no vector materialization in signal hot paths..."
if rg --type rust "collect::<Vec|to_vec\\(|marked_indices\\(|current_indices\\(" \
  crates/forge-signal/src/logic/evaluation \
  crates/forge-signal/src/logic/invalidation.rs \
  crates/forge-signal/src/logic/transaction/runtime.rs >/dev/null 2>&1; then
  echo "FAIL: hot-path vector materialization found in forge-signal runtime paths."
  rg --type rust "collect::<Vec|to_vec\\(|marked_indices\\(|current_indices\\(" \
    crates/forge-signal/src/logic/evaluation \
    crates/forge-signal/src/logic/invalidation.rs \
    crates/forge-signal/src/logic/transaction/runtime.rs || true
  exit 1
fi

echo "[signal-guards] Enforcing no GC alive snapshot allocation pattern..."
if rg --type rust "alive_snapshot" crates/forge-signal/src/data/graph >/dev/null 2>&1; then
  echo "FAIL: GC alive snapshot allocation pattern found."
  rg --type rust "alive_snapshot" crates/forge-signal/src/data/graph || true
  exit 1
fi

echo "[signal-guards] Enforcing transaction ownership of committed runtime writes..."
if rg --type rust "parent\\.graph\\s*=|parent\\.checkpoint\\s*=|parent\\.event_bus\\s*=" \
  crates/forge-signal/src/logic \
  --glob '!**/transaction/**' >/dev/null 2>&1; then
  echo "FAIL: committed runtime assignment found outside transaction module."
  rg --type rust "parent\\.graph\\s*=|parent\\.checkpoint\\s*=|parent\\.event_bus\\s*=" \
    crates/forge-signal/src/logic \
    --glob '!**/transaction/**' || true
  exit 1
fi

echo "[signal-guards] Running determinism-critical forge-signal tests..."
cargo test -p forge-signal tests::determinism::kv64_parallel_branches_deterministic --quiet
cargo test -p forge-signal logic::events::tests::deterministic_order_independent_of_registration --quiet
cargo test -p forge-signal logic::events::tests::rollback_runs_reverse_order --quiet
cargo test -p forge-signal logic::transaction::tests::begin_commit_applies_staged_state_once --quiet
cargo test -p forge-signal logic::transaction::tests::begin_rollback_preserves_committed_state --quiet
cargo test -p forge-signal logic::transaction::tests::failure_during_event_begin_rewinds_graph --quiet
cargo test -p forge-signal tests::node_conditions::ondemand_blocks_default_evaluate --quiet
cargo test -p forge-signal tests::node_conditions::aspect_filter_skips_unmatched_dirty_aspect --quiet
cargo test -p forge-signal tests::reentrancy::nested_scratch_acquire_returns_structured_error --quiet
cargo test -p forge-signal logic::transaction::tests::node_tier_metadata_is_generation_safe_on_slot_reuse --quiet

echo "[signal-guards] PASS"
