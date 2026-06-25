# Touched Graph Milestone 4: Spatial Geometry Evidence Touch Authority

> **Status:** Draft
>
> **Purpose:** freeze the canonical spatial evidence touch authority product
> that consumes sealed boolean/spatial evidence receipts, lowers to Query
> touch/adoption proof, and feeds Milestone 5 without reopening topology,
> raw ledger, projection, or local Query authority.

## Goal

Make boolean and spatial geometry evidence produce a sealed spatial touch
authority product before it can drive Query obligation proof, graph-read
coverage, evidence lookup, replay, cache, diagnostics, or later boolean work.

By the end of this milestone:

- a sealed boolean or spatial evidence receipt can enter one and only one
  spatial touch authority admission boundary
- manual evidence rows, copied receipt fields, raw stage scans, topology
  touched-basis proofs, schema vocabulary rows, and local Query ceremony are
  rejected before authority construction
- the admitted authority product carries only spatial admission facts: receipt
  identity, boolean stage, support, counters, complete-ledger identity,
  stage-index identity, stage-link proof, digest, and denial locality
- lookup identity, Query descriptor digest, operating-world digest, Consumer
  Kit adoption proof, residue posture, and public diagnostics are derived
  products with separate identities and constructors
- `worth-spatial` can prove Query obligation adoption through Query's Consumer
  Kit rather than local reports or local support rows
- downstream consumers can inspect public status and diagnostics without
  gaining constructors or substituting derived products for authority

## Why This Milestone Exists

Milestones 2 and 3 make topology operators declare touched graph basis before
they execute. Spatial evidence has a different authority source: receipts,
stage indexes, and evidence ledgers produced by `worth-spatial`. Milestone 4
freezes that branch without pretending geometry evidence is topology basis and
without letting raw evidence rows, copied receipt fields, type-name checks, or
projection/admission experiments stand in for authority.

This milestone belongs immediately before Query obligation selection because
Milestone 5 needs two honest inputs: topology touched-basis lowering from
Milestone 3 and spatial Query descriptors/adoption proof from this milestone.

## Governing Summaries

- `MENTALITY.md` protects adversarial-constraint-first design. The shaping
  constraint here is that small local boolean evidence inside large geometry
  must never trigger global rediscovery or local proof folklore.
- `arch_laws.md` protects proof-bearing phase transitions. The spatial receipt
  path must produce a typed proof product before Query lowering, and later
  phases must consume that proof instead of re-proving from rows.
- `composition_laws.md` protects named responsibilities. Receipt admission,
  ledger lookup, Query descriptor lowering, Consumer Kit adoption, residue
  cleanup, and public facade proof must live as separate concepts.
- `domain_structure_laws.md` protects visible ownership. `worth-spatial` owns
  spatial evidence authority, `forge-query` owns Query descriptors and
  adoption proof, and `worth-topo` must not become a geometry evidence adapter.
- `perf_laws.md` protects semantic-delta-bounded execution. Evidence lookup and
  Query proof must scale with the admitted receipt/stage touch, not broad stage
  scans, raw ledger walks, or global topology size.
- `touched-graph-roadmap.md` protects the product ladder. Milestone 4 is the
  spatial branch step between sealed receipts and Query/touched proof.
- `crates/forge-query/docs/AI_README.md` protects Query ownership. Downstream
  crates must use Query's facade and Consumer Kit for graph obligation proof
  instead of inventing local reports, local digests, local support pins, or
  pseudo-Query adapters.

## Adversarial Constraint

Spatial boolean work must survive long boolean chains where a local stage
receipt touches a narrow geometry surface while the model contains unrelated
topology, stale projections, retained replay evidence, diagnostics, and cached
stage products.

If a production path can satisfy spatial evidence authority from raw
`WorkloadEvidenceRow` values, manual rows, copied receipt fields, public schema
constructors, broad ledger scans, type-name guards, or topology touched-basis
laundering, this milestone has failed.

## Product Decision Lock

- `worth-spatial` owns the spatial evidence authority product.
- `forge-query` owns graph touch descriptors, selectors, support posture,
  obligation registration, selection, execution proof, and Consumer Kit
  adoption.
- `worth-schema` may provide shared vocabulary, but it must not admit spatial
  authority.
- `worth-topo` topology touched-basis products are adjacent proof products, not
  substitutes for spatial evidence authority.
- Milestone 4 may produce Query descriptors and adoption proof. It must not
  implement Milestones 6 through 8 graph-read access plans or local access
  folklore migrations.

## DX Target

The ordinary spatial author should be able to admit a boolean receipt, expose
the resulting spatial touch proof, and hand Query the canonical descriptor
without knowing how the ledger, stage index, Consumer Kit, or Query selector
matrix is wired.

```rust
let spatial_touch = SpatialGeometryEvidenceTouchRequest::from_boolean_receipt(&event_receipt)
    .with_complete_ledger(workload.complete_evidence_ledger())
    .admit()?;

let query_descriptor = spatial_touch.query_touch_descriptor();
let operating_world = spatial_touch.query_operating_world();

let adoption = graph_obligation_consumer_kit("worth-spatial")
    .register_obligations(spatial_touch.registration_declaration())
    .declare_selector_coverage(spatial_touch.selector_coverage())
    .pin_support(spatial_touch.support_pin())
    .audit_local_ceremony(spatial_touch.local_ceremony_audit())
    .account_for_residue(spatial_touch.residue_manifest())
    .prove_execution_with(query_descriptor, operating_world)?
    .prove_adoption_with_execution()?;

let lookup_key = spatial_touch.evidence_lookup_key();
```

The exact type names may change during implementation, but this shape is
non-negotiable: callers start with a sealed spatial receipt or complete ledger,
receive a proof-bearing spatial touch authority product, and only then cross
into Query.

## Architectural Flow

```text
BooleanEvidenceReceipt plus CompleteWorkloadEvidenceLedger
-> spatial receipt admission
-> SpatialGeometryEvidenceTouchAuthority
-> spatial evidence lookup key and counters derived from authority
-> ForgeQueryGraphTouchDescriptor plus operating world descriptor derived from authority
-> Consumer Kit registration, selector coverage, support pin, audit, residue
-> execution-backed adoption proof derived from Query descriptor
-> public read-only spatial authority status derived from proof products
```

The authority flow is one-way. Query descriptors are derived from spatial
authority and cannot reconstruct it. Evidence lookup products are derived from
spatial authority and cannot stand in for Query descriptors. Topology
touched-basis proof may be referenced for correlation, but it cannot admit
spatial evidence.

## Admission Contract

Ordinary spatial touch authority has exactly one admission contract:

```text
sealed BooleanEvidenceReceipt
+ CompleteWorkloadEvidenceLedger
+ stage-index-backed receipt lookup
+ admitted support/counter/stage-link proof
-> SpatialGeometryEvidenceTouchAuthority
```

The contract separates three categories that must not blur:

- admission source:
  - a sealed `BooleanEvidenceReceipt` value supplied by an admitted spatial
    stage
  - no raw row, copied receipt field set, topology proof, Query descriptor,
    schema vocabulary row, or public diagnostic can be an admission source
- supporting evidence used during admission:
  - `CompleteWorkloadEvidenceLedger`
  - `WorkloadEvidenceStageIndexProduct`
  - `WorkloadEvidenceBooleanReceiptLookupProduct`
  - `WorkloadEvidenceStageLinkSet`
  - these products help prove admission but cannot construct authority without
    the sealed receipt
- derived products after admission:
  - spatial lookup key/proof
  - Query descriptor and operating-world descriptor
  - Consumer Kit adoption proof
  - public status and diagnostics
  - these products consume spatial touch authority and can never recreate it

