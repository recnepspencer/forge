# Worth Kernel Milestone 5: Binding Authority, Rebinding, History, And Certification

> **Status:** Draft
>
> **Purpose:** freeze topology-to-geometry binding and rebinding as a real
> authority boundary so later exact planar work, booleans, continuity,
> history, and certification inherit an honest substrate instead of
> retrofitting one.

## Goal

Freeze one coherent Milestone 5 substrate in which:

- `worth-spatial` owns binding, anchor, rebinding, continuity, diagnostics,
  history, and binding-layer certification truth
- `worth-topo` owns topology truth, topology-safe navigation helpers, and
  topology legality proof consumed by binding workflows
- `worth-geom` owns carrier evaluation, inversion, closest-point, parameter
  domain, and admitted curved-family math
- `worth-kernel` owns workflow composition, Query-facing lowering, and
  milestone scenario/certification assembly without becoming a second binding
  runtime
- Query remains the canonical declaration/progression/inspection/readiness/
  outcome boundary instead of being shadowed by Worth-local substitutes

## Why This Milestone Exists

Milestone 4 proved the construction-time birth seam.

Milestone 5 widens that seam into real binding and rebinding truth so later:

- exact planar hostility
- topology replacement
- continuity-sensitive edits
- historical inspection
- branch-local inspection
- replay and certification
- narrow curved pressure

all inherit an explicit authority substrate rather than folklore.

Without this milestone:

- topology identity, naming identity, and binding identity would keep
  collapsing into one blurred story
- anchor semantics would remain implied by coordinates and local helper
  behavior instead of carrier-owned truth
- rebinding would keep drifting toward broad search, host-order dependence, or
  explanation-first reasoning
- inspection and replay work would be forced to reconstruct truth from live
  state after the fact
- `worth-kernel` and Query-facing workflow seams would keep pressure to invent
  shadow runtime logic for gaps that should be solved in `worth-spatial`

## Governing Summaries

- `MENTALITY.md`: solve the hard authority and replay problem first. Do not
  build user-facing or DX-friendly surfaces on top of ambiguous binding truth.
- `arch_laws.md`: authority, derivation, orchestration, and inspection must
  stay distinct. Kernel composes; spatial decides; Query owns the public
  declaration/progression/inspection grammar.
- `composition_laws.md`: binding authority, anchors, rebinding, continuity,
  history, kernel workflow, and certification must stay in separate named
  responsibilities instead of collapsing into helpers or mega workflow files.
- `domain_structure_laws.md`: the tree must preserve distinct homes for
  authority, identity, anchors, rebinding, history, curved pressure, workflow,
  and certification.
- `perf_laws.md`: identity, locality, replay, and reuse claims require
  explicit equivalence contracts and scope-bounded evaluation. Cheap-looking
  APIs must not hide broad scans, live-state reconstruction, or shadow
  runtime behavior.
- `pre-milestone-5-cleanup.md`: canonical identity and digest truth are
  prerequisite substrate, not end-of-milestone polish. Replay-grade basis
  discipline must exist before higher inspection and certification claims.

## Adversarial Constraint

Under topology replacement, host-order variation, branch divergence, replay,
restart, retained-history inspection, curved-carrier pressure, asymmetric
carrier pressure, and Query-lowered workflow variation, the same canonical
binding inputs must produce the same binding identity, the same anchor meaning,
the same rebinding continuity class, the same typed outcome, the same retained
inspection truth, and the same certification artifacts unless the scenario is
intentionally semantically different.

If any supported path:

- allows topology identity or naming identity to masquerade as binding truth
- infers carrier ownership from coordinates instead of explicit authority
- lets rebinding depend on broad search or candidate iteration order
- stores incomplete retained artifacts and then patches them from live state
- lets kernel or Query-lowering helpers reinterpret spatial truth
- allows planar or symmetric shortcuts to survive under the admitted curved
  pressure family
- or yields different retained inspection, replay, or certification truth for
  equivalent semantic inputs

