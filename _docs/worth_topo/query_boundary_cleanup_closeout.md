# Worth-Topo Query Boundary Cleanup Closeout

> Historical closeout note: this closeout belongs to the earlier query-boundary
> cleanup wave. The current public topology read-entry contract is now the
> Phase 2 `query_domain` closeout, not the root facade.

## Status

Closed.

The cleanup described in
`_docs/worth_topo/query_boundary_cleanup_audit.md` is implemented and now has a
machine-checkable closeout surface in:

- `crates/worth-topo/src/certification/query_boundary_cleanup_closeout/mod.rs`

That closeout report certifies the five acceptance areas the audit named:

- operator path
- snapshot assembly
- read-view decode
- basis adapter
- public facade

## What Closed

### Phase 1: Operator-Path Admission and Binding Cleanup

Closed behavior:

- product-path topology operators no longer use raw Query row payloads as the
  operational contract for support, bindings, or post-write completion
- typed binding resolution and typed post-write consumption now flow through the
  designated Query runtime boundary instead of `topology_operators/application/*`

Primary landing areas:

- `crates/worth-topo/src/topology_operators/application/*`
- `crates/worth-topo/src/projection/runtime_boundary/query_runtime/operator_bindings.rs`
- `crates/worth-topo/src/projection/runtime_boundary/query_runtime/operator_post_write.rs`

### Phase 2: Query Assembly and Historical Snapshot Hardening

Closed behavior:

- declared query surfaces no longer behave like an ad hoc second Query runtime at the
  entry surface
- current-head and historical snapshot row ownership are concentrated in the
  designated `declared_query_surfaces` seam
- materialization and naming ingestion now flow through narrower typed boundary
  contracts instead of row-shaped ordinary product entry points

Primary landing areas:

- `crates/worth-topo/src/projection/runtime_boundary/declared_query_surfaces/*`
- `crates/worth-topo/src/derived_topology/materialized_graph/query_input.rs`
- `crates/worth-topo/src/projection/truth_surfaces/persistent_naming.rs`

### Phase 3: Read-View Retained-Result Decode Tightening

Closed behavior:

- public read views no longer walk retained Query payload maps directly
- typed neighborhood facts are now produced by the runtime-boundary decode seam
  and consumed by thin domain-view assemblers

Primary landing areas:

- `crates/worth-topo/src/projection/runtime_boundary/read_execution/neighborhood_decode/*`
- `crates/worth-topo/src/projection/read_views/domain/views/*`

### Phase 4: Basis Adapter Hardening and Facade/Export Tightening

Closed behavior:

- basis-mode detection is isolated to the designated runtime contract seam
  instead of leaking through general read-execution code
- the public facade no longer exports the old row-shaped
  `*_from_query_rows` helper family as ordinary product contracts

Primary landing areas:

- `crates/worth-topo/src/projection/runtime_boundary/query_runtime/contracts.rs`
- `crates/worth-topo/src/projection/runtime_boundary/read_execution/basis_context.rs`
- `crates/worth-topo/src/facade.rs`
- `crates/worth-topo/src/certification/public_facade_contracts/*`

## Designated Survivors

These seams remain intentionally, and the cleanup should not be read as a plan
to delete them:

- `crates/worth-topo/src/projection/runtime_boundary/query_runtime/*`
  This remains the designated topology-owned Query runtime adapter subtree.
- `crates/worth-topo/src/projection/runtime_boundary/declared_query_surfaces/*`
  This remains the designated current/historical snapshot surface seam.
- `crates/worth-topo/src/projection/runtime_boundary/read_execution/neighborhood_decode/*`
  This remains the designated retained-result decode seam for typed read facts.
- `crates/worth-topo/src/projection/runtime_boundary/query_runtime/contracts.rs`
  This remains the designated compatibility seam for historical-basis posture
  detection.

These are survivors by design, not leftover debt.

## Deferred Lower-Priority Seams

The original audit also named lower-priority areas that were not the target of
this cleanup wave:

- certification internals that legitimately prove deeper facts than ordinary
  product code
- validation/reference-integrity surfaces that already live closer to the
  lower-runtime boundary
- test support and harness scaffolding that may still tolerate compatibility
  shapes longer than product APIs

Those seams are not implicitly blessed forever. They are simply outside the
scope of this cleanup contract and should be evaluated under their own targeted
specs if they become the next precedent risk.

## Acceptance Evidence

The cleanup is now backed by two layers of evidence:

### Direct phase proofs

- operator boundary tests
- query-assembly boundary tests
- domain-view boundary tests
- runtime posture tests
- public facade and `query_domain` compile-fail tests

### Unified cleanup closeout

- `certify_topology_query_boundary_cleanup_closeout()`
- `TopologyQueryBoundaryCleanupCloseoutReport`
- `TopologyQueryBoundaryCleanupArea`
- `TopologyQueryBoundaryCleanupRow`

The closeout report is intended to answer the audit's core question directly:

> are the dangerous row-shaped and runtime-folklore seams still alive in the
> places the geometry kernel would copy?

For the five named cleanup areas, the answer is now no.

## Verification Snapshot

Representative verification commands that passed during closeout:

- `cargo check -p worth-topo`
- `cargo test -p worth-topo --lib certification::query_boundary_cleanup_closeout::tests::query_boundary_cleanup_closeout_certifies_all_acceptance_areas -- --exact --nocapture`
- `cargo test -p worth-topo --lib certification::query_boundary_cleanup_closeout::tests::query_boundary_cleanup_closeout_names_designated_survivors_for_every_area -- --exact --nocapture`
- `cargo test -p worth-topo topo_public_traced_boundaries_compile_with_envelope_contracts -- --exact --nocapture`
- `cargo test -p worth-topo topo_public_boundary_rejects_internal_runtime_bypass -- --exact --nocapture`

## Net Result

`worth-topo` is now aligned with the important `forge-query 9.3.x` boundary
model in the production-path and public-surface seams this audit was written to
correct.

That means:

- ordinary topology code no longer treats raw Query rows as the normal API
- public topology reads and edits now depend on typed Query facts and typed
  support/admission posture
- historical basis compatibility is isolated to one documented adapter seam
- the public topology surface teaches the post-`9.3.x` seam vocabulary instead
  of the old row-shaped one, with Phase 2 read entry now under `query_domain`

This closes the cleanup spec, not all possible future worth-topo evolution.
