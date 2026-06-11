# Worth Kernel Milestone 6: Exact Planar Contracts, Structural Identity, And Boolean-Readiness

> **Status:** Draft
>
> **Purpose:** freeze exact planar predicate authority, planar structural
> identity, retained/projection-consumed planar facts, movement/rotation posture,
> and clean-fail diagnostics as the boolean-readiness substrate for Milestone 7.

## Goal

Freeze one coherent Milestone 6 substrate in which:

- `worth-math` owns the certified predicate machinery: Shewchuk adaptive
  predicates, certified tri-signs, precision modes, escalation metadata, and
  exact-rational budget tracking
- `worth-spatial` owns exact planar predicate authority over Worth geometry by
  routing through `worth-math`, recording the spatial tolerance/basis context,
  planar structural identity, planar clean-fail taxonomy, movement/rotation
  posture, projection-consumed planar facts, recovery posture, and
  spatial-side certification truth
- `worth-topo` owns topology truth, topology legality, topology-local
  neighborhoods, topology query surfaces, and topology-to-spatial contract
  completeness consumed by planar workflows
- `worth-kernel` owns boolean-readiness workflow composition and closeout
  certification without becoming a second planar predicate runtime, identity
  runtime, retained-history runtime, or Query replacement
- Forge Query remains the ordinary public runtime layer for declaration,
  readiness, route, receipt, envelope, retained artifact, projection
  consumption, recovery, inspection, signal/continuation, and support posture

Milestone 6 does not implement boolean split/classify/assemble. It prepares the
planar facts that Milestone 7 booleans may consume without inventing predicate
or identity shortcuts.

## Why This Milestone Exists

Milestone 5 closed the binding/rebinding and Query-native geometry hard break on
current evidence. That means Milestone 6 can no longer be a narrow planar
predicate add-on. It must prove that planar exactness participates in the same
runtime story as binding identity, retained geometry, projection consumption,
recovery, branch/historical inspection, and movement/rotation posture.

Without this milestone:

- booleans would have to rediscover exact planar classification locally
- structural identity would collapse into topology ids, names, binding identity,
  or kernel workflow summaries
- retained and projection-consumed planar facts could drift from live
  classification
- movement and rotation could be inferred from final coordinates instead of
  retained as explicit semantic posture
- clean-fail behavior would arrive too late, after boolean split/classify work
  has already tried to consume impossible planar input

## Governing Summaries

- `MENTALITY.md`: protects adversarial, foundation-first engineering. The spec
  must solve exact planar truth and replay/retention pressure before boolean
  features depend on it.
- `arch_laws.md`: protects authority separation and proof-carrying pipelines.
  The spec must make planar admission, classification, identity, retained
  facts, projection consumption, recovery, and certification distinct typed
  transitions.
- `composition_laws.md`: protects semantic file and function ownership. The
  spec must not encourage planar mega-helpers or certification bags where
  predicate authority, identity, recovery, diagnostics, and tests blur.
- `domain_structure_laws.md`: protects tree topology as ownership proof. The
  spec must preserve `worth-spatial` as planar semantic owner, `worth-topo` as
  topology owner, and `worth-kernel` as composition/certification owner.
- `perf_laws.md`: protects visible cost and bounded breadth. The spec must name
  counters for precision escalation, identity lookup, retained basis,
  projection consumption, movement/rotation posture, and clean-fail
  localization.
- `forge-query/docs/AI_README.md`: protects the Query rule "declare intent once,
  lower it once, execute or inspect it through canonical runtime-owned
  artifacts." The spec must use Query for ordinary domain entry, support
  posture, declaration identity, readiness, routing, receipts/envelopes,
  retained artifacts, inspection, projection consumption, recovery, signal and
  continuation, instead of creating local pseudo-Query paths.
- `_docs/worth/worth_roadmap.md`: protects M6 as the bridge between
  binding/rebinding and boolean pipelines. The spec must prove exact planar and
  identity substrate now because M7 needs trustworthy split/classify input.
- `_docs/worth/m6-premetaboss.md`: protects the hostile proof bar. The spec must
  require the `MB-M6-*` suites before M6 can close.
- `_docs/worth-kernel/miletsone-5.md`: protects the predecessor substrate. M6
  must consume M5 binding/rebinding, retained inspection, replay, recovery,
  projection consumption, and Query-native geometry surfaces rather than
  restating or bypassing them.

## Existing Surface Inventory

Milestone 6 must reuse these existing surfaces before adding new ones:

- `worth_spatial::facade::binding`: primitive binding Query domain, declaration
  entry, mutation evidence, projection facts, target identity, prior binding
  facts, and candidate facts
- `worth_spatial::facade::rebinding`: primitive rebinding authoring and outcome
  surfaces
- `worth_spatial::facade::neighborhood`: grouped local replacement,
  contribution workflow, and topology-neighborhood replacement facts
- `worth_spatial::facade::inspection`: retained geometry subject, historical
  inspection, branch-local inspection, retained view payload, and replay parity
- `worth_spatial::facade::projection`: receipt-backed geometry projection
  consumption
- `worth_spatial::facade::recovery`: typed geometry recovery actions and
  recovery fact receipts
- `worth_spatial::facade::continuation`: signal compatibility and bridge
  continuation execution
- `worth_spatial::facade::support`: geometry applicability matrix and public
  surface inventory
- `worth_spatial::facade::tolerance`: tolerance and precision certification
  Query domain and fact receipts
- `worth_math::predicates`: certified Shewchuk predicate wrappers for
  `orient2d`, `orient3d`, `incircle`, and `in_sphere`
- `worth_math::arithmetic::precision`: `PrecisionMode`,
  `PrecisionEscalation`, and `PrecisionBudget`
- `worth_math::sign`: `CertifiedTriSign` and `TriSign`
- `worth_topo::construction::query_native_boundary`: topology construction
  Query receipt, envelope, handoff, admitted handoff, inspection surface,
  read surface, and fact rows
- `worth_topo::projection::runtime_boundary::declared_query_surfaces`:
  topology live views, derived materialization, diagnostics, validation, and
  equivalence contract surfaces
- `worth-kernel` binding closeout tests, especially
  `geometry_hard_break_closeout`, as proof that the predecessor Query-native
  geometry story is closed on current evidence

New M6 surfaces are allowed only where this inventory cannot honestly express
planar-specific predicate authority, structural identity, retained planar facts,
movement/rotation posture, or boolean-readiness certification. M6 must not add a
second predicate engine beside `worth-math`.

## Adversarial Constraint

Under coplanar storms, high-valence planar degeneracy, thin-feature scale
separation, dirty topology, unbounded/open planar domains, local planar
rebuilds, topology replacement, movement, rotation, retained-history replay,
projection consumption, recovery, signal/continuation, and boolean-readiness
certification pressure, the same semantic planar basis must produce the same
exact planar classification, the same structural identity, the same retained
planar fact, the same projection-consumed fact, the same clean-fail posture, the
same movement/rotation posture, and the same certification artifact unless the
input is intentionally semantically different.

If any supported path:

- infers planar truth from topology ids, names, binding identity, final
  coordinates, or kernel summaries
- performs snapping, closest-plane repair, bounded conversion, or topology-only
  reconstruction before typed planar denial
- lets movement/rotation order, host order, authoring order, retained basis
  order, or projection-consumption order perturb equivalent meaning
- lets retained or projection-consumed planar facts disagree with live
  classification
- allows Query-visible-but-unsupported surfaces to masquerade as admitted
  runtime-backed support
- or lets M7 consume boolean input without a complete boolean-readiness contract

then this milestone has failed.

## Product Decision Lock

- M6 is a Query-native milestone. Every phase must name where it uses Query and
  whether that surface is required now, support-gated, or explicitly out.
- Query declaration, readiness, progression, route, receipt, envelope, ordinary
  outcome, retained artifact, projection consumption, recovery, inspection,
  signal/continuation, support posture, and capability matrix are all ordinary
  M6 design tools.
- New planar-specific domains should follow the existing `worth-spatial`
  Query-native family pattern. They must not be kernel-owned.
- Movement and rotation are not fixture details. They are semantic posture and
  must be carried through declaration identity, retained facts, projection
  consumption, recovery, and certification where relevant.
- M6 may classify unbounded/open planar classes, dirty planar input, and extreme
  degeneracy as unsupported or policy-required. It may not crash, hang, coerce,
  silently repair, or allow later boolean code to consume unclear input.
- All `MB-M6-*` tests in `_docs/worth/m6-premetaboss.md` are closeout
  requirements. `MB-M6-7` and `MB-M6-8` are mandatory final closeout gates.
- `MB-M6-*` tests must be operational end-to-end proofs. They must assemble
  real topology truth, spatial binding/projection facts, retained Query
  artifacts, movement/rotation posture, and production user-response surfaces
  through the ordinary public runtime. A test that hand-builds only spatial
  rectangles, re-extracts the same inputs as "replay," changes identity strings
  instead of transforming geometry, or asserts fixture-generator arithmetic is
  not an MB closeout proof.
- M6 must not implement M7 boolean split/classify/assemble, EMBER, or B-rep
  boolean execution.

## Status Honesty Rule

- A phase is not done because a representative planar fixture passes.
- A phase is not done because a visible Query surface exists.
- A phase is not done because kernel can assemble a convincing summary.
- An MB phase is not done because a production spatial extractor was called
  from a synthetic harness. The proof must also include the real upstream
  topology, geometry-binding, projection, retained/replay, movement/rotation,
  and user-response path required by that workload.
- A phase is done only when production surfaces exist, support posture is
  explicit, and the phase's adversarial proofs pass.
- If a required Query lane is missing, harden Query or mark the specific support
  row as blocked. Do not build a local Worth substitute.

## Product Response And MB Harness Lock

Before any MB closeout test may count, M6 must operationalize two product
surfaces that the MB1 audit exposed as missing or too narrow:

- a shared planar user-response layer that can express admitted,
  policy-required, unsupported, denied, predicate-uncertain, integrity-mismatch,
  and no-options outcomes with typed causes, selectable policy decisions only
  when safe, and human-readable explanations
- a real MB operational harness that starts from production topology and
  geometry setup, binds spatial facts through the ordinary Query-native path,
  applies actual movement/rotation to geometry rather than identity strings,
  consumes retained artifacts for replay rather than re-running the same
  extractor, and records Query-visible receipts/counters for every step

The overlap-specific response surface created for `MB-M6-1` is acceptable only
as the first concrete branch of this broader product layer. If later MB phases
need the same response taxonomy, the broader planar response layer must be
built and the overlap surface must either wrap it or be refactored into it.

`MB-M6-1` must be refactored before it can remain a closeout gate. Its storm may
still use generated hostile scale, but the closeout proof must no longer rely on
procedural rectangles alone, synthetic world placement alone, fake retained
replay, motion labels standing in for transformed geometry, or self-referential
fixture-count assertions.

