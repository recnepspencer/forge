# Worth Milestone 7.2: Planar Boolean Event Extraction

> **Status:** Draft
>
> **Purpose:** freeze the canonical point / segment / interval event ledger
> that later planar B-rep boolean split work must consume instead of
> recomputing raw segment relations.

## Goal

Milestone `7.2` closes the gap between the `7.1` certified common-plane
reduction and the `7.3` edge-splitting phase.

By the end of this milestone:

- a `7.1` reduced planar boolean operand pair can enter one and only one event
  extraction path
- projected boundary segments are carried with operand, loop, edge, projection,
  and common-plane provenance
- segment-pair enumeration is complete, deterministic, counter-bearing, and
  impossible to substitute with hand-picked segment pairs
- proper crossings, endpoint contacts, shared endpoints, collinear touching,
  disjoint collinearity, partial overlaps, containment overlaps, identical
  coincidence, and anti-parallel coincidence each have typed event products
- degenerate or unsupported micro-events deny with typed policy posture before
  they can poison later split work
- later milestones consume one canonical event ledger rather than re-running
  segment relation classification

Milestone `7.2` does **not** split edges, rebuild loops, extract overlap
regions, classify fragments, assemble result faces, or perform topology cleanup.
It freezes the event substrate those later phases must inherit.

## Why This Milestone Exists

The tempting mistake after common-plane reduction is to treat event extraction
as a geometry helper: loop over segment pairs, call an intersection predicate,
then hand a list of points to split code.

That is not enough for production-grade booleans. The split pipeline needs to
know not merely where two segments touch, but what was proven, which source
topology produced it, which predicate and precision basis certified it, whether
the event is a point or interval, whether an interval is anti-parallel or
contained, whether duplicate reports collapsed to one canonical event, and
whether later replay can produce exactly the same ledger.

`7.2` exists to make that event truth explicit. The result must be a
proof-bearing event ledger that later edge-splitting consumes as authority.

## Governing Summaries

- `MENTALITY.md`: protect the hard failure first. The milestone must assume
  the event lattice is hostile and freeze the full event-proof substrate before
  any split feature can claim success.
- `arch_laws.md`: protect proof-bearing phase transitions. Each phase must
  consume the previous proof artifact and produce the next one so downstream
  code never defensively re-derives segment relation truth.
- `composition_laws.md`: protect semantic decomposition. Segment carriers,
  pair enumeration, predicate binding, point events, interval events, grouping,
  denial posture, and ledger certification must not collapse into one giant
  classifier.
- `domain_structure_laws.md`: protect visible ownership. Query and workload
  composition own entry/proof rails; `worth-spatial` owns planar event
  semantics; `worth-kernel` owns workload composition and evidence-stage
  pressure; `worth-topo` remains topology truth.
- `perf_laws.md`: protect bounded breadth and visible cost. Segment-pair
  enumeration must expose breadth counters and canonical ordering rather than
  hiding scalar loops behind cheap-looking APIs.
- `_docs/worth/milestone-7-roadmap.md`: protect `7.2` as event extraction
  only. Edge splitting, loop reconstruction, overlap-region extraction, and
  classification belong to later `7.x` milestones.
- `_docs/worth/milestone-7.1.md`: protect the certified reduced-pair artifact.
  `7.2` must consume the `7.1` common-plane reduction and must not reselect a
  local frame or reprojection path.
- `crates/forge-query/docs/AI_README.md`: protect the rule `declare intent
  once, lower it once, execute or inspect it through canonical runtime-owned
  artifacts`. `7.2` may add domain event artifacts, but must not invent a
  local pseudo-Query route, caller-owned identity, or support-posture shortcut.

## Adversarial Constraint

Given a real Query/workload-composed planar boolean operand pair containing the
admitted point, segment, and interval contact families, the system must produce
one canonical event ledger that is complete, deterministic,
provenance-preserving, replay-identical, counter-bearing, and impossible to
replace with synthetic or hand-built event evidence.

For the same `7.1` reduced operand pair, event extraction must either:

- deny before event construction with a typed, localized, replay-stable reason

or:

- emit one event ledger whose segment-carrier identities, segment-pair
  enumeration, predicate basis, point events, interval events, grouping,
  ordering, counters, and downstream-consumption identity remain stable across
  replay, benign enumeration variation, reversed segment orientation, and
  semantically valid operand-order variation.

If `7.3` still has to ask "do these segments intersect?" instead of consuming
`7.2` event products, this milestone has failed.

## Product Decision Lock

- `7.2` starts from the `7.1` reduced operand pair and nowhere else.
- Query continues to own declaration, admission, support posture, runtime
  handles, receipt/envelope shape, and retained artifact progression.
- `worth-kernel` owns workload catalog additions, workload composition, stage
  requirements, evidence rows, and public anti-theatre fences.
- `worth-spatial` owns segment-carrier extraction from the reduced pair,
  planar event classification semantics, event identity, event grouping,
  event-ledger receipts, diagnostics, replay, and counters.
- `worth-topo` owns topology truth and source edge/loop/face identity. `7.2`
  may preserve topology provenance but must not rewrite topology.
- No phase may count raw segment fixtures, hand-built event rows, synthetic
  segment-pair lists, or re-extraction replay as closeout proof.
- `Milestone 8` remains EMBER. `7.2` stays in the B-rep planar lane.

## Existing Surface Inventory

Milestone `7.2` should widen live surfaces before inventing new ones:

