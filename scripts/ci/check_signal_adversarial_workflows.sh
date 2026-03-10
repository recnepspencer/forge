#!/usr/bin/env bash

set -euo pipefail

echo "[signal-adversarial] workflow and property certification lane"

cargo test -p forge-signal --lib --features parallel adversarial_properties -- --nocapture
cargo test -p forge-signal --lib --features parallel geometry_kernel_adversarial_seed_matrix_keeps_invariants -- --exact --nocapture
cargo test -p forge-signal --lib --features parallel fintech_adversarial_seed_matrix_keeps_invariants -- --exact --nocapture
cargo test -p forge-signal --lib --features parallel policy_overlap_for_generated_workflows_matches_guaranteed_truth -- --exact --nocapture
cargo test -p forge-signal --lib --features parallel focused_parallel_branch_restore_and_evaluate_dirty_regression -- --exact --nocapture
cargo test -p forge-signal --lib --features parallel parallel_geometry_hostile_session_matches_serial_truth -- --exact --nocapture
cargo test -p forge-signal --lib --features parallel parallel_fintech_hostile_session_matches_serial_truth -- --exact --nocapture

if [[ "${1:-}" == "--long" ]]; then
  cargo test -p forge-signal --lib --features parallel adversarial_workflows -- --ignored --nocapture
fi

echo "[signal-adversarial] lane is green"
