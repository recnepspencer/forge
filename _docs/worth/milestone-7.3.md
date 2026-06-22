# Worth Milestone 7.3: Edge Splitting And Edge-Chain Normalization

> **Status:** Draft
>
> **Purpose:** freeze the canonical edge-level split products that consume the
> `7.2` planar boolean event ledger and feed `7.4` loop reconstruction without
> reopening segment relation, projection, plane, or workload-entry authority.

## Goal

Milestone `7.3` closes the gap between the `7.2` event ledger and honest
loop-level reconstruction.

By the end of this milestone:

- a `7.2` event ledger can enter one and only one edge-splitting request
  boundary
- point events, endpoint contacts, shared endpoints, and interval events are
  lowered into edge-local split schedules with source topology provenance
- T-junctions, endpoint no-ops, duplicate reports, micro-intervals, partial
  overlaps, containment overlaps, and opposite-sense coincidences each have
  explicit split posture
- split vertices, split fragments, and overlap edge chains receive canonical,
  replay-stable identities
- persistent-name and subshape-signature propagation through split is explicit
  enough for later loop reconstruction, overlap extraction, and classification
  to inherit
- workload evidence, replay, diagnostics, and public-contract fences prove that
  split work consumed the real event ledger rather than synthetic rows

Milestone `7.3` does **not** rebuild loops, extract overlap regions, classify
fragments, assemble faces, clean final topology, or certify final boolean
results. It freezes the canonical split edge-chain product those later phases
must consume.

## Why This Milestone Exists

The tempting mistake after `7.2` is to treat edge splitting as a local topology
helper: gather event points, sort by parameter, split edges, and move on.

That is not enough for Worth. Edge splitting is where event truth first becomes
topology-rewrite intent. If this milestone loses source edge provenance,
coalesces vertices by coordinate folklore, collapses interval sense, invents
new split identities from debug strings, or lets later loop work recompute raw
segment relations, the planar boolean lane becomes impossible to trust.

`7.3` therefore makes edge splitting a receipt-backed, phase-typed, operator-
named boundary. It does not yet commit final topology truth, but it must produce
the exact canonical edge-chain ledger that topology reconstruction can consume
without rediscovering geometry.

## Governing Summaries

- `MENTALITY.md`: protects adversarial-constraint-first design. `7.3` must
  solve the hostile edge-splitting authority problem before loop reconstruction
  can claim progress.
- `arch_laws.md`: protects proof-bearing phase transitions and authority
  separation. Every split phase must consume the previous proof artifact and
  produce a stronger one; no phase may accept raw events when a receipt-backed
  artifact exists.
- `composition_laws.md`: protects semantic decomposition. Split request,
  source-edge recovery, point parameterization, interval parameterization,
  T-junction handling, schedule normalization, vertex identity, fragment
  construction, name propagation, diagnostics, and certification must remain
  separate named responsibilities.
- `domain_structure_laws.md`: protects visible ownership. Query remains the
  runtime entry and retained-artifact owner; `worth-kernel` owns workload
  composition and evidence pressure; `worth-spatial` owns planar split
  semantics and edge-chain artifacts; `worth-topo` remains topology truth and
  topology-operator authority.
- `perf_laws.md`: protects visible breadth and carry-forward proof. Split work
  must expose schedule, cut, fragment, overlap-chain, collapse, and denial
  counters rather than hiding broad loops behind cheap-looking APIs.
- `_docs/worth/worth_roadmap.md`: protects workflow closure over toy examples.
  `7.3` must operate over the admitted planar boolean edge-event workflow class,
  not only over a few hand-picked crossing examples.
- `_docs/worth/milestone-7-roadmap.md`: protects `7.3` as edge splitting and
  edge-chain normalization only. Loop reconstruction, overlap-region
  extraction, classification, assembly, cleanup, and topology legality remain
  later `7.x` milestones.
- `_docs/worth/milestone-7.0.md`: protects the Query/workload entry and
  anti-theatre boundary. `7.3` must inherit the same entry proof through the
  event ledger.
- `_docs/worth/milestone-7.1.md`: protects common-plane and local-frame
  reduction. `7.3` must not reselect plane, frame, projection, or precision
  facts.
- `_docs/worth/milestone-7.2.md`: protects event-ledger authority. `7.3` must
  consume `PlanarBooleanEventLedgerReceipt` and must not recompute segment
  relations.
- `_docs/worth_topo/operators-list.md`: protects the operator vocabulary that
  makes topology surgery explicit. `7.3` must name the split, imprint, overlap,
  identity, decision-log, and anti-theatre operators it adds instead of hiding
  them behind vague "edge split" prose.
- `_docs/worth_topo/validators.md`: protects validator-family closure. `7.3`
  must split validation work across enough phases that edge schedule, vertex,
  fragment, overlap, naming, replay, and anti-theatre proof each get direct
  tests.
- `crates/forge-query/docs/AI_README.md`: protects the rule `declare intent
  once, lower it once, execute or inspect it through canonical runtime-owned
  artifacts`. `7.3` may add domain split artifacts, but it must not invent a
  caller-owned split route, pseudo-Query runtime lane, or local support posture.

## Adversarial Constraint

Given a real Query/workload-composed planar boolean operand pair that has
successfully produced a `7.2` event ledger bound to a Query-owned segment
candidate-index product, edge splitting must either:

- deny before split-ledger construction with a typed, localized, replay-stable
  reason

or:

- emit one canonical split edge-chain ledger whose split request identity,
  source-edge carriers, point parameters, interval parameters, T-junction
  promotions, normalized schedules, split vertices, split fragments, overlap
  edge chains, naming propagation, counters, decision log, diagnostics, and
  downstream-consumption identity remain stable across replay, reversed source
  edge sense, duplicate event reports, opposite-sense coincidence, overlap
  interval pressure, endpoint no-op pressure, and benign input-order variation.

If `7.4` still has to ask raw event, segment-relation, projection, or
common-plane questions instead of consuming a `7.3` split edge-chain receipt,
this milestone has failed.

## Product Decision Lock

- `7.3` starts from `PlanarBooleanEventLedgerReceipt` and nowhere else.
- That event-ledger receipt must be bound to the Query-owned segment
  candidate-index product before split request admission; cross-product-only
  discovery or Query-digest-decorated host-local discovery cannot satisfy
  production closeout.
- Query continues to own declaration, admission, support posture, runtime
  handles, retained artifact progression, receipts, envelopes, inspection, and
  ordinary outcomes.
- `worth-kernel` owns workload composition, stage requirements, evidence rows,
  catalog hostile recipes, public anti-theatre fences, and closeout pressure.
- `worth-spatial` owns split request semantics, point/interval lowering,
  schedule normalization, split-edge-chain artifacts, split diagnostics,
  counters, and replay identity.
- `worth-topo` owns topology truth, topology operator public surfaces, and the
  eventual execution of topology rewrites. `7.3` may prepare split topology
  intent and consume topology provenance, but it does not finalize loop or face
  topology.
- Persistent naming through split is admitted as split-level lineage and
  subshape-signature propagation only; merge/conflict naming semantics remain
  later roadmap work.
- `Milestone 8` remains EMBER. `7.3` stays in the B-rep planar lane.

## Existing Surface Inventory

Milestone `7.3` should widen live surfaces before inventing new ones:

- `crates/worth-spatial/src/facade/planar_boolean_events.rs`
  - `PlanarBooleanEventLedgerReceipt`
  - `PlanarBooleanEventLedger`
  - `PlanarBooleanOrderedEventSet`
  - `PlanarBooleanPointEvent`
  - `PlanarBooleanPointEventKind`
  - `PlanarBooleanPointEventSegmentParameterFact`
  - `PlanarBooleanIntervalEvent`
  - `PlanarBooleanIntervalEventKind`
  - `PlanarBooleanNormalizedInterval`
  - `PlanarBooleanSourceInterval`
  - `PlanarBooleanSourceIntervalSense`
  - `PlanarBooleanEventGroup`
  - `PlanarBooleanEventGroupKind`
  - `PlanarBooleanSegmentCarrier`
  - `PlanarBooleanSegmentCarrierSet`
  - `PlanarBooleanSegmentPairEnumerationReceipt`
- `crates/worth-spatial/src/workload_platform/planar_boolean_events/pair_enumeration/worklist.rs`
  - current full-breadth pair-enumeration precedent that `7.3` must not accept
    as the final production-confidence discovery proof
- `crates/worth-spatial/src/workload_platform/planar_boolean_events/*`
  - event-ledger, point-event, interval-event, event-grouping, segment-carrier,
    endpoint-normalization, collinear-relation, and predicate-binding modules
- `crates/worth-spatial/src/workload_platform/evidence_ledger/*`
  - `BooleanEvidenceReceipt`
  - `BooleanEvidenceStageKind`
  - `CompleteWorkloadEvidenceLedger`
  - `WorkloadEvidenceStage`
  - `WorkloadEvidenceStageCounters`
- `crates/worth-kernel/src/workload_composition/worth_workload.rs`
  - `WorthWorkload`
  - `WorthWorkload::require_boolean_event_ledger`
  - prior `require_boolean_*` evidence gates from `7.0`, `7.1`, and `7.2`
- `crates/worth-kernel/src/workload_composition/stage_requirements.rs`
  - `WorkloadStageRequirement`
- `crates/worth-kernel/src/workload_composition/boolean_event_extraction/*`
  - `PlanarBooleanEventExtractionRequest`
- `crates/worth-kernel/src/workload_composition/boolean_evidence.rs`
  - prior `BooleanEvidenceReceipt` implementations for `7.0`, `7.1`, and
    `7.2` artifacts
- `crates/worth-kernel/src/workload_composition/workload_catalog/*`
  - boolean workload catalog recipes and hostile recipe substrate
- `crates/worth-topo/src/certification/public_facade_contracts/contracts/public_api_topology_operator_split_surface.rs`
  - topology split public-contract precedent
- `crates/worth-topo/src/certification/public_facade_contracts/contracts/public_api_topology_operator_surface.rs`
  - topology operator public-contract precedent
- `crates/worth-topo/src/topology_operators/*`
  - existing topology operator public lanes and split/rehome precedent
- `crates/worth-topo/src/topology_operators/query_workflow/*`
  - `TopologyOperatorWorkflowHandleExt`
  - `TopologyOperatorCanonicalDeclaration`
  - `TopologyOperatorProgressedDeclaration`
  - `TopologyOperatorRoutePlan`
  - `TopologyOperatorDeclarationReceipt`
  - `TopologyOperatorEnvelope`
  - `TopologyOperatorGroupedInput`
  - `TopologyOperatorGroupedDeclaration`
  - `TopologyOperatorGroupedContributionInput`
  - `TopologyOperatorContributionInput`
  - `TopologyOperatorContributionDeclaration`
- `crates/worth-topo/src/topology_operators/declaration_entry/grouped/split_connected_half_edge_set_to_new_wire.rs`
  - `TopologySplitConnectedHalfEdgeSetToNewWireDeclaration`
  - `TopologyWireSplitHalfEdgeMember`
  - `ForgeQueryDeclarationInput<TopologyQueryDomain>` precedent for split
    operators
- `crates/forge-query/docs/authoring/graph-composition-authoring.md`
  - `workspace.compose_graph(...)`
  - `workspace.compose_graph_with_invariant_pack(...)`
  - graph composition program, resolution map, lifecycle outcomes, evidence,
    assumption summary, and lineage summary
- `crates/forge-query/docs/AI_README.md`
  - ordinary domain work starts at Query
  - intent must be declared once, lowered once, and executed or inspected
    through canonical runtime-owned artifacts
  - graph/index views are proof boundaries, not late performance cleanup
- `crates/forge-query/docs/domain-capabilities/invariants/registering-domain-invariants-through-query.md`
  - Query-owned invariant registration and graph-composition domain-invariant
    denial posture

New `7.3` surfaces are allowed where existing surfaces cannot honestly express:

- a Query-shaped indexed segment-candidate discovery receipt for the `7.2`
  event ledger being consumed
- a proof-bearing split request from an event-ledger receipt
- source-edge split carriers recovered from event-ledger provenance
- point and interval split candidates
- per-edge split schedules and schedule-normalization receipts
- split vertex identities and coalescence decisions
- split edge fragments and overlap edge chains
- split-level naming propagation and subshape-signature mapping
- one canonical split edge-chain ledger receipt consumed by `7.4`
- split evidence rows and public-contract fences against synthetic split proof

## Query, Graph, And Invariant Integration Contract

`7.3` must distinguish three different things that are easy to collapse:

- spatial split products
  - prepared artifacts owned by `worth-spatial`
  - examples: split request, split candidates, schedules, split vertices, split
    fragments, overlap chains, split ledger
  - these do not mutate topology truth by themselves
- topology operator declarations
  - Query-declared topology mutation intent owned by `worth-topo`
  - examples: future consumed forms of `SplitEdge`, `SplitIntersectedEdges`,
    `InsertVertexOnEdgeForTJunction`, `SplitConnectedHalfEdgeSetToNewWire`, or
    grouped local rewrites
  - these must implement `ForgeQueryDeclarationInput<TopologyQueryDomain>` and
    travel through `TopologyOperatorWorkflowHandleExt` declaration, review,
    progression, route, receipt, envelope, grouped, contribution, and recovery
    surfaces where applicable
- validators and legality rules
  - domain invariant meaning owned by Worth/topology/spatial semantics
  - where they block graph-shaped topology authoring, they must be registered
    or invoked through Query invariant-pack / domain-invariant denial posture
    rather than remembered by split executors as manual checks

This means:

- a new `7.3` operator name in the spec is not automatically a public mutation
  method
- topology-affecting operators must either:
  - remain prepared split-ledger artifacts in `worth-spatial`, or
  - become explicit topology declaration families with canonical Query
    declaration entries, support/admission posture, route/receipt/envelope
    proof, and public-contract fences
- graph-shaped same-batch authoring must use Query graph composition or the
  existing topology grouped/contribution workflow, not caller-owned batch
  choreography
- segment-pair candidate discovery must have a Query-owned candidate-index
  product before split work consumes the event ledger; full cross-operand
  breadth is allowed only as a hostile baseline fixture, never as the
  production-confidence proof path
- "Query-owned candidate-index product" means the Query-owned path produces the
  candidate rows, candidate counters, cull counters, fallback posture, and
  lifecycle outcome. A host-local candidate list decorated with Query
  declaration/envelope digests is not sufficient.
- validator families must be attached to declared graph shape, split ledger
  consumption, or topology operator declaration families so the runtime can
  emit typed denials; "remember to run validator X" is not an admitted plan
- split evidence rows are not validator execution. Evidence proves the stage
  happened; registered/domain validators prove the stage is legal.

## Phase Plan

### Phase 0: Query Indexing Debt Audit And Candidate-Index Product Repair

Before `7.3` starts, repair the runtime surfaces that currently look
Query-admitted while still producing lookup, candidate, or evidence-selection
facts through host-local scans. This phase is numbered `0` intentionally: it is
a prerequisite cleanup gate, not split functionality.