- `crates/worth-kernel/src/workload_composition/boolean_common_plane_reduction/*`
- `crates/worth-kernel/src/workload_composition/worth_workload.rs`
- `crates/worth-kernel/src/workload_composition/stage_requirements.rs`
- `crates/worth-kernel/src/workload_composition/workload_catalog/*`
- `crates/worth-spatial/src/workload_platform/planar_boolean_common_plane/*`
- `crates/worth-spatial/src/planar_contracts/segment_segment_2d/*`
- `crates/worth-spatial/src/facade/planar_segment_segment.rs`
- `crates/worth-spatial/src/planar_contracts/predicate_authority/*`
- `crates/worth-spatial/src/planar_contracts/predicate_consumption/*`
- `crates/worth-spatial/src/workload_platform/evidence_ledger/*`
- `crates/worth-spatial/src/workload_platform/workload_operators/*`
- `crates/worth-spatial/src/workload_platform/projected_overlap_faces/*`
- `crates/worth-spatial/src/certification/public_facade_contracts/*`
- `crates/worth-kernel/src/certification/public_facade_contracts/*`

New `7.2` surfaces are allowed where existing surfaces cannot honestly express:

- a proof-bearing event extraction request from a reduced operand pair
- reduced-pair segment carriers with source topology and projection provenance
- canonical segment identities and orientation-normalized endpoint facts
- complete segment-pair enumeration with counters
- typed point and interval event products
- typed degenerate / unsupported micro-event denial posture
- one certified planar boolean event ledger consumed by `7.3`
- workload and public-contract proof that event extraction used real workload
  rails rather than synthetic segment fixtures

## Phase Plan

### Phase 1: Event Extraction Request Boundary

Freeze the only artifact that may enter `7.2`: a request built from the `7.1`
reduced operand pair.

**Relevant subsystems**
- `worth-kernel` workload composition
- `worth-spatial` planar boolean common-plane platform
- Query retained artifact progression

**Construction requirements**
- Add a proof-bearing event extraction request that consumes
  `PlanarBooleanCommonPlaneReducedOperandPairRequest`.
- Preserve inside the request:
  - boolean declaration identity
  - reduced operand-pair identity
  - shared-plane identity
  - local-frame identity
  - operand projection identities
  - precision agreement identity
  - workload evidence-stage identity
- Expose the request only through the appropriate facade or workload-composition
  public boundary.
- Reject any request built from raw projected points, raw segment pairs,
  spatial-only fixtures, or a generic workload summary.
- Construction target files:
  - `crates/worth-kernel/src/workload_composition/boolean_event_extraction/*`
  - `crates/worth-kernel/src/workload_composition/mod.rs`
  - `crates/worth-kernel/src/workload_composition/worth_workload.rs`
  - `crates/worth-spatial/src/workload_platform/planar_boolean_events/*`
  - `crates/worth-spatial/src/facade/planar_boolean_events.rs`

**Relevant APIs**
- `PlanarBooleanCommonPlaneReducedOperandPairRequest`
- `PlanarBooleanDeclarationReceipt`
- `PlanarBooleanCommonPlaneSharedPlaneIdentityReceipt`
- `PlanarBooleanCommonPlaneLocalFrameSelectionReceipt`
- new `PlanarBooleanEventExtractionRequest`

**Required Query posture**
- required now:
  - retained artifact to next step
  - declaration progression
  - receipt / envelope identity preservation
  - support posture carried from the admitted planar boolean lane
- support-gated:
  - any future EMBER event lane
- out:
  - local event extraction entrypoints that bypass the reduced pair

**Warnings**
- Do not accept the `7.1` receipt data as copied fields without preserving the
  receipt identity that makes it authoritative.
- Do not add a second boolean event route in `worth-spatial` that the kernel
  cannot evidence.

**Test requirements**
- `event_extraction_request_preserves_reduced_pair_identity_across_replay`
- `event_extraction_request_rejects_raw_projected_segment_substitution`
- `event_extraction_request_rejects_mismatched_common_plane_and_projection_ids`

**Engineering decisions**
- The request boundary is a proof transition, not an ergonomic constructor.
- `7.2` starts with a reduced-pair request so later event phases do not reopen
  plane, frame, projection, or precision authority.

**Open questions**
- Final public facade name: `planar_boolean_events` versus a narrower
  `planar_boolean_event_extraction`.

### Phase 2: Reduced-Pair Segment Carrier Extraction

Freeze the projected boundary-segment carriers that event extraction may reason
about.

**Relevant subsystems**
- `worth-spatial` planar boolean event platform
- `worth-topo` topology provenance
- `worth-kernel` workload catalog recipes

**Construction requirements**
- Add a carrier artifact for every projected boundary segment in the reduced
  pair.
- Each carrier must preserve:
  - operand side
  - source face identity
  - source loop identity
  - source edge identity
  - loop role when available
  - projected endpoint facts
  - local-frame identity
  - projection stage identity
  - precision basis identity
- Deny before pair enumeration if any carrier lacks source topology provenance
  or projected endpoint proof.
- Add real workload catalog recipe variants that produce carrier-rich clean
  planar body pairs for event extraction, rather than hand-built segment lists.
- Construction target files:
  - `crates/worth-spatial/src/workload_platform/planar_boolean_events/segment_carriers/*`
  - `crates/worth-kernel/src/workload_composition/workload_catalog/recipe_kind.rs`
  - `crates/worth-kernel/src/workload_composition/workload_catalog/recipe_pipeline.rs`
  - `crates/worth-kernel/src/workload_composition/workload_catalog/boolean_operand_pair.rs`
  - `crates/worth-spatial/src/certification/public_facade_contracts/contracts/planar_boolean_events/*`

**Relevant APIs**
- `PlanarBooleanCommonPlaneReducedOperandPairRequest`
- `CertifiedProjectedSegment2D`
- `ProjectedPlanarWorkload`
- topology workload source identities exposed by the reduced pair
- new `PlanarBooleanSegmentCarrierSet`

