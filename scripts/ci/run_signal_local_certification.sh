#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

PRESET="${1:-full}"
SNAPSHOT_DIR="${RUNNER_TEMP:-/tmp}/signal-local-snapshots"
DETERMINISM_DIR="${RUNNER_TEMP:-/tmp}/signal-local-determinism"

case "$PRESET" in
  web)
    echo "[signal-local-cert] preset=web"
    bash scripts/ci/check_signal_core_profiles.sh
    bash scripts/ci/check_signal_failure_matrix.sh
    bash scripts/ci/check_signal_semantic_snapshots.sh "$SNAPSHOT_DIR"
    ;;
  game-engine)
    echo "[signal-local-cert] preset=game-engine"
    bash scripts/ci/check_signal_core_profiles.sh
    bash scripts/ci/check_signal_failure_matrix.sh
    bash scripts/ci/check_signal_parallel_determinism_cert.sh 2 "$DETERMINISM_DIR"
    ;;
  fintech)
    echo "[signal-local-cert] preset=fintech"
    bash scripts/ci/check_signal_core_profiles.sh
    bash scripts/ci/check_signal_failure_matrix.sh
    bash scripts/ci/check_signal_parallel_determinism_cert.sh 2 "$DETERMINISM_DIR"
    bash scripts/ci/run_signal_perf_lane.sh
    ;;
  kernel)
    echo "[signal-local-cert] preset=kernel"
    bash scripts/ci/check_signal_core_profiles.sh
    bash scripts/ci/check_signal_failure_matrix.sh
    bash scripts/ci/check_signal_parallel_determinism_cert.sh 2 "$DETERMINISM_DIR"
    bash scripts/ci/run_signal_perf_lane.sh
    ;;
  full)
    echo "[signal-local-cert] preset=full"
    bash scripts/ci/check_signal_core_profiles.sh
    bash scripts/ci/check_signal_failure_matrix.sh
    bash scripts/ci/check_signal_parallel_determinism_cert.sh 2 "$DETERMINISM_DIR"
    bash scripts/ci/run_signal_perf_lane.sh
    ;;
  *)
    echo "usage: bash scripts/ci/run_signal_local_certification.sh [web|game-engine|fintech|kernel|full]" >&2
    exit 2
    ;;
esac

echo "[signal-local-cert] PASS"