**Relevant subsystems**
- Forge Query indexed read / graph-index authority
- `worth-spatial` planar boolean segment candidate discovery
- `worth-spatial` workload evidence ledger
- `worth-kernel` workload composition and operator harness
- `worth-spatial` workload operators that consume kernel evidence

**Phase 0 implementation split**
- `Phase 0A: Candidate-Index Query Product Authority`
  - Repair the planar boolean segment candidate-index path so the Query-owned
    product is the authority for candidate rows, row identities, counters,
    culls, fallback posture, and lifecycle outcome before segment-pair
    enumeration or predicate binding can consume anything.
- `Phase 0B: Workload Evidence Stage Index Product`
  - Replace runtime evidence-stage row scans with a typed indexed receipt view
    that owns stage lookup, receipt matching, duplicate-stage denial,
    counter-bearing boolean receipt lookup, and static/test-only scan
    classification.
- `Phase 0C: Operator Harness And Spatial Operator Binding`
  - Migrate operator execution and spatial workload operators from raw
    `Vec<WorkloadEvidenceRow>` and string-prefix stage linkage to the typed
    evidence-stage index product, then add anti-theatre fences that reject raw
    evidence row vectors, foreign stage links, and synthetic operator
    consumption.

**Existing API references**
- `crates/forge-query/docs/AI_README.md`
  - ordinary domain work starts at Query
  - declare intent once, lower it once, execute or inspect it through canonical
    runtime-owned artifacts
  - graph/index views are proof boundaries, not performance cleanup
- `crates/worth-spatial/src/workload_platform/planar_boolean_events/pair_enumeration/query_index.rs`
  - current `query_index_evidence(...)` only proves Query admission digests
  - current `CandidateIndexQueryEvidence` does not own candidate rows,
    counters, cull accounting, or fallback posture
- `crates/worth-spatial/src/workload_platform/planar_boolean_events/pair_enumeration/candidate_index.rs`
  - current `indexed_candidate_pairs(...)` and
    `append_left_segment_candidates(...)` produce candidate rows host-locally
    after Query evidence is obtained
- `crates/worth-spatial/src/workload_platform/planar_boolean_events/pair_enumeration/worklist.rs`
  - current pipeline consumes the host-built candidate index and only later
    records query-index identity fields on the receipt
- `crates/worth-spatial/src/workload_platform/evidence_ledger/ledger.rs`
  - current `evidence_for_stage(...)`, `row_for_stage(...)`,
    `boolean_row_for_receipt(...)`, duplicate-stage detection, and guard paths
    are row-scan based
- `crates/worth-kernel/src/workload_composition/worth_workload.rs`
  - `WorthWorkload::evidence_ledger`
  - `WorthWorkload::require_boolean_segment_pair_enumeration`
  - `require_evidence_stage(...)`
  - `require_boolean_evidence(...)`
- `crates/worth-kernel/src/workload_composition/operator_harness/run.rs`
  - current `OperatorRun` copies `CompleteWorkloadEvidenceLedger::rows()`
  - current `require_projection_counters(...)` scans copied evidence rows
- `crates/worth-kernel/src/workload_composition/operator_harness/receipt_set.rs`
  - current `links_to_stage(...)` infers stage linkage from string prefixes
- `crates/worth-spatial/src/workload_platform/workload_operators/coplanar_overlap.rs`
  - current `CoplanarOverlapWorkloadOperator` consumes raw evidence rows and
    scans for required stages
- `crates/worth-spatial/src/workload_platform/workload_operators/coplanar_overlap_receipt.rs`
  - current receipt linkage is string-identity based

**New operators added**
- `AuditQueryIndexingDebtSurface`
- `RegisterPlanarBooleanSegmentCandidateIndexQueryProduct`
- `DeclarePlanarBooleanSegmentCandidateIndexQuery`
- `PlanPlanarBooleanSegmentCandidateIndexQuery`
- `ExecutePlanarBooleanSegmentCandidateIndexQuery`
- `EmitPlanarBooleanSegmentCandidateIndexProduct`
- `EmitPlanarBooleanSegmentCandidateIndexRows`
- `EmitPlanarBooleanSegmentCandidateIndexCounters`
- `EmitPlanarBooleanSegmentCandidateIndexFallbackPosture`
- `BindSegmentPairEnumerationToCandidateIndexProduct`
- `RegisterWorkloadEvidenceStageIndexProduct`
- `BuildWorkloadEvidenceStageIndex`
- `BindOperatorHarnessToEvidenceStageIndex`
- `RejectHostLocalCandidateRowsWithQueryDigestDecoration`
- `RejectRawEvidenceRowOperatorConsumption`
- `RejectStringPrefixStageLinkage`

**Warnings**
- Do not call a candidate index "Query-owned" unless Query returns or owns the
  candidate rows, emitted/cull counters, fallback posture, and denial outcome.
- Do not let `query_index_evidence(...)` remain a digest wrapper around
  host-local candidate construction.
- Do not let the boolean pipeline consume `PlanarBooleanSegmentPairWorkItem`
  rows that were produced outside the Query-owned candidate-index product.
- Do not let workload operators consume raw `Vec<WorkloadEvidenceRow>` as their
  proof-bearing lookup surface.
- Do not infer stage linkage from formatted strings such as
  `starts_with(format!("{stage:?}:"))`; use typed stage-link rows or an indexed
  receipt view.
- Do not treat this audit as documentation only. Every listed debt surface must
  either be repaired, explicitly classified as bounded/static/non-runtime, or
  blocked with a follow-up milestone that prevents `7.3` certification.

**Test requirements**
- `query_candidate_index_product_owns_rows_counters_culls_and_fallback_posture`
- `segment_pair_enumeration_rejects_host_local_candidate_rows_with_query_digests`
- `boolean_event_pipeline_consumes_candidate_index_product_not_local_work_items`
- `candidate_index_product_denies_or_marks_nonproduction_full_breadth_fallback`
- `workload_evidence_stage_index_replaces_row_scan_lookup_for_runtime_receipts`
- `operator_harness_rejects_raw_evidence_row_vector_consumption`
- `operator_receipt_stage_links_are_typed_not_string_prefixes`
- `query_indexing_debt_audit_lists_every_runtime_scan_or_marks_it_static`

**Engineering decisions**
- Add a first-class candidate-index Query product, not only a Query admission
  receipt:
  - stable product identity
  - query declaration digest
  - query plan digest
  - query envelope digest
  - candidate index strategy
  - candidate row identities
  - emitted candidate rows
  - theoretical full-breadth pair count
  - emitted candidate-pair count
  - culled candidate-pair count
  - event-bearing candidate-pair count when known
  - fallback posture
  - lifecycle / denial outcome
- `PlanarBooleanSegmentPairEnumerationReceipt` must bind to this product and
  must not be constructible from candidate rows produced outside it.
- Add or expose a `WorkloadEvidenceStageIndex` / equivalent indexed receipt
  view so kernel and spatial operators can ask for typed stage rows without
  repeatedly scanning raw evidence rows.
- Keep raw row slices only as serialization/inspection surfaces, not as the
  runtime lookup authority for operator execution or stage requirements.
- Produce an audit table in the implementation closeout with at least these
  classifications:
  - repaired Query-index product
  - repaired typed evidence-stage index
  - bounded static table
  - test-only scan
  - blocked runtime debt

**Open questions**
- Whether the candidate-index Query product belongs entirely in
  `worth-spatial` or whether `worth-kernel` should own the workload-stage
  binding receipt. The product itself must still be Query-owned before the
  boolean pipeline consumes candidate rows.

### Phase 1: Query Graph, Topology Declaration, And Validator Registration Blueprint

Freeze how `7.3` split operators and validators enter the existing Query and
topology workflow surfaces before any split request, schedule, vertex, fragment,
or ledger type is designed against the wrong authority lane.

**Relevant subsystems**
- `worth-topo` topology operator Query workflow
- `worth-topo` grouped and contribution operator surfaces
- `worth-spatial` split ledger platform
- `worth-kernel` workload composition
- Forge Query graph composition and invariant registration

**Existing API references**
- `crates/worth-topo/src/topology_operators/query_workflow/workflow_handle_ext.rs`
  - `TopologyOperatorWorkflowHandleExt::declare_topology_operator`
  - `TopologyOperatorWorkflowHandleExt::review_topology_operator`
  - `TopologyOperatorWorkflowHandleExt::declare_review_and_progress_topology_operator`
  - `TopologyOperatorWorkflowHandleExt::orchestrate_topology_operator_route`
  - `TopologyOperatorWorkflowHandleExt::orchestrate_topology_operator_receipt`
  - `TopologyOperatorWorkflowHandleExt::orchestrate_topology_operator_envelope_from_progressed`
  - `TopologyOperatorWorkflowHandleExt::declare_topology_grouped_operator`
  - `TopologyOperatorWorkflowHandleExt::grouped_topology_operator_contributions_checked`
  - `TopologyOperatorWorkflowHandleExt::orchestrate_topology_operator_with_contributions`
- `crates/worth-topo/src/topology_operators/query_workflow/workflow_artifacts.rs`
  - `TopologyOperatorCanonicalDeclaration`
  - `TopologyOperatorProgressedDeclaration`
  - `TopologyOperatorRoutePlan`
  - `TopologyOperatorDeclarationReceipt`
  - `TopologyOperatorEnvelope`
  - `TopologyOperatorGroupedInput`
  - `TopologyOperatorContributionInput`
  - `TopologyOperatorContributionDeclaration`
- `crates/worth-topo/src/topology_operators/query_workflow/grouped_and_contribution_builders.rs`
  - `topology_grouped_operator_neighborhood`
  - `topology_operator_contribution_workflow`
- `crates/worth-topo/src/topology_operators/query_workflow/retained_contribution_semantics.rs`
  - `validate_topology_retained_contribution_composition`
  - `validated_topology_retained_contribution_semantic_projection`
- `crates/worth-topo/src/topology_operators/declaration_entry/grouped/split_connected_half_edge_set_to_new_wire.rs`
  - `TopologySplitConnectedHalfEdgeSetToNewWireDeclaration`
  - `TopologyWireSplitHalfEdgeMember`
  - `ForgeQueryDeclarationInput<TopologyQueryDomain>` implementation precedent
  - `ForgeQueryDeclarationFamilyMarker` precedent with grouped posture
- `crates/worth-topo/src/topology_operators/application/mod.rs`
  - `TopologyMutationApplicationRunner`
  - `TopologyQueryBindingIndex`
  - `finalize_graph_or_batch_receipt_closeout`
- `crates/forge-query/docs/authoring/graph-composition-authoring.md`
  - `workspace.compose_graph(...)`
  - `workspace.compose_graph_with_invariant_pack(...)`
  - graph composition program, resolution map, lifecycle outcomes, evidence,
    assumption summary, and lineage summary
- `crates/forge-query/docs/domain-capabilities/invariants/registering-domain-invariants-through-query.md`
  - `ForgeQueryRuntime::builder().invariant_catalog(...)`
  - `ForgeQueryRuntime::builder().custom_invariant(...)`
  - `ForgeQueryRuntime::builder().register_invariant(...)`
  - `ForgeQueryRuntime::builder().invariant_registration_artifact(...)`

**New operators added**
- `RegisterEdgeSplitOperatorDeclarationFamily`
- `RegisterEdgeSplitGroupedOperatorFamily`
- `RegisterEdgeSplitContributionWorkflow`
- `RegisterEdgeSplitGraphInvariantPack`
- `MapSplitLedgerToTopologyOperatorDeclarations`
- `ClassifyPreparedVsAuthoritativeSplitOperator`
- `ValidateSplitOperatorQueryProgression`
- `ValidateSplitValidatorRuntimeRegistration`

**Warnings**
- Do not implement topology-affecting `7.3` operators as free functions in
  `worth-spatial` or `worth-kernel`.
- Do not add split validators as a list the executor manually calls after
  building fragments; validators that govern graph legality must attach to the
  Query invariant/domain-denial path or the topology operator declaration
  review path.
- Do not use `workspace.batch(...)` or caller-owned ordered write lists for
  same-batch edge split topology programs that need symbolic handles,
  retargeting, supersession, lineage, or verified existing-truth evidence.
- Do not use topology grouped/contribution surfaces as a shortcut that bypasses
  canonical declaration entries, support posture, or retained semantic
  contribution validation.

**Test requirements**
- `edge_split_operator_declarations_expose_query_canonical_entries_and_family_markers`
- `edge_split_grouped_operator_workflow_preserves_grouped_support_and_contribution_evidence`
- `edge_split_validators_register_through_invariant_or_declaration_review_lanes`
- `edge_split_graph_composition_rejects_domain_invalid_topology_with_typed_invariant_denial`
- `prepared_spatial_split_artifacts_cannot_be_called_as_authoritative_topology_mutations`

**Engineering decisions**
- Split artifacts produced after this phase remain `worth-spatial` prepared
  products until a later phase explicitly maps them to topology declarations.
- Topology-affecting operators listed in this spec must be classified as one
  of:
  - `PreparedSpatialOnly`
  - `TopologyDeclarationFamily`
  - `TopologyGroupedDeclarationFamily`
  - `TopologyContributionWorkflow`
  - `QueryGraphCompositionProgram`
  - `SupportGatedFutureTopologyMutation`
- Any operator classified as `TopologyDeclarationFamily` or
  `TopologyGroupedDeclarationFamily` must have canonical declaration entries,
  family marker, required capability/config sections, legality contract,
  progression contract, route contract, public facade contract, and compile-fail
  anti-forgery proof.
- Any operator classified as `QueryGraphCompositionProgram` must expose graph
  composition receipt evidence: program, resolution map, lifecycle outcomes,
  evidence, assumption summary when verified targets participate, and lineage
  summary when retarget/supersession participates.
- Validators that decide legality of split topology must be bound to declared
  graph shape, topology operator declaration review, or Query invariant-pack
  denials. They may have thin phase-local helpers, but the closeout proof must
  show the runtime-facing denial path.

**Open questions**
- Which `7.3` operators should remain prepared-only until `7.4` versus become
  topology declaration families immediately. The spec requires this
  classification to be explicit before downstream split artifacts are treated
  as implementable.

### Phase 2: Candidate-Index Product Consumption Gate

Freeze the proof that the `7.2` event ledger being consumed was produced from a
first-class Query-owned candidate-index product, not from caller-owned N+1
detail reads, host-local AABB sweeps, or a full left-by-right cross-product loop
disguised as admission.

**Relevant subsystems**
- Forge Query expression, planning, inspection, and graph/index surfaces
- `worth-spatial` planar boolean segment-pair discovery
- `worth-kernel` workload evidence and stage requirements
- `worth-spatial` event-ledger receipt platform

**Existing API references**
- `crates/forge-query/docs/AI_README.md`
  - ordinary domain work starts at Query
  - declare intent once, lower it once, execute or inspect it through canonical
    runtime-owned artifacts
  - graph/index views are proof boundaries rather than later optimization