then this milestone has failed.

## Product Decision Lock

- Milestone 5 keeps the original 16 explicit phases. They are the primary
  execution order and may not be collapsed into coarse bands.
- Query lane split is fixed now:
  - `declaration-entry` is the primary public workflow, retained-artifact,
    inspection, readiness, ordinary-outcome, and recovery lane
  - graph composition is the mutation-lowering lane only when one admitted
    binding or rebinding workflow truly needs graph-shaped mixed create,
    retarget, update, or retire semantics
- The minimum first-shipping binding family matrix is fixed now:
  - face -> surface
  - edge -> curve
  - coedge -> p-curve
  - vertex -> geometry
- The minimum first-shipping continuity vocabulary is fixed now:
  - `Exact`
  - `AuthoritativeSuccessor`
  - `CorrespondenceOnly`
  - `InsufficientEvidence`
  - `Ambiguous`
  - `None`
- The minimum topology support-query floor is fixed now:
  - `GetFaceLoops`
  - `GetFaceEdges`
  - `GetLoopCoedges`
  - `GetCoedgeEdge`
  - `GetEdgeVertices`
  - `GetShellFaces`
- Additional topology support helpers may widen only when binding/rebinding
  truth honestly requires them, not as speculative convenience inventory.
- The first asymmetric curved pressure family is fixed now:
  - `triaxial_ellipsoid`
- Grouped and contribution-composed Query workflow families are out unless one
  later phase proves that a named Milestone 5 workflow cannot be expressed
  honestly without them.

## Phase Plan

### Phase 1: Freeze Binding Truth As Its Own Authority Surface

**Relevant subsystems**
- `worth-spatial` binding authority
- `worth-topo` topology ownership context
- `worth-kernel` binding authoring workflow entry

**Relevant Query families**
- configured domain handles
- canonical domain declarations
- declaration family taxonomy
- declaration legality
- declaration progression

**Warnings**
- Do not treat binding as incidental construction metadata.
- Do not collapse face, edge, coedge, and vertex bindings into one generic bag.
- Do not let kernel own binding legality.

**Test requirements**
- `planar_binding_authority_roundtrip_preserves_binding_truth`
- prove all four binding families survive authoring, declaration lowering, and
  retained readback as distinct authority artifacts
- `binding_authority_rejects_topology_only_reconstruction_after_geometry_drift`
- prove stable topology identity and stable naming cannot reconstruct a passing
  result after the actual bound geometry changes

**Engineering decisions**
- binding truth is a first-class spatial authority surface
- face, edge, coedge, and vertex bindings all participate in first shipping
- Query declaration/progression is the public lowering path, not a Worth-local
  declaration runtime

**Open questions**
- None

### Phase 2: Freeze Binding Identity As Distinct From Topology And Naming

**Relevant subsystems**
- `worth-spatial` binding identity
- `worth-spatial` certification
- `worth-kernel` authoring parity proof

**Relevant Query families**
- declaration progression
- declaration-entry inspection
- retained artifact inspection

**Warnings**
- Do not let topology identity, naming identity, and binding identity collapse
  into one digest story.
- Do not let host-order or builder-order variation perturb identity.

**Test requirements**
- `binding_identity_diverges_from_topology_and_naming_when_geometry_changes`
- prove equivalent binding meaning converges while changed binding meaning
  diverges even under stable topology id and name
- `binding_identity_is_stable_under_equivalent_authoring_order_variation`
- prove host-order, builder-order, and declaration-order variation do not
  perturb identity when semantic binding meaning is unchanged

**Engineering decisions**
- binding identity is its own truth family
- identity must commit binding meaning, not just topology host facts
- identity and digest basis will later widen into retained history identity

**Open questions**
- None

### Phase 3: Replace Placeholder Anchor Denial With Real Carrier-Local Anchor Truth

**Relevant subsystems**
- `worth-spatial` anchors
- `worth-geom` inversion and carrier math
- `worth-kernel` anchor-bearing declaration authoring

