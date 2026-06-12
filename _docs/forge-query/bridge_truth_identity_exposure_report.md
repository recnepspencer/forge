# Bridge Truth Identity Exposure Report

Milestone: 9.6 Phase 2 hard break
Generated: 2026-06-11

## Commands

- `cargo check --workspace`
- `cargo check --workspace --keep-going`
- `cargo test -p forge-runtime-bridge --test phase_boundaries_bridge_truth_identity_compile_fail`

Raw command captures were written to:

- `%TEMP%\forge_query_milestone_9_6_phase2_exposure_raw.txt`
- `%TEMP%\forge_query_milestone_9_6_phase2_exposure_keepgoing_raw.txt`

The normalized committed exposure fixture is:

- `_docs/forge-query/fixtures/bridge_truth_identity_phase2_keepgoing_errors.txt`

## Gate Summary

The workspace is intentionally red.

The bridge facade hard gate made `BridgeIdentity::new(...)` and
`BridgeIdentity::as_str()` crate-internal. That removes the inherited public
string constructor and string accessor from `TruthCommitIdentity`,
`TruthSnapshotIdentity`, `TruthPatchIdentity`, `TruthBranchIdentity`, and the
same bridge identity wrapper family. It also removes public `Display` and
`PartialEq<&str>` implementations from the generic bridge identity wrapper so
received truth identities cannot be formatted or compared as raw text.

The query facade hard gate made the string fields on
`ForgeQueryMutationReceipt`, `ForgeQueryMutationDelta`, and
`ForgeQueryLivePatch` crate-internal, and removed
`snapshot_token(&self) -> String` from `ForgeQueryRuntimeBackend` and
`ForgeQueryRuntimeSourceAdapter`.

## Compile-Fail Boundary Proofs

`forge-runtime-bridge` now has an active trybuild gate:

- `crates/forge-runtime-bridge/tests/phase_boundaries_bridge_truth_identity_compile_fail.rs`
- `crates/forge-runtime-bridge/tests/ui/bridge_truth_identity/truth_commit_identity_string_facade_private.rs`

Result:

- `cargo test -p forge-runtime-bridge --test phase_boundaries_bridge_truth_identity_compile_fail` passes.
- The fixture proves an external caller cannot call `new(...)` on
  `TruthCommitIdentity`, `TruthSnapshotIdentity`, `TruthPatchIdentity`, or
  `TruthBranchIdentity`.
- The fixture proves an external caller cannot call `as_str()` on those truth
  identities.
- The fixture proves an external caller cannot format received truth identities
  through `Display`.
- The fixture proves an external caller cannot compare received truth identities
  to `&str`.

`forge-query` has the matching active trybuild fixture:

- `crates/forge-query/tests/phase_boundaries_bridge_truth_identity_compile_fail.rs`
- `crates/forge-query/tests/ui/bridge_truth_identity/mutation_receipt_string_literal_fields_private.rs`
- `crates/forge-query/tests/ui/bridge_truth_identity/live_patch_string_literal_fields_private.rs`
- `crates/forge-query/tests/ui/bridge_truth_identity/runtime_source_adapter_snapshot_token_removed.rs`
- `crates/forge-query/tests/ui/bridge_truth_identity/runtime_backend_snapshot_token_removed.rs`
- `crates/forge-query/tests/ui/bridge_truth_identity/declaration_initialization_snapshot_str_removed.rs`

That fixture is intentionally not independently runnable while Phase 2 leaves
the workspace red at `forge-relational`, because `forge-query` depends on
`forge-relational`. The fixture is present so the query public struct-literal
gate has a concrete expected compiler contract. Actual execution is a hard
follow-up gate once the compile frontier advances through Phase 3 relational
lawful mint and the Phase 5/6 query adapter/internal impl drift.

The installed query fixtures cover:

- external struct literals for `ForgeQueryMutationReceipt`
- external struct literals for `ForgeQueryMutationDelta`
- external struct literals for `ForgeQueryLivePatch`
- stale `ForgeQueryRuntimeSourceAdapter::snapshot_token(&self) -> String`
  implementation attempts
- stale `ForgeQueryRuntimeBackend::snapshot_token(&self) -> String`
  implementation attempts
- stale declaration-initialization adapter methods that still accept
  `snapshot_token: &str`

## Certification Digests

These digests are SHA-256 hashes of the Phase 2 proof artifacts.
They are mechanically enforced by
`crates/forge-runtime-bridge/tests/phase_boundaries_bridge_truth_identity_digest.rs`.