- `crates/worth-spatial/src/facade/planar_boolean_events.rs`
  - `PlanarBooleanSegmentPairEnumerationReceipt`
  - `PlanarBooleanEventLedgerReceipt`
  - `PlanarBooleanSegmentCarrierSet`
  - `PlanarBooleanSegmentCarrier`
- new Phase 0 candidate-index Query product and product receipt
- `crates/worth-spatial/src/workload_platform/planar_boolean_events/pair_enumeration/worklist.rs`
  - must consume the Phase 0 candidate-index Query product instead of producing
    local candidate rows after Query admission
- `crates/worth-kernel/src/workload_composition/worth_workload.rs`
  - `WorthWorkload::require_boolean_event_ledger`
- `crates/worth-spatial/src/workload_platform/evidence_ledger/stage.rs`
  - `BooleanEvidenceStageKind::EventLedger`
  - `WorkloadEvidenceStage::BooleanEventLedger`

**New operators added**
- `ConsumePlanarBooleanSegmentCandidateIndexProduct`
- `BindEventLedgerToCandidateIndexProduct`
- `BindSegmentPairEnumerationToCandidateIndexProduct`
- `ValidateIndexedCandidateDiscoveryReceipt`
- `RejectFullBreadthCandidateDiscoveryAsProductionProof`
- `RejectQueryDigestDecoratedLocalCandidateIndex`
- `CountCulledSegmentCandidatePairs`
- `ValidateCandidateIndexProductLifecycleOutcome`

**Warnings**
- Do not let `7.3` consume an event ledger whose only discovery proof is
  "every left segment was paired with every right segment."
- Do not treat a spatial acceleration index as a private optimization or local
  helper hidden under the event extractor. The Query-owned product identity,
  query plan identity, candidate rows, emitted-pair count, culled-pair count,
  fallback posture, and lifecycle outcome are part of the proof boundary.
- Do not replace full cross-product loops with N detail queries per segment.
  Query must own the indexed candidate view and the lowered plan.
- Do not accept Query declaration/envelope digests that merely decorate a local
  candidate list.
- Do not allow the metaboss fixture size to excuse the production path. Small
  fixtures must still assert indexed planner behavior.

**Test requirements**
- `planar_boolean_candidate_index_product_records_plan_rows_counters_culls_and_fallback`
- `edge_split_rejects_event_ledger_without_candidate_index_product`
- `candidate_index_product_does_not_execute_left_by_right_full_breadth_for_production_closeout`
- `candidate_index_product_rejects_query_digest_decorated_local_candidate_rows`
- `candidate_index_product_is_stable_across_replay_and_operand_order_variation`
- `full_breadth_pair_enumeration_is_allowed_only_as_named_hostile_baseline`

**Engineering decisions**
- `7.3` may consume `7.2` event ledger facts, but only through an admission path
  that proves the ledger is bound to the Phase 0 candidate-index Query product.
- The candidate index may be introduced by a `7.2` repair or by Phase 0, but the
  `7.3` closeout cannot certify against cross-product-only discovery or
  host-local candidate rows decorated with Query digests.
- The product receipt must distinguish:
  - source segment count
  - theoretical full-breadth pair count
  - indexed candidate pair count
  - culled pair count
  - emitted event-bearing pair count
  - fallback or unsupported-index posture
- Any fallback to full breadth must produce a typed non-production proof
  outcome that cannot satisfy the summum bonum certification target.

**Open questions**
- Whether the Phase 0 repair lands as a late `7.2` correction or as the first
  implementation slice of `7.3`. The `7.3` spec treats it as a non-negotiable
  prerequisite either way.

### Phase 3: Split Request Boundary From Event Ledger

Freeze the only artifact that may enter split execution: a split request built
from the `7.2` event-ledger receipt, the Phase 0 Query-owned candidate-index
product, and the workload evidence that proves both are real.

**Relevant subsystems**
- `worth-kernel` workload composition
- `worth-spatial` planar boolean event platform
- Query retained artifact progression

**Existing API references**
- `crates/worth-spatial/src/facade/planar_boolean_events.rs`
  - `PlanarBooleanSegmentPairEnumerationReceipt`
  - `PlanarBooleanEventLedgerReceipt`
  - `PlanarBooleanEventLedgerReceipt::event_ledger_identity`
  - `PlanarBooleanEventLedgerReceipt::downstream_consumption_identity`
  - `PlanarBooleanEventLedgerReceipt::reduced_pair_identity`
- `crates/worth-kernel/src/workload_composition/worth_workload.rs`
  - `WorthWorkload::require_boolean_event_ledger`
- `crates/worth-kernel/src/workload_composition/boolean_event_extraction/request.rs`
  - `PlanarBooleanEventExtractionRequest`
- `crates/worth-spatial/src/workload_platform/evidence_ledger/stage.rs`
  - `BooleanEvidenceStageKind::EventLedger`
  - `WorkloadEvidenceStage::BooleanEventLedger`
  - existing visible `BooleanEvidenceStageKind::Split`
  - existing visible `WorkloadEvidenceStage::BooleanSplit`

**New operators added**
- `ConsumePlanarBooleanEventLedger`
- `DeclarePlanarBooleanEdgeSplit`
- `AdmitPlanarBooleanEdgeSplit`
- `BindEdgeSplitToEventLedgerReceipt`
- `BindEdgeSplitToCandidateIndexProduct`
- `RejectSyntheticEdgeSplitEntry`

**Warnings**
- Do not accept `PlanarBooleanEventLedger` data copied into a new struct
  without the receipt identity.
- Do not let split code take raw point events, raw interval events, or raw
  segment carriers as its public entry.
- Do not add a kernel-local split route that bypasses Query/workload evidence.
- Do not accept an event ledger that lacks the Phase 0 Query-owned
  candidate-index product required by Phase 2.

**Test requirements**
- `edge_split_request_preserves_event_ledger_and_reduced_pair_identities`
- `edge_split_request_preserves_candidate_index_product_identity`
- `edge_split_request_rejects_raw_point_interval_or_segment_substitution`
- `edge_split_request_requires_boolean_event_ledger_evidence_row`

**Engineering decisions**
- Introduce `PlanarBooleanEdgeSplitRequest` or equivalent as a proof-bearing
  request artifact.
- The request identity must include or bind the Query-owned candidate-index
  product identity so replay cannot silently swap discovery strategies.
- The request must implement or feed a `BooleanEvidenceReceipt` path for
  `BooleanEvidenceStageKind::Split`, but only after phase-local admission has
  succeeded.

**Open questions**
- Final public facade name: `planar_boolean_edge_splitting` versus
  `planar_boolean_split_edges`.

### Phase 4: Split Scope And Policy Admission

Freeze the admitted `7.3` edge-surgery class before source-edge recovery or
parameter work begins.

**Relevant subsystems**
- `worth-spatial` split policy
- `worth-kernel` boolean outcome taxonomy
- Query support posture and ordinary outcomes

**Existing API references**
- `crates/worth-spatial/src/workload_platform/planar_boolean_events/policy.rs`
  - `PlanarBooleanEventExtractionPolicyExit`
  - `PlanarBooleanEventExtractionPolicyExitKind`
- `crates/worth-spatial/src/workload_platform/planar_boolean_events/denial.rs`
  - `PlanarBooleanEventExtractionDenial`
  - `PlanarBooleanEventExtractionDenialKind`
- `crates/worth-kernel/src/workload_composition/boolean_outcome/*`
  - `PlanarBooleanOutcomeKind`
  - `PlanarBooleanOutcomeReceipt`
- `crates/worth-spatial/src/facade/workload_vocabulary/mod.rs`
  - `WorkloadStageSupport`
  - `WorkloadEvidenceSupport`

**New operators added**
- `AdmitEdgeSplitScope`
- `SetEdgeSplitDegeneracyPolicy`
- `SetEdgeSplitDeterminismPolicy`
- `SetEdgeSplitOverlapPolicy`
- `EmitEdgeSplitPolicyOutcome`

**Warnings**
- Do not defer basic scope denials to later split phases if the event ledger
  already proves the case is outside the admitted edge-surgery class.
- Do not flatten unsupported, denied, blocked, policy-required, and integrity-
  mismatch split postures into one error.
- Do not widen into loop reconstruction or overlap-region extraction here.

**Test requirements**
- `edge_split_scope_admits_only_event_families_closed_by_7_2`
- `edge_split_scope_denies_unsupported_event_family_before_schedule_building`
- `edge_split_policy_outcomes_preserve_machine_kind_and_event_ledger_identity`

**Engineering decisions**
- Keep the initial admitted class broad for planar B-rep line-segment carriers,
  but fail closed on curved, non-linear, or missing-provenance events.
- Treat degeneracy policy as a split-stage artifact, not a global implicit
  tolerance.

**Open questions**
- Which same-operand duplicate/stacked segment cases are admitted as metadata
  versus denied until cleanup milestones?

### Phase 5: Source Edge Carrier Recovery

Recover one split source-edge carrier family from the event-ledger provenance.
This is the bridge from event truth to edge-level rewrite intent.

**Relevant subsystems**
- `worth-spatial` segment-carrier and event-ledger platform
- `worth-topo` topology identity provenance
- `worth-kernel` workload catalog hostile recipes

**Existing API references**
- `crates/worth-spatial/src/facade/planar_boolean_events.rs`
  - `PlanarBooleanEventLedgerReceipt::point_events`
  - `PlanarBooleanEventLedgerReceipt::interval_events`
  - `PlanarBooleanEventLedgerReceipt::event_groups`
  - `PlanarBooleanSegmentCarrier`
  - `PlanarBooleanSegmentCarrier::source_edge_identity`
  - `PlanarBooleanSegmentCarrier::source_loop_identity`
  - `PlanarBooleanSegmentCarrier::source_face_identity`
  - `PlanarBooleanSegmentCarrier::carrier_identity`
  - `PlanarBooleanLoopRole`
- `crates/worth-spatial/src/workload_platform/planar_boolean_events/segment_carriers/*`
  - `PlanarBooleanSegmentCarrierSet`
  - `PlanarBooleanSegmentCarrierSetDenial`
  - `PlanarBooleanSegmentCarrierSetDenialKind`

**New operators added**
- `RecoverBooleanSplitSourceEdgeCarriers`
- `ValidateSplitSourceEdgeCarrierCoverage`
- `RejectCoordinateOnlySplitCarriers`
- `BindSplitCarrierToTopologySourceEdge`

**Warnings**
- Do not derive source-edge identity from coordinate equality or event ordering.
- Do not let missing face, loop, edge, projection, local-frame, or precision
  provenance become a warning.
- Do not treat the carrier set used during `7.2` as implicitly available unless
  the event ledger receipt carries enough identity to recover or bind it.

**Test requirements**
- `source_edge_carrier_recovery_preserves_face_loop_edge_and_carrier_identity`
- `source_edge_carrier_recovery_rejects_coordinate_only_event_rows`
- `source_edge_carrier_recovery_is_stable_under_event_order_variation`

**Engineering decisions**
- Add a `PlanarBooleanSplitSourceEdgeCarrierSet` or equivalent rather than
  passing `PlanarBooleanSegmentCarrierSet` through unchanged.
- Each recovered carrier must be keyed by source edge plus operand side and
  must retain carrier identity for event participation lookup.

**Open questions**
- Whether the recovered split carrier should store full carrier data or only
  receipt-backed carrier identity plus read-only accessors into the event
  ledger.

### Phase 6: Split Event Participation Index

Freeze a deterministic index from source edge carriers to participating point
events, interval events, and event groups.

**Relevant subsystems**
- `worth-spatial` event-ledger and grouping platform
- `worth-spatial` split planning

**Existing API references**
- `crates/worth-spatial/src/facade/planar_boolean_events.rs`
  - `PlanarBooleanPointEvent::participating_carrier_identities`
  - `PlanarBooleanPointEvent::segment_pair_identities`
  - `PlanarBooleanIntervalEvent::left_carrier_identity`
  - `PlanarBooleanIntervalEvent::right_carrier_identity`
  - `PlanarBooleanEventGroup::participating_carrier_identities`
  - `PlanarBooleanEventGroup::point_event_identities`
  - `PlanarBooleanEventGroup::interval_event_identities`
  - `PlanarBooleanOrderedEventSet`
- `crates/worth-spatial/src/workload_platform/planar_boolean_events/event_grouping/*`
  - existing point and interval grouping identity logic

**New operators added**
- `BuildSplitEventParticipationIndex`
- `CanonicalizeSplitEventParticipationOrder`
- `RejectUnindexedSplitEvent`
- `ValidateSplitEventGroupCoverage`

**Warnings**
- Do not scan event vectors repeatedly in later phases when a carrier-to-event
  index can be derived once.
- Do not let event-group ordering decide split ordering unless the group
  identity basis says so.
- Do not drop no-op or diagnostic event families before policy has consumed
  them.

**Test requirements**
- `split_event_participation_index_covers_every_event_carrier_reference`
- `split_event_participation_index_orders_events_canonically`
- `split_event_participation_index_rejects_event_with_unknown_carrier`

**Engineering decisions**
- The participation index is a derived split-planning artifact, not source
  authority; it must be rebuildable from the event ledger and carrier set.
- The index must expose counters for carriers indexed, point references,
  interval references, group references, and rejected orphan references.

**Open questions**
- Whether relation diagnostics retained in the ledger participate in the index
  or stay diagnostic-only for `7.3`.

### Phase 7: Point Split Candidate Extraction

Lower point events into edge-local split candidates without yet deciding
whether each point is an interior split, endpoint no-op, T-junction promotion,
or duplicate.

**Relevant subsystems**
- `worth-spatial` point-event platform
- `worth-spatial` split candidate planning

**Existing API references**
- `crates/worth-spatial/src/facade/planar_boolean_events.rs`
  - `PlanarBooleanPointEvent`
  - `PlanarBooleanPointEvent::kind`
  - `PlanarBooleanPointEvent::coordinate_fact`
  - `PlanarBooleanPointEvent::operand_a_parameter`
  - `PlanarBooleanPointEvent::operand_b_parameter`
  - `PlanarBooleanPointEventSegmentParameterFact`
  - `PlanarBooleanPointEventSegmentParameterFact::segment_identity`
  - `PlanarBooleanPointEventSegmentParameterFact::carrier_identity`
  - `PlanarBooleanPointEventSegmentParameterFact::parameter`
  - `PlanarBooleanPointEventKind`

**New operators added**
- `ExtractPointSplitCandidates`
- `BindPointEventToSourceEdgeParameter`
- `AdmitEdgeSplitPoint`
- `RejectPointSplitWithoutCarrierParameter`

**Warnings**
- Do not use 2D coordinate sorting as the primary split basis; edge-local
  parameter facts are the split basis.
- Do not collapse endpoint contacts into no-ops before endpoint posture is
  decided.
- Do not infer missing parameters from coordinates in this phase.

**Test requirements**
- `point_split_candidates_preserve_event_kind_coordinate_and_parameter_facts`
- `point_split_candidate_extraction_rejects_missing_carrier_parameter`
- `point_split_candidates_are_stable_under_point_event_order_variation`