Ordinary admission requires complete-ledger proof. Receipt-only mode is allowed
only as a diagnostic preview posture named in the denial/status surface, and it
must not satisfy spatial touch authority, Query descriptor lowering, lookup
authority, Consumer Kit adoption, Milestone 5 handoff, replay proof, or public
closeout.

`BooleanEvidenceRowAuthority` is not a second public admission source. It is a
sealed marker that permits receipt-backed evidence rows to be produced inside
the evidence ledger boundary. External callers must still enter ordinary
spatial touch admission through a sealed receipt plus complete-ledger proof.

Denials are deterministic. Admission must classify source-substitution denials
before support denials, support denials before counter/stage-link denials, and
all spatial admission denials before Query expressiveness gaps. A multi-cause
diagnostic may report additional context, but the blocking denial kind must be
stable.

## Artifact Ladder

Milestone 4 should be implemented as an explicit artifact ladder, not as one
broad "spatial evidence authority" helper.

Admission source:

- `BooleanEvidenceReceipt`

Ledger proof required for ordinary admission:

- `CompleteWorkloadEvidenceLedger`

Supporting evidence inside admission:

- `BooleanEvidenceRowAuthority`
- `WorkloadEvidenceStageIndexProduct`
- `WorkloadEvidenceBooleanReceiptLookupProduct`

Request:

- `SpatialGeometryEvidenceTouchRequest`
- `SpatialGeometryEvidenceTouchAdmissionInput`

Admission:

- `SpatialGeometryEvidenceTouchAuthority`
- `SpatialGeometryEvidenceTouchDigest`
- `SpatialGeometryEvidenceTouchCounters`
- `SpatialGeometryEvidenceTouchDenial`

Lookup:

- `SpatialEvidenceLookupKey`
- `SpatialEvidenceLookupProof`
- `SpatialEvidenceStageLinkProof`

Query lowering:

- `SpatialEvidenceQueryTouchDescriptor`
- `SpatialEvidenceQueryOperatingWorld`
- `SpatialEvidenceQueryGapRow`

Query adoption:

- `SpatialEvidenceGraphObligationRegistrationDeclaration`
- `SpatialEvidenceGraphObligationSelectorCoverage`
- `SpatialEvidenceGraphObligationSupportPin`
- `SpatialEvidenceGraphObligationResidueManifest`
- `SpatialEvidenceGraphObligationAdoptionProof`

Public proof:

- `SpatialGeometryEvidenceTouchStatus`
- `SpatialGeometryEvidenceTouchDiagnostic`
- `SpatialGeometryEvidenceTouchCloseoutReport`

Exact names may change during implementation, but every artifact class above
must either exist as a named type/module or be explicitly collapsed into an
existing Worth/Query product with the same proof boundary. Cosmetic wrapping is
not allowed.

## Spatial Authority Exclusivity Law

After Milestone 4, every Milestone 5+ spatial evidence consumer must consume
the admitted spatial touch authority product or a product derived from it. It
must not consume raw evidence rows, raw receipt fields, broad stage scans,
topology touched-basis proofs, Query descriptors, local support rows, or schema
vocabulary as substitutes for spatial evidence touch truth.

## Existing Surface Inventory

Milestone 4 should widen live surfaces before inventing new ones:

- `crates/worth-spatial/src/facade/workload_vocabulary/mod.rs`
  - `BooleanEvidenceReceipt`
  - `BooleanEvidenceRowAuthority`
  - `CompleteWorkloadEvidenceLedger`
  - `WorkloadEvidenceLedger`
  - `WorkloadEvidenceRow`
  - `WorkloadEvidenceStage`
  - `BooleanEvidenceStageKind`
  - `WorkloadEvidenceStageIndexProduct`
  - `WorkloadEvidenceBooleanReceiptLookupProduct`
  - `WorkloadEvidenceStageLinkSet`
- `crates/worth-spatial/src/workload_platform/evidence_ledger/*`
  - row construction, receipt-backed row construction, stage-index products,
    guards, complete-ledger certification, boolean receipt lookup, support
    posture, counters, and stage links
- `crates/worth-spatial/src/workload_platform/planar_boolean_events/*`
  - event ledger and segment-pair enumeration receipt authority
- `crates/worth-spatial/src/workload_platform/planar_boolean_edge_splitting/*`
  - split edge-chain receipt authority
- `crates/worth-spatial/src/workload_platform/planar_boolean_loop_reconstruction/*`
  - loop reconstruction receipt authority
- `crates/worth-spatial/src/facade/query_adoption.rs`
  - spatial Query adoption inventory, support pins, adoption status, and
    performance counters
- `crates/worth-kernel/src/workload_composition/*`
  - workload evidence requirements and boolean evidence consumption
- `crates/worth-topo/src/facade.rs`
  - topology touched-basis proof and topology Query descriptor lowering as a
    reference boundary, not as a spatial substitute
- `crates/forge-query/docs/AI_README.md`
  - Query public facade and Consumer Kit are the ordinary downstream proof
    surfaces
- `crates/forge-query/src/runtime/mutation/graph_composition/touch_descriptor/*`
  - Query graph touch descriptor constructors and descriptor digests
- `crates/forge-query/src/consumer_kit/graph_obligation_adoption/*`
  - registration declaration, selector coverage, support pin, local ceremony
    audit, residue manifest, in-memory proof, execution proof, and adoption
    proof

New Milestone 4 surfaces are allowed only where existing surfaces cannot
honestly express:

- a receipt-backed spatial touch authority product
- a denial that distinguishes manual rows, unsupported support, missing ledger
  authority, topology laundering, Query descriptor substitution, and Query
  expressiveness gaps
- a spatial lookup key that carries stage-index identity and spatial touch
  digest
- a Consumer Kit-backed adoption proof for spatial evidence descriptors
- public read-only authority status and compile-fail fences

## Query, Worth, And Evidence Integration Contract

Milestone 4 must distinguish five products that are easy to collapse:

- spatial receipt authority
  - source proof owned by `worth-spatial`
  - examples: `BooleanEvidenceReceipt`, `BooleanEvidenceRowAuthority`,
    receipt-backed rows, complete ledger proof
  - this is the only ordinary admission source
- spatial touch authority
  - admitted touch proof owned by `worth-spatial`
  - examples: request, authority product, digest, counters, denial, lookup key
  - this is the product Milestone 5+ spatial consumers must reference
- Query touch/adoption proof
  - derived proof owned by `forge-query`
  - examples: `ForgeQueryGraphTouchDescriptor`, operating-world descriptor,
    selector coverage, support pin, execution-backed adoption proof
  - this selects or proves Query obligations but cannot admit spatial evidence
- topology touched-basis proof
  - topology proof owned by `worth-topo`
  - examples: `TopologyTouchedGraphBasis`,
    `TopologyDeclaredTouchedGraphBasisProof`,
    `topology_operator_touch_descriptor_from_touched_graph_basis`
  - this can correlate with spatial evidence but cannot replace it
- schema vocabulary
  - shared language owned by `worth-schema`
  - examples: platform authority, relations, entities, aspects
  - this can name facts but cannot prove them

This means:

- a Query descriptor is never a backdoor constructor for spatial authority
- a topology touched-basis proof is never accepted where a spatial receipt is
  required
- a stage-index lookup product is not itself Query obligation proof
- a Consumer Kit adoption proof can prove Query consumption but not the
  existence of the underlying spatial evidence receipt
- every denied substitution must name which product boundary was violated

## Proof Product Invariants

Each proof product in this milestone must state its constructor inputs, derived
outputs, identity basis, and forbidden substitutions before implementation
closeout.

`SpatialGeometryEvidenceTouchAuthority` proves only spatial admission:

- constructor inputs:
  - sealed `BooleanEvidenceReceipt`
  - `CompleteWorkloadEvidenceLedger`
  - stage-index-backed receipt lookup
  - support/counter/stage-link proof
