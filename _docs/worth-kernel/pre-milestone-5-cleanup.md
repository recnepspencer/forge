# Pre-Milestone 5 Cleanup

This document captures the robustness work that must be completed before
Milestone 5 planning begins in earnest.

The central admitted-scaffold closeout seam is already in the right shape:

- family-local admission and realization stay small
- shared birth lowering owns placement embedding, realization-report closeout,
  scaffold digesting, and spatial birth handoff
- direct planar families do not pretend to share the same realization mechanics
  as realized solids

That seam is the baseline architecture. The phases below name the remaining
corrections required before Milestone 5 expands authority, replay, inspection,
and existing-truth surfaces on top of this substrate.

No item in this document is advisory debt. Each phase is a pre-Milestone 5
correction requirement.

## Governing Rule

This cleanup is not about making the current code "good enough."

It exists to ensure Milestone 5 does not build on:

- ambiguous geometry identity
- split digest protocols
- duplicated witness definitions
- under-admitted planar witness layouts
- manually restated family contracts
- weak geometry-hostile certification

The robust target is one shared canonical geometry-and-contract substrate that
`worth-kernel`, `worth-spatial`, `worth-geom`, and `worth-topo` all consume.

## Priority Bands

- `P1`: must be fixed before Milestone 5 implementation begins
- `P2`: must be fixed before Milestone 5 closeout can be claimed
- `P3`: must be fixed before any Milestone 5 surface depends on the affected
  truth

## Query Alignment

These cleanup phases are mostly geometry and contract work, not Query-lifecycle
work. Still, some Query capability surfaces matter because Milestone 5 will
consume the resulting truth as canonical identity, replay truth, and
inspection-grade substrate.

The most relevant Query docs are:

- `crates/forge-query/docs/capabilities/existing-truth.md`
- `crates/forge-query/docs/capabilities/inspection.md`
- `crates/forge-query/docs/capabilities/lineage-and-correspondence.md`
- `crates/forge-query/docs/capabilities/projection-consumption.md`
- `crates/forge-query/docs/domain-capabilities/canonical-domain-declarations.md`
- `crates/forge-query/docs/domain-capabilities/declaration-entry-inspection.md`
- `crates/forge-query/docs/domain-capabilities/declaration-progression.md`
- `crates/forge-query/docs/domain-capabilities/continuation-pipeline.md`
- `crates/forge-query/docs/domain-capabilities/contribution-composed-orchestration.md`
- `crates/forge-query/docs/domain-capabilities/orchestration-inventory.md`
- `crates/forge-query/docs/domain-capabilities/workflow/retained-artifact-to-next-step.md`

The working rule is:

- if a cleanup phase produces canonical geometry identity or family contract
  truth, it must be strong enough to survive later Query-facing digest,
  inspection, replay, existing-truth, projection-consumption, and lineage
  surfaces
- do not add local "report identity" or "parity helper" toy seams where
  Query-facing identity-bearing surfaces will later need canonical truth

## Phase Order

Implementation order:

1. Canonical geometry identity and shared digest protocol
2. Canonical witness geometry and planar witness authority
3. Canonical family contract registry and derived counts
4. Geometry-hostile certification and replay-grade proof surfaces

This order is load-bearing.

Phase 1 freezes identity and digest truth first. Phase 2 then freezes the
actual witness geometry that identity commits. Phase 3 freezes family contract
truth and derived topology/support counts on top of that geometry. Phase 4
proves the resulting substrate is hostile enough for Milestone 5 to trust.

## Phase 1: Canonical Geometry Identity And Shared Digest Protocol

### Priority

- `P1`

### Closes

- item `1`
- item `2`
- geometry-identity portion of item `5`

### Problem Statement

Current scaffold and realization digests do not commit geometry strongly
enough, and digest protocols are split between canonical SHA-256 and local
`DefaultHasher`.

That leaves the system with identity-bearing truth that is too weak for later:

- replay parity
- inspection
- existing-truth consumption
- lineage / correspondence
- retained-artifact identity

### Surfaces Touched

- `crates/worth-kernel/src/construction/phase_chain/admitted_scaffold/family_birth_input/birth_scaffold.rs`
- `crates/worth-kernel/src/construction/proof/digest_protocol.rs`
- `crates/worth-spatial/src/bindings/primitive_birth.rs`
- `crates/worth-geom/src/primitives/shape_realization/schema.rs`
- `crates/worth-geom/src/primitives/shape_realization/exhaustion.rs`
- `crates/worth-geom/src/primitives/shape_realization/witnesses.rs`