**Engineering decisions**
- Introduce `PlanarBooleanPointSplitCandidate` with event identity, carrier
  identity, source edge identity, coordinate fact, parameter fact, and point
  event kind.
- Candidate extraction is a pure lowering step; classification into split,
  no-op, T-junction, or denial happens later.

**Open questions**
- Whether shared-endpoint point events should lower to two endpoint candidates
  immediately or one grouped shared-endpoint candidate.

### Phase 8: Point Parameter Domain Admission

Validate that each point split candidate belongs to the admitted parameter
domain of its source edge.

**Relevant subsystems**
- `worth-spatial` split parameter validation
- `worth-spatial` endpoint-normalization precedent
- `worth-topo` source-edge identity semantics

**Existing API references**
- `crates/worth-spatial/src/facade/planar_boolean_events.rs`
  - `PlanarBooleanPointEventSegmentParameterFact::parameter`
  - `PlanarBooleanSegmentCarrier::start`
  - `PlanarBooleanSegmentCarrier::end`
  - `PlanarBooleanSegmentCarrierEndpointFacts`
  - `PlanarBooleanSegmentCarrierEndpointFacts::source_endpoint_identity`
  - `PlanarBooleanSegmentCarrierEndpointFacts::projected_endpoint_fact_identity`
- `crates/worth-spatial/src/workload_platform/planar_boolean_events/endpoint_normalization/*`
  - `PlanarBooleanNormalizedEndpoint`
  - `PlanarBooleanNormalizedEndpointPair`

**New operators added**
- `ParameterizeEdgeSplitPoint`
- `ValidateSplitPointParameterDomain`
- `ClassifySplitPointEndpointPosture`
- `RejectOutOfDomainSplitPoint`

**Warnings**
- Do not accept parameter values outside `[0, 1]` or outside the carrier's
  normalized domain unless a later explicit policy admits it.
- Do not treat exact endpoint contact and near-endpoint ambiguity as the same
  machine class.
- Do not silently clamp point parameters.

**Test requirements**
- `split_point_parameter_domain_accepts_interior_and_exact_endpoint_points`
- `split_point_parameter_domain_rejects_out_of_range_or_nan_parameters`
- `split_point_parameter_domain_preserves_endpoint_identity_when_exact`

**Engineering decisions**
- Point admission produces a proof-bearing `AdmittedPointSplitCandidate`.
- Endpoint classification is recorded but not yet normalized away.

**Open questions**
- Whether parameter facts should become fixed rational/ordered wrappers in this
  milestone or remain validated numeric facts tied to the `7.1` precision basis.

### Phase 9: Interval Split Candidate Extraction

Lower interval events into edge-local interval split candidates, preserving both
canonical interval facts and source-edge sense.

**Relevant subsystems**
- `worth-spatial` interval-event platform
- `worth-spatial` split candidate planning

**Existing API references**
- `crates/worth-spatial/src/facade/planar_boolean_events.rs`
  - `PlanarBooleanIntervalEvent`
  - `PlanarBooleanIntervalEvent::kind`
  - `PlanarBooleanIntervalEvent::normalized_interval`
  - `PlanarBooleanIntervalEvent::left_source_interval`
  - `PlanarBooleanIntervalEvent::right_source_interval`
  - `PlanarBooleanIntervalEvent::left_carrier_identity`
  - `PlanarBooleanIntervalEvent::right_carrier_identity`
  - `PlanarBooleanIntervalEventKind`
  - `PlanarBooleanNormalizedInterval`
  - `PlanarBooleanSourceInterval`
  - `PlanarBooleanSourceInterval::source_parameter_range`
  - `PlanarBooleanSourceInterval::sense`
  - `PlanarBooleanSourceIntervalSense`

**New operators added**
- `ExtractIntervalSplitCandidates`
- `AdmitEdgeSplitInterval`
- `BindIntervalEventToSourceEdgeRange`
- `ResolveEdgeEdgePartialOverlap`
- `RejectIntervalWithoutSourceRange`

**Warnings**
- Do not flatten partial overlap, containment overlap, identical same-direction
  coincidence, and identical anti-parallel coincidence into one interval case.
- Do not normalize away source sense; later overlap-chain work needs it.
- Do not treat interval extraction as overlap-region extraction. That belongs
  to `7.5`.

**Test requirements**
- `interval_split_candidates_preserve_kind_source_range_and_source_sense`
- `interval_split_candidate_extraction_rejects_missing_source_interval`
- `anti_parallel_interval_candidate_preserves_opposite_source_sense`

**Engineering decisions**
- Introduce `PlanarBooleanIntervalSplitCandidate` with event identity, carrier
  identity, source edge identity, source parameter range, normalized interval
  identity, interval kind, and source sense.
- Candidate extraction emits one candidate per participating source edge, not a
  single pair-level interval blob.

**Open questions**
- Whether interval candidates should carry overlap-chain hints now or leave
  all region/topology grouping to Phase 19 and `7.5`.

### Phase 10: Interval Parameter Domain Admission

Validate interval candidate parameter ranges before they can subdivide source
edges.

**Relevant subsystems**
- `worth-spatial` interval normalization
- `worth-spatial` split admission

**Existing API references**
- `crates/worth-spatial/src/facade/planar_boolean_events.rs`
  - `PlanarBooleanSourceInterval::source_parameter_range`
  - `PlanarBooleanSourceInterval::source_interval_identity`
  - `PlanarBooleanNormalizedInterval::parameter_range`
  - `PlanarBooleanNormalizedInterval::precision_basis_identity`
  - `PlanarBooleanNormalizedInterval::local_frame_identity`
- `crates/worth-spatial/src/workload_platform/planar_boolean_events/interval_normalization/*`
  - existing `parameter_range` and collapsed-interval precedent

**New operators added**
- `ParameterizeEdgeSplitInterval`
- `ValidateSplitIntervalParameterDomain`
- `RejectCollapsedSplitInterval`
- `RejectContradictorySplitIntervalSense`

**Warnings**
- Do not allow collapsed intervals to become zero-length overlap chains.
- Do not sort interval endpoints without preserving whether the source edge was
  traversed in normal or reversed sense.
- Do not infer tolerance policy independently of the event ledger's precision
  basis.

**Test requirements**
- `split_interval_parameter_domain_accepts_ordered_non_collapsed_ranges`
- `split_interval_parameter_domain_rejects_collapsed_or_nan_ranges`
- `split_interval_parameter_domain_preserves_source_sense_after_ordering`

**Engineering decisions**
- Interval admission produces `AdmittedIntervalSplitCandidate`.
- Collapsed intervals must become a typed denial or a policy exit, not a point
  split unless a later explicit conversion operator admits that behavior.

**Open questions**
- Whether interval endpoint equality with an existing point event should be
  normalized in this phase or in split schedule assembly.

### Phase 11: T-Junction And Endpoint Touch Promotion

Freeze the split posture for endpoint-on-interior contacts, shared endpoints,
and endpoint-only no-op contacts before schedules are assembled.

**Relevant subsystems**
- `worth-spatial` point-event and shared-endpoint event platform
- `worth-spatial` split policy
- topology operator vocabulary for T-junction promotion

**Existing API references**
- `crates/worth-spatial/src/facade/planar_boolean_events.rs`
  - `PlanarBooleanPointEvent::kind`
  - `PlanarBooleanPointEvent::shared_endpoint_event`
  - `PlanarBooleanPointEvent::endpoint_source_identities`
  - `PlanarBooleanPointEvent::source_endpoint_identities`
  - `PlanarBooleanSharedEndpointEvent`
  - `PlanarBooleanPointEventKind`
- `_docs/worth_topo/operators-list.md`
  - `DetectTJunctions`
  - `PromoteTJunctionToVertexSplit`
  - `InsertVertexOnEdgeForTJunction`

**New operators added**
- `DetectTJunctions`
- `PromoteTJunctionToVertexSplit`
- `InsertVertexOnEdgeForTJunction`
- `ClassifyEndpointTouchSplitPosture`
- `CollapseEndpointNoOpSplits`

**Warnings**
- Do not treat endpoint-on-interior contact as a harmless point event; it is
  exactly the T-junction shape that must become explicit before loop rebuilding.
- Do not create split vertices for endpoint-only contacts that produce no
  topology change unless the policy explicitly requires an identity event.
- Do not drop shared-endpoint provenance when collapsing endpoint no-ops.

**Test requirements**
- `t_junction_endpoint_on_interior_promotes_to_vertex_split`
- `shared_endpoint_contact_preserves_endpoint_identities_without_extra_fragment`
- `endpoint_only_noop_split_is_counted_and_does_not_create_zero_length_edge`

**Engineering decisions**
- Add a `PlanarBooleanPointSplitPosture` enum or equivalent with variants such
  as `InteriorSplit`, `EndpointNoOp`, `TJunctionPromotion`, `SharedEndpoint`,
  and `Denied`.
- T-junction promotion remains prepared split intent, not final loop or face
  topology.

**Open questions**
- Whether endpoint-touch posture should preserve both original endpoint
  identities when two carriers share one topological endpoint.

### Phase 12: Per-Edge Split Schedule Assembly

Assemble admitted point and interval candidates into one schedule per source
edge before normalization.

**Relevant subsystems**
- `worth-spatial` split schedule platform
- `worth-spatial` event participation index

**Existing API references**
- `crates/worth-spatial/src/facade/planar_boolean_events.rs`
  - `PlanarBooleanEventLedgerReceipt::ordered_events`
  - `PlanarBooleanOrderedEventSet::point_event_identities`
  - `PlanarBooleanOrderedEventSet::interval_event_identities`
  - `PlanarBooleanPointEventSegmentParameterFact::parameter`
  - `PlanarBooleanSourceInterval::source_parameter_range`
- `crates/worth-spatial/src/workload_platform/planar_boolean_events/event_ledger/ordered_events.rs`
  - canonical event ordering precedent

**New operators added**
- `AssemblePerEdgeSplitSchedule`
- `InsertPointCandidateIntoSplitSchedule`
- `InsertIntervalCandidateIntoSplitSchedule`
- `ValidateSplitScheduleSourceEdgeCoverage`
- `RejectMixedSourceEdgeSchedule`

**Warnings**
- Do not sort by insertion order, event vector order, or hashmap iteration.
- Do not assemble one global cut list detached from source-edge identity.
- Do not collapse duplicates yet; this phase should preserve raw candidate
  participation so normalization can prove what it removed.

**Test requirements**
- `per_edge_split_schedule_groups_candidates_by_source_edge`
- `per_edge_split_schedule_rejects_mixed_source_edge_candidates`
- `per_edge_split_schedule_preserves_raw_candidate_participation_counts`

**Engineering decisions**
- Introduce `PlanarBooleanRawEdgeSplitSchedule` keyed by recovered split source
  edge identity.
- The raw schedule must expose counts for point candidates, interval candidates,
  T-junction candidates, endpoint no-ops, and source event groups.

**Open questions**
- Whether schedules should be assembled operand-local first and then merged,
  or keyed solely by source edge identity with operand side retained.

### Phase 13: Canonical Schedule Ordering And Tie-Break Coverage

Freeze deterministic ordering for every per-edge schedule before duplicate or
micro-interval normalization can run.

**Relevant subsystems**
- `worth-spatial` split schedule platform
- `worth-spatial` determinism counters

**Existing API references**
- `crates/worth-spatial/src/facade/planar_boolean_events.rs`
  - `PlanarBooleanPointEvent::event_identity`
  - `PlanarBooleanIntervalEvent::event_identity`
  - `PlanarBooleanEventGroup::group_identity`
  - `PlanarBooleanOrderedEventSet`
- `_docs/worth_topo/validators.md`
  - `ValidateCanonicalOrderingStable`
  - `ValidateTieBreakerCoverage`
  - `ValidateHashStabilityAcrossRuns`

**New operators added**
- `CanonicalizeEdgeSplitPointOrder`
- `CanonicalizeEdgeSplitIntervalOrder`
- `CanonicalizeBooleanTraversalOrder`
- `ValidateTieBreakerCoverageForSplitSchedule`
- `RejectUnorderedSplitSchedule`

**Warnings**
- Do not allow equal parameter values to rely on event iteration order.
- Do not use stringified floating-point coordinates as a final tie-breaker.
- Do not hide tie-break rules inside sort closures; they must be named and
  testable.

**Test requirements**
- `split_schedule_order_is_stable_under_candidate_order_variation`
- `split_schedule_tie_breakers_cover_equal_parameter_point_and_interval_edges`
- `split_schedule_order_does_not_depend_on_debug_or_display_strings`

**Engineering decisions**
- Ordering basis should include source edge identity, normalized parameter,
  candidate kind, event identity, event group identity where applicable, and
  carrier identity.
- The ordered schedule must expose an order digest for replay proof.

**Open questions**
- Whether the final ordering digest should live on each schedule or only on the
  later split edge-chain ledger.

### Phase 14: Duplicate Cut And Redundant Report Normalization

Collapse duplicate point cuts and redundant reports while preserving every
source event that contributed to the normalized cut.

**Relevant subsystems**
- `worth-spatial` point deduplication precedent
- `worth-spatial` split schedule normalization

**Existing API references**
- `crates/worth-spatial/src/workload_platform/planar_boolean_events/point_deduplication/*`
  - `PlanarBooleanDeduplicatedPointEventSet`
  - existing point-event deduplication key precedent
- `crates/worth-spatial/src/facade/planar_boolean_events.rs`
  - `PlanarBooleanPointEvent::segment_pair_identities`
  - `PlanarBooleanPointEvent::participating_carrier_identities`
  - `PlanarBooleanPointEvent::predicate_receipt_identities`
  - `PlanarBooleanEventGroup`

**New operators added**
- `CollapseDuplicateSplitPoints`
- `MergeRedundantSplitEventReports`
- `RetainSplitDuplicateProvenance`
- `ValidateNoDuplicateSplitParameters`
- `RejectContradictoryDuplicateSplitPoint`

**Warnings**
- Do not drop duplicate-event provenance just because the final split point is
  one cut.
- Do not merge equal-parameter candidates with contradictory source edge,
  carrier, precision, or event-kind posture.
- Do not count duplicate collapse as cleanup; this is split schedule
  normalization.

**Test requirements**
- `duplicate_split_points_collapse_to_one_cut_with_all_event_provenance`
- `contradictory_duplicate_split_points_deny_instead_of_merging`
- `duplicate_split_point_collapse_counters_are_exact`

**Engineering decisions**
- Add `PlanarBooleanNormalizedSplitCut` with a provenance list rather than a
  single source event.
- Duplicate collapse must emit before/after counters and a duplicate report
  identity.

**Open questions**
- Whether endpoint no-op duplicates and interior split duplicates use the same
  normalized cut type or distinct posture-specific types.

### Phase 15: Endpoint No-Op And Boundary Split Normalization

Normalize endpoint-only cuts and boundary contacts so they do not create
zero-length fragments while still preserving identity and decision evidence.

**Relevant subsystems**
- `worth-spatial` split schedule normalization
- `worth-spatial` shared-endpoint event platform

