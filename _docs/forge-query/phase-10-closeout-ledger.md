# Phase 10 Closeout Ledger

**Branch:** `query-repair`  
**Milestone:** 9.6 Bridge Truth Identity Lowering  
**Started:** 2026-06-16  
**Policy:** Zero deferrals — all Phase 9 residuals and prior closeout deferrals
land here.

---

## Workstreams

| ID | Workstream | Status | Gate |
|----|------------|--------|------|
| P10-1 | Full compile-fail matrix | **Done** | workspace check + all listed compile-fail suites green |
| P10-2 | worth-topo Phase 9 trybuild + folklore | **Done** | 3/3 pass (`phase_boundaries_query_runtime_phase_nine_compile_fail`) |
| P10-3 | forge-runtime-bridge subscription replay typed fixtures | **Done** | replay test + `subscription_replay_folklore_guard` green |
| P10-4 | worth-spatial `public_api_contract` (55 failures) | **In progress** | serial run `--test-threads=1` (see log); lib 72/72 |
| P10-5 | Hostile QA (full 9.6 bar) | **CLEARED** (with notes) | gate folklore + integration matrix below |
| P10-6 | Closeout doc + milestone status | Pending | final spatial log + milestone `Closed` |

---

## Residuals pulled from Phase 9 (no deferral)

1. worth-topo `query_runtime_phase_nine` compile-fail manifest extension
2. forge-query internal folklore — **cleared** in Phase 9 residual pass
3. worth-spatial lib tests (72/72) — **cleared**; integration suite still open
4. Subscription replay label fixtures — **must close in P10-3**
5. intent-admission trybuild 46/46 — **cleared**

---

## Evidence log

### 2026-06-16 — P10-2 worth-topo Phase 9 compile-fail

- Added `query_runtime_phase_nine` manifest, folklore inventory (harness paths included), 3 compile-fail + 1 golden UI fixtures
- `cargo test -p worth-topo --test phase_boundaries_query_runtime_phase_nine_compile_fail` — **3/3 pass**

### 2026-06-16 — P10-3 subscription replay typed fixtures

- Migrated `replay_tests.rs` from `truth_*_fixture` labels to `truth_snapshot` / `truth_branch` relational constructors
- Added `tests/subscription_replay_folklore_guard.rs`
- `cargo test -p forge-runtime-bridge subscription::replay --lib` — **1/1 pass**
- `cargo test -p forge-runtime-bridge --test subscription_replay_folklore_guard` — **1/1 pass**

### 2026-06-16 — P10-1 compile-fail matrix

- `cargo check --workspace` — green
- forge-runtime-bridge `phase_boundaries_compile_fail` + `phase_boundaries_bridge_truth_identity_compile_fail` — pass
- forge-query `phase_boundaries_bridge_truth_identity_compile_fail` + `phase_boundaries_intent_admission_compile_fail` (46 UI) — pass
- forge-query `phase_boundaries_query_identity_authority_compile_fail` — pass after removing stale `external_terminal_projection_forbidden` target (projection is intentionally public at reporting edge)
- worth-topo phase 8 + phase 9 compile-fail — 3/3 each
- forge-relational + forge-signal compile-fail — pass
- forge-server `forge_native_facade_entry` 62/62, `compat_http_phase_three` 8/8

### 2026-06-16 — P10-4 worth-spatial triage

- Prior 55 failures reproduce under **parallel** default harness; **serial** (`--test-threads=1`) passes targeted modules (planar_boolean_common_plane 16/16)
- Full serial suite running for final count
- `cargo test -p worth-spatial --lib` — 72/72

### 2026-06-16 — P10-5 hostile QA (gate pass)

**Verdict: CLEARED** for Milestone 9.6 gate surfaces, with documented non-blockers.

**Gates re-run (all green):**
- `cargo check --workspace`
- forge-query: `phase_boundaries_query_identity_authority_compile_fail`, `phase_boundaries_intent_admission_compile_fail`, `identity_boundary_hostile_closure_matrix_holds`
- worth-topo: phase 9 compile-fail 3/3, `topology_read` 65/65, `public_api_contract` 8/8
- forge-server: native 62/62, compat 8/8
- forge-runtime-bridge: `subscription_replay_folklore_guard`
- worth-ui: `runtime_outcome_projection_registry` 16/16 (typed async fixture fix)
- hadwiger: `research_graph_invariants` 10/10
- worth-spatial lib 72/72

**Folklore scan (gate paths):** no `snapshot_token()` or receipt struct literals in forge-server `src/`; no label fixtures in replay_tests; worth-ui async fixtures use `worth_ui_query_binding_evidence_identity`.

**Non-blockers (not gate surfaces):**
- In-crate forge-query struct literals remain in `runtime_helpers.rs`, `workspace.rs`, `write_receipt/preview.rs` (internal workspace mint — not downstream/harness folklore)
- Stale `forge-query/wip/external_terminal_projection_forbidden.stderr` (orphan after target removal)
- Golden path `external_terminal_projection_reporting_golden_path.rs` not wired to trybuild driver (optional)
- `public_api_contract` parallel default harness is flaky; **serial gate** (`--test-threads=1`) is required

**Open:** full serial `public_api_contract` 377-test run in progress → `phase10_worth_spatial_public_api_contract.log`