**Required Query posture**
- required now:
  - projection consumption and typed fact preservation
  - materialized facts without reopening source authority
  - declaration identity carried through retained artifacts
- support-gated:
  - event carriers for non-B-rep or EMBER lanes
- out:
  - segment carriers synthesized from test-local coordinates

**Warnings**
- Do not reduce carriers to endpoint coordinates. Segment identity is topology
  provenance plus projection proof, not geometry alone.
- Do not let missing loop/edge provenance silently become a diagnostic warning.

**Test requirements**
- `segment_carriers_preserve_operand_loop_edge_and_projection_provenance`
- `segment_carrier_extraction_rejects_coordinate_only_segments`
- `catalog_event_recipes_produce_real_carrier_backed_operand_pairs`

**Engineering decisions**
- Segment-carrier extraction is distinct from segment-pair enumeration.
- Carrier identity must be stable enough for later split provenance and
  duplicate-event suppression.

**Open questions**
- Whether loop role should be mandatory in `7.2` or carried as admitted
  optional posture until loop reconstruction.

### Phase 3: Canonical Segment Identity And Endpoint Normalization

Freeze the per-segment identity and orientation-normalized endpoint model used
by every event family.

**Relevant subsystems**
- `worth-spatial` planar boolean event platform
- `worth-spatial` planar segment-segment contracts

**Construction requirements**
- Add canonical segment identity derived from carrier provenance and projection
  proof, not from raw coordinate formatting.
- Add an orientation-normalized endpoint representation that can reason about:
  - original source direction
  - canonical low/high parameter direction
  - endpoint identity
  - endpoint coordinate fact
  - segment length admissibility
- Deny zero-length or collapsed projected segments before pair enumeration.
- Record whether endpoint orientation was reversed during canonicalization so
  later interval events can preserve source sense.
- Construction target files:
  - `crates/worth-spatial/src/workload_platform/planar_boolean_events/segment_identity/*`
  - `crates/worth-spatial/src/workload_platform/planar_boolean_events/endpoint_normalization/*`
  - `crates/worth-spatial/src/planar_contracts/segment_segment_2d/*`
  - `crates/worth-spatial/src/facade/planar_boolean_events.rs`

**Relevant APIs**
- `CertifiedProjectedSegment2D`
- `CertifiedSegmentSegment2D`
- `CertifiedSegmentSegment2DReceipt`
- new `PlanarBooleanCanonicalSegment`
- new `PlanarBooleanNormalizedEndpointPair`

**Required Query posture**
- required now:
  - canonical identity preservation
  - typed artifacts instead of caller-owned string digests
- support-gated:
  - non-linear edge segment identity
- out:
  - `Debug` / `Display` / coordinate-string identity

**Warnings**
- Reversing a segment for canonical comparison must not erase the original edge
  sense required by later split topology.
- Endpoint normalization must not choose a tolerance independently of the `7.1`
  precision basis.

**Test requirements**
- `canonical_segment_identity_is_stable_under_endpoint_order_reversal`
- `endpoint_normalization_rejects_zero_length_or_collapsed_segments`
- `canonical_segment_identity_does_not_depend_on_debug_or_display_strings`

**Engineering decisions**
- Segment identity and endpoint normalization are their own phase because every
  later event classification depends on them.
- Collapsed segment denial happens before pair enumeration to avoid poisoned
  cardinality.

**Open questions**
- Whether same-operand duplicate canonical segments deny in this phase or in a
  later degenerate posture phase.

### Phase 4: Segment-Pair Enumeration And Completeness Proof

Freeze the complete cross-operand segment-pair worklist and its breadth
accounting.

**Relevant subsystems**
- `worth-spatial` planar boolean event platform
- `worth-kernel` workload evidence ledger

**Construction requirements**
- Enumerate all admitted cross-operand segment pairs from the canonical carrier
  sets.
- Preserve deterministic pair ordering independent of input vector ordering.
- Emit counters for:
  - operand A segment count
  - operand B segment count
  - expected pair breadth
  - emitted pair breadth
  - skipped pairs by typed policy, if any
- Add a pair-enumeration receipt that later event classifiers consume.
- Deny if emitted pair breadth does not match the expected breadth for the
  admitted pair policy.
- Construction target files:
  - `crates/worth-spatial/src/workload_platform/planar_boolean_events/pair_enumeration/*`
  - `crates/worth-spatial/src/workload_platform/evidence_ledger/*`
  - `crates/worth-kernel/src/workload_composition/stage_requirements.rs`
  - `crates/worth-kernel/src/workload_composition/worth_workload.rs`

**Relevant APIs**
- `WorkloadEvidenceStage`
- `WorkloadStageRequirement`
- `WorkloadEvidenceStageCounters`
- new `PlanarBooleanSegmentPairEnumerationReceipt`
- new `PlanarBooleanSegmentPairWorkItem`

**Required Query posture**
- required now:
  - planning before execution
  - canonical ordering
  - boundary counters exposed through receipt artifacts
- support-gated:
  - spatial acceleration structure admission
- out:
  - hidden nested loops with no evidence-stage receipt

**Warnings**
- This phase may use brute-force cross product for the admitted class, but the
  breadth must be explicit and counter-tested.
- Do not skip same-bounding-box-negative pairs silently unless the skip policy
  is itself a typed proof artifact.

**Test requirements**
- `segment_pair_enumeration_emits_exact_cross_operand_breadth`
- `segment_pair_enumeration_is_deterministic_under_input_order_variation`
- `segment_pair_enumeration_rejects_missing_or_synthetic_pair_rows`