**Existing API references**
- `crates/worth-spatial/src/facade/planar_boolean_events.rs`
  - `PlanarBooleanPointEvent::source_endpoint_identities`
  - `PlanarBooleanPointEvent::endpoint_projection_fact_digests`
  - `PlanarBooleanSharedEndpointEvent`
  - `PlanarBooleanSegmentCarrierEndpointFacts::source_endpoint_identity`
- `_docs/worth_topo/validators.md`
  - `ValidateNoUnexpectedZeroLengthEdges`
  - `ValidateShortEdgePolicyApplied`

**New operators added**
- `CollapseEndpointNoOpSplits`
- `RecordEndpointContactDecision`
- `ValidateEndpointNoOpSplitPolicy`
- `RejectEndpointSplitThatWouldCreateZeroLengthFragment`

**Warnings**
- Endpoint no-ops must remain visible in counters and decision logs.
- Do not invent a new vertex for a source endpoint unless topology semantics
  require a promoted split vertex.
- Do not collapse an endpoint contact that is also a T-junction promotion.

**Test requirements**
- `endpoint_noop_split_preserves_contact_decision_without_fragment`
- `endpoint_boundary_split_rejects_zero_length_fragment_creation`
- `endpoint_normalization_distinguishes_noop_from_t_junction_promotion`

**Engineering decisions**
- Endpoint no-op normalization produces decision records consumed by the final
  split ledger but not fragment construction.
- Boundary split posture must preserve original source endpoint identity for
  later loop reconstruction.

**Open questions**
- Whether endpoint-only contact decisions belong in the split fragment ledger
  or a separate split diagnostic section.

### Phase 16: Micro-Interval And Redundant Interval Normalization

Normalize interval-driven cuts, redundant collinear subdivisions, and micro-
intervals before split vertices or fragments are minted.

**Relevant subsystems**
- `worth-spatial` interval normalization
- `worth-spatial` split schedule normalization
- boolean overlap surgery operator vocabulary

**Existing API references**
- `crates/worth-spatial/src/facade/planar_boolean_events.rs`
  - `PlanarBooleanIntervalEvent::kind`
  - `PlanarBooleanIntervalEvent::normalized_interval`
  - `PlanarBooleanSourceInterval::source_parameter_range`
  - `PlanarBooleanSourceIntervalSense`
- `crates/worth-spatial/src/workload_platform/planar_boolean_events/interval_normalization/*`
  - collapsed interval and parameter range precedent
- `_docs/worth_topo/operators-list.md`
  - `MergeCollinearEdgeIntervals`
  - `RemoveMicroBridgeEdges`
  - `RemoveRedundantImprintEdges`

**New operators added**
- `MergeCollinearEdgeIntervals`
- `RemoveMicroBridgeEdges`
- `RemoveRedundantImprintEdges`
- `NormalizeOverlapIntervalSubdivision`
- `ValidateOverlapIntervalSubdivisionConsistency`
- `RejectMicroIntervalBelowAdmittedPolicy`

**Warnings**
- Do not let micro-interval policy silently erase topological meaning.
- Do not merge opposite-sense interval facts unless the sense-preservation
  contract is still represented in the normalized output.
- Do not run cleanup reserved for `7.8`; this phase only normalizes the split
  schedule product.

**Test requirements**
- `overlap_interval_subdivision_normalizes_redundant_collinear_boundaries`
- `micro_interval_policy_denies_or_collapses_with_explicit_decision`
- `opposite_sense_interval_normalization_preserves_source_sense`

**Engineering decisions**
- Normalized interval cuts should preserve both source-edge parameter ranges
  and normalized interval identity.
- Micro-interval handling must be typed as admitted collapse, policy-required,
  or denied.

**Open questions**
- Whether micro-bridge removal should be visible as a split decision now or
  deferred entirely to `7.8` cleanup when it affects topology outside source
  edge fragments.

### Phase 17: Split Vertex Identity Minting

Mint canonical split vertex identities from normalized split cuts, not from
coordinates or local allocation order.

**Relevant subsystems**
- `worth-spatial` split identity
- `worth-topo` topology identity precedent
- persistent naming and subshape-signature support

**Existing API references**
- `crates/worth-spatial/src/facade/planar_boolean_events.rs`
  - `PlanarBooleanPointEvent::coordinate_fact`
  - `PlanarBooleanPointEventCoordinateFact::coordinate_fact_identity`
  - `PlanarBooleanPointEventSegmentParameterFact::parameter_fact_identity`
  - `PlanarBooleanIntervalEvent::event_identity`
  - `PlanarBooleanSourceInterval::source_interval_identity`
- `_docs/worth_topo/operators-list.md`
  - `AllocateEntityId`
  - `ForkEntityLineage`
  - `ExtractStableSubshapeSignatures`
  - `PropagatePersistentNamesThroughSplit`

**New operators added**
- `MintBooleanSplitVertexIdentity`
- `CoalesceSharedSplitVertexIdentity`
- `ValidateSplitVertexIdentityCoalescence`
- `ExtractStableSubshapeSignatures`
- `RejectCoordinateOnlySplitVertexIdentity`

**Warnings**
- Do not allocate split vertex identities from vector position or allocation
  sequence.
- Do not coalesce vertices solely by coordinate equality; coalescence must be
  backed by event and carrier provenance.
- Do not let interval endpoints and point events mint competing identities for
  the same certified split location.

**Test requirements**
- `split_vertex_identity_is_stable_under_replay_and_event_order_variation`
- `shared_crossing_vertices_coalesce_by_event_provenance_not_coordinate_string`
- `coordinate_only_split_vertex_identity_is_rejected_by_public_contract`

**Engineering decisions**
- Split vertex identity basis includes source edge identity, normalized
  parameter, event provenance, carrier identities, coordinate fact identity,
  and precision basis identity.
- Coalescence decisions must be recorded as split decision-log entries.

**Open questions**
- Whether split vertex identities should be topology-reserved handles in `7.3`
  or spatial split identities that `7.4` later maps into topology authority.

### Phase 18: Split Edge Fragment Construction

Construct canonical split edge fragments from normalized schedules and split
vertex identities.

**Relevant subsystems**
- `worth-spatial` split fragment platform
- topology operator vocabulary for edge splitting

**Existing API references**
- `crates/worth-spatial/src/facade/planar_boolean_events.rs`
  - `PlanarBooleanSegmentCarrier::source_edge_identity`
  - `PlanarBooleanSegmentCarrier::carrier_identity`
  - `PlanarBooleanSegmentCarrier::local_frame_identity`
  - `PlanarBooleanSegmentCarrier::precision_basis_identity`
  - `PlanarBooleanSourceInterval::source_parameter_range`
- `_docs/worth_topo/operators-list.md`
  - `SplitEdge`
  - `SplitIntersectedEdges`
  - `SplitEdgeAtOverlapInterval`
  - `SplitEdgeAndCurves`

**New operators added**
- `BuildSplitEdgeFragments`
- `SplitEdgeAtBooleanPointEvents`
- `SplitEdgeAtOverlapInterval`
- `SplitIntersectedEdges`
- `ValidateSplitFragmentNonZeroLength`
- `RejectCollapsedSplitFragment`

**Warnings**
- Do not commit final topology truth in this phase; emit canonical fragment
  artifacts and prepared topology intent.
- Do not produce zero-length fragments from endpoint no-ops or collapsed
  intervals.
- Do not drop original source edge sense; loop reconstruction needs it.

**Test requirements**
- `split_edge_fragments_cover_source_edge_parameter_domain_without_gaps`
- `split_edge_fragment_construction_rejects_zero_length_fragments`
- `split_edge_fragments_preserve_source_edge_carrier_and_sense`

**Engineering decisions**
- Introduce `PlanarBooleanSplitEdgeFragment` with source edge identity, start
  split vertex, end split vertex, parameter range, source sense, fragment
  identity, and event-cause provenance.
- `SplitEdgeAndCurves` remains support-gated if full coupled geometry split is
  not admitted yet; the spec must still name the posture explicitly.

**Open questions**
- Whether fragment identities should include future topology target handles or
  stay topology-handle-free until `7.4`.

### Phase 19: Overlap Edge-Chain Construction

Construct canonical edge chains for overlap intervals without extracting
overlap regions or rebuilding loops.

**Relevant subsystems**
- `worth-spatial` interval-event and split fragment platform
- boolean overlap surgery operator vocabulary

**Existing API references**
- `crates/worth-spatial/src/facade/planar_boolean_events.rs`
  - `PlanarBooleanIntervalEventKind`
  - `PlanarBooleanIntervalEvent::left_source_interval`
  - `PlanarBooleanIntervalEvent::right_source_interval`
  - `PlanarBooleanSourceIntervalSense`
  - `PlanarBooleanEventGroupKind`
  - `PlanarBooleanEventGroup::source_interval_identities`
- `_docs/worth_topo/operators-list.md`
  - `ResolveEdgeEdgePartialOverlap`
  - `ResolveCoincidentButOppositeSenseEdges`
  - `ResolveCoincidentEdgesDifferentParameterization`
  - `ConvertOverlapToSharedTopology`

**New operators added**
- `BuildOverlapEdgeChain`
- `ResolveCoincidentButOppositeSenseEdges`
- `ResolveCoincidentEdgesDifferentParameterization`
- `ClassifyOverlapChainBoundaryRole`
- `ValidateCoincidentOppositeSensePreservation`

**Warnings**
- Do not extract overlap islands or loops; that belongs to `7.5`.
- Do not convert overlap to shared topology truth in `7.3`; emit prepared
  overlap-chain products and explicit posture.
- Do not erase same-direction versus opposite-sense coincidence.

**Test requirements**
- `overlap_edge_chain_preserves_partial_containment_and_identical_interval_kinds`
- `opposite_sense_overlap_chain_preserves_both_source_senses`
- `overlap_edge_chain_construction_does_not_emit_region_or_loop_products`

**Engineering decisions**
- Introduce `PlanarBooleanOverlapEdgeChain` keyed by overlap interval event
  identity and normalized source interval identities.
- The chain must carry enough source-sense and source-edge fragment identity
  for `7.5` to build overlap regions without re-reading raw intervals.

**Open questions**
- Whether `ConvertOverlapToSharedTopology` remains purely future posture or
  should have a `PreparedOnly` variant in `7.3`.

### Phase 20: Split Chain Continuity And Coverage Validation

Validate that split fragments and overlap chains cover each source edge
honestly and do not introduce gaps, overlaps, or dangling chain references.

**Relevant subsystems**
- `worth-spatial` split validation
- `worth-topo` validator vocabulary

**Existing API references**
- `_docs/worth_topo/validators.md`
  - `ValidateNoDanglingHandles`
  - `ValidateEdgeEndpointsMatchCoedgeVertices`
  - `ValidateNoUnexpectedZeroLengthEdges`
  - `ValidateCanonicalOrderingStable`
  - `ValidateNoDanglingIntersectionSpurs`
  - `ValidateConsistentVertexMergesInGraph`
- `crates/worth-spatial/src/facade/planar_boolean_events.rs`
  - source carrier, point event, interval event, and event group identities
    consumed by the split artifacts

**New operators added**
- `ValidateSplitEdgeChainClosure`
- `ValidateSplitFragmentDomainCoverage`
- `ValidateNoDanglingSplitChainReferences`
- `ValidateOverlapChainFragmentReferences`
- `RejectSplitChainGapOrOverlap`

**Warnings**
- Do not confuse source-edge coverage with loop closure. This phase proves
  per-edge chain integrity only.
- Do not allow overlap chains to reference fragments that the split fragment
  ledger did not mint.
- Do not treat validation as diagnostics-only; invalid split chains deny the
  milestone output.

**Test requirements**
- `split_chain_validation_proves_each_source_edge_domain_is_covered_once`
- `split_chain_validation_rejects_gap_overlap_or_dangling_fragment_reference`
- `overlap_chain_validation_rejects_fragment_reference_outside_source_interval`

**Engineering decisions**
- Add a validation receipt for split edge-chain coverage before naming
  propagation and ledger assembly.
- Validation counters must include source edges checked, fragments checked,
  overlap chains checked, gaps, overlaps, dangling references, and denied
  chains.

**Open questions**
- Whether this validation receipt becomes part of the final split ledger or a
  separate evidence row.

### Phase 21: Query-Native Persistent Naming And Split Identity Evolution

Freeze persistent naming as a first-class runtime artifact for the full `7.3`
split output scope. Later loop reconstruction, classification, inspection, and
ledger assembly must inherit Query/topology lineage evidence rather than
reconstructing names from geometry, display strings, local maps, or subshape
signature heuristics.

**Relevant subsystems**
- `worth-spatial` split identity, split fragments, split vertices, overlap
  chains, retained interval entries, and split-chain validation receipts
- `worth-topo` persistent naming, topology identity, naming continuity matrix,
  retained Query contribution semantics, and persistent-name live/query surfaces
- `worth-kernel` evidence/certification and final split-ledger stage gating
- `forge-query` identity evolution, structural correspondence, retained
  artifacts, declaration identity, and projection-consumption fact receipts

**Existing API references**
- `crates/forge-query/docs/AI_README.md`
  - canonical runtime-owned artifacts and declaration identity
  - Query-owned graph/index views as proof boundaries
  - typed identity artifacts instead of caller-owned strings
  - retained artifacts and typed binding/resolver surfaces
- `crates/forge-query/docs/capabilities/lineage-and-correspondence.md`
  - `IdentityEvolutionQueryContext`
  - `LineageTraversalDescriptor`
  - `admit_identity_evolution_query(...)`
  - `execute_admitted_identity_evolution_query(...)`
  - `IdentityEvolutionResultBundle`
  - `SingularIdentityContinuityResult`
  - `PluralIdentitySuccessorSet`
  - `IdentityEvolutionAmbiguityBundle`
  - `IdentityEvolutionIdentityBreakBundle`
  - `IdentityEvolutionDeniedBundle`
- `crates/worth-topo/src/query_domain.rs`
  - `ForgeQueryCapabilityFamily::IdentityEvolution`
- `crates/worth-topo/src/topology_operators/naming_continuity/*`
  - naming continuity matrix precedent
- `crates/worth-topo/src/topology_operators/mutation_sequence.rs`
  - `NamingMutationContinuityMatrix`
  - `naming_mutation_continuity_matrix_from_rows(...)`
- `crates/worth-topo/src/topology_operators/query_workflow/retained_contribution_semantics.rs`
  - retained Query contribution semantic projection precedent
- `crates/worth-topo/src/projection/runtime_boundary/declared_query_surfaces/truth_surfaces/persistent_naming.rs`
  - persistent-name live view declaration
  - Query-owned persistent-name row attachment report
- `crates/worth-topo/src/projection/runtime_boundary/query_runtime/adapters/query_rows.rs`
  - topology entity rows and persistent-name rows with `lineage.provenance`
- `crates/worth-topo/src/projection/runtime_boundary/query_runtime/tests/relation_update/successor.rs`
  - graph composition lineage summary precedent
- `_docs/worth_topo/worth-topo-query-native-migration-plan.md`
  - naming continuity crosses Query contribution lanes
  - entity rows and persistent-name rows arrive as one Query-owned live artifact
    binding rather than a local pair of reads
