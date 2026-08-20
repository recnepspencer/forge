#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
RELATIONAL_ROOT="$ROOT_DIR/crates/worth-relational/src"
RAW_REQUESTS="$RELATIONAL_ROOT/merge/data/requests/raw.rs"
HISTORY_STATE="$RELATIONAL_ROOT/runtime/state/subsystems/history.rs"

failures=0

check_absent() {
  local label="$1"
  local pattern="$2"
  shift 2
  if rg -n -w "$pattern" "$@" >/tmp/worth-relational-phase4-residue.txt 2>&1; then
    echo "[relational-phase4-residue] FAIL: $label" >&2
    cat /tmp/worth-relational-phase4-residue.txt >&2
    failures=1
  fi
}

check_absent "retired combined CommitReference remains in production" "CommitReference" "$RELATIONAL_ROOT"
check_absent "retired ExpectedBranchHead remains in production" "ExpectedBranchHead" "$RELATIONAL_ROOT"
check_absent "retired admitted branch-basis lane remains in production" "AdmittedRelationalBranchBasis" "$RELATIONAL_ROOT"
check_absent "retired generic branch-authority readmission remains in production" "admit_relational_branch_observation" "$RELATIONAL_ROOT"
if rg -n "pub fn (snapshot_for_branch|admit_execution_basis)\(" "$RELATIONAL_ROOT" --glob '*.rs' >/tmp/worth-relational-phase4-residue.txt 2>&1; then
  echo "[relational-phase4-residue] FAIL: raw branch visibility/currentness door remains public" >&2
  cat /tmp/worth-relational-phase4-residue.txt >&2
  failures=1
fi
if rg -n "pub fn (snapshot|snapshot_for_identity)\(" "$RELATIONAL_ROOT/visibility/authority.rs" >/tmp/worth-relational-phase4-residue.txt 2>&1; then
  echo "[relational-phase4-residue] FAIL: transitional visibility projection remains public" >&2
  cat /tmp/worth-relational-phase4-residue.txt >&2
  failures=1
fi
if rg -n "pub fn (pin_snapshot|admit_execution_basis_for_identity)\(" "$RELATIONAL_ROOT/visibility/authority.rs" >/tmp/worth-relational-phase4-residue.txt 2>&1; then
  echo "[relational-phase4-residue] FAIL: later visibility lease authority remains public" >&2
  cat /tmp/worth-relational-phase4-residue.txt >&2
  failures=1
fi
if rg -n "admit_application_commit|admit_execution_basis_for_identity|admit_truth_view_execution_basis|project_version|retain_version_for_replay|historical_snapshot|historical_branch_head|historical_merge_branch_basis|replay_(commit|range)|replay_authority|(^|[^[:alnum:]_])(self|world|replay_api|recovery_api|runtime|[[:alnum:]_]*runtime|world\.runtime)\.replay[[:space:]]*\(" "$ROOT_DIR/crates/worth-relational/tests/relational_certification" --glob '*.rs' --glob '!**/phase4_compatibility.rs' >/tmp/worth-relational-phase4-residue.txt 2>&1; then
  echo "[relational-phase4-residue] FAIL: Phase-4 Supply Chain certification imported a later compatibility authority" >&2
  cat /tmp/worth-relational-phase4-residue.txt >&2
  failures=1
fi

if rg -n "test_for_(main_branch|branch)|with_test_|with_owner_binding|legacy_branch_binding\(" \
  "$RELATIONAL_ROOT/transactions/data/options.rs" \
  "$RELATIONAL_ROOT/transactions/runtime_entry.rs" \
  "$RELATIONAL_ROOT/runtime/state/runtime_state/core_access.rs" \
  "$RELATIONAL_ROOT/branch/authority.rs" >/tmp/worth-relational-phase4-residue.txt 2>&1; then
  echo "[relational-phase4-residue] FAIL: transaction authority still has a test-only synthetic lane" >&2
  cat /tmp/worth-relational-phase4-residue.txt >&2
  failures=1
fi
check_absent "ambient-main transaction helper remains" "main_transaction_options" "$RELATIONAL_ROOT"
if rg -n "\bbranch_heads[[:space:]]*:[[:space:]]*BTreeMap" "$RELATIONAL_ROOT" --glob '*.rs' --glob '!**/tests/**' >/tmp/worth-relational-phase4-residue.txt 2>&1; then
  echo "[relational-phase4-residue] FAIL: legacy branch-head map remains in production" >&2
  cat /tmp/worth-relational-phase4-residue.txt >&2
  failures=1
fi
check_absent "retired BranchHead projection remains" "struct BranchHead" "$RELATIONAL_ROOT"
check_absent "retired VersionGraphSnapshot projection remains" "struct VersionGraphSnapshot" "$RELATIONAL_ROOT"
if test -e "$RELATIONAL_ROOT/history/authority/commit_publication.rs"; then
  echo "[relational-phase4-residue] FAIL: retired broad HistoryAuthority publication module remains" >&2
  failures=1
