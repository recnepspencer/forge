# Touched Graph Milestone 5: Query Obligation Selection Migration

> **Status:** Draft
>
> **Purpose:** test the strangler-migration format for Worth by replacing
> Worth-local graph obligation selection paths with a parallel Query-owned
> selection substrate, migrating one vertical lane at a time, proving parity,
> and deleting or capping the old selector residue.

## Goal

Milestone 5 freezes Query obligation selection as the ordinary consumer of
topology touched graph basis products and spatial Query descriptors.

The milestone does not rebuild the kernel from scratch. It builds a new
Query-owned selection path beside the current Worth-local selection/adoption
surfaces, routes real touched authority products through it, proves parity where
the old path is trusted, and then deletes or caps the old selector folklore.

By the end of this milestone:

- topology touched graph basis and spatial Query descriptors can select Query
  graph obligations through Query-owned selector semantics
- primitive construction remains the first vertical migration lane and becomes
  the reference shape for later topology and boolean lanes
- Worth-local selector copies, broad collection-only selector shortcuts,
  lifecycle-only selector shortcuts, local ceremony audits, and fabricated
  execution proof are deleted or mechanically capped
- selection counters prove attempted buckets, matches, deduplication,
  rejection, selected obligations, and execution-backed adoption breadth
- Query-gap rows explain selector expressiveness still missing from Query rather
  than letting Worth keep a parallel selector engine

Milestone 5 does **not** close Query graph-read access planning. Milestones 6
through 8 own graph-read inventory, declaration, and access-plan adoption.

## Why This Milestone Exists

Milestone 4 made spatial evidence touch authority real, but selected Query graph
obligations are still unevenly distributed across existing Worth construction,
topology operator, spatial adoption, runtime boundary, and certification
surfaces.

The dangerous version of Milestone 5 would patch those surfaces in place. That
would let old selector assumptions, local support rows, in-memory adoption
shortcuts, broad collection selectors, and topology/spatial laundering survive
as future dependencies.

This milestone instead uses a strangler migration: keep the current path working
while a parallel Query-owned path is built, migrate one vertical lane, prove
equivalence and stronger denials, then delete or cap the old path.

## Governing Summaries

- `MENTALITY.md`: protects adversarial-constraint-first foundation work. This
  milestone must solve selector authority before later validation,
  invalidation, replay, cache, and diagnostic milestones depend on selected
  obligations.
- `arch_laws.md`: protects proof-bearing phase transitions. Touched authority
  products must lower into Query obligation registration, selector coverage,
  support posture, execution proof, and adoption proof without weaker products
  promoting themselves.
- `composition_laws.md`: protects responsibility-owned files. The migration
  must not create god certification files, broad helper modules, or
  provenance-named `phase_five` buckets.
- `domain_structure_laws.md`: protects physical ownership. Query-owned
  selection, Worth topology descriptors, spatial descriptors, kernel closeout,
  certification, and deletion residue need visible homes.
- `perf_laws.md`: protects semantic-delta-bounded work. Selection breadth must
  scale with touched descriptor breadth and selector bucket precision, not with
  global topology size or broad scan rediscovery.
- `touched-graph-roadmap.md`: places this milestone immediately after spatial
  touch authority and before graph-read access planning, because selected
  obligations are the proof product later graph reads, validators,
  invalidation, replay, conflict, and diagnostics consume.

## Adversarial Constraint

Equivalent topology touched graph basis, equivalent spatial Query descriptor,
equivalent operating world, and equivalent registered obligation catalog must
converge to the same selected Query graph obligation set, execution-backed
adoption proof, support posture, counters, residue manifest, and closeout
identity.

Worth-local selector forks, broad collection-only matches, lifecycle-only
matches, local support matrices, in-memory adoption presented as execution
proof, fabricated selector coverage, copied descriptor fields, topology/spatial
authority substitution, and unowned selector residue must fail closed or appear
as explicit capped residue with owner, cap, blocker, and removal trigger.

No later milestone may need to preserve a Worth-local graph obligation selector
after its vertical lane has a Query-owned selected-obligation proof.

## Product Decision Lock

- Use the strangler migration format for this milestone.
- Preserve current topology and spatial source truth while introducing the new
  Query-owned selection substrate beside existing adoption surfaces.