- `_docs/worth_topo/operators-list.md`
  - `BuildPersistentNamingMap`
  - `UpdatePersistentNamingMap`
  - `BuildPersistentNamingSeeds`
  - `PropagatePersistentNamesThroughSplit`
  - `ExtractStableSubshapeSignatures`
  - `RecordEntityParentage`
  - `ForkEntityLineage`
  - `ResolveNameConflictsAfterBoolean`
- `_docs/worth_topo/validators.md`
  - `ValidatePersistentNameUniqueness`
  - `ValidateNameSurvivalThroughSplitMerge`
  - `ValidateNoDanglingNameReferences`
  - `ValidateSelectorResolutionDeterminism`
- `crates/worth-topo/src/certification/topology_operator_closeout/acceptance_rows/validator_family_coverage.rs`
  - topology validator coverage precedent

**New operators added**
- `BuildSplitPersistentNamingMap`
- `BuildSplitPersistentNamingSeeds`
- `AdmitSplitIdentityEvolutionQuery`
- `PropagatePersistentNamesThroughSplit`
- `RecordSplitEntityParentage`
- `ForkSplitEntityLineage`
- `ExtractSplitStableSubshapeSignatures`
- `ResolveSplitNameConflictsAfterBoolean`
- `BindSplitPersistentNamesToQueryLineage`
- `ValidateSplitNameSurvival`
- `ValidateSplitPersistentNameUniqueness`
- `ValidateSplitSelectorResolutionDeterminism`
- `RejectDanglingSplitNameReference`
- `RejectSplitNameFromGeometryOrDisplayString`
- `RejectAmbiguousSplitIdentityEvolution`

**Warnings**
- Do not implement persistent naming as a local spatial sidecar, a topo-only
  naming map, or a string-derived digest. The ordinary lane must be
  Query/topology lineage shaped, with typed identity-evolution outcomes.
- Do not confuse subshape signatures with naming authority. Subshape signatures
  are structural correspondence evidence; persistent naming authority comes
  from source identity, split artifact authority, Query lineage basis, and
  topology persistent-name continuity.
- Do not claim durable store-backed historical naming parity beyond the runtime
  support posture. This phase must close complete persistent naming for the
  current retained `7.3` split ledger and replay-certified runtime path; broader
  store-backed/restart-stable historical parity remains governed by Query
  support/admission.
- Do not let geometry coordinates or display names become persistent naming
  authority.
- Do not defer split persistent naming to ledger assembly. The final split
  ledger must consume this phase's naming artifact as required evidence.

**Test requirements**
- `split_persistent_naming_binds_every_split_artifact_to_query_identity_evolution`
- `split_identity_evolution_emits_plural_successors_for_source_edge_fragments`
- `split_persistent_naming_propagates_source_edge_name_to_fragments_vertices_overlap_chains_and_retained_intervals`
- `split_persistent_naming_rejects_duplicate_or_dangling_name_references`
- `split_persistent_naming_rejects_geometry_display_string_or_coordinate_authority`
- `split_subshape_signatures_are_stable_under_replay_reversed_edge_sense_and_source_order_shuffle`
- `split_selector_resolution_remains_deterministic_after_replay_and_reversal`
- `split_identity_evolution_denies_ambiguous_or_broken_lineage_without_autopicking_a_name`
- `metaboss_split_ledger_requires_query_native_persistent_naming_receipt_before_loop_reconstruction`

**Engineering decisions**
- Add a `PlanarBooleanSplitPersistentNamingReceipt` or equivalent proof-bearing
  artifact, and require it before final split-ledger assembly.
- The receipt must carry Query lineage/evolution identity, topology persistent
  naming basis, source edge identity, split fragment identities, split vertex
  identities, overlap-chain identities, retained interval identities,
  event-cause identities, subshape signature rows, selector-resolution rows,
  counters, and typed denial posture.
- Source edge identity must evolve into plural split successors through a
  Query-shaped identity-evolution lane. A source edge that splits into multiple
  fragments is not singular continuity.
- Name propagation must distinguish source edge identity, split fragment
  identity, split vertex identity, overlap-chain identity, retained interval
  identity, event-cause identity, structural correspondence signature, and
  persistent-name identity. These are not interchangeable string categories.
- Subshape signatures must be derived evidence for correspondence and selector
  stability, not the primary name minting authority.
- Naming counters must include source identities inspected, split artifacts
  named, plural successors emitted, singular continuities emitted, ambiguity
  denials, identity-break denials, dangling references rejected, duplicate names
  rejected, selector-resolution rows emitted, and geometry/display authority
  attempts rejected.
- Public facade and compile-fail fences must prove external callers cannot
  construct split persistent-name rows, identity-evolution receipts, selector
  resolution rows, or naming receipts from raw strings, coordinates, or local
  maps.

**Open questions**
- Whether the implementation root lives in `worth-spatial` with a topology
  persistent-name/Query lineage binding, or in `worth-topo` with spatial split
  artifacts as admitted inputs. The ownership must be explicit, but the
  authority path must be Query/topology lineage native either way.
- Whether the final ledger stores the naming receipt inline or as a required
  sibling retained artifact linked by Query identity. The ledger must not be
  valid without it.

### Phase 22: Split Decision Log And Diagnostics

Record every edge-splitting decision in a machine-checkable decision log and
diagnostic surface.

**Relevant subsystems**
- `worth-spatial` split diagnostics
- `worth-kernel` boolean outcome and blocker provenance
- Query inspection and ordinary outcome posture

**Existing API references**
- `crates/worth-kernel/src/workload_composition/boolean_outcome/*`
  - `PlanarBooleanOutcomeKind`
  - `PlanarBooleanOutcomeReceipt`
  - event-extraction stop and blocker precedent
- `crates/worth-spatial/src/workload_platform/planar_boolean_events/*`
  - event denials, policy exits, ledger denials, counters
- `crates/forge-query/docs/AI_README.md`
  - ordinary outcomes, checked stops, inspection, retained artifacts
- `_docs/worth_topo/operators-list.md`
  - `RecordBooleanDecisionLog`
  - `LocalizePlanarBooleanFailure`
  - `BuildStructuredBooleanDisagreementReport`

**New operators added**
- `RecordBooleanDecisionLog`
- `RecordEdgeSplitDecisionLog`
- `LocalizePlanarBooleanFailure`
- `BuildStructuredEdgeSplitFailureReport`
- `EmitPlanarBooleanOutcome`

**Warnings**
- Do not make diagnostics change split identity or split outcome.
- Do not use human-readable strings as the only denial or decision surface.
- Do not let a denied split lose event-ledger identity, source edge identity,
  phase identity, or policy posture.

**Test requirements**
- `edge_split_decision_log_covers_every_split_collapse_coalescence_and_denial`
- `edge_split_failure_localization_identifies_phase_source_edge_and_event`
- `edge_split_diagnostics_do_not_change_operational_split_digest`

**Engineering decisions**
- Add a split decision-log receipt with O(1) lookup by decision identity or
  affected split artifact identity.
- Diagnostics are derived from split decisions and receipts, not a second
  authority path.

**Open questions**
- Whether decision logs should be stored as part of the split ledger receipt or
  as a sibling diagnostic receipt linked by identity.

### Phase 23: Split Edge-Chain Ledger Assembly

Assemble the single canonical output artifact of `7.3`.

**Relevant subsystems**
- `worth-spatial` split ledger platform
- `worth-kernel` workload evidence and stage requirements
- Query retained artifact progression

**Existing API references**
- `crates/worth-spatial/src/facade/planar_boolean_events.rs`
  - `PlanarBooleanEventLedgerReceipt`
  - `PlanarBooleanOrderedEventSet`
  - `PlanarBooleanEventLedgerCounters`
- `crates/worth-spatial/src/workload_platform/evidence_ledger/stage.rs`
  - `BooleanEvidenceStageKind::Split`
  - `WorkloadEvidenceStage::BooleanSplit`
- `crates/worth-kernel/src/workload_composition/stage_requirements.rs`
  - `WorkloadStageRequirement` existing boolean progression precedent
- `crates/worth-kernel/src/workload_composition/boolean_evidence.rs`
  - prior `BooleanEvidenceReceipt` implementations

**New operators added**
- `AssemblePlanarBooleanSplitEdgeChainLedger`
- `BuildSplitEdgeChain`
- `BuildSplitLedgerReceipt`
- `CanonicalizeSplitLedgerOrdering`
- `ValidateSplitLedgerReceiptChain`

**Warnings**
- The split ledger is the authority boundary for `7.3`; do not expose partial
  schedules, vertices, or fragments as equivalent downstream products.
- Do not let `7.4` consume source events, raw split schedules, or fragment
  vectors without the ledger receipt.
- Do not omit counters because the result looks deterministic in small tests.

**Test requirements**
- `split_edge_chain_ledger_contains_request_schedules_vertices_fragments_overlap_chains_names_and_decisions`
- `split_edge_chain_ledger_orders_all_products_canonically_across_replay`
- `split_edge_chain_ledger_rejects_missing_validation_or_name_propagation_receipts`

**Engineering decisions**
- Introduce `PlanarBooleanSplitEdgeChainLedger` and
  `PlanarBooleanSplitEdgeChainLedgerReceipt`.
- The ledger receipt must implement `BooleanEvidenceReceipt` for
  `BooleanEvidenceStageKind::Split`.
- The ledger must expose a `downstream_consumption_identity` for `7.4`.

**Open questions**
- Final naming: `SplitEdgeChainLedger`, `EdgeSplitLedger`, or
  `BooleanSplitLedger`. Prefer the name that makes `7.4` consumption obvious.

### Phase 24: Workload Evidence And Stage Requirement Closure

Wire the split ledger into workload composition so split evidence is mandatory
and receipt-backed.

**Relevant subsystems**
- `worth-kernel` workload composition
- `worth-spatial` evidence ledger
- Query retained artifact progression

**Existing API references**
- `crates/worth-kernel/src/workload_composition/worth_workload.rs`
  - `WorthWorkload`
  - `WorthWorkload::require_boolean_event_ledger`
  - existing `require_boolean_*` pattern
- `crates/worth-kernel/src/workload_composition/stage_requirements.rs`
  - `WorkloadStageRequirement`
- `crates/worth-kernel/src/workload_composition/boolean_evidence_requirement.rs`
  - boolean evidence requirement mapping pattern
- `crates/worth-spatial/src/workload_platform/evidence_ledger/stage.rs`
  - `WorkloadEvidenceStage::BooleanSplit`
  - `BooleanEvidenceStageKind::Split`
- `crates/worth-spatial/src/workload_platform/evidence_ledger/stage_counters.rs`
  - `WorkloadEvidenceStageCounters`

**New operators added**
- `RequireBooleanSplitEvidence`
- `RegisterBooleanSplitStageRequirement`
- `CountBooleanSplitEvidenceRows`
- `RejectManualBooleanSplitEvidence`

**Warnings**
- Do not rely on `WorkloadEvidenceStage::BooleanSplit` being visible; it must
  be connected to an admitted requirement and receipt-backed counter path.
- Do not allow manual split evidence rows to satisfy closeout.
- Do not widen classify/assemble/cleanup requirements in this milestone except
  as visible future posture.

**Test requirements**
- `worth_workload_requires_boolean_split_receipt_for_7_4_consumption`
- `boolean_split_evidence_rejects_manual_or_counterless_rows`
- `boolean_split_stage_requirement_maps_only_to_split_receipts`

**Engineering decisions**
- Add `WorkloadStageRequirement::BooleanSplit` if it is not already present
  when implementation begins.
- Add `WorthWorkload::require_boolean_split` or equivalent.
- Add `BooleanEvidenceReceipt` implementation for the split ledger receipt.

**Open questions**
- Whether `BooleanSplit` should be a single evidence stage for all `7.3` proof
  or split into sub-stage counters inside the split receipt.

### Phase 25: Replay, Orientation Variation, And Checkpoint Parity

Prove the split ledger is stable under replay, retained replay, reversed source
edge sense, and checkpoint/non-checkpoint execution.

**Relevant subsystems**
- `worth-spatial` replay and retained artifact proof
- `worth-kernel` workload catalog hostile recipes
- Query retained artifact progression

**Existing API references**
- `crates/worth-spatial/src/facade/workload_vocabulary/mod.rs`
  - `RetainedReplayWorkloadReceipt`
  - `WorkloadStageEnvelope`
  - retained replay stage identity surfaces
- `crates/worth-spatial/src/facade/planar_boolean_events.rs`
  - event ledger replay-stable identities
- `crates/worth-kernel/src/workload_composition/workload_catalog/*`
  - boolean workload catalog recipe substrate
- `_docs/worth_topo/validators.md`
  - `ValidatePlanarBooleanReplayParity`
  - `ValidatePlanarBooleanCheckpointParity`
  - `ValidateJournalReplayExactness`

**New operators added**
- `ReplayPlanarBooleanEdgeSplit`
- `CompareEdgeSplitReplayParity`
- `CompareEdgeSplitCheckpointParity`
- `CanonicalizeReversedEdgeSenseSplit`
- `ValidatePlanarBooleanReplayParity`

**Warnings**
- Do not prove replay by re-extracting events from raw geometry outside the
  retained artifact chain.
- Do not treat reversed source edge sense as a new semantic outcome when the
  event ledger proves the same split meaning.
- Do not let diagnostics richness alter replay digests.

**Test requirements**
- `edge_split_replay_preserves_split_ledger_digest_and_downstream_identity`
- `edge_split_reversed_source_edge_sense_preserves_canonical_fragment_set`
- `edge_split_checkpoint_and_non_checkpoint_paths_preserve_split_decisions`

**Engineering decisions**
- The replay proof compares split ledger receipts and decision-log digests, not
  just fragment counts.
- Orientation variation proof must include point events, interval events, and
  overlap chains.

**Open questions**
- Whether checkpoint parity belongs in `7.3` closeout or is partially deferred
  to `7.10`; this milestone should at least prove retained replay parity for
  the split stage.

### Phase 26: Public Contract And Anti-Theatre Fences

Make synthetic split proof mechanically harder than the real path.

**Relevant subsystems**
- `worth-kernel` public facade contracts
- `worth-spatial` public facade contracts
- `worth-topo` topology operator public-contract precedent

**Existing API references**
- `crates/worth-kernel/src/certification/public_facade_contracts/compile_fail/pb_events/*`
  - event-ledger anti-forgery compile-fail precedent
- `crates/worth-spatial/src/certification/public_facade_contracts/compile_fail/planar_boolean_events/*`
  - event classifier and predicate-binding anti-forgery precedent
- `crates/worth-topo/src/certification/public_facade_contracts/contracts/public_api_topology_operator_split_surface.rs`
  - topology split surface precedent
- `crates/worth-topo/src/certification/public_facade_contracts/compile_fail/*`
  - public topology construction and mutation anti-forgery precedent

