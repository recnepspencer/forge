# Worth-Topo Query Boundary Cleanup Audit

## Purpose

This document captures the post-`forge-query 9.3.x` cleanup work that should
happen in `worth-topo` now that `9.3.6` has landed. `worth-topo` was built before the
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

It also now distinguishes between two very different states that existed at
different points in the Query rewrite:

- older pre-`9.3.x` boundary debt, where topology really did bypass or
  reconstruct Query/runtime behavior locally
- current post-`9.3.6` boundary debt, where topology often does execute through
  Query-owned lowering, admission, and runtime support, but still decodes or
  republishes row-shaped results too low in the stack

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

## Already Closed By Current Query Integration

Before listing the remaining cleanup work, it is important to say what has
already changed in `worth-topo`.

The crate no longer lives in the purely pre-`9.3.x` world. It now has a real
Query-owned domain-read and runtime-support story:

- `TopologyDomainQuery` is the public executed-read boundary for topology reads
- topology read requests now lower through canonical Query lowering rather than
  ad hoc row joins
- the bridge-backed runtime now exposes typed runtime posture, read support,
  edit support, and closeout through `TopologyRuntimeSupport`
- the public read families now share a typed family universe with runtime
  support instead of maintaining separate naming layers

Primary references:

- `crates/worth-topo/docs/domain-reads.md`
- `crates/worth-topo/docs/runtime-support.md`
- `crates/worth-topo/src/projection/runtime_boundary/read_lowering/mod.rs`
- `crates/worth-topo/src/projection/runtime_boundary/read_execution/family_execution.rs`
- `crates/worth-topo/src/projection/runtime_boundary/query_runtime/mod.rs`

That means this audit should not be read as "worth-topo still has no Query-owned
boundary." It should be read as:

- the Query-owned boundary now exists
- some production paths still live on the wrong side of it
- some domain helpers still decode or republish row-shaped results too directly
- some adapter seams are still too hand-wired for the post-`9.3.6` standard

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

### 3. Public topology read views still decode retained Query result payloads too low in the stack

Severity: High

Files:

- `crates/worth-topo/src/projection/read_views/domain/views/adjacency.rs`
- `crates/worth-topo/src/projection/read_views/domain/views/local_rewire.rs`
- `crates/worth-topo/src/projection/read_views/domain/views/loop_cycle.rs`
- `crates/worth-topo/src/projection/runtime_boundary/read_execution/row_decode.rs`

Evidence:

- Shared-vertex, radial, local-rewire, and loop-cycle views all execute through
  `TopologyDomainQuery` and Query-owned lowering/execution first.
- But after the Query-backed read returns, the final domain views still use
  `row_payload`, `relation_identity`, `relation_record_identity`, and
  `relation_identities`.
- The resulting domain views are reconstructed by traversing retained
  `relations` and `relation_identities` payload maps instead of consuming a
  narrower typed read-result contract.

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

- These are public domain read surfaces, and they now do have a real Query
  boundary beneath them.
- The remaining problem is not "no Query integration"; it is that the final
  decode still retells retained payload structure at the domain-view layer.
- This is still a meaningful `9.3.4` cleanup cluster, but it is now narrower
  than the fully pre-`9.3.x` version of the problem.

Expected cleanup direction:

- define typed consumed facts or typed retained-result adapters for
  neighborhood-style reads
- move payload interpretation down into a smaller boundary-owned decode layer
- have the public read views depend on typed fact inputs rather than direct
  `ForgeQueryEntity` payload walking

### 4. Historical-basis execution is now concentrated in an adapter seam, but still hand-builds Query basis contexts

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

- `9.3.2` turns basis into a first-class capability lifecycle.
- The good news is that this logic now lives in a designated
  `projection/runtime_boundary/read_execution` seam rather than being spread
  across public product code.
- The remaining problem is that the adapter still infers historical posture
  from `public_api_contract()` evidence and manually performs historical
  admission, materialization-path resolution, basis binding, and basis
  admission itself.
- So this is no longer "basis leakage everywhere." It is "the designated basis
  adapter is still too hand-wired for the stabilized Query model."

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
- This cluster has become more important now that the public read domain has a
  stronger Query-owned story; the historical/snapshot assembly path stands out
  more clearly as the remaining place where topology still rebuilds too much
  locally.

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
- Over time, topo should consume typed materialization facts or a smaller
  topology-owned boundary adapter contract rather than normalize raw rows
  itself.

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
- This is not hypothetical. The current facade still publicly re-exports
  helpers like `derived_read_diagnostics_from_query_rows`,
  `equivalence_contract_from_diagnostics_rows`,
  `interpreted_topology_from_materialized_rows`,
  `naming_attachment_report_from_query_rows`, and
  `validation_report_from_query_rows`.

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