- Query owns selector matching, selected obligation identity, support posture,
  execution proof, adoption proof, and selector counters.
- Worth owns descriptor construction from its authority products, public
  workflow composition, and closeout pressure.
- Primitive construction is the first migrated vertical lane because it already
  has graph-obligation adoption surfaces and real touched-basis construction.
- Spatial touch authority is the second proof input because Milestone 4 closed
  its authority boundary but did not close Query obligation selection.
- Deletion is preferred over residue. Residue is allowed only with owner, cap,
  blocker, removal trigger, and certification preventing growth.

## Phase Plan

### Phase 1: Query Selection Boundary Inventory

This phase freezes the migration map before selector behavior changes. Every
existing graph-obligation registration, selector coverage surface, local
ceremony audit, support pin, runtime registration site, selected-count proof,
and Query adoption closeout surface is classified as source descriptor,
Query-owned selection, migration projection, certification-only support,
deletion target, capped residue, or Query-gap.

**Relevant subsystems**
- `crates/worth-kernel/src/construction/graph_obligation_adoption`
- `crates/worth-kernel/src/construction/query_authority`
- `crates/worth-topo/src/construction/query_native_boundary`
- `crates/worth-topo/src/topology_operators`
- `crates/worth-spatial/src/query_adoption`
- `crates/worth-spatial/src/workload_platform/evidence_ledger/spatial_touch_admission`
- `crates/forge-query/src/consumer_kit/graph_obligation_adoption`

**Relevant APIs**
- `graph_obligation_consumer_kit(...)`
- `ForgeQueryGraphObligationRegistration`
- `ForgeQueryGraphObligationSelectorCoverageDeclaration`
- `ForgeQueryGraphObligationSupportPin`
- `ForgeQueryGraphObligationLocalCeremonyAudit`
- `ForgeQueryGraphObligationResidueManifest`
- `TopologyPrimitiveConstructionBirthDeclaredTouchedBasis`
- `SpatialEvidenceQueryTouchDescriptor`

**Warnings**
- Do not start by adding a new selector helper. Start by proving every existing
  selector-looking surface is classified.
- Do not classify local selector code as compatibility unless the Query-owned
  replacement surface, owner, cap, and removal trigger are named.
- Do not let certification inventories become production inputs.

**Test requirements**
- `query_selection_inventory_covers_every_graph_obligation_surface`: every
  Worth file exporting graph-obligation registration, selector coverage,
  support pin, local ceremony audit, adoption proof, selected count, or graph
  obligation envelope digest appears in the typed inventory exactly once.
- `unclassified_selector_surface_fails_inventory`: adding a public or
  production-reachable selector, registration, local audit, or support-row
  surface without classification fails the certification test.
- `inventory_rejects_in_memory_adoption_as_execution_proof`: inventory rows
  that mark in-memory proof as final selected-obligation execution proof fail.

**Engineering decisions**
- The inventory lives in certification or closeout support, not in the
  production selector path.
- Inventory rows must include source path, exported facade path if any,
  classification, current caller, deletion action, owner, cap, blocker, and
  removal trigger.

**Open questions**
- None.

### Phase 2: Parallel Query Selection Substrate

This phase creates the new selection home beside current surfaces without
switching public callers yet. The substrate accepts only touched authority
products or spatial Query descriptors as input and returns Query-owned
selection/adoption proof products.

**Relevant subsystems**
- `crates/worth-kernel/src/query_obligation_selection/`
- `crates/worth-kernel/src/certification/public_facade_contracts`
- `crates/worth-topo/src/construction/query_native_boundary`
- `crates/worth-spatial/src/query_adoption`
- `crates/forge-query/src/consumer_kit/graph_obligation_adoption`

**Relevant APIs**
- `graph_obligation_consumer_kit(...)`
- `ForgeQueryGraphObligationConsumerRegistrationDeclaration::for_runtime_family(...)`
- `ForgeQueryGraphObligationSelectorCoverageDeclaration::required(...)`
- `ForgeQueryGraphObligationSupportPin`
- `prove_execution_with(...)`
- `prove_adoption_with_execution()`

**Warnings**
- Do not name the folder `v2`, `new`, `migration`, or `phase_five`; name it for
  the responsibility it will keep if the old path disappears.