**New operators added**
- `RejectSyntheticSplitLedgerConstruction`
- `RejectRawEventVectorSplitConsumption`
- `RejectHandFilledSplitEvidenceRows`
- `RejectCoordinateOnlySplitVertices`
- `FenceLoopReconstructionToSplitLedgerReceipt`

**Warnings**
- Do not count "private constructor exists" as enough unless public contracts
  prove callers cannot forge the artifact.
- Do not leave test-only split helpers that bypass event ledger receipt
  consumption.
- Do not allow `7.4` loop reconstruction to accept raw fragments without the
  split ledger receipt.

**Test requirements**
- `split_public_contract_rejects_synthetic_split_ledger_rows`
- `split_public_contract_rejects_raw_event_vector_and_coordinate_only_vertices`
- `loop_reconstruction_consumption_requires_split_edge_chain_ledger_receipt`

**Engineering decisions**
- Add compile-fail tests for field privacy and missing constructors on split
  request, split schedule, split vertex, split fragment, overlap chain, and
  split ledger receipt types.
- Add public-contract tests proving `WorthWorkload` requires split evidence for
  downstream split consumption.

**Open questions**
- Which fences belong in kernel versus spatial public-contract trees. The
  enforcement must be public either way.

### Phase 27: Summum Bonum Closeout Certification

Freeze the production-grade confidence test for `7.3`.

**Relevant subsystems**
- `worth-kernel` workload catalog and public contracts
- `worth-spatial` split ledger platform and public contracts
- `worth-topo` topology operator and validator vocabulary
- Query/workload retained artifact rails

**Existing API references**
- `crates/worth-kernel/src/workload_composition/workload_catalog/*`
  - real boolean operand-pair recipe substrate
- `crates/worth-kernel/src/workload_composition/worth_workload.rs`
  - `WorthWorkload::require_boolean_event_ledger`
  - new split evidence requirement from Phase 24
- `crates/worth-spatial/src/facade/planar_boolean_events.rs`
  - `PlanarBooleanSegmentPairEnumerationReceipt`
  - Query-owned candidate-index product required by Phase 0 and consumed by
    Phase 2
  - `PlanarBooleanEventLedgerReceipt`
  - `PlanarBooleanPointEvent`
  - `PlanarBooleanIntervalEvent`
  - `PlanarBooleanEventGroup`
  - `PlanarBooleanOrderedEventSet`
- `crates/worth-spatial/src/workload_platform/evidence_ledger/stage.rs`
  - `WorkloadEvidenceStage::BooleanSplit`
  - `BooleanEvidenceStageKind::Split`
- public-contract compile-fail harnesses in `worth-kernel`, `worth-spatial`,
  and `worth-topo`

**New operators added**
- `CertifyPlanarBooleanEdgeSplittingMetaboss`
- `BuildEdgeSplitMetabossWorkloadRecipe`
- `EmitEdgeSplitMetabossProofBundle`
- `ValidateEdgeSplitSummumBonumCloseout`
- `RegisterMilestone7_3CloseoutRows`

**Warnings**
- The summum bonum test is not optional polish; it is the confidence bar for
  the milestone.
- The test must start from a workload-catalog-backed boolean pair and pass
  through `7.0`, `7.1`, the Phase 0 Query-owned candidate-index product, and
  `7.2` before split work begins.
- If the test can pass with full-breadth cross-product pair enumeration as its
  only candidate-discovery proof, the closeout is wrong.
- If the test can pass with host-local candidate rows decorated by Query
  declaration/envelope digests, the closeout is wrong.
- If the test can pass while loop reconstruction consumes raw fragments,
  synthetic split rows, or re-extracted events, the closeout is wrong.

**Test requirements**
- `planar_boolean_edge_splitting_metaboss_chain_is_canonical_replayable_name_preserving_and_unforgeable`
- `edge_split_metaboss_replay_orientation_and_checkpoint_parity_hold`
- `edge_split_metaboss_proves_candidate_index_product_rows_and_culled_pair_counts`
- `edge_split_metaboss_rejects_synthetic_split_ledgers_raw_events_and_hand_filled_evidence`
- `edge_split_metaboss_rejects_cross_product_candidate_discovery_as_production_proof`
- `edge_split_metaboss_localizes_every_denial_to_phase_source_edge_and_event`

**Summum bonum workload**
- one real workload-catalog recipe that produces a planar operand pair whose
  `7.2` event ledger includes:
  - proper crossing that splits both participating source edges
  - endpoint-on-interior T-junction promotion
  - shared endpoint duplicate reports
  - collinear disjoint relation retained as no split
  - collinear endpoint touch that must not create a zero-length fragment
  - partial overlap subdividing both source edges
  - containment overlap where one edge is fully inside another
  - identical same-direction coincidence
  - identical anti-parallel coincidence
  - reversed source edge orientation replay variant
  - duplicate event reports that collapse canonically
  - micro-interval or near-duplicate split that must deny, collapse, or policy-
    exit explicitly
  - one unsupported degenerate split case that exits with typed localization

**Summum bonum assertions**
- exact candidate index identity
- exact candidate-index product query plan identity
- exact candidate-index Query product identity
- exact candidate-index Query product lifecycle outcome
- exact candidate-index Query product fallback posture
- exact proof that candidate rows were emitted by the Query product, not
  host-local construction
- exact theoretical full-breadth segment-pair count
- exact indexed candidate-pair count
- exact culled segment-pair count
- exact event-bearing emitted-pair count
- exact typed non-production outcome for any full-breadth fallback fixture
- exact event-ledger identity consumed
- exact split request identity
- exact operator classification matrix for every `7.3` operator consumed by
  the proof
- exact Query/topology declaration, grouped contribution, or graph-composition
  evidence for every topology-affecting split operator admitted by the proof
- exact validator registration and runtime-visible denial route for every
  validator family exercised by the proof
- exact source-edge carrier count
- exact point split candidate count
- exact interval split candidate count
- exact T-junction promotion count
- exact endpoint no-op count
- exact duplicate collapse count
- exact micro-interval policy/denial count
- exact split vertex identities
- exact coalesced vertex identities
- exact split fragment identities
- exact overlap edge-chain identities
- exact source edge to fragment lineage map
- exact persistent-name propagation map
- exact decision-log coverage for every split, no-op, collapse, coalescence,
  overlap-chain, denial, and policy exit
- exact split ledger digest
- exact downstream-consumption identity for `7.4`
- replay, reversed-edge-sense, and retained-replay parity digests
- graph-composition program, resolution map, lifecycle outcome, assumption
  summary, and lineage summary where the hostile case exercises symbolic or
  existing-truth topology authoring
- typed graph-composition domain-invariant denial for the unsupported
  degenerate split case when the graph substrate is supported but the domain
  topology is invalid
- public-contract proof that synthetic split ledger rows, raw event vectors,
  coordinate-only split vertices, and hand-filled split evidence cannot satisfy
  closeout

**Engineering decisions**
- The closeout proof must combine correctness, determinism, replay, naming,
  diagnostics, workload truth, counters, and anti-theatre fences in one proof
  bundle.
- The summum bonum test should become a named certification target, not only a
  unit test.

**Open questions**
- Final geometry for the hostile recipe should be chosen during implementation,
  but it must remain workload-catalog backed and exercise every listed split
  family in one real chain.

## Admitted Surface

- real `7.0`-admitted planar boolean workload entry
- real `7.1` certified common-plane reduced operand pairs
- real `7.2` planar boolean event ledgers bound to Query-owned
  candidate-index products
- point-event driven edge splitting
- endpoint contact and shared-endpoint split posture
- endpoint-on-interior T-junction promotion posture
- interval-event driven overlap subdivision
- partial overlap, containment overlap, identical same-direction coincidence,
  and identical anti-parallel coincidence as split inputs
- duplicate report collapse with retained provenance
- endpoint no-op normalization
- micro-interval denial, policy exit, or explicit collapse posture
- split vertex identity minting and coalescence
- split edge fragment construction
- overlap edge-chain construction
- split-level persistent-name and subshape-signature propagation
- split decision logs, diagnostics, counters, replay proof, and anti-theatre
  public contracts

## Excluded Surface

- loop reconstruction
- loop closure certification
- overlap-region island extraction
- fragment inside/outside classification
- keep/discard labeling
- planar face assembly
- post-split degeneracy cleanup beyond schedule-local normalization
- final topology legality certification
- shell/body result certification
- EMBER execution or B-rep/EMBER parity
- curved-edge, trim-network, p-curve, seam, or periodic-surface split execution
  except as explicit support-gated posture
- full persistent naming through merge/conflict semantics

## Workflow Surface

`7.3` is not done because one crossing edge can be split.

It is only done when admitted edge-splitting workflows operate generically over:

- arbitrary admitted source-edge counts produced by workload-catalog-backed
  planar boolean operand pairs
- arbitrary admitted candidate-index product breadth, with emitted and culled
  pair counts produced by the Query-owned product before split scheduling
- arbitrary admitted point-event counts per source edge
- arbitrary admitted interval-event counts per source edge
- arbitrary admitted duplicate reports that collapse deterministically
- arbitrary admitted endpoint no-op and T-junction promotion combinations
- arbitrary admitted overlap interval families within the `7.2` event surface
- retained replay and reversed-edge-sense variants of the same split workload
- typed failure for unsupported carrier, parameter, interval, degeneracy,
  naming, validation, or evidence cases

## Operator Closure

Milestone `7.3` closes the following operator families:

- Query indexing debt audit and candidate-index product:
  - `AuditQueryIndexingDebtSurface`
  - `RegisterPlanarBooleanSegmentCandidateIndexQueryProduct`
  - `DeclarePlanarBooleanSegmentCandidateIndexQuery`
  - `PlanPlanarBooleanSegmentCandidateIndexQuery`
  - `ExecutePlanarBooleanSegmentCandidateIndexQuery`
  - `EmitPlanarBooleanSegmentCandidateIndexProduct`
  - `EmitPlanarBooleanSegmentCandidateIndexRows`
  - `EmitPlanarBooleanSegmentCandidateIndexCounters`
  - `EmitPlanarBooleanSegmentCandidateIndexFallbackPosture`
  - `BindSegmentPairEnumerationToCandidateIndexProduct`
  - `RegisterWorkloadEvidenceStageIndexProduct`
  - `BuildWorkloadEvidenceStageIndex`
  - `BindOperatorHarnessToEvidenceStageIndex`
  - `RejectHostLocalCandidateRowsWithQueryDigestDecoration`
  - `RejectRawEvidenceRowOperatorConsumption`
  - `RejectStringPrefixStageLinkage`
- candidate-index product consumption:
  - `ConsumePlanarBooleanSegmentCandidateIndexProduct`
  - `BindEventLedgerToCandidateIndexProduct`
  - `BindSegmentPairEnumerationToCandidateIndexProduct`
  - `ValidateIndexedCandidateDiscoveryReceipt`
  - `RejectFullBreadthCandidateDiscoveryAsProductionProof`
  - `RejectQueryDigestDecoratedLocalCandidateIndex`
  - `CountCulledSegmentCandidatePairs`
  - `ValidateCandidateIndexProductLifecycleOutcome`
- split entry and admission:
  - `ConsumePlanarBooleanEventLedger`
  - `DeclarePlanarBooleanEdgeSplit`
  - `AdmitPlanarBooleanEdgeSplit`
  - `BindEdgeSplitToEventLedgerReceipt`
  - `RejectSyntheticEdgeSplitEntry`
- split policy:
  - `AdmitEdgeSplitScope`
  - `SetEdgeSplitDegeneracyPolicy`
  - `SetEdgeSplitDeterminismPolicy`
  - `SetEdgeSplitOverlapPolicy`
  - `EmitEdgeSplitPolicyOutcome`
- carrier and event indexing:
  - `RecoverBooleanSplitSourceEdgeCarriers`
  - `ValidateSplitSourceEdgeCarrierCoverage`
  - `RejectCoordinateOnlySplitCarriers`
  - `BindSplitCarrierToTopologySourceEdge`
  - `BuildSplitEventParticipationIndex`
  - `CanonicalizeSplitEventParticipationOrder`
  - `RejectUnindexedSplitEvent`
  - `ValidateSplitEventGroupCoverage`
- point and interval lowering:
  - `ExtractPointSplitCandidates`
  - `BindPointEventToSourceEdgeParameter`
  - `AdmitEdgeSplitPoint`
  - `ParameterizeEdgeSplitPoint`
  - `ValidateSplitPointParameterDomain`
  - `ExtractIntervalSplitCandidates`
  - `BindIntervalEventToSourceEdgeRange`
  - `AdmitEdgeSplitInterval`
  - `ParameterizeEdgeSplitInterval`
  - `ValidateSplitIntervalParameterDomain`
- T-junction and endpoint posture:
  - `DetectTJunctions`
  - `PromoteTJunctionToVertexSplit`
  - `InsertVertexOnEdgeForTJunction`
  - `ClassifyEndpointTouchSplitPosture`
  - `CollapseEndpointNoOpSplits`
- schedule assembly and normalization:
  - `AssemblePerEdgeSplitSchedule`
  - `InsertPointCandidateIntoSplitSchedule`
  - `InsertIntervalCandidateIntoSplitSchedule`
  - `CanonicalizeEdgeSplitPointOrder`
  - `CanonicalizeEdgeSplitIntervalOrder`
  - `CanonicalizeBooleanTraversalOrder`
  - `CollapseDuplicateSplitPoints`
  - `MergeRedundantSplitEventReports`
  - `RetainSplitDuplicateProvenance`
  - `MergeCollinearEdgeIntervals`
  - `RemoveMicroBridgeEdges`
  - `RemoveRedundantImprintEdges`
  - `NormalizeOverlapIntervalSubdivision`
- split products:
  - `MintBooleanSplitVertexIdentity`
  - `CoalesceSharedSplitVertexIdentity`
  - `BuildSplitEdgeFragments`
  - `SplitEdgeAtBooleanPointEvents`
  - `SplitEdgeAtOverlapInterval`
  - `SplitIntersectedEdges`
  - `BuildOverlapEdgeChain`
  - `ResolveCoincidentButOppositeSenseEdges`
  - `ResolveCoincidentEdgesDifferentParameterization`
- naming, diagnostics, and evidence:
  - `BuildSplitPersistentNamingSeeds`
  - `PropagatePersistentNamesThroughSplit`
  - `RecordSplitEntityParentage`
  - `ForkSplitEntityLineage`
  - `RecordBooleanDecisionLog`
  - `RecordEdgeSplitDecisionLog`
  - `LocalizePlanarBooleanFailure`
  - `BuildStructuredEdgeSplitFailureReport`
  - `EmitPlanarBooleanOutcome`
  - `AssemblePlanarBooleanSplitEdgeChainLedger`
  - `BuildSplitLedgerReceipt`
  - `RequireBooleanSplitEvidence`
  - `RegisterBooleanSplitStageRequirement`
  - `ReplayPlanarBooleanEdgeSplit`
  - `CompareEdgeSplitReplayParity`
  - `CompareEdgeSplitCheckpointParity`
  - `RejectSyntheticSplitLedgerConstruction`
  - `FenceLoopReconstructionToSplitLedgerReceipt`
