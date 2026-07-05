# Worth Milestone 7.6: Fragment Classification

> **Status:** Draft
>
> **Purpose:** freeze the canonical fragment-classification products that
> consume the `7.5` overlap-region ledger and feed `7.7` planar face assembly
> without reopening split, loop, overlap, touched graph, Query, readiness,
> selected-route, workload-entry, or overlap-ledger authority.

## Goal

Milestone `7.6` closes the gap between the `7.5` overlap-region ledger and
honest terminal classification plus result-contribution truth for planar B-rep
booleans.

By the end of this milestone:

- a `PlanarBooleanOverlapRegionLedgerReceipt` can enter one and only one
  fragment-classification request boundary
- fragment subjects, overlap-region participation, boundary-only outcomes,
  shared-area outcomes, canonical winding, persistent-name seeds, and
  downstream-consumption identity are carried from the overlap ledger instead
  of rediscovered from raw loops, raw split fragments, raw arrangement cells, or
  pairwise geometry scans
- operand membership, boundary posture, overlap-region posture, operation
  semantics, terminal classification, result contribution sense, ambiguity
  exits, and policy-required denials each have typed, localized,
  replay-stable products
- admitted fragments receive classification identities and downstream assembly
  handoff strong enough for `7.7` face assembly to consume without reopening
  classification meaning
- workload evidence, replay, diagnostics, operator registration, validator
  registration, and public-contract fences prove that classification consumed
  the real `7.5` overlap ledger rather than synthetic fragment rows, raw loop
  walks, raw overlap-chain folklore, or hand-filled terminal outcome labels

Milestone `7.6` does **not** assemble faces, build holes, clean degeneracies,
certify final topology legality, or produce shell/body results. It freezes the
canonical fragment-classification ledger those later phases must consume.

## Why This Milestone Exists

The tempting mistake after `7.5` is to treat classification as a local predicate
pass: test each fragment against the other operand, apply a union/intersection/
subtraction table, and hand kept pieces to assembly.

That is not enough for Worth. Fragment classification is where overlap-region
truth first becomes operation-specific result truth. If this milestone reopens
raw geometry, drops non-overlap or containment-only fragments because the
overlap ledger was treated as overlap-participant-only, lets boundary-only
contact masquerade as kept area, derives subtract sense from incidental operand
order, collapses 1D boundary spans and 2D area regions into one generic
fragment kind, or allows face assembly to inspect local classification folklore
instead of a receipt-backed classification ledger, the planar boolean lane
becomes impossible to certify.

`7.6` therefore makes classification a receipt-backed, phase-typed,
operator-named boundary. It still does not assemble faces or certify final
topology, but it must produce the exact canonical classification ledger that
face assembly can consume without rediscovering inside/outside, boundary,
overlap, terminal outcome, or result-contribution meaning.

## Governing Summaries

- `MENTALITY.md`: protects adversarial-constraint-first design. `7.6` must
  solve the hostile classification problem before face assembly can claim
  progress: fragments near overlap, boundary-only contact, subtract direction,
  ambiguous membership, and benign ordering variation must produce one stable
  ledger or a typed localized denial.
- `arch_laws.md`: protects proof-bearing phase transitions and authority
  separation. Every classification phase must consume the previous proof
  artifact and produce a stronger one; no phase may accept raw fragments,
  generic boolean flags, or local inside/outside summaries when a
  receipt-backed artifact exists.
- `composition_laws.md`: protects semantic decomposition. Classification
  request, subject recovery, support-field construction, boundary posture,
  membership, overlap classification, operation-rule lowering, keep/discard
  assignment, ambiguity handling, identity, diagnostics, and certification must
  remain separate named responsibilities.
- `domain_structure_laws.md`: protects visible ownership. Query remains the
  runtime entry and retained-artifact owner; `worth-kernel` owns workload
  composition and evidence pressure; `worth-spatial` owns planar fragment
  classification semantics and classification-ledger artifacts; `worth-topo`
  remains topology truth and topology-operator authority.
- `perf_laws.md`: protects semantic-delta-bounded execution. Classification
  work must expose fragment, support-field, overlap-region, boundary, rule, and
  ambiguity counters rather than hiding broad operand scans behind cheap-looking
  APIs.
- `_docs/worth/milestone-7-roadmap.md`: protects `7.6` as fragment
  classification only. Face assembly, cleanup, legality, replay closure, and
  planar metaboss closure remain later `7.x` milestones.
- `_docs/worth/touched-graph-roadmap.md`: protects the unified touched graph,
  indexing, aspect, selected-route, replay, conflict, reuse, public-proof, and
  diagnostics architecture. `7.6` must inherit that architecture through the
  `7.5` overlap-ledger receipt instead of rebuilding local touched graph,
  Query, or selected-route proof.
- `_docs/worth/milestone-7.5.md`: protects overlap-ledger authority. `7.6`
  must consume `PlanarBooleanOverlapRegionLedgerReceipt` and must not rebuild
  overlap regions, loop participation, boundary contact, shared-area admission,
  canonical winding, or overlap identity from raw loops or arrangement cells.
- `_docs/forge-query/forge_query_roadmap.md`: protects the Query rule
  `declare query intent once, lower it once, execute it against canonical
  truth`. `7.6` may add Worth classification artifacts, but it must not invent
  a caller-owned query, support, inspection, or runtime route.
- `crates/forge-query/docs/AI_README.md`: protects the rule `declare intent
  once, lower it once, execute or inspect it through canonical runtime-owned
  artifacts`. `7.6` must use Query-owned entry, support, admission,
  projection-consumption, Consumer Kit, and runtime evidence surfaces where
  they apply instead of building local pseudo-Query proof.
- `_docs/worth_topo/operators-list.md`: protects the operator vocabulary that
  already names boolean classification and audit work. `7.6` must bind the
  existing families `ClassifyPlanarBooleanFragmentsKeepDiscard`,
  `ClassifyFacesKeepDiscard`, `MarkKeepFaces`, `MarkDiscardFaces`,
  `ResolvePlanarBooleanClassificationAmbiguity`,
  `CanonicalizeBooleanTraversalOrder`, `RecordBooleanDecisionLog`,
  `EmitPlanarBooleanOutcome`, and `LocalizePlanarBooleanFailure` instead of
  inventing an unrecognizable parallel classifier vocabulary.
- `_docs/worth_topo/validators.md`: protects runtime, parity, determinism,
  receipt-chain, decision-log, persistent-naming, and classification
  validators. `7.6` must extend those validator families with
  fragment-classification-specific proof rather than proving keep/discard rows
  through local assertions.

## Adversarial Constraint

Given a real Query/workload-composed planar boolean operand pair that has
successfully produced a `7.5` overlap-region ledger, fragment classification
must either stage-deny before classification-ledger construction with a typed,
localized, replay-stable reason, or emit one canonical classification ledger.

A successful ledger may contain fragment-local terminal outcomes:
`Keep`, `Discard`, `Ambiguous`, `PolicyExited`, and `LocallyDenied`. These do
not contradict successful ledger construction when they are localized,
typed, and justified by the proof chain.

The canonical classification ledger's request identity, parent overlap route
identity, child classification route identity, complete subject universe,
subject-kind map, support-field and predicate-authority identity, operand
membership outcomes, boundary posture outcomes, overlap-region classification
outcomes, operation-rule plan, terminal outcomes, result contribution sense,
ambiguity exits, policy-required exits, local denials, naming propagation,
counters, decision log, and downstream assembly handoff receipt must remain
stable across replay, benign fragment-order variation, reversed source-edge
sense, boundary-only-versus-area pressure, opposite-sense overlap presentation,
subtract operand-role reversal, nested overlap islands, thin sliver adjacency,
and ambiguous containment pressure.

If `7.7` still has to ask raw loop, raw fragment, overlap-chain, arrangement,
inside/outside, boundary-contact, Query-posture, or pairwise geometry questions
instead of consuming a `7.6` classification ledger receipt, this milestone has
failed.

## Summum Bonum Pressure

The summum bonum test for `7.6` is:

```text
planar_boolean_fragment_classification_metaboss_preserves_operation_truth_across_coplanar_overlap_boundary_and_subtraction_pressure
```

This is the milestone's single hostile closeout program. It must run through
real workload rails and the real `7.5` overlap-ledger receipt, then classify
the same hostile operand pair under union, intersection, A-minus-B, and
B-minus-A.

The workload bundle must combine the catalog pressures that matter to
fragment classification:

- MB-D4-style coplanar faces lying on both boundaries, where boundary-only
  contact must not become kept area
- shared-area overlap with opposite-sense presentation, where canonical
  winding inherited from `7.5` must preserve operation truth
- mixed boundary/area overlap islands, where ambiguous portions localize
  instead of contaminating all fragments
- subtract-role reversal, where A-minus-B and B-minus-A produce different
  rule plans and cannot share a role-erased keep/discard table
- benign fragment and overlap-region ordering variation, where
  `CanonicalizeBooleanTraversalOrder` and classification ordering preserve the
  ledger digest
- checkpoint and replay consumption, where the keep/discard ledger, ambiguity
  exits, classified-fragment identity, and face-assembly handoff remain stable
- anti-theatre bypass attempts, where raw fragment rows, raw overlap-region
  rows, synthetic keep/discard rows, or serialized classification rows cannot
  satisfy the stage
- ordinary boolean completeness pressure: disjoint faces, one face completely
  inside another with no boundary crossing, identical faces with same and
  opposite orientation, edge-touch only, vertex-touch only, A contains B, B
  contains A, A-minus-identical-B empties, holes and inner loops,
  hole-boundary-overlaps-outer-boundary, nested islands, tiny slivers near
  boundaries, duplicate coincident edges, zero-area or near-zero-area
  fragments, and non-overlap containment-dependent classification

Every phase in this spec exists to survive that program. A phase that cannot
explain which part of the metaboss pressure it closes is suspect.

## Product Decision Lock

- `7.6` starts from `PlanarBooleanOverlapRegionLedgerReceipt` and nowhere else.
- That is only valid because `7.5` must expose a complete classification
  universe through its receipt chain. If the overlap ledger cannot prove all
  split fragment subjects, non-overlap subjects, containment-only subjects,
  untouched loop/region carry-forward subjects, hole/inner-loop boundary
  subjects, disjoint subjects, and overlap participants, `7.6` must stage-deny
  before request construction.
- Query continues to own declaration, admission, support posture, runtime
  handles, retained artifact progression, receipts, envelopes, inspection, and
  ordinary outcomes.
- The `7.5` overlap ledger is not merely geometry input. It is the carrier of
  Milestone 16 selected-route, touched-closure, representative-family,
  Query-posture, residue, source-firewall, architecture-claim, replay,
  conflict, reuse, public-proof, and diagnostics readiness. `7.6` must carry
  those identities forward rather than summarize or recompute them.
- `worth-kernel` owns workload composition, stage requirements, evidence rows,
  catalog hostile recipes, public anti-theatre fences, and closeout pressure.
- `worth-spatial` owns classification request semantics, fragment subject
  recovery, membership and boundary posture, overlap classification,
  operation-specific terminal classification, result contribution sense,
  classification identities, diagnostics, counters, and replay identity.
