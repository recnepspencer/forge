# Worth Milestone 7.4: Loop Splitting And Loop Reconstruction

> **Status:** Draft
>
> **Purpose:** freeze the canonical loop-level reconstruction products that
> consume the `7.3` split edge-chain ledger and feed `7.5` overlap-region
> extraction without reopening split, event, projection, plane, or
> workload-entry authority.

## Goal

Milestone `7.4` closes the gap between the `7.3` split edge-chain ledger and
honest loop-level rebuilding.

By the end of this milestone:

- a `7.3` split ledger can enter one and only one loop-reconstruction request
  boundary
- split fragments, overlap edge chains, split vertices, persistent-name seeds,
  and decision-log lineage are lowered into loop-level continuation products
  instead of host-local walk folklore
- continuation ambiguity, unsupported branch posture, open-chain residue,
  collapsed loops, role ambiguity, and degenerate-loop outcomes each have
  typed, localized, replay-stable outcomes
- reconstructed loops, born loops, split islands, and role-preserving loops
  receive canonical, replay-stable identities
- loop-level persistent naming, subshape signatures, lineage, decision logs,
  evidence, and downstream-consumption identity are explicit enough for `7.5`
  overlap-region extraction, `7.6` fragment classification, and `7.7` face
  assembly to inherit without reopening split truth
- workload evidence, replay, diagnostics, operator registration, validator
  registration, and public-contract fences prove that loop reconstruction
  consumed the real split ledger rather than synthetic fragment bags, raw
  continuation maps, or hand-filled loop rows

Milestone `7.4` does **not** extract overlap regions, classify fragments,
assemble result faces, clean final topology, or certify final boolean results.
It freezes the canonical loop reconstruction ledger those later phases must
consume.

## Why This Milestone Exists

The tempting mistake after `7.3` is to treat loop reconstruction as a local
graph helper: gather split fragments by vertex, choose the next edge by angle,
walk until the chain closes, and call the result a loop.

That is not enough for Worth. Loop reconstruction is where split truth first
becomes boundary truth. If this milestone loses branch provenance, chooses
continuations by untyped helper heuristics, silently discards residue, lets
closure failure flatten into a generic boolean error, or allows later overlap
or classification work to re-walk raw fragments, the planar boolean lane
becomes impossible to trust.

`7.4` therefore makes loop reconstruction a receipt-backed, phase-typed,
operator-named boundary. It still does not finalize face or shell topology
truth, but it must produce the exact canonical loop ledger that overlap-region
extraction and fragment classification can consume without rediscovering loop
semantics from split geometry.

## Governing Summaries

- `MENTALITY.md`: protects adversarial-constraint-first design. `7.4` must
  solve the hostile continuation and loop-outcome authority problem before
  overlap-region extraction or fragment classification can claim progress.
- `arch_laws.md`: protects proof-bearing phase transitions and authority
  separation. Every loop phase must consume the previous proof artifact and
  produce a stronger one; no phase may accept raw fragments, caller-owned
  continuation decisions, or generic topology bags when a receipt-backed
  artifact exists.
- `composition_laws.md`: protects semantic decomposition. Loop request,
  provenance recovery, continuation indexing, ambiguity admission, seed
  selection, walk assembly, walk outcome classification, role preservation,
  degeneracy posture, naming propagation, diagnostics, and certification must
  remain separate named responsibilities.
- `domain_structure_laws.md`: protects visible ownership. Query remains the
  runtime entry and retained-artifact owner; `worth-kernel` owns workload
  composition and evidence pressure; `worth-spatial` owns planar loop
  reconstruction semantics and loop-ledger artifacts; `worth-topo` remains
  topology truth and topology-operator authority.
- `perf_laws.md`: protects visible breadth and carry-forward proof. Loop work
  must expose continuation, seed, walk, residue, role, and degeneracy counters
  rather than hiding broad graph scans behind cheap-looking APIs.
- `_docs/worth/milestone-7-roadmap.md`: protects `7.4` as loop splitting and
  loop reconstruction only. Overlap-region extraction, fragment
  classification, face assembly, cleanup, and legality remain later `7.x`
  milestones.
- `_docs/worth/milestone-7.1.md`: protects common-plane and local-frame
  reduction. `7.4` must not reselect plane, frame, projection, or precision
  facts.
- `_docs/worth/milestone-7.2.md`: protects event-ledger authority. `7.4` must
  not recompute segment relations or event families.
- `_docs/worth/milestone-7.3.md`: protects split-ledger authority. `7.4` must
  consume `PlanarBooleanSplitEdgeChainLedgerReceipt` and must not rebuild split
  schedules, split fragments, or overlap chains from raw events or raw source
  geometry.
- `_docs/worth_topo/operators-list.md`: protects the operator vocabulary that
  makes loop surgery explicit. `7.4` must name the loop, containment, island,
  role, and anti-theatre operators it adds instead of hiding them behind vague
  "rebuild loops" prose.
- `_docs/worth_topo/validators.md`: protects validator-family closure. `7.4`
  must split validation work across enough phases that continuation, closure,
  role, degeneracy, naming, replay, and anti-theatre proof each get direct
  tests.
- `crates/forge-query/docs/AI_README.md`: protects the rule `declare intent
  once, lower it once, execute or inspect it through canonical runtime-owned
  artifacts`. `7.4` may add domain loop artifacts, but it must not invent a
  caller-owned loop route, pseudo-Query runtime lane, or local support
  posture.

## Adversarial Constraint

Given a real Query/workload-composed planar boolean operand pair that has
successfully produced a `7.3` split edge-chain ledger, loop reconstruction must
either:

- deny before loop-ledger construction with a typed, localized, replay-stable
  reason

or:

- emit one canonical loop reconstruction ledger whose loop request identity,
  source-loop carriers, fragment-continuation index, continuation-policy
  outcomes, walk seeds, walk outcomes, reconstructed loops, born loops, split
  islands, role outcomes, degeneracy outcomes, naming propagation, counters,
  decision log, and downstream-consumption identity remain stable across
  replay, benign input-order variation, reversed source-edge sense, shared
  continuation pressure, imprint-born island pressure, boundary-role ambiguity,
  and degenerate-loop pressure.

If `7.5` still has to ask raw split, fragment, continuation, or branch-choice
questions instead of consuming a `7.4` loop reconstruction ledger receipt, this
milestone has failed.

## Product Decision Lock

- `7.4` starts from `PlanarBooleanSplitEdgeChainLedgerReceipt` and nowhere
  else.
- Query continues to own declaration, admission, support posture, runtime
  handles, retained artifact progression, receipts, envelopes, inspection, and
  ordinary outcomes.
- `worth-kernel` owns workload composition, stage requirements, evidence rows,
  catalog hostile recipes, public anti-theatre fences, and closeout pressure.
- `worth-spatial` owns loop request semantics, provenance recovery,
  continuation indexing, walk assembly, role and degeneracy outcomes,
  loop-level diagnostics, counters, and replay identity.
- `worth-topo` owns topology truth, topology operator public surfaces, and the
  eventual execution of authoritative loop / face / shell rewrites. `7.4` may
  prepare loop-level topology intent and consume topology provenance, but it
  does not finalize face or shell topology truth.
- Every branch and outcome that affects loop reconstruction must be typed and
  traceable. No branch-choice, role-choice, or degeneracy-choice may survive
  only as inline syntax, string messages, or debug-only annotations.