### Phase 1: Operator-path admission and binding cleanup

Goal:

- stop using raw rows for support inference, binding lookup, and post-write
  materialized decode on the production edit path

Primary targets:

- `topology_operators/application/mod.rs`
- `topology_operators/application/bindings.rs`
- composed edit-lane completion paths

Deliverables:

- typed support/admission consumption for operator families and lanes
- typed binding-resolution contract
- unified post-write topology projection consumption surface

Why first:

- this is still the highest-value remaining production-path cleanup
- it is the strongest example of topology still treating row shape and row
  provenance as the operational contract
- this is the precedent we least want the geometry kernel to copy

### Phase 2: Query assembly and historical snapshot hardening

Goal:

- stop reconstructing current-head and historical topology snapshots too
  aggressively inside topology-owned boundary code

Primary targets:

- `projection/runtime_boundary/query_assembly/mod.rs`
- `projection/runtime_boundary/query_assembly/historical_rows.rs`
- `projection/truth_surfaces/persistent_naming.rs`
- `derived_topology/materialized_graph/mod.rs`

Deliverables:

- smaller row-interpreting snapshot assembly surface
- narrower historical fallback path
- clearer topology-owned boundary contracts for naming, materialization,
  interpreted, validation, and diagnostics surfaces

Why second:

- once the edit path is cleaner, this becomes the next strongest place where
  topology still behaves like a Query sub-runtime
- it is especially important because geometry-kernel snapshot and replay work
  is likely to copy this pattern if we leave it vague

### Phase 3: Read-view retained-result decode tightening

Goal:

- stop public topology read views from decoding retained Query payload maps at
  the domain-view layer

Primary targets:

- `projection/runtime_boundary/read_execution/row_decode.rs`
- `projection/read_views/domain/views/adjacency.rs`
- `projection/read_views/domain/views/local_rewire.rs`
- `projection/read_views/domain/views/loop_cycle.rs`

Deliverables:

- typed consumed facts or typed retained-result adapters for neighborhood and
  loop-cycle surfaces
- read views rewritten to depend on typed fact inputs, not direct
  `ForgeQueryEntity` payload traversal

Why third:

- the crate now already has real Query-owned lowering and execution for public
  domain reads
- that means this is no longer the very first migration to do; it is now the
  cleanup pass that shrinks the remaining decode seam after the bigger product
  boundary problems are fixed

### Phase 4: Basis adapter hardening and facade/export tightening

Goal:

- collapse custom historical-basis logic onto the stabilized Query basis model
- shrink the remaining row-oriented helpers and public exports

Primary targets:

- `projection/runtime_boundary/read_execution/basis_context.rs`
- `projection/runtime_boundary/read_execution/family_execution.rs`
- `facade.rs`

Deliverables:

- thinner basis adapter surface
- less evidence-token and support-matrix inference in topo code
- cleaner public surface that does not normalize old seam shapes

## Expected End State

After this cleanup, `worth-topo` should look different in a few important ways:

- public topology read and edit flows consume typed Query facts and typed
  support/admission surfaces
- historical-basis execution uses a narrow Query basis capability surface
  inside a small documented adapter, instead of custom evidence-driven
  branching leaking outward
- the designated runtime-boundary subtree remains, but ordinary topo code no
  longer acts like it is part of Query runtime internals
- raw row and payload interpretation is concentrated in a small number of
  boundary adapters, not spread across domain views, operator flows, assembly
  paths, and public exports

That is the real payoff of `forge-query 9.3.x` for `worth-topo`: less glue,
less folklore, fewer boundary leaks, and a much cleaner architectural split
between "topology domain behavior" and "Query/runtime boundary plumbing."

## Follow-On Work

Once `9.3.6` finishes, the next useful document should be a concrete migration
spec that turns this audit into implementation slices. A good breakdown would
be:

- operator-path admission/binding cleanup spec
- query-assembly and historical snapshot hardening spec
- read-view retained-result decode tightening spec
- historical-basis boundary hardening spec
- facade/export tightening pass
