#!/usr/bin/env bash
set -euo pipefail

# Enforces one policy for worth-relational CI test lanes: a named test selection
# must reach at least one compiled test, and is then executed under that exact
# same selection. The preflight and the execution share a single filter vector,
# so a lane can never assert one selection and run another.
#
# Usage:
#   run_relational_named_test_selection.sh (--lib | --test <target>) \
#     --selection <filter> [--exact] [--ignored] [--features <list>]

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

PACKAGE="worth-relational"
CARGO_TARGET_ARGS=()
FEATURE_ARGS=()
HARNESS_FILTER_ARGS=()
SELECTION=""
TARGET_LABEL=""
FEATURE_LABEL="<none>"
POSTURE_LABEL="<default>"

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

  # One filter vector, built once, used by both the preflight and the execution.
  HARNESS_FILTER_ARGS=("$SELECTION")
  [[ -n "$exact" ]] && HARNESS_FILTER_ARGS+=("$exact")
  [[ -n "$ignored" ]] && HARNESS_FILTER_ARGS+=("$ignored")

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

assert_selection_is_non_empty() {
  local listing selected

  listing="$(cargo test -p "$PACKAGE" "${CARGO_TARGET_ARGS[@]}" "${FEATURE_ARGS[@]}" \
    -- --list "${HARNESS_FILTER_ARGS[@]}")"

  selected="$(printf '%s\n' "$listing" | grep -cE '^[A-Za-z0-9_:]+: test$' || true)"

  if [[ "$selected" -eq 0 ]]; then
    report_empty_selection
  fi

  echo "[relational-selection] '$SELECTION' selects $selected test(s) under $TARGET_LABEL"
}

execute_named_selection() {
  echo "[relational-selection] executing '$SELECTION'"
  cargo test -p "$PACKAGE" "${CARGO_TARGET_ARGS[@]}" "${FEATURE_ARGS[@]}" \
    -- "${HARNESS_FILTER_ARGS[@]}" --nocapture --test-threads=1
}

parse_selection_request "$@"
assert_selection_is_non_empty
execute_named_selection
