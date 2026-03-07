#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

echo "[signal-perf] Running forge-signal performance lane..."

cargo test -p forge-signal tests::performance::push_perf_10k_nodes --quiet
cargo test -p forge-signal tests::performance::ondemand_defer_perf_10k_nodes --quiet
cargo test -p forge-signal tests::performance::chain_1000_minimal_recomputation --quiet
cargo test -p forge-signal tests::transaction_stress::rollback_heavy_workload_leaves_runtime_consistent --quiet
cargo test -p forge-signal tests::transaction_stress::stress_100k_nodes_transaction_commit -- --ignored --quiet

echo "[signal-perf] PASS"