- identity basis:
  - receipt identity
  - boolean evidence stage
  - evidence support posture
  - evidence counters
  - complete-ledger identity
  - stage-index identity
  - stage-link proof digest
- derived outputs:
  - read-only admission status
  - authority digest
  - denial locality
- forbidden substitutions:
  - raw evidence rows
  - copied receipt fields
  - Query descriptors
  - topology touched-basis proofs
  - schema vocabulary rows
  - public diagnostics

`SpatialEvidenceLookupKey` proves bounded evidence lookup:

- constructor inputs:
  - admitted spatial touch authority
  - stage-index-backed receipt lookup product
- identity basis:
  - spatial touch digest
  - stage-index identity
  - boolean stage
  - evidence identity
  - support posture
- derived outputs:
  - lookup counters
  - lookup proof/status
- forbidden substitutions:
  - Query descriptor digest
  - raw evidence vector
  - broad stage scan result

`SpatialEvidenceQueryTouchDescriptor` proves Query descriptor lowering:

- constructor inputs:
  - admitted spatial touch authority
  - Query descriptor lowering policy
- identity basis:
  - spatial touch digest
  - Query descriptor digest
  - operating-world descriptor digest
  - Query-gap digest when lowering is incomplete
- derived outputs:
  - `ForgeQueryGraphTouchDescriptor`
  - `ForgeQueryGraphObligationOperatingWorldDescriptor`
  - gap rows
- forbidden substitutions:
  - using the descriptor as spatial admission authority
  - using the descriptor as evidence lookup authority
  - claiming Milestone 5 obligation selection closeout

`SpatialEvidenceGraphObligationAdoptionProof` proves Query consumption only:

- constructor inputs:
  - spatial Query descriptor product
  - Consumer Kit registration declaration
  - selector coverage
  - support pin
  - local ceremony audit
  - residue manifest
  - in-memory and execution proof where claimed
- identity basis:
  - Query descriptor digest
  - operating-world digest
  - registration declaration digest
  - selector coverage digest
  - support pin digest
  - residue manifest digest
  - execution proof digest
- derived outputs:
  - adoption status
  - Query selection/execution counters
- forbidden substitutions:
  - using adoption proof as receipt authority
  - using in-memory selection proof as execution proof
  - using local Query reports or support rows as proof

## Phase Plan

### Phase 1: Spatial Evidence Surface Inventory And Deletion Ledger

Freeze the current spatial evidence authority surface before adding new types.
This phase must classify every production and certification path that currently
lets spatial evidence act as touch authority, lookup authority, Query proof, or
topology substitute.

**Relevant subsystems**

- `crates/worth-spatial/src/facade/workload_vocabulary`
- `crates/worth-spatial/src/workload_platform/evidence_ledger`
- `crates/worth-spatial/src/query_adoption.rs`
- `crates/worth-kernel/src/workload_composition`
- existing public facade contract tests and compile-fail fixtures

**Relevant APIs**

- `BooleanEvidenceReceipt`
- `BooleanEvidenceRowAuthority`
- `WorkloadEvidenceRow::new`
- `WorkloadEvidenceRow::from_boolean_evidence_receipt`
- `WorkloadEvidenceLedger::from_rows`
- `WorkloadEvidenceLedger::certify_complete`
- `CompleteWorkloadEvidenceLedger`
- `current_spatial_query_consumer_kit_adoption_status`
- `spatial_query_adoption_inventory`

**Warnings**

- `WorkloadEvidenceRow::new` creates manual rows. Manual rows may remain for
  certification-only residue only if they are mechanically unable to satisfy
  spatial touch authority.
- Existing Query adoption inventory is not spatial touch authority. It can
  report adoption posture, but it cannot admit evidence.
- Projection/admission/type-name experiments must be deleted or capped with an
  owner, count, blocker, and removal trigger during closeout. Phase 1 records
  the action; later phases migrate and delete once replacements exist.

**Test requirements**

- Inventory parity test: source inventory rows must cover every public facade
  export, ledger constructor, boolean receipt implementation, spatial query
  adoption row, and kernel workload evidence consumption path.
- Deletion pressure test: any inventory row classified as production bypass
  must have a deletion or cap plan in Phase 1 and must fail final closeout if
  the row is still production-reachable after its replacement exists.
- Boundary denial test: a raw `WorkloadEvidenceRow::new(...)` manual row must
  not satisfy spatial touch authority, even when stage, identity, and counters
  resemble a real boolean receipt.
- Topology laundering test: no production path may classify
  `TopologyTouchedGraphBasis` or `TopologyDeclaredTouchedGraphBasisProof` as a
  spatial evidence authority substitute.

**Engineering decisions**

- The inventory row must record source path, exported facade path, authority
  category, current caller, deletion action, owner, cap, and removal trigger.
- The first implementation step is not adding new admission helpers. It is
  classifying residue precisely enough that later replacement, migration, and
  deletion work cannot skip cleanup.
- If a path is needed only for compile-fail or certification proof, name it as
  certification support and keep it out of ordinary production facades.

**Open questions**

- None.

### Phase 2: Admission Contract And Denial Precedence

Freeze the ordinary admission contract before vocabulary and constructors are
introduced. This phase decides what can enter authority, what can only support
admission, what is derived after authority, and how denials are ordered.

**Relevant subsystems**

- `crates/worth-spatial/src/facade/workload_vocabulary`
- `crates/worth-spatial/src/workload_platform/evidence_ledger`
- `crates/worth-kernel/src/workload_composition`

**Relevant APIs**

- `BooleanEvidenceReceipt`
- `BooleanEvidenceRowAuthority`
- `CompleteWorkloadEvidenceLedger`
- `WorkloadEvidenceStageIndexProduct`
- `WorkloadEvidenceBooleanReceiptLookupProduct`
- `WorkloadEvidenceStageLinkSet`
- `WorkloadEvidenceLedgerError`

**Warnings**

- Do not let `BooleanEvidenceRowAuthority`, stage-index products, lookup
  products, Query descriptors, public status, or topology proof become alternate
  constructors.
- Do not postpone the receipt-only decision. Ordinary admission requires a
  complete ledger. Receipt-only mode is diagnostic preview only and cannot
  satisfy any production authority surface.
- Do not allow denial order to emerge from whichever check happens to run
  first.

**Test requirements**

- Contract parity test: every ordinary admission path uses sealed receipt plus
  complete-ledger proof and reports the same blocking contract when called from
  spatial, kernel, or certification entrypoints.
- Rejection test: receipt-only, row-only, lookup-only, Query-descriptor-only,
  topology-proof-only, and schema-vocabulary-only inputs all deny before any
  authority product is constructed.
- Denial precedence test: source-substitution denials dominate support denials,
  support denials dominate counter/stage-link denials, and spatial admission
  denials dominate Query expressiveness gaps.
- Diagnostic posture test: receipt-only diagnostic preview can expose a status
  row but cannot lower to Query, build lookup authority, satisfy replay, or
  pass closeout.

**Engineering decisions**

- The admission API must encode the contract in its type shape rather than
  relying on runtime comments.
- The denial enum must have explicit classes for source substitution, ledger
  incompleteness, support posture, counter honesty, stage-link failure,
  diagnostic-only posture, and Query gap.
- Multi-cause diagnostics may be emitted only after the stable blocking denial
  is chosen.

**Open questions**

- None.

### Phase 3: Spatial Touch Authority Vocabulary

Introduce the sealed vocabulary that names spatial touch authority separately
from topology touched basis and separately from Query descriptors. This phase
freezes the shape of the product later phases consume.

**Relevant subsystems**

- `crates/worth-spatial/src/workload_platform`
- `crates/worth-spatial/src/facade/workload_vocabulary`
- `crates/worth-schema/src/facade.rs`
- `crates/worth-topo/src/facade.rs`

**Relevant APIs**