- `worth-topo` owns topology truth, topology operator public surfaces, and the
  eventual execution of authoritative face, shell, and body rewrites. `7.6`
  prepares classified assembly inputs; it does not mutate final topology truth.
- Every branch and outcome that affects classification must be typed and
  traceable. No inside/outside choice, boundary choice, overlap choice,
  operation-rule choice, or ambiguity choice may survive only as inline syntax,
  string messages, or debug-only annotations.
- Persistent naming through classification is admitted as classified-fragment
  lineage and subshape-signature propagation only; final face-level naming,
  merge/conflict semantics, hole identity, shell identity, and body identity
  remain later roadmap work.
- `7.6` uses regularized planar 2D face boolean semantics. Boundary-only or
  vertex-only contacts are not area results. Lower-dimensional contacts may be
  preserved as diagnostic/contact evidence and assembly boundary context only;
  they do not become standalone result geometry in this milestone.
- `Milestone 8` remains EMBER. `7.6` stays in the B-rep planar lane.

## Semantic Decisions Locked

`7.6` resolves the following semantic questions up front so implementation does
not discover them ad hoc:

- A classification subject is a typed contribution candidate, not a generic
  fragment bag.
- The subject universe must be complete before classification starts.
- The milestone classifies both 1D boundary/span subjects and 2D area/region
  subjects, but their terminal outcomes and contribution rules are distinct.
- Keep/discard is not enough for `7.7`; each terminal result must carry result
  contribution sense.
- Stage-level denial prevents ledger construction. Fragment-local terminal
  outcomes may be recorded inside a successful ledger when they are localized
  and do not invalidate the request as a whole.
- Parent `7.5` route identity and child `7.6` classification route identity
  are distinct and linked by receipt lineage.

Subject kinds are:

- `BoundarySpan`
- `AreaRegion`
- `SharedAreaRegion`
- `BoundaryOnlyContact`
- `MixedBoundaryAreaRegion`
- `SourceLoopCarryForward`
- `ContainmentOnlyRegion`
- `DisjointCarryForwardRegion`
- `HoleBoundaryCarryForward`

Terminal outcomes are:

- `Keep`
- `Discard`
- `Ambiguous`
- `PolicyExited`
- `LocallyDenied`

Result contribution kinds are:

- `AreaContribution`
- `BoundaryContribution`
- `ContactEvidenceOnly`
- `DiagnosticOnly`

Contribution sense must include:

- result side: `InteriorLeft`, `InteriorRight`, or `None`
- orientation action: `Preserve`, `Reverse`, `Canonicalize`, or
  `NotApplicable`
- source role: `FromA`, `FromB`, `FromSharedOverlap`, or `FromCarryForward`
- operation role: `Union`, `Intersection`, `AMinusB`, or `BMinusA`

## Canonical Fragment Classification Rule Table

The operation-rule plan must lower to this canonical table before terminal
classification assignment. The table is intentionally regularized: area result
truth is 2D; lower-dimensional contacts are contact or boundary contribution
evidence unless the operation row explicitly preserves them as assembly
boundary context.

| Subject state | Union | Intersection | AMinusB | BMinusA |
| --- | --- | --- | --- | --- |
| A area interior outside B | `Keep AreaContribution FromA Preserve` | `Discard` | `Keep AreaContribution FromA Preserve` | `Discard` |
| B area interior outside A | `Keep AreaContribution FromB Preserve` | `Discard` | `Discard` | `Keep AreaContribution FromB Preserve` |
| area inside both | `Keep AreaContribution FromSharedOverlap Canonicalize` | `Keep AreaContribution FromSharedOverlap Canonicalize` | `Discard` | `Discard` |
| shared area same orientation | `Keep AreaContribution FromSharedOverlap Canonicalize` | `Keep AreaContribution FromSharedOverlap Canonicalize` | `Discard` | `Discard` |
| shared area opposite orientation | `Keep AreaContribution FromSharedOverlap Canonicalize` | `Keep AreaContribution FromSharedOverlap Canonicalize` | `Discard` | `Discard` |
| A boundary contributing to union exterior | `Keep BoundaryContribution FromA Preserve` | `Discard` unless also bounds intersection area | `Keep BoundaryContribution FromA Preserve` | `Discard` |
| B boundary contributing to union exterior | `Keep BoundaryContribution FromB Preserve` | `Discard` unless also bounds intersection area | `Discard` | `Keep BoundaryContribution FromB Preserve` |
| B boundary bounding A-minus-B cut | `Discard` unless union exterior | `Discard` unless also bounds intersection area | `Keep BoundaryContribution FromB Reverse` | `Discard` |
| A boundary bounding B-minus-A cut | `Discard` unless union exterior | `Discard` unless also bounds intersection area | `Discard` | `Keep BoundaryContribution FromA Reverse` |
| boundary-only edge contact | `ContactEvidenceOnly` | `ContactEvidenceOnly` | `ContactEvidenceOnly` | `ContactEvidenceOnly` |
| vertex-only contact | `ContactEvidenceOnly` | `ContactEvidenceOnly` | `ContactEvidenceOnly` | `ContactEvidenceOnly` |
| mixed boundary/area region | lower per area cells and boundary spans; unresolved coupling becomes `Ambiguous` | lower per area cells and boundary spans; unresolved coupling becomes `Ambiguous` | lower per area cells and boundary spans; unresolved coupling becomes `Ambiguous` | lower per area cells and boundary spans; unresolved coupling becomes `Ambiguous` |
| containment-only A inside B | `Keep AreaContribution FromA/FromB per exterior ownership Canonicalize` | `Keep AreaContribution FromA Canonicalize` | `Discard` | `Keep AreaContribution FromB Preserve` with A boundary as reversed cut context |
| containment-only B inside A | `Keep AreaContribution FromA/FromB per exterior ownership Canonicalize` | `Keep AreaContribution FromB Canonicalize` | `Keep AreaContribution FromA Preserve` with B boundary as reversed cut context | `Discard` |
| disjoint A/B areas | keep both as area contributions | discard both | keep A only | keep B only |
| identical faces same orientation | keep one canonical shared area contribution | keep one canonical shared area contribution | discard all area | discard all area |
| identical faces opposite orientation | keep one canonical shared area contribution | keep one canonical shared area contribution | discard all area | discard all area |

Any row that cannot identify subject kind, source role, operation role, result
side, and orientation action must produce `Ambiguous`, `PolicyExited`, or
`LocallyDenied`; it may not default to keep or discard.

## Unified Architecture Carry-Forward

`7.6` is a consumer of the unified touched graph architecture built before
`7.5`. It may add classification products, but it may not create a local
classification routing world beside the semantic-graph routing model.

The classification request and ledger must therefore preserve two route levels:

- parent overlap route identity, proving the `7.5` overlap-ledger lineage
- child classification route identity, proving the `7.6` registered family
  binding and classification execution

The parent route carry-forward includes:

- parent selected-route identity
- parent selected family identity
- parent selected product identity
- parent selected witness identity
- touched-closure digest
- overlap identity digests
- topology and spatial Query posture digests
- residue digest
- source-firewall digest
- architecture-claim digest
- overlap-ledger receipt identity
- replay/checkpoint/counter identity inherited from the overlap lane where it
  affects classification proof

The child route identity includes:

- classification selected family identity
- classification selected product identity
- classification selected witness identity
- classification registered family binding digest
- classification operation-profile digest
- classification route/execution receipt identity

The classification family catalog must declare applicability once against the
inherited touched closure, overlap-region product, operation profile, and
classification stage. Validators, invariants, replay, diagnostics, evidence,
public proof, ambiguity exits, and operation-rule lowering must route from that
registered family binding.

Forbidden substitutes:

- local selected-route summaries
- copied touched-closure digests
- stringly Query posture or support rows
- operator-local validator lists
- executor-local diagnostic choreography
- raw overlap-region or fragment rows acting as route proof
- cache/reuse identity derived from row count, operator family, pointer
  identity, or display shape

If a later engineer cannot trace classification proof back to the same
semantic-graph route chain that admitted `7.5`, this milestone has regressed
from unified architecture to scoped local refactoring.

## Artifact Ladder

`7.6` should be implemented as an explicit artifact ladder, not as one broad
"classify fragments" procedure.

Input:
- `PlanarBooleanOverlapRegionLedgerReceipt`

Request:
- `PlanarBooleanFragmentClassificationRequest`
- `PlanarBooleanFragmentClassificationOperationProfile`
- `PlanarBooleanFragmentClassificationSemanticGraphCarryForward`
- `PlanarBooleanFragmentClassificationRegisteredFamilyBinding`

Subjects:
- `PlanarBooleanClassificationFragmentUniverseReceipt`
- `PlanarBooleanClassificationFragmentSubjectSet`
- `PlanarBooleanClassificationSubjectKindMap`
- `PlanarBooleanClassificationFragmentProvenanceMap`
- `PlanarBooleanOverlapRegionClassificationBinding`

Support fields:
- `PlanarBooleanClassificationSupportField`
- `PlanarBooleanClassificationPredicateAuthority`
- `PlanarBooleanFragmentOperandMembershipField`
- `PlanarBooleanFragmentBoundaryPostureField`

Overlap and boundary:
- `PlanarBooleanOverlapFragmentClassificationSet`
- `PlanarBooleanBoundaryOnlyFragmentOutcomeSet`
- `PlanarBooleanSharedAreaFragmentOutcomeSet`
- `PlanarBooleanMixedBoundaryAreaFragmentOutcomeSet`

Operation lowering:
- `PlanarBooleanBooleanOperationRulePlan`
- `PlanarBooleanUnionFragmentRuleSet`
- `PlanarBooleanIntersectionFragmentRuleSet`
- `PlanarBooleanSubtractionFragmentRuleSet`

Classification products:
- `PlanarBooleanFragmentClassificationCandidateSet`
- `PlanarBooleanDeniedFragmentClassificationSet`
- `PlanarBooleanFragmentTerminalOutcomeSet`
- `PlanarBooleanResultContributionSet`
- `PlanarBooleanKeepAreaContributionSet`
- `PlanarBooleanKeepBoundaryContributionSet`
- `PlanarBooleanDiscardedSubjectSet`
- `PlanarBooleanAmbiguousFragmentClassificationSet`
- `PlanarBooleanFragmentClassificationPolicyExitSet`

Identity and naming:
- `PlanarBooleanClassifiedFragmentIdentityMap`
- `PlanarBooleanClassifiedFragmentPersistentNamePropagationMap`
- `PlanarBooleanClassifiedFragmentSubshapeSignatureMap`
- `PlanarBooleanClassifiedFragmentNamingSeedPayload`

Handoff:
- `PlanarBooleanFaceAssemblyInputHandoff`
- `PlanarBooleanFaceAssemblyInputHandoffReceipt`

Ledger:
- `PlanarBooleanFragmentClassificationDecisionLog`
- `PlanarBooleanFragmentClassificationLedger`
- `PlanarBooleanFragmentClassificationLedgerReceipt`

Closeout:
- `PlanarBooleanFragmentClassificationEvidenceReceipt`
- `PlanarBooleanFragmentClassificationStageRequirement`
- `PlanarBooleanFragmentClassificationReplayParityReceipt`
- `PlanarBooleanFragmentClassificationCheckpointParityReceipt`
- `PlanarBooleanFragmentClassificationPublicContractFenceProof`
- `PlanarBooleanFragmentClassificationAntiTheatreFenceProof`
- `PlanarBooleanFragmentClassificationSummumBonumCloseout`