| Required output | Digest | Inputs |
|-----------------|--------|--------|
| `bridge_truth_identity_compile_fail_boundary_digest` | `9a5244063ba41ec734e5a928349b84462ed7a364544159bdde7fd9d99140fafb` | Bridge trybuild driver, fixture, and stderr |
| `query_receipt_string_field_compile_fail_boundary_digest` | `b55b6c4319374edde7ab82023f7622159780d984be257c959ea824e580126ef9` | Query trybuild driver, receipt/delta fixture, live-patch fixture, and stderr |
| `adapter_snapshot_token_compile_fail_boundary_digest` | `ffc708affc8e10d4a3f873b8ad75e4600cb9e22526cffbc00ed4447b000c6c57` | Source-adapter, backend, and declaration-initialization stale snapshot fixtures and stderr |
| `workspace_red_exposure_digest` | `0b5396e93bc743a90ad78d1c0253a9ec5d5fca3b1aa85f28b59c5b9237e7b48d` | `_docs/forge-query/fixtures/bridge_truth_identity_phase2_keepgoing_errors.txt` |
| `collapse_matrix_cross_check_digest` | `1da47acff387f0e6597578a9d8a59bca276fc4cd5018c75106474504d4a87c55` | `_docs/forge-query/milestone-9.6-bridge-truth-identity-lowering.md` |

## Workspace Error Summary

`cargo check --workspace --keep-going` still stops at the first shared
truth-routing dependency cliff: `forge-relational`. Cargo cannot compile later
crates that depend on `forge-relational`, including `forge-query`, until Phase 3
replaces the relational string mint/parse paths.

Observed error kinds:

| Crate | Error kind | Count | Meaning |
|-------|------------|-------|---------|
| `forge-relational` | `E0624`: private `as_str` method | 17 | Relational bridge consumers were reading bridge truth identities as display text. |
| `forge-relational` | `E0624`: private `new` associated function | 7 | Relational bridge export paths were minting bridge truth identities from formatted text. |
| `forge-relational` | `E0277`: missing `Display` implementation | 1 | Relational error reporting was formatting bridge truth identity as display text. |

## `forge-relational` Error Catalog

| Path | Error | Collapse Matrix row |
|------|-------|---------------------|
| `crates/forge-relational/src/grouped_truth/canonical_digest.rs:16` | private `as_str` | Phase 3 grouped-truth canonical digest |
| `crates/forge-relational/src/grouped_truth/canonical_digest.rs:36` | private `as_str` | Phase 3 grouped-truth canonical digest |
| `crates/forge-relational/src/presentation/bridge/identities.rs:95` | private `new` | Phase 3 relational bridge identities |
| `crates/forge-relational/src/presentation/bridge/identities.rs:104` | private `as_str` | Phase 3 relational bridge identities |
| `crates/forge-relational/src/presentation/bridge/identities.rs:111` | private `as_str` | Phase 3 relational bridge identities |
| `crates/forge-relational/src/presentation/bridge/patch_envelopes.rs:25` | private `new` | Phase 3 patch envelopes |
| `crates/forge-relational/src/presentation/bridge/patch_envelopes.rs:27` | private `new` | Phase 3 patch envelopes |
| `crates/forge-relational/src/presentation/bridge/patch_envelopes.rs:28` | private `new` | Phase 3 patch envelopes |
| `crates/forge-relational/src/presentation/bridge/patch_envelopes.rs:30` | private `new` | Phase 3 patch envelopes |
| `crates/forge-relational/src/presentation/bridge/patch_envelopes.rs:46` | private `as_str` | Phase 3 patch envelopes |
| `crates/forge-relational/src/presentation/bridge/patch_envelopes.rs:59` | private `as_str` | Phase 3 patch envelopes |
| `crates/forge-relational/src/presentation/bridge/runtime_source/branch_heads.rs:15` | private `as_str` | Phase 3 branch heads |
| `crates/forge-relational/src/presentation/bridge/runtime_source/branch_heads.rs:20` | private `as_str` | Phase 3 branch heads |
| `crates/forge-relational/src/presentation/bridge/runtime_source/branch_heads.rs:27` | private `as_str` | Phase 3 branch heads |
| `crates/forge-relational/src/presentation/bridge/runtime_source/committed_patches.rs:16` | private `as_str` | Phase 3 committed patches |
| `crates/forge-relational/src/presentation/bridge/runtime_source/committed_patches.rs:20` | missing `Display` | Phase 3 committed patches |
| `crates/forge-relational/src/presentation/bridge/runtime_source/continuity_lineage.rs:22` | private `as_str` | Phase 3 continuity lineage |
| `crates/forge-relational/src/presentation/bridge/runtime_source/continuity_lineage.rs:59` | private `new` | Phase 3 continuity lineage |
| `crates/forge-relational/src/presentation/bridge/runtime_source/continuity_lineage.rs:79` | private `new` | Phase 3 continuity lineage |
| `crates/forge-relational/src/presentation/bridge/runtime_source/snapshot_authority.rs:27` | private `as_str` | Phase 3 snapshot authority |
| `crates/forge-relational/src/presentation/bridge/runtime_source/snapshot_authority.rs:34` | private `as_str` | Phase 3 snapshot authority |
| `crates/forge-relational/src/presentation/bridge/snapshot_reading.rs:58` | private `as_str` | Phase 3 snapshot reading |
| `crates/forge-relational/src/presentation/bridge/snapshot_reading.rs:66` | private `as_str` | Phase 3 snapshot reading |
| `crates/forge-relational/src/presentation/bridge/snapshot_reading.rs:77` | private `as_str` | Phase 3 snapshot reading |
| `crates/forge-relational/src/presentation/bridge/snapshot_reading.rs:85` | private `as_str` | Phase 3 snapshot reading |