### Query Surfaces That Must Be Kept In Scope

- `existing-truth`
- `inspection`
- `lineage-and-correspondence`
- `projection-consumption`
- `canonical-domain-declarations`

These phases do not need to call Query runtime APIs directly, but the resulting
identity surfaces must be robust enough for those Query surfaces to use later
without inventing a second digest story.

### Subsystems Updated

- `worth-kernel` admitted scaffold digesting
- `worth-kernel` proof digest protocol
- `worth-spatial` primitive-birth identity
- `worth-geom` realization digesting and witness digesting

### Surfaces To Add

- one shared digest protocol surface below the crate-local helper layer
  - `TruthDigestVersion`
  - `TruthDigestScope`
  - `truth_digest_parts(...)`
- one explicit geometry identity surface
  - `PrimitiveSupportPlaneIdentity`
  - `PrimitiveVertexIdentity`
  - `PrimitiveRealizedSupportIdentity`
- one explicit geometry digest surface
  - `PrimitiveScaffoldGeometryDigest`
  - `PrimitiveRealizationGeometryDigest`
  - `PrimitiveGeometryIdentityBundle`
  - `PrimitiveGeometryIdentityDigest`

### Robust Version

The robust end state is:

- scaffold identity commits actual support-plane and vertex identity, not just
  counts plus a report digest
- realization identity commits actual realized support and embedded geometry,
  not just attempted strategy summaries
- kernel, spatial, and geom all use one explicit digest protocol with one
  version story
- no local `DefaultHasher` truth artifacts remain on canonical identity lanes

The non-robust toy version would be:

- keeping the current summary digest and merely renaming it
- swapping `DefaultHasher` for SHA-256 without freezing geometry identity
  components explicitly
- adding one crate-local helper while leaving three different digest payload
  stories alive

### Integration Tests

1. `scaffold_geometry_digest_changes_when_plane_or_vertex_identity_changes`
   - Build two admitted scaffolds with identical family/count/report summaries
     but different support-plane or embedded-vertex geometry.
   - Assert scaffold identity digest changes.
   - This proves we no longer have summary-only digest collisions.

2. `kernel_spatial_geom_geometry_identity_protocol_is_shared`
   - Construct equivalent geometry identity payloads through kernel birth,
     spatial primitive birth, and geom realization.
   - Assert they all use the same protocol version, scope, and canonical digest
     encoding rules.
   - This proves the digest story is unified rather than merely stronger in one
     crate.

## Phase 2: Canonical Witness Geometry And Planar Witness Authority

### Priority

- `P1`

### Closes

- item `3`
- item `4`
- witness-duplication portion of item `5`
- item `9`

### Problem Statement

Planar family witness geometry still relies on fixed layout shortcuts whose
authority is unclear, and canonical witness geometry is duplicated across
kernel and geom.

Right now the code cannot cleanly answer:

- whether these witnesses are canonical scaffold witnesses
- whether they are request-derived
- what legality rules make them safe as hole counts grow
- what canonical ratio story owns the simplex approximation

### Surfaces Touched

- `crates/worth-kernel/src/construction/phase_chain/admitted_scaffold/family_birth_input/geometry.rs`
- `crates/worth-kernel/src/construction/phase_chain/admitted_scaffold/family_birth_input/families/shell_with_hole.rs`
- `crates/worth-geom/src/primitives/shape_realization/support/simplex.rs`

### Query Surfaces That Must Be Kept In Scope

- `inspection`
- `declaration-entry-inspection`
- `existing-truth`
- `projection-consumption`

Again, this phase is still pre-Query runtime work, but the resulting witness
surfaces must later survive inspection and existing-truth parity as canonical
geometry truth, not "helper geometry."

### Subsystems Updated

- `worth-kernel` family birth-input geometry
- `worth-kernel` planar family admission
- `worth-geom` canonical local witness realization

### Surfaces To Add

- one shared canonical witness geometry source of truth
  - `PrimitiveCanonicalWitnessGeometry`
  - `SimplexCanonicalWitnessGeometry`
  - `OrthotopeCanonicalWitnessGeometry`
  - `RegularPrismCanonicalWitnessGeometry`
  - `RegularPyramidCanonicalWitnessGeometry`
  - `WireBodyCanonicalWitnessGeometry`
  - `ShellWithHoleCanonicalWitnessGeometry`
- one explicit witness-authority surface
  - `PrimitivePlanarWitnessAuthority`
  - `CanonicalScaffoldWitness`
  - `RequestDerivedWitness`