- `BooleanEvidenceStageKind`
- `WorkloadEvidenceStage`
- `WorkloadEvidenceStageCounters`
- `WorkloadEvidenceCounters`
- `WorkloadEvidenceSupport`
- `WorkloadEvidenceStageLinkSet`
- `worth_schema::facade::platform::authority`
- `TopologyDeclaredTouchedGraphBasisProof`
- `TopologyTouchedGraphBasis`

**Warnings**

- Do not put admission authority in `worth-schema`. Schema may name shared
  spatial/topology vocabulary, but proof construction belongs in
  `worth-spatial`.
- Do not reuse topology touched-basis constructors, seals, or proof types. A
  spatial touch product can correlate with topology identity, but it is not
  topology truth.
- Do not model the authority product as a bag of strings. The product must
  encode stage, support, counters, receipt identity, ledger identity, and
  operating-world posture as named fields.

**Test requirements**

- Digest stability test: equivalent receipt/stage/link/counter inputs must
  produce the same spatial touch digest regardless of row order or caller
  construction order.
- Boundary denial test: raw ids, raw strings, copied receipt fields, schema
  vocabulary rows, topology touched-basis proofs, and Query descriptors cannot
  construct a spatial touch authority product.
- Counter honesty test: a spatial touch product with missing or zeroed counters
  where the source receipt reports work must deny or expose a violation row.
- Stage vocabulary test: every current `BooleanEvidenceStageKind` maps to a
  concrete `WorkloadEvidenceStage`, and adding a new boolean stage fails until
  spatial touch vocabulary handles it.

**Engineering decisions**

- Introduce a proof-bearing product shaped like
  `SpatialGeometryEvidenceTouchAuthority`, with private fields and read-only
  accessors.
- Introduce an admission request shaped like
  `SpatialGeometryEvidenceTouchRequest` only if it simplifies call sites
  without becoming a generic builder.
- Include counters and digest in the product. Later milestones must not
  rediscover breadth from the ledger.
- Keep proposed public exports under `worth_spatial::facade`, not deep module
  paths.

**Open questions**

- Exact type names are implementation-local, but the authority categories above
  are locked.

### Phase 4: Receipt And Ledger Admission

Make sealed boolean receipts and complete ledgers the only ordinary way to
admit spatial evidence into spatial touch authority. This phase closes the
gap between "the row exists in a ledger" and "the evidence is allowed to drive
touch proof."

**Relevant subsystems**

- `crates/worth-spatial/src/workload_platform/evidence_ledger`
- `crates/worth-spatial/src/workload_platform/planar_boolean_events`
- `crates/worth-spatial/src/workload_platform/planar_boolean_edge_splitting`
- `crates/worth-spatial/src/workload_platform/planar_boolean_loop_reconstruction`
- `crates/worth-kernel/src/workload_composition`

**Relevant APIs**

- `BooleanEvidenceReceipt::boolean_stage`
- `BooleanEvidenceReceipt::evidence_identity`
- `BooleanEvidenceReceipt::evidence_support`
- `BooleanEvidenceReceipt::evidence_counters`
- `BooleanEvidenceRowAuthority`
- `CompleteWorkloadEvidenceLedger::require_boolean_receipt`
- `CompleteWorkloadEvidenceLedger::require_boolean_receipt_lookup`
- `CompleteWorkloadEvidenceLedger::with_boolean_evidence_receipt`
- `CompleteWorkloadEvidenceLedger::link_required_stages`
- `WorkloadEvidenceLedger::stage_index`
- `WorkloadEvidenceLedger::guards`
- `WorkloadEvidenceLedger::covers_authority_stages`
- `WorkloadEvidenceLedgerError`
- `PlanarBooleanEventLedgerReceipt`
- `PlanarBooleanSegmentPairEnumerationReceipt`
- `PlanarBooleanSplitEdgeChainLedgerReceipt`
- `PlanarBooleanLoopReconstructionLedgerReceipt`

**Warnings**

- A sealed receipt is necessary but not automatically sufficient. The admitted
  product must prove the receipt is supported by the ledger/stage index and is
  not contradicted by guard errors or missing authority stages.
- Manual, unsupported, blocked, or certification-only evidence must not promote
  to ordinary touch authority.
- Receipt field copying is hostile. A struct with the same stage and identity
  is not a receipt unless it passes the sealed trait path.

**Test requirements**

- Replay/equivalence test: admitting the same sealed boolean receipt through a
  complete ledger after deterministic replay produces the same spatial touch
  digest, lookup key, stage counters, and support posture.
- Rejection test: manual rows, unsupported rows, missing authority stages,
  guard failures, and copied receipt-shaped structs all deny before Query
  descriptor construction.
- Receipt coverage test: each current boolean receipt implementor can admit
  through the ordinary path and reports the expected `BooleanEvidenceStageKind`.
- Ledger-locality test: admission counters prove the implementation uses the
  stage index or receipt lookup product rather than scanning the full evidence
  ledger for every request.

**Engineering decisions**

- Admission consumes a `BooleanEvidenceReceipt` or complete ledger proof and
  returns one spatial touch product. It does not return raw rows or mutable
  ledger references.
- The denial type must be structured and must preserve the failing source:
  receipt denial, ledger denial, guard denial, support denial, or stage-link
  denial.
- Kernel workload composition may call into the spatial authority facade, but
  it must not reconstruct the proof from ledger internals.

**Open questions**

- None.

### Phase 5: Spatial Evidence Lookup Identity

Define the evidence lookup identity that later boolean phases and Milestone 11
will consume. This phase separates spatial lookup products from Query
descriptors so neither can satisfy the other's contract.

**Relevant subsystems**

- `crates/worth-spatial/src/workload_platform/evidence_ledger/stage_index`
- `crates/worth-spatial/src/workload_platform/evidence_ledger`
- `crates/worth-spatial/src/facade/workload_vocabulary`

**Relevant APIs**

- `WorkloadEvidenceStageIndexProduct`
- `WorkloadEvidenceStageIndexProduct::index_identity`
- `WorkloadEvidenceStageIndexProduct::counters`
- `WorkloadEvidenceStageIndexProduct::require_boolean_receipt`
- `WorkloadEvidenceStageIndexProduct::require_boolean_receipt_lookup`
- `WorkloadEvidenceBooleanReceiptLookupProduct`
- `WorkloadEvidenceBooleanReceiptLookupProduct::boolean_stage`
- `WorkloadEvidenceBooleanReceiptLookupProduct::evidence_stage`
- `WorkloadEvidenceBooleanReceiptLookupProduct::evidence_identity`
- `WorkloadEvidenceBooleanReceiptLookupProduct::support`
- `WorkloadEvidenceBooleanReceiptLookupProduct::counters`
- `WorkloadEvidenceBooleanReceiptLookupProduct::lookup_counters`
- `WorkloadEvidenceBooleanReceiptLookupProduct::stage_index_identity`

**Warnings**

- Evidence lookup identity is not a Query descriptor digest. A Query descriptor
  may help select obligations; it cannot prove a receipt exists in the spatial
  evidence ledger.
- Do not expose raw evidence vectors as lookup products. The lookup product
  must carry stage, receipt identity, support, counters, stage-index identity,
  and spatial touch digest.
- Do not allow lookup to drift into a broad stage scan. The lookup key must
  explain the bounded access path.

**Test requirements**

- Lookup equivalence test: equivalent ledger/stage-index/receipt inputs produce
  the same lookup key and lookup product digest even when unrelated evidence
  rows exist in the ledger.
- Boundary denial test: wrong boolean stage, wrong evidence identity, wrong
  stage-index identity, unsupported support posture, and Query descriptor
  digest substitution all deny.
- Locality counter test: lookup counters prove the operation touched the
  expected stage-index slots and did not scan unrelated boolean stages.