**Relevant Query families**
- canonical domain declarations
- declaration progression
- declaration-entry inspection

**Warnings**
- Do not infer carrier ownership from coordinates.
- Do not let point and direction anchors collapse into one generic anchor bag.
- Do not accept nearest-carrier fallback as authority truth.

**Test requirements**
- `parameter_space_anchor_roundtrip_resolves_on_admitted_carrier`
- `wrong_carrier_anchor_is_typed_denied_not_silently_coerced`
- `parameter_space_direction_anchor_cannot_collapse_to_generic_vector_truth`
- prove direction anchors preserve carrier-local semantic role and cannot pass
  through a generic vector fallback
- `wrong_domain_anchor_is_denied_before_nearest_projection_or_repair`
- prove out-of-domain anchors fail typed before any projection, snapping, or
  closest-point fallback can reinterpret them

**Engineering decisions**
- parameter-space point and direction anchors are separate types
- carrier ownership is explicit and typed
- wrong-carrier and wrong-domain cases deny typed before any fallback

**Open questions**
- None

### Phase 4: Freeze Binding Completeness Rules

**Relevant subsystems**
- `worth-spatial` completeness authority
- `worth-spatial` denial taxonomy
- `worth-kernel` authoring/rejection workflow wiring

**Relevant Query families**
- declaration legality
- declaration progression
- ordinary outcomes

**Warnings**
- Do not treat completeness as merely "all fields are present."
- Do not blur unsupported, incomplete, and illegal states.

**Test requirements**
- `binding_completeness_policy_distinguishes_complete_partial_unsupported_and_illegal`
- prove admitted complete, admitted partial, denied partial, and typed
  unsupported cases remain explicit and structurally distinct
- `binding_completeness_replay_does_not_upgrade_missing_evidence_into_success`
- prove replay, readback, or workflow-lane variation cannot turn incomplete
  evidence into admitted completeness

**Engineering decisions**
- completeness is a policy-bearing spatial truth surface
- partial binding allowance or denial must be explicit and typed
- later phases may consume completeness facts but may not redefine them

**Open questions**
- whether one shared completeness result wrapper is honest across all four
  families

### Phase 5: Freeze Canonical Binding Identity And Digest Truth

**Relevant subsystems**
- `worth-spatial` identity and digest protocol
- `worth-kernel` declaration/progression parity proof
- `worth-geom` carrier and witness identity contributions

**Relevant Query families**
- existing truth
- inspection
- lineage and correspondence
- projection consumption
- canonical domain declarations

**Warnings**
- Do not leave canonical digest truth as summary-only folklore.
- Do not allow crate-local digest stories to diverge.

**Test requirements**
- `canonical_binding_identity_digest_protocol_is_shared_across_kernel_spatial_and_retained_paths`
- prove canonical binding, anchor, and rebinding identity/digest truth stays
  equivalent across spatial, kernel, and later retained artifact consumption
- `canonical_binding_digest_changes_when_geometry_meaning_changes_but_not_when_formatting_changes`
- prove geometry-bearing semantic drift changes canonical digest while
  formatting, ordering, and non-semantic declaration differences do not

**Engineering decisions**
- one shared identity and digest protocol governs canonical binding truth
- digest basis commits geometry-bearing meaning, not report summaries
- this phase must close before replay, retained inspection, or higher reuse
  claims

**Open questions**
- None

### Phase 6: Freeze Motion-Aware Binding Semantics

**Relevant subsystems**
- `worth-spatial` motion posture
- `worth-kernel` admitted move/rotate/reorient workflow composition
- `worth-geom` carrier evaluation

**Relevant Query families**
- declaration progression
- ordinary outcomes
- recovery boundary

**Warnings**
- Do not treat binding as static birth-only truth.
- Do not infer motion posture from candidate presence alone.

**Test requirements**
- `motion_aware_binding_posture_distinguishes_preserved_transformed_invalidated_and_unresolved`
- prove move, rotate, and reorient workflows classify preserved,
  transformed-with-carrier, invalidated, and unresolved cases explicitly