## Phase Plan

### Phase 1: Freeze Planar Admission Vocabulary

Phase 1 defines the exact vocabulary for what the planar layer may admit, deny,
mark unsupported, or require policy for. It closes the "maybe this is planar
enough" loophole before predicate or identity work begins.

**Relevant subsystems**
- `worth-spatial` planar support posture
- `worth-spatial` geometry applicability matrix
- Forge Query support matrix and admission posture
- `worth-kernel` certification inventory

**Relevant APIs**
- `worth_spatial::facade::support::geometry_applicability_matrix`
- `worth_spatial::facade::support::GeometryRuntimeConcern`
- `worth_spatial::facade::support::GeometryApplicabilityStatus`
- new `worth-spatial` planar support-posture family if existing inventory cannot
  represent planar admission classes

**Required Query posture**
- required now:
  - configured domain handles
  - support matrix and admission
  - canonical domain declarations
  - declaration family taxonomy
  - declaration family capability matrix
- support-gated:
  - broad public planar DX helpers
- out:
  - local kernel support enums

**Warnings**
- Do not infer admission from API visibility.
- Do not encode clean-fail classes as strings or diagnostics prose.
- Do not let unbounded/open planar classes silently enter admitted boolean
  readiness.

**Test requirements**
- `planar_admission_matrix_classifies_exact_ambiguous_unbounded_dirty_and_policy_required_classes`
- prove every M6 planar class has a typed support posture before predicate
  classification or retained facts can be emitted
- `visible_planar_surface_without_admission_fails_closed_before_kernel_summary`
- prove kernel cannot treat a visible but unsupported planar surface as admitted
  merely because a facade export exists
- `mb_m6_admission_rows_cover_premetaboss_input_families`
- prove `MB-M6-1` through `MB-M6-8` each maps to explicit admitted, denied,
  unsupported, or policy-required posture

**Engineering decisions**
- Admission posture is a first-class Query-backed runtime fact.
- M6 starts from support honesty, not predicates.
- Unbounded/open and dirty planar cases may be classified without being admitted
  as boolean-ready.

**Open questions**
- Exact public enum names for planar admission classes.

### Phase 2: Freeze Exact Planar Predicate Authority

Phase 2 makes exact planar classification a spatial authority surface instead
of a kernel-local helper or topology-derived assumption.

**Relevant subsystems**
- `worth-math` certified Shewchuk predicates and precision metadata
- `worth-spatial` planar predicate authority
- `worth-spatial` tolerance and precision certification
- `worth-topo` topology facts consumed by predicate classification
- Forge Query declaration progression

**Relevant APIs**
- `worth_math::predicates::{orient2d, orient3d, incircle, in_sphere}`
- `worth_math::arithmetic::precision::{PrecisionEscalation, PrecisionMode}`
- `worth_math::sign::CertifiedTriSign`
- existing `worth_spatial::facade::tolerance` certification surfaces
- new `worth-spatial` planar predicate Query domain and declaration family
- `worth_topo::projection::runtime_boundary::declared_query_surfaces`
- `worth_topo::construction::query_native_boundary` fact rows where topology
  construction facts are the input basis

**Required Query posture**
- required now:
  - canonical domain declarations
  - declaration aspect contracts
  - declaration legality
  - declaration progression
  - ordinary outcomes
  - lower-runtime route contract
  - mutation evidence where predicate facts are authored as truth-bearing facts
- support-gated:
  - grouped neighborhood workflow until Phase 19
  - projection consumption until Phase 16
- out:
  - predicate classification by kernel-local coordinate math

**Warnings**
- Do not compute predicate truth from topology labels or binding identity.
- Do not allow nearest-plane, snap, or epsilon fallback before typed predicate
  denial.
- Do not hide predicate uncertainty behind advisory diagnostics.

**Test requirements**
- `exact_planar_predicate_authority_converges_across_equivalent_authoring_order`
- prove equivalent planar declarations produce identical classification facts
  under host-order and declaration-order variation
- `exact_planar_predicate_authority_denies_near_graze_before_snap_or_repair`
- prove near-graze inputs fail typed before any repair or coercion can produce
  admitted facts
- `mb_m6_1_coplanar_overlap_contract_storm_predicate_rows`
- start the `MB-M6-1` proof by asserting classification stability and typed
  denial for coplanar overlap regions

**Engineering decisions**
- `worth-math` is the only certified predicate engine.
- `worth-spatial` owns planar predicate authority by adapting Worth geometry,
  topology basis, tolerance policy, movement/rotation posture, and Query
  declaration context into `worth-math` predicates and by retaining the resulting
  certified facts.
- Query declaration progression is the normal predicate entry path.
- Predicate uncertainty is a typed outcome class, not a warning.

**Open questions**
- Whether predicate facts should be stored under a new planar domain or a
  narrower family inside the existing binding domain.

### Phase 3: Freeze Precision Escalation And Tolerance Basis

Phase 3 freezes how exact planar work consumes `worth-math` precision
escalation and records the spatial tolerance basis, including scale separation
and local coordinate normalization. It does not invent a new escalation ladder.

**Relevant subsystems**
- `worth-math` `PrecisionEscalation`, `PrecisionMode`, and `PrecisionBudget`
- `worth-spatial` tolerance and precision certification
- `worth-spatial` planar predicate authority
- `worth-kernel` pre-MetaBoss scale fixtures
- Forge Query capability posture for certification facts

**Relevant APIs**
- `worth_math::arithmetic::precision::{PrecisionEscalation, PrecisionMode, PrecisionBudget}`
- `worth_math::predicates::{orient2d, orient3d, incircle, in_sphere}`
- `worth_spatial::facade::tolerance::*`
- new planar precision-escalation fact family if tolerance certification is too
  construction-specific
- `worth_primitives::truth_digest_parts`

**Required Query posture**
- required now:
  - canonical declarations
  - declaration legality
  - declaration route contract
  - ordinary outcomes
  - retained artifact basis for certified precision facts
  - inspection for precision certificates
- support-gated:
  - recovery for precision escalation synthesis
- out:
  - ambient global epsilon settings

**Warnings**
- Do not use global coordinate magnitude as the precision basis for
  micro-features.
- Do not reimplement Shewchuk predicates, expansion arithmetic, certified
  signs, or exact-rational budget tracking outside `worth-math`.
- Do not let precision escalation mutate predicate meaning after a fact is
  retained.
- Do not make cost invisible; escalation breadth must be counted.

**Test requirements**
- `planar_precision_escalation_uses_local_feature_scale_not_world_magnitude`
- prove 21-order scale separation records local coordinate basis and escalates
  from local feature scale
- `planar_precision_escalation_denies_when_required_basis_is_missing`
- prove missing, ambiguous, or contradictory tolerance basis denies before
  predicate facts are emitted
- `mb_m6_3_thin_feature_scale_separation_contract`
- satisfy the scale-separation assertions from `MB-M6-3`

**Engineering decisions**
- Shewchuk predicate resolution and raw precision metadata come from
  `worth-math`.
- Precision basis is part of planar fact identity.
- Spatial certificates must pair `worth-math` metadata with local feature-scale
  basis, movement/rotation posture, tolerance policy, and Query declaration
  identity.
- Counters must expose escalation breadth, local-coordinate normalization, and
  retained consumption of `worth-math` precision metadata.
- Recovery may suggest next steps but may not synthesize missing precision truth.

**Open questions**
- Whether local coordinate normalization lives as its own typed certificate or
  as part of the planar predicate fact.

### Phase 4: Freeze Planar Local Frame Certificates

Phase 4 makes the local planar frame a proof-bearing artifact rather than an
implicit coordinate convenience. Massive-scale planar work must carry the
origin, normal, `u/v` axes, feature-scale basis, transform-chain digest,
movement/rotation posture, tolerance policy, and Query declaration identity that
made later predicate calls meaningful.

**Relevant subsystems**
- `worth-math` finite coordinates, unit vectors, and linalg primitives
- `worth-spatial` tolerance, movement/rotation, and planar predicate authority
- Forge Query retained artifact and inspection surfaces

**Relevant APIs**
- `worth_math::numeric::{FinitePoint3, FiniteVector3, UnitVector3}`
- `worth_math::linalg::*`
- `worth_spatial::facade::tolerance::*`
- new `PlanarLocalFrameCertificate` Query-backed spatial fact family

**Required Query posture**
- required now:
  - canonical declarations
  - declaration aspect contracts
  - ordinary outcomes
  - retained artifact basis
  - inspection for local-frame certificates
- support-gated:
  - recovery suggestions for missing frame inputs
- out:
  - ambient global coordinate frame as predicate basis

**Warnings**
- Do not let world-coordinate magnitude define planar precision meaning.
- Do not allow local frames to be recomputed silently in projection consumers.
- Do not hide transform-chain or movement/rotation posture outside the
  certificate identity.

**Test requirements**
- `planar_local_frame_certificate_is_stable_under_equivalent_translation_and_rotation`
- prove equivalent move/rotate authoring orders produce the same local-frame
  certificate digest
- `planar_local_frame_certificate_changes_when_semantic_rotation_exits_planar_class`
- prove tiny inadmissible rotation yields typed denial or a distinct posture
  rather than coordinate coincidence
- `mb_m6_3_local_frame_basis_survives_scale_separation`
- satisfy the local-basis assertions from `MB-M6-3`

**Engineering decisions**
- Local frame is a retained spatial certificate, not a temporary math helper.
- Later planar facts consume the certificate by identity rather than rebuilding
  it from raw coordinates.
- Counters must expose local-frame derivations and retained frame consumption.

**Open questions**
- Exact public type name for the certificate family.

### Phase 5: Freeze Certified Plane-To-2D Projection

Phase 5 freezes projection into the certified local planar frame. The operation
is not closest-point repair; it admits only points whose declared basis can be
projected under the local-frame certificate and fails typed when the point/plane
relationship is not admissible.

**Relevant subsystems**
- `worth-math` coordinate finite-ness and vector arithmetic
- `worth-spatial` planar local-frame certificates
- `worth-spatial` planar admission and clean-fail taxonomy
- Forge Query declaration progression and ordinary outcomes

**Relevant APIs**
- `PlanarLocalFrameCertificate`
- new `ProjectPointToCertifiedPlane2D` spatial operator family
- `worth_math::numeric::{FinitePoint2, FinitePoint3}`
- `worth_spatial::facade::tolerance::*`

**Required Query posture**
- required now:
  - canonical declarations
  - declaration legality
  - ordinary outcomes
  - mutation evidence for projection facts
  - inspection for projection-denial basis
- support-gated:
  - projection-consumed planar facts until retained planar phases