- Separation test: a lookup product cannot be passed to APIs requiring
  `ForgeQueryGraphTouchDescriptor`, and a Query descriptor cannot be passed to
  APIs requiring spatial lookup authority.

**Engineering decisions**

- Introduce a lookup key product only if existing
  `WorkloadEvidenceBooleanReceiptLookupProduct` cannot carry the full identity
  needed by later phases. Do not wrap it cosmetically.
- If a new key is introduced, derive it from spatial touch authority and the
  stage index product, not from caller strings.
- Keep lookup products immutable and read-only at the facade.

**Open questions**

- Milestone 11 may expand lookup identity to include related topology touch
  digest. Milestone 4 should leave an explicit field or extension point only if
  it can do so without making topology digest required for all spatial evidence.

### Phase 6: First Downstream Consumer Migration

Migrate one real downstream spatial evidence consumer to the admitted spatial
touch authority path before Query lowering and public facade work grow around
an unused product. This phase proves the authority product is usable in the
ordinary workload path.

**Relevant subsystems**

- `crates/worth-kernel/src/workload_composition`
- `crates/worth-spatial/src/facade/workload_vocabulary`
- `crates/worth-spatial/src/workload_platform/evidence_ledger`

**Relevant APIs**

- `CompleteWorkloadEvidenceLedger`
- `WorkloadEvidenceBooleanReceiptLookupProduct`
- `BooleanEvidenceReceipt`
- `WorthWorkload` boolean evidence requirement surfaces
- the spatial touch authority product from Phase 4
- the lookup product from Phase 5

**Warnings**

- Do not migrate by deep-importing ledger internals into the kernel.
- Do not migrate a certification-only caller and call it production proof.
- Do not delete all old residue in this phase. Record what the first migration
  proves and what remains for closeout.

**Test requirements**

- Migration parity test: the migrated workload consumer receives the same
  receipt identity, support, counters, lookup key, and denial locality as the
  spatial facade proof product.
- Rejection test: the migrated consumer rejects manual rows, receipt-only
  diagnostic posture, raw vectors, Query descriptors, and topology proof before
  workload evidence can treat them as spatial authority.
- Dependency-direction test: the kernel consumes the spatial facade proof and
  does not reconstruct authority from ledger rows or stage indexes.
- Residue accounting test: old caller paths displaced by the first migration
  are classified as deleted, capped residue, or still pending closeout.

**Engineering decisions**

- Pick the smallest real workload consumer that exercises one current boolean
  receipt implementor and a complete ledger with unrelated evidence present.
- The migration should prove the API is implementable before Query lowering
  introduces derived proof products.
- The migrated path becomes the reference route for later caller cleanup.

**Open questions**

- None.

### Phase 7: Query Descriptor Lowering

Lower admitted spatial touch authority into Query's graph touch vocabulary
without giving Query descriptors the power to construct or replace spatial
authority. This phase creates the derived Query descriptor product Milestone 5
can consume without performing Milestone 5 selection.

**Relevant subsystems**

- `crates/forge-query/src/runtime/mutation/graph_composition/touch_descriptor`
- `crates/forge-query/src/runtime/mutation/graph_composition/obligation`
- `crates/worth-spatial/src/query_adoption.rs`
- `crates/worth-spatial/src/facade/query_adoption.rs`

**Relevant APIs**

- `ForgeQueryGraphTouchDescriptor`
- `ForgeQueryGraphTouchDescriptor::read_family`
- `ForgeQueryGraphTouchDescriptor::read_family_shape`
- `ForgeQueryGraphTouchDescriptor::live_read`
- `ForgeQueryGraphTouchDescriptor::live_read_shape`
- `ForgeQueryGraphTouchDescriptor::declared_mutation_collection`
- `ForgeQueryGraphTouchReadVerb`
- `ForgeQueryGraphReadTouchShape`
- `ForgeQueryGraphTouchSelector`
- `ForgeQueryGraphTouchSelector::collection`
- `ForgeQueryGraphTouchSelector::relation_kind`
- `ForgeQueryGraphTouchSelector::aspect_path`
- `ForgeQueryGraphTouchSelector::read_verb`
- `ForgeQueryGraphTouchSelector::declared_mutation_collection`
- `ForgeQueryGraphObligationOperatingWorldDescriptor`
- `ForgeQueryGraphObligationOperatingWorldDescriptor::any_committed_authority`
- `ForgeQueryGraphObligationOperatingWorldDescriptor::configured_domain_handle`

**Warnings**

- Spatial evidence reads should prefer Query's read-family descriptor lane:
  `read_family` or `read_family_shape` with verbs such as
  `ObservesCollection`, `ObservesRelationKind`, `ObservesAspectPath`,
  `ExposesDerivedTopology`, `MaterializesDiagnostic`,
  `RequiresPolicyBasis`, or `CrossesOperatingWorld`.
- Use `declared_mutation_collection` only when spatial evidence is actually
  declaring graph mutation meaning. Do not pretend a geometry lookup is a
  mutation to gain selector coverage.
- If Query lacks the descriptor expressiveness for a spatial evidence fact,
  create a typed Query-gap row. Do not add a Worth-local selector dialect.
- Do not select or execute the full Milestone 5 obligation set here. This phase
  proves descriptor lowering and gap posture only.

**Test requirements**

- Descriptor parity test: the same admitted spatial touch authority lowers to
  the same Query descriptor digest and operating-world descriptor across replay
  and through each public facade path.
- Denial test: raw rows, copied receipts, topology touched-basis proofs, and
  previously lowered Query descriptors cannot call the lowering API without the
  spatial touch authority product.
- Selector precision test: collection, relation-kind, aspect-path, read-verb,
  and declared-mutation selectors match only the descriptors the spatial touch
  product says they should match.
- Query-gap test: a spatial evidence stage that cannot be expressed by Query
  produces a structured gap with owner, cap, blocker, and removal trigger
  rather than a local adapter.
- Milestone-boundary test: Query lowering can build a descriptor proof without
  claiming full Query obligation selection closeout.

**Engineering decisions**

- The lowering module should be a narrow child responsibility of spatial touch
  authority, not part of the ledger or boolean stage implementation.
- Store Query descriptor digest and operating-world digest only on the derived
  Query lowering product. Never store them as spatial admission source truth.
- Use Query's public facade exports. Do not deep-import Query internal modules
  from Worth.

**Open questions**

- Whether spatial evidence needs a dedicated Query support lane is a Query
  design question. Until Query adds one, use existing covered lanes honestly
  and record gaps instead of minting a local lane.

### Phase 8: Consumer Kit Adoption And Residue Proof

Prove that `worth-spatial` consumes Query graph obligation authority through
Query's Consumer Kit instead of local proof machinery.

**Relevant subsystems**

- `crates/forge-query/src/consumer_kit/graph_obligation_adoption`
- `crates/forge-query/docs/authoring/graph-obligation-consumer-kit.md`
- `crates/worth-spatial/src/query_adoption.rs`
- `crates/worth-spatial/src/facade/query_adoption.rs`

**Relevant APIs**

- `graph_obligation_consumer_kit`
- `ForgeQueryGraphObligationConsumerRegistrationDeclaration::for_runtime_family`
- `ForgeQueryGraphObligationSelectorCoverageDeclaration::required`
- `ForgeQueryGraphObligationSupportPin`
- `ForgeQueryGraphObligationSupportMatrix`
- `ForgeQueryGraphObligationLocalCeremonyAudit`
- `ForgeQueryGraphObligationResidueManifest`
- `ForgeQueryGraphObligationInMemoryTestWorkspace`
- `ForgeQueryGraphObligationInMemoryProof`
- `ForgeQueryGraphObligationExecutionProof`
- `ForgeQueryGraphObligationExecutionBackedAdoptionProof`
- `ForgeQueryGraphObligationKind`
- `ForgeQueryGraphObligationSupportLane`
- `ForgeQueryGraphObligationSupportStatus`