- `motion_posture_is_not_rederived_from_rebinding_candidate_presence`
- prove motion-aware posture is decided from motion semantics rather than from
  whether later rebinding candidates happen to exist

**Engineering decisions**
- motion-aware posture is a first-class spatial truth surface
- rebinding requirement must be explicit before rebinding candidate search
  begins

**Open questions**
- exact split between transformed-but-still-exact versus preserved wording

### Phase 7: Freeze Rebinding Semantics For Local Topology Replacement

**Relevant subsystems**
- `worth-spatial` local replacement neighborhood
- `worth-topo` navigation/query floor
- `worth-kernel` rebinding workflow

**Relevant Query families**
- declaration progression
- declaration-entry inspection
- ordinary outcomes

**Warnings**
- Do not allow broad search or host-order iteration to masquerade as rebinding.
- Do not begin rebinding without an explicit local replacement neighborhood.
- Do not omit vertex replacement from first-shipping rebinding semantics.

**Test requirements**
- `local_topology_replacement_rebinds_or_denies_canonically_under_replay`
- prove face, edge, coedge, and vertex replacement workflows consume explicit
  local neighborhoods and produce stable results
- `local_rebinding_candidate_order_variation_cannot_change_authoritative_outcome`
- prove candidate/container iteration order cannot perturb preserved,
  reattached, ambiguous, or denied results
- `vertex_rebinding_uses_the_same_local_neighborhood_law_as_other_core_families`
- prove vertex replacement is first-class and not a typed-unsupported escape
  hatch inside the first-shipping rebinding matrix

**Engineering decisions**
- local replacement neighborhood is a typed input artifact
- rebinding is scope-bounded before evaluation
- topology support helpers are mandatory substrate, not optional polish

**Open questions**
- whether reverse/adjacency helper expansion is needed beyond the minimum floor

### Phase 8: Freeze Typed Rebinding Outcome Classes

**Relevant subsystems**
- `worth-spatial` outcome classification
- `worth-spatial` denial taxonomy
- `worth-kernel` outcome transport

**Relevant Query families**
- ordinary outcomes
- recovery boundary
- declaration-entry inspection

**Warnings**
- Do not encode rebinding results as booleans or prose.
- Do not blur unsupported, ambiguous, orphaned, and continuity-justified cases.

**Test requirements**
- `typed_rebinding_outcomes_remain_distinct_under_equivalent_candidate_pressure`
- prove preserved, exact reattachment, continuity-justified reattachment,
  ambiguous, orphaned, and unsupported remain distinct typed outcomes
- `rebinding_outcome_transport_through_kernel_and_query_does_not_collapse_denial_shape`
- prove workflow transport does not blur unsupported, ambiguous, orphaned, or
  continuity-justified outcomes into one generic failure surface

**Engineering decisions**
- rebinding outcomes are first-class spatial truth
- outcome classes consume continuity truth rather than hiding it

**Open questions**
- whether `InsufficientEvidence` needs a distinct outcome mirror or remains
  continuity-only

### Phase 9: Freeze Continuity And Rebinding Diagnostics

**Relevant subsystems**
- `worth-spatial` continuity assessment
- `worth-spatial` diagnostics/explanations
- `worth-kernel` retained workflow proof wiring

**Relevant Query families**
- declaration-entry inspection
- ordinary outcomes
- lineage and correspondence

**Warnings**
- Do not let explanation prose become the source of truth.
- Do not let correspondence masquerade as authoritative continuity.

**Test requirements**
- `continuity_classification_distinguishes_authoritative_successor_correspondence_and_insufficient_evidence`
- prove continuity classes remain typed and stable across denial and ambiguity
  paths
- `rebinding_diagnostics_preserve_candidate_inventory_and_no_winner_cases_without_false_authority`
- prove explanations retain candidate inventory and do not invent a winner for
  ambiguous, orphaned, or unsupported outcomes