**Engineering decisions**
- Pair enumeration is a planned workload product, not an implementation detail
  inside point/interval classifiers.
- A dedicated boolean evidence stage such as `BooleanEventExtraction` or
  `BooleanSegmentPairEnumeration` is required so tests prove this work is real.

**Open questions**
- Whether to add one combined `BooleanEventExtraction` stage now or split
  pair-enumeration and final-ledger stages as separate evidence requirements.

### Phase 5: Predicate And Precision Basis Binding

Freeze the predicate authority every event decision must consume.

**Relevant subsystems**
- `worth-spatial` planar predicate authority
- `worth-spatial` planar segment-segment contracts
- Query-native planar predicate bindings

**Construction requirements**
- Bind the segment-pair worklist to the existing certified planar predicate and
  segment-segment contract surfaces.
- Preserve in the event extraction context:
  - predicate authority identity
  - precision basis identity
  - segment-segment contract identity
  - local-frame identity
  - reduced-pair identity
- Require every event classifier to consume this bound context instead of
  invoking raw math independently.
- Deny if the predicate context does not match the reduced-pair precision or
  local-frame identity.
- Construction target files:
  - `crates/worth-spatial/src/workload_platform/planar_boolean_events/predicate_binding/*`
  - `crates/worth-spatial/src/planar_contracts/predicate_authority/*`
  - `crates/worth-spatial/src/planar_contracts/predicate_consumption/*`
  - `crates/worth-spatial/src/planar_contracts/segment_segment_2d/*`
  - `crates/worth-spatial/src/bindings/query_native_planar_segment_segment/*`

**Relevant APIs**
- `CertifiedSegmentSegment2DContracts`
- `CertifiedSegmentSegment2DReceipt`
- `PlanarPredicateAuthority`
- `PredicateConsumptionReceipt`
- new `PlanarBooleanEventPredicateBinding`

**Required Query posture**
- required now:
  - typed fact consumption
  - support posture and admission for predicate surfaces
  - no lower-runtime bypass
- support-gated:
  - alternate exact-arithmetic predicate lanes
- out:
  - ad hoc tolerances in event classifiers

**Warnings**
- Do not let classifier code call raw vector math and then wrap the answer in a
  receipt-looking type.
- Do not rebuild predicate support posture locally from visible APIs.

**Test requirements**
- `event_predicate_binding_preserves_reduced_pair_precision_and_frame_identity`
- `event_predicate_binding_rejects_mismatched_segment_segment_contracts`
- `event_classifiers_cannot_compile_without_predicate_binding_context`

**Engineering decisions**
- Predicate binding is a named phase because it is the authority seam between
  geometric computation and event truth.
- Every event record must be traceable to this binding.

**Open questions**
- Whether exact arithmetic admission remains advisory in `7.2` or gets an
  explicit support-gated posture row.

### Phase 6: Proper Crossing And Endpoint-Interior Point Events

Freeze point-event extraction for non-collinear segment relations that produce
one point event.

**Relevant subsystems**
- `worth-spatial` planar boolean event platform
- `worth-spatial` planar segment-segment contracts

**Construction requirements**
- Classify non-collinear segment-pair receipts into point-event variants:
  - proper interior / interior crossing
  - operand A endpoint on operand B interior
  - operand B endpoint on operand A interior
  - rejected near-miss under the bound predicate basis
- Emit typed point events with:
  - event identity
  - event coordinate fact
  - segment-pair identity
  - source carrier identities
  - parameter value on each participating segment
  - predicate receipt identity
  - contact kind
- Deny if a non-collinear relation is ambiguous under the admitted predicate
  basis instead of guessing.
- Construction target files:
  - `crates/worth-spatial/src/workload_platform/planar_boolean_events/point_events/*`
  - `crates/worth-spatial/src/workload_platform/planar_boolean_events/event_identity/*`
  - `crates/worth-spatial/src/planar_contracts/segment_segment_2d/classification.rs`
  - `crates/worth-spatial/src/certification/public_facade_contracts/contracts/planar_boolean_events/*`

**Relevant APIs**
- `CertifiedSegmentSegment2DReceipt`
- `CertifiedProjectedSegment2D`
- new `PlanarBooleanPointEvent`
- new `PlanarBooleanPointEventKind`

**Required Query posture**
- required now:
  - consumption of certified predicate / segment relation receipts
  - event identity derived from canonical artifacts
- support-gated:
  - curved-edge point events
- out:
  - coordinate-only point event construction

**Warnings**
- Do not collapse proper crossing and endpoint-interior contact into a generic
  point event; later splitting and classification need the distinction.
- Do not use approximate coordinate equality to infer endpoint ownership.

**Test requirements**
- `proper_crossing_point_event_is_stable_under_segment_orientation_reversal`
- `endpoint_interior_point_event_preserves_which_operand_contributed_endpoint`
- `near_endpoint_miss_does_not_become_endpoint_contact_without_predicate_proof`

**Engineering decisions**
- Point-event identity includes contact kind and carrier provenance, not just
  coordinates.
- Ambiguity is a denial or policy posture, never silent rounding.

**Open questions**
- Whether parameter values should be represented as normalized scalars now or
  as certified endpoint/interior facts until exact parameter policy closes.

### Phase 7: Shared-Endpoint Point Events And Duplicate Suppression

Freeze endpoint / endpoint contact extraction and deduplication for closure
seams and multi-edge contacts.

**Relevant subsystems**
- `worth-spatial` planar boolean event platform
- `worth-topo` source loop/edge provenance

**Construction requirements**
- Classify endpoint / endpoint contacts separately from endpoint / interior
  contacts.