**Warnings**

- Consumer Kit adoption proof is not optional documentation. It is the
  mechanical proof that `worth-spatial` is not hand-rolling Query ceremony.
- In-memory selection proof is not execution proof. Where the milestone claims
  execution-backed adoption, use `prove_execution_with` and
  `prove_adoption_with_execution`.
- Residue rows must decrease or stay capped. Do not add a broad permanent
  residue class for "spatial evidence is special."

**Test requirements**

- Adoption proof test: `worth-spatial` can build a Consumer Kit adoption proof
  from registration declaration, selector coverage, support pin, support
  matrix, local ceremony audit, residue manifest, and execution-backed proof.
- Rejection test: missing registration, uncovered selector, unsupported pin,
  unevaluated local ceremony audit, non-empty uncapped residue, or in-memory
  proof substituted for execution proof fails adoption.
- Precision test: selected obligation counts, attempted bucket lookup counts,
  candidate registration counts, denied row counts, residue counts, and full
  scan counts are exposed and asserted for spatial descriptors.
- Hard-break test: local report structs, local digest strings, local support
  row lists, local source greps, and fabricated Query receipts are deleted or
  mechanically reported as capped residue.

**Engineering decisions**

- Keep the adoption proof facade small: status, counters, manifest digest,
  residue count, and selected obligation rows are enough for public inspection.
- Use `current_spatial_query_consumer_kit_adoption_status`,
  `current_spatial_phase_eight_performance_counters`,
  `current_spatial_workload_support_pin_rows`, and
  `spatial_query_adoption_inventory` as existing reporting surfaces where they
  remain honest.
- If an existing spatial adoption report duplicates Consumer Kit proof, delete
  or collapse it into a projection of the Consumer Kit product.

**Open questions**

- Whether every spatial stage needs one registration family or stage-specific
  families should be decided by selector precision and support posture, not by
  convenience.

### Phase 9: Public Facade And Compile-Fail Fences

Expose the spatial touch authority status as a read-only public product while
making every invalid construction path mechanically fail. This phase turns the
architecture from convention into enforcement.

**Relevant subsystems**

- `crates/worth-spatial/src/facade`
- `crates/worth-spatial/src/facade/workload_vocabulary`
- `crates/worth-spatial/src/facade/query_adoption.rs`
- `crates/worth-spatial/src/certification`
- `crates/worth-kernel/src/certification/public_facade_contracts`

**Relevant APIs**

- `worth_spatial::facade::workload_vocabulary::*`
- `worth_spatial::facade::query_adoption::*`
- `BooleanEvidenceReceipt`
- `BooleanEvidenceRowAuthority`
- `CompleteWorkloadEvidenceLedger`
- `WorkloadEvidenceLedger`
- `WorkloadEvidenceRow`
- `WorkloadEvidenceBooleanReceiptLookupProduct`
- `ForgeQueryGraphTouchDescriptor`
- `ForgeQueryGraphObligationExecutionBackedAdoptionProof`

**Warnings**

- Public read-only accessors are allowed. Public constructors, mutable fields,
  raw row admission, or exposed seals are not allowed.
- Tests must prove outside crates cannot implement `BooleanEvidenceReceipt` or
  `BooleanEvidenceRowAuthority` as an authority path.
- Do not hide facade problems by moving forbidden constructors into test-only
  helpers that production can still reach through feature flags.

**Test requirements**

- Compile-fail denial test: an external crate cannot implement spatial receipt
  authority, construct the spatial touch product directly, construct authority
  from raw rows, or substitute Query descriptors/topology proofs for spatial
  authority.
- Public facade parity test: every public status/accessor path reports the
  same digest, stage, support, counter, Query descriptor digest, and residue
  posture as the internal proof product.
- Feature-gate leakage test: certification/test support cannot expose a
  production-usable constructor through `cfg(test)`, docs, examples, or hidden
  exports.
- Naming honesty test: public names distinguish receipt authority, lookup
  product, Query descriptor, adoption proof, and topology basis without generic
  `adapter`, `helper`, `manager`, or `bridge` language.

**Engineering decisions**

- Public facade files may aggregate exports but must not implement admission
  behavior.
- Compile-fail fixtures should live with the existing public facade contract
  strategy rather than as scattered one-off tests.
- Any new public type must expose the proof it carries in its name. If the name
  cannot say what was proven, the type is not ready to be public.

**Open questions**

- None.

### Phase 10: Cross-Crate Adoption And Milestone Closeout

Replace downstream spatial evidence callers with the new authority path and
close the milestone with deletion, counters, docs, and roadmap proof. This
phase confirms that the implementation is the ordinary route, not a side
artifact.

**Relevant subsystems**

- `crates/worth-spatial`
- `crates/worth-kernel/src/workload_composition`
- `crates/worth-topo`
- `_docs/worth/touched-graph-roadmap.md`
- `_docs/worth/milestone-7-roadmap.md`

**Relevant APIs**

- `worth_kernel` workload evidence consumption APIs
- `worth_spatial::facade::workload_vocabulary`
- `worth_spatial::facade::query_adoption`
- `worth_topo::facade::topology_operator_touch_descriptor_from_touched_graph_basis`
- `TopologyDeclaredTouchedGraphBasisProof`
- Consumer Kit adoption proof APIs from Phase 8

**Warnings**

- Kernel adoption must call the spatial facade. It must not deep-import ledger
  internals or reconstruct touch proof from rows.
- `worth-topo` must remain topology-owned. It may keep its topology descriptor
  lowering path, but it must not import spatial admission authority as a hidden
  bridge.
- Milestone closeout requires deletion of obsolete projection/admission/type
  guard residue. Merely routing new code through the new path is not enough.

**Test requirements**

- Cross-crate replay test: a kernel workload using boolean evidence obtains the
  same spatial authority digest, lookup key, Query descriptor digest, and
  adoption proof after replay as it did during original execution.
- Residue rejection test: reintroducing a deleted raw-row, broad-stage-scan,
  topology-laundering, local Query support, or type-name-guard path fails
  certification.
- Dependency-direction test: `worth-spatial` depends on Query facade and shared
  schema vocabulary as needed, `worth-kernel` consumes spatial facade proof,
  and `worth-topo` does not become a spatial evidence adapter.
- Roadmap closeout test: Milestone 4 acceptance rows, residue counts, public
  facade surfaces, and Query adoption counters match the implementation and do
  not claim Milestone 5 or Milestones 6 through 8 work as complete.

**Engineering decisions**

- Closeout must include exact deleted surfaces, capped residue rows, and the
  reason any residue remains.
- Run line-cap and composition checks for touched Rust files. Splitting files
  touched during this milestone is part of the work unless an explicit
  exemption exists.
- Update docs only after the architecture is implemented and tests prove the
  claims. Docs may not get ahead of certified behavior.

**Open questions**

- None.

## Admitted Surface

- sealed boolean/spatial evidence receipt admission
- complete-ledger-backed spatial touch authority
- stage-index-backed receipt lookup
- typed denial for manual, unsupported, blocked, missing, copied, foreign, and
  topology-laundered evidence
- Query read-family descriptor lowering from spatial authority
- Query descriptor gap rows where current Query vocabulary is insufficient
- Consumer Kit registration, selector coverage, support pin, local ceremony
  audit, residue manifest, in-memory proof, execution proof, and adoption proof
- public read-only spatial authority status and diagnostics
- compile-fail fences against forged receipts, forged authority products, raw
  rows, and Query/topology substitution

## Excluded Surface

- Query obligation selection for all touched graph products; that is Milestone
  5
- covered Worth graph-read access inventory, declarations, and access plans;
  those are Milestones 6 through 8
- broad boolean evidence lookup across later boolean phases; that is Milestone
  11