- out:
  - snap-to-plane and nearest-plane repair

**Warnings**
- Do not project by repair, snapping, or "close enough" heuristics.
- Do not emit 2D coordinates without the local-frame certificate digest.
- Do not conflate projection denial with predicate uncertainty.

**Test requirements**
- `certified_plane_projection_replays_to_identical_2d_coordinates_and_basis_digest`
- prove retained projection facts reproduce the same 2D basis under replay
- `certified_plane_projection_denies_off_plane_or_missing_basis_before_predicates`
- prove inadmissible point/plane relationships fail before `orient2d` or
  winding facts are emitted
- `mb_m6_1_projection_basis_survives_coplanar_overlap_storm`
- assert projection facts remain stable across the coplanar overlap workload

**Engineering decisions**
- Projection into 2D is a certified spatial boundary.
- Projected coordinates are not authoritative truth; the certificate plus source
  geometry basis is the authority for their use.
- Denials remain typed and inspectable through Query.

**Open questions**
- Whether projected coordinate payloads live with predicate facts or in a
  separate reusable certificate row.

### Phase 6: Freeze Certified Segment-Segment 2D Classification

Phase 6 freezes a reusable 2D segment classification contract for admitted
planar workloads. It classifies disjoint, proper crossing, endpoint touch,
collinear disjoint, collinear overlap, identical, reverse-identical, and
policy-required/uncertain cases using `worth-math` certified predicates.

**Relevant subsystems**
- `worth-math` `orient2d`, `CertifiedTriSign`, and precision metadata
- `worth-spatial` certified projection and planar predicate authority
- Forge Query retained artifact, inspection, and diagnostics surfaces

**Relevant APIs**
- `worth_math::predicates::orient2d`
- `worth_math::arithmetic::precision::PrecisionEscalation`
- new `CertifiedSegmentSegment2D` spatial operator family
- `ProjectPointToCertifiedPlane2D`

**Required Query posture**
- required now:
  - declaration aspect contracts
  - declaration legality
  - ordinary outcomes
  - retained artifact basis for segment certificates
  - inspection for classification rows
- support-gated:
  - coplanar overlap extraction until Phase 9
- out:
  - boolean splitting or imprint execution

**Warnings**
- Do not return raw bools for segment relationships.
- Do not decide collinear overlap with epsilon interval tests.
- Do not merge or split topology in this phase.

**Test requirements**
- `certified_segment_segment_2d_classifies_all_contact_classes_deterministically`
- prove every admitted contact class emits a stable typed classification and
  consumed `worth-math` predicate metadata
- `certified_segment_segment_2d_denies_policy_required_collinear_ambiguity_without_imprint`
- prove ambiguous collinear cases do not sneak into topology mutation
- `mb_m6_1_segment_contact_rows_survive_overlap_storm`
- satisfy coplanar overlap segment-contact assertions from `MB-M6-1`

**Engineering decisions**
- Segment classification is a certificate producer, not a topology editor.
- The classification basis includes local frame, projected endpoints, predicate
  signs, precision metadata, and tolerance policy.
- Counters must expose segment pairs evaluated and predicate escalations used.

**Open questions**
- Exact taxonomy names for identical versus reverse-identical segment classes.

### Phase 7: Freeze Certified Polygon Winding And Loop Containment

Phase 7 freezes certified winding and loop containment for admitted projected
planar loops. This supports holes, nested loops, figure-eight denial, and loop
containment without requiring boolean code to rediscover winding rules.

**Relevant subsystems**
- `worth-math` `orient2d` and certified tri-signs
- `worth-spatial` projected planar facts and segment classifications
- `worth-topo` loop membership and topology-to-spatial contract completeness
- Forge Query inspection and retained artifact surfaces

**Relevant APIs**
- new `CertifiedPolygonWinding2D` spatial operator family
- `CertifiedSegmentSegment2D`
- `worth_topo::construction::query_native_boundary` loop fact rows
- `worth_topo::projection::runtime_boundary::declared_query_surfaces`

**Required Query posture**
- required now:
  - canonical declarations
  - declaration progression
  - ordinary outcomes
  - retained winding facts
  - topology-to-spatial consumed fact rows
- support-gated:
  - overlap island extraction until Phase 9
- out:
  - keep/discard boolean classification

**Warnings**
- Do not infer containment from topology loop labels alone.
- Do not accept self-intersecting or figure-eight loops unless explicitly
  admitted by policy.
- Do not let host iteration order pick tie-breakers.

**Test requirements**
- `certified_polygon_winding_is_stable_under_loop_rotation_reversal_and_authoring_order`
- prove equivalent loop authoring variations produce stable winding and
  containment facts
- `certified_polygon_winding_denies_self_intersection_and_ambiguous_touch`
- prove figure-eight, duplicate-vertex, and ambiguous touch cases fail typed
  before retained planar facts are emitted
- `mb_m6_1_nested_hole_winding_rows_are_retained_and_replayable`
- satisfy nested-hole and figure-eight assertions from `MB-M6-1`

**Engineering decisions**
- Winding is spatially certified from projected geometry and topology loop
  basis together.
- Loop containment facts are retained, inspectable, and separate from topology
  identity.
- Counters must expose loop edges walked, segment contacts classified, and
  winding tie-breaks used.

**Open questions**
- Whether winding facts use one family for point-in-loop and loop-in-loop or
  two narrower certificate families.

### Phase 8: Freeze Certified Signed Area And Degeneracy Basis

Phase 8 freezes scale-safe signed area and degeneracy classification for
admitted planar loops/faces. It supplies zero-area, sliver, needle, tiny-hole,
and policy-required classifications without forcing topology or booleans to
make geometry guesses.

**Relevant subsystems**
- `worth-math` interval, rational, precision-budget, and finite coordinate
  primitives
- `worth-spatial` local-frame and projection certificates
- `worth-topo` degeneracy and loop/facial legality surfaces
- Forge Query diagnostics and support posture

**Relevant APIs**
- new `CertifiedSignedArea2D` spatial operator family
- `worth_math::arithmetic::{Interval, Rational, PrecisionBudget}`
- `worth_math::arithmetic::precision::PrecisionEscalation`
- `PlanarLocalFrameCertificate`

**Required Query posture**
- required now:
  - support posture
  - declaration legality
  - ordinary outcomes
  - retained area/dependency basis
  - inspection for degeneracy classification
- support-gated:
  - recovery candidates for dirty input until recovery phase
- out:
  - topology collapse, heal, or repair mutation

**Warnings**
- Do not classify area from raw world-coordinate shoelace sums at massive
  scale.
- Do not collapse degeneracy detection into topology policy.
- Do not silently tighten or relax tolerances to force admission.

**Test requirements**
- `certified_signed_area_uses_local_frame_scale_for_1e12_world_1e_minus_9_feature`
- prove scale-separated loops classify from local basis and record precision
  cost
- `certified_signed_area_denies_zero_sliver_and_needle_cases_with_localized_cause`
- prove dirty degeneracy cases fail with exact local loop/edge cause
- `mb_m6_3_signed_area_and_degeneracy_survive_thin_feature_pressure`
- satisfy signed-area and micro-feature assertions from `MB-M6-3`

**Engineering decisions**
- Area and degeneracy classifications are planar certificates, not repair
  actions.
- Degeneracy policy remains explicit and Query-visible.
- Counters must expose area terms evaluated, precision escalations, and
  degeneracy-localization breadth.

**Open questions**
- Exact admitted area classes for tiny holes versus policy-required dirty loops.

### Phase 9: Freeze Coplanar Overlap Contract Extraction

Phase 9 freezes coplanar overlap contract extraction without performing boolean
imprinting. It emits overlap islands, shared intervals, containment relations,
ambiguous tangent/contact classes, and policy-required exits as contract facts
that M7 can consume.

**Relevant subsystems**
- `worth-spatial` segment, winding, signed-area, and planar structural
  certificate families
- `worth-topo` loop membership and topology-to-spatial contract completeness
- Forge Query retained, inspection, diagnostics, and projection-consumption
  surfaces

**Relevant APIs**
- new `CoplanarOverlapContractExtractor` spatial operator family
- `CertifiedSegmentSegment2D`
- `CertifiedPolygonWinding2D`
- `CertifiedSignedArea2D`
- `worth_spatial::facade::neighborhood`

**Required Query posture**
- required now:
  - canonical declarations
  - declaration legality
  - ordinary outcomes
  - retained overlap contract facts
  - inspection for overlap rows
  - support matrix and admission posture
- support-gated:
  - boolean split/classify/assemble to M7
- out:
  - imprint topology mutation
  - keep/discard classification

**Warnings**
- Do not split faces or edges in M6 overlap extraction.
- Do not treat overlap extraction as a successful boolean.
- Do not hide ambiguous coplanar intent behind deterministic-looking output.

**Test requirements**
- `coplanar_overlap_contract_extractor_emits_stable_islands_intervals_and_containment`
- prove equivalent authoring, host, movement, and rotation order produce the
  same overlap contract bundle
- `coplanar_overlap_contract_extractor_denies_ambiguous_or_policy_required_cases_before_imprint`
- prove ambiguous contact classes fail typed before topology mutation can occur
- `mb_m6_1_coplanar_overlap_contract_storm_complete_contract_bundle`
- satisfy the full contract-extraction assertions from `MB-M6-1`

**Engineering decisions**
- Coplanar overlap contracts are boolean-readiness inputs, not boolean results.
- Extracted overlap facts must name their consumed segment, winding, area,
  local-frame, and projection certificates.
- Counters must expose candidate pair breadth, overlap islands, shared
  intervals, and policy-required exits.

**Open questions**
- Whether overlap islands use a standalone public facade or remain inside the
  planar predicate facade.

### Phase 10: Freeze Planar Contract Bundle Validation

Phase 10 freezes the validator that proves a planar workload has a complete M6
contract bundle. The validator checks that admission, topology basis, local
frame, projection, certified predicates, structural identity, retained fact,
projection-consumed fact, movement/rotation posture, diagnostics, and counters
are all present and mutually consistent.

**Relevant subsystems**
- `worth-spatial` planar contract certificate families
- `worth-topo` topology-to-spatial contract surfaces
- `worth-kernel` boolean-readiness certification
- Forge Query inspection and retained artifact surfaces

**Relevant APIs**
- new `PlanarContractBundleValidator` certification family
- `worth_spatial::facade::{binding, inspection, projection, support, tolerance}`
- `worth_topo::projection::runtime_boundary::declared_query_surfaces`
- `worth_topo::construction::query_native_boundary`

**Required Query posture**
- required now:
  - inspection
  - retained artifacts
  - projection consumption
  - support posture
  - ordinary outcomes
  - certification receipts
- support-gated:
  - M7 boolean execution consumption