## Cross-Check Against Collapse Matrix

No new unowned path appeared in the compile output. Every surfaced error maps to
an existing Phase 3 Collapse Matrix row.

The report does not claim downstream absence. It records the first hard compile
frontier. Later phases must continue to move the frontier forward and update the
Collapse Matrix as each next ordinary covered surface becomes visible.

## Predicted Blocked Break Appendix

The following break families are blocked behind the `forge-relational`
dependency cliff and are already represented in the Collapse Matrix:

| Crate / slice | Expected break family | Matrix owner |
|---------------|-----------------------|--------------|
| `forge-query` memory workspace and runtime write surfaces | private `commit_identity`, `snapshot_token`, `entity_identity`, and `deltas` fields; stale receipt struct literals | Phase 6 |
| `forge-query` runtime backend/source adapters | stale `snapshot_token(&self) -> String` impl members and call sites | Phase 5 / Phase 6 |
| `forge-query` inspection, intent, signal, and evidence surfaces | receipt identity copied or digested as string text | Phase 7 |
| `worth-topo` query runtime adapters | production receipts and snapshot tokens minted from formatted strings | Phase 8 |
| `forge-query` hostile certification and harnesses | string truth identity constructors, string receipt assertions, and journal suffix parsing | Phase 9 |
| `forge-server` compat/native mutation surfaces | JSON/request/response identity text copied from Query receipts | Phase 9 downstream consumer |
| `hadwiger-research` query harnesses | test write authority receipt string construction | Phase 9 downstream consumer |

## Static Blocked Break Catalog

These concrete `forge-query` paths are not surfaced by Cargo until the
`forge-relational` frontier is repaired, but the Phase 2 gate changes make them
known next-frontier failures:

| Path | Expected error kind | Gate |
|------|---------------------|------|
| `crates/forge-query/src/harness/runtime_api_stabilization/transcript_runtime.rs:127` | `E0407` stale `snapshot_token` trait member | `ForgeQueryRuntimeBackend` |
| `crates/forge-query/src/lower_runtime_routing/certification/surface/fixtures/core.rs:395` | `E0407` stale `snapshot_token` trait member | runtime adapter fixture |
| `crates/forge-query/src/intent_admission/certification/fixtures/runtime.rs:163` | `E0407` stale `snapshot_token` trait member | runtime adapter fixture |
| `crates/forge-query/src/runtime/backend/bridge_backed.rs:177` | `E0599` stale backend/source `snapshot_token` call | adapter trait removal |
| `crates/forge-query/src/runtime/tests/support/stateful_bridge_runtime/backend.rs:250` | `E0407` stale `snapshot_token` trait member | `ForgeQueryRuntimeBackend` |
| `crates/forge-query/src/runtime/tests/support/adapters/schema_and_source.rs:103` | `E0407` stale `snapshot_token` trait member | `ForgeQueryRuntimeSourceAdapter` |
| `crates/forge-query/src/runtime/tests/support/adapters/schema_and_source.rs:148` | `E0407` stale `snapshot_token` trait member | `ForgeQueryRuntimeSourceAdapter` |
| `crates/forge-query/src/runtime/tests/support/adapters/schema_and_source.rs:180` | `E0407` stale `snapshot_token` trait member | `ForgeQueryRuntimeSourceAdapter` |
| `crates/forge-query/src/runtime/runtime_declarations.rs` prior call shape | removed in Phase 2 | declaration initialization `&str` gate |

The query trybuild fixtures cover these error families, but the fixtures cannot
execute until the real crate compiles far enough for trybuild to run.

## Phase 3 Handoff

Phase 3 must start at relational lawful mint. It should not repair worth-topo,
query harnesses, server, or other downstream consumers first. The hard break
has exposed that relational export and relational runtime-source consumption
still depend on bridge identity display text and formatted bridge identity
construction.