- topology validator and invariant derivation; that is Milestone 9
- replay, undo, conflict, cache, and public diagnostic closeout for the whole
  touched-graph program; those are Milestones 12 through 16
- final boolean overlap extraction, fragment classification, face assembly,
  or cleanup
- curved-edge, NURBS, extrusion, fillet, seam, periodic-surface, or trim-network
  spatial authority beyond support-gated posture

## Query Boundary Non-Goal

Milestone 4 may prove that `worth-spatial` can lower admitted spatial authority
into Query descriptor/adoption products. It must not claim that Query has
selected every obligation needed for all touched graph products.

Allowed in Milestone 4:

- build a Query touch descriptor from spatial authority
- build an operating-world descriptor
- prove selector precision for representative spatial descriptors
- prove Consumer Kit adoption for `worth-spatial`
- record typed Query gaps with caps and removal triggers

Deferred to Milestone 5:

- selecting the full obligation set from topology and spatial touched products
- making Query obligation selection the shared cross-branch authority product
- deleting selector adapters whose replacement requires Milestone 5 selection
- certifying selected obligations for all touched graph product families

## Workflow Surface

Milestone 4 is not done because one receipt can be wrapped into a descriptor.

It is only done when admitted spatial touch workflows operate generically over:

- every current boolean receipt implementor exposed by `worth-spatial`
- complete ledgers with unrelated evidence rows present
- stage indexes containing multiple boolean stages
- replayed ledgers and benign row-order variation
- unsupported, blocked, manual, certification-only, and missing-stage evidence
- topology-correlated evidence that still requires spatial receipt authority
- Query descriptor lowering for read-family spatial evidence
- Query-gap posture when spatial evidence cannot be expressed by existing Query
  descriptor vocabulary
- public facade consumers that inspect status without gaining constructors

## Operator Closure

Milestone 4 closes the following named authority families. Each family must be
classified before implementation closeout as one of:

- `SpatialReceiptAdmission`
- `SpatialAuthorityProduct`
- `SpatialLookupProduct`
- `QueryDescriptorLowering`
- `QueryConsumerKitAdoption`
- `PublicFacadeProof`
- `CertificationOnlyResidue`
- `DeletedResidue`

No family may remain an unclassified helper name.

Admission families:

- `AdmitSpatialTouchFromBooleanEvidenceReceipt`
- `AdmitSpatialTouchFromCompleteEvidenceLedger`
- `RejectManualEvidenceRowAsSpatialTouchAuthority`
- `RejectCopiedReceiptFieldsAsSpatialTouchAuthority`
- `RejectTopologyTouchedBasisAsSpatialTouchAuthority`
- `RejectSchemaVocabularyAsSpatialTouchAuthority`

Lookup families:

- `BuildSpatialEvidenceLookupKey`
- `BindSpatialTouchAuthorityToStageIndex`
- `RequireSpatialBooleanReceiptLookup`
- `RejectWrongStageSpatialEvidenceLookup`
- `RejectQueryDescriptorAsSpatialLookupAuthority`

Query lowering families:

- `LowerSpatialTouchAuthorityToQueryReadFamilyDescriptor`
- `LowerSpatialTouchAuthorityToQueryOperatingWorld`
- `RecordSpatialEvidenceQueryGap`
- `RejectQueryDescriptorAsSpatialAdmissionSource`

Consumer Kit families:

- `RegisterSpatialEvidenceGraphObligations`
- `DeclareSpatialEvidenceSelectorCoverage`
- `PinSpatialEvidenceGraphObligationSupport`
- `AuditSpatialEvidenceLocalQueryCeremony`
- `AccountForSpatialEvidenceQueryResidue`
- `ProveSpatialEvidenceQueryExecutionAdoption`

Public and closeout families:

- `ExposeSpatialGeometryEvidenceTouchStatus`
- `ExposeSpatialGeometryEvidenceTouchDiagnostics`
- `FenceSpatialTouchAuthorityConstructors`
- `DeleteSpatialProjectionAdmissionResidue`
- `DeleteSpatialTypeNameGuardResidue`
- `CertifySpatialAuthorityExclusivity`

## Validator Closure

Milestone 4 closes these validator families at spatial touch authority scope:

- request and admission validators:
  - `ValidateSpatialTouchReceiptSeal`
  - `ValidateSpatialTouchLedgerCompleteness`
  - `ValidateSpatialTouchStageSupport`
  - `ValidateSpatialTouchEvidenceCounters`
  - `ValidateSpatialTouchStageLinkCoverage`
- substitution validators:
  - `RejectManualRowsAsSpatialAuthority`
  - `RejectCopiedReceiptFieldsAsSpatialAuthority`
  - `RejectRawEvidenceVectorsAsLookupAuthority`
  - `RejectTopologyBasisAsSpatialAuthority`
  - `RejectQueryDescriptorAsSpatialAuthority`
  - `RejectSchemaVocabularyAsSpatialAuthority`
- Query lowering validators:
  - `ValidateSpatialQueryDescriptorDigestStability`
  - `ValidateSpatialQuerySelectorPrecision`
  - `ValidateSpatialQueryOperatingWorldPosture`
  - `ValidateSpatialQueryGapRowsAreCapped`
- Consumer Kit validators:
  - `ValidateSpatialConsumerKitRegistrationDeclaration`
  - `ValidateSpatialConsumerKitSelectorCoverage`
  - `ValidateSpatialConsumerKitSupportPin`
  - `ValidateSpatialConsumerKitExecutionBackedProof`
  - `ValidateSpatialQueryResidueManifestCaps`
- public and closeout validators:
  - `ValidateSpatialFacadeReadOnlyProof`
  - `ValidateSpatialCompileFailFenceCoverage`
  - `ValidateSpatialAuthorityDependencyDirection`
  - `ValidateSpatialAuthorityRoadmapClaims`

Validators that govern Query consumption must close through Query Consumer Kit
proof. Validators that govern spatial admission must close through sealed
spatial evidence and ledger proof. Manual executor-local validation is allowed
only as a thin helper around those routes, not as the authority for closeout.

## Workload Composition Additions

- Add a dedicated spatial touch authority path in `worth-spatial` rather than
  overloading the evidence ledger or Query adoption inventory.
- Add kernel workload consumption that asks `worth-spatial` for spatial touch
  proof instead of deep-importing ledger internals.
- Add boolean evidence requirement mapping for spatial touch authority where
  existing workload evidence consumption needs the new product.
- Add workload catalog or certification recipes that exercise real current
  boolean receipt implementors with unrelated evidence rows present.
- Add public-contract tests proving synthetic receipts, manual rows, raw
  vectors, Query descriptors, and topology touched-basis proofs cannot satisfy
  spatial touch authority.
- Add Query Consumer Kit adoption proof that uses real spatial descriptors and
  asserts selection/execution counters.

## Replay Closure

Replaying the same spatial evidence request must preserve:

- receipt identity
- boolean evidence stage
- evidence support posture
- evidence counters
- stage-index identity
- stage-link proof
- spatial touch digest
- lookup key identity
- Query descriptor digest
- operating-world descriptor digest
- Consumer Kit adoption manifest digest
- residue manifest digest
- denial kind and denial locality
- public status digest

## Diagnostics Closure

Denials must localize whether failure occurred at:

- receipt seal admission
- ledger completeness
- stage-index lookup
- evidence support posture
- evidence counters
- stage-link binding
- manual-row rejection
- copied-receipt rejection
- topology-basis substitution
- schema-vocabulary substitution
- Query descriptor substitution
- Query descriptor expressiveness
- Consumer Kit registration
- selector coverage
- support pinning
- local ceremony audit
- residue manifest cap
- execution-backed adoption proof
- public facade construction fence
- cross-crate dependency direction

## Determinism Closure