- out:
  - rebuilding missing facts inside the validator

**Warnings**
- Do not let the bundle validator synthesize missing planar truth.
- Do not allow a partial bundle to pass because later boolean code could
  recompute the missing piece.
- Do not collapse topology, spatial, and kernel certification rows.

**Test requirements**
- `planar_contract_bundle_validator_accepts_complete_retained_and_projection_consumed_bundle`
- prove a complete bundle passes with all certificate families consumed by
  identity
- `planar_contract_bundle_validator_rejects_missing_or_mismatched_certificate_family`
- prove every missing, stale, mismatched, or wrong-posture family fails with a
  localized typed reason
- `mb_m6_8_boolean_readiness_final_boss_requires_complete_contract_bundle`
- satisfy complete-bundle assertions from `MB-M6-8`

**Engineering decisions**
- Bundle validation is a certification step, not a computation step.
- Missing facts must route back to their producing phase rather than being
  recreated locally.
- Counters must expose inspected bundle rows and rejected missing-family rows.

**Open questions**
- Exact kernel closeout facade name for bundle validation.

### Phase 11: Freeze Predicate Certificate Consumption Validation

Phase 11 freezes the firewall proving every M6 planar classification consumed
`worth-math` certified signs and precision metadata. It blocks local epsilon,
topology-label, binding-identity, or kernel-summary substitutes.

**Relevant subsystems**
- `worth-math` certified predicates and precision metadata
- `worth-spatial` planar predicate authority and retained facts
- `worth-kernel` certification and legacy-deletion proof
- Forge Query inspection and receipt surfaces

**Relevant APIs**
- new `PredicateCertificateConsumptionValidator` certification family
- `worth_math::predicates::{orient2d, orient3d, incircle, in_sphere}`
- `worth_math::sign::CertifiedTriSign`
- `worth_math::arithmetic::precision::PrecisionEscalation`

**Required Query posture**
- required now:
  - inspection
  - retained artifacts
  - ordinary outcomes
  - certification receipts
  - declaration family capability matrix
- support-gated:
  - broader M7 boolean predicate consumers
- out:
  - local kernel or spatial substitute predicates

**Warnings**
- Do not accept predicate outcomes without consumed `worth-math` metadata.
- Do not let certified signs be copied into untyped summaries that lose
  precision basis.
- Do not permit topology-derived classifications to satisfy spatial predicate
  contracts.

**Test requirements**
- `predicate_certificate_consumption_validator_accepts_only_worth_math_certified_signs`
- prove valid planar rows carry `CertifiedTriSign`, `PrecisionEscalation`, and
  local spatial basis together
- `predicate_certificate_consumption_validator_rejects_epsilon_topology_and_kernel_summary_substitutes`
- prove each forbidden substitute fails closed with exact diagnostic class
- `mb_m6_7_projection_consumed_planar_fact_parity_requires_predicate_metadata`
- satisfy projection-consumption parity assertions from `MB-M6-7`

**Engineering decisions**
- This validator is the M6 no-second-predicate-engine firewall.
- Predicate certificate consumption is required before boolean-readiness bundle
  certification.
- Counters must expose certified predicate rows inspected and substitute rows
  rejected.

**Open questions**
- Whether the validator lives in `worth-spatial` certification or kernel
  closeout, with a spatial facade entry.

### Phase 12: Freeze Planar Structural Identity Basis

Phase 12 defines planar structural identity as a distinct semantic identity
family, separate from topology identity, naming identity, lineage identity,
binding identity, and final coordinates.

**Relevant subsystems**
- `worth-spatial` planar structural identity
- `worth-spatial` binding identity
- `worth-topo` topology identity and naming truth
- Forge Query canonical declaration identity

**Relevant APIs**
- existing binding target identity surfaces in
  `worth_spatial::facade::binding`
- new planar structural identity and fingerprint family
- `ForgeQueryDeclarationCanonicalEntry`
- `worth_topo` topology and persistent-name query surfaces

**Required Query posture**
- required now:
  - canonical declaration entries
  - declaration family identity
  - declaration progression
  - declaration-entry inspection
  - retained artifact inspection
- support-gated:
  - structural correspondence and historical materialization until Phase 16
- out:
  - identity derived from final coordinates alone

**Warnings**
- Do not reuse binding identity as planar structural identity.
- Do not use topology ids or persistent names as the digest basis.
- Do not collapse equivalent movement/rotation cancellation into coordinate
  closeness; canonical transform basis must be present.

**Test requirements**
- `planar_structural_identity_diverges_from_topology_naming_binding_and_lineage`
- prove stable topology ids, names, lineage, and binding identity cannot force
  structural identity stability when planar meaning changes
- `planar_structural_identity_converges_for_canonical_transform_equivalence`
- prove move/rotate/reorient variants with the same canonical transform basis
  converge without depending on final coordinate coincidence
- `mb_m6_4_retained_planar_history_cancellation_identity_rows`
- start the `MB-M6-4` proof by asserting exact cancellation checkpoints produce
  identical structural identity

**Engineering decisions**
- Planar structural identity commits planar meaning plus canonical transform
  basis.
- Binding identity remains an input boundary, not the planar identity itself.
- Structural fingerprints are boolean-readiness facts, not display helpers.

**Open questions**
- Exact digest-field vocabulary for canonical transform basis.

### Phase 13: Freeze Movement And Rotation Posture

Phase 13 makes movement, rotation, reorientation, and cancellation posture typed
planar inputs that survive retained and projection-consumed paths.

**Relevant subsystems**
- `worth-spatial` motion posture
- `worth-spatial` planar structural identity
- `worth-kernel` movement/rotation workflow fixtures
- Forge Query declaration identity and continuation

**Relevant APIs**
- Milestone 5 motion-aware binding semantics
- new planar movement/rotation posture family if existing motion posture is
  binding-only
- `worth_spatial::facade::continuation::*`

**Required Query posture**
- required now:
  - canonical declarations
  - declaration readiness
  - declaration progression
  - ordinary outcomes
  - retained artifact to next step
  - signal compatibility
  - continuation pipeline
- support-gated:
  - bridge-mediated writeback if movement becomes a lower truth mutation
- out:
  - final-coordinate-only movement reconstruction

**Warnings**
- Do not infer movement/rotation from candidate availability.
- Do not let orientation reversal silently flip identity into success.
- Do not drop transform basis from retained facts.

**Test requirements**
- `planar_motion_posture_preserves_translation_rotation_reorientation_and_cancellation`
- prove movement and rotation posture is retained explicitly and exact
  cancellation replays bit-identically
- `planar_motion_posture_denies_orientation_flip_before_projection_consumption`
- prove orientation-changing or invalidating transforms deny before downstream
  projection facts can consume the planar basis
- `mb_m6_motion_rotation_stack_is_present_in_every_premetaboss_family`
- prove all `MB-M6-*` suites carry movement/rotation posture where applicable

**Engineering decisions**
- Movement/rotation posture is semantic input, not fixture metadata.
- Signal and continuation compatibility must be classified for planar transform
  workflows where a retained planar fact becomes the next step.
- Transform basis participates in identity, retained facts, projection
  consumption, and diagnostics.

**Open questions**
- Whether reorientation should be a subtype of rotation posture or a separate
  posture class.

### Phase 14: Freeze Topology-To-Spatial Contract Completeness

Phase 14 proves topology facts are complete enough for exact planar work before
planar identity, retained facts, or boolean-readiness bundles can be emitted.

**Relevant subsystems**
- `worth-topo` topology truth and validation
- `worth-topo` declared query surfaces
- `worth-spatial` topology-to-spatial contract validator
- Forge Query projection consumption and inspection

**Relevant APIs**
- `worth_topo::construction::query_native_boundary::*`
- `worth_topo::projection::runtime_boundary::declared_query_surfaces`
- `TopologyConstructionQueryFactRow`
- new planar topology-contract completeness validator in `worth-spatial`

**Required Query posture**
- required now:
  - read composition
  - topology live views
  - topology derived materialization
  - topology validation surfaces
  - inspection receipt
  - projection consumption from inspection receipt
  - basis capability lifecycle
- support-gated:
  - structural correspondence for historical topology basis until Phase 16
- out:
  - spatial re-walks over raw topology internals

**Warnings**
- Do not let `worth-spatial` reimplement topology traversal logic.
- Do not emit planar predicate facts from incomplete loop, shell, orientation,
  or neighborhood facts.
- Do not let topology legality failures appear as predicate uncertainty.

**Test requirements**
- `topology_to_spatial_planar_contract_completeness_blocks_incomplete_loop_shell_and_orientation_basis`
- prove missing or contradictory topology facts deny before planar predicate or
  structural identity facts are emitted
- `topology_query_projection_consumption_feeds_planar_contract_without_raw_topology_spelunking`
- prove spatial contract completeness consumes Query-owned topology facts and
  not raw topology internals
- `mb_m6_2_high_valence_contract_runs_topology_completeness_before_identity`
- satisfy the topology-completeness assertions from `MB-M6-2`

**Engineering decisions**
- Topology completeness is a gate before planar classification.
- `worth-topo` remains topology owner; `worth-spatial` consumes typed topology
  facts.
- Topology failures and predicate failures stay separate in diagnostics.

**Open questions**
- Exact minimal topology fact floor beyond the Milestone 5 support-query floor.

### Phase 15: Freeze Retained Planar Facts

Phase 15 makes live planar classification retainable and replayable from Query
artifacts without live-state repair.

**Relevant subsystems**
- `worth-spatial` retained planar fact payloads
- `worth-spatial` historical and branch-local inspection
- Forge Query retained artifacts and basis lifecycle
- `worth-kernel` replay parity certification

**Relevant APIs**
- `worth_spatial::facade::inspection::*`
- existing retained geometry subject and payload patterns
- new retained planar fact payload family
- Query basis capability lifecycle and historical diff/basis

**Required Query posture**
- required now:
  - retained artifact inspection
  - basis capability lifecycle
  - historical diff and basis
  - branch-local inspection
  - declaration-entry inspection
  - retained artifact to next step
- support-gated:
  - structural correspondence and historical materialization if planar
    materialization paths need it
- out:
  - retained facts patched from live topology or ambient caches

**Warnings**
- Do not reconstruct retained planar truth from current live state.
- Do not store summary digests without the typed basis needed for replay.
- Do not let branch-local planar truth masquerade as authoritative truth.

**Test requirements**
- `retained_planar_facts_replay_without_live_state_repair`
- prove retained planar classification, identity, transform posture, and denial
  basis replay after live state changes
- `retained_planar_facts_reject_wrong_or_truncated_basis_before_partial_answer`
- prove wrong basis, truncated transform posture, or incomplete topology facts
  deny before best-effort inspection can answer