**Engineering decisions**
- continuity classes are fixed milestone vocabulary
- explanation is derived from typed truth, not vice versa
- diagnostics must preserve prior meaning, changed meaning, candidates, and
  why a winner did or did not exist

**Open questions**
- whether diagnostics should later split explanation and ambiguity artifacts

### Phase 10: Add Narrow Curved Carrier Pressure

**Relevant subsystems**
- `worth-spatial` curved pressure cases
- `worth-geom` curved carrier evaluation and inversion
- `worth-kernel` certification scenarios

**Relevant Query families**
- declaration progression
- declaration-entry inspection
- ordinary outcomes

**Warnings**
- Do not let Milestone 5 remain secretly planar-only.
- Do not widen into broad freeform or NURBS closure.

**Test requirements**
- `curved_carrier_pressure_breaks_planar_anchor_and_rebinding_shortcuts`
- prove carrier-local anchors, curved binding semantics, curved rebinding, and
  curved continuity diagnostics stay honest on the admitted curved pressure set
- `curved_binding_and_rebinding_do_not_fall_back_to_planarized_identity_or_domain_assumptions`
- prove admitted curved cases reject planarized identity, domain, and
  continuity shortcuts rather than silently coercing them

**Engineering decisions**
- curved pressure remains narrow and intentional
- its purpose is substrate hardening, not broad curved completion

**Open questions**
- whether one additional narrow curved case is needed beyond the first
  asymmetric family

### Phase 11: Add At Least One Asymmetric Curved Primitive Or Carrier Family

**Relevant subsystems**
- `worth-spatial` asymmetric-family binding and rebinding cases
- `worth-geom` asymmetric carrier representation and interrogation
- `worth-kernel` hostile certification scenarios

**Relevant Query families**
- declaration progression
- inspection
- ordinary outcomes

**Warnings**
- Do not rely on planar shortcuts, circular symmetry shortcuts, or axis
  interchange assumptions.
- Do not claim curved robustness using only symmetric carriers.

**Test requirements**
- `triaxial_ellipsoid_breaks_symmetry_dependent_binding_identity_and_anchor_reuse`
- prove the first asymmetric family breaks fake identity stability and anchor
  reuse that survived only on symmetric carriers
- `triaxial_ellipsoid_rebinding_and_continuity_do_not_reuse_axis_interchange_shortcuts`
- prove asymmetric replacement pressure kills continuity and rebinding logic
  that relied on circular symmetry or axis interchange assumptions

**Engineering decisions**
- `triaxial_ellipsoid` is the first required asymmetric pressure family
- this phase hardens previous phases before retained history and replay close

**Open questions**
- None

### Phase 12: Freeze Clean Kernel-To-Spatial Rebinding Seams

**Relevant subsystems**
- `worth-kernel` binding authoring workflow
- `worth-kernel` rebinding workflow
- `worth-spatial` binding/rebinding authority
- Query-facing lowering in `worth-kernel`

**Relevant Query families**
- declaration-entry orchestration
- typed binding pipeline
- ordinary outcomes
- recovery boundary

**Warnings**
- Do not let kernel become a second rebinding or inspection runtime.
- Do not let DX lane and generic Query lane drift into different truths.
- Do not patch over missing spatial/topology semantics with kernel helpers.

**Test requirements**
- `kernel_binding_workflow_consumes_spatial_authority_without_local_rebinding_logic`
- `kernel_rebinding_dx_lane_and_generic_query_lane_converge_to_same_artifacts`
- `kernel_authoring_lane_and_generic_query_lane_share_canonical_declaration_and_progression_truth`
- prove ergonomic and generic entry paths converge at declaration and
  progression boundaries rather than only at final user-visible results
- `kernel_cannot_reinterpret_spatial_denials_or_continuity_classes_for_convenience`
- prove kernel transport cannot widen support posture, hide denials, or
  reclassify continuity to make the workflow seem smoother