- one shell-with-hole layout legality surface
  - `ShellWithHoleWitnessLayoutPolicy`
  - `ShellWithHoleWitnessLayout`
  - `ShellWithHoleLayoutLegality`
  - `HoleContainmentAudit`
  - `HoleClearanceAudit`
- one named canonical ratio surface
  - `CANONICAL_SIMPLEX_LATERAL_RATIO`
  - or `SimplexCanonicalWitnessRatios`

### Robust Version

The robust end state is:

- kernel birth embedding and geom realization both consume one canonical
  witness geometry source
- planar witness geometry carries an explicit authority story instead of
  accidental fixed constants
- shell-with-hole witness generation is mechanically legal or explicitly
  rejected as hole counts/layout requests grow
- the simplex ratio story is named and canonical, not a magic decimal

The non-robust toy version would be:

- deleting one duplicate witness helper while leaving geometry authority
  implicit
- keeping fixed shell-with-hole radii and just adding more comments
- naming `0.7071` without tying it to a real canonical witness definition

### Integration Tests

1. `kernel_and_geom_share_canonical_witness_geometry_for_each_family`
   - For the families with canonical local witnesses, generate kernel birth
     witness geometry and geom realization witness geometry from the shared
     source.
   - Assert vertex coordinates and support definitions are identical.
   - This proves the duplication and drift risk is actually gone.

2. `shell_with_hole_canonical_layout_rejects_illegal_hole_growth`
   - Increase hole count or loop sizes until the previous fixed layout would
     imply overlap or failed containment.
   - Assert the new layout legality surface either derives a legal layout or
     rejects the request with explicit failure evidence.
   - This proves shell-with-hole is no longer quietly under-admitted.

## Phase 3: Canonical Family Contract Registry And Derived Counts

### Priority

- `P2`

### Closes

- item `6`
- item `8`

### Problem Statement

Family contract truth is still manually restated across kernel, spatial, and
topology layers, and topology counts are still typed in manually by family
helpers.

That leaves the system brittle as family support grows because the same truth
must be updated in multiple places.

### Surfaces Touched

- `crates/worth-kernel/src/construction/phase_chain/admitted_scaffold/family_birth_input/birth_scaffold.rs`
- `crates/worth-kernel/src/construction/phase_chain/admitted_scaffold/family_birth_input/topology_counts.rs`
- `crates/worth-spatial/src/bindings/primitive_birth_contract.rs`
- `crates/worth-topo/src/construction/query_native_boundary/admission.rs`

### Query Surfaces That Must Be Kept In Scope

- `canonical-domain-declarations`
- `declaration-progression`
- `contribution-composed-orchestration`
- `retained-artifact-to-next-step`

The registry created here is the contract truth later declaration/progression
surfaces will depend on. The goal is to prevent Milestone 5 from layering
runtime lifecycle over split family truth.

### Subsystems Updated

- `worth-kernel` family birth-input normalization
- `worth-kernel` topology-count derivation
- `worth-spatial` primitive-birth contract validation
- `worth-topo` query-native birth admission

### Surfaces To Add

- one canonical family contract registry
  - `PrimitiveConstructionFamilyContractRegistry`
  - `PrimitiveConstructionFamilyContract`
  - `PrimitiveConstructionTopologyContract`
  - `PrimitiveConstructionSupportContract`
- one family witness descriptor / derived summary layer
  - `PrimitiveWitnessDescriptor`
  - `PrimitiveWitnessTopologySummary`
  - `PrimitiveWitnessSupportSummary`
- one bridge surface from family contract into topology-ready handoff
  - `TopologyReadyBirthContractView`
  - `PrimitiveConstructionBirthSynopsisContract`

### Robust Version

The robust end state is:

- family support/topology/count truth is defined once and projected downward
  into kernel, spatial, and topo
- topology counts are derived from one family contract or witness descriptor
  rather than entered as loose numeric tuples
- downstream validation becomes contract consumption, not parallel
  contract-authoring

The non-robust toy version would be:

- introducing a registry while still manually restating count formulas in
  kernel or topo
- deriving only some counts while leaving other structural family truths as
  crate-local repetition
- adding a helper to "compare" contracts instead of making one source
  authoritative

### Integration Tests

1. `kernel_spatial_topo_family_contract_projection_stays_in_lockstep`
   - For each supported family, derive the contract once from the canonical
     registry and project it into kernel birth, spatial validation, and topo
     admission.
   - Assert all three layers see the same support/topology/count truth.
   - This proves the split contract authority is gone.

