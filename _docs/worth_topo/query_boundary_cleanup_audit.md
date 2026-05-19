# Worth-Topo Query Boundary Cleanup Audit

## Purpose

This document captures the post-`forge-query 9.3.x` cleanup work that should
happen in `worth-topo` once `9.3.6` lands. `worth-topo` was built before the
current Query capability and runtime-boundary model stabilized, so a meaningful
slice of its production path still treats raw Query rows, payload maps,
historical-basis internals, and local runtime inference as public contracts.

That was reasonable while Query was still missing those surfaces. It becomes
architectural debt after `9.3.1` through `9.3.6`.

This audit separates:

- production-path violations that should be migrated
- designated runtime-boundary survivors that may remain, but should be trimmed
- lower-priority certification and legacy seams
- a recommended migration order

## Query Capability Baseline

The relevant `forge-query` crate docs and `9.3.x` milestone specs establish the
new expectations:

- `9.3.1` makes inspection and explanation a Query-owned surface.
- `9.3.2` makes basis a capability lifecycle rather than a loose snapshot token.
- `9.3.3` makes effect execution route through admitted, authority-scoped plans.
- `9.3.4` makes projection/materialization consumption a typed fact contract,
  rather than a raw row/payload contract.
- `9.3.5` unifies public admission and support posture around a shared lattice.
- `9.3.6` hardens the lower-runtime seam so product code should not bypass the
  routed Query boundary except inside a clearly designated adapter layer.

For `worth-topo`, the most important consequence is simple:

> raw Query rows are no longer the right ordinary product contract when topo
> actually wants typed facts, admission posture, basis capability, or receipt
> inspection.

## Findings

### 1. Production edit execution still uses raw rows as an operational contract

Severity: High

Files:

- `crates/worth-topo/src/topology_operators/application/mod.rs`

Evidence:

- `TopologyOperatorRunner::apply` reads entity and relation rows directly before
  doing family support and lowering decisions.
- It uses those rows to decide unsupported families and whether composed
  successor or membership programs are admitted.
- It then materializes the derived surface and decodes the first row directly
  into `MaterializedTopologyView`.

Relevant lines:

- `workspace.read(self.assembly.entities())` and
  `workspace.read(self.assembly.relations())`
- unsupported-family and composed-program branching
- `workspace.materialize(self.assembly.materialized())`
- `serde_json::from_value(materialized_rows[0].clone())`

Concrete references:

- `crates/worth-topo/src/topology_operators/application/mod.rs:74`
- `crates/worth-topo/src/topology_operators/application/mod.rs:76`
- `crates/worth-topo/src/topology_operators/application/mod.rs:82`
- `crates/worth-topo/src/topology_operators/application/mod.rs:94`
- `crates/worth-topo/src/topology_operators/application/mod.rs:107`
- `crates/worth-topo/src/topology_operators/application/mod.rs:137`

Why this is now a boundary violation:

- `9.3.4` says topo should not need to infer typed topology facts by walking raw
  query rows.
- `9.3.5` says topo should not be deciding operational support posture from raw
  row shape when Query admission/support surfaces exist.
- The production edit path is still effectively saying "row shape is the API."

Expected cleanup direction:

- replace row-driven support inference with Query-owned support/admission facts
- replace direct row-to-materialized decode with a declared consumed projection
  fact or typed post-write read contract

### 2. Binding lookup is still row archaeology

Severity: High

Files:

- `crates/worth-topo/src/topology_operators/application/bindings.rs`

Evidence:

- Binding helpers repeatedly walk `row.payload["topology"]` and
  `row.payload["lineage"]["provenance"]`.
- Existing entity/relation lookup is done by decoding provenance ids from raw
  payload values.
- Query identity, topology kind, source identity, and target identity are all
  reconstructed manually.

Concrete references:

- `crates/worth-topo/src/topology_operators/application/bindings.rs:19`
- `crates/worth-topo/src/topology_operators/application/bindings.rs:65`
- `crates/worth-topo/src/topology_operators/application/bindings.rs:114`
- `crates/worth-topo/src/topology_operators/application/bindings.rs:160`
- `crates/worth-topo/src/topology_operators/application/bindings.rs:209`
- `crates/worth-topo/src/topology_operators/application/bindings.rs:234`
- `crates/worth-topo/src/topology_operators/application/bindings.rs:289`

Why this is now a boundary violation:

- This is the clearest example of worth-topo depending on raw row layout rather
  than consuming a typed fact contract.
- `9.3.4` exists precisely so product code does not have to decode this kind of
  binding information itself.

Expected cleanup direction:

- introduce a typed binding-consumption surface for entity/relation lookup
- move provenance decoding and row interpretation behind Query-owned or
  topology-boundary-owned fact adapters

### 3. Public topology read views still decode neighborhoods from raw row payloads