**Engineering decisions**
- kernel composes workflow only
- spatial remains the sole authority for binding, anchor, rebinding, and
  continuity meaning
- canonical Query artifacts are the only workflow truth

**Open questions**
- exact parity witness naming and whether boundary proof stays test-local or
  earns a narrow production-internal module

### Phase 13: Freeze Historical Binding Inspection

**Relevant subsystems**
- `worth-spatial` retained history artifacts
- `worth-spatial` historical inspection
- `worth-kernel` inspection workflow

**Relevant Query families**
- declaration boundary receipts
- declaration boundary envelopes
- declaration-entry inspection
- existing truth
- historical diff and basis

**Warnings**
- Do not reconstruct historical truth from current live state.
- Do not let incomplete retained evidence degrade into best-effort inspection.

**Test requirements**
- `historical_binding_inspection_reconstructs_transition_truth_without_live_state`
- prove rebinding transition, continuity class, and explanation survive from
  retained artifacts alone and stay unchanged if current live state mutates
- `historical_binding_inspection_rejects_wrong_or_truncated_basis_before_partial_interpretation`
- prove wrong-basis and incomplete retained evidence deny typed before any
  best-effort historical answer can leak out
- `historical_inspection_digest_is_stable_under_equivalent_retained_artifact_ordering`
- prove equivalent retained basis ordering or formatting differences do not
  perturb historical inspection truth or digest

**Engineering decisions**
- retained artifacts are canonical inspection substrate
- wrong retained basis must deny typed before interpretation
- historical inspection is a distinct responsibility from branch-local
  inspection

**Open questions**
- whether historical and branch-local inspection should share one wrapped error
  family or remain separate

### Phase 14: Freeze Branch-Local Binding Inspection

**Relevant subsystems**
- `worth-spatial` branch-local inspection
- `worth-kernel` branch-local inspection workflow
- retained history/basis artifacts shared with Phase 13

**Relevant Query families**
- declaration-entry inspection
- existing truth
- lineage and correspondence
- continuity vs correspondence

**Warnings**
- Do not let branch-local truth masquerade as authoritative truth.
- Do not let wrong-branch inspection cross-bleed into plausible current truth.

**Test requirements**
- `branch_local_binding_inspection_distinguishes_branch_state_from_authoritative_state`
- prove branch-local and authoritative truth can diverge while preserving
  explicit identity separation and typed wrong-branch denial
- `branch_local_correspondence_never_upgrades_to_authoritative_continuity_under_replay`
- prove correspondence-only branch-local relations do not replay as
  authoritative continuity
- `wrong_branch_binding_inspection_is_denied_before_cross_branch_reconstruction`
- prove wrong-branch input fails typed before any plausible current-state or
  neighboring-branch reconstruction path can answer

**Engineering decisions**
- branch-local inspection is distinct from historical checkpoint inspection
- correspondence does not count as authoritative continuity
- branch-local identity separation must remain explicit

**Open questions**
- exact branch-local correspondence artifact naming

### Phase 15: Freeze Replay-Safe Binding And Rebinding Histories

**Relevant subsystems**
- `worth-spatial` retained identity and replay
- `worth-kernel` replay-grade workflow parity proof
- curved/asymmetric certification scenarios

**Relevant Query families**
- inspection
- retained artifact to next step
- continuation pipeline
- lineage and correspondence

**Warnings**
- Do not claim replay safety from summary digests alone.
- Do not let replay depend on live runtime memory or host-order accident.

**Test requirements**
- `binding_and_rebinding_replay_is_identical_across_equivalent_retained_histories`
- prove binding identity, anchor identity, rebinding outcomes, continuity
  classes, and diagnostics replay identically across retained-history paths
- `replay_parity_fails_loudly_when_retained_identity_or_explanation_basis_is_semantically_different`
- prove replay differences surface as typed parity failure when retained basis
  or explanation meaning genuinely changes

**Engineering decisions**
- replay-safe histories must consume the same canonical identity and digest
  substrate frozen earlier in the milestone