## Classification Ledger Exclusivity Law

After `7.6`, every `7.7+` planar boolean phase must consume
`PlanarBooleanFragmentClassificationLedgerReceipt` and must not consume raw
split fragments, raw loops, raw overlap chains, raw overlap-region rows,
caller-owned inside/outside tests, or host-local keep/discard summaries as
substitutes for classification truth.

`7.7+` may consume sealed geometry handles referenced by classified-fragment
identities and the ledger-minted face assembly handoff receipt. It may not use
those handles to recompute inside/outside, boundary posture, overlap
participation, operation semantics, terminal classification, or result
contribution truth.

## Existing Surface Inventory

Milestone `7.6` should widen live surfaces before inventing new ones:

- `crates/worth-spatial/src/facade/planar_boolean_overlap_region_extraction.rs`
  - `PlanarBooleanOverlapRegionLedgerReceipt`
  - overlap request, participation, adjacency, arrangement, containment,
    winding, overlap-region, naming, decision-log, replay, and evidence
    surfaces from `7.5`
- `crates/worth-spatial/src/workload_platform/planar_boolean_overlap_region_extraction/*`
  - the parallel overlap-region lane and its phase-typed products
- `crates/worth-spatial/src/workload_platform/planar_boolean_loop_reconstruction/*`
  - loop reconstruction products carried into `7.5` and downstream identity
    only through the overlap ledger
- `crates/worth-spatial/src/workload_platform/planar_boolean_edge_splitting/*`
  - split fragment and overlap-chain provenance carried into `7.5` and
    downstream identity only through the overlap ledger
- `crates/worth-spatial/src/workload_platform/evidence_ledger/*`
  - `BooleanEvidenceReceipt`
  - `BooleanEvidenceStageKind`
  - `CompleteWorkloadEvidenceLedger`
  - typed stage-index products from prior boolean milestones
- `crates/worth-kernel/src/workload_composition/planar_boolean_overlap_region_extraction/*`
  - overlap evidence, stage requirement, replay, public-contract, closeout, and
    handoff products from `7.5`
- `crates/worth-kernel/src/workload_composition/*`
  - stage requirements, workload evidence requirements, catalog hostile
    recipes, and public anti-theatre fences
- `crates/worth-topo/src/topology_operators/*`
  - topology declaration and validator registration precedent
- `crates/forge-query/docs/AI_README.md`
  - ordinary domain work starts at Query
  - intent must be declared once, lowered once, and executed or inspected
    through canonical runtime-owned artifacts
  - graph/index views, projection consumption, support posture, Consumer Kit
    proof, and lower-runtime boundary envelopes are proof boundaries, not local
    convenience helpers

New `7.6` surfaces are allowed where existing surfaces cannot honestly express:

- a proof-bearing classification request from an overlap-ledger receipt
- operation-profile admission for union, intersection, and subtraction without
  deriving semantics from incidental operand order
- classification fragment subjects recovered from overlap-ledger products
- operand membership, boundary posture, and overlap classification products
- operation-rule lowering and keep/discard outcomes
- typed ambiguity and policy-required denial outcomes
- one canonical fragment-classification ledger receipt consumed by `7.7`
- classification evidence rows and public-contract fences against synthetic
  keep/discard proof

## Parallel Folder Cutover Target

Build `7.6` as a parallel fragment-classification lane. Do not refactor the
`7.5` `planar_boolean_overlap_region_extraction` lane in place, do not move
Query or touched-graph readiness code into classification, and do not add local
Query, selected-route, overlap-region, loop, split, or support-posture helpers
under the classification folders.

The intended directory skeleton is:

```text
crates/worth-spatial/src/workload_platform/planar_boolean_fragment_classification/
  mod.rs
  request_boundary/
  operator_surface/
  subject_recovery/
  support_field/
  boundary_posture/
  operand_membership/
  overlap_classification/
  operation_rules/
  keep_discard/
  ambiguity/
  identity_naming/
  classification_ledger/
  replay_closeout/
  closeout/

crates/worth-kernel/src/workload_composition/planar_boolean_fragment_classification/
  mod.rs
  evidence.rs
  runtime_registration.rs
  public_contract.rs
  replay_authority.rs
  handoff.rs
  closeout.rs
```

The skeleton is a target responsibility map, not permission to create empty
folders. Each folder must own a real phase product or stay absent.

## Query, Graph, And Invariant Integration Contract

`7.6` must distinguish three different things that are easy to collapse:

- spatial classification products
  - prepared artifacts owned by `worth-spatial`
  - examples: classification request, subject set, support field, membership
    outcomes, boundary posture, operation-rule plan, keep/discard outcomes,
    ambiguous classifications, classification ledger
  - these do not mutate topology truth by themselves
- topology assembly declarations
  - future Query-declared topology mutation intent owned by `worth-topo`
  - examples: face assembly, hole assembly, loop attachment, face birth, face
    deletion, face orientation, and result topology contribution families in
    `7.7+`
  - these must consume `PlanarBooleanFragmentClassificationLedgerReceipt`; they
    must not consume raw classification rows
- validators and legality rules
  - domain invariant meaning owned by Worth/topology/spatial semantics
  - classification validators must attach to classification-ledger admission,
    topology declaration review, grouped contribution composition, or Query
    registered invariant denial posture rather than executor-local reminder
    code

This means:

- a new `7.6` operator name is not automatically a public topology mutation
  method
- classification-affecting operators must either remain prepared spatial
  products or become explicit Query/topology declaration families with
  admission, route, receipt, envelope, contribution, and recovery proof
- operation-rule lowering must happen before keep/discard execution; executors
  may not re-decide union, intersection, or subtraction semantics from runtime
  conditionals
- membership and boundary lookup must have first-class products before
  keep/discard assignment consumes them; repeated geometry scans are allowed
  only as hostile baseline fixtures, never as production proof
- classification evidence rows are not validator execution. Evidence proves
  the stage happened; registered/domain validators prove the stage is legal

## Phase Plan

### Phase 1: Fragment Classification Request Boundary

Freeze the only artifact that may enter `7.6`: a request built from the `7.5`
overlap-region ledger receipt and an admitted boolean operation profile.

**Consumes**
- `PlanarBooleanOverlapRegionLedgerReceipt`
- admitted planar boolean operation kind and operand-role identity

**Produces**
- `PlanarBooleanFragmentClassificationRequest`
- `PlanarBooleanFragmentClassificationOperationProfile`
- `PlanarBooleanFragmentClassificationSemanticGraphCarryForward`

**Relevant subsystems**
- `worth-kernel` workload composition
- `worth-spatial` overlap-region extraction and fragment classification
- Query retained artifact progression

**Relevant APIs**
- `PlanarBooleanOverlapRegionLedgerReceipt`
- overlap-region handoff products
- new classification request and operation-profile admission surfaces

**Warnings**
- Do not accept raw split fragments, raw loops, raw overlap regions, raw
  arrangement cells, or copied overlap-ledger fields as substitutes for the
  request artifact.
- Do not derive subtraction direction from list order, fixture names, or
  display labels.
- Do not copy selected-route, touched-closure, Query-posture, residue,
  source-firewall, or architecture-claim digests out of the overlap ledger into
  local route summaries.

**Test requirements**
- Adversarial parity test: the same overlap ledger and operation profile must
  produce the same classification request identity across replay.
- Adversarial rejection test: a synthetic request built from raw fragments,
  copied overlap rows, or an untyped operation string must fail before subject
  recovery.
- Adversarial carry-forward test: a request whose semantic-graph carry-forward
  cannot be proven from the `7.5` overlap-ledger receipt chain must fail before
  operator registration.

**Engineering decisions**
- The operation profile is part of the request identity because union,
  intersection, and subtraction classify the same fragment subjects
  differently.
- The request boundary carries overlap-ledger receipt identity forward; it does
  not reopen `7.5` participation, arrangement, or winding products.
- The semantic graph carry-forward is a proof product, not a diagnostics row;
  later phases consume it instead of reopening touched graph readiness.

**Open questions**
- None.

### Phase 2: Classification Catalog Operator Binding

Bind the existing boolean classification operators to the milestone-specific
proof operators before classification functionality spreads across crates.

**Consumes**
- `PlanarBooleanFragmentClassificationRequest`
- `PlanarBooleanFragmentClassificationSemanticGraphCarryForward`
- milestone-scoped classification operator inventory

**Produces**
- `PlanarBooleanFragmentClassificationOperatorMatrix`
- `PlanarBooleanFragmentClassificationCatalogBinding`

**Relevant subsystems**
- `worth-spatial`
- `worth-topo`
- `worth-kernel`
- boolean operator catalog surfaces

**Relevant APIs**
- topology operator declaration families
- grouped declaration / contribution workflow surfaces
- `ClassifyPlanarBooleanFragmentsKeepDiscard`
- `ClassifyFacesKeepDiscard`
- `MarkKeepFaces`
- `MarkDiscardFaces`
- `ResolvePlanarBooleanClassificationAmbiguity`
- `CanonicalizeBooleanTraversalOrder`

**Warnings**
- Do not let classification operators remain unnamed helpers such as
  `classify`, `filter`, or `apply_boolean_rule`.
- Do not let milestone-specific proof operators replace existing catalog
  operators when the catalog already names the public boolean responsibility.

**Test requirements**
- Adversarial parity test: every existing catalog operator and every new `7.6`
  proof operator must map to exactly one classification class in the operator
  matrix and catalog binding.
- Adversarial rejection test: a classification-affecting operator missing
  catalog binding must fail closeout certification.

**Engineering decisions**
- Treat operator growth as a milestone output, not a summary table after
  implementation.
- Predeclare which phases add prepared spatial products versus topology
  declaration families, grouped programs, or support-gated future mutations.
- The existing catalog operators are the public responsibility names; the
  milestone-specific operators are proof products under those names.

**Open questions**
- None.

### Phase 3: Classification Registered Family And Validator Routing

Register the classification family once against the inherited touched graph
proof so validators, diagnostics, replay, evidence, and ambiguity posture route
from the unified architecture rather than operator-local lists.

**Consumes**
- `PlanarBooleanFragmentClassificationRequest`
- `PlanarBooleanFragmentClassificationSemanticGraphCarryForward`
- `PlanarBooleanFragmentClassificationOperatorMatrix`
- `PlanarBooleanFragmentClassificationCatalogBinding`
- milestone-scoped classification validator inventory

**Produces**
- `PlanarBooleanFragmentClassificationValidatorRegistrationPlan`
- `PlanarBooleanFragmentClassificationRegisteredFamilyBinding`

**Relevant subsystems**
- `worth-spatial`
- `worth-topo`
- `worth-kernel`
- Forge Query graph composition and invariant registration

**Relevant APIs**
- Query invariant registration and Consumer Kit proof surfaces
- touched graph registered family and selected-route carry-forward surfaces
- boolean runtime / parity / audit validator catalog surfaces

**Warnings**
- Do not defer validator registration until closeout; operation semantics and
  denial posture must be designed with the family binding.
- Do not register validators, diagnostics, replay, or evidence as
  classification-local lists. They must route from the registered family binding
  over the inherited touched-closure and overlap-product proof.