- Persistent naming through loop reconstruction is admitted as loop-level
  lineage and subshape-signature propagation only; final face-level or
  merge/conflict naming semantics remain later roadmap work.
- `Milestone 8` remains EMBER. `7.4` stays in the B-rep planar lane.

## Artifact Ladder

`7.4` should be implemented as an explicit artifact ladder, not as one broad
"rebuild loops" procedure.

Input:
- `PlanarBooleanSplitEdgeChainLedgerReceipt`

Request:
- `PlanarBooleanLoopReconstructionRequest`

Provenance:
- `PlanarBooleanSourceLoopCarrierSet`
- `PlanarBooleanFragmentMembershipMap`
- `PlanarBooleanOverlapChainLineageMap`

Continuation:
- `PlanarBooleanFragmentContinuationIndex`
- `PlanarBooleanContinuationOrderingBasis`
- `PlanarBooleanContinuationOutcomeSet`

Walk:
- `PlanarBooleanCanonicalLoopSeedSet`
- `PlanarBooleanClosedWalkCandidateSet`
- `PlanarBooleanWalkOutcomeSet`
- `PlanarBooleanFragmentConsumptionProof`

Loop candidates:
- `PlanarBooleanLoopCandidateSet`
- `PlanarBooleanDeniedLoopCandidateSet`

Loop products:
- `PlanarBooleanAdmittedReconstructedLoopSet`
- `PlanarBooleanBornLoopSet`
- `PlanarBooleanLoopIslandPartition`
- `PlanarBooleanSourceLoopSplitAttribution`
- `PlanarBooleanLoopRoleOutcomeSet`
- `PlanarBooleanDegenerateLoopOutcomeSet`

Identity and naming:
- `PlanarBooleanLoopIdentityMap`
- `PlanarBooleanLoopPersistentNamePropagationMap`
- `PlanarBooleanLoopSubshapeSignatureMap`

Ledger:
- `PlanarBooleanLoopDecisionLog`
- `PlanarBooleanLoopReconstructionLedger`
- `PlanarBooleanLoopReconstructionLedgerReceipt`

## Loop Ledger Exclusivity Law

After `7.4`, every `7.5+` planar boolean phase must consume
`PlanarBooleanLoopReconstructionLedgerReceipt` and must not consume raw split
fragments, raw continuation maps, raw walk candidates, source-local loop
walks, or host-local loop summaries as substitutes for loop truth.

## Existing Surface Inventory

Milestone `7.4` should widen live surfaces before inventing new ones:

- `crates/worth-spatial/src/facade/planar_boolean_edge_splitting.rs`
  - `PlanarBooleanSplitEdgeChainLedgerReceipt`
  - `PlanarBooleanSplitEdgeChainLedger`
  - split fragment, overlap chain, split vertex, and split naming surfaces
- `crates/worth-spatial/src/workload_platform/planar_boolean_edge_splitting/*`
  - split request, split scope, source-edge carrier recovery, participation
    indexing, point/interval lowering, schedule normalization, split vertex,
    fragment, overlap-chain, persistent naming, decision-log, and split-ledger
    modules
- `crates/worth-spatial/src/workload_platform/evidence_ledger/*`
  - `BooleanEvidenceReceipt`
  - `BooleanEvidenceStageKind`
  - `CompleteWorkloadEvidenceLedger`
  - typed stage-index products from `7.3`
- `crates/worth-kernel/src/workload_composition/boolean_edge_splitting/*`
  - `WorthWorkload::require_boolean_split`
  - split evidence requirement support
- `crates/worth-kernel/src/workload_composition/stage_requirements.rs`
  - `WorkloadStageRequirement`
- `crates/worth-kernel/src/workload_composition/workload_catalog/*`
  - boolean workload catalog recipes and hostile recipe substrate
- `crates/worth-topo/src/topology_operators/*`
  - loop, containment, grouped, and graph-composition operator precedent
- `crates/worth-topo/src/topology_operators/query_workflow/*`
  - Query-shaped topology declaration, progression, route, receipt, envelope,
    grouped, and contribution workflow precedent
- `crates/forge-query/docs/AI_README.md`
  - ordinary domain work starts at Query
  - intent must be declared once, lowered once, and executed or inspected
    through canonical runtime-owned artifacts
  - graph/index views are proof boundaries, not late performance cleanup
- `crates/forge-query/docs/domain-capabilities/invariants/registering-domain-invariants-through-query.md`
  - Query-owned invariant registration and graph-composition domain-invariant
    denial posture

New `7.4` surfaces are allowed where existing surfaces cannot honestly express:

- a proof-bearing loop reconstruction request from a split-ledger receipt
- source-loop carriers and fragment membership recovered from split-ledger
  provenance
- a fragment-continuation index and canonical seed set
- typed continuation-policy outcomes and walk outcomes
- reconstructed-loop, born-loop, island, role, and degeneracy products
- one canonical loop reconstruction ledger receipt consumed by `7.5`
- loop evidence rows and public-contract fences against synthetic loop proof

## Query, Graph, And Invariant Integration Contract

`7.4` must distinguish three different things that are easy to collapse:

- spatial loop products
  - prepared artifacts owned by `worth-spatial`
  - examples: loop request, source-loop carriers, continuation indexes, seed
    sets, walk outcomes, reconstructed loops, role outcomes, degeneracy
    outcomes, loop ledger
  - these do not mutate topology truth by themselves
- topology operator declarations
  - Query-declared topology mutation intent owned by `worth-topo`
  - examples: future consumed forms of `SplitLoop`, `CreateLoop`,
    `DestroyLoop`, `AttachLoop`, `DetachLoop`, `PromoteInnerLoop`,
    `DemoteOuterLoop`, `SetLoopContainment`, or grouped local rewrites
  - these must implement `ForgeQueryDeclarationInput<TopologyQueryDomain>` and
    travel through `TopologyOperatorWorkflowHandleExt` declaration, review,
    progression, route, receipt, envelope, grouped, contribution, and recovery
    surfaces where applicable
- validators and legality rules
  - domain invariant meaning owned by Worth/topology/spatial semantics
  - where they block graph-shaped topology authoring, they must be registered
    or invoked through Query invariant-pack / domain-invariant denial posture
    rather than remembered by loop executors as manual checks

This means:

- a new `7.4` operator name in the spec is not automatically a public mutation
  method
- loop-affecting operators must either:
  - remain prepared loop-ledger artifacts in `worth-spatial`, or
  - become explicit topology declaration families with canonical Query
    declaration entries, support/admission posture, route/receipt/envelope
    proof, and public-contract fences
- graph-shaped same-batch authoring must use Query graph composition or the
  existing topology grouped/contribution workflow, not caller-owned batch
  choreography
- continuation lookup must have a first-class loop-owned index product before
  walk assembly consumes split fragments; repeated fragment scans are allowed
  only as a hostile baseline fixture, never as the production-confidence proof
  path
- validator families must be attached to declared graph shape, loop-ledger
  consumption, or topology operator declaration families so the runtime can
  emit typed denials; "remember to run validator X" is not an admitted plan
- loop evidence rows are not validator execution. Evidence proves the stage
  happened; registered/domain validators prove the stage is legal

## Phase Plan

### Phase 1: Loop Reconstruction Request Boundary

Freeze the only artifact that may enter `7.4`: a request built from the `7.3`
split edge-chain ledger receipt.

**Consumes**
- `PlanarBooleanSplitEdgeChainLedgerReceipt`

**Produces**
- `PlanarBooleanLoopReconstructionRequest`