- `mb_m6_4_retained_planar_history_cancellation_chain`
- satisfy all retained-history assertions from `MB-M6-4`

**Engineering decisions**
- Retained planar facts are canonical replay substrate.
- Movement/rotation posture and precision basis are part of the retained fact.
- Historical and branch-local planar inspection extend the M5 retained geometry
  pattern instead of creating another runtime story.

**Open questions**
- Whether retained planar facts share a subject wrapper with retained rebinding
  facts or use a dedicated planar subject type.

### Phase 16: Freeze Projection-Consumed Planar Facts

Phase 16 makes planar facts consumable downstream through Query projection
consumption receipts instead of retained payload spelunking or kernel summaries.

**Relevant subsystems**
- `worth-spatial` projection consumption
- `worth-spatial` retained planar facts
- Forge Query projection consumption
- `worth-kernel` boolean-readiness assembly

**Relevant APIs**
- `worth_spatial::facade::projection::*`
- existing `GeometryProjectionConsumptionDeclarationFamily`
- new planar projected fact kind and receipt family
- Query projection consumption declarations and receipts

**Required Query posture**
- required now:
  - projection consumption
  - declaration route contract
  - ordinary outcomes
  - retained artifact basis
  - materialization digest binding
  - inspection for projected fact receipts
- support-gated:
  - live maintained projection consumers beyond M6 certification
- out:
  - reading retained payload maps directly in kernel

**Warnings**
- Do not let projection consumption recompute planar truth.
- Do not let projection receipts omit retained source digest, precision basis,
  transform basis, or structural identity.
- Do not allow denied retained facts to become projected success.

**Test requirements**
- `projection_consumed_planar_facts_match_live_and_retained_basis`
- prove projection-consumed planar facts match live and retained facts for
  equivalent semantic basis
- `projection_consumed_planar_facts_preserve_denials_without_summary_upgrade`
- prove denied or unsupported retained facts stay denied and cannot be upgraded
  through projected summaries
- `mb_m6_7_projection_consumed_planar_fact_parity`
- satisfy the parity assertions from `MB-M6-7`

**Engineering decisions**
- Projection consumption is the only downstream fact-delivery lane for M6
  boolean-readiness consumers.
- Kernel certification consumes receipts, not payload internals.
- Projected fact identity includes retained source digest.

**Open questions**
- Exact name for planar projected fact kind.

### Phase 17: Freeze Planar Recovery Posture

Phase 17 defines typed next-step recovery for denied or unsupported planar facts
without letting recovery synthesize missing planar truth.

**Relevant subsystems**
- `worth-spatial` geometry recovery
- `worth-spatial` planar clean-fail taxonomy
- Forge Query recovery boundary
- `worth-kernel` clean-fail certification

**Relevant APIs**
- `worth_spatial::facade::recovery::*`
- existing `GeometryRecoveryActionDeclarationFamily`
- new planar recovery target scopes where existing scopes are too broad
- Query ordinary outcomes and recovery boundary

**Required Query posture**
- required now:
  - ordinary outcomes
  - checked stops
  - recovery brief
  - recovery boundary
  - mutation evidence if recovery action mutates authoritative posture
  - declaration route contract
- support-gated:
  - replay-bearing recovery equivalence unless support matrix admits it
- out:
  - recovery that changes predicate or identity truth

**Warnings**
- Do not turn `Unsupported` into `Bound` through recovery.
- Do not hide policy-required decisions as retry suggestions.
- Do not synthesize bounded truth for unbounded/open planar classes.

**Test requirements**
- `planar_recovery_consumes_typed_denial_without_reclassifying_truth`
- prove recovery action is derived from typed denial facts and cannot change the
  planar outcome class
- `planar_recovery_rejects_missing_retained_or_projection_basis`
- prove recovery cannot proceed from incomplete retained facts, missing
  projection receipt, or unknown transform posture
- `mb_m6_5_dirty_input_and_mb_m6_6_unbounded_posture_recovery_rows`
- satisfy recovery assertions from `MB-M6-5` and `MB-M6-6`

**Engineering decisions**
- Recovery is a next-step lane, not a repair authority.
- Recovery actions are receipt-backed Query declarations.
- Recovery posture must name whether the blocker is predicate, topology,
  binding/rebinding, unsupported class, policy, or transform invalidation.

**Open questions**
- Whether policy-required planar classes need a dedicated action family or a
  narrower recovery action kind.

### Phase 18: Freeze Planar Diagnostics And Causal Localization

Phase 18 makes explanations derived from typed facts and receipts, including
cross-runtime causal inspection where the failure crosses topology, spatial,
bridge, or Query boundaries.

**Relevant subsystems**
- `worth-spatial` planar diagnostics
- `worth-topo` topology diagnostics and validation
- Forge Query inspection and cross-runtime causal inspection
- `worth-kernel` diagnostic certification

**Relevant APIs**
- `worth_topo::projection::runtime_boundary::declared_query_surfaces`
- `worth_spatial::facade::inspection::*`
- Query inspection
- Query cross-runtime causal inspection
- new planar diagnostic bundle family

**Required Query posture**
- required now:
  - declaration-entry inspection
  - retained artifact inspection
  - cross-runtime causal inspection at reference richness
  - basis capability lifecycle
  - ordinary outcomes
  - projection consumption receipts
- support-gated:
  - materialized causal archive
- out:
  - explanation prose as authority

**Warnings**
- Do not let diagnostics decide predicate truth.
- Do not collapse topology contract failure into predicate uncertainty.
- Do not claim materialized causal archive if Query only admits reference
  richness.

**Test requirements**
- `planar_diagnostics_localize_predicate_topology_binding_policy_and_transform_failures`
- prove diagnostics distinguish every M6 clean-fail class with exact trigger
  locality
- `planar_causal_inspection_explains_cross_runtime_failure_without_reopening_truth`
- prove causal inspection can explain topology-to-spatial and projection
  failures without recomputing predicate or topology truth
- `mb_m6_8_final_boss_orientation_flip_localizes_exact_step`
- ensure the orientation flip from `MB-M6-8` is localized at the exact retained
  step and cannot be hidden by later lanes

**Engineering decisions**
- Diagnostics are derived from typed facts, receipts, and basis artifacts.
- Cross-runtime causal inspection is used for why-across-runtimes, not as a
  substitute for retained inspection.
- Diagnostic bundles must be machine-checkable.

**Open questions**
- Exact richness level for first-shipping planar causal diagnostics.

### Phase 19: Freeze Local Planar Rebuild And Rebinding Parity

Phase 19 proves local planar rebuild and topology replacement consume the M5
rebinding substrate and M6 planar facts without broad search or kernel
reclassification.

**Relevant subsystems**
- `worth-spatial` local replacement and rebinding
- `worth-spatial` planar predicate facts
- `worth-topo` local topology neighborhoods
- Forge Query grouped neighborhood workflow and contribution composition

**Relevant APIs**
- `worth_spatial::facade::neighborhood::*`
- `worth_spatial::facade::rebinding::*`
- `worth_spatial::facade::binding::*`
- new planar rebuild facts if existing rebinding facts cannot carry planar
  predicate basis

**Required Query posture**
- required now:
  - grouped neighborhood workflow
  - contribution composition where policy-bearing planar inputs participate
  - declaration readiness
  - declaration-entry orchestration
  - lower-runtime routing
  - boundary receipts and envelopes
  - projection consumption
  - recovery boundary
- support-gated:
  - graph composition only if a named planar rebuild cannot be expressed through
    grouped neighborhood workflow
- out:
  - broad candidate search
  - kernel-local planar rebuild summaries

**Warnings**
- Do not start local rebuild without explicit neighborhood facts.
- Do not let correspondence-only rebinding become authoritative continuity.
- Do not let local rebuild recompute planar identity after projection
  consumption.

**Test requirements**
- `local_planar_rebuild_and_rebinding_converge_for_equivalent_neighborhoods`
- prove equivalent local neighborhoods converge across candidate order,
  movement order, and retained replay
- `local_planar_rebuild_denies_broad_search_or_missing_neighborhood_before_identity`
- prove missing neighborhood, broad search, or unsupported replacement denies
  before structural identity or projected facts are emitted
- `mb_m6_2_and_mb_m6_8_local_rebuild_rebinding_parity`
- satisfy high-valence and final-boss local rebuild/rebinding assertions from
  `MB-M6-2` and `MB-M6-8`

**Engineering decisions**
- Local rebuild is neighborhood-scoped and Query-grouped.
- Rebinding remains spatial authority; kernel assembles proof only.
- Planar facts are consumed by rebuild/rebinding, not rediscovered there.

**Open questions**
- Whether the existing rebinding contribution workflow needs planar-specific
  contribution rows.

### Phase 20: Freeze Dirty And Unbounded Planar Clean-Fail Boundaries

Phase 20 closes the two most tempting false-success paths: dirty input repair
and unbounded/open planar conversion. M6 may classify them; it may not silently
heal or boolean them.

**Relevant subsystems**
- `worth-spatial` clean-fail taxonomy
- `worth-topo` topology validation
- Forge Query ordinary outcomes and recovery
- `worth-kernel` pre-MetaBoss clean-fail suites

**Relevant APIs**
- topology validation and diagnostics query surfaces
- planar predicate and admission families from earlier phases
- recovery surfaces from Phase 17
- diagnostics bundle from Phase 18

**Required Query posture**
- required now:
  - declaration legality
  - ordinary outcomes
  - recovery boundary
  - inspection
  - projection consumption denial posture
  - support matrix and admission
- support-gated:
  - policy resolution surfaces if policy-required classes need richer product
    decisions
- out:
  - hidden bounded conversion
  - self-intersection repair as an M6 success path

**Warnings**
- Do not heal dirty input in M6.
- Do not convert unbounded/open planar input into bounded manifold truth.
- Do not let stable topology ids or names override failed planar meaning.

**Test requirements**
- `dirty_planar_input_fails_cleanly_without_heuristic_repair`
- prove self-intersection, non-manifold wire, thin wall, and orientation
  inconsistency produce typed clean-fail posture
- `unbounded_half_space_posture_classifies_without_bounded_conversion`
- prove half-space/open planar classes are classified without hidden clipping or
  manifold repair
- `mb_m6_5_dirty_planar_input_clean_fail_localization`
- `mb_m6_6_unbounded_half_space_planar_posture`

**Engineering decisions**
- Dirty and unbounded classes are explicit M6 clean-fail boundaries.
- Policy-required is an acceptable outcome only with exact diagnostic trigger.
- Boolean execution remains out of scope.

**Open questions**
- Which unbounded/open planar classes become admitted in M7, if any.

### Phase 21: Freeze Boolean-Readiness Contract Bundles

Phase 21 defines the artifact M7 is allowed to consume. It is the M6 product:
a complete planar boolean-readiness bundle, not a boolean result.