**Test requirements**
- Adversarial parity test: adding the classification registered family once
  must make validators, diagnostics, replay, evidence, and ambiguity posture
  visible to every matching admitted classification request without
  operator-local wiring.
- Adversarial rejection test: a validator, diagnostic, replay, evidence, or
  ambiguity path that is reachable only from a local list and not from the
  registered family binding must fail closeout certification.

**Engineering decisions**
- The registered family binding is the classification-specific continuation of
  touched graph declare-once routing.
- Validator registration is a routing product, not a crate-local test checklist.

**Open questions**
- None.

### Phase 4: Classification Fragment Subject Recovery

Recover the fragment subjects that classification is allowed to classify from
the overlap-ledger receipt.

**Consumes**
- `PlanarBooleanFragmentClassificationRequest`
- `PlanarBooleanOverlapRegionLedgerReceipt`

**Produces**
- `PlanarBooleanClassificationFragmentUniverseReceipt`
- `PlanarBooleanClassificationFragmentSubjectSet`
- `PlanarBooleanClassificationSubjectKindMap`
- `PlanarBooleanClassificationFragmentProvenanceMap`
- `PlanarBooleanOverlapRegionClassificationBinding`

**Relevant subsystems**
- `worth-spatial`
- `worth-kernel`

**Relevant APIs**
- overlap-region ledger receipt
- overlap-region participation, boundary-only, shared-area, canonical winding,
  identity, and naming products

**Warnings**
- Do not recover subjects from raw split fragments or loop walks.
- Do not flatten overlap-region provenance into generic metadata; later phases
  need typed lineage to explain classification.
- Do not accept an overlap ledger that proves only overlap-region participants
  and omits A-only, B-only, containment-only, disjoint, untouched, hole-boundary,
  or overlap-adjacent subjects.

**Test requirements**
- Adversarial parity test: the same overlap ledger must recover the same
  subject set and provenance binding across replay and benign ordering
  variation.
- Adversarial rejection test: dangling, foreign, duplicated, or
  overlap-ledger-unbacked subject references must deny before support-field
  construction.
- Adversarial completeness test: a ledger missing non-overlap,
  containment-only, untouched, hole-boundary, or disjoint carry-forward
  subjects must stage-deny before classification support construction.

**Engineering decisions**
- The subject set is the classification universe. Later phases may classify,
  deny, or mark ambiguity for these subjects, but may not add subjects by
  scanning geometry.
- The binding product records which overlap-region, boundary-only, shared-area,
  and source-lineage facts each subject inherits.
- The subject-kind map separates 1D boundary/span subjects from 2D area/region
  subjects before any operation rule can run.

**Open questions**
- None.

### Phase 5: Classification Support Field Construction

Build the proof-bearing support field that later membership and boundary
classification consume.

**Consumes**
- `PlanarBooleanClassificationFragmentSubjectSet`
- `PlanarBooleanClassificationFragmentProvenanceMap`
- `PlanarBooleanOverlapRegionClassificationBinding`

**Produces**
- `PlanarBooleanClassificationSupportField`
- `PlanarBooleanClassificationPredicateAuthority`

**Relevant subsystems**
- `worth-spatial`
- Query projection-consumption surfaces where materialized facts are consumed

**Relevant APIs**
- overlap-region canonical winding products
- overlap-region arrangement and containment products through the ledger
- projection-consumption receipts where retained facts are already materialized
- predicate/tolerance authority, robust predicate class, and missing-support
  denial surfaces

**Warnings**
- Do not read raw materialization rows or bridge-only helper state where
  projection consumption owns the public lane.
- Do not recompute containment or winding from coordinates when the overlap
  ledger already carries the proof.
- Do not let support-field construction choose a new tolerance, epsilon,
  predicate fallback class, or containment/winding authority.

**Test requirements**
- Adversarial parity test: support-field identity must stay stable across
  replay, checkpoint consumption, and benign subject ordering.
- Adversarial rejection test: support fields built from raw arrangement cells,
  local point-in-polygon helpers, or direct materialization-row reads must fail
  public-contract proof.
- Adversarial predicate-authority test: missing predicate source,
  tolerance/epsilon model, robust predicate class, containment proof provenance,
  winding proof provenance, projection-consumption receipt ids, or
  missing-support denial rows must deny.

**Engineering decisions**
- The support field is derived state, not authority; it must be destroyable and
  rebuildable from the overlap ledger.
- Support-field counters must expose subject count, inherited overlap products,
  projection-consumption receipts, and denied missing-support rows.
- Predicate authority is inherited proof. It is not an implementation setting.

**Open questions**
- None.

### Phase 6: Boundary Posture Classification

Classify fragment boundary posture before inside/outside membership or
operation-specific keep/discard rules can consume it.

**Consumes**
- `PlanarBooleanClassificationSupportField`
- `PlanarBooleanClassificationFragmentSubjectSet`

**Produces**
- `PlanarBooleanFragmentBoundaryPostureField`
- `PlanarBooleanBoundaryOnlyFragmentOutcomeSet`

**Relevant subsystems**
- `worth-spatial`
- topology validator registration surfaces

**Relevant APIs**
- overlap boundary-contact and boundary-only products
- classification boundary-posture validators

**Warnings**
- Do not let boundary-only contact become shared-area classification by
  accident.
- Do not encode boundary posture as a boolean such as `on_boundary`; boundary
  kind, source operand, overlap-region link, and denial posture must remain
  typed.

**Test requirements**
- Adversarial parity test: boundary-only, shared-boundary, and mixed
  boundary/area fragments must classify to stable boundary posture across
  replay and reversed source-edge sense.
- Adversarial rejection test: a fragment claiming area membership solely from a
  boundary-only overlap outcome must deny before operation-rule lowering.

**Engineering decisions**
- Boundary posture is a support product for operation semantics, not an
  operation-specific keep/discard decision.
- Boundary posture denials must localize whether the fault came from missing
  overlap evidence, conflicting inherited products, or unsupported contact
  posture.

**Open questions**
- None.

### Phase 7: Operand Membership Classification

Classify each fragment subject relative to each operand using the support field
and boundary posture.

**Consumes**
- `PlanarBooleanClassificationSupportField`
- `PlanarBooleanFragmentBoundaryPostureField`
- `PlanarBooleanClassificationFragmentSubjectSet`

**Produces**
- `PlanarBooleanFragmentOperandMembershipField`

**Relevant subsystems**
- `worth-spatial`
- Query retained artifact / projection-consumption surfaces

**Relevant APIs**
- overlap-region winding and containment products
- operand identity and role surfaces from workload composition

**Warnings**
- Do not re-run point-in-polygon or segment containment scans as the ordinary
  membership path.
- Do not collapse inside, outside, boundary, overlap-interior,
  overlap-boundary, ambiguous, and unsupported into a binary truth value.

**Test requirements**
- Adversarial parity test: equivalent fragment subjects must receive stable
  operand membership across replay, benign ordering variation, and reversed
  source-edge sense where semantics permit.
- Adversarial rejection test: missing support-field evidence, contradictory
  operand membership, or membership derived from raw coordinate tests must deny
  before overlap classification.

**Engineering decisions**
- Membership outcomes are operand-local and operation-neutral.
- Subtraction must not be handled here; subtract direction belongs to the
  operation-rule plan.

**Open questions**
- None.

### Phase 8: Coplanar Overlap Fragment Classification

Classify fragments that inherit `7.5` overlap-region truth separately from
ordinary non-overlap fragments.

**Consumes**
- `PlanarBooleanFragmentOperandMembershipField`
- `PlanarBooleanFragmentBoundaryPostureField`
- `PlanarBooleanOverlapRegionClassificationBinding`

**Produces**
- `PlanarBooleanOverlapFragmentClassificationSet`
- `PlanarBooleanSharedAreaFragmentOutcomeSet`
- `PlanarBooleanMixedBoundaryAreaFragmentOutcomeSet`

**Relevant subsystems**
- `worth-spatial`
- overlap-region ledger products

**Relevant APIs**
- overlap-region shared-area admission outcomes
- canonical winding outcomes
- mixed boundary/area localization outcomes

**Warnings**
- Do not treat overlap fragments as ordinary inside/outside fragments with
  incidental extra metadata.
- Do not lose opposite-sense normalization when classifying shared-area
  fragments.

**Test requirements**
- Adversarial parity test: shared-area, boundary-only, mixed boundary/area, and
  opposite-sense overlap fragments must classify to stable overlap outcomes
  across replay.
- Adversarial rejection test: overlap fragments whose inherited region,
  canonical winding, or mixed-boundary/area evidence is missing or inconsistent
  must deny before keep/discard assignment.

**Engineering decisions**
- Overlap classification is operation-neutral; it prepares the facts that the
  operation-rule plan consumes.
- Boundary-only outcomes remain explicit products even if the eventual
  operation discards them.

**Open questions**
- None.

### Phase 9: Boolean Operation Rule Lowering

Lower union, intersection, and subtraction semantics into one explicit rule
plan before terminal classification executes.

**Consumes**
- `PlanarBooleanFragmentClassificationOperationProfile`
- `PlanarBooleanFragmentOperandMembershipField`
- `PlanarBooleanFragmentBoundaryPostureField`
- `PlanarBooleanOverlapFragmentClassificationSet`
- `PlanarBooleanFragmentClassificationCatalogBinding`

**Produces**
- `PlanarBooleanBooleanOperationRulePlan`
- `PlanarBooleanUnionFragmentRuleSet`
- `PlanarBooleanIntersectionFragmentRuleSet`
- `PlanarBooleanSubtractionFragmentRuleSet`

**Relevant subsystems**
- `worth-spatial`
- Query admission / ordinary outcome surfaces

**Relevant APIs**
- operation-profile admission surfaces
- classification rule-plan validators
- canonical fragment classification rule table

**Warnings**
- Do not let keep/discard execution re-decide operation semantics from
  conditionals scattered through the executor.
- Do not derive subtraction operand roles from vector position, fixture naming,
  or display ordering.
- Do not produce a rule plan that cannot be traced back to
  `ClassifyPlanarBooleanFragmentsKeepDiscard` and the catalog keep/discard
  selection operators.

**Test requirements**
- Adversarial parity test: the same operation profile and membership field must
  lower to the same rule-plan identity across replay.
- Adversarial rejection test: missing operation profile, swapped subtraction
  operand authority, unsupported operation kind, or rule plans built from
  booleans instead of typed membership/posture products must deny.
- Adversarial catalog-binding test: a lowered rule plan that bypasses
  `ClassifyPlanarBooleanFragmentsKeepDiscard`, `ClassifyFacesKeepDiscard`,
  `MarkKeepFaces`, `MarkDiscardFaces`, or
  `ResolvePlanarBooleanClassificationAmbiguity` must fail closeout.

**Engineering decisions**
- Rule lowering is a planner phase. Keep/discard assignment consumes a lowered
  rule plan and does not perform strategy branching.
- The rule plan must expose which fragment classes are kept, discarded,
  ambiguous, boundary-preserved, or policy-exited per operation.
- The lowered plan must encode the canonical rule table, including regularized
  boundary/contact policy, subject-kind-specific rows, and subtraction
  orientation actions.