**Relevant subsystems**
- `worth-kernel` workload composition
- `worth-spatial` planar boolean edge splitting and loop reconstruction
- Query retained artifact progression

**Relevant APIs**
- `PlanarBooleanSplitEdgeChainLedgerReceipt`
- `PlanarBooleanSplitEdgeChainLedger`
- new `PlanarBooleanLoopReconstructionRequest`

**Warnings**
- Do not accept raw split fragments, overlap chains, split vertices, or
  decision-log rows as substitutes for the request artifact.
- Do not add a second loop reconstruction route in `worth-spatial` that the
  kernel cannot evidence.

**Test requirements**
- Adversarial parity test: the same split ledger must produce the same loop
  reconstruction request identity across replay.
- Adversarial rejection test: a synthetic loop request built from raw fragment
  bags or copied receipt fields must fail to enter the reconstruction boundary.

**Engineering decisions**
- The request boundary is a proof transition, not an ergonomic constructor.
- Preserve the `7.3` split-ledger, decision-log, and naming receipt identities
  so later loop evidence rows can point back to real split truth.

**Open questions**
- Should the request artifact live under workload composition or under a
  dedicated loop reconstruction subtree shared by workload composition and
  spatial certification?

### Phase 2: Loop Operator And Validator Blueprint Registration

Freeze the operator classification matrix and validator registration plan before
loop functionality spreads across crates.

**Consumes**
- `PlanarBooleanLoopReconstructionRequest`
- milestone-scoped loop operator inventory
- milestone-scoped loop validator inventory

**Produces**
- `PlanarBooleanLoopOperatorClassificationMatrix`
- `PlanarBooleanLoopValidatorRegistrationPlan`

**Relevant subsystems**
- `worth-topo`
- `worth-kernel`
- `worth-spatial`
- Forge Query graph composition and invariant registration

**Relevant APIs**
- topology operator declaration families
- grouped declaration / contribution workflow surfaces
- Query graph-composition and invariant-pack registration surfaces

**Warnings**
- Do not let loop operators remain unnamed helpers.
- Do not defer validator registration until closeout; the runtime denial surface
  must be designed with the operators, not added after them.

**Test requirements**
- Adversarial parity test: every new `7.4` operator must map to exactly one
  classification class in the operator matrix.
- Adversarial rejection test: a topology-affecting loop operator missing Query
  or validator registration evidence must fail closeout certification.

**Engineering decisions**
- Treat operator growth and validator growth as first-class milestone outputs,
  not summary-table afterthoughts.
- Predeclare which phases add prepared spatial products versus topology
  declaration families or grouped/query programs.

**Open questions**
- None.

### Phase 3: Source Loop And Fragment Provenance Recovery

Recover the source-loop, source-face, fragment-membership, and overlap-chain
lineage that loop reconstruction is allowed to consume.

**Consumes**
- `PlanarBooleanLoopReconstructionRequest`
- `PlanarBooleanSplitEdgeChainLedgerReceipt`

**Produces**
- `PlanarBooleanSourceLoopCarrierSet`
- `PlanarBooleanFragmentMembershipMap`
- `PlanarBooleanOverlapChainLineageMap`

**Relevant subsystems**
- `worth-spatial`
- `worth-topo`
- `worth-kernel`

**Relevant APIs**
- split-ledger fragment and overlap-chain surfaces
- topology provenance identities
- new source-loop carrier products

**Warnings**
- Do not reconstruct source-loop membership from coordinates or display labels.
- Do not flatten inherited loop role, fragment origin, or overlap-chain lineage
  into untyped metadata.

**Test requirements**
- Adversarial parity test: recovered source-loop carriers must preserve exact
  loop, face, fragment, and lineage facts from the same split ledger.
- Adversarial rejection test: foreign, dangling, or coordinate-only fragment
  provenance must deny before continuation indexing.

**Engineering decisions**
- Separate source-loop recovery from walk logic so denial locality stays exact.
- Preserve enough lineage here for later island attribution and role outcomes to
  remain typed.

**Open questions**
- Should loop-role inheritance be partially recovered here or only in the later
  role phase?

### Phase 4: Vertex Continuation Index And Deterministic Ordering

Build the canonical continuation product keyed by split-vertex identity,
fragment sense, and source-loop membership before any walk begins.

**Consumes**
- `PlanarBooleanSourceLoopCarrierSet`
- `PlanarBooleanFragmentMembershipMap`
- `PlanarBooleanOverlapChainLineageMap`

**Produces**
- `PlanarBooleanFragmentContinuationIndex`
- `PlanarBooleanContinuationOrderingBasis`

**Relevant subsystems**
- `worth-spatial`
- `worth-kernel`

**Relevant APIs**
- split vertex identities
- split fragment identities
- new fragment-continuation index and continuation counters

**Warnings**
- Do not allow repeated fragment scans to masquerade as production traversal.
- Do not let hash-map iteration decide continuation ordering.

**Test requirements**
- Adversarial parity test: equivalent fragment neighborhoods must produce the
  same continuation index identity and ordering.
- Adversarial rejection test: missing vertex bindings, duplicated continuation
  slots, or dangling fragment references must deny before policy admission.

**Engineering decisions**
- Continuation lookup is a proof boundary, not a local optimization.
- Ordering must commit to a named basis carried forward into walk seeds and loop
  identities.

**Open questions**
- None.

### Phase 5: Continuation Policy And Ambiguity Admission

Resolve continuation applicability and emit typed continuation outcomes before
walk assembly consumes any branch.

**Consumes**
- `PlanarBooleanFragmentContinuationIndex`
- `PlanarBooleanContinuationOrderingBasis`

**Produces**
- `PlanarBooleanContinuationOutcomeSet`

**Relevant subsystems**
- `worth-spatial`
- `worth-topo`

**Relevant APIs**
- continuation index products
- loop policy outcome kinds
- typed continuation admission and ambiguity products

**Warnings**
- Do not choose a branch and only later record that it was ambiguous.
- Do not hide unsupported branch families inside generic loop-reconstruction
  denial text.
- Do not let "ambiguous" become a junk drawer for unsupported, contradictory,
  dangling, and policy-exited continuation families.

**Test requirements**
- Adversarial parity test: semantically equivalent branch neighborhoods must
  produce the same continuation outcome classification.
- Adversarial rejection test: multiply-admitted, unsupported, or contradictory
  continuation families must deny before seed selection.

**Engineering decisions**
- Branching and branch outcome are distinct artifacts.
- Every continuation decision that affects later reconstruction must already be
  typed at this phase.
- Continuation outcome kinds must be non-overlapping. At minimum, the milestone
  should distinguish:
  - `SingleAdmittedContinuation`
  - `MultiAdmittedContinuationSet`
  - `AmbiguousContinuationDenied`
  - `UnsupportedContinuationFamily`
  - `ContradictoryContinuationEvidence`
  - `DanglingContinuationReference`
  - `PolicyExitedContinuation`

**Open questions**
- Should policy-required branch exits remain a dedicated outcome kind or fold
  into ambiguity with sub-classification?

### Phase 6: Canonical Loop Seed Selection

Choose deterministic walk seeds from admitted continuation products so later
walk assembly never invents starting folklore.

**Consumes**
- `PlanarBooleanContinuationOutcomeSet`

**Produces**
- `PlanarBooleanCanonicalLoopSeedSet`

**Relevant subsystems**
- `worth-spatial`
- `worth-kernel`

**Relevant APIs**
- continuation outcomes
- canonical seed products
- seed counters and identities