**Relevant subsystems**
- `worth-spatial` boolean-readiness facts
- `worth-topo` topology contract facts
- `worth-kernel` bundle assembly and certification
- Forge Query receipts, envelopes, retained facts, and projection consumption

**Relevant APIs**
- all M6 planar fact families
- `worth_spatial::facade::projection`
- `worth_spatial::facade::inspection`
- `worth_topo` query receipt/envelope/fact surfaces
- new kernel certification bundle that consumes only typed facts and receipts

**Required Query posture**
- required now:
  - declaration progression
  - declaration readiness
  - route plan
  - boundary receipts
  - boundary envelopes
  - ordinary outcomes
  - retained artifacts
  - projection consumption
  - inspection
  - recovery
  - signal/continuation posture where retained facts become next-step inputs
  - lower-runtime capability routing
  - authoritative mutation evidence where facts are mutation-bearing
- support-gated:
  - M7 boolean execution lanes
- out:
  - split/classify/assemble

**Warnings**
- Do not let bundle assembly create facts that spatial/topology owners did not
  produce.
- Do not accept partial bundles as M7 input.
- Do not allow unsupported families to be absent; they must be explicitly typed.

**Test requirements**
- `boolean_readiness_bundle_contains_all_required_planar_fact_families`
- prove complete bundles include predicate, identity, topology-completeness,
  precision, transform, retained, projection, recovery, diagnostics, and support
  posture rows
- `boolean_readiness_bundle_rejects_partial_or_kernel_synthesized_facts`
- prove missing fact families, kernel-synthesized facts, or unsupported hidden
  classes fail before M7 can consume the bundle
- `mb_m6_8_boolean_readiness_final_boss`
- satisfy the complete final-boss assertions from `MB-M6-8`

**Engineering decisions**
- M7 consumes only boolean-readiness bundles, not arbitrary planar facts.
- Kernel owns assembly and certification, not planar fact truth.
- Bundle completeness is mechanically testable.

**Open questions**
- Exact bundle type and whether it lives under `worth-kernel` certification or a
  narrow `worth-spatial` boolean-readiness facade.

### Phase 22: Freeze MB-M6-1 Coplanar Overlap Contract Storm

Phase 22 closes the coplanar-overlap storm as its own production proof. It must
prove the full workload and the user-facing outcome surface, not a synthetic
test-local matrix.
It also refactors the current MB1 harness so the storm enters through real
topology, geometry binding, projection, retained replay, and actual
movement/rotation semantics instead of procedural-only spatial setup.

**Relevant subsystems**
- `worth-spatial` coplanar overlap contracts
- `worth-spatial` planar diagnostics and clean-fail posture
- `worth-spatial` exact predicate authority and precision/local-frame basis
- Forge Query declaration, receipt, envelope, and diagnostic evidence lanes

**Relevant APIs**
- `CoplanarOverlapContractExtractor`
- `CoplanarOverlapUserOutcome`
- `CoplanarOverlapUserDecision`
- `CoplanarOverlapNoOptionsCause`
- `PlanarDiagnosticBundle`
- `PlanarCleanFailBoundary`

**Required Query posture**
- required now:
  - overlap declaration entry and bound receipt
  - policy-required ordinary outcome projection
  - diagnostic receipt for every denied movement/rotation lane
  - clean-fail receipt for every dirty or unsupported no-options lane
  - predicate-authority error projection for predicate uncertainty
- support-gated:
  - M7 boolean imprint or split/classify decisions
- out:
  - test-local outcome rows, synthetic evidence strings, or fake user choices

**Policy, matrix, and human response requirements**
- The production overlap outcome surface must branch every observed result into:
  certified, policy-required, no-options dirty input, no-options unsupported
  input, no-options denied movement/rotation, and no-options predicate
  uncertainty.
- Policy-required rows must expose typed user decisions with readable labels.
- No-options rows must expose a typed cause and a human-readable explanation;
  they must not advertise selectable outcomes.
- Machine identifiers may remain as evidence digests, but the user-facing
  message must be readable without knowing internal token names.

**Warnings**
- Do not satisfy the matrix with a test-local enum or helper-only adapter.
- Do not ignore the full storm in the closeout suite.
- Do not assert only absence of bad machine strings; assert exact
  human-readable messages where the message is public contract.

**Test requirements**
- `mb_m6_1_coplanar_overlap_storm_end_to_end_receipts`
- prove the full storm runs in the normal public contract suite and exercises
  hundreds of coplanar faces across admitted overlap regions; the proof must
  start from real topology and geometry-binding setup and must not be satisfied
  by procedural rectangles alone
- `mb_m6_1_user_outcome_matrix_branches_every_stop`
- prove the matrix is made of production `CoplanarOverlapUserOutcome` values
  derived from real receipts, denials, diagnostics, clean-fail receipts, and
  predicate-authority errors
- `mb_m6_1_equivalent_motion_subset_converges_without_full_storm_replay`
- prove equivalent movement, rotation, host order, and retained replay converge;
  actual coordinates and retained Query artifacts must change/flow through the
  tested path, so identity-label changes and second extractor calls with the
  same inputs are not acceptable replay or motion proof
- `mb_m6_1_fixture_arithmetic_cannot_satisfy_storm_truth`
- prove storm counters are cross-checked against production receipts and
  geometry/topology evidence rather than only against generator cardinality

**Engineering decisions**
- The full storm is a real executed proof, not an ignored artifact.
- The matrix is a production outcome contract, not test scaffolding.
- User-facing policy and no-options causes are part of the overlap public
  surface.
- Generated hostile regions are permitted only as one input source. They must
  be routed through the same topology, binding, projection, retained replay, and
  movement/rotation path as non-generated Worth geometry.

**Open questions**
- Whether the overlap user-outcome surface should remain overlap-owned or be
  promoted to a broader planar user-outcome boundary if later MB phases share
  the same response taxonomy.

### Phase 23: Freeze MB-M6-2 High-Valence Planar Singularity Contract

Phase 23 closes the high-valence singularity proof as a separate pressure
family. It must prove exact predicate localization and topology-to-spatial
admission before any rebuild, rebinding, or correspondence summary can help.

**Relevant subsystems**
- `worth-spatial` predicate authority and precision escalation
- `worth-spatial` topology-to-spatial contract completeness
- `worth-spatial` local rebuild/rebinding parity
- `worth-topo` topology legality and local neighborhood truth
- Forge Query inspection and support posture

**Relevant APIs**
- `PlanarPredicateAuthority`
- `PlanarPrecisionCertification`
- `PlanarTopologyContractCompleteness`
- `PlanarLocalRebuildParity`
- `PlanarDiagnosticBundle`
- production user-outcome surface for singularity admission and no-options
  causes, added before the test if no adequate surface exists

**Required Query posture**
- required now:
  - predicate declaration and receipt/error envelope
  - topology completeness receipt before spatial identity or projection facts
  - local neighborhood Query receipt for any admitted rebuild pressure
  - diagnostic receipt naming singular vertex/neighborhood and movement posture
- support-gated:
  - manifold repair or non-M6 singularity healing
- out:
  - topology-name or binding-summary substitution for predicate authority

**Policy, matrix, and human response requirements**
- The production matrix must classify admitted, policy-required,
  predicate-uncertain, topology-contract-failed, and movement-neighborhood
  denied outcomes.
- Each no-options row must name whether the blocker is predicate uncertainty,
  topology contract failure, unsupported valence posture, or movement/rebuild
  incompatibility.
- Policy-required singularity rows must expose selectable policy decisions only
  when a real production policy surface owns those decisions.
- Human-readable responses must name the singular vertex or local neighborhood,
  not just a digest.

**Warnings**
- Do not pre-solve valence pressure in fixtures by constructing already-clean
  low-valence cases.
- Do not let correspondence continuity substitute for topology completeness.
- Do not collapse topology failure and predicate uncertainty into one generic
  denial.

**Test requirements**
- `mb_m6_2_high_valence_planar_singularity_contract`
- prove deterministic predicate posture across admitted incidence-preserving
  movement/rotation variants
- `mb_m6_2_singularity_no_options_matrix_names_exact_blocker`
- prove each denied or unavailable singularity branch has a typed production
  cause and human-readable response
- `mb_m6_2_rebuild_movement_break_denies_before_correspondence`
- prove a movement that breaks the local replacement neighborhood fails before
  rebinding can become fake continuity

**Engineering decisions**
- High valence is a predicate/topology contract boundary, not a boolean repair
  boundary.
- User response policy must be production-owned before the matrix test exists.

**Open questions**
- Exact admitted valence ceiling for M6, if any; unsupported valence must be
  explicit if the ceiling is intentionally bounded.

### Phase 24: Freeze MB-M6-3 Thin-Feature Scale-Separation Contract

Phase 24 closes thin-feature scale separation under large-world and micro-local
coordinates. It must prove local scale, precision escalation, and user-facing
uncertainty without hiding cost or fallback.

**Relevant subsystems**
- `worth-math` precision escalation metadata
- `worth-spatial` precision certification and local frame certificates
- `worth-spatial` projection-consumed facts
- `worth-spatial` diagnostics and recovery posture
- Forge Query counters and retained receipts

**Relevant APIs**
- `PlanarPrecisionCertification`
- `PlanarLocalFrameCertificate`
- `ProjectPointToCertifiedPlane2D`
- `ProjectionConsumedPlanarFacts`
- `PlanarDiagnosticBundle`
- production thin-feature outcome/policy surface, added before tests if missing

**Required Query posture**
- required now:
  - precision declaration and receipt
  - local-frame receipt with local feature scale and world magnitude
  - projection receipt and projection-consumption receipt
  - diagnostic receipt for every tiny-rotation or predicate-uncertain branch
- support-gated:
  - M7 micro-feature boolean split/classify execution
- out:
  - global-coordinate epsilon fallback or hidden snapping

**Policy, matrix, and human response requirements**
- The production matrix must classify admitted scale-separated facts,
  predicate-uncertain micro-feature cases, unsupported tiny-rotation cases, and
  no-options precision-basis failures.
- Human-readable responses must distinguish local scale failure from world
  coordinate magnitude and must name the affected micro-feature/local frame.
- Policy choices must be typed only where the modeler can responsibly choose a
  handling policy; precision failures with no safe options must remain
  no-options.

**Warnings**
- Do not test one toy micro-feature and claim scale-separation closure.
- Do not let elapsed time stand in for structural counters.
- Do not let projection consumption recompute local frame truth.

**Test requirements**
- `mb_m6_3_thin_feature_scale_separation_contract`
- prove precision escalation is based on local feature scale and survives
  large-world coordinates
- `mb_m6_3_micro_feature_outcome_matrix_is_production_owned`
- prove admitted, predicate-uncertain, and no-options branches come from
  production receipts and errors