**Open questions**
- None.

### Phase 10: Terminal Classification And Result Contribution Assignment

Assign one operation-specific terminal outcome and result contribution posture
to every admitted classification subject.

**Consumes**
- `PlanarBooleanBooleanOperationRulePlan`
- `PlanarBooleanClassificationFragmentSubjectSet`
- `PlanarBooleanFragmentOperandMembershipField`
- `PlanarBooleanFragmentBoundaryPostureField`
- `PlanarBooleanOverlapFragmentClassificationSet`
- `PlanarBooleanClassificationSubjectKindMap`

**Produces**
- `PlanarBooleanFragmentClassificationCandidateSet`
- `PlanarBooleanFragmentTerminalOutcomeSet`
- `PlanarBooleanResultContributionSet`
- `PlanarBooleanKeepAreaContributionSet`
- `PlanarBooleanKeepBoundaryContributionSet`
- `PlanarBooleanDiscardedSubjectSet`
- `PlanarBooleanAmbiguousFragmentClassificationSet`
- `PlanarBooleanDeniedFragmentClassificationSet`

**Relevant subsystems**
- `worth-spatial`
- `worth-kernel` workload composition

**Relevant APIs**
- classification rule-plan executor
- workload evidence and stage requirement surfaces
- result contribution sense and assembly-boundary context surfaces

**Warnings**
- Do not let kept subjects silently omit contribution kind, result side,
  orientation action, source role, or operation role.
- Do not let one generic keep set cover both 1D boundary semantics and 2D area
  semantics.
- Do not discard boundary, overlap, containment-only, or carry-forward subjects
  without a typed rule-plan row explaining why the operation permits discard.

**Test requirements**
- Adversarial parity test: union, intersection, A-minus-B, and B-minus-A must
  produce stable terminal outcome and result contribution sets for the same
  overlap ledger and operation profile across replay.
- Adversarial rejection test: uncovered subjects, duplicate classification,
  conflicting terminal outcomes, missing contribution sense, or operation-rule
  rows missing for a subject kind must deny before policy-exit lowering.
- Adversarial orientation test: A-minus-B and B-minus-A must produce distinct
  contribution sense for cut boundaries and may not share a role-erased
  keep/discard table.

**Engineering decisions**
- Every subject receives exactly one terminal classification posture: keep,
  discard, locally denied, ambiguous, or policy-exited.
- Result contributions are not topology rewrites. They are prepared assembly
  inputs for `7.7` that carry orientation and side semantics so assembly does
  not rediscover operation meaning.

**Open questions**
- None.

### Phase 11: Ambiguity Classification

Localize classification ambiguity before policy exits, identity, naming, or
ledger assembly.

**Consumes**
- `PlanarBooleanFragmentClassificationCandidateSet`
- `PlanarBooleanDeniedFragmentClassificationSet`
- `PlanarBooleanFragmentTerminalOutcomeSet`
- `PlanarBooleanResultContributionSet`

**Produces**
- `PlanarBooleanAmbiguousFragmentClassificationSet`

**Relevant subsystems**
- `worth-spatial`
- Query ordinary outcomes and recovery surfaces

**Relevant APIs**
- structured denial / advisory / violation outcome surfaces
- recovery brief and checked-stop surfaces where applicable

**Warnings**
- Do not flatten ambiguity into discard.
- Do not let ambiguous fragments proceed to identity minting as ordinary kept
  or discarded fragments.

**Test requirements**
- Adversarial parity test: ambiguous containment, unsupported boundary posture,
  and mixed boundary/area uncertainty must localize to the same fragment
  identity and reason across replay.
- Adversarial rejection test: ambiguous fragments that proceed as kept or
  discarded without a terminal ambiguity row must fail before policy-exit
  lowering.

**Engineering decisions**
- Ambiguity is a first-class terminal posture, not a lack of result.

**Open questions**
- None.

### Phase 12: Policy-Required Exit And Recovery Posture

Turn ambiguous or unsupported classification outcomes into typed policy exits
and recovery posture without changing operational classification truth.

**Consumes**
- `PlanarBooleanAmbiguousFragmentClassificationSet`
- `PlanarBooleanDeniedFragmentClassificationSet`
- `PlanarBooleanFragmentClassificationRegisteredFamilyBinding`

**Produces**
- `PlanarBooleanFragmentClassificationPolicyExitSet`

**Relevant subsystems**
- `worth-spatial`
- Query ordinary outcomes and recovery surfaces

**Relevant APIs**
- structured denial / advisory / violation outcome surfaces
- recovery brief and checked-stop surfaces where applicable

**Warnings**
- Do not let policy-required exits become debug strings or test-only labels.
- Do not let recovery surfaces mutate keep/discard truth or route around the
  classification ledger.

**Test requirements**
- Adversarial parity test: the same ambiguity set must produce the same policy
  exit identities and recovery posture across replay.
- Adversarial rejection test: a recovery path that converts ambiguity into
  kept/discarded truth without a typed policy exit must fail.

**Engineering decisions**
- Recovery surfaces may explain next steps, but they must not alter
  operational classification truth.
- Policy exits are terminal classification products that the ledger records.

**Open questions**
- None.

### Phase 13: Deterministic Classification Ordering

Freeze canonical ordering for classification products before identity and
ledger construction.

**Consumes**
- all classification subject, support, posture, membership, overlap, rule,
  terminal outcome, contribution, ambiguity, and policy-exit products

**Produces**
- `PlanarBooleanFragmentClassificationOrderingBasis`

**Relevant subsystems**
- `worth-spatial`
- determinism certification surfaces

**Relevant APIs**
- canonical traversal order surfaces
- `CanonicalizeBooleanTraversalOrder`
- hash stability and tie-breaker validator surfaces

**Warnings**
- Do not let hashmap iteration, source row order, or fixture order choose
  classification product order.
- Do not let diagnostic richness change ordering or ledger identity.

**Test requirements**
- Adversarial parity test: classification ordering must remain stable across
  replay and benign fragment ordering variation.
- Adversarial rejection test: missing tie-breakers, unstable ordering, or
  hash-dependent traversal must fail before identity minting.

**Engineering decisions**
- Deterministic ordering is a proof product consumed by identity and ledger
  assembly, not a formatting concern.

**Open questions**
- None.

### Phase 14: Classification Counters And Complexity Proof

Freeze visible cost accounting for classification after ordering is stable.

**Consumes**
- `PlanarBooleanFragmentClassificationOrderingBasis`
- all classification subject, support, posture, membership, overlap, rule,
  terminal outcome, contribution, ambiguity, and policy-exit products

**Produces**
- `PlanarBooleanFragmentClassificationCounters`
- classification complexity proof rows

**Relevant subsystems**
- `worth-spatial`
- `worth-kernel`
- performance certification surfaces

**Relevant APIs**
- classification counters
- workload-stage performance accounting
- complexity contract registry where applicable

**Warnings**
- Do not hide broad scans behind elapsed-time-only metrics.
- Do not let diagnostic richness change counters or ledger identity.

**Test requirements**
- Adversarial parity test: classification counters must remain stable across
  replay and benign fragment ordering variation.
- Adversarial rejection test: production classification paths that perform
  repeated raw fragment, loop, overlap-region, or operand scans beyond the
  declared support-field boundary must fail complexity proof.

**Engineering decisions**
- Required counters include subjects classified, overlap-region bindings
  consumed, support-field rows consumed, boundary posture rows, membership rows,
  rule-plan rows, kept fragments, discarded fragments, ambiguous fragments,
  policy exits, and denied classifications.
- The production-confidence path is ledger-derived and support-field-backed,
  not raw geometry scan-backed.
- Complexity contract:
  `O(S + R + P + M + B + O)` where `S` is classification subjects, `R` is
  overlap-region bindings, `P` is support-field/projection rows, `M` is
  membership rows, `B` is boundary-posture rows, and `O` is operation-rule rows.
- Forbidden production shapes:
  `O(S * raw_loop_count)`, `O(S * other_operand_edge_count)`, repeated
  point-in-polygon scans after support-field construction, and pairwise
  fragment/fragment rediscovery after subject recovery.

**Open questions**
- None.

### Phase 15: Classified Fragment Identity

Mint classified-fragment identity from terminal classification products.

**Consumes**
- `PlanarBooleanFragmentTerminalOutcomeSet`
- `PlanarBooleanResultContributionSet`
- `PlanarBooleanKeepAreaContributionSet`
- `PlanarBooleanKeepBoundaryContributionSet`
- `PlanarBooleanDiscardedSubjectSet`
- `PlanarBooleanAmbiguousFragmentClassificationSet`
- `PlanarBooleanFragmentClassificationPolicyExitSet`
- `PlanarBooleanFragmentClassificationOrderingBasis`
- `PlanarBooleanFragmentClassificationCounters`

**Produces**
- `PlanarBooleanClassifiedFragmentIdentityMap`
- `PlanarBooleanClassifiedFragmentPersistentNamePropagationMap`
- `PlanarBooleanClassifiedFragmentSubshapeSignatureMap`
- `PlanarBooleanClassifiedFragmentNamingSeedPayload`

**Relevant subsystems**
- `worth-spatial`
- `worth-topo`
- Query contribution workflow surfaces

**Relevant APIs**
- identity minting surfaces
- persistent naming and lineage propagation surfaces
- topology contribution workflow precedent

**Warnings**
- Do not let persistent naming widen into final face identity, hole ownership,
  shell/body ownership, or final topology conflict resolution.
- Do not mint identity from raw keep/discard rows without ordering, counters,
  and terminal posture proof.
- Do not under-carry source role, operation role, result side, orientation
  action, subject kind, source loop, overlap-region, or containment-only
  provenance needed by `7.7` to mint final face names without reopening
  classification semantics.

**Test requirements**
- Adversarial parity test: the same classified fragment sets must produce the
  same classified-fragment identities, persistent names, and subshape
  signatures across replay.
- Adversarial rejection test: duplicate classified-fragment identity, dangling
  name references, or identity rows not backed by terminal classification proof
  must deny.
- Adversarial naming-seed test: a classified fragment whose naming seed payload
  lacks source role, operation role, result side, orientation action, subject
  kind, or source provenance must deny before handoff construction.

**Engineering decisions**
- `7.6` may produce names for classified fragment identity, source loop
  provenance, source overlap-region provenance, operation-profile provenance,
  boundary posture, and assembly seed signatures.
- `7.6` may not decide final face identity, hole identity, shell/body identity,
  or post-cleanup topology name conflict resolution.

**Open questions**
- None.

### Phase 16: Face Assembly Input Handoff

Prepare the only handoff `7.7` may consume.

**Consumes**
- `PlanarBooleanFragmentTerminalOutcomeSet`
- `PlanarBooleanResultContributionSet`
- `PlanarBooleanKeepAreaContributionSet`
- `PlanarBooleanKeepBoundaryContributionSet`
- `PlanarBooleanDiscardedSubjectSet`
- `PlanarBooleanAmbiguousFragmentClassificationSet`
- `PlanarBooleanFragmentClassificationPolicyExitSet`
- `PlanarBooleanClassifiedFragmentIdentityMap`
- `PlanarBooleanClassifiedFragmentPersistentNamePropagationMap`
- `PlanarBooleanClassifiedFragmentSubshapeSignatureMap`

