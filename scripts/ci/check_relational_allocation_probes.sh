#!/usr/bin/env bash
#
# The allocation-slope evidence for runtime.bounded_adjacency.read lives behind
# the `allocation-probes` feature, so an automation lane that forgets the
# feature compiles every probe out and still exits zero. A contract proved only
# by tests nothing runs is not proved. This lane runs them, and refuses to pass
# on an empty selection.
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

echo "[relational-allocation-probes] enforcing executed allocation-slope evidence"

# One selector, used by both the preflight and the run, so the lane cannot
# verify one set of tests and execute another.
SELECTOR="tests::complexity::contracts::substrate_edition_budgets::allocation_slope"
EXPECTED_TESTS=(
  "${SELECTOR}::bounded_adjacency_allocation_is_flat_in_fanout_beyond_the_budget"
  "${SELECTOR}::bounded_adjacency_allocation_is_flat_in_partitions_the_anchor_is_not_in"
  "${SELECTOR}::isolated_bounded_adjacency_fanout_allocation_slope_probe"
  "${SELECTOR}::isolated_bounded_adjacency_partition_allocation_slope_probe"
)

echo "[relational-allocation-probes] preflight: selection under the feature"
listing="$(cargo test -p worth-relational --lib --features allocation-probes "$SELECTOR" -- --list)"
selected="$(printf '%s\n' "$listing" | grep -c ': test$' || true)"

if [[ "$selected" -eq 0 ]]; then
  echo "FAIL: selector '$SELECTOR' matched no test under --features allocation-probes"
  echo "      the slope evidence is compiled out or has been renamed"
  exit 1
fi

for expected in "${EXPECTED_TESTS[@]}"; do
  if ! printf '%s\n' "$listing" | grep -qxF "${expected}: test"; then
    echo "FAIL: missing allocation-slope test: $expected"
    exit 1
  fi
done

if [[ "$selected" -ne "${#EXPECTED_TESTS[@]}" ]]; then
  echo "FAIL: selector '$SELECTOR' matched $selected tests, expected ${#EXPECTED_TESTS[@]}"
  echo "      update EXPECTED_TESTS when the slope family changes"
  exit 1
fi

echo "[relational-allocation-probes] running $selected slope tests"
report="$(cargo test -p worth-relational --lib --features allocation-probes "$SELECTOR" 2>&1)"
printf '%s\n' "$report"

# A filter that matched nothing also prints a passing summary, so the lane
# checks that the tests it selected are the tests that ran.
if ! printf '%s\n' "$report" | grep -qF "${#EXPECTED_TESTS[@]} passed"; then
  echo "FAIL: expected ${#EXPECTED_TESTS[@]} allocation-slope tests to execute"
  exit 1
fi

echo "[relational-allocation-probes] PASS"
