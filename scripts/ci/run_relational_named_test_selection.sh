#!/usr/bin/env bash
set -euo pipefail

# Enforces one policy for worth-relational CI test lanes: a named test selection
# must reach at least one test that the lane will actually execute, and is then
# executed under that exact same selection. The preflight and the execution
# share a single filter vector, so a lane can never assert one selection and
# run another.
#
# libtest's --list reports every test matching the filter regardless of its
# #[ignore] posture, but an ordinary run executes only the non-ignored ones.
# Listing alone therefore cannot prove executability: a family that became
# entirely #[ignore]d would list as present and then run nothing, reporting
# success. In ordinary posture this script subtracts the --ignored listing from
# the full listing to count what will really run. Under --ignored the listing is
# already exactly the set that executes.
#
# Usage:
#   run_relational_named_test_selection.sh (--lib | --test <target>) \
#     --selection <filter> [--exact] [--ignored] [--features <list>]

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

PACKAGE="worth-relational"
CARGO_TARGET_ARGS=()
FEATURE_ARGS=()
SELECTION_FILTER_ARGS=()
HARNESS_FILTER_ARGS=()
IGNORED_PROBE_ARGS=()
SELECTION=""
TARGET_LABEL=""
FEATURE_LABEL="<none>"
POSTURE_LABEL="<default>"
IGNORED_POSTURE=""

parse_selection_request() {
  local exact="" ignored=""

  while [[ $# -gt 0 ]]; do
    case "$1" in
      --lib)
        [[ -n "$TARGET_LABEL" ]] && fail_usage "--lib and --test are mutually exclusive"
        CARGO_TARGET_ARGS=(--lib)
        TARGET_LABEL="--lib"
        shift
        ;;
      --test)
        [[ -n "$TARGET_LABEL" ]] && fail_usage "--lib and --test are mutually exclusive"
        [[ $# -ge 2 ]] || fail_usage "--test requires a target name"
        CARGO_TARGET_ARGS=(--test "$2")
        TARGET_LABEL="--test $2"
        shift 2
        ;;
      --features)
        [[ $# -ge 2 ]] || fail_usage "--features requires a feature list"
        FEATURE_ARGS=(--features "$2")
        FEATURE_LABEL="$2"
        shift 2
        ;;
      --selection)
        [[ $# -ge 2 ]] || fail_usage "--selection requires a test filter"
        SELECTION="$2"
        shift 2
        ;;
      --exact)
        exact="--exact"
        shift
        ;;
      --ignored)
        ignored="--ignored"
        shift
        ;;
      *)
        fail_usage "unknown argument '$1'"
        ;;
    esac
  done

  [[ -n "$TARGET_LABEL" ]] || fail_usage "one of --lib or --test <target> is required"
  [[ -n "$SELECTION" ]] || fail_usage "--selection <filter> is required"

  # One authoritative selection, built once. Every vector below derives from it,
  # so the preflight, the executability probe and the execution cannot drift.
  SELECTION_FILTER_ARGS=("$SELECTION")
  [[ -n "$exact" ]] && SELECTION_FILTER_ARGS+=("$exact")

  HARNESS_FILTER_ARGS=("${SELECTION_FILTER_ARGS[@]}")
  [[ -n "$ignored" ]] && HARNESS_FILTER_ARGS+=("$ignored")

  # Same target, features, filter and --exact posture; only --ignored is added.
  IGNORED_PROBE_ARGS=("${SELECTION_FILTER_ARGS[@]}" --ignored)

  IGNORED_POSTURE="$ignored"

  POSTURE_LABEL="${exact:+$exact }${ignored}"
  POSTURE_LABEL="${POSTURE_LABEL:-<default>}"
}

fail_usage() {
  echo "FAIL: $1" >&2
  echo "Usage: $(basename "${BASH_SOURCE[0]}") (--lib | --test <target>) --selection <filter> [--exact] [--ignored] [--features <list>]" >&2
  exit 2
}

report_empty_selection() {
  cat >&2 <<EOF
FAIL: selection '$SELECTION' matched 0 compiled tests.
  package : $PACKAGE
  target  : $TARGET_LABEL
  features: $FEATURE_LABEL
  posture : $POSTURE_LABEL
A named CI selection must reach at least one test. The test name, its #[ignore]
posture, or its feature gate changed. Enumerate the real names with:
  cargo test -p $PACKAGE $TARGET_LABEL -- --list
EOF
  exit 1
}

report_non_executable_selection() {
  local selected="$1"
  cat >&2 <<EOF
FAIL: selection '$SELECTION' matched $selected test(s), all of them #[ignore]d.
  package : $PACKAGE
  target  : $TARGET_LABEL
  features: $FEATURE_LABEL
  posture : $POSTURE_LABEL
This lane runs in ordinary posture, which executes only non-ignored tests, so
the selection would report success while executing nothing. Listing cannot
prove executability on its own: --list reports ignored tests too.
Either remove #[ignore] from at least one matching test, or move this selection
to a scheduled lane and pass --ignored so the ignored tests are the ones that
run. Enumerate the real postures with:
  cargo test -p $PACKAGE $TARGET_LABEL -- --list
  cargo test -p $PACKAGE $TARGET_LABEL -- --list --ignored
EOF
  exit 1
}

count_listed_tests() {
  cargo test -p "$PACKAGE" "${CARGO_TARGET_ARGS[@]}" "${FEATURE_ARGS[@]}" \
    -- --list "$@" | grep -cE '^[A-Za-z0-9_:]+: test$' || true
}

assert_selection_is_executable() {
  local selected ignored_selected executable

  selected="$(count_listed_tests "${HARNESS_FILTER_ARGS[@]}")"

  if [[ "$selected" -eq 0 ]]; then
    report_empty_selection
  fi

  if [[ -n "$IGNORED_POSTURE" ]]; then
    # Under --ignored the listing is already exactly the set that executes.
    executable="$selected"
  else
    ignored_selected="$(count_listed_tests "${IGNORED_PROBE_ARGS[@]}")"
    executable=$(( selected - ignored_selected ))

    if [[ "$executable" -le 0 ]]; then
      report_non_executable_selection "$selected"
    fi
  fi

  echo "[relational-selection] '$SELECTION' selects $executable executable test(s) under $TARGET_LABEL"
}

execute_named_selection() {
  echo "[relational-selection] executing '$SELECTION'"
  cargo test -p "$PACKAGE" "${CARGO_TARGET_ARGS[@]}" "${FEATURE_ARGS[@]}" \
    -- "${HARNESS_FILTER_ARGS[@]}" --nocapture --test-threads=1
}

parse_selection_request "$@"
assert_selection_is_executable
execute_named_selection