**Produces**
- `PlanarBooleanFaceAssemblyInputHandoff`

**Relevant subsystems**
- `worth-spatial`
- `worth-topo`
- Query contribution workflow surfaces

**Relevant APIs**
- topology contribution workflow precedent
- face assembly admission and handoff surfaces

**Warnings**
- Do not let face assembly inspect raw keep/discard rows without the
  classification-ledger receipt.
- Do not let the handoff include final face identity, hole ownership,
  shell/body ownership, or final topology conflict decisions.

**Test requirements**
- Adversarial parity test: the same classified fragment identities and terminal
  posture sets must produce the same face-assembly handoff across replay.
- Adversarial rejection test: a handoff missing classified-fragment identity,
  terminal posture rows, or policy-exit rows must deny before ledger assembly.

**Engineering decisions**
- The handoff is a prepared spatial product, not a topology mutation.
- `7.7` consumes the handoff only through the classification ledger receipt.

**Open questions**
- None.

### Phase 17: Classification Decision Log And Ledger Assembly

Assemble the canonical fragment-classification ledger and record every typed
branch and downstream-visible outcome that affected classification.

**Consumes**
- `PlanarBooleanFragmentClassificationRequest`
- `PlanarBooleanFragmentClassificationSemanticGraphCarryForward`
- `PlanarBooleanFragmentClassificationRegisteredFamilyBinding`
- `PlanarBooleanFragmentClassificationOperatorMatrix`
- `PlanarBooleanFragmentClassificationCatalogBinding`
- `PlanarBooleanFragmentClassificationValidatorRegistrationPlan`
- `PlanarBooleanClassificationFragmentUniverseReceipt`
- `PlanarBooleanClassificationFragmentSubjectSet`
- `PlanarBooleanClassificationSubjectKindMap`
- `PlanarBooleanClassificationFragmentProvenanceMap`
- `PlanarBooleanOverlapRegionClassificationBinding`
- `PlanarBooleanClassificationSupportField`
- `PlanarBooleanClassificationPredicateAuthority`
- `PlanarBooleanFragmentOperandMembershipField`
- `PlanarBooleanFragmentBoundaryPostureField`
- `PlanarBooleanOverlapFragmentClassificationSet`
- `PlanarBooleanSharedAreaFragmentOutcomeSet`
- `PlanarBooleanMixedBoundaryAreaFragmentOutcomeSet`
- `PlanarBooleanBooleanOperationRulePlan`
- `PlanarBooleanFragmentTerminalOutcomeSet`
- `PlanarBooleanResultContributionSet`
- `PlanarBooleanKeepAreaContributionSet`
- `PlanarBooleanKeepBoundaryContributionSet`
- `PlanarBooleanDiscardedSubjectSet`
- `PlanarBooleanAmbiguousFragmentClassificationSet`
- `PlanarBooleanDeniedFragmentClassificationSet`
- `PlanarBooleanFragmentClassificationPolicyExitSet`
- `PlanarBooleanFragmentClassificationOrderingBasis`
- `PlanarBooleanFragmentClassificationCounters`
- `PlanarBooleanClassifiedFragmentIdentityMap`
- `PlanarBooleanClassifiedFragmentPersistentNamePropagationMap`
- `PlanarBooleanClassifiedFragmentSubshapeSignatureMap`
- `PlanarBooleanClassifiedFragmentNamingSeedPayload`
- `PlanarBooleanFaceAssemblyInputHandoff`

**Produces**
- `PlanarBooleanFragmentClassificationDecisionLog`
- `PlanarBooleanFragmentClassificationLedger`
- `PlanarBooleanFragmentClassificationLedgerReceipt`
- `PlanarBooleanFaceAssemblyInputHandoffReceipt`

**Relevant subsystems**
- `worth-spatial`
- `worth-kernel`
- Query artifact-envelope surfaces

**Relevant APIs**
- classification decision-log and ledger receipt surfaces
- structured failure localization surfaces

**Warnings**
- Do not summarize membership, overlap, operation, ambiguity, terminal outcome,
  contribution, identity, naming, handoff, or counter outcomes without retaining
  typed decision rows.
- Do not let classification-ledger assembly change operational truth based on
  diagnostic richness.
- Do not assemble a ledger whose route, family, product, witness, Query posture,
  residue, source-firewall, or architecture-claim identity is locally derived
  rather than carried from the overlap-ledger receipt chain.

**Test requirements**
- Adversarial parity test: the same admitted classification path must produce
  the same classification decision-log digest and classification-ledger digest
  across replay.
- Adversarial rejection test: missing decision rows, missing identity receipts,
  missing predicate authority, missing policy exits, missing contribution
  sense, missing naming seeds, or ledger rows that cannot be justified from
  prior proof products must deny.
- Adversarial unified-architecture test: ledger assembly must fail if the
  registered family binding, semantic-graph carry-forward, and overlap-ledger
  receipt chain do not certify the same selected-route and touched-closure
  identity.

**Engineering decisions**
- The classification ledger is the only downstream product accepted by `7.7`.
- Every branch and outcome affecting downstream consumption must be represented
  in typed decision-log rows or typed ledger rows.
- The Classification Ledger Exclusivity Law becomes operational here.
- The ledger is also the `7.7` carrier for the unified architecture identities;
  face assembly must not ask the overlap lane, touched graph closeout, or Query
  posture surfaces again.
- The face assembly handoff receipt is minted by ledger assembly. The raw
  handoff product is not an admissible downstream authority by itself.

**Open questions**
- None.

### Phase 18: Workload Evidence And Stage Requirement

Make classification a real workload stage with typed evidence.

**Consumes**
- `PlanarBooleanOverlapRegionLedgerReceipt`
- `PlanarBooleanFragmentClassificationRequest`
- `PlanarBooleanFragmentClassificationSemanticGraphCarryForward`
- `PlanarBooleanFragmentClassificationRegisteredFamilyBinding`
- `PlanarBooleanFragmentClassificationLedgerReceipt`
- `PlanarBooleanFragmentClassificationOperatorMatrix`
- `PlanarBooleanFragmentClassificationValidatorRegistrationPlan`

**Produces**
- `PlanarBooleanFragmentClassificationEvidenceReceipt`
- `PlanarBooleanFragmentClassificationStageRequirement`

**Relevant subsystems**
- `worth-kernel`
- `worth-spatial`
- `worth-topo`
- compile-fail public facade contract surfaces

**Relevant APIs**
- overlap-region ledger receipts
- `BooleanEvidenceReceipt`
- `WorkloadStageRequirement`
- classification-ledger receipts
- workload evidence and stage requirement surfaces

**Warnings**
- Do not accept evidence that proves classification ran but omits the overlap
  ledger receipt, semantic-graph carry-forward, registered family binding, or
  operation profile.
- Do not let manual keep/discard rows, copied classification labels, or raw
  fragment fixtures satisfy workload requirements.

**Test requirements**
- Adversarial parity test: real workload-backed classification should satisfy
  the same stage requirement for the same overlap ledger and registered family
  binding across equivalent workload entry.
- Adversarial rejection test: synthetic classification evidence, raw fragment
  keep/discard rows, bypassed overlap proof, or local route summaries must fail
  the stage requirement.

**Engineering decisions**
- Fragment classification gets its own evidence boundary rather than
  piggy-backing on overlap evidence.
- Evidence proves the stage happened; it does not replace validators or replay.

**Open questions**
- None.

### Phase 19: Replay And Checkpoint Parity

Prove that the canonical classification ledger is replay-safe and
checkpoint-safe.

**Consumes**
- `PlanarBooleanFragmentClassificationEvidenceReceipt`
- `PlanarBooleanFragmentClassificationStageRequirement`
- `PlanarBooleanFragmentClassificationLedgerReceipt`
- `PlanarBooleanFragmentClassificationSemanticGraphCarryForward`
- `PlanarBooleanFragmentClassificationRegisteredFamilyBinding`

**Produces**
- `PlanarBooleanFragmentClassificationReplayParityReceipt`
- `PlanarBooleanFragmentClassificationCheckpointParityReceipt`

**Relevant subsystems**
- `worth-kernel`
- `worth-spatial`
- retained / checkpoint parity surfaces

**Relevant APIs**
- classification-ledger receipts
- replay parity receipts
- retained / checkpoint parity surfaces

**Warnings**
- Do not call replay success closeout if keep/discard outcomes, ambiguity
  outcomes, identity, naming, or semantic-graph carry-forward drifts.
- Do not replay from serialized classification rows without proving the receipt
  chain.

**Test requirements**
- Adversarial parity test: the same workload-backed classification must produce
  the same ledger receipt, keep/discard outcomes, ambiguity exits, identities,
  naming, and handoff across replay and checkpoint consumption.
- Adversarial rejection test: replay/checkpoint paths that require reopening
  raw loops, raw scans, bypassed overlap proof, or local route summaries must
  fail.

**Engineering decisions**
- Replay parity consumes the classification ledger; it does not reconstruct
  classification from raw fragments.
- Checkpoint parity must preserve the registered family binding.

**Open questions**
- None.

### Phase 20: Public Contract And Anti-Theatre Fences

Fence the classification lane against forged products, bypassed ledgers, and
local pseudo-Query proof.

**Consumes**
- `PlanarBooleanFragmentClassificationEvidenceReceipt`
- `PlanarBooleanFragmentClassificationReplayParityReceipt`
- `PlanarBooleanFragmentClassificationCheckpointParityReceipt`
- `PlanarBooleanFragmentClassificationLedgerReceipt`

**Produces**
- `PlanarBooleanFragmentClassificationPublicContractFenceProof`
- `PlanarBooleanFragmentClassificationAntiTheatreFenceProof`

**Relevant subsystems**
- `worth-kernel`
- `worth-spatial`
- `worth-topo`
- compile-fail public facade contract surfaces

**Relevant APIs**
- public facade contract tests
- compile-fail contract tests
- Consumer Kit and source-firewall proof surfaces

**Warnings**
- Do not allow synthetic classification ledgers, raw fragment rows, raw overlap
  rows, or copied route digests to satisfy public contracts.
- Do not treat source-firewall proof as optional documentation.

**Test requirements**
- Adversarial parity test: public contract fences must admit the real
  workload-backed classification receipt chain and reject no legitimate
  registered-family path.
- Adversarial rejection test: synthetic classification ledgers, raw fragment
  keep/discard rows, copied selected-route digests, or local support summaries
  must fail compile-fail/source-firewall proof.

**Engineering decisions**
- Anti-theatre proof belongs before the summum bonum phase.
- Public contracts expose receipts and proof status, not constructors for
  authority.

**Open questions**
- None.

### Phase 21: Summum Bonum Closeout Certification

Run one hostile certification program that proves `7.6` survives the exact
pressure that would break a naive fragment classifier.

**Consumes**
- `PlanarBooleanOverlapRegionLedgerReceipt`
- `PlanarBooleanFragmentClassificationRequest`
- `PlanarBooleanFragmentClassificationSemanticGraphCarryForward`
- `PlanarBooleanFragmentClassificationRegisteredFamilyBinding`
- `PlanarBooleanFragmentClassificationLedgerReceipt`
- `PlanarBooleanFragmentClassificationEvidenceReceipt`
- replay and checkpoint proof from `Phase 19`
- public-contract and anti-theatre fence proof from `Phase 20`