- Do not accept raw ids, raw strings, mutation rows, copied touch fields, local
  support rows, or in-memory proof as selector authority.
- Do not build a Worth selector engine. Worth may assemble declarations and
  descriptors; Query selects.

**Test requirements**
- `parallel_selection_substrate_accepts_only_touched_authority_inputs`: the
  new entrypoints require topology touched basis proof or spatial Query
  descriptor and reject raw or copied substitutes.
- `parallel_selection_substrate_returns_execution_backed_query_proof`: the
  positive path returns a `ForgeQueryGraphObligationExecutionBackedAdoptionProof`
  with nonempty execution rows, manifest digest, and selected-obligation count.
- `local_selector_copy_cannot_satisfy_selection_substrate`: a fake Worth-local
  selected row, local support row, or copied descriptor cannot construct the
  selected-obligation proof type.

**Engineering decisions**
- `worth-kernel` owns migration orchestration and closeout pressure for the
  first parallel substrate because it consumes both topology and spatial proof
  products.
- `worth-topo` and `worth-spatial` own descriptor construction from their
  authority products; they do not own Query selection semantics.

**Open questions**
- Whether the final public selection facade belongs in `worth-kernel` or stays
  as crate-local closeout until graph-read access planning closes.

### Phase 3: Primitive Construction Vertical Lane Migration

This phase migrates the existing primitive-construction graph obligation lane
onto the parallel substrate first. It is the lowest-risk real lane because it
already has touched-basis construction, graph-obligation registrations,
selected-count evidence, and certification tests.

**Relevant subsystems**
- `crates/worth-kernel/src/construction/graph_obligation_adoption`
- `crates/worth-kernel/src/construction/query_authority`
- `crates/worth-topo/src/construction/query_native_boundary/compose_execution`
- `crates/worth-topo/src/projection/runtime_boundary/query_runtime`

**Relevant APIs**
- `primitive_construction_graph_obligation_catalog()`
- `primitive_construction_graph_obligation_adoption_proof()`
- `primitive_construction_graph_obligation_selector_coverage()`
- `primitive_construction_graph_obligation_support_pin()`
- `topology_primitive_construction_birth_graph_obligation_registration(...)`
- `TopologyPrimitiveConstructionBirthDeclaredTouchedBasis`

**Warnings**
- Do not delete primitive-construction adoption before the parallel lane proves
  parity.
- Do not preserve primitive construction as a special case after the general
  selected-obligation proof exists.
- Do not count selected obligations by reading display strings or receipt text.

**Test requirements**
- `primitive_construction_selection_parity_survives_parallel_migration`: for
  the supported primitive construction families, old and new lanes agree on
  selected count, registration identities, support posture, execution digest,
  and graph obligation envelope digest where the old lane is trusted.
- `primitive_construction_local_selector_residue_is_deleted_or_capped`: any
  old primitive construction selector, support row, local ceremony source, or
  adoption helper not consumed by the parallel lane is either deleted or appears
  in one capped residue row.
- `primitive_construction_replay_preserves_selected_obligation_identity`:
  repeated construction touched-basis inputs produce identical Query selection
  and execution-backed adoption identity.

**Engineering decisions**
- Primitive construction becomes the reference vertical lane for later topology
  operator families, not a permanent one-off.
- Parity is allowed only against trusted old outputs; places where the new
  Query path denies more strongly are recorded as intended hardening.

**Open questions**
- None.

### Phase 4: Spatial Touch Descriptor Selection Lane

This phase connects Milestone 4 spatial authority to Query obligation selection.
The spatial path must consume `SpatialEvidenceQueryTouchDescriptor` produced
from sealed spatial touch authority and lookup proof, not raw evidence rows or
topology touched basis substitutes.

**Relevant subsystems**
- `crates/worth-spatial/src/workload_platform/evidence_ledger/spatial_touch_admission`
- `crates/worth-spatial/src/query_adoption`
- `crates/worth-spatial/src/facade/workload_vocabulary`
- `crates/worth-kernel/src/workload_composition/worth_workload/spatial_touch_authority.rs`
- `crates/worth-kernel/src/query_obligation_selection/`

