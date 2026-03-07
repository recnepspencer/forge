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

echo "[signal-guards] PASS"