- Preserve both source endpoint identities when two segment carriers share the
  same point.
- Suppress duplicate point events caused by loop closure seams or adjacent
  segment-pair reports.
- Emit counters for:
  - shared-endpoint candidate count
  - emitted shared-endpoint events
  - duplicate point reports suppressed
  - high-valence point groups detected
- Construction target files:
  - `crates/worth-spatial/src/workload_platform/planar_boolean_events/shared_endpoint_events/*`
  - `crates/worth-spatial/src/workload_platform/planar_boolean_events/point_deduplication/*`
  - `crates/worth-spatial/src/workload_platform/planar_boolean_events/counters/*`
  - `crates/worth-spatial/src/certification/public_facade_contracts/contracts/planar_boolean_events/*`

**Relevant APIs**
- `PlanarBooleanPointEvent`
- `PlanarBooleanSegmentPairWorkItem`
- topology endpoint / edge provenance carried by segment carriers
- new `PlanarBooleanSharedEndpointEvent`

**Required Query posture**
- required now:
  - retained identity rather than coordinate-only dedupe
  - projection-consumed facts preserved through event products
- support-gated:
  - same-operand endpoint dedupe policies beyond the admitted cross-operand
    event scope
- out:
  - deduplication by display coordinate string

**Warnings**
- Duplicate suppression must not erase high-valence information. One canonical
  point event may still carry many participating source endpoints.
- Loop closure seam contacts must be handled intentionally, not fixed in later
  topology splitting.

**Test requirements**
- `shared_endpoint_events_collapse_duplicate_loop_closure_reports_once`
- `shared_endpoint_event_identity_is_stable_under_operand_pair_enumeration_order`
- `high_valence_shared_endpoint_group_preserves_all_participating_carriers`

**Engineering decisions**
- Shared endpoints get their own phase because duplicate suppression changes
  event cardinality.
- High-valence grouping is detected and carried here, even though topology
  resolution waits for later milestones.

**Open questions**
- Whether high-valence groups should be advisory event metadata or a distinct
  event-family product in `7.2`.

### Phase 8: Collinear Relation Classification Boundary

Freeze the decision table for collinear segment pairs before interval
construction begins.

**Relevant subsystems**
- `worth-spatial` planar boolean event platform
- `worth-spatial` planar segment-segment contracts

**Construction requirements**
- Classify collinear segment pairs into:
  - disjoint
  - touching at one endpoint
  - partial overlap
  - containment overlap
  - identical same-direction coincidence
  - identical anti-parallel coincidence
  - unsupported degenerate collinearity
- Emit a collinear classification receipt that interval extraction consumes.
- Ensure disjoint collinear pairs emit a typed no-event relation rather than
  disappearing silently.
- Ensure one-point collinear touches become point events or touch relations
  without being confused with overlap intervals.
- Construction target files:
  - `crates/worth-spatial/src/workload_platform/planar_boolean_events/collinear_classification/*`
  - `crates/worth-spatial/src/planar_contracts/segment_segment_2d/classification.rs`
  - `crates/worth-spatial/src/certification/public_facade_contracts/contracts/planar_boolean_events/*`

**Relevant APIs**
- `CertifiedSegmentSegment2DReceipt`
- `PlanarBooleanSegmentPairWorkItem`
- new `PlanarBooleanCollinearRelation`
- new `PlanarBooleanCollinearRelationReceipt`

**Required Query posture**
- required now:
  - typed relation products instead of lost no-event cases
  - canonical identity for relation receipts
- support-gated:
  - fuzzy collinearity policy beyond current predicate support
- out:
  - boolean flags such as `is_collinear_overlap`

**Warnings**
- Collinear disjoint is a classification result, not absence of evidence.
- Collinear touch and zero-length interval must not collapse into the same
  representation.

**Test requirements**
- `collinear_disjoint_pairs_emit_typed_no_event_relation`
- `collinear_touching_endpoint_does_not_emit_overlap_interval`
- `anti_parallel_identical_segments_classify_distinct_from_same_direction`

**Engineering decisions**
- Collinear relation classification precedes interval extraction so interval
  code consumes a narrowed proof type.
- Same-direction and anti-parallel coincidence remain distinct because later
  split and overlap-region work need source sense.

**Open questions**
- Whether collinear no-event relations remain in the final event ledger or in a
  separate diagnostic relation section.

### Phase 9: Interval Event Extraction And Endpoint Normalization

Freeze interval-event products for collinear overlap and coincidence.

**Relevant subsystems**
- `worth-spatial` planar boolean event platform
- `worth-spatial` coplanar overlap contract family

**Construction requirements**
- Consume only the collinear relation receipt from phase 8.
- Emit typed interval events for:
  - partial overlap
  - containment overlap
  - identical same-direction coincidence
  - identical anti-parallel coincidence
- Normalize interval endpoints to the canonical segment direction while
  preserving original source direction and sense for each carrier.
- Preserve interval parameter bounds on both participating segments.
- Deny or policy-exit if the interval collapses to a point after normalization.
- Construction target files:
  - `crates/worth-spatial/src/workload_platform/planar_boolean_events/interval_events/*`
  - `crates/worth-spatial/src/workload_platform/planar_boolean_events/interval_normalization/*`
  - `crates/worth-spatial/src/planar_contracts/coplanar_overlap_contract/*`
  - `crates/worth-spatial/src/certification/public_facade_contracts/contracts/planar_boolean_events/*`

**Relevant APIs**
- `PlanarBooleanCollinearRelationReceipt`
- `CoplanarOverlapContractReceipt`
- `CertifiedSegmentSegment2DReceipt`
- new `PlanarBooleanIntervalEvent`
- new `PlanarBooleanIntervalEventKind`