Severity: High

Files:

- `crates/worth-topo/src/projection/read_views/domain/views/adjacency.rs`
- `crates/worth-topo/src/projection/read_views/domain/views/local_rewire.rs`
- `crates/worth-topo/src/projection/read_views/domain/views/loop_cycle.rs`
- `crates/worth-topo/src/projection/runtime_boundary/read_execution/row_decode.rs`

Evidence:

- Shared-vertex, radial, local-rewire, and loop-cycle views all use
  `row_payload`, `relation_identity`, `relation_record_identity`, and
  `relation_identities`.
- The resulting domain views are reconstructed by traversing `relations` and
  `relation_identities` payload maps instead of consuming typed read results.

Concrete references:

- `crates/worth-topo/src/projection/runtime_boundary/read_execution/row_decode.rs:8`
- `crates/worth-topo/src/projection/runtime_boundary/read_execution/row_decode.rs:23`
- `crates/worth-topo/src/projection/runtime_boundary/read_execution/row_decode.rs:40`
- `crates/worth-topo/src/projection/read_views/domain/views/adjacency.rs:33`
- `crates/worth-topo/src/projection/read_views/domain/views/adjacency.rs:83`
- `crates/worth-topo/src/projection/read_views/domain/views/adjacency.rs:170`
- `crates/worth-topo/src/projection/read_views/domain/views/local_rewire.rs:32`
- `crates/worth-topo/src/projection/read_views/domain/views/local_rewire.rs:68`
- `crates/worth-topo/src/projection/read_views/domain/views/local_rewire.rs:108`
- `crates/worth-topo/src/projection/read_views/domain/views/loop_cycle.rs:25`
- `crates/worth-topo/src/projection/read_views/domain/views/loop_cycle.rs:48`

Why this is now a boundary violation:

- These are public domain read surfaces. They should consume typed Query-backed
  topology facts, not retell the payload schema.
- This is the strongest `9.3.4` migration cluster in the crate.

Expected cleanup direction:

- define typed consumed facts for neighborhood-style reads
- have read views depend on those consumed facts instead of `ForgeQueryEntity`
  payload walking

### 4. Historical-basis execution still hand-builds Query basis contexts

Severity: Medium-High

Files:

- `crates/worth-topo/src/projection/runtime_boundary/read_execution/basis_context.rs`
- `crates/worth-topo/src/projection/runtime_boundary/read_execution/family_execution.rs`

Evidence:

- `TopologyReadBasisExecutionMode::for_workspace` infers historical posture by
  inspecting `public_api_contract()` support evidence.
- Historical execution path manually performs:
  - historical path admission
  - materialization-path resolution
  - query-basis binding
  - query-basis admission
- Read families are still defined and executed locally with
  `define_read_family(...)`, `execute_read_family(...)`, and
  `execute_read_family_in_basis_context(...)`.

Concrete references:

- `crates/worth-topo/src/projection/runtime_boundary/read_execution/basis_context.rs:20`
- `crates/worth-topo/src/projection/runtime_boundary/read_execution/basis_context.rs:33`
- `crates/worth-topo/src/projection/runtime_boundary/read_execution/basis_context.rs:46`
- `crates/worth-topo/src/projection/runtime_boundary/read_execution/basis_context.rs:61`
- `crates/worth-topo/src/projection/runtime_boundary/read_execution/basis_context.rs:72`
- `crates/worth-topo/src/projection/runtime_boundary/read_execution/basis_context.rs:83`
- `crates/worth-topo/src/projection/runtime_boundary/read_execution/basis_context.rs:92`
- `crates/worth-topo/src/projection/runtime_boundary/read_execution/family_execution.rs:43`
- `crates/worth-topo/src/projection/runtime_boundary/read_execution/family_execution.rs:75`
- `crates/worth-topo/src/projection/runtime_boundary/read_execution/family_execution.rs:113`
- `crates/worth-topo/src/projection/runtime_boundary/read_execution/family_execution.rs:188`

Why this is now a boundary violation:

- `9.3.2` turns basis into a first-class capability lifecycle. Topo should not
  need to infer basis posture from support evidence and hand-wire context
  binding unless it is acting as the designated boundary adapter.
- This code may remain in a thinner adapter form, but its current shape still
  exposes too much prefreeze basis machinery.

Expected cleanup direction:

- collapse basis-mode detection onto a narrower stabilized Query basis surface
- keep any remaining manual admission/binding logic inside a clearly documented
  boundary adapter, not a general read execution layer

### 5. Query assembly snapshot paths still reconstruct too much locally

Severity: Medium

Files:

- `crates/worth-topo/src/projection/runtime_boundary/query_assembly/mod.rs`
- `crates/worth-topo/src/projection/runtime_boundary/query_assembly/historical_rows.rs`
- `crates/worth-topo/src/projection/truth_surfaces/persistent_naming.rs`