fi

if rg -n "pub\(crate\) fn (publish_commit|publish_metadata_artifact)" "$RELATIONAL_ROOT/history/authority" --glob '*.rs' >/tmp/worth-relational-phase4-residue.txt 2>&1; then
  echo "[relational-phase4-residue] FAIL: broad HistoryAuthority publication door remains" >&2
  cat /tmp/worth-relational-phase4-residue.txt >&2
  failures=1
fi

if ! test -f "$RELATIONAL_ROOT/mvcc/publication/authority.rs"; then
  echo "[relational-phase4-residue] FAIL: dedicated MVCC publication authority is missing" >&2
  failures=1
fi

if rg -n "pub fn create_branch" "$RELATIONAL_ROOT/history" >/tmp/worth-relational-phase4-residue.txt 2>&1; then
  echo "[relational-phase4-residue] FAIL: raw public branch creation remains" >&2
  cat /tmp/worth-relational-phase4-residue.txt >&2
  failures=1
fi

if rg -n "pub fn fork_branch_from" "$RELATIONAL_ROOT" --glob '*.rs' >/tmp/worth-relational-phase4-residue.txt 2>&1; then
  echo "[relational-phase4-residue] FAIL: raw public fork selector remains" >&2
  cat /tmp/worth-relational-phase4-residue.txt >&2
  failures=1
fi

if rg -n "derive\([^)]*(Serialize|Deserialize)|impl Default" "$RELATIONAL_ROOT/transactions/data/options.rs" >/tmp/worth-relational-phase4-residue.txt 2>&1; then
  echo "[relational-phase4-residue] FAIL: TransactionOptions regained wire/default construction" >&2
  cat /tmp/worth-relational-phase4-residue.txt >&2
  failures=1
fi

if rg -n -P "#\[cfg\(not\(test\)\)\][\\r\\n]+\\s*pub\\s+(target_branch|source_branch|merge_intent):" "$RAW_REQUESTS" >/tmp/worth-relational-phase4-residue.txt 2>&1; then
  echo "[relational-phase4-residue] FAIL: raw merge selector is public outside cfg(test)" >&2
  cat /tmp/worth-relational-phase4-residue.txt >&2
  failures=1
fi

population_iteration_count="$(rg -n "branch_cells\.(values|keys)" "$HISTORY_STATE" | wc -l | tr -d ' ')"
if [[ "$population_iteration_count" != "1" ]]; then
  echo "[relational-phase4-residue] FAIL: branch population iteration escaped the instrumented boundary (found $population_iteration_count sites)" >&2
  rg -n "branch_cells\.(values|keys)" "$HISTORY_STATE" >&2 || true
  failures=1
fi

if rg -n "branch_cells\.(iter|iter_mut|into_iter)" "$HISTORY_STATE" >/tmp/worth-relational-phase4-residue.txt 2>&1; then
  echo "[relational-phase4-residue] FAIL: branch population iteration bypassed the named boundary" >&2
  cat /tmp/worth-relational-phase4-residue.txt >&2
  failures=1
fi

if rg -n "pub\(crate\) fn append\(" "$RELATIONAL_ROOT/history/commit/catalog.rs" >/tmp/worth-relational-phase4-residue.txt 2>&1; then
  echo "[relational-phase4-residue] FAIL: catalog artifact append door is wider than its envelope boundary" >&2
  cat /tmp/worth-relational-phase4-residue.txt >&2
  failures=1
fi

materialization_sites="$(rg -n "RelationalCommitArtifact::from_envelope" "$RELATIONAL_ROOT" --glob '*.rs' | wc -l | tr -d ' ')"
if [[ "$materialization_sites" != "1" ]]; then
  echo "[relational-phase4-residue] FAIL: artifact materialization has $materialization_sites production sites (expected the catalog boundary only)" >&2
  rg -n "RelationalCommitArtifact::from_envelope" "$RELATIONAL_ROOT" --glob '*.rs' >&2 || true
  failures=1
fi

counter_sites="$(rg -n "materializations\.fetch_add" "$RELATIONAL_ROOT/history/commit/catalog.rs" | wc -l | tr -d ' ')"
if [[ "$counter_sites" != "1" ]]; then
  echo "[relational-phase4-residue] FAIL: catalog materialization accounting has $counter_sites construction sites (expected one)" >&2
  rg -n "materializations\.fetch_add" "$RELATIONAL_ROOT/history/commit/catalog.rs" >&2 || true
  failures=1
fi

rm -f /tmp/worth-relational-phase4-residue.txt

if (( failures != 0 )); then
  exit 1
fi

echo "[relational-phase4-residue] PASS: retired branch authority and later currentness lease doors are absent; Phase-4 projections are private and exact-cell derived, while bounded compatibility reads stay outside certification"
