#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

echo "[relational-contract-matrix] enforcing contract-tagged test modules"

required_modules=(
  "crates/forge-relational/src/tests/replay_contracts.rs"
  "crates/forge-relational/src/tests/index_contracts.rs"
  "crates/forge-relational/src/tests/lineage_contracts.rs"
  "crates/forge-relational/src/tests/durability_contracts.rs"
)

for module in "${required_modules[@]}"; do
  if [[ ! -f "$module" ]]; then
    echo "FAIL: missing contract test module: $module"
    exit 1
  fi
  if ! rg -q "^// CONTRACT:" "$module"; then
    echo "FAIL: contract tag missing in $module"
    exit 1
  fi
  if ! rg -q "^// LANES:" "$module"; then
    echo "FAIL: lane declaration missing in $module"
    exit 1
  fi
done

echo "[relational-contract-matrix] replay contract lanes"
cargo test -p forge-relational tests::replay_contracts::replay_contract_success_reproduces_canonical_surfaces -- --nocapture
cargo test -p forge-relational tests::replay_contracts::replay_contract_failure_wrong_branch_is_explicit -- --nocapture
cargo test -p forge-relational tests::replay_contracts::replay_contract_failure_missing_parent_chain_is_explicit -- --nocapture
cargo test -p forge-relational tests::replay_contracts::replay_contract_success_preserves_merge_parent_order -- --nocapture

echo "[relational-contract-matrix] derived index contract lanes"
cargo test -p forge-relational tests::index_contracts::derived_index_contract_success_branch_scoped_build_keeps_storage_fallback -- --nocapture
cargo test -p forge-relational tests::index_contracts::derived_index_contract_failure_unknown_index_keeps_truth_reads_correct -- --nocapture

echo "[relational-contract-matrix] lineage contract lanes"
cargo test -p forge-relational tests::lineage_contracts::lineage_contract_correspondence_stays_advisory_until_promoted -- --nocapture
cargo test -p forge-relational tests::lineage_contracts::lineage_contract_failure_invalid_references_do_not_promote -- --nocapture

echo "[relational-contract-matrix] durability contract lanes"
cargo test -p forge-relational tests::durability_contracts::durability_contract_recovery_rebuilds_branch_heads_and_latest_commit -- --nocapture
cargo test -p forge-relational tests::durability_contracts::durability_contract_failure_schema_mismatch_is_explicit -- --nocapture
cargo test -p forge-relational tests::durability_contracts::durability_contract_failure_missing_parent_chain_is_explicit -- --nocapture
cargo test -p forge-relational tests::durability_contracts::durability_contract_recovery_preserves_merge_parent_order -- --nocapture

echo "[relational-contract-matrix] cross-contract determinism / history guards"
cargo test -p forge-relational cross_order_equivalent_mutations_converge -- --nocapture
cargo test -p forge-relational branch_creation_and_branch_targeted_commits_build_a_version_graph -- --nocapture
cargo test -p forge-relational merge_commit_uses_deterministic_parent_order_and_advances_target_branch -- --nocapture
cargo test -p forge-relational merge_commit_requires_existing_parent_branch_heads -- --nocapture
cargo test -p forge-relational merge_inspection_reports_overlapping_authority -- --nocapture
cargo test -p forge-relational merge_commit_rejects_overlapping_authority_since_merge_base -- --nocapture
cargo test -p forge-relational publication_bundle_is_the_single_visible_commit_surface -- --nocapture

echo "[relational-contract-matrix] complexity budgets"
bash scripts/ci/check_relational_complexity_budgets.sh

echo "[relational-contract-matrix] PASS"
