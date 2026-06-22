# Phase 9 Discovery Ledger

**Branch:** `query-repair`  
**Slice:** S0–S6 (workspace green + gate verification)  
**Date:** 2026-06-15 (hostile QA pass)

This ledger classifies Phase 9 identity folklore and compile blockers per crate.
Violations are tagged **Blocker** (authority mint/compare), **Harness** (test/cert
fixtures that must migrate), **Terminal** (projection-only; OK if not parsed back),
**CompileFail** (intentional trybuild fixtures), or **API drift** (typed API
surface changed; not folklore but blocks workspace green).

---

## Workspace compile status

| Crate | `cargo check` | Error count | Primary cause |
|-------|---------------|-------------|---------------|
| forge-query | Green | 0 | — |
| worth-topo | Green | 0 | — |
| forge-kernel | Green | 0 | Transitive fixes landed via spatial/topo |
| forge-server | Green | 0 | S2 harness migrated |
| hadwiger-research | Green | 0 | S6 `terminal_projection_for_reporting` migration |
| worth-ui | Green | 0 | S6 public evidence helper + reporting accessors |
| worth-spatial | Green | 0 | S3 certification causal_runtime migrated |

**Workspace:** **green** (`cargo check --workspace` passes).

---

## S0 completed

- Added `forge-query/src/runtime/tests/support/mutation_receipt_support.rs`
- Migrated `runtime/tests/support` adapters + stateful bridge backend to
  `from_authoritative_parts` / `from_bridge_authoritative_parts`

## S1–S2 forge-server

- **Done:** `src/` API drift migration (remask/evidence identity, `snapshot_identity()`)
- **Done:** `schema.rs` uses `RelationalBridgeRecordIdentityParts` via `forge-query` facade
- **Done (S2):** test harness — `runtime_mutation_support` pattern, `current_snapshot_identity()`,
  typed entity targets, `support_evidence_identity`, `admit_preview_basis` label type;
  `forge_native_facade_entry` + `compat_http_phase_three` green; added
  `forge-runtime-bridge` dev-dep for harness relational identity parts

## S3 worth-spatial

- **Done:** `planar_diagnostics/evidence.rs` — `anchor_for_reporting()` / `request_for_reporting()`
- **Done (S3b):** `certification/.../causal_runtime.rs` — typed Truth* mint,
  `from_authoritative_parts` receipt, removed `snapshot_token` adapter,
  `support_evidence_identity`, typed bridge mapping/signal scope constructors
- **Done (S3c):** `local_frame_selection/receipt_test_support` — admission imports
  via `facade::planar_contracts`, `pub(crate)` receipt helper visibility;
  `structure_guard` scoped to production deps + non-certification sources

## S4 worth-topo harness

- **Done:** `public_api_contract` — split entry (`public_api_contract_entry.rs`) +
  lib-test workload-seed module with `crate::facade` imports (8/8 pass)
- **Phase 10 (required):** Phase 9 compile-fail manifest extension — see
  `phase-10-closeout-ledger.md` P10-2; Phase 8 suite remains production guard

## S0b forge-query harness

- **Done:** `runtime/backend/receipts_tests.rs` — `from_authoritative_parts` /
  `from_bridge_authoritative_parts`
- **Done:** `harness/runtime_api_stabilization/transcript_runtime*.rs`
- **Done:** `intent_admission/certification/fixtures/*.rs`
- **Done (residual pass):** intent execution, lower-runtime cert fixtures,
  write_receipt accessors test helper, domain_capabilities DX tests,
  runtime_backend adapter tests — all typed receipt constructors

## S6 hadwiger-research

- **Done:** lib + test `as_str()` → `terminal_projection_for_reporting()`
- **Done:** `research_graph_invariants` integration test (10/10 pass)

## S6 worth-ui

- **Done:** `result_state_for_reporting()`, `result_shape_identity()` projections
- **Done:** `worth_ui_query_binding_evidence_identity` public helper in forge-query
- **Done:** unit tests (2/2 pass)

## forge-query additions (Phase 9 public surface)

- `RelationalBridgeRecordIdentityParts` re-exported from facade
- `worth_ui_query_binding_evidence_identity()` in application/support
- `ValidatedResultShapeArtifact::validated_result_shape_identity()`