**Produces**
- `PlanarBooleanFragmentClassificationSummumBonumCloseout`

**Relevant subsystems**
- `worth-kernel`
- `worth-spatial`
- workload catalog hostile recipes

**Relevant APIs**
- overlap-region handoff
- classification-ledger receipt
- workload catalog hostile recipes
- classification replay / checkpoint parity surfaces

**Warnings**
- Do not use synthetic classification rows, hand-filled membership fields, or
  local helper fixtures as the primary closeout proof.
- Do not let the metaboss quietly reopen touched graph, Query, split, loop,
  overlap, containment, winding, or face-assembly work.
- Do not let the metaboss pass unless the same registered family binding drives
  operation-rule lowering, validators, diagnostics, evidence, and replay.

**Test requirements**
- Adversarial parity test:
  `planar_boolean_fragment_classification_metaboss_preserves_operation_truth_across_coplanar_overlap_boundary_and_subtraction_pressure`
  must produce stable classification request digest, operation-rule plan
  digest, classification-ledger digest, keep/discard outcomes, ambiguous
  outcomes, classified-fragment identities, and assembly handoff across replay,
  benign ordering variation, and checkpoint consumption, all under the same
  semantic-graph carry-forward and registered family binding.
- Adversarial rejection test: any hostile scenario that attempts to turn
  boundary-only contact into kept area, to reverse subtraction operand roles,
  to collapse mixed overlap fragments, to bypass ambiguity posture, or to
  bypass the overlap ledger receipt with raw fragment data must deny with typed
  localization.

**Engineering decisions**
- The summum bonum closeout test for `7.6` is
  `planar_boolean_fragment_classification_metaboss_preserves_operation_truth_across_coplanar_overlap_boundary_and_subtraction_pressure`.
- The metaboss bundle should contain named hostile subcases, not one opaque
  mega fixture.
- The summum bonum assertion must prove the existing catalog operators are the
  public semantic lane and the `7.6` proof operators are subordinate products,
  not a second classifier.
- The summum bonum assertion must also prove the unified touched graph route
  chain remains sufficient after 7.5; no classification-local route,
  diagnostic, replay, or support lane may appear.

**Open questions**
- None.

## Operator Inventory

Milestone `7.6` closes boolean fragment classification by binding existing
catalog operators to new fragment-classification proof products.

Existing `operators-list.md` operators executed in the `7.6` classification
lane:

- boolean fragment classification / selection operators:
  - `ClassifyPlanarBooleanFragmentsKeepDiscard`
  - `ResolvePlanarBooleanClassificationAmbiguity`
- boolean audit / determinism operators:
  - `CanonicalizeBooleanTraversalOrder`
  - `RecordBooleanDecisionLog`
  - `EmitPlanarBooleanOutcome`
  - `LocalizePlanarBooleanFailure`
  - `ReplayPlanarBoolean`
  - `ComparePlanarBooleanParity`
  - `BuildStructuredBooleanDisagreementReport`
- persistent naming / identity operators:
  - `BuildPersistentNamingSeeds`
  - `PropagatePersistentNamesThroughSplit`, consumed only through prior
    ledger receipts
  - `ExtractStableSubshapeSignatures`
  - `RepairDanglingNameReferences`, as a validator-denial target rather than a
    `7.6` repair action

Existing `operators-list.md` face-level operators declared as `7.7`
dependencies but not executable in `7.6`:

- `ClassifyFacesKeepDiscard`
- `MarkKeepFaces`
- `MarkDiscardFaces`

These face-level operators may appear in dependency declarations and
source-firewall tests. They may not execute until `7.7` consumes
`PlanarBooleanFragmentClassificationLedgerReceipt` and the ledger-minted
`PlanarBooleanFaceAssemblyInputHandoffReceipt`.

New milestone-scoped proof operators needed under those catalog families:

- request and operation-profile proof operators:
  - `ConsumePlanarBooleanOverlapRegionLedger`
  - `DeclarePlanarBooleanFragmentClassification`
  - `AdmitPlanarBooleanFragmentClassification`
  - `BindClassificationToOverlapLedgerReceipt`
  - `AdmitFragmentClassificationOperationProfile`
  - `RejectSyntheticFragmentClassificationEntry`
- subject and support proof operators:
  - `RecoverClassificationFragmentSubjects`
  - `BindClassificationSubjectToOverlapRegion`
  - `BuildClassificationSupportField`
  - `ConsumeClassificationProjectionFacts`
  - `RejectRawGeometryClassificationSupport`
- boundary and membership proof operators:
  - `ClassifyFragmentBoundaryPosture`
  - `ClassifyBoundaryOnlyFragmentOutcome`
  - `ClassifyFragmentOperandMembership`
  - `RejectContradictoryOperandMembership`
  - `RejectBoundaryOnlyContactAsAreaMembership`
- overlap and operation-rule proof operators:
  - `ClassifyCoplanarOverlapFragment`
  - `ClassifySharedAreaFragmentOutcome`
  - `ClassifyMixedBoundaryAreaFragmentOutcome`
  - `LowerUnionFragmentRules`
  - `LowerIntersectionFragmentRules`
  - `LowerSubtractionFragmentRules`
  - `RejectSubtractionOperandRoleDrift`
- terminal outcome, contribution, and ambiguity proof operators:
  - `AssignFragmentTerminalOutcome`
  - `AssignResultContributionSense`
  - `PreserveBoundaryFragmentWhereOperationRequires`
  - `RejectUncoveredClassificationSubject`
  - `ClassifyFragmentClassificationAmbiguity`
  - `EmitFragmentClassificationPolicyExit`
- identity, naming, and handoff proof operators:
  - `MintClassifiedFragmentIdentity`
  - `PropagatePersistentNamesThroughFragmentClassification`
  - `RecordClassifiedFragmentEntityParentage`
  - `BuildFaceAssemblyInputHandoff`
- ledger and evidence proof operators:
  - `RegisterFragmentClassificationGraphInvariantPack`
  - `RecordFragmentClassificationDecisionLog`
  - `LocalizeFragmentClassificationFailure`
  - `BuildStructuredFragmentClassificationFailureReport`
  - `AssemblePlanarBooleanFragmentClassificationLedger`
  - `BuildFragmentClassificationLedgerReceipt`
  - `RequireBooleanFragmentClassificationEvidence`
  - `RegisterBooleanFragmentClassificationStageRequirement`
  - `ReplayPlanarBooleanFragmentClassification`
  - `CompareFragmentClassificationReplayParity`
  - `CompareFragmentClassificationCheckpointParity`
- public-contract fence operators:
  - `RejectSyntheticKeepDiscardRows`
  - `RejectRawFragmentClassificationBypass`
  - `FenceFaceAssemblyToClassificationLedgerReceipt`
  - `ValidateFragmentClassificationValidatorRuntimeRegistration`
  - `RejectSyntheticClassificationLedgerConstruction`

No `7.6` operator may remain an unclassified helper name. No `7.6` proof
operator may replace the existing catalog operator when the catalog already
names the public boolean responsibility.

## Validator Inventory

Milestone `7.6` closes fragment classification by binding existing validator
families to new fragment-classification-specific checks.

Existing `validators.md` validators that must become runtime-visible in the
`7.6` lane:

- boolean runtime / audit validators:
  - `ValidatePlanarBooleanAdmissionClassification`
  - `ValidateBooleanRoutePlanDeterminism`
  - `ValidateBooleanReceiptEnvelopeConsistency`
  - `ValidatePlanarBooleanReplayParity`
  - `ValidatePlanarBooleanCheckpointParity`
  - `ValidateBooleanOutcomeClassificationConsistency`
  - `ValidateBooleanFailureLocalizationConsistency`
  - `ValidateBooleanDecisionLogCoverage`
  - `ValidateBooleanPolicyOutcomeConsistency`
  - `ValidateBooleanDivergenceClassificationConsistency`
- receipt-chain and lineage validators:
  - `RejectSplitLedgerMissingValidationReceipt`, preserved as precedent for
    classification receipt-chain denial
  - `RejectSplitLedgerMissingPersistentNamingReceipt`, preserved as precedent
    for classified-fragment naming receipt denial
  - `RejectSplitLedgerMissingDecisionLogReceipt`, preserved as precedent for
    classification decision-log denial
  - `RejectSplitLedgerForeignProductLineage`, preserved as precedent for
    overlap-ledger and classification-ledger lineage denial
- determinism and naming validators:
  - `ValidateCanonicalOrderingStable`
  - `ValidateHashStabilityAcrossRuns`
  - `ValidateTieBreakerCoverage`
  - `ValidatePersistentNameUniqueness`
  - `ValidateSelectorResolutionDeterminism`
  - `ValidateNoDanglingNameReferences`

New milestone-scoped validators needed under those families:

- request and receipt validators:
  - `ValidatePlanarBooleanOverlapLedgerConsumption`
  - `ValidateFragmentClassificationReceiptEnvelopeConsistency`
  - `ValidateFragmentClassificationLedgerReceiptChain`
  - `RejectClassificationLedgerMissingDecisionLogReceipt`
  - `RejectClassificationLedgerMissingIdentityReceipt`
  - `RejectClassificationLedgerMissingPersistentNamingReceipt`
  - `RejectClassificationLedgerForeignProductLineage`
- subject and support validators:
  - `ValidateClassificationSubjectCoverage`
  - `ValidateClassificationSubjectOverlapBindingConsistency`
  - `ValidateNoDanglingClassificationSubjectReferences`
  - `ValidateClassificationSupportFieldCoverage`
  - `ValidateProjectionConsumptionReceiptsPresent`
  - `ValidateNoRawGeometryClassificationSupport`
- boundary, membership, and overlap validators:
  - `ValidateBoundaryPostureConsistency`
  - `ValidateBoundaryOnlyIsNotAreaMembership`
  - `ValidateOperandMembershipConsistency`
  - `ValidateNoContradictoryOperandMembership`
  - `ValidateOverlapFragmentClassificationConsistency`
  - `ValidateSharedAreaFragmentOutcomeConsistency`
  - `ValidateMixedBoundaryAreaLocalization`
- operation and keep/discard validators:
  - `ValidateOperationRulePlanConsistency`
  - `ValidateUnionFragmentRuleCoverage`
  - `ValidateIntersectionFragmentRuleCoverage`
  - `ValidateSubtractionOperandRoleConsistency`
  - `ValidateSubtractRoleReversalProducesDistinctRulePlan`
  - `ValidateKeepDiscardExhaustiveness`
  - `ValidateNoFragmentKeptAndDiscarded`
  - `ValidateBoundaryOnlyContactDoesNotBecomeKeptArea`
  - `ValidateAmbiguousFragmentPolicyExit`
- identity, naming, diagnostics, and determinism validators:
  - `ValidateClassifiedFragmentIdentityCanonicality`
  - `ValidateClassifiedFragmentPersistentNameUniqueness`
  - `ValidateClassifiedFragmentNameSurvival`
  - `ValidateFaceAssemblyHandoffBackedByLedgerReceipt`
  - `ValidateFragmentClassificationDecisionLogCoverage`
  - `ValidateFragmentClassificationFailureLocalizationConsistency`
  - `ValidateFragmentClassificationReplayParity`
  - `ValidateFragmentClassificationCheckpointParity`