**Relevant APIs**
- `SpatialGeometryEvidenceTouchAuthority`
- `SpatialEvidenceLookupProduct`
- `lower_spatial_touch_authority_to_query_descriptor(...)`
- `SpatialEvidenceQueryTouchDescriptor`
- `spatial_query_graph_obligation_adoption_proof_for_descriptor(...)`
- `ForgeQueryGraphTouchDescriptor`

**Warnings**
- Do not let a Query descriptor alone satisfy spatial authority. The descriptor
  is selected-obligation input only after spatial authority and lookup proof
  have already been produced.
- Do not route spatial evidence through `worth-topo`.
- Do not keep broad stage scans as a fallback when descriptor selection misses.

**Test requirements**
- `spatial_touch_descriptor_selects_query_obligations_from_real_authority`:
  a real spatial authority plus lookup product lowers to a descriptor and
  produces execution-backed Query obligation adoption proof.
- `raw_row_lookup_product_and_topology_basis_cannot_select_spatial_obligations`:
  raw rows, lookup products without authority, copied descriptor fields, and
  topology touched basis products fail before Query selection.
- `spatial_descriptor_selection_records_query_gaps_without_claiming_milestone_six`:
  missing selector expressiveness is recorded as Query-gap posture and does not
  claim graph-read access planning closeout.

**Engineering decisions**
- Spatial descriptor selection proves selected obligations only; evidence
  lookup authority remains the Milestone 4 product and graph-read access plans
  remain Milestones 6 through 8.
- The spatial lane should prefer deleting old query-adoption support projection
  residue if the new selected-obligation proof makes it unnecessary.

**Open questions**
- Whether the one Milestone 4 support-projection residue row can be deleted in
  this milestone or must remain until graph-read access planning closes.

### Phase 5: Selector Precision, Counters, And Query Gaps

This phase makes selection breadth visible. It replaces broad collection-only
confidence with Query-owned selector precision, counters, deduplication proof,
rejection rows, and typed Query gaps for selector expressiveness Worth still
needs.

**Relevant subsystems**
- `crates/worth-kernel/src/query_obligation_selection/`
- `crates/worth-kernel/src/construction/graph_obligation_adoption/selector_matrix.rs`
- `crates/worth-spatial/src/query_adoption/consumer_kit.rs`
- `crates/forge-query/src/consumer_kit/graph_obligation_adoption`
- `crates/forge-query/src/runtime/tests/mutation/graph_obligation_*`

**Relevant APIs**
- `ForgeQueryGraphTouchDescriptor`
- `ForgeQueryGraphTouchSelector`
- `ForgeQueryGraphObligationExecutionProof`
- `ForgeQueryGraphObligationExecutionBudget`
- `ForgeQueryGraphObligationExecutionScope`
- Query selection counters exposed by execution proof or adoption status

**Warnings**
- Do not use elapsed time as selector proof.
- Do not treat broad collection selector success as precision unless the
  counters prove touched-descriptor-bounded selection.
- Do not fill missing selector capabilities with Worth-side selector forks.

**Test requirements**
- `selector_precision_counters_scale_with_touched_descriptor_breadth`: small
  touched descriptors attempt fewer buckets and select fewer obligations than
  intentionally broader descriptors, with exact counters recorded.
- `broad_collection_or_lifecycle_only_selector_is_capped_residue`: any
  selector that matches primarily by collection or lifecycle without aspect,
  relation, scope, or operating-world precision is rejected or appears in a
  capped residue manifest.
- `missing_selector_expressiveness_records_query_gap`: selector shapes Worth
  needs but Query cannot express produce typed Query-gap rows with owner,
  blocker, and follow-on milestone instead of a local selector copy.

**Engineering decisions**
- Counter assertions belong beside the selected-obligation proof, not in broad
  runner logs.
- Query gaps are allowed only when the missing selector expressiveness is
  genuinely Query-owned and too large to close inside this milestone.

**Open questions**
- The exact public accessor names for Query selection counters may need to be
  adjusted to the current `forge-query` runtime surface during implementation.

### Phase 6: Local Ceremony And Selector Residue Deletion

This phase performs the hard break. Once a vertical lane has the Query-owned
selection proof, the old local selector, support row, source-grep, in-memory
adoption, broad scan, and fabricated proof surfaces are deleted or capped.