Evidence:

- `TopologyQueryAssembly::snapshot` reads rows and materialized surfaces
  directly, then decodes the snapshot locally.
- Historical snapshot support falls back to rebuilding materialized,
  interpreted, validation, diagnostics, and equivalence rows locally when
  materialized surfaces are absent.
- Naming attachment reporting still walks entity and persistent-name rows
  directly.

Concrete references:

- `crates/worth-topo/src/projection/runtime_boundary/query_assembly/mod.rs:153`
- `crates/worth-topo/src/projection/runtime_boundary/query_assembly/mod.rs:172`
- `crates/worth-topo/src/projection/runtime_boundary/query_assembly/mod.rs:176`
- `crates/worth-topo/src/projection/runtime_boundary/query_assembly/mod.rs:181`
- `crates/worth-topo/src/projection/runtime_boundary/query_assembly/mod.rs:191`
- `crates/worth-topo/src/projection/runtime_boundary/query_assembly/mod.rs:205`
- `crates/worth-topo/src/projection/runtime_boundary/query_assembly/historical_rows.rs:31`
- `crates/worth-topo/src/projection/runtime_boundary/query_assembly/historical_rows.rs:37`
- `crates/worth-topo/src/projection/runtime_boundary/query_assembly/historical_rows.rs:45`
- `crates/worth-topo/src/projection/runtime_boundary/query_assembly/historical_rows.rs:63`
- `crates/worth-topo/src/projection/runtime_boundary/query_assembly/historical_rows.rs:81`
- `crates/worth-topo/src/projection/runtime_boundary/query_assembly/historical_rows.rs:100`
- `crates/worth-topo/src/projection/truth_surfaces/persistent_naming.rs:55`

Why this is now a cleanup target:

- Some of this logic may remain topology-owned, but it still shows that worth-topo
  owns row interpretation and derived-surface fallback more aggressively than a
  post-`9.3.x` Query consumer should.
- This is less urgent than the production edit/read paths, but it should be
  narrowed after the first migration wave.

Expected cleanup direction:

- reduce the amount of row decoding that escapes the boundary layer
- convert naming/materialized/interpreted/validation access into clearer typed
  consumption seams

### 6. Materialization still treats raw Query rows as a stable domain input

Severity: Medium

Files:

- `crates/worth-topo/src/derived_topology/materialized_graph/mod.rs`

Evidence:

- `TopologyMaterializer` supports both relational truth and Query rows.
- `materialize_from_query_rows(...)` decodes `ForgeQueryEntity` rows into local
  materialization records before building the domain view.

Concrete references:

- `crates/worth-topo/src/derived_topology/materialized_graph/mod.rs:38`
- `crates/worth-topo/src/derived_topology/materialized_graph/mod.rs:59`

Why this is now a cleanup target:

- The materializer is valid domain logic, but the Query-row entry point keeps
  the raw row schema alive as a domain-facing API.
- Over time, topo should consume typed materialization facts rather than
  normalize raw rows itself.

Expected cleanup direction:

- preserve the domain materializer
- replace the `ForgeQueryEntity` entry point with a typed topology-fact entry
  point where practical

### 7. Composed edit-lane finish paths still decode materialized rows directly

Severity: Medium

Files:

- `crates/worth-topo/src/topology_operators/local_rewrites/boundary_wiring/composed_successor_program.rs`
- `crates/worth-topo/src/topology_operators/local_rewrites/sheet_wire_laminar/membership_programs/mod.rs`

Evidence:

- Both paths inspect receipts correctly, but then materialize and directly
  decode the first `materialized` row afterward.

Concrete references:

- `crates/worth-topo/src/topology_operators/local_rewrites/boundary_wiring/composed_successor_program.rs:71`
- `crates/worth-topo/src/topology_operators/local_rewrites/sheet_wire_laminar/membership_programs/mod.rs:86`

Why this is now a cleanup target:

- These are smaller instances of the broader "row as result contract" pattern.
- Once the main post-write typed projection path exists, these should collapse
  onto it.

Expected cleanup direction:

- route all operator completion paths through the same typed post-write
  projection-consumption surface

### 8. Public facade still exports pre-`9.3.x` seam shapes

Severity: Medium-Low

Files:

- `crates/worth-topo/src/facade.rs`

Evidence:

- The facade exports row-oriented helpers and the full runtime-boundary surface,
  including helpers that explicitly encode "from query rows" behavior.

Concrete references:

- `crates/worth-topo/src/facade.rs:102`
- `crates/worth-topo/src/facade.rs:115`

Why this is now a cleanup target:

- Even after internals improve, the public API can keep teaching downstream code
  to depend on the old seam if these exports remain broad and row-shaped.

Expected cleanup direction:

- review exports after the internal migration
- hide or de-emphasize row-oriented helpers that should no longer be ordinary
  product entry points

## Designated Runtime-Boundary Survivors

Not every lower-runtime contact in `worth-topo` is a bug.

The subtree under:

- `crates/worth-topo/src/projection/runtime_boundary/query_runtime`
- `crates/worth-topo/src/projection/runtime_boundary/bridge`

looks intentionally designated as the topology-owned Query runtime boundary.

Supporting evidence:

- `crates/worth-topo/docs/runtime-support.md` explicitly describes
  `TopologyRuntimeSupport` as the public admission boundary for the
  bridge-backed Query runtime.
- `topology_runtime(...)` wires the Query runtime from bridge, source, schema,
  write, preview, and inspection adapters.

Concrete references:

- `crates/worth-topo/docs/runtime-support.md:3`
- `crates/worth-topo/src/projection/runtime_boundary/query_runtime/mod.rs:37`

Current judgment:

- keep this boundary layer
- do not treat it as ordinary product code
- trim it so it owns only adapter/routing responsibilities
- prevent raw row contracts and lower-runtime assumptions from leaking outward

## Lower-Priority Seams

The audit also found many direct lower-runtime imports in:

- `crates/worth-topo/src/certification/...`
- `crates/worth-topo/src/validation/reference_integrity/...`
- test support and certification harnesses

Those should be reviewed eventually, but they are lower priority than the
production-path cleanup above because:

- certification is allowed to prove deeper facts than ordinary product code
- invariant registration legitimately talks to lower runtime layers
- tests can tolerate compatibility scaffolding longer than product APIs can

## Migration Strategy

### Phase 1: Read-view fact migration

Goal:

- stop public topology read views from decoding raw payload maps

Primary targets:

- `projection/runtime_boundary/read_execution/row_decode.rs`
- `projection/read_views/domain/views/adjacency.rs`
- `projection/read_views/domain/views/local_rewire.rs`
- `projection/read_views/domain/views/loop_cycle.rs`

Deliverables:

- typed consumed Query facts for neighborhood and loop-cycle surfaces
- read views rewritten to depend on typed fact inputs, not `ForgeQueryEntity`
  payload traversal

Why first:

- this removes the broadest row-coupling cluster with the clearest `9.3.4`
  mismatch

### Phase 2: Operator-path admission and binding cleanup

Goal:

- stop using raw rows for support inference, binding lookup, and post-write
  materialized decode

Primary targets:

- `topology_operators/application/mod.rs`
- `topology_operators/application/bindings.rs`
- composed edit-lane completion paths

Deliverables:

- typed support/admission consumption for operator families and lanes
- typed binding-resolution contract
- unified post-write topology projection consumption surface

Why second:

- this is the highest-value production-path cleanup after read views

### Phase 3: Basis and historical execution hardening

Goal:

- collapse custom historical-basis logic onto the stabilized Query basis model

Primary targets:

- `projection/runtime_boundary/read_execution/basis_context.rs`
- `projection/runtime_boundary/read_execution/family_execution.rs`
- `projection/runtime_boundary/query_assembly/historical_rows.rs`

Deliverables:

- thinner basis adapter surface
- less evidence-token and support-matrix inference in topo code
- clearer split between allowed boundary code and ordinary product code

Why third:

- this is important, but it is easier to do cleanly once row-based domain reads
  have already been collapsed

### Phase 4: Assembly and facade tightening

Goal:

- shrink the remaining row-oriented helpers and public exports

Primary targets:

- `projection/runtime_boundary/query_assembly/mod.rs`
- `projection/truth_surfaces/persistent_naming.rs`
- `derived_topology/materialized_graph/mod.rs`
- `facade.rs`

Deliverables:

- smaller row-interpreting boundary layer
- cleaner public surface that does not normalize old seam shapes

## Expected End State

After this cleanup, `worth-topo` should look different in a few important ways:

- public topology read and edit flows consume typed Query facts and typed
  support/admission surfaces
- historical-basis execution uses a narrow Query basis capability surface
  instead of custom evidence-driven branching
- the designated runtime-boundary subtree remains, but ordinary topo code no
  longer acts like it is part of Query runtime internals
- raw row and payload interpretation is concentrated in a small number of
  boundary adapters, not spread across domain views and operator flows

That is the real payoff of `forge-query 9.3.x` for `worth-topo`: less glue,
less folklore, fewer boundary leaks, and a much cleaner architectural split
between "topology domain behavior" and "Query/runtime boundary plumbing."

## Follow-On Work

Once `9.3.6` finishes, the next useful document should be a concrete migration
spec that turns this audit into implementation slices. A good breakdown would
be:

- read-view fact migration spec
- operator-path admission/binding cleanup spec
- historical-basis boundary hardening spec
- facade/export tightening pass