2. `family_topology_counts_are_derived_not_typed`
   - Exercise each family birth helper through the real phase-chain path.
   - Assert the resulting topology counts come from the family descriptor /
     contract view and cannot drift from hand-entered numeric tuples.
   - This proves the manual count truth surface is dead.

## Phase 4: Geometry-Hostile Certification And Replay-Grade Proof Surfaces

### Priority

- digest / witness hostility portions are `P2`
- any dependent Milestone 5 proof that consumes these surfaces is `P3` gated
  until this phase is complete

### Closes

- item `7`
- certification-dependent remainder of items `1` through `9`

### Problem Statement

Current boundary proof is stronger on architecture honesty than on geometry
honesty. That imbalance is dangerous because Milestone 5 will trust geometry
truth for replay, parity, inspection, and existing-truth surfaces.

We need proof surfaces that do more than check "the right folder owns the
logic." They must prove the actual geometry truth is adversarially stable.

### Surfaces Touched

- Phase 5 boundary and closeout proof areas in `worth-kernel`
- geometric realization test/certification lanes in `worth-geom`
- spatial birth validation/certification lanes in `worth-spatial`

### Query Surfaces That Must Be Kept In Scope

- `inspection`
- `existing-truth`
- `lineage-and-correspondence`
- `projection-consumption`
- `continuation-pipeline`
- `orchestration-inventory`

These proof surfaces are where we certify that later Query-facing replay,
inspection, and existing-truth usage is actually safe.

### Subsystems Updated

- `worth-kernel` hostile boundary / closeout proof
- `worth-kernel` replay-grade proof substrate
- `worth-geom` realization hostility proof
- `worth-spatial` birth hostility / contract validation proof

### Surfaces To Add

- one digest-sensitivity certification surface
  - `PrimitiveGeometryDigestSensitivityReport`
  - `PrimitiveGeometryDigestMutationCase`
- one witness parity certification surface
  - `PrimitiveCanonicalWitnessParityReport`
  - `PrimitiveCanonicalWitnessParityMismatch`
- one planar layout hostility surface
  - `ShellWithHoleLayoutHostilitySuite`
  - `PlanarWitnessContainmentReport`
  - `PlanarWitnessNonOverlapReport`
- one canonical-ratio proof surface
  - `SimplexCanonicalRatioReport`
  - `SimplexCanonicalWitnessDefinition`

### Robust Version

The robust end state is:

- geometry digest mutations are proven sensitive to actual geometry changes
- kernel embedding and geom realization are proven to share the same canonical
  witness geometry
- shell-with-hole canonical witnesses are proven legal or rejected under hostile
  growth conditions
- simplex canonical-ratio authority is named and machine-checked

The non-robust toy version would be:

- adding more unit tests around helper functions without creating digest parity
  or witness parity certification artifacts
- proving only the happy path for shell-with-hole layouts
- checking one canonical family while leaving the rest as informal convention

### Integration Tests

1. `geometry_digest_mutation_and_replay_parity_hostility_suite`
   - Starting from one canonical admitted scaffold, mutate support planes,
     canonical witness coordinates, and embedded vertex positions across a set
     of hostile cases.
   - Assert digest changes are reflected consistently in kernel/scaffold birth,
     spatial consequence truth, and geom realization truth.
   - This proves the geometry identity substrate is replay-grade instead of
     summary-grade.

2. `canonical_witness_and_contract_hostility_suite_survives_full_cross_crate_flow`
   - Run a family set through kernel admitted scaffold, spatial primitive birth,
     geom realization, and topo admission using the new canonical witness and
     contract registry surfaces.
   - Assert parity of canonical witness geometry, derived counts, and contract
     truth across the full path.
   - This proves the cleanup is not just locally correct; it survives the real
     multi-crate flow Milestone 5 will depend on.

## Summary

The admitted-scaffold architecture is the right baseline, but Milestone 5 must
not build on:

- ambiguous geometry authority
- summary-only geometry digests
- split digest protocols
- duplicated canonical witness geometry
- under-admitted planar witness layouts
- manually restated family contracts
- weak geometry-hostile proof

The four phases above are the required substrate corrections.

Milestone 5 planning should begin only after:

- canonical geometry identity is shared and digest-bearing
- canonical witness geometry and planar authority are explicit
- family contract truth and topology counts are derived from one authority
- hostile proof demonstrates that geometry truth is robust enough for replay,
  inspection, existing-truth, and projection-consumption surfaces to trust it

Every phase above must be completed at its stated gate before the dependent
Milestone 5 work proceeds.