**Warnings**
- Do not let scan order or incidental fragment order choose which loop is born
  first.
- Do not mix seed selection with actual walk execution.

**Test requirements**
- Adversarial parity test: equivalent continuation products must yield the same
  canonical seed set regardless of benign input-order variation.
- Adversarial rejection test: seed sets that skip admitted fragments or claim
  disallowed seeds must deny before walk assembly.

**Engineering decisions**
- Separate seed selection from walk execution so loop ordering remains replay
  stable and auditable.
- Seed identity should commit to the same ordering basis used by continuation
  indexing.

**Open questions**
- None.

### Phase 7: Closed-Walk Assembly

Consume admitted continuation outcomes and canonical seeds into proof-bearing
walk candidates.

**Consumes**
- `PlanarBooleanContinuationOutcomeSet`
- `PlanarBooleanCanonicalLoopSeedSet`

**Produces**
- `PlanarBooleanClosedWalkCandidateSet`
- `PlanarBooleanFragmentConsumptionProof`

**Relevant subsystems**
- `worth-spatial`
- `worth-kernel`

**Relevant APIs**
- continuation outcomes
- canonical seed products
- walk candidate products

**Warnings**
- Do not classify walk success or failure inline while still assembling.
- Do not allow fragment reuse, skipped fragments, or open-chain residue to hide
  inside assembled walk candidates.

**Test requirements**
- Adversarial parity test: the same seed and continuation products must yield
  the same fragment-consumption multiset and walk candidate identity.
- Adversarial rejection test: repeated-fragment, missing-fragment, or foreign
  continuation consumption must deny during walk assembly.

**Engineering decisions**
- Walk assembly only builds candidates; later phases decide what each candidate
  means.
- Carry exact fragment-consumption proof forward for closure, island, and
  anti-theatre checks.

**Open questions**
- None.

### Phase 8: Walk Outcome Classification And Closure Validation

Classify each assembled walk candidate as closed, open, self-colliding,
residual, unsupported, or denied before loop artifacts exist.

**Consumes**
- `PlanarBooleanClosedWalkCandidateSet`
- `PlanarBooleanFragmentConsumptionProof`

**Produces**
- `PlanarBooleanWalkOutcomeSet`

**Relevant subsystems**
- `worth-spatial`
- `worth-topo`

**Relevant APIs**
- walk candidate products
- typed walk outcome kinds
- closure-validation products

**Warnings**
- Do not flatten all failed walks into one generic "bad loop" outcome.
- Do not silently drop open-chain residue and still certify closure.

**Test requirements**
- Adversarial parity test: closure classification must remain stable across
  replay and benign source-edge orientation variation.
- Adversarial rejection test: open walks, self-collisions, or residue leaks
  must deny or emit typed non-admitted outcomes before loop identity minting.

**Engineering decisions**
- Closure is an explicit proof boundary, not an emergent property of successful
  walking.
- Every branch outcome that affects downstream consumers must become a typed
  walk outcome here.
- A closed walk is not automatically an admitted loop. This phase stops at
  `ClosedWalkCandidate` and typed walk outcome proof; later phases decide
  whether a closed walk becomes a loop candidate or a denied loop candidate.

**Open questions**
- Should open-chain residue and multi-branch collision be separate outcome
  families or one family with reason subkinds?

### Phase 9: Loop Candidate Promotion And Denial Boundary

Promote closed walks into loop candidates only after closure proof, and emit
typed denied loop candidates when closure alone is insufficient.

**Consumes**
- `PlanarBooleanWalkOutcomeSet`

**Produces**
- `PlanarBooleanLoopCandidateSet`
- `PlanarBooleanDeniedLoopCandidateSet`

**Relevant subsystems**
- `worth-spatial`
- `worth-topo`
- `worth-kernel`

**Relevant APIs**
- closure-validated walk outcomes
- loop candidate and denied loop candidate products
- loop candidate promotion and denial products

**Warnings**
- Do not treat every closed walk as an admitted loop.
- Do not skip the denied-loop-candidate surface and force downstream phases to
  rediscover why a closed walk was not admissible.

**Test requirements**
- Adversarial parity test: the same closed-walk outcomes must yield the same
  loop candidate and denied-loop-candidate identities across replay.
- Adversarial rejection test: role-ambiguous, lineage-contradictory, or
  unsupported closed walks must stop as denied loop candidates before admitted
  loop products exist.

**Engineering decisions**
- The milestone must distinguish:
  - `ClosedWalkCandidate`
  - `LoopCandidate`
  - `AdmittedReconstructedLoop`
  - `DeniedLoopCandidate`
- This boundary protects the system from the false equivalence `closed == loop`.

**Open questions**
- None.

### Phase 10: Imprint-Born Loop Construction And Island Partition

Turn admitted closed walks into reconstructed loops, including loops born from
imprint or overlap neighborhoods and source loops split into multiple islands.

**Consumes**
- `PlanarBooleanLoopCandidateSet`

**Produces**
- `PlanarBooleanAdmittedReconstructedLoopSet`
- `PlanarBooleanBornLoopSet`
- `PlanarBooleanLoopIslandPartition`
- `PlanarBooleanSourceLoopSplitAttribution`

**Relevant subsystems**
- `worth-spatial`
- `worth-topo`
- `worth-kernel`

**Relevant APIs**
- loop candidate products
- reconstructed-loop and island products
- island attribution and source-loop split products

**Warnings**
- Do not treat imprint-born loops as unnamed leftovers.
- Do not lose attribution when one source loop becomes multiple reconstructed
  islands.

**Test requirements**
- Adversarial parity test: the same walk outcomes must produce the same born
  loop and island partition identities across replay.
- Adversarial rejection test: contradictory island attribution, fragment
  leakage across islands, or untracked born loops must deny before role
  preservation.

**Engineering decisions**
- Island partition is a loop-level product in its own right, not a summary
  counter.
- Preserve enough source attribution here for later overlap-region and face
  assembly stages to reason about split ancestry honestly.

**Open questions**
- None.

### Phase 11: Loop Role Preservation And Role Outcome Classification

Preserve inherited outer / inner meaning where it survives and emit typed role
outcomes where it does not.

**Consumes**
- `PlanarBooleanAdmittedReconstructedLoopSet`
- `PlanarBooleanBornLoopSet`
- `PlanarBooleanLoopIslandPartition`
- `PlanarBooleanSourceLoopSplitAttribution`

**Produces**
- `PlanarBooleanLoopRoleOutcomeSet`
- `PlanarBooleanLoopContainmentEvidencePostureSet`

**Relevant subsystems**
- `worth-spatial`
- `worth-topo`

**Relevant APIs**
- reconstructed-loop products
- loop role outcome kinds
- containment-evidence and role provenance surfaces

**Warnings**
- Do not guess outer / inner role from winding alone when inherited provenance
  is available.
- Do not silently coerce born or ambiguous loops into inherited roles.

**Test requirements**
- Adversarial parity test: loops with preserved ancestry must keep the same role
  outcome across replay and benign traversal variation.
- Adversarial rejection test: contradictory role evidence, impossible
  containment posture, or unsupported role ambiguity must emit typed role
  outcomes before degeneracy classification.

**Engineering decisions**
- Role preservation and role outcome classification are separate from loop
  closure.
- Role ambiguity is a first-class output, not a formatting concern.
- `7.4` should classify and record containment evidence posture, not finalize
  containment truth for later face assembly.