**Required Query posture**
- required now:
  - typed consumed facts from existing overlap / segment contracts
  - event identity independent of local coordinate formatting
- support-gated:
  - interval events on curved or non-linear carriers
- out:
  - raw `[start, end]` intervals with no segment provenance

**Warnings**
- Containment overlap is not the same semantic event as partial overlap, even
  if both produce an interval.
- Identical anti-parallel coincidence must preserve opposite source sense
  rather than normalizing it away.

**Test requirements**
- `partial_overlap_interval_event_preserves_normalized_and_source_sense_bounds`
- `containment_overlap_interval_event_preserves_contained_segment_identity`
- `identical_anti_parallel_interval_event_preserves_opposite_source_sense`
- `collapsed_interval_after_normalization_denies_instead_of_becoming_overlap`

**Engineering decisions**
- Interval events carry both canonical interval facts and source-sense facts.
- Collapsed intervals are denied or classified as point touches before they can
  reach split work.

**Open questions**
- Whether interval events should carry overlap-region hints now or defer all
  island semantics to `7.5`.

### Phase 10: Event Grouping, Canonical Ordering, And Ledger Assembly

Freeze one canonical event ledger assembled from point events, interval events,
no-event relations where retained, and denial/policy posture.

**Relevant subsystems**
- `worth-spatial` planar boolean event platform
- `worth-kernel` workload composition
- `worth-spatial` evidence ledger

**Construction requirements**
- Assemble a ledger that includes:
  - reduced-pair identity
  - event extraction request identity
  - segment-carrier set identity
  - segment-pair enumeration receipt identity
  - predicate-binding identity
  - point event list
  - interval event list
  - relation / no-event diagnostics where admitted
  - event groups
  - counters
  - downstream-consumption identity for `7.3`
- Canonically order events by declared identity basis, not insertion order.
- Group coincident events that share the same canonical location or interval
  while preserving all source carriers.
- Emit exact counters for all event families and duplicate suppression.
- Construction target files:
  - `crates/worth-spatial/src/workload_platform/planar_boolean_events/event_ledger/*`
  - `crates/worth-spatial/src/workload_platform/planar_boolean_events/event_grouping/*`
  - `crates/worth-spatial/src/workload_platform/evidence_ledger/*`
  - `crates/worth-kernel/src/workload_composition/boolean_event_extraction/*`
  - `crates/worth-kernel/src/certification/public_facade_contracts/contracts/*`

**Relevant APIs**
- `WorkloadEvidenceStage`
- `WorkloadStageRequirement`
- `PlanarBooleanPointEvent`
- `PlanarBooleanIntervalEvent`
- new `PlanarBooleanEventLedger`
- new `PlanarBooleanEventLedgerReceipt`

**Required Query posture**
- required now:
  - one canonical retained artifact
  - typed binding / resolver surface for the next phase
  - inspection-ready receipt and counters
- support-gated:
  - cross-lane EMBER event ledger parity
- out:
  - split code recomputing relation truth from carriers

**Warnings**
- Ledger assembly is the authority boundary for event truth; do not expose
  partially assembled event vectors as equivalent public products.
- Canonical ordering must be testable and named, not incidental sort order.

**Test requirements**
- `event_ledger_orders_point_and_interval_events_canonically_across_replay`
- `event_ledger_groups_coincident_point_reports_without_losing_provenance`
- `event_ledger_rejects_missing_pair_enumeration_or_predicate_binding_receipts`

**Engineering decisions**
- The ledger receipt is the single `7.2` output artifact that `7.3` consumes.
- Event grouping belongs before ledger closeout because grouping changes the
  observable event product.

**Open questions**
- Whether no-event collinear relations are retained in the public ledger or
  only in diagnostics / decision trace.

### Phase 11: Degenerate Micro-Event Denial And Policy Posture

Freeze typed denial and policy posture for cases too degenerate to produce
safe event products in `7.2`.

**Relevant subsystems**
- `worth-spatial` planar boolean event platform
- `worth-spatial` user outcome / diagnostics
- `worth-kernel` workload composition

**Construction requirements**
- Define denial / policy variants for:
  - zero-length projected carrier
  - same-operand duplicate or stacked segment where unsupported
  - predicate ambiguity under admitted precision
  - near-coincident but not certified contact
  - interval collapse after normalization
  - missing topology provenance
  - mixed reduced-pair or frame identities
  - unsupported high-valence posture, if not admitted as event metadata
- Ensure every denial carries:
  - phase-local kind
  - reduced-pair identity
  - carrier or pair identity when available
  - predicate / precision basis when relevant
  - workload evidence-stage identity
- Construction target files:
  - `crates/worth-spatial/src/workload_platform/planar_boolean_events/denial.rs`
  - `crates/worth-spatial/src/workload_platform/planar_boolean_events/policy.rs`
  - `crates/worth-spatial/src/workload_platform/user_response/source_adapters/planar_boolean_outcome.rs`
  - `crates/worth-kernel/src/workload_composition/boolean_outcome/*`

**Relevant APIs**
- `WorthUserOutcome`
- `WorthUserOutcomeCause`
- `PlanarBooleanCommonPlaneReducedOperandPairRequest`
- new `PlanarBooleanEventExtractionDenial`
- new `PlanarBooleanEventExtractionPolicyExit`

**Required Query posture**
- required now:
  - ordinary outcomes and checked stops
  - typed support/admission posture
  - no prose-only denials
- support-gated:
  - future wider degenerate support families
- out:
  - "best effort" event ledgers with warning-only degeneracies