**Relevant subsystems**
- `crates/worth-kernel/src/construction/graph_obligation_adoption`
- `crates/worth-kernel/src/query_obligation_selection/`
- `crates/worth-topo/src/topology_operators`
- `crates/worth-spatial/src/query_adoption`
- `crates/worth-kernel/src/certification/public_facade_contracts`

**Relevant APIs**
- `ForgeQueryGraphObligationLocalCeremonyAudit::evaluate(...)`
- `ForgeQueryGraphObligationResidueManifest::capped(...)`
- `ForgeQueryGraphObligationResidueManifest::certify_candidate_against_previous(...)`
- public facade compile-fail fixtures for non-exported local ceremony surfaces

**Warnings**
- Do not keep a local selector path because it is useful for tests. Tests must
  use the Query-owned proof lane or a certification-only fixture with no
  production reachability.
- Do not rename old selector authority as a diagnostic if it can still decide
  selected obligations.
- Do not let residue counts grow after introduction.

**Test requirements**
- `local_graph_obligation_ceremony_blocks_adoption`: injected local selector
  tables, local graph walks, private support matrices, or source-grep audits
  fail the Query local ceremony audit.
- `selector_residue_count_cannot_grow_after_introduction`: capped residue
  manifests reject any later increase in current count, cap, blocker drift, or
  removal-trigger drift.
- `deleted_selector_surfaces_are_not_publicly_reexported`: public facade and
  compile-fail tests prove local ceremony and selector-residue helpers are not
  ordinary public APIs.

**Engineering decisions**
- Deletion pressure is phase-local: after the primitive lane migrates, primitive
  selector residue cannot wait for unrelated boolean or graph-read phases.
- Residue rows must say why deletion is blocked and what later Query or Worth
  milestone removes the block.

**Open questions**
- None.

### Phase 7: Public DX And Facade Firewall

This phase defines the public caller experience for selected-obligation proof
without exposing Query internals as forgeable authority or leaking migration
folders.

**Relevant subsystems**
- `crates/worth-kernel/src/workload_composition`
- `crates/worth-kernel/src/query_obligation_selection`
- `crates/worth-topo/src/facade.rs`
- `crates/worth-spatial/src/facade/query_adoption.rs`
- `crates/worth-kernel/src/certification/public_facade_contracts`

**Relevant APIs**
- `WorthWorkload`
- `TopologyPrimitiveConstructionBirthDeclaredTouchedBasis`
- `SpatialEvidenceQueryTouchDescriptor`
- `ForgeQueryGraphObligationExecutionBackedAdoptionProof`
- selected-obligation read-only status wrappers

**DX target**

```rust
let touched_basis = admitted_operator.declare_touched_graph_basis()?;
let selected = worth_workload.select_query_graph_obligations(&touched_basis)?;

assert!(selected.execution_proof().has_real_executor_rows());
assert_eq!(selected.touched_basis_digest(), touched_basis.digest());
```

Forbidden route:

```rust
let selected = WorthSelectedGraphObligations::from_copied_counts(
    "graph-obligation-envelope",
    1,
);
```

**Warnings**
- Do not expose public constructors for selected-obligation proof wrappers.
- Do not make callers import migration-internal modules.
- Do not flatten Query execution/adoption proof into booleans or strings.

**Test requirements**
- `public_dx_selects_obligations_from_touched_authority_only`: public-facing
  examples route from touched basis or spatial descriptor to selected Query
  proof and preserve typed Query proof identity.
- `public_selected_obligation_proof_not_forgeable`: compile-fail fixtures prove
  copied counts, strings, raw descriptors, and local support rows cannot
  construct the public selected-obligation proof.
- `facade_does_not_export_migration_internals`: public facade scans reject
  local ceremony audits, residue helper constructors, selector matrices, and
  parallel-substrate internals.

**Engineering decisions**
- Public DX may wrap Query proof for Worth ergonomics, but the wrapper must
  preserve Query identity privately and expose read-only status.
- If public DX is still premature, the milestone may close with certification
  facades only, but then the roadmap must say public selection DX remains a
  follow-on target before Milestone 7.5.

**Open questions**
- Whether selected-obligation DX should live on `WorthWorkload` immediately or
  remain a narrower certification/public-facade contract until graph-read
  access planning lands.

### Phase 8: Cross-Crate Closeout And Milestone 6 Readiness

This phase closes the migration only when the old selector substrate is no
longer a future dependency and Milestone 6 can start graph-read access inventory
from selected Query obligations rather than local selector folklore.