**Open questions**
- Should role-preserving born loops become a distinct role outcome or remain in
  the same family with explicit lineage markers?

### Phase 12: Collapsed And Degenerate Loop Classification

Classify zero-area, tiny-cardinality, self-touching, or otherwise poisoned loop
artifacts before the loop ledger is assembled.

**Consumes**
- `PlanarBooleanAdmittedReconstructedLoopSet`
- `PlanarBooleanLoopRoleOutcomeSet`
- `PlanarBooleanLoopContainmentEvidencePostureSet`

**Produces**
- `PlanarBooleanDegenerateLoopOutcomeSet`

**Relevant subsystems**
- `worth-spatial`
- `worth-topo`
- `worth-kernel`

**Relevant APIs**
- reconstructed-loop and role products
- degeneracy policy outcome kinds
- typed degenerate-loop products

**Warnings**
- Do not hide degenerate-loop posture inside later cleanup milestones.
- Do not admit one-edge, two-edge, or zero-area loops without explicit typed
  policy.

**Test requirements**
- Adversarial parity test: the same poisoned loop geometry and topology facts
  must yield the same degeneracy outcome across replay.
- Adversarial rejection test: collapsed, self-touching, or tiny-cardinality
  loops must deny or policy-exit before loop identity minting.

**Engineering decisions**
- Degeneracy posture is part of loop truth, not later cosmetic cleanup.
- Keep degeneracy outcomes distinct from role outcomes so downstream phases know
  exactly what failed.

**Open questions**
- None.

### Phase 13: Loop Identity, Persistent Naming, And Lineage Propagation

Mint canonical loop identities and propagate split-level naming and lineage into
loop-level artifacts.

**Consumes**
- `PlanarBooleanAdmittedReconstructedLoopSet`
- `PlanarBooleanLoopRoleOutcomeSet`
- `PlanarBooleanDegenerateLoopOutcomeSet`
- `PlanarBooleanDeniedLoopCandidateSet`

**Produces**
- `PlanarBooleanLoopIdentityMap`
- `PlanarBooleanLoopPersistentNamePropagationMap`
- `PlanarBooleanLoopSubshapeSignatureMap`

**Relevant subsystems**
- `worth-spatial`
- `worth-kernel`
- `worth-topo`

**Relevant APIs**
- reconstructed-loop, role, and degeneracy products
- persistent naming and subshape-signature surfaces
- loop identity products

**Warnings**
- Do not invent loop identity from display strings, traversal formatting, or
  incidental fragment order.
- Do not let naming propagation outrun typed loop outcome proof.

**Test requirements**
- Adversarial parity test: admitted loops must receive the same identity and
  naming propagation map across replay.
- Adversarial rejection test: missing split-level naming seeds, foreign lineage,
  or dangling name references must deny before loop ledger assembly.

**Engineering decisions**
- Identity and naming must commit to the same deterministic basis as seed and
  walk ordering.
- Loop naming remains reconstruction-local; face-level naming semantics stay in
  later milestones.
- Diagnostics may still need identities for denied or degenerate candidates, so
  the implementation should distinguish candidate-tracking identities from
  canonical admitted loop identities.

**Open questions**
- None.

### Phase 14: Loop Decision Log And Loop Ledger Assembly

Assemble the canonical loop reconstruction ledger and record every typed branch
and outcome that mattered along the path.

**Consumes**
- `PlanarBooleanContinuationOutcomeSet`
- `PlanarBooleanWalkOutcomeSet`
- `PlanarBooleanLoopCandidateSet`
- `PlanarBooleanDeniedLoopCandidateSet`
- `PlanarBooleanAdmittedReconstructedLoopSet`
- `PlanarBooleanBornLoopSet`
- `PlanarBooleanLoopIslandPartition`
- `PlanarBooleanSourceLoopSplitAttribution`
- `PlanarBooleanLoopRoleOutcomeSet`
- `PlanarBooleanDegenerateLoopOutcomeSet`
- `PlanarBooleanLoopIdentityMap`
- `PlanarBooleanLoopPersistentNamePropagationMap`
- `PlanarBooleanLoopSubshapeSignatureMap`

**Produces**
- `PlanarBooleanLoopDecisionLog`
- `PlanarBooleanLoopReconstructionLedger`
- `PlanarBooleanLoopReconstructionLedgerReceipt`

**Relevant subsystems**
- `worth-spatial`
- `worth-kernel`

**Relevant APIs**
- loop identity and naming products
- loop decision-log products
- loop reconstruction ledger receipt

**Warnings**
- Do not summarize continuation, closure, role, or degeneracy outcomes without
  retaining typed decision rows.
- Do not let loop-ledger assembly change operational truth based on diagnostic
  richness.

**Test requirements**
- Adversarial parity test: the same admitted reconstruction path must produce
  the same loop decision-log digest and loop-ledger digest across replay.
- Adversarial rejection test: missing decision rows, missing naming receipts, or
  ledger rows that cannot be justified from prior proof products must deny.

**Engineering decisions**
- The loop ledger is the only downstream product accepted by `7.5`.
- Every branch and outcome affecting downstream consumption must be represented
  in typed decision-log rows or typed ledger rows.
- The Loop Ledger Exclusivity Law becomes operational here: after this phase,
  later milestones must consume `PlanarBooleanLoopReconstructionLedgerReceipt`
  rather than reopening fragments, continuation maps, walk candidates, or
  source-local loop walks.

**Open questions**
- None.

### Phase 15: Workload Evidence, Stage Requirement, And Runtime Registration

Make loop reconstruction a real workload stage with typed evidence and runtime
registration closure.

**Consumes**
- `PlanarBooleanLoopReconstructionLedgerReceipt`
- `PlanarBooleanLoopOperatorClassificationMatrix`
- `PlanarBooleanLoopValidatorRegistrationPlan`

**Produces**
- `PlanarBooleanLoopReconstructionEvidenceReceipt`
- `PlanarBooleanLoopReconstructionStageRequirement`
- `PlanarBooleanLoopRuntimeRegistrationProof`

**Relevant subsystems**
- `worth-kernel`
- `worth-spatial`
- `worth-topo`
- Forge Query runtime support surfaces

**Relevant APIs**
- `BooleanEvidenceReceipt`
- `WorkloadStageRequirement`
- workload catalog recipe and evidence requirement surfaces

**Warnings**
- Do not let manual loop ledgers or copied loop rows satisfy workload
  requirements.
- Do not register operators without matching validator or evidence closure.

**Test requirements**
- Adversarial parity test: real workload-backed loop reconstruction should
  satisfy the same stage requirement through replay and retained consumption.
- Adversarial rejection test: synthetic loop evidence, raw loop rows, or
  missing registration evidence must fail workload admission and public
  contract tests.

**Engineering decisions**
- Loop reconstruction gets its own evidence boundary rather than piggy-backing
  on split evidence.
- Runtime registration proof belongs in the milestone body, not only in closeout
  notes.

**Open questions**
- None.

### Phase 16: Replay, Determinism, And Checkpoint Parity

Prove that the canonical loop ledger is replay-safe, orientation-stable, and
checkpoint-stable where admitted.

**Consumes**
- `PlanarBooleanLoopReconstructionLedgerReceipt`
- `PlanarBooleanLoopReconstructionEvidenceReceipt`

**Produces**
- `PlanarBooleanLoopReplayParityReceipt`
- `PlanarBooleanLoopCheckpointParityReceipt`

**Relevant subsystems**
- `worth-spatial`
- `worth-kernel`