- replay parity includes retained explanations and denied-path artifacts, not
  just admitted outcomes

**Open questions**
- None

### Phase 16: Freeze Determinism And Certification For The Binding Layer

**Relevant subsystems**
- `worth-spatial` certification
- `worth-kernel` scenario assembly
- `worth-topo` topology support certification
- `worth-geom` curved/asymmetric hostile cases

**Relevant Query families**
- inspection
- existing truth
- declaration-entry inspection
- ordinary outcomes

**Warnings**
- Do not claim milestone closure from happy-path unit tests.
- Do not leave determinism, replay parity, or inspection parity as informal
  expectations.

**Test requirements**
- `binding_layer_certification_bundle_proves_determinism_replay_and_inspection_parity_under_hostile_order_variation`
- prove rebinding determinism, continuity classification determinism, binding
  identity stability, historical inspection parity, and branch-local
  inspection parity in one hostile certification bundle
- `binding_layer_certification_bundle_proves_curved_and_asymmetric_pressure_do_not_reopen_earlier_shortcuts`
- prove curved and asymmetric pressure do not reopen planar, symmetric,
  topology-only, or explanation-first shortcuts that earlier phases claimed to
  close

**Engineering decisions**
- certification closeout is a first-class milestone phase, not cleanup
- milestone closure requires hostile proof bundles, not only production
  feature presence

**Open questions**
- exact certification bundle/file grouping once implementation lands

## Must Ship

- explicit binding authority for face, edge, coedge, and vertex bindings
- explicit binding identity separate from topology and naming identity
- carrier-local point and direction anchors with explicit ownership
- typed binding completeness rules
- canonical identity and digest truth for binding, anchor, and rebinding
- motion-aware binding posture
- local replacement rebinding semantics
- typed rebinding outcomes and continuity diagnostics
- one admitted narrow curved pressure surface
- one admitted asymmetric curved family
- historical and branch-local inspection
- replay-safe retained histories
- kernel-to-spatial workflow seams that stay orchestration-only
- hostile certification proving determinism, replay parity, and inspection
  parity

## Must Preserve

- `worth-spatial` as the sole binding/rebinding/history semantic authority
- `worth-topo` as topology truth and navigation authority
- `worth-geom` as math authority rather than binding meaning authority
- `worth-kernel` as workflow composition only
- Query as the canonical declaration/progression/inspection/public workflow
  grammar
- explicit equivalence contracts for identity, replay, and retained inspection
- locality-bounded rebinding evaluation
- typed denial instead of silent fallback or heuristic downgrade

## Acceptance Evidence

- all 16 phases have named production surfaces rather than only tests or notes
- each phase has at least one hostile integration or certification proof that
  directly pressures its adversarial failure mode
- declaration-entry, retained inspection, and replay lanes all converge on the
  same canonical binding truth
- curved and asymmetric pressure demonstrate that planar and symmetric
  shortcuts no longer survive
- kernel ergonomic lanes and generic Query lanes converge to the same artifact
  truth wherever both are admitted

## Architectural Notes

- The milestone should be implemented in the written phase order unless a later
  doc revision explicitly reorders phases. The current order is load-bearing.
- Phases 13 through 16 must consume the substrate closed by phases 1 through
  12. They are not a place to discover missing core semantics retroactively.
- The topology support-query floor is prerequisite substrate for rebinding and
  retained inspection phases, even though it is not a standalone milestone
  phase.
- Grouped/contribution-composed Query families remain out unless one later
  phase proves an admitted workflow cannot be expressed honestly without them.

## Sequencing Notes

- The most dangerous sequencing mistake is to freeze replay, inspection, or
  certification before canonical identity, rebinding truth, and curved
  pressure have actually closed.
- The second most dangerous sequencing mistake is to let kernel or Query
  workflow work proceed by inventing temporary local substitutes for missing
  spatial or retained-history semantics.
- If implementation discovers that one phase depends on undeclared substrate,
  the spec should be updated before code continues rather than silently
  bypassed.