- `mb_m6_3_projection_consumption_preserves_local_basis`
- prove projection-consumed facts match retained/local-frame basis without
  broadening to whole-model scans

**Engineering decisions**
- Scale-separation user outcomes belong at the precision/local-frame boundary,
  not inside boolean execution.
- Counters are acceptance evidence, not advisory diagnostics.

**Open questions**
- Exact micro-feature workload cardinality for the closeout proof.

### Phase 25: Freeze MB-M6-4 Retained Planar History Cancellation Chain

Phase 25 closes retained-history cancellation as a distinct replay proof. It
must prove exact cancellation, near-graze localization, and response policy at
the retained step where truth changes.

**Relevant subsystems**
- `worth-spatial` retained planar facts
- `worth-spatial` movement/rotation posture
- `worth-spatial` structural identity
- `worth-spatial` diagnostics and projection consumption
- Forge Query retained artifact and replay lanes

**Relevant APIs**
- `RetainedPlanarFacts`
- `PlanarMotionPosture`
- `PlanarStructuralIdentity`
- `ProjectionConsumedPlanarFacts`
- `PlanarDiagnosticBundle`
- production retained-history user-outcome surface if no existing surface can
  honestly represent cancellation and near-graze outcomes

**Required Query posture**
- required now:
  - retained fact declaration/progression/route/receipt/envelope
  - movement/rotation posture receipt for every step
  - structural identity receipt at every cancellation checkpoint
  - diagnostic receipt localizing near-graze step
- support-gated:
  - M7 boolean chain execution
- out:
  - final-coordinate-only cancellation, history summary substitution, or
    deferred localization at the end of the chain

**Policy, matrix, and human response requirements**
- The production matrix must distinguish exact cancellation success,
  policy-required near-graze, predicate uncertainty, retained replay mismatch,
  and no-options motion/rotation invalidation.
- Human-readable responses must name the retained step, transform posture, and
  exact blocker.
- Policy choices may be offered only at the retained step that produced the
  ambiguous or policy-required evidence.

**Warnings**
- Do not prove cancellation only at final output; checkpoint every named
  cancellation boundary.
- Do not let replay reorder or hide the injected near-graze trigger.
- Do not use final coordinate equality as structural identity.

**Test requirements**
- `mb_m6_4_retained_planar_history_cancellation_chain`
- prove exact cancellation checkpoints are bit-identical and near-graze is
  localized to the injected step
- `mb_m6_4_retained_outcome_matrix_branches_each_history_stop`
- prove production outcomes cover cancellation success, policy-required,
  predicate uncertainty, replay mismatch, and no-options transform denial
- `mb_m6_4_projection_consumed_facts_match_retained_checkpoints`
- prove projection-consumed facts before and after the trigger match retained
  basis for the same semantic step

**Engineering decisions**
- Retained-history outcomes are step-local.
- Recovery and diagnostics must not change retained planar truth.

**Open questions**
- Whether the closeout chain uses 500 steps or a bounded but slope-sensitive M6
  variant with exact counters.

### Phase 26: Freeze MB-M6-5 Dirty Planar Input Clean-Fail Localization

Phase 26 closes dirty planar input as a clean-fail proof. It must prove dirty
input never becomes admitted truth through repair, stable topology ids, or
movement/rotation pressure.

**Relevant subsystems**
- `worth-spatial` clean-fail boundary
- `worth-spatial` recovery posture
- `worth-spatial` diagnostics
- `worth-spatial` movement/rotation posture
- `worth-topo` topology contract truth

**Relevant APIs**
- `PlanarCleanFailBoundary`
- `PlanarCleanFailInput`
- `PlanarRecoveryPosture`
- `PlanarDiagnosticBundle`
- production dirty-input outcome surface, added before tests if missing

**Required Query posture**
- required now:
  - clean-fail boundary receipt
  - recovery posture receipt
  - diagnostic receipt naming first blocker
  - movement/rotation posture receipt where transforms expose or preserve dirt
- support-gated:
  - repair or healing operations outside M6
- out:
  - heuristic repair, topology-only success, or hidden bounded conversion

**Policy, matrix, and human response requirements**
- The production matrix must classify dirty self-intersection, non-manifold
  wire, thin wall, orientation inconsistency, and movement/rotation
  invalidation as typed no-options or policy-required branches.
- Human-readable responses must name the dirty input class and the first
  blocking feature.
- If there are no safe choices, the response must explain why no options are
  offered rather than returning an empty or generic failure.

**Warnings**
- Do not reuse one dirty fixture while changing only labels.
- Do not let stable topology ids, names, or binding identity reconstruct
  passing structural identity.
- Do not let projection-consumed facts consume dirty retained basis as admitted
  truth.

**Test requirements**
- `mb_m6_5_dirty_planar_input_clean_fail_localization`
- prove every dirty class fails cleanly without heuristic repair
- `mb_m6_5_dirty_outcome_matrix_branches_each_dirty_kind`
- prove production outcomes expose each dirty class and no-options cause with a
  human-readable message
- `mb_m6_5_dirty_transform_pressure_preserves_failure_class`
- prove translations, rotations, and orientation-reversing transforms do not
  hide or repair dirty input

**Engineering decisions**
- Dirty input policy belongs to clean-fail/recovery/diagnostics, not overlap or
  boolean execution.
- Source detail is identity-bearing.

**Open questions**
- Whether any dirty class in M6 may become policy-required instead of
  no-options, and what user decisions would be safe.

### Phase 27: Freeze MB-M6-6 Unbounded Half-Space Planar Posture

Phase 27 closes open and unbounded planar posture. It must prove M6 classifies
these cases honestly without bounded conversion or hidden manifold repair.

**Relevant subsystems**
- `worth-spatial` clean-fail boundary
- `worth-spatial` recovery posture
- `worth-spatial` structural identity
- `worth-spatial` diagnostics
- Forge Query support and admission matrix

**Relevant APIs**
- `PlanarCleanFailBoundary`
- `PlanarOpenInputKind`
- `PlanarRecoveryPosture`
- `PlanarStructuralIdentity`
- production unbounded/open outcome surface, added before tests if missing

**Required Query posture**
- required now:
  - support/admission row for every open/unbounded class
  - clean-fail or admitted posture receipt
  - recovery posture receipt that does not synthesize bounded truth
  - diagnostic receipt naming half-space group or open domain
- support-gated:
  - bounded conversion and M7 open-sheet handling
- out:
  - clipping, inferred manifold repair, or hidden finite surrogate geometry

**Policy, matrix, and human response requirements**
- The production matrix must classify half-space groups and open planar domains
  as admitted, unsupported, policy-required, or predicate-uncertain before M7.
- Human-readable responses must name the open/unbounded class and explain why
  bounded boolean overlap cannot proceed when no options exist.
- Policy choices must be explicit if a half-space arrangement can be admitted
  only with modeler intent.

**Warnings**
- Do not silently convert open domains to bounded domains.
- Do not leave unsupported posture as absence of a row.
- Do not collapse orientation-changing rotation into canonical equivalence.

**Test requirements**
- `mb_m6_6_unbounded_half_space_planar_posture`
- prove half-space/open domains classify without bounded conversion
- `mb_m6_6_unbounded_outcome_matrix_explains_no_options`
- prove production outcomes expose unsupported/policy/predicate branches with
  readable user responses
- `mb_m6_6_half_space_transform_canonicalization_and_divergence`
- prove equivalent transform cycles converge and semantic inversions perturb
  identity or outcome

**Engineering decisions**
- Open/unbounded posture is a first-class M6 clean-fail/admission result.
- Recovery suggests next steps only; it cannot create bounded truth.

**Open questions**
- Which half-space arrangements, if any, become admitted in M7.

### Phase 28: Freeze MB-M6-7 Projection-Consumed Planar Fact Parity

Phase 28 closes projection-consumed parity across live, retained, recovered,
replayed, movement/rotation, and rebuild views. It proves denied paths remain
denied and admitted paths converge.

**Relevant subsystems**
- `worth-spatial` projection-consumed planar facts
- `worth-spatial` retained planar facts
- `worth-spatial` recovery posture and diagnostics
- `worth-spatial` local rebuild/rebinding parity
- Forge Query projection consumption and retained artifact lanes

**Relevant APIs**
- `ProjectionConsumedPlanarFacts`
- `RetainedPlanarFacts`
- `PlanarRecoveryPosture`
- `PlanarLocalRebuildParity`
- `PlanarDiagnosticBundle`
- production parity outcome surface, added before tests if no existing surface
  can honestly represent parity mismatches and denied views

**Required Query posture**
- required now:
  - live planar receipt
  - retained planar receipt
  - projection-consumed receipt
  - recovery receipt
  - replay artifact receipt
  - local rebuild/rebinding receipt
  - diagnostic receipt for every mismatch
- support-gated:
  - M7 boolean result materialization
- out:
  - summary-only parity, projection-success upgrade of denied paths, or hidden
    retained basis rebuild

**Policy, matrix, and human response requirements**
- The production matrix must classify admitted parity, denied parity,
  projection mismatch, retained mismatch, recovery mismatch, replay mismatch,
  and rebuild mismatch.
- Human-readable responses must name the surface where parity broke: live,
  retained, projection, recovery, replay, movement, rotation, or rebuild.
- Policy choices are allowed only when parity failure is genuinely
  policy-required; integrity mismatches are no-options.

**Warnings**
- Do not compare only live and projection while skipping retained/recovery.
- Do not let denied retained basis become projection success.
- Do not accept same digest from a helper that rebuilt basis locally.

**Test requirements**
- `mb_m6_7_projection_consumed_planar_fact_parity`
- prove equivalent semantic inputs converge across live, retained,
  projection-consumed, recovered, replayed, movement/rotation, and rebuild views
- `mb_m6_7_denied_paths_remain_denied_across_all_views`
- prove denied workloads never upgrade through projection or recovery summaries
- `mb_m6_7_parity_outcome_matrix_localizes_each_mismatch_surface`
- prove production outcomes explain each mismatch surface with exact readable
  messages and typed causes

**Engineering decisions**
- Projection parity is a cross-view production contract, not a helper
  comparison.
- Integrity mismatch is no-options unless a real policy surface owns choices.

**Open questions**
- Exact representative workload set for admitted and denied parity rows.

### Phase 29: Freeze MB-M6-8 Boolean-Readiness Final Boss

Phase 29 closes the combined pre-boolean final boss. It must compose all M6
proof families and stop exactly at the boolean-readiness boundary.

**Relevant subsystems**
- all M6 `worth-spatial` planar fact families
- `worth-topo` topology contract completeness
- `worth-kernel` boolean-readiness certification
- Forge Query support, declaration, receipt, retained, projection, recovery,
  diagnostics, and continuation lanes