**Relevant APIs**
- loop reconstruction ledger receipts
- replay parity receipts
- retained / checkpoint parity surfaces

**Warnings**
- Do not call replay success closeout if canonical identities drift.
- Do not re-execute earlier split or event phases inside loop replay proof.

**Test requirements**
- Adversarial parity test: the same split ledger must produce the same loop
  ledger, role outcomes, degeneracy outcomes, and decision-log digest across
  replay and benign orientation variation.
- Adversarial rejection test: any replay, checkpoint, or retained-consumption
  path that requires raw fragment rebuilding must fail closeout certification.

**Engineering decisions**
- Determinism is part of loop truth, not a later polishing step.
- Replay proof must certify that downstream loop consumers do not reopen split
  work.
- Replay proof must be phase-local to `7.4`: it should certify loop-ledger
  parity without silently re-executing earlier split, event, or common-plane
  phases inside the asserted `7.4` closeout.

**Open questions**
- None.

### Phase 17: Public Contract, Compile-Fail, And Anti-Theatre Fences

Fence loop reconstruction against synthetic products, raw fragments, and
hand-filled evidence.

**Consumes**
- `PlanarBooleanLoopReconstructionLedgerReceipt`
- `PlanarBooleanLoopReconstructionEvidenceReceipt`
- public facade contract and compile-fail fixtures

**Produces**
- `PlanarBooleanLoopPublicContractFenceProof`
- `PlanarBooleanLoopAntiTheatreFenceProof`

**Relevant subsystems**
- `worth-kernel`
- `worth-spatial`
- compile-fail public facade contract surfaces

**Relevant APIs**
- loop reconstruction request
- loop outcome and loop ledger products
- public facade and trybuild contract surfaces

**Warnings**
- Do not allow raw fragment or walk candidate construction to masquerade as a
  public loop lane.
- Do not let anti-theatre proof rely on local helper visibility rather than
  public contract fences.

**Test requirements**
- Adversarial parity test: public contract surfaces should allow real
  workload-backed loop reconstruction while preserving the same loop-ledger
  identities as crate-local proof.
- Adversarial rejection test: raw fragments, raw walk outcomes, manual role
  outcomes, manual degeneracy rows, and hand-filled loop evidence must fail the
  public boundary.

**Engineering decisions**
- Compile-fail and public contract proof are part of the production path.
- Anti-theatre closure must certify both artifact construction fences and
  downstream consumption fences.
- Anti-theatre proof must explicitly fence:
  - raw fragments
  - raw continuation maps
  - raw walk outcomes
  - manual role outcomes
  - manual degeneracy rows
  - hand-filled loop evidence
  - synthetic loop ledger construction

**Open questions**
- None.

### Phase 18: Summum Bonum Closeout Certification

Close `7.4` only when a real workload-backed metaboss chain proves canonical,
replayable, role-preserving, branch-traceable, and unforgeable loop
reconstruction.

**Consumes**
- `PlanarBooleanLoopReconstructionLedgerReceipt`
- `PlanarBooleanLoopReconstructionEvidenceReceipt`
- `PlanarBooleanLoopReplayParityReceipt`
- `PlanarBooleanLoopPublicContractFenceProof`
- real workload-backed planar boolean metaboss recipe

**Produces**
- `PlanarBooleanLoopSummumBonumCloseoutProofBundle`

**Relevant subsystems**
- `worth-kernel`
- `worth-spatial`
- workload catalog hostile recipes

**Relevant APIs**
- real workload-backed planar boolean metaboss recipe
- loop reconstruction ledger receipt
- closeout proof-bundle support surfaces

**Warnings**
- Do not treat the summum bonum test as optional polish.
- Do not let a happy-path loop example substitute for the hostile chain.

**Test requirements**
- Adversarial parity test:
  `planar_boolean_loop_reconstruction_metaboss_chain_is_canonical_replayable_role_preserving_and_unforgeable`
- Adversarial rejection test:
  `loop_reconstruction_metaboss_rejects_synthetic_loop_ledgers_raw_fragments_and_hand_filled_evidence`

**Engineering decisions**
- The summum bonum closeout must certify the real production chain and not only
  a crate-local helper bundle.
- The closeout proof must assert that every branch and outcome that mattered is
  typed, localized, and recoverable from the loop ledger and decision log.

**Open questions**
- None.

## Admitted Surface

- real `7.3` split edge-chain ledger entry
- source-loop and fragment provenance recovery
- split-fragment continuation indexing
- typed continuation-policy and non-overlapping continuation outcomes
- canonical loop seed selection
- closed-walk assembly and typed walk outcomes
- explicit loop-candidate promotion and denied-loop-candidate posture
- imprint-born loop construction
- source-loop split into multiple reconstructed islands
- preserved outer / inner loop meaning where admitted
- typed role ambiguity and typed degeneracy outcomes
- loop identity minting
- loop-level persistent naming and subshape-signature propagation
- loop decision logs, diagnostics, counters, replay proof, and anti-theatre
  public contracts

## Excluded Surface

- overlap-region island extraction
- coplanar overlap region classification
- fragment inside / outside classification
- keep / discard labeling
- planar face assembly
- post-loop topology cleanup beyond typed loop degeneracy posture
- final topology legality certification
- shell / body result certification
- EMBER execution or B-rep/EMBER parity
- full face-level or merge/conflict persistent naming semantics
- curved-edge, trim-network, seam, or periodic-surface loop reconstruction
  except as explicit support-gated posture

## Workflow Surface

`7.4` is not done because one simple split polygon can be walked into a loop.

It is only done when admitted loop-reconstruction workflows operate generically
over:

- arbitrary admitted source-loop counts produced by workload-catalog-backed
  planar boolean operand pairs
- arbitrary admitted split-fragment counts produced by the real `7.3` split
  ledger
- arbitrary admitted continuation valence at split vertices, including typed
  ambiguity posture
- arbitrary admitted walk counts and island counts per source loop
- arbitrary admitted role-preserving and born-loop combinations
- arbitrary admitted degenerate-loop, open-walk, and unsupported-branch
  outcomes localized to the right phase
- retained replay and reversed-edge-sense variants of the same loop workload
- typed failure for unsupported continuation, closure, role, degeneracy,
  naming, validation, or evidence cases

## Operator Closure

Milestone `7.4` closes the following named families:

- admission operators:
  - `ConsumePlanarBooleanSplitEdgeChainLedger`
  - `DeclarePlanarBooleanLoopReconstruction`
  - `AdmitPlanarBooleanLoopReconstruction`
  - `BindLoopReconstructionToSplitLedgerReceipt`
  - `RejectSyntheticLoopReconstructionEntry`
- operator and validator registration:
  - `RegisterLoopReconstructionOperatorDeclarationFamily`
  - `RegisterLoopReconstructionGroupedOperatorFamily`
  - `RegisterLoopReconstructionContributionWorkflow`
  - `RegisterLoopReconstructionGraphInvariantPack`
  - `ValidateLoopOperatorQueryProgression`
  - `ValidateLoopValidatorRuntimeRegistration`