Milestone 4 must make the following stable:

- authority product identity
- receipt ordering when multiple receipts are admitted
- stage-index lookup ordering
- selector coverage row ordering
- support pin row ordering
- residue manifest row ordering
- diagnostic ordering
- public status ordering
- Query descriptor digest
- lookup key digest
- Consumer Kit adoption manifest digest

## Complexity / Proof Closure

- Spatial touch work must expose counters for:
  - receipt rows admitted
  - manual rows rejected
  - unsupported rows rejected
  - missing authority stages
  - stage-index slots touched
  - boolean stages inspected
  - lookup rows emitted
  - Query descriptors emitted
  - Query gaps emitted
  - selected obligations
  - attempted Query bucket lookups
  - candidate Query registrations
  - denied Query registration rows
  - local ceremony findings
  - capped residue rows
- The complexity boundary starts at sealed receipt or complete ledger admission
  and continues through stage-index lookup and Query descriptor lowering.
  Repeated broad ledger scans cannot be the production-confidence proof path.
- Diagnostic richness must not change spatial touch identity, Query descriptor
  identity, lookup identity, or operational counters.

## Allowed Debt

- No debt is allowed that lets raw rows, manual rows, copied receipt fields, or
  raw evidence vectors satisfy production spatial touch authority.
- No debt is allowed that lets topology touched-basis proof or Query descriptor
  proof satisfy spatial evidence admission.
- No debt is allowed that keeps local Query reports, local support rows, or
  fabricated receipts as ordinary proof.
- Query descriptor expressiveness gaps are allowed only as capped, owned,
  typed gap rows with removal triggers.
- Full graph-read access plan adoption remains deferred to Milestones 6 through
  8.
- Full evidence lookup across later boolean phases remains deferred to
  Milestone 11.
- Curved/NURBS/extrusion/fillet authority remains support-gated future work.

## Milestone Done When

- every current boolean receipt implementor can enter one canonical spatial
  touch admission boundary
- manual rows, unsupported rows, copied receipts, raw vectors, topology proofs,
  Query descriptors, and schema vocabulary rows cannot construct authority
- admitted spatial touch products carry stable spatial admission digest,
  counters, stage, support, ledger identity, stage-index identity, and
  stage-link proof
- derived lookup and Query products carry separate lookup identity, Query
  descriptor digest, and operating-world digest
- Query descriptor lowering uses Query public facade vocabulary and records
  typed gaps rather than local selector dialects
- `worth-spatial` proves Query adoption through Consumer Kit with
  execution-backed proof where claimed
- kernel workload composition consumes the spatial facade proof and does not
  reconstruct evidence authority from ledger internals
- public facades expose read-only proof/status without constructors
- obsolete projection/admission/type-name/local-Query residue is deleted or
  capped with owner, count, blocker, and removal trigger
- the handoff to Milestone 5 is the spatial Query descriptor/adoption proof,
  not a local selector adapter

## Must Ship

- A sealed spatial touch authority product admitted from boolean/spatial
  evidence receipts and complete ledger proof.
- A structured denial surface for manual rows, unsupported evidence, copied
  receipts, missing ledger authority, stage-link failures, topology laundering,
  and Query expressiveness gaps.
- Query descriptor lowering through `ForgeQueryGraphTouchDescriptor` and
  `ForgeQueryGraphObligationOperatingWorldDescriptor`.
- Spatial evidence lookup identity that stays separate from Query descriptors.
- Consumer Kit-backed Query adoption proof for `worth-spatial`.
- Public read-only facade status and compile-fail fences against forged
  authority.
- Deletion or capped residue for projection/admission/type-name bridges, raw
  evidence scans, local Query proof, and topology geometry-only lowering.

## Must Preserve

- `worth-spatial` owns spatial evidence and boolean receipt authority.
- `forge-query` owns Query descriptors, selectors, support posture, obligation
  selection, execution proof, and Consumer Kit adoption.
- `worth-schema` remains vocabulary and cannot admit proof.
- `worth-topo` owns topology touched graph basis and cannot substitute for
  spatial evidence.
- Evidence lookup products, Query descriptors, topology basis proofs, and
  Consumer Kit adoption proofs remain distinct products.
- Ordinary execution breadth stays bounded by the admitted receipt/stage touch.

## Acceptance Evidence

- Inventory and deletion ledger for all old spatial evidence authority surfaces.
- Unit, integration, replay, counter, and compile-fail tests named in every
  phase.
- Consumer Kit execution-backed adoption proof for representative spatial
  evidence descriptors.
- Public facade contract tests proving read-only status without public
  constructors.
- Cross-crate tests proving kernel consumption uses spatial authority and topo
  does not launder spatial proof.
- Documentation updates that cite only certified behavior and exact capped
  residue.

## Representative Acceptance Matrix

The test suite must cover this minimum matrix. More cases are allowed, but the
runner may not treat the milestone as done with fewer.

| Scenario | Expected product | Required denial or proof |
| --- | --- | --- |
| Sealed event-ledger receipt plus complete ledger | `SpatialGeometryEvidenceTouchAuthority` | stable authority digest and stage-index proof |
| Sealed segment-pair receipt plus complete ledger | `SpatialGeometryEvidenceTouchAuthority` | stable authority digest and receipt lookup proof |
| Sealed split-edge-chain receipt plus complete ledger | `SpatialGeometryEvidenceTouchAuthority` | stable authority digest and lookup key |
| Sealed loop-reconstruction receipt plus complete ledger | `SpatialGeometryEvidenceTouchAuthority` | stable authority digest and stage support proof |
| Sealed receipt without complete ledger | diagnostic status only | diagnostic-only denial; no lookup, Query, replay, or closeout authority |
| Manual row with matching stage and identity | no authority | manual-row source-substitution denial |
| Copied receipt-shaped struct | no authority | copied-receipt/source-substitution denial |
| Unsupported or blocked support posture | no authority | support-posture denial |
| Missing stage link or missing authority stage | no authority | stage-link or ledger-completeness denial |
| Topology touched-basis proof as input | no authority | topology-laundering denial |
| Query descriptor as input | no authority | Query-substitution denial |
| Valid authority lowered to Query read descriptor | derived Query descriptor | descriptor digest, operating-world digest, no Milestone 5 selection claim |
| Query descriptor expressiveness missing | typed Query gap | capped gap row with owner, blocker, and removal trigger |
| Public facade inspection | read-only status | no public constructors or mutable authority fields |
| Replayed valid receipt/ledger | same products | authority, lookup, Query descriptor, and status digests preserved |

## Sequencing Notes

Milestone 4 does not select Query obligations for all touched graph products;
that is Milestone 5. It does not inventory or migrate covered Worth graph-read
access plans after Query `9.10`; that is Milestones 6 through 8. It does not
finish broad boolean evidence lookup for all later stages; that is Milestone
11.

The correct handoff to Milestone 5 is a topology touched-basis Query descriptor
from Milestone 3 plus the spatial Query descriptor/adoption proof produced
here. If either branch is missing, Milestone 5 must deny rather than invent a
local selector path.

## Required Self-Check

- This milestone solves a real structural problem: spatial evidence authority
  currently risks being treated as raw rows, topology proof, or local Query
  ceremony.
- The adversarial constraint is load-bearing: long boolean chains with narrow
  local evidence inside large unrelated state force sealed proof and bounded
  lookup.
- The roadmap justifies this milestone now because Milestone 5 needs spatial
  Query descriptors as input.
- Crate authority boundaries are preserved: spatial admits evidence, Query
  proves Query consumption, topology owns topology, schema owns vocabulary.
- The phases carry the design and each phase centers one transition.
- Each phase includes adversarial tests for equivalence/locality and
  denial/residue.
- A competent engineer can map the spec into types, modules, facade exports,
  denials, and tests without needing a second architecture document.