**Relevant APIs**
- `PlanarBooleanReadinessBundle`
- all M6 planar fact receipts
- `PlanarM7Readiness` or equivalent pre-boolean readiness surface
- production final-boss outcome surface, added before tests if no existing
  surface can honestly represent bundle-ready vs typed clean failure

**Required Query posture**
- required now:
  - every Query surface required by Phases 1 through 28
  - complete boolean-readiness bundle receipt
  - typed clean-fail receipt for every unsupported/denied sub-workload
  - diagnostic receipt for the exact final-boss trigger
  - public support matrix proof that M7 may consume only complete bundles
- support-gated:
  - M7 split/classify/assemble
- out:
  - any boolean result, manifold output, or kernel summary standing in for the
    readiness bundle

**Policy, matrix, and human response requirements**
- The production matrix must classify complete boolean-readiness,
  policy-required final-boss branch, typed clean failure, unsupported family,
  predicate uncertainty, projection mismatch, recovery mismatch, and
  orientation-flip localization.
- Human-readable responses must say whether M7 may proceed, which exact
  sub-workload blocked it if not, and what user policy choices are available if
  any.
- No-options final-boss responses must include exact cause and evidence family;
  they must not collapse to "readiness failed."

**Warnings**
- Do not perform M7 boolean split/classify/assemble.
- Do not let a complete-looking bundle omit unsupported families.
- Do not let kernel-local workflow summaries substitute for spatial/topology
  proof receipts.

**Test requirements**
- `mb_m6_8_boolean_readiness_final_boss`
- prove the final output is either a complete boolean-readiness bundle or a
  typed clean failure with exact trigger localization
- `mb_m6_8_final_boss_outcome_matrix_is_production_owned`
- prove the final-boss matrix uses production readiness/clean-fail/user-outcome
  surfaces only
- `mb_m6_8_no_kernel_summary_can_substitute_for_readiness_receipts`
- prove kernel summaries cannot replace spatial predicate authority, retained
  facts, projection-consumed facts, movement/rotation posture, recovery, or
  diagnostics

**Engineering decisions**
- MB-M6-8 is the final pre-boolean proof, not the start of M7.
- Every admitted and denied sub-workload must preserve its class across live,
  retained, projection-consumed, recovered, replayed, and rebuild views.

**Open questions**
- Final public type names for the combined readiness user-outcome surface.

### Phase 30: Freeze Milestone 6 Certification And Legacy Deletion Proof

Phase 30 closes M6 after every individual MB phase is real, registered, and
passing. It locks out the shortcuts this milestone exists to kill.

**Relevant subsystems**
- `worth-kernel` certification
- `worth-spatial` planar semantic certification
- `worth-topo` topology-query certification
- Forge Query public contract and UI compile-fail suites

**Relevant APIs**
- `MB-M6-*` suites from `_docs/worth/m6-premetaboss.md`
- kernel public API contract suite
- spatial public API contract suite
- topology public API contract suite
- legacy deletion fixtures for forbidden planar shortcuts

**Required Query posture**
- required now:
  - all Query surfaces required by Phases 1 through 29
  - support matrix and admission closeout
  - public API contract proof
  - compile-fail deletion proof for local pseudo-Query and kernel-owned planar
    runtime paths
- support-gated:
  - any Query surface explicitly classified as not admitted for M6
- out:
  - certification summaries that do not execute hostile proof rows

**Warnings**
- Do not close M6 with only happy-path unit tests.
- Do not leave old planar helper paths available as compatibility.
- Do not mark M6 complete if any `MB-M6-*` suite is missing, red, ignored,
  synthetic-only, or only partially registered.

**Test requirements**
- `m6_certification_bundle_proves_live_retained_projection_recovery_replay_and_boolean_readiness_parity`
- prove admitted planar workloads converge across every M6 fact lane
- `m6_legacy_deletion_blocks_kernel_local_predicate_identity_retained_and_projection_shortcuts`
- prove forbidden kernel-local predicate, identity, retained, projection, and
  recovery shortcuts cannot be imported or used
- `mb_m6_1_coplanar_overlap_contract_storm`
- `mb_m6_2_high_valence_planar_singularity_contract`
- `mb_m6_3_thin_feature_scale_separation_contract`
- `mb_m6_4_retained_planar_history_cancellation_chain`
- `mb_m6_5_dirty_planar_input_clean_fail_localization`
- `mb_m6_6_unbounded_half_space_planar_posture`
- `mb_m6_7_projection_consumed_planar_fact_parity`
- `mb_m6_8_boolean_readiness_final_boss`

**Engineering decisions**
- M6 closeout requires every pre-MetaBoss suite as a real production-boundary
  test.
- Certification must prove both success and denied-path honesty.
- Legacy deletion is part of the milestone, not cleanup.

**Open questions**
- Final names for compile-fail fixtures once implementation lands.

## Must Ship

- explicit planar admission and support posture
- exact planar predicate authority owned by `worth-spatial`
- precision escalation and tolerance basis with visible counters
- `PlanarLocalFrameCertificate`
- `ProjectPointToCertifiedPlane2D`
- `CertifiedSegmentSegment2D`
- `CertifiedPolygonWinding2D`
- `CertifiedSignedArea2D`
- `CoplanarOverlapContractExtractor`
- `PlanarContractBundleValidator`
- `PredicateCertificateConsumptionValidator`
- planar structural identity and fingerprint surfaces distinct from topology,
  naming, lineage, binding identity, and final coordinates
- movement, rotation, reorientation, and cancellation posture as retained
  semantic input
- topology-to-spatial contract completeness over Query-owned topology facts
- retained planar facts that replay without live-state repair
- projection-consumed planar facts with receipt-backed basis identity
- typed planar recovery posture that cannot synthesize missing truth
- machine-checkable planar diagnostics and reference-rich causal localization
- local planar rebuild and rebinding parity over grouped Query neighborhoods
- dirty and unbounded/open planar clean-fail classification
- boolean-readiness contract bundles suitable for M7 input
- every `MB-M6-*` pre-MetaBoss suite registered, passing, non-ignored, and
  backed by production-owned policy, outcome matrix, and human-readable
  response surfaces
- legacy deletion proofs blocking kernel-local predicate, identity, retained,
  projection, recovery, and pseudo-Query shortcuts

## Must Preserve

- `worth-spatial` as planar predicate, structural identity, recovery,
  projection-consumption, movement/rotation posture, and spatial diagnostics
  authority
- `worth-topo` as topology truth, validation, topology-query fact, and topology
  diagnostic authority
- `worth-kernel` as workflow composition and certification owner only
- Forge Query as the ordinary public runtime layer for M6 work
- Milestone 5 binding/rebinding, retained inspection, replay, recovery,
  projection consumption, signal/continuation, and Query-native hard-break
  closure
- support posture as explicit runtime fact rather than inferred visibility
- clean-fail behavior before repair, coercion, boolean execution, or local
  summary generation
- exact counters and proof-bearing artifacts at every named cost boundary

## Acceptance Evidence

Milestone 6 is accepted only when all of the following evidence exists:

- `cargo check -p worth-spatial -p worth-kernel -p worth-topo`
- `cargo test -p worth-spatial --test public_api_contract -- --nocapture`
- `cargo test -p worth-spatial --test ui -- --nocapture`
- `cargo test -p worth-topo --test public_api_contract -- --nocapture`
- `cargo test -p worth-topo --test ui -- --nocapture`
- `cargo test -p worth-kernel --test public_api_contract -- --nocapture`
- `cargo test -p worth-kernel --test ui -- --nocapture`
- focused M6 planar predicate, local-frame, certified projection,
  segment-segment classification, winding/containment, signed-area/degeneracy,
  coplanar-overlap contract, bundle-validation, predicate-consumption,
  identity, retained, projection-consumption, recovery, movement/rotation,
  topology-completeness, and diagnostics suites
- `MB-M6-1` through `MB-M6-8` suites from `_docs/worth/m6-premetaboss.md`,
  each with its own closeout phase, production policy branches, outcome matrix
  coverage, and human-readable no-options/denial/unsupported explanations
- a final M6 certification bundle proving admitted and denied paths across
  live, retained, projection-consumed, recovered, replayed, movement/rotation,
  local rebuild/rebinding, and boolean-readiness lanes
- compile-fail or public-contract proof that forbidden local shortcuts cannot
  return

The exact test module names may change during implementation, but every named
proof family in this spec must have an executed proof row before closeout. No
MB proof may be satisfied by synthetic-only fixtures, test-local policy enums,
ignored tests, or kernel summaries standing in for production receipts.

## Sequencing Notes

- Do not start M6 until the M5 Query-native geometry hard break remains green on
  current verification.
- Implement phases in order unless the spec is revised first. Later retained,
  projection, recovery, and certification phases must consume earlier typed
  facts, not discover missing semantics.
- If M6 exposes a missing generic Query capability, harden Query first. Do not
  create local Worth pseudo-Query.
- If an extreme pre-MetaBoss case cannot be admitted without a major boolean or
  curved-geometry build, classify it as typed unsupported or policy-required
  and prove the clean failure.
- Do not move M7 boolean split/classify/assemble into M6. M6 ends at the
  boolean-readiness contract bundle.

## Required Self-Check

- Does the milestone solve a real structural problem or just package work
  cosmetically? Yes: it freezes exact planar truth and fact-consumption
  boundaries before boolean execution can depend on them.
- Is the adversarial constraint precise and load-bearing? Yes: every phase ties
  back to coplanar, degeneracy, scale, dirty input, unbounded, movement,
  retained, projection, recovery, and boolean-readiness pressure.
- Does the roadmap justify this milestone now? Yes: it sits after
  binding/rebinding and before booleans.
- Does the spec preserve crate authority boundaries? Yes: spatial decides
  planar meaning, topo owns topology truth, kernel certifies/assembles, Query
  owns runtime lanes.
- Are the phases carrying most of the real design information? Yes.
- Is each phase centered on one conceptual detail or boundary? Yes: admission,
  predicate authority, precision, local frame, certified projection,
  segment classification, winding/containment, signed area/degeneracy,
  coplanar overlap contracts, bundle validation, predicate-consumption
  firewall, identity, movement/rotation, topology completeness, retained facts,
  projection consumption, recovery, diagnostics, local rebuild/rebinding,
  clean-fail, boolean-readiness bundle, MB-M6-1 through MB-M6-8 closeout, and
  final certification/deletion closeout.
- Does each phase contain at least 2 adversarial tests by default? Yes.
- Could a competent engineer map this spec into honest types, modules, and
  tests? Yes, with open naming questions left explicit.
- Does the milestone belong in this roadmap sequence, or is it out of order?
  It belongs here because booleans need exact planar and fact-consumption
  substrate first.