**Relevant subsystems**
- `crates/worth-kernel/src/certification/public_facade_contracts`
- `crates/worth-kernel/src/query_obligation_selection`
- `crates/worth-topo/src/construction/query_native_boundary`
- `crates/worth-spatial/src/query_adoption`
- `_docs/worth/touched-graph-roadmap.md`

**Relevant APIs**
- all selected-obligation proof/status wrappers introduced in this milestone
- Graph obligation Consumer Kit proof and residue APIs
- Milestone 6 graph-read inventory inputs

**Warnings**
- Do not close on smoke tests.
- Do not claim graph-read access planning or validator derivation.
- Do not let old selector residue remain unnamed just because it is not used by
  the migrated primitive lane.

**Test requirements**
- `milestone_five_query_obligation_selection_closeout_is_closed`: topology and
  spatial positive paths produce nonempty Query-selected obligation identity,
  execution proof, adoption proof, support posture, counters, and residue
  manifest with no open findings.
- `milestone_five_rejects_each_old_selector_authority_path`: local selector
  table, broad collection selector, lifecycle-only shortcut, local support row,
  in-memory proof, copied count, raw descriptor, and topology/spatial
  substitution each fail in its own typed proof family.
- `milestone_six_starts_from_selected_query_obligations`: closeout exposes the
  selected-obligation product that graph-read access inventory must consume,
  while explicitly not claiming access-plan admission.

**Engineering decisions**
- The runner should stop after this milestone and require human confirmation
  before Milestone 6 because the next boundary changes from obligation
  selection to graph-read access planning.
- The closeout should update the roadmap with exact residue counts and Query
  gaps, not broad prose.

**Open questions**
- None.

## Must Ship

- A typed inventory of all current Worth graph obligation selection, adoption,
  support, local ceremony, selected-count, and facade surfaces.
- A parallel Query-owned selection substrate with responsibility-named modules,
  not `v2` or provenance folders.
- Primitive construction migrated as the first vertical lane with parity and
  stronger denial proof.
- Spatial touch authority connected to Query obligation selection through
  `SpatialEvidenceQueryTouchDescriptor`.
- Query selector precision counters, deduplication/rejection evidence, and
  Query-gap rows.
- Local selector ceremony deletion or capped residue manifests with owner, cap,
  blocker, and removal trigger.
- Public or certification facade proof that selected-obligation products are
  read-only and not forgeable.
- Milestone 6 readiness proof that graph-read access inventory starts from
  selected Query obligations.

## Must Preserve

- Topology truth remains in `worth-topo`.
- Spatial evidence authority remains in `worth-spatial`.
- Query owns selector matching, support posture, execution proof, adoption
  proof, local ceremony audits, and residue proof.
- `worth-kernel` owns workload composition and closeout pressure.
- Primitive construction behavior that is already trusted remains available
  during migration until parity and deletion gates pass.
- Milestone 5 does not claim graph-read access planning, validator derivation,
  invalidation, replay, conflict, cache, or public diagnostics closeout.

## Acceptance Evidence

This milestone is complete only when:

- every graph obligation selection surface is inventoried and classified
- at least one real primitive-construction vertical lane uses the parallel
  Query-owned selected-obligation path
- spatial touch authority can select Query obligations through its descriptor
  without raw-row, lookup-only, topology-basis, or copied-field substitution
- selector precision counters prove touched-descriptor-bounded breadth
- Query gaps are explicit and owned
- old local selector authority is deleted or capped with non-growing residue
- public or certification facades cannot construct selected-obligation proof
  from raw strings, copied counts, local support rows, or in-memory proof
- Milestone 6 has a concrete selected-obligation product to consume

## Sequencing Notes

This milestone belongs immediately after Milestone 4 because spatial touch
authority and topology touched basis products are the inputs Query obligation
selection must consume.

It belongs before Milestones 6 through 8 because graph-read access planning
should start from selected Query obligations, not from Worth-local selector
folklore or broad graph-read helper loops.

The strangler shape is intentional. Worth can keep the old primitive
construction lane working while the new selected-obligation path proves parity,
but old authority-looking surfaces must be deleted or capped before the
milestone closes. The migration is successful only when the old path stops being
a future dependency.