**Warnings**
- Degenerate events must fail before ledger construction if they make split
  truth unsafe.
- Do not use `Allowed Debt` to hide ordinary denial taxonomy that can be built
  inside this milestone.

**Test requirements**
- `event_extraction_denies_zero_length_carrier_before_pair_enumeration`
- `event_extraction_denies_predicate_ambiguous_near_contact_without_event`
- `event_extraction_policy_exit_preserves_phase_and_pair_identity`

**Engineering decisions**
- Denial and policy posture are first-class event-extraction outputs.
- The implementation should prefer typed failure over low-confidence event
  construction.

**Open questions**
- Which same-operand duplicate cases are denied in `7.2` versus admitted as
  event metadata for later cleanup milestones.

### Phase 12: Summum Bonum Closeout And Anti-Theatre Certification

Freeze the full production-grade confidence test for `7.2`.

**Relevant subsystems**
- `worth-kernel` workload composition and public contracts
- `worth-spatial` planar boolean event platform and public contracts
- Query-backed workload / retained artifact rails

**Construction requirements**
- Add the summum bonum certification target:
  `planar_boolean_event_extraction_metaboss_ledger_is_complete_canonical_and_unforgeable`.
- The test must start from a real workload catalog recipe and pass through the
  `7.0` entry and `7.1` reduced-pair path.
- The hostile scene must include, in one real workload-backed pair:
  - proper crossing
  - endpoint / interior contact
  - shared endpoint
  - collinear disjoint pair
  - collinear endpoint touch
  - partial overlap
  - containment overlap
  - identical same-direction coincidence
  - identical anti-parallel coincidence
  - duplicate point reports from closure or adjacency
  - at least one typed degenerate or policy-denied micro-event case
- Verify exact ledger shape:
  - event counts
  - event kinds
  - segment-pair breadth
  - point and interval identities
  - source operand / loop / edge provenance
  - normalized coordinates and parameters
  - duplicate suppression counters
  - denied / policy-exit counters
  - replay identity
- Add compile-fail and public-contract fences rejecting:
  - synthetic event-ledger construction
  - hand-built event rows
  - raw segment-pair fixtures
  - mismatched reduced-pair receipts
  - split-stage consumption without the event-ledger receipt
- Construction target files:
  - `crates/worth-kernel/src/certification/public_facade_contracts/contracts/public_api_planar_boolean_event_extraction*.rs`
  - `crates/worth-kernel/src/certification/public_facade_contracts/compile_fail/pb_events/*`
  - `crates/worth-spatial/src/certification/public_facade_contracts/contracts/planar_boolean_events/*`
  - `crates/worth-spatial/src/certification/public_facade_contracts/compile_fail/planar_boolean_events/*`
  - `crates/worth-kernel/src/workload_composition/workload_catalog/*`

**Relevant APIs**
- `PlanarBooleanEventLedgerReceipt`
- `PlanarBooleanEventExtractionRequest`
- `PlanarBooleanSegmentPairEnumerationReceipt`
- `WorkloadEvidenceStage`
- `WorkloadStageRequirement`
- public facade compile-fail harnesses

**Required Query posture**
- required now:
  - real declaration / workload entry
  - retained artifact progression from `7.1` to `7.2`
  - canonical identity and support posture preservation
  - inspection-ready counters
- support-gated:
  - EMBER parity
- out:
  - proof that starts from segment coordinates instead of a workload recipe

**Warnings**
- The summum bonum test is not optional polish; it is the production confidence
  bar for the milestone.
- If the closeout test can pass while `7.3` recomputes segment relations, the
  milestone is closed incorrectly.

**Test requirements**
- `planar_boolean_event_extraction_metaboss_ledger_is_complete_canonical_and_unforgeable`
- `event_ledger_replay_and_orientation_variation_preserve_canonical_identity`
- `event_ledger_public_contract_rejects_synthetic_rows_and_raw_pair_fixtures`
- `edge_split_consumption_requires_event_ledger_receipt_not_raw_events`

**Engineering decisions**
- Closeout proof must combine event correctness, workload truth, replay
  identity, counter evidence, and anti-theatre fences.
- This phase is the handoff contract to `7.3`.

**Open questions**
- Final hostile recipe geometry should be chosen during implementation, but it
  must remain workload-catalog backed and must exercise every listed event
  family.

## Admitted Surface

- real `7.0`-admitted planar boolean workload entry
- real `7.1` certified common-plane reduced operand pairs
- line-segment boundary carriers derived from reduced B-rep planar operands
- cross-operand segment-pair enumeration for the admitted planar body-pair
  class
- point events for proper crossings, endpoint / interior contacts, and shared
  endpoints
- interval events for partial overlap, containment overlap, identical
  same-direction coincidence, and identical anti-parallel coincidence
- typed no-event or diagnostic relations for collinear disjoint pairs where
  retained
- typed denial or policy posture for unsupported degenerate micro-events

## Excluded Surface

- edge splitting
- loop reconstruction
- overlap-region island extraction
- fragment classification
- face assembly
- cleanup and topology legality of boolean results
- EMBER execution or B-rep / EMBER parity
- curved-edge or non-linear event extraction
- non-planar or mixed-surface event extraction

## Workflow Surface

- reduced-pair event extraction over workload-catalog-backed planar boolean
  operand pairs
- retained replay of the same event extraction request
- downstream binding from `7.2` event ledger to `7.3` split planning
- typed failure for unsupported event, carrier, predicate, or provenance cases

## Operator Closure

- event extraction request compilation
- segment-carrier extraction
- canonical segment identity and endpoint normalization
- segment-pair enumeration
- predicate / precision basis binding
- point-event extraction
- collinear relation classification
- interval-event extraction
- event grouping and ledger assembly
- event-ledger closeout certification