- prepared spatial products:
  - `RecoverBooleanLoopSourceCarriers`
  - `BindFragmentToSourceLoop`
  - `BuildLoopFragmentContinuationIndex`
  - `CanonicalizeLoopContinuationOrder`
  - `AdmitLoopContinuationPolicy`
  - `ClassifyLoopContinuationAmbiguity`
  - `EmitLoopContinuationOutcome`
  - `SelectCanonicalLoopSeeds`
  - `AssembleClosedWalkCandidates`
  - `ClassifyWalkOutcome`
  - `PromoteClosedWalkToLoopCandidate`
  - `RejectLoopCandidateBeforeIdentity`
  - `BuildReconstructedLoop`
  - `BuildBornLoopFromImprintNeighborhood`
  - `PartitionLoopIslands`
  - `SplitSourceLoopIntoReconstructedIslands`
  - `PreserveLoopRoleFromSource`
  - `ClassifyLoopRoleOutcome`
  - `ClassifyLoopContainmentEvidencePosture`
  - `RecordLoopContainmentEvidence`
  - `ClassifyDegenerateLoopOutcome`
  - `RejectCollapsedLoopBeforeLedger`
  - `RejectTinyCardinalityLoopBeforeLedger`
  - `RejectUnsupportedSelfTouchingLoopOutcome`
  - `MintBooleanLoopIdentity`
  - `PropagatePersistentNamesThroughLoopReconstruction`
  - `RecordLoopEntityParentage`
  - `ForkLoopEntityLineage`
- topology declaration families:
  - `SplitLoop`
  - `CreateLoop`
  - `DestroyLoop`
  - `AttachLoop`
  - `DetachLoop`
  - `PromoteInnerLoop`
  - `DemoteOuterLoop`
  - `SetLoopContainment`
- ledger assemblers and diagnostics:
  - `RecordLoopReconstructionDecisionLog`
  - `LocalizeLoopReconstructionFailure`
  - `BuildStructuredLoopReconstructionFailureReport`
  - `AssemblePlanarBooleanLoopReconstructionLedger`
  - `BuildLoopReconstructionLedgerReceipt`
  - `FenceOverlapExtractionToLoopLedgerReceipt`
- replay and certification checks:
  - `RequireBooleanLoopReconstructionEvidence`
  - `RegisterBooleanLoopReconstructionStageRequirement`
  - `ReplayPlanarBooleanLoopReconstruction`
  - `CompareLoopReconstructionReplayParity`
  - `CompareLoopReconstructionCheckpointParity`
- public-contract fences:
  - `RejectUnindexedLoopFragment`
  - `RejectSyntheticLoopLedgerConstruction`

Every named family in this closure must be classified before implementation
closeout as one of:

- `PreparedSpatialOnly`
- `TopologyDeclarationFamily`
- `TopologyGroupedDeclarationFamily`
- `TopologyContributionWorkflow`
- `QueryGraphCompositionProgram`
- `SupportGatedFutureTopologyMutation`

No operator may remain an unclassified helper name.

## Validator Closure

Milestone `7.4` closes these validator families at loop-reconstruction scope:

- request and lineage validators:
  - `ValidatePlanarBooleanSplitLedgerConsumption`
  - `ValidateLoopReceiptEnvelopeConsistency`
  - `ValidateLoopLedgerReceiptChain`
  - `RejectLoopLedgerMissingDecisionLogReceipt`
  - `RejectLoopLedgerMissingPersistentNamingReceipt`
  - `RejectLoopLedgerForeignProductLineage`
- provenance and continuation validators:
  - `ValidateLoopCarrierCoverage`
  - `ValidateFragmentMembershipCoverage`
  - `ValidateLoopContinuationIndexCoverage`
  - `ValidateNoDanglingLoopFragmentReferences`
  - `ValidateCanonicalContinuationOrderingStable`
  - `ValidateNoNPlusOneLoopContinuationDiscovery`
- branch and walk validators:
  - `ValidateLoopContinuationOutcomeConsistency`
  - `ValidateLoopAmbiguityClassificationConsistency`
  - `ValidateCanonicalLoopSeedSelection`
  - `ValidateClosedWalkFragmentConsumption`
  - `ValidateWalkClosure`
  - `ValidateWalkOutcomeLocalization`
- loop-candidate promotion validators:
  - `ValidateClosedWalkPromotionBoundary`
  - `ValidateDeniedLoopCandidateLocalization`
  - `ValidateClosedIsNotAutomaticallyAdmittedLoop`
- role and degeneracy validators:
  - `ValidateInnerOuterLoopFlagsConsistent`
  - `ValidateLoopRoleOutcomeConsistency`
  - `ValidateLoopContainmentEvidencePostureConsistency`
  - `ValidateDegenerateLoopPolicyConsistency`
  - `ValidateLoopHasMinimumCardinality`
  - `ValidateNoUnexpectedZeroAreaLoops`
- identity, naming, and diagnostics validators:
  - `ValidateLoopIdentityCanonicality`
  - `ValidatePersistentNameUniqueness`, scoped to loop products
  - `ValidateNameSurvivalThroughLoopReconstruction`
  - `ValidateNoDanglingNameReferences`
  - `ValidateLoopDecisionLogCoverage`
  - `ValidateLoopFailureLocalizationConsistency`
- determinism and replay validators:
  - `ValidateCanonicalOrderingStable`
  - `ValidateHashStabilityAcrossRuns`
  - `ValidateTieBreakerCoverage`
  - `ValidatePlanarBooleanLoopReplayParity`
  - `ValidatePlanarBooleanLoopCheckpointParity`
- Query/runtime registration validators:
  - `ValidateLoopOperatorQueryProgression`
  - `ValidateLoopValidatorRuntimeRegistration`
  - `ValidateLoopGraphInvariantPackRegistration`
  - `ValidatePreparedSpatialLoopProductsCannotMutateTopologyTruth`
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

- Add loop reconstruction workload composition under a dedicated kernel path
  such as
  `crates/worth-kernel/src/workload_composition/boolean_loop_reconstruction/`.
- Add loop-stage requirement support for `BooleanLoopReconstruction` if it is
  not already admitted in `WorkloadStageRequirement`.
- Add loop evidence mapping in
  `crates/worth-kernel/src/workload_composition/boolean_evidence_requirement.rs`.
- Add `BooleanEvidenceReceipt` for
  `PlanarBooleanLoopReconstructionLedgerReceipt`.
- Add `WorthWorkload::require_boolean_loop_reconstruction` or equivalent.
- Add workload catalog recipes that produce real loop-hostile operand pairs
  through the existing topology, spatial, projection, transform, replay,
  diagnostics, response, common-plane, event-ledger, and split-ledger rails.
- Add public-contract tests proving synthetic loop rows, raw fragment bags, raw
  walk outcomes, and hand-filled loop evidence cannot satisfy the loop stage.
- Add an operator classification registry or closeout matrix that records, for
  every new `7.4` operator, whether it is prepared spatial work, a topology
  declaration family, a grouped declaration family, a contribution workflow, a
  Query graph-composition program, or support-gated future mutation.
- Add validator registration evidence showing which validator families are
  attached to loop-ledger admission, topology declaration review, grouped
  contribution composition, graph-composition invariant packs, or Query
  registered invariants.

## Replay Closure

Replaying the same loop request must preserve:

- loop request identity
- source-loop carrier identities
- continuation-index identity
- continuation-policy outcomes
- canonical seed identities
- walk outcome identities
- loop candidate and denied-loop-candidate identities
- reconstructed-loop identities
- island partition identities
- role outcome identities
- degeneracy outcome identities
- persistent-name propagation map
- decision-log digest
- loop-ledger digest
- downstream-consumption identity
- counters
- denial and policy posture

## Diagnostics Closure

Denials must localize whether failure occurred at:

- loop request admission
- operator / validator registration closure
- source-loop and fragment provenance recovery
- continuation indexing
- continuation-policy and ambiguity admission
- canonical seed selection
- closed-walk assembly
- walk outcome classification
- loop candidate promotion and denial
- imprint-born loop construction
- island partition and source-loop split attribution
- role preservation and role outcome classification
- degeneracy classification
- loop identity minting
- naming propagation
- decision-log construction
- loop-ledger assembly
- workload evidence
- replay parity
- public-contract certification

## Determinism Closure

`7.4` must make the following stable:

- loop request identity
- source-loop carrier ordering
- continuation-index ordering
- continuation outcome ordering
- canonical seed ordering
- walk outcome ordering
- reconstructed-loop ordering
- island partition ordering
- role outcome ordering
- degeneracy outcome ordering
- loop identity
- loop name propagation identity
- decision-log identity
- loop-ledger digest
- downstream-consumption identity

## Complexity / Proof Closure

- Loop work must expose counters for:
  - source loops recovered
  - split fragments inspected
  - overlap chains inspected
  - continuation slots emitted
  - ambiguous continuation outcomes
  - canonical seeds emitted
  - walk candidates assembled
  - loop candidates promoted
  - denied loop candidates emitted
  - open walks denied
  - reconstructed loops emitted
  - born loops emitted
  - source loops split into multiple islands
  - role-preserving loops
  - role-ambiguous loops
  - degenerate loops denied / policy-exited
  - loop identities minted
  - name propagation rows emitted
  - decision-log rows emitted
- The complexity boundary starts at the `7.3` split-ledger receipt and then
  continues through continuation indexing and walk assembly; repeated fragment
  rescans cannot be the production-confidence proof path.
- Later phases must consume the loop-ledger receipt; they may not rebuild loop
  walks from split fragments or overlap chains. This is the Loop Ledger
  Exclusivity Law and not merely a workflow suggestion.
- Diagnostic richness must not change loop-ledger identity or operational
  counters.

## Allowed Debt

- No debt is allowed that lets raw fragment scans or caller-owned continuation
  maps satisfy production closeout.
- No debt is allowed that leaves branch, role, or degeneracy outcomes untyped.
- No debt is allowed that lets synthetic loop ledgers or copied split-ledger
  fields satisfy workload evidence or public-contract proof.
- Full overlap-region extraction, inside/outside classification, face assembly,
  and final topology legality remain deferred to later `7.x` milestones.
- Full face-level or merge/conflict naming semantics remain deferred.
- Curved, non-linear, seam, periodic, and trim-network loop reconstruction
  support remains deferred unless explicitly admitted by existing planar
  surfaces.

## Milestone Done When

- every admitted `7.3` split ledger enters one canonical loop-reconstruction
  request boundary
- every fragment referenced by loop work has recovered source-loop provenance
- every continuation branch is classified into non-overlapping typed
  continuation outcomes before walk assembly
- every walk candidate is classified with typed closure outcome before loop
  identity minting
- every closed walk is either promoted to a loop candidate or preserved as a
  denied loop candidate before admitted loop products exist
- reconstructed loops, born loops, and split islands have canonical,
  replay-stable identities
- preserved role, ambiguous role, and degenerate-loop posture are explicit
  products rather than inferred summaries
- the loop ledger is the only downstream product accepted by `7.5`
- loop-level persistent naming, decision logs, diagnostics, replay proof, and
  anti-theatre public contracts prove the loop stage cannot be faked
- the summum bonum certification target passes with machine-checkable evidence
  for both loop correctness and branch-trace closure

## Acceptance Evidence

- `cargo check -p worth-spatial -p worth-kernel -p worth-topo`
- focused public-contract tests for:
  - loop request admission
  - source-loop and fragment provenance recovery
  - continuation indexing and canonical ordering
  - continuation ambiguity and policy admission
  - canonical seed selection
  - closed-walk assembly and closure validation
  - loop candidate promotion and denied-loop-candidate localization
  - imprint-born loop construction and island partition
  - role preservation and role ambiguity
  - degenerate-loop classification
  - loop identity and naming propagation
  - loop decision logs and diagnostics
  - loop reconstruction ledger receipt
  - loop workload evidence
- compile-fail proof that loop request, continuation outcomes, walk outcomes,
  role outcomes, degeneracy outcomes, and loop-ledger receipt artifacts cannot
  be forged from raw fragments or raw walks
- anti-theatre proof that `7.5+` downstream consumers accept
  `PlanarBooleanLoopReconstructionLedgerReceipt` and reject raw split
  fragments, raw continuation maps, raw walk candidates, and source-local loop
  walks
- replay proof that the same split ledger produces the same loop ledger
- reversed-edge-sense proof where semantics permit
- retained replay / checkpoint parity proof where admitted
- workload catalog proof that hostile loop recipes are workload-backed
- operator classification matrix proving no `7.4` operator is an unclassified
  helper or pseudo-runtime entry
- Query/topology declaration proof for every topology-affecting loop operator
  admitted in `7.4`
- grouped/contribution workflow proof for every loop operator that relies on
  topology grouped neighborhoods or semantic contributions
- graph-composition/invariant-pack proof for every graph-shaped loop topology
  program admitted in `7.4`
- validator registration proof showing graph/topology legality validators deny
  through runtime-visible Query or topology declaration lanes
- summum bonum test:
  `planar_boolean_loop_reconstruction_metaboss_chain_is_canonical_replayable_role_preserving_and_unforgeable`

## Sequencing Notes

- Do not start `7.5` overlap-region extraction until `7.4` closes with a loop
  reconstruction ledger receipt that overlap work can consume.
- `7.5+` must consume `PlanarBooleanLoopReconstructionLedgerReceipt` as the
  exclusive loop-truth boundary and must not reopen raw fragments,
  continuation maps, or walk candidates.
- Do not put overlap-region extraction, fragment classification, or face
  assembly into `7.4`.
- Do not widen into EMBER here.
- If a Query-owned retained artifact, support, inspection, outcome, or evidence
  boundary is missing, extend the Query-shaped path or mark the loop surface
  blocked rather than inventing a local runtime lane.
- If additional hostile recipes are needed, add them through the workload
  catalog. Do not write loop-only fragment fixtures and call them proof.

## Required Self-Check

- Does the milestone solve a real structural problem or just package work
  cosmetically? Yes: it freezes the loop-level reconstruction authority that
  overlap extraction, classification, and face assembly depend on.
- Is the adversarial constraint precise and load-bearing? Yes: it requires one
  canonical loop ledger or a typed localized denial for the same admitted split
  ledger across replay, branch pressure, role pressure, and degeneracy
  pressure.
- Does the roadmap justify this milestone now? Yes: `7.4` is the first loop
  consumer of the `7.3` split ledger and the roadmap places overlap extraction,
  classification, and assembly after it.
- Does the spec preserve crate authority boundaries? Yes: Query owns runtime
  entry and progression, `worth-kernel` owns workload evidence, `worth-spatial`
  owns loop reconstruction semantics, and `worth-topo` remains topology truth.
- Are the phases carrying most of the real design information? Yes.
- Is each phase centered on one conceptual detail or boundary? Yes.
- Does each phase contain at least 2 adversarial tests by default? Yes.
- Could a competent engineer map this spec into honest types, modules, and
  tests? Yes.
- Does the milestone belong in this roadmap sequence, or is it out of order?
  It belongs here because loop truth must freeze before overlap regions,
  classification, and face assembly can consume it honestly.