- Query/topology registration and classification:
  - `RegisterEdgeSplitOperatorDeclarationFamily`
  - `RegisterEdgeSplitGroupedOperatorFamily`
  - `RegisterEdgeSplitContributionWorkflow`
  - `RegisterEdgeSplitGraphInvariantPack`
  - `MapSplitLedgerToTopologyOperatorDeclarations`
  - `ClassifyPreparedVsAuthoritativeSplitOperator`
  - `ValidateSplitOperatorQueryProgression`
  - `ValidateSplitValidatorRuntimeRegistration`

Every operator in this closure must be classified before implementation
closeout as one of:

- `PreparedSpatialOnly`
- `TopologyDeclarationFamily`
- `TopologyGroupedDeclarationFamily`
- `TopologyContributionWorkflow`
- `QueryGraphCompositionProgram`
- `SupportGatedFutureTopologyMutation`

No operator may remain an unclassified helper name.

## Validator Closure

Milestone `7.3` closes these validator families at split-edge-chain scope:

- Query candidate-discovery validators:
  - `ValidateIndexedCandidateDiscoveryReceipt`
  - `ValidateCandidateQueryPlanIdentity`
  - `ValidateCandidateIndexCoverage`
  - `ValidateCandidatePairCullAccounting`
  - `ValidateNoNPlusOneCandidateDiscovery`
  - `ValidateFullBreadthFallbackCannotCertifyProductionCloseout`
  - `ValidateCandidateRowsWereProducedByQueryProduct`
  - `ValidateCandidateIndexProductFallbackPosture`
  - `ValidateNoQueryDigestDecorationOfLocalCandidateRows`
- Query indexing debt validators:
  - `ValidateQueryIndexingDebtAuditCoverage`
  - `ValidateWorkloadEvidenceStageIndexUsage`
  - `ValidateNoRawEvidenceRowOperatorConsumption`
  - `ValidateNoStringPrefixStageLinkage`
- request and evidence validators:
  - `ValidatePlanarBooleanEventLedgerConsumption`
  - `ValidateBooleanReceiptEnvelopeConsistency`
  - `ValidateBooleanOutcomeClassificationConsistency`
  - `ValidateBooleanPolicyOutcomeConsistency`
- source and index validators:
  - `ValidateSplitSourceEdgeCarrierCoverage`
  - `ValidateSplitEventGroupCoverage`
  - `ValidateNoDanglingSplitEventReferences`
  - `ValidateNoDanglingHandles`, scoped to split artifact references
- parameter and schedule validators:
  - `ValidateSplitPointParameterDomain`
  - `ValidateSplitIntervalParameterDomain`
  - `ValidateSplitScheduleCanonicalOrdering`
  - `ValidateNoDuplicateSplitParameters`
  - `ValidateEndpointNoOpSplitPolicy`
  - `ValidateOverlapIntervalSubdivisionConsistency`
  - `ValidateShortEdgePolicyApplied`, scoped to split fragments
- identity and fragment validators:
  - `ValidateSplitVertexIdentityCoalescence`
  - `ValidateSplitFragmentNonZeroLength`
  - `ValidateSplitEdgeChainClosure`
  - `ValidateSplitFragmentDomainCoverage`
  - `ValidateOverlapChainFragmentReferences`
  - `ValidateCoincidentOppositeSensePreservation`
  - `ValidateTJunctionPromotionConsistency`
- naming and diagnostics validators:
  - `ValidatePersistentNameUniqueness`, scoped to split products
  - `ValidateNameSurvivalThroughSplitMerge`, scoped to split only
  - `ValidateNoDanglingNameReferences`
  - `ValidateBooleanDecisionLogCoverage`
  - `ValidateBooleanFailureLocalizationConsistency`
- determinism and replay validators:
  - `ValidateCanonicalOrderingStable`
  - `ValidateHashStabilityAcrossRuns`
  - `ValidateTieBreakerCoverage`
  - `ValidatePlanarBooleanReplayParity`
  - `ValidatePlanarBooleanCheckpointParity`, at least for retained split
    artifacts where admitted
- Query/runtime registration validators:
  - `ValidateSplitOperatorQueryProgression`
  - `ValidateSplitValidatorRuntimeRegistration`
  - `ValidateEdgeSplitGraphInvariantPackRegistration`
  - `ValidatePreparedSpatialSplitCannotMutateTopologyTruth`
  - `ValidateTopologyDeclarationFamilyCanonicalEntries`

Validators that govern graph or topology legality must close through one of
these runtime-visible routes:

- topology operator declaration review / progression denial
- topology grouped declaration or contribution composition denial
- Query graph-composition invariant-pack denial
- Query-registered invariant denial surfaced through the ordinary runtime

Manual executor-local validation is allowed only as a thin helper around one of
those routes, not as the authority for closeout.

## Workload Composition Additions

- Add split-edge-chain workload composition under a dedicated kernel path such
  as `crates/worth-kernel/src/workload_composition/boolean_edge_splitting/`.
- Add a Phase 0 workload/evidence binding for the Query-owned segment
  candidate-index product before split request admission.
- Add an indexed workload evidence-stage view so kernel requirement checks and
  operator harnesses do not depend on raw row scans.
- Add split-stage requirement support for `BooleanSplit` if it is not already
  admitted in `WorkloadStageRequirement`.
- Add split evidence mapping in
  `crates/worth-kernel/src/workload_composition/boolean_evidence_requirement.rs`.
- Add `BooleanEvidenceReceipt` for `PlanarBooleanSplitEdgeChainLedgerReceipt`.
- Add `WorthWorkload::require_boolean_split` or equivalent.
- Add workload catalog recipes that produce real split-hostile operand pairs
  through the existing topology, spatial, projection, transform, replay,
  diagnostics, response, common-plane, and event-ledger rails.
- Add public-contract tests proving synthetic split rows, raw event vectors,
  coordinate-only split vertices, and hand-filled split evidence cannot satisfy
  the split stage.
- Add an operator classification registry or closeout matrix that records, for
  every new `7.3` operator, whether it is prepared spatial work, a topology
  declaration family, a grouped declaration family, a contribution workflow, a
  Query graph-composition program, or support-gated future mutation.
- Add validator registration evidence showing which validator families are
  attached to split-ledger admission, topology declaration review, grouped
  contribution composition, graph-composition invariant packs, or Query
  registered invariants.
- Add Query-indexing debt audit evidence that lists every runtime lookup,
  candidate-discovery, evidence-stage, or operator-consumption surface that was
  repaired, classified as static/test-only, or blocked.

## Replay Closure

Replaying the same split request must preserve:

- split request identity
- source-edge carrier identities
- split event participation index identity
- point split candidate identities
- interval split candidate identities
- T-junction and endpoint no-op decisions
- raw and normalized split schedule identities
- split vertex identities
- coalescence decisions
- split fragment identities
- overlap edge-chain identities
- persistent-name propagation map
- decision-log digest
- split ledger digest
- downstream-consumption identity
- counters
- denial and policy posture

## Diagnostics Closure

Denials must localize whether failure occurred at:

- split request admission
- split scope / policy admission
- source-edge carrier recovery
- event participation indexing
- point candidate extraction
- point parameter admission
- interval candidate extraction
- interval parameter admission
- T-junction or endpoint posture classification
- schedule assembly
- schedule ordering
- duplicate collapse
- endpoint no-op normalization
- micro-interval or redundant interval normalization
- split vertex identity minting
- split fragment construction
- overlap edge-chain construction
- split chain validation
- naming propagation
- decision-log construction
- split ledger assembly
- workload evidence
- replay parity
- public-contract certification

## Determinism Closure

`7.3` must make the following stable:

- split request identity
- source-edge carrier ordering
- carrier-to-event participation index ordering
- point split candidate ordering
- interval split candidate ordering
- per-edge raw schedule ordering
- normalized schedule ordering
- duplicate collapse decision ordering
- endpoint no-op decision ordering
- split vertex identity
- coalesced vertex identity
- split fragment identity
- overlap edge-chain identity
- split name propagation identity
- decision-log identity
- split ledger digest
- downstream-consumption identity

## Complexity / Proof Closure

- Split work must expose counters for:
  - source segments indexed
  - theoretical full-breadth segment pairs
  - indexed candidate pairs emitted
  - culled segment pairs
  - event-bearing candidate pairs
  - candidate rows emitted by Query product
  - host-local candidate rows rejected
  - evidence-stage index lookups
  - raw evidence row operator-consumption attempts rejected
  - source edges recovered
  - point events inspected
  - interval events inspected
  - event groups inspected
  - point candidates emitted
  - interval candidates emitted
  - T-junction promotions
  - endpoint no-ops
  - raw schedule entries
  - normalized schedule entries
  - duplicate cuts collapsed
  - micro-intervals denied / collapsed / policy-exited
  - split vertices minted
  - split vertices coalesced
  - split fragments emitted
  - overlap edge chains emitted
  - name propagation rows emitted
  - decision-log rows emitted
- The complexity boundary starts at the Query-owned segment candidate-index
  product and then continues through event-ledger and source-edge schedule
  breadth; a whole-model cross product or host-local candidate-row path cannot
  be the production-confidence proof path.
- Later phases must consume the split ledger receipt; they may not rebuild
  split schedules from event rows or source geometry.
- Diagnostic richness must not change split ledger identity or operational
  counters.

## Allowed Debt

- No debt is allowed that lets full cross-operand segment-pair breadth satisfy
  production closeout. Full breadth may exist only as a named hostile baseline
  or typed non-production fallback outcome.
- No debt is allowed that lets Query declaration/envelope digests decorate
  host-local candidate rows and satisfy production closeout.
- No debt is allowed that leaves runtime operator execution dependent on raw
  evidence row vectors or string-prefix stage linkage.
- Full coupled curve/p-curve split execution may remain support-gated if
  split artifacts preserve enough posture for the future coupled path and do
  not claim geometry-binding completion.
- Full topology commit of split edges, loops, faces, shells, or bodies remains
  deferred to later `7.x` milestones.
- Full persistent naming through merge/conflict semantics remains deferred.
- Curved, non-linear, seam, periodic, and trim-network split support remains
  deferred unless explicitly admitted by existing planar surfaces.

## Milestone Done When

- every admitted `7.2` event ledger is bound to a Query-owned candidate-index
  product before it enters one canonical edge-splitting request boundary
- every runtime lookup/candidate/evidence/operator-consumption surface audited
  in Phase 0 is either repaired to a Query/indexed product, classified as
  static/test-only, or blocked from certification
- every source edge referenced by split events has a recovered split carrier
- every point and interval event is either lowered to an admitted split
  candidate, recorded as a no-op, policy-exited, or denied with typed locality
- every per-edge split schedule is ordered, normalized, and counter-bearing
- split vertices and fragments have canonical, replay-stable identities
- overlap intervals produce canonical overlap edge chains without extracting
  regions
- split-level persistent-name propagation and subshape signatures are explicit
- the split ledger is the only downstream product accepted by `7.4`
- replay, reversed-edge-sense, retained replay, and anti-theatre public
  contracts prove the split stage cannot be faked
- the summum bonum certification target passes with machine-checkable evidence
  for both split correctness and indexed candidate-planner behavior

## Acceptance Evidence

- `cargo check -p worth-spatial -p worth-kernel -p worth-topo`
- focused public-contract tests for:
  - Query-owned segment candidate-index product
  - Phase 0 Query indexing debt audit coverage
  - candidate-index product query plan identity, emitted-pair counts, and
    culled-pair counts
  - rejection of full-breadth discovery as production closeout proof
  - rejection of Query-digest-decorated host-local candidate rows
  - rejection of raw evidence row operator consumption and string-prefix stage
    linkage
  - split request admission
  - split source-edge carrier recovery
  - split event participation indexing
  - point candidate extraction and parameter admission
  - interval candidate extraction and parameter admission
  - T-junction and endpoint no-op posture
  - schedule assembly and canonical ordering
  - duplicate and interval normalization
  - split vertex identity and coalescence
  - split fragment construction
  - overlap edge-chain construction
  - split naming propagation
  - split decision logs and diagnostics
  - split ledger receipt
  - split workload evidence
- compile-fail proof that split request, schedule, vertex, fragment, overlap
  chain, name propagation, decision log, and ledger receipt artifacts cannot be
  forged from raw coordinates or raw events
- replay proof that the same event ledger produces the same split ledger
- reversed-edge-sense proof where semantics permit
- retained replay / checkpoint parity proof where admitted
- workload catalog proof that hostile split recipes are workload-backed
- Query candidate-discovery proof showing candidate-index product identity,
  query plan identity, emitted-pair count, culled-pair count, event-bearing pair
  count, fallback posture, and no N+1/cross-product or Query-digest-decorated
  host-local production path
- operator classification matrix proving no `7.3` operator is an unclassified
  helper or pseudo-runtime entry
- Query/topology declaration proof for every topology-affecting split operator
  admitted in `7.3`
- grouped/contribution workflow proof for every split operator that relies on
  topology grouped neighborhoods or semantic contributions
- graph-composition/invariant-pack proof for every graph-shaped split topology
  program admitted in `7.3`
- validator registration proof showing graph/topology legality validators deny
  through runtime-visible Query or topology declaration lanes
- summum bonum test:
  `planar_boolean_edge_splitting_metaboss_chain_is_canonical_replayable_name_preserving_and_unforgeable`

## Sequencing Notes

- Do not start `7.4` loop reconstruction until `7.3` closes with a split
  ledger receipt that loop work can consume.
- Do not put loop reconstruction or overlap-region extraction into `7.3`.
- Do not widen into EMBER here.
- If a Query-owned retained artifact, support, inspection, outcome, or evidence
  boundary is missing, extend the Query-shaped path or mark the split surface
  blocked rather than inventing a local runtime lane.
- If additional hostile recipes are needed, add them through the workload
  catalog. Do not write split-only geometry fixtures and call them proof.

## Required Self-Check

- Does the milestone solve a real structural problem or just package work
  cosmetically? Yes: it freezes the edge-level split authority that loop
  reconstruction, overlap extraction, and classification depend on.
- Is the adversarial constraint precise and load-bearing? Yes: it requires one
  canonical, replay-stable, provenance-preserving, unforgeable split edge-chain
  ledger from a real event ledger.
- Does the roadmap justify this milestone now? Yes: `7.4` cannot honestly
  rebuild loops until split edge chains exist.
- Does the spec preserve crate authority boundaries? Yes: Query owns runtime
  entry and retained artifacts, `worth-kernel` owns workload composition,
  `worth-spatial` owns split artifacts, and `worth-topo` owns topology truth and
  eventual topology operator execution.
- Are the phases carrying most of the real design information? Yes.
- Is each phase centered on one conceptual detail or boundary? Yes.
- Does each phase contain at least 2 adversarial tests by default? Yes.
- Could a competent engineer map this spec into honest types, modules, and
  tests? Yes.
- Does the milestone belong in this roadmap sequence, or is it out of order?
  It belongs immediately after `7.2` because it consumes the event ledger and
  produces the canonical split edge-chain product for `7.4`.