## Validator Closure

- reduced-pair request validators
- carrier provenance validators
- canonical segment and endpoint validators
- segment-pair completeness validators
- predicate / precision basis validators
- point-event and interval-event validators
- event grouping and duplicate-suppression validators
- event-ledger receipt validators
- anti-theatre public-boundary validators

## Workload Composition Additions

- Add event-extraction workload composition under
  `crates/worth-kernel/src/workload_composition/boolean_event_extraction/`.
- Add event-stage requirements to `WorkloadStageRequirement` only where the
  phase produces proof that later phases must consume.
- Add event-stage rows to `WorkloadEvidenceStage` so event extraction cannot be
  hidden behind generic projection or split evidence.
- Add workload catalog recipes that produce real event-hostile planar operand
  pairs through the existing topology, spatial, projection, transform, replay,
  diagnostics, and response rails.
- Add closeout contracts proving that synthetic event rows, raw segment pairs,
  and mismatched reduced-pair receipts cannot satisfy event extraction.

## Replay Closure

- Replaying the same event extraction request must preserve:
  - segment-carrier identities
  - segment-pair enumeration order
  - predicate-binding identity
  - point-event identities
  - interval-event identities
  - event grouping
  - ledger digest
  - counters
  - denial / policy posture

## Diagnostics Closure

- Denials must localize whether failure occurred at:
  - event extraction request admission
  - segment-carrier extraction
  - segment identity / endpoint normalization
  - pair enumeration
  - predicate / precision binding
  - point-event classification
  - collinear relation classification
  - interval extraction
  - event grouping
  - ledger assembly
  - workload evidence or public-boundary certification

## Determinism Closure

- event extraction request identity
- segment-carrier identity
- canonical segment identity
- segment-pair ordering
- predicate-binding identity
- point-event ordering
- interval-event ordering
- grouped-event identity
- ledger digest
- replay counters

## Complexity / Proof Closure

- Segment-pair breadth must be explicit and counter-tested.
- Predicate decisions must be consumed from the bound predicate authority rather
  than rediscovered by local math.
- Event grouping must be deterministic and bounded by emitted event breadth.
- Ledger assembly must be the single canonical event authority for later split
  work.
- Diagnostic richness must not change operational event identity.

## Allowed Debt

- Spatial acceleration structures may remain deferred if the admitted class uses
  a counter-tested brute-force pair enumeration and the cost boundary is named.
- Wider support for curved or non-linear carriers remains deferred to later
  non-planar / EMBER work.
- Same-operand duplicate or stacked segment support may remain fail-closed if
  the denial is typed, localized, and covered by hostile tests.
- Overlap-region island semantics are deferred to `7.5`; interval events may
  carry source-sense facts without constructing overlap regions.

## Milestone Done When

- every admitted `7.1` reduced operand pair enters one canonical event
  extraction request boundary
- every emitted event is a typed point event, interval event, no-event relation,
  or typed denial / policy posture
- segment carriers preserve source topology, projection, frame, and precision
  provenance
- segment-pair enumeration is complete, deterministic, and counter-bearing
- the event ledger is replay-stable and orientation-stable
- workload composition proves event tests start from real catalog recipes
- public-contract and compile-fail fences block synthetic event proof
- `7.3` can consume `PlanarBooleanEventLedgerReceipt` without recomputing raw
  segment relations

## Acceptance Evidence

- `cargo check -p worth-spatial -p worth-kernel`
- focused public-contract tests for the event extraction request, carriers,
  pair enumeration, point events, interval events, event grouping, and ledger
  receipt
- hostile workload-catalog tests proving real event-family recipes are
  workload-backed
- compile-fail proof that event ledger rows, event receipts, and event-stage
  evidence cannot be forged from raw coordinates or synthetic segment pairs
- replay proof that the same reduced pair produces the same event ledger
- orientation and operand-order variation proof where semantics permit
- the summum bonum test:
  `planar_boolean_event_extraction_metaboss_ledger_is_complete_canonical_and_unforgeable`

## Sequencing Notes

- Do not start `7.3` edge splitting until `7.2` closes with a ledger receipt
  that split work can consume.
- Do not put overlap-region extraction into `7.2`; interval events are the
  substrate, not the region product.
- Do not widen into EMBER here.
- If a Query-owned support or retained-artifact seam is missing, extend the
  Query-shaped path or support posture rather than building a local substitute.
- If event extraction needs additional workload catalog recipes, add them here;
  do not write geometry-only fixtures and call them production proof.

## Required Self-Check

- Does the milestone solve a real structural problem or just package work
  cosmetically? Yes: it freezes the event truth authority that all later split
  and classify work depends on.
- Is the adversarial constraint precise and load-bearing? Yes: it requires a
  complete, canonical, replay-identical, unforgeable event ledger from real
  workload rails.
- Does the roadmap justify this milestone now? Yes: `7.3` edge splitting cannot
  honestly begin until typed event products exist.
- Does the spec preserve crate authority boundaries? Yes: Query owns runtime
  entry, `worth-kernel` owns workload composition, `worth-spatial` owns event
  semantics, and `worth-topo` owns topology truth.
- Are the phases carrying most of the real design information? Yes.
- Is each phase centered on one conceptual detail or boundary? Yes.
- Does each phase contain at least 2 adversarial tests by default? Yes.
- Could a competent engineer map this spec into honest types, modules, and
  tests? Yes.
- Does the milestone belong in this roadmap sequence, or is it out of order?
  It belongs immediately after `7.1` because event extraction consumes the
  reduced pair and produces the input authority for `7.3`.