- Query/runtime registration validators:
  - `ValidateFragmentClassificationValidatorRuntimeRegistration`
  - `ValidateFragmentClassificationGraphInvariantPackRegistration`
  - `ValidatePreparedClassificationProductsCannotMutateTopologyTruth`

## Anti-Theatre Standard

For `7.6`, the following count as theatre and are not admitted as closeout
proof:

- constructing `PlanarBooleanFragmentClassificationLedger` directly in tests
- constructing `PlanarBooleanFragmentClassificationRequest` without
  `PlanarBooleanOverlapRegionLedgerReceipt`
- copying overlap-region, loop, split, Query-posture, selected-route, or
  touched-graph digests into local structs without the receipt chain
- bypassing `PlanarBooleanOverlapRegionLedgerReceipt`
- using raw split fragments, raw loops, raw overlap chains, raw arrangement
  cells, or local point-in-polygon tests as classifier input
- asserting keep/discard rows without membership, boundary, overlap, and
  operation-rule evidence
- using fixture-only classified fragment IDs not derived from canonical
  identity inputs
- replaying from serialized classification rows without proving the receipt
  chain
- passing `PlanarBooleanFaceAssemblyInputHandoff` directly to `7.7` without
  `PlanarBooleanFaceAssemblyInputHandoffReceipt` minted by
  `PlanarBooleanFragmentClassificationLedgerReceipt`

Enforcement mechanisms required:

- private constructors for authority-bearing request, universe, terminal
  outcome, contribution, handoff receipt, ledger, and evidence products
- sealed receipts and opaque authority ids
- compile-fail tests for synthetic construction and raw-row substitution
- no serde/public constructors for authority-bearing artifacts
- fixture-only builders behind test-only capabilities
- digest validation through receipt chain, not field equality or copied strings
- source-firewall proof against local selected-route, Query-posture,
  overlap-ledger, or support-summary clones

## Must Ship

- one canonical fragment-classification request boundary from
  `PlanarBooleanOverlapRegionLedgerReceipt`
- one semantic-graph carry-forward product preserving selected-route,
  selected-family, selected-product, selected-witness, touched-closure,
  Query-posture, residue, source-firewall, and architecture-claim identity from
  the `7.5` receipt chain
- one registered family binding that makes classification validators,
  diagnostics, evidence, replay, ambiguity exits, and operation-rule lowering
  route from the inherited touched graph proof instead of operator-local wiring
- one admitted operation profile for union, intersection, A-minus-B, and
  B-minus-A semantics
- one parallel `planar_boolean_fragment_classification` folder lane in
  `worth-spatial`, with workload composition surfaces in `worth-kernel` and no
  in-place refactor of `planar_boolean_overlap_region_extraction`
- first-class complete subject-universe, subject-kind, support-field,
  predicate-authority, boundary-posture, membership, overlap-fragment,
  operation-rule, terminal-outcome, result-contribution, ambiguity, identity,
  naming-seed, handoff-receipt, and ledger products
- typed ambiguity and policy-required exit outcomes
- fragment-classification workload evidence and stage requirement closure
- replay, checkpoint, determinism, complexity, and anti-theatre proof
- one canonical `PlanarBooleanFragmentClassificationLedgerReceipt` that `7.7+`
  must consume

## Must Preserve

- Query remains the ordinary runtime entry and proof boundary.
- `PlanarBooleanOverlapRegionLedgerReceipt` remains the only overlap truth
  accepted by classification.
- Selected-route, touched-closure, Query-posture, residue, source-firewall, and
  architecture-claim identity remain inherited proof, not classification-local
  derivation.
- Registered-family declare-once routing remains the mechanism for validators,
  replay, evidence, diagnostics, ambiguity exits, and public proof.
- `7.6` prepared spatial products do not mutate topology truth.
- Operation-rule lowering precedes terminal outcome and result contribution
  assignment.
- Diagnostic richness cannot change classification truth, counters, ordering,
  or ledger identity.
- Face assembly, cleanup, topology legality, and shell/body result truth remain
  later milestones.

## Acceptance Evidence

- focused positive and hostile tests for every named phase
- overlap-ledger admission proof showing `PlanarBooleanOverlapRegionLedgerReceipt`
  enters the classification request boundary
- semantic-graph carry-forward proof showing the classification request and
  ledger carry the same selected-route, touched-closure, Query-posture,
  residue, source-firewall, and architecture-claim identities as the `7.5`
  receipt chain
- registered-family proof showing classification validators, diagnostics,
  evidence, replay, ambiguity exits, operation-rule lowering, and public proof
  route from one family binding rather than operator-local wiring
- operation-profile proof for union, intersection, A-minus-B, and B-minus-A
- complete classification universe proof covering A-only, B-only,
  containment-only, disjoint, untouched, hole/inner-loop, overlap-adjacent,
  and overlap-participant subjects
- subject-kind proof for boundary spans, area regions, shared-area regions,
  boundary-only contacts, mixed boundary/area regions, source-loop carry
  forward, containment-only regions, disjoint carry-forward regions, and
  hole-boundary carry-forward
- canonical rule-table proof showing regularized 2D semantics and
  contribution-sense output for every admitted operation row
- result-contribution proof showing area/boundary/contact/diagnostic
  contribution kind, result side, orientation action, source role, and
  operation role for every terminal result
- validator registration proof showing classification legality validators deny
  through typed runtime lanes
- operator classification matrix proving no `7.6` operator is an unclassified
  helper
- public-contract and compile-fail proof that synthetic classification entry,
  raw fragment bypass, local overlap clones, in-place overlap-region refactors,
  and synthetic ledger construction are rejected
- workload evidence and stage-requirement proof for classification
- replay and checkpoint parity proof for terminal outcomes, result
  contributions, ambiguity outcomes, classified-fragment identity, naming seed,
  and ledger-minted assembly handoff receipt
- summum bonum test:
  `planar_boolean_fragment_classification_metaboss_preserves_operation_truth_across_coplanar_overlap_boundary_and_subtraction_pressure`
  - named hostile subcases:
    - `boundary_only_contact_does_not_become_kept_area`
    - `shared_area_overlap_classifies_stably_for_all_operations`
    - `a_minus_b_and_b_minus_a_do_not_share_subtraction_role`
    - `mixed_boundary_area_overlap_localizes_ambiguity`
    - `disjoint_faces_preserve_independent_area_contributions`
    - `a_inside_b_without_crossing_classifies_from_containment_subjects`
    - `b_inside_a_without_crossing_classifies_from_containment_subjects`
    - `identical_faces_same_orientation_canonicalize_shared_area`
    - `identical_faces_opposite_orientation_canonicalize_shared_area`
    - `edge_touch_only_is_contact_evidence_not_area`
    - `vertex_touch_only_is_contact_evidence_not_area`
    - `a_minus_identical_b_is_empty_under_regularized_area_semantics`
    - `holes_and_inner_loops_preserve_subject_kind`
    - `hole_boundary_overlap_outer_boundary_localizes_policy_exit`
    - `nested_islands_preserve_subject_universe`
    - `tiny_sliver_near_boundary_uses_predicate_authority`
    - `duplicate_coincident_edges_do_not_duplicate_subjects`
    - `zero_area_fragment_is_locally_denied_or_policy_exited`
    - `non_overlap_containment_dependent_classification_is_not_dropped`
    - `catalog_boolean_classification_operators_drive_the_lane`
    - `boolean_outcome_classification_validators_bind_the_ledger`
    - `semantic_graph_carry_forward_survives_classification`
    - `registered_family_binding_drives_validation_diagnostics_evidence_and_replay`
    - `benign_fragment_order_variation_preserves_ledger_digest`
    - `synthetic_classification_ledger_is_rejected`
    - `raw_fragment_or_raw_overlap_region_bypass_is_rejected`
    - `checkpoint_replay_preserves_keep_discard_and_identity`
    - `classification_storm_uses_support_field_not_pairwise_rediscovery`

## Non-Goals

- planar face assembly
- hole assembly or shell/body result construction
- post-split degeneracy cleanup
- final topology legality certification
- EMBER or non-planar widening
- curved-edge, seam, periodic-surface, or trim-network classification
- final face-level persistent naming or topology merge/conflict resolution
- standalone lower-dimensional result products from boundary-only or
  vertex-only contact

## Sequencing Notes

- Do not start `7.7` face assembly until `7.6` closes with a fragment
  classification ledger receipt that assembly can consume.
- `7.7+` must consume `PlanarBooleanFragmentClassificationLedgerReceipt` as the
  exclusive classification-truth boundary and must not reopen raw fragments,
  loops, overlap regions, inside/outside tests, or keep/discard summaries.
- `7.7+` may consume sealed geometry handles referenced by the classification
  ledger and handoff receipt for assembly mechanics. It may not use those
  handles to recompute subject kind, containment, boundary posture, overlap
  participation, operation rule, terminal outcome, or result contribution
  truth.
- `7.7+` must consume `PlanarBooleanFaceAssemblyInputHandoffReceipt`, not a raw
  `PlanarBooleanFaceAssemblyInputHandoff`, when entering assembly.
- Do not put face assembly, cleanup, or topology legality into `7.6`.
- Do not widen into EMBER here.
- If a Query-owned retained artifact, support, inspection, outcome, or evidence
  boundary is missing, extend the Query-shaped path or mark the classification
  surface blocked rather than inventing a local runtime lane.
- If a touched-graph, selected-route, registered-family, Query-posture,
  diagnostics, replay, conflict, reuse, public-proof, or source-firewall
  identity is needed, consume the inherited receipt chain or registered family
  binding. Do not rederive it in classification.
- If additional hostile recipes are needed, add them through the workload
  catalog. Do not write classification-only synthetic fixtures and call them
  proof.

## Required Self-Check

- Does the milestone solve a real structural problem or just package work
  cosmetically? Yes: it freezes operation-specific fragment truth that face
  assembly depends on.
- Is the adversarial constraint precise and load-bearing? Yes: it requires one
  canonical classification ledger or a typed localized denial for the same
  overlap ledger across replay, operation-role pressure, boundary/area
  pressure, ambiguity pressure, and ordering variation.
- Does the roadmap justify this milestone now? Yes: `7.6` follows `7.5`
  overlap-region extraction and must close before `7.7` face assembly.
- Does the spec preserve crate authority boundaries? Yes: Query owns runtime
  entry and progression, `worth-kernel` owns workload evidence,
  `worth-spatial` owns classification semantics, and `worth-topo` remains
  topology truth.
- Does the spec continue the unified touched graph architecture? Yes:
  classification carries forward selected-route, touched-closure, Query
  posture, residue, source-firewall, and architecture-claim identity from
  `7.5`, and routes validators, diagnostics, evidence, replay, ambiguity, and
  public proof through one registered family binding.
- Are the phases carrying most of the real design information? Yes.
- Is each phase centered on one conceptual detail or boundary? Yes.
- Does each phase contain at least 2 adversarial tests by default? Yes.
- Could a competent engineer map this spec into honest types, modules, and
  tests? Yes.
- Does the milestone belong in this roadmap sequence, or is it out of order?
  It belongs here because classification truth must freeze before face assembly
  can consume it honestly.