---

## Folklore scan results (residual)

### `Truth*Identity::new(` — CompileFail only

| Path | Tag | Notes |
|------|-----|-------|
| `worth-topo/tests/ui/query_runtime_phase_eight/*` | CompileFail | Phase 8 trybuild |
| `forge-runtime-bridge/tests/ui/*` | CompileFail | Bridge truth identity trybuild |

### `ForgeQueryMutationReceipt {` struct literal — cleared

All gate-path struct literals migrated to `from_authoritative_parts` /
`from_bridge_authoritative_parts`. Intentional compile-fail UI fixtures unchanged.

| Path | Tag | Notes |
|------|-----|-------|
| `forge-query/runtime/intent/execution.rs` | **Cleared** | idempotent_noop + invariant_violation |
| `forge-query/lower_runtime_routing/certification/surface/fixtures/core.rs` | **Cleared** | representative write/signal fixtures |
| `forge-query/runtime/surface/mutation/write_receipt/accessors.rs` | **Cleared** | test_only preview helper |
| `forge-query/domain_capabilities/dx/lower_runtime_tests.rs` | **Cleared** | write_authority_boundary_source |
| `forge-query/lower_runtime_routing/adapters/runtime_backend/tests.rs` | **Cleared** | boundary envelope test |

Residual struct literals remain only in compile-fail UI fixtures under `forge-query/tests/ui/`.

### `snapshot_token(` — cleared in gate surfaces

No open gate-path `snapshot_token()` adapters in forge-server tests or worth-spatial
certification causal harness.

---

## Gate checklist (Phase 9 sign-off target)

| Gate | Status |
|------|--------|
| `cargo check --workspace` | **Met** |
| forge-query harness support typed receipts | **Met (S0 + S0b)** |
| forge-server compat + native facade tests | **Met** (`compat_http_phase_three` 8/8, `forge_native_facade_entry` 62/62) |
| worth-topo topology_reads | **Met** (65/65 pass via `--lib topology_read`) |
| worth-topo `public_api_contract` | **Met** (8/8 pass) |
| worth-spatial `cargo test --lib` | **Met** (72/72 pass; local-frame receipt drift + structure_guard aligned) |
| hadwiger research_graph_invariants | **Met** (10/10 pass) |
| worth-ui query binding evidence | **Met** (2/2 unit tests pass) |
| Phase 9 compile-fail per crate | **Met** — `intent_admission_dx_boundaries_hold` 46/46 pass |
| Hostile QA CLEARED | **CLEARED** |

---

## Hostile QA verdict: CLEARED

**Resolved blockers (this pass):**

1. **forge-server S2:** harness typed receipts, `current_snapshot_identity`, entity
   identity tokens, inspection/snapshot parity in `direct_mutation` / `direct_projection`.
2. **worth-spatial causal_runtime:** typed Truth* mint, `from_authoritative_parts`,
   removed folklore adapters.
3. **worth-topo `public_api_contract`:** harness import split (integration vs lib-test).
4. **forge-query S0b:** struct-literal receipts migrated in receipts_tests,
   transcript_runtime, intent_admission fixtures.
5. **worth-spatial local-frame certification:** receipt test support import/visibility
   drift fixed; `cargo test -p worth-spatial --lib` green (72/72).
6. **forge-query internal folklore:** remaining struct literals in intent execution,
   lower-runtime fixtures, accessors, DX tests migrated to typed constructors.
7. **intent-admission trybuild:** `phase_boundaries_intent_admission_compile_fail` 46/46 pass.

**Pulled into Phase 10 (zero-deferral policy):**

- worth-topo Phase 9 compile-fail manifest extension → P10-2
- worth-spatial `public_api_contract` integration failures → P10-4
- forge-runtime-bridge subscription replay label fixtures → P10-3
- Full compile-fail matrix + hostile QA + closeout doc → P10-1, P10-5, P10-6

**Cleared surfaces:**

- All production lib crates compile green under Phase 9 typed identity API.
- Core gate proofs: topology_reads (65), research_graph_invariants (10), worth-ui evidence (2),
  forge-server native/compat facade tests.
- No `snapshot_token()` in forge-server production `src/` or gate harness paths.
