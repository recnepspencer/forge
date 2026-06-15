# Worth Milestone 7.1: Certified Common-Plane Reduction

> **Status:** Draft
>
> **Purpose:** freeze the honest reduction of a real `7.0`-admitted planar
> boolean operand pair into one shared certified planar frame that every later
> B-rep planar boolean milestone must consume.

## Goal

Milestone `7.1` closes the reduction gap between "we have a real admitted
boolean workload" and "later phases may perform split / overlap / classify work
inside one canonical 2D frame."

By the end of this milestone:

- a `7.0` boolean operand pair can enter one and only one common-plane
  reduction path
- plane agreement, posture agreement, and precision agreement are each proved
  explicitly rather than assumed together
- the shared-plane result has one canonical identity, one canonical local
  frame, and one replay-stable projection story
- later boolean milestones consume a typed common-plane reduction artifact
  instead of inventing local basis folklore or re-projecting ad hoc
- workload composition, evidence, replay, and certification surfaces make the
  reduction mechanically visible rather than test-local

Milestone `7.1` does **not** implement segment events, split topology, overlap
islands, fragment classification, or face assembly. It freezes the single
certified reduction substrate those later steps must inherit.

## Why This Milestone Exists

The next easy mistake after `7.0` is to say "the operands are planar enough"
and let every later boolean subsystem pick its own local frame, project its own
coordinates, and quietly reinterpret what "same plane" meant.

That failure mode is fatal because it allows:

- one phase to treat geometric coplanarity as sufficient while another requires
  motion-normalized agreement
- one phase to pick a basis from operand A while another picks a mirrored basis
  from operand B
- one projection pass to produce a different identity ordering from a later
  replay pass
- later boolean phases to re-open plane questions that should already have been
  rejected or certified
- hostile tests to fabricate projected geometry without proving that a real
  shared-plane reduction ever occurred

`7.1` exists to prevent that drift. The system should certify shared planar
reduction once, carry the proof forward, and make later phases consume the
proof-bearing artifact rather than rediscovering it.

## Governing Summaries

- `MENTALITY.md`: protect the hard problem first. The milestone must close the
  naive "just pick a plane and project" failure mode before event extraction
  begins.
- `arch_laws.md`: protect proof-bearing phase transitions. Each reduction step
  must widen proof from admitted operand pair to shared-plane artifact rather
  than hiding multiple decisions behind one helper.
- `composition_laws.md`: protect narrow semantic slices. Plane eligibility,
  posture agreement, basis identity, projection consumption, and replay closure
  should not collapse into one overloaded reduction bucket.
- `domain_structure_laws.md`: protect visible ownership boundaries. Query keeps
  the runtime entry and operating contexts; `worth-spatial` owns planar
  reduction semantics; `worth-kernel` owns workload composition and evidence
  closure.
- `perf_laws.md`: protect carry-forward proof and bounded breadth. Shared-plane
  reduction should happen once per admitted operand pair, emit reusable
  artifacts, and prevent later stages from re-walking the same proof work.
- `_docs/worth/worth_roadmap.md`: protect general workflow closure over toy
  examples. The milestone must admit a real planar workflow class and fail
  cleanly outside it.
- `_docs/worth/milestone-7-roadmap.md`: protect `7.1` as certified common-plane
  reduction, not as early event extraction or generic boolean execution.
- `_docs/worth/milestone-7.0.md`: protect the `7.0` entry boundary. `7.1`
  starts from real admitted boolean operand pairs and must not reopen a local
  pseudo-entry lane.
- `crates/forge-query/docs/AI_README.md`: protect the rule `declare intent
  once, lower it once, execute or inspect it through canonical runtime-owned
  artifacts`. `7.1` may add domain artifacts, but it must not invent a new
  caller-owned routing surface.

## Adversarial Constraint

For the same admitted planar boolean operand pair, the system must either:

- deny the pair before projection with a typed, localized, replay-stable reason

or:

- produce one and only one shared-plane reduction artifact whose plane
  identity, posture basis, precision basis, local frame identity, projection
  stage identity, operand ordering, and retained replay story remain stable
  across replay, enumeration-order variation, and later downstream milestone
  consumption.

If two later boolean phases can take the same admitted operand pair and derive
different local frames, different projected identity orderings, or different
plane agreement stories without an earlier typed denial, the milestone has
failed.

## Product Decision Lock

- `7.1` builds entirely on the `7.0` public entry and workload truth boundary.
- No new kernel-owned or caller-owned boolean front door is allowed.
- Query continues to own operating contexts, declaration lowering, admission,
  and runtime-backed handles.
- `worth-spatial` owns common-plane semantics, canonical local-frame selection,
  and certified projection consumption.
- `worth-kernel` owns workload-composition artifacts, stage requirements,
  evidence rows, and public anti-theatre closeout fences for the reduction.
- `worth-topo` remains the topology truth source for the admitted operand
  workload, but `7.1` does not yet perform result-topology construction.
- Later milestones must consume the `7.1` reduction artifact. They may not
  silently recompute a local basis from raw operands.

## Existing Surface Inventory

Milestone `7.1` should widen live surfaces before inventing new ones:

- `crates/worth-kernel/src/workload_composition/workload_catalog/boolean_operand_pair.rs`
- `crates/worth-kernel/src/workload_composition/worth_workload.rs`
- `crates/worth-kernel/src/workload_composition/stage_requirements.rs`
- `crates/worth-spatial/src/workload_platform/boolean_readiness_workload/*`
- `crates/worth-spatial/src/facade/projection_workload/mod.rs`
- `crates/worth-spatial/src/workload_platform/projected_overlap_faces/*`
- `crates/worth-spatial/src/certification/geometry_support_family_contracts.rs`
- existing Query-native bindings for planar precision, local frame, projection,
  segment relation, winding, signed area, overlap, and predicate authority

New `7.1` surfaces are allowed only where the existing inventory cannot
honestly express:

- a proof-bearing common-plane reduction request
- typed denial of plane / posture / precision disagreement
- a canonical shared-plane identity
- a canonical shared-frame reduction artifact consumed by later milestones
- workload evidence and replay closure for that reduction

## Phase Plan

### Phase 1: Common-Plane Reduction Request Boundary

Freeze the artifact that enters `7.1` from the `7.0` admitted boolean
operand-pair boundary.

**Relevant subsystems**
- `worth-kernel`
- `worth-spatial`

**Relevant APIs**
- `crates/worth-kernel/src/workload_composition/workload_catalog/boolean_operand_pair.rs`
- `crates/worth-kernel/src/workload_composition/worth_workload.rs`
- `crates/worth-kernel/src/workload_composition/boolean_common_plane_reduction/*`
- `crates/worth-spatial/src/workload_platform/boolean_readiness_workload/mod.rs`

**Warnings**
- Do not accept raw faces, loops, projected points, or ad hoc local-frame data
  as a substitute for the request artifact.
- Do not let later phases choose their own entry shape.

**Test requirements**
- Adversarial parity test: the same admitted operand pair must produce the same
  request identity regardless of benign caller enumeration order.
- Adversarial rejection test: a synthetic pair assembled outside the workload
  catalog and readiness receipts must fail to enter the reduction boundary.

**Engineering decisions**
- Introduce a dedicated request artifact rather than passing the generic operand
  pair through every downstream phase.
- Preserve the `7.0` declaration and operand-pair identities inside the request
  artifact so later evidence rows can point back to real workload truth.

**Open questions**
- Should the request artifact live under workload composition or under a
  dedicated boolean reduction subtree owned jointly by workload composition and
  spatial certification?

### Phase 2: Operand Pair Shape And Scope Admission

Freeze what operand-pair shapes `7.1` is actually allowed to reduce before any
plane-agreement proof begins.

**Relevant subsystems**
- `worth-kernel`
- `worth-spatial`

**Relevant APIs**
- `crates/worth-kernel/src/workload_composition/workload_catalog/catalog.rs`
- `crates/worth-spatial/src/workload_platform/boolean_readiness_workload/validation.rs`
- `crates/worth-spatial/src/workload_platform/boolean_readiness_workload/stage_coverage.rs`
- `crates/worth-kernel/src/workload_composition/boolean_common_plane_reduction/*`

**Warnings**
- "Planar" is not enough. The phase must define the admitted operand-set shape
  explicitly.
- Do not hide scope denials behind later plane-agreement failures.

**Test requirements**
- Adversarial parity test: equivalent admitted scope recipes must lower to the
  same shape-admission result without changing denial class.
- Adversarial rejection test: a mixed or unsupported operand scope must deny
  before plane-agreement or projection code runs.

**Engineering decisions**
- Separate shape / scope admission from common-plane truth so denial locality is
  preserved.
- Keep the admitted class narrow and explicit for `7.1`; let later milestones
  widen scope only when they can preserve the same proof model.

**Open questions**
- Does `7.1` admit only one face-pair class, or one general planar operand-pair
  class with multiple face / loop carriers?

### Phase 3: Plane-Agreement Eligibility

Freeze the geometric question "are these operands certifiably on one plane at
all?"

**Relevant subsystems**
- `worth-spatial`
- `worth-topo`

**Relevant APIs**
- planar precision and planar local-frame Query-native bindings already used by
  `worth-spatial`
- `crates/worth-spatial/src/certification/geometry_support_family_contracts.rs`
- `crates/worth-spatial/src/workload_platform/planar_boolean_common_plane/*`

**Warnings**
- Do not collapse nearly-coplanar, distinct-plane, and unsupported-plane cases
  into one generic denial.
- Do not let epsilon-only folklore masquerade as certified agreement.

**Test requirements**
- Adversarial parity test: replaying the same admitted pair through the same
  precision basis must preserve the exact plane-agreement outcome.
- Adversarial rejection test: visually similar but certifiably distinct planes
  must deny before any shared-frame identity is minted.

**Engineering decisions**
- Plane agreement is a proof-bearing eligibility phase, not a helper predicate.
- Denials must carry enough machine context for later diagnostics and
  anti-theatre tests to localize disagreement honestly.

**Open questions**
- Should the denial taxonomy distinguish "not common-plane" from "common-plane
  unavailable under current precision basis" as separate machine classes?

### Phase 4: Motion / Rotation Posture Agreement

Freeze the requirement that both operands agree not only geometrically but also
through the same admitted movement / rotation posture.

**Relevant subsystems**
- `worth-spatial`

**Relevant APIs**
- `crates/worth-spatial/src/workload_platform/projected_overlap_faces/authority.rs`
- `crates/worth-spatial/src/workload_platform/projected_overlap_faces/bundle.rs`
- `crates/worth-spatial/src/workload_platform/planar_boolean_common_plane/*`
- transform / motion posture surfaces already consumed by the workload platform

**Warnings**
- Same geometric plane is not sufficient if posture identity diverges.
- Do not bury posture mismatch inside generic projection failure.

**Test requirements**
- Adversarial parity test: identical admitted posture histories must preserve
  the same posture-agreement identity across replay.
- Adversarial rejection test: operands from mismatched motion / rotation
  posture identities must deny before shared-frame construction.

**Engineering decisions**
- Reuse the existing context/posture identity pattern already present in the
  projected overlap authority surfaces.
- Keep posture agreement distinct from geometric plane agreement so later
  diagnostics do not flatten different failure causes.

**Open questions**
- Should opposite-normal but certifiably reconcilable operands admit under one
  normalized posture class, or remain denied until a later milestone?

### Phase 5: Precision And Tolerance Basis Agreement

Freeze the precision regime used to certify shared reduction.

**Relevant subsystems**
- `worth-spatial`
- Query-native planar precision bindings

**Relevant APIs**
- planar precision certification bindings already used by local-frame and
  overlap certification
- `crates/worth-spatial/src/workload_platform/projected_overlap_faces/bundle.rs`
- `crates/worth-spatial/src/workload_platform/planar_boolean_common_plane/*`

**Warnings**
- Precision basis cannot be inferred separately by each downstream phase.
- Do not let one operand smuggle a different tolerance story into the pair.

**Test requirements**
- Adversarial parity test: the same admitted pair under the same precision
  basis must preserve one precision-agreement identity and one downstream
  reduction story.
- Adversarial rejection test: a mismatched precision basis must deny before any
  local frame or projected operand artifact is emitted.

**Engineering decisions**
- Carry precision-basis identity forward explicitly in the reduction artifact.
- Separate precision agreement from shared-plane identity so later phases can
  observe exactly what was certified.

**Open questions**
- Do we need a dedicated pair-level precision agreement receipt, or can the
  reduction artifact itself own that proof without collapsing phase boundaries?

### Phase 6: Canonical Shared-Plane Identity

Freeze one stable shared-plane identity for every admitted reduction.

**Relevant subsystems**
- `worth-spatial`
- `worth-kernel`

**Relevant APIs**
- `crates/worth-spatial/src/workload_platform/planar_boolean_common_plane/*`
- `crates/worth-kernel/src/workload_composition/boolean_common_plane_reduction/*`
- `crates/worth-kernel/src/workload_composition/worth_workload.rs`

**Warnings**
- Do not derive identity from caller-owned strings, debug formatting, or
  enumeration order.
- Do not couple shared-plane identity to the later 2D basis selection step.

**Test requirements**
- Adversarial parity test: semantically identical admitted pairs must mint the
  same shared-plane identity even when the carrier enumeration order varies.
- Adversarial rejection test: a pair missing any proved agreement prerequisite
  must fail before a shared-plane identity exists.

**Engineering decisions**
- Mint shared-plane identity only after shape, plane, posture, and precision
  agreement have all succeeded.
- Keep shared-plane identity distinct from projected-frame identity so replay
  diagnostics can localize drift precisely.

**Open questions**
- Does the identity need explicit topology-basis participation in addition to
  spatial agreement fields for anti-theatre proof?

### Phase 7: Canonical Local Frame Selection

Freeze the one certified 2D basis that all later planar boolean work must
consume.

**Relevant subsystems**
- `worth-spatial`

**Relevant APIs**
- `crates/worth-spatial/src/facade/projection_workload/mod.rs`
- `crates/worth-spatial/src/workload_platform/planar_boolean_common_plane/*`
- existing local-frame certification surfaces

**Warnings**
- Do not let operand A or operand B win basis selection implicitly.
- Do not allow mirrored or rotated-but-equivalent frames to drift by
  enumeration order.

**Test requirements**
- Adversarial parity test: the same shared-plane identity must always yield the
  same local-frame identity and basis orientation.
- Adversarial rejection test: ambiguous or unsupported basis selection cases
  must deny explicitly instead of falling through to arbitrary frame picking.

**Engineering decisions**
- Make local-frame selection a named phase with a named artifact, not a helper
  inside projection.
- Carry enough orientation and digest proof that downstream event extraction can
  trust the 2D basis without re-choosing it.

**Open questions**
- Should basis canonicalization include explicit tie-break rules for opposite
  normals in `7.1`, or remain fail-closed until a later admitted class?

### Phase 8: Projection Consumption For Operand A

Freeze certified projection consumption for the first operand through the shared
frame.

**Relevant subsystems**
- `worth-spatial`

**Relevant APIs**
- `crates/worth-spatial/src/facade/projection_workload/mod.rs`
- projection receipt / consumed workload surfaces
- `crates/worth-spatial/src/workload_platform/planar_boolean_common_plane/*`

**Warnings**
- Do not expose generic projected geometry blobs without reduction provenance.
- Do not permit operand-local reprojection after this phase.

**Test requirements**
- Adversarial parity test: operand A must project to the same consumed identity
  and projection stage identity across replay.
- Adversarial rejection test: an operand that does not match the certified
  shared-frame basis must fail projection consumption locally and explicitly.

**Engineering decisions**
- Split operand A and operand B projection consumption into separate phases even
  if they share machinery, so denial and provenance stay operand-local.
- Preserve source operand identity, shared-plane identity, and projection stage
  identity together in the consumed artifact.

**Open questions**
- Should operand-local consumption artifacts expose loop / face carrier
  breakdown directly, or only through a later reduced-pair assembly surface?

### Phase 9: Projection Consumption For Operand B

Freeze certified projection consumption for the second operand through the same
shared frame.

**Relevant subsystems**
- `worth-spatial`

**Relevant APIs**
- `crates/worth-spatial/src/facade/projection_workload/mod.rs`
- projection receipt / consumed workload surfaces
- `crates/worth-spatial/src/workload_platform/planar_boolean_common_plane/*`

**Warnings**
- Do not let operand B ride on operand A's proof without its own consumed
  receipt path.
- Do not let a one-sided success produce a fake reduced pair.

**Test requirements**
- Adversarial parity test: operand B must preserve the same consumed identity,
  shared-frame linkage, and projection stage identity across replay.
- Adversarial rejection test: a pair where operand A projects cleanly and
  operand B fails must localize the failure to operand B without poisoning the
  already-proved shared-plane denial story.

**Engineering decisions**
- Mirror the projection-consumption contract of phase 8 while preserving
  operand-local denial and evidence.
- Keep the two operand receipts structurally parallel so pair assembly can
  reason about them symmetrically without erasing identity meaning.

**Open questions**
- Should operand B denial classes be byte-for-byte parallel with operand A
  denial classes, or is there a meaningful asymmetry in subtraction workflows
  that belongs later?

### Phase 10: Pair Reduction Assembly And Ordering Stabilization

Freeze the reduced operand-pair artifact and the canonical ordering rules that
later milestones inherit.

**Relevant subsystems**
- `worth-spatial`
- `worth-kernel`

**Relevant APIs**
- `crates/worth-spatial/src/workload_platform/planar_boolean_common_plane/*`
- `crates/worth-kernel/src/workload_composition/boolean_common_plane_reduction/*`
- `crates/worth-kernel/src/workload_composition/workload_catalog/boolean_operand_pair.rs`

**Warnings**
- Do not let later milestones infer ordering from vector position alone without
  a declared contract.
- Do not let pair assembly depend on projection traversal order.

**Test requirements**
- Adversarial parity test: semantically identical operand pairs must assemble
  to the same reduced-pair digest and ordering contract across replay and input
  enumeration variation.
- Adversarial rejection test: attempts to assemble a reduced pair from mixed
  shared-plane identities, mixed frame identities, or mixed projection stages
  must fail explicitly.

**Engineering decisions**
- Make ordering stabilization explicit here instead of burying it inside later
  replay or event phases.
- Treat the reduced pair as the single input to `7.2`; later milestones should
  not consume the separate operand receipts directly unless the reduced-pair
  surface says so.

**Open questions**
- Should reduced-pair ordering be entirely canonicalized, or should later
  operator kind retain one declared left/right semantic layer on top of a
  canonical storage order?

### Phase 11: Workload Evidence, Retained Replay, And Anti-Theatre Closure

Freeze the mechanical proof that common-plane reduction really happened and that
later milestones cannot fake or bypass it.

**Relevant subsystems**
- `worth-kernel`
- `worth-spatial`

**Relevant APIs**
- `crates/worth-kernel/src/workload_composition/stage_requirements.rs`
- `crates/worth-kernel/src/workload_composition/worth_workload.rs`
- `crates/worth-kernel/src/workload_composition/boolean_common_plane_reduction/*`
- `crates/worth-spatial/src/workload_platform/evidence_ledger/*`
- `crates/worth-spatial/src/workload_platform/planar_boolean_common_plane/*`
- `crates/worth-kernel/src/certification/public_facade_contracts/contracts/*`

**Warnings**
- Do not close the milestone with geometry-only tests.
- Do not let retained replay or later milestone fixtures rebuild reduced facts
  from raw operands outside the certified reduction path.

**Test requirements**
- Adversarial parity test: retained replay and non-replay execution of the same
  admitted operand pair must consume the same reduction artifact, emit the same
  evidence-stage story, and preserve the same reduced-pair digest.
- Adversarial rejection test: compile-fail and public-contract fences must
  block synthetic reduced-pair construction, synthetic evidence rows, generic
  projection substitution, and downstream use without the `7.1` stage proof.

**Engineering decisions**
- Add explicit stage requirements and evidence rows for common-plane reduction
  rather than piggybacking silently on generic projection stages.
- Make `7.2` and later milestones prove they consumed the `7.1` reduced-pair
  artifact boundary, not merely projected geometry with similar shape.

**Open questions**
- Does `7.1` need its own closeout contract bundle the way `7.0` did, or is a
  new reduction-specific downstream-consumption fence sufficient?

## Admitted Surface

- real `7.0`-admitted planar boolean operand pairs
- one common-plane reduction path
- typed denial of shape, plane, posture, precision, and projection mismatch
- one canonical shared-plane identity
- one canonical local frame
- one reduced operand-pair artifact suitable for `7.2`

## Excluded Surface

- segment-event extraction
- split-edge topology rewriting
- overlap-region island extraction
- fragment classification
- face assembly
- cleanup and topology legality of boolean results
- curved or non-planar operand-pair reduction

## Workflow Surface

- admitted planar boolean operand-pair reduction over arbitrary admitted planar
  operand pairs within the milestone's shape class
- replay and retained-replay consumption of that same reduction workflow
- clean failure for any operand pair outside the admitted common-plane class

## Operator Closure

- common-plane reduction request compilation
- pair-level plane / posture / precision agreement certification
- canonical local-frame selection
- operand-local projection consumption
- reduced-pair assembly

## Validator Closure

- shape and scope admission validators
- common-plane eligibility validators
- posture-agreement validators
- precision-basis agreement validators
- reduced-pair identity and ordering validators
- evidence-stage and downstream-consumption validators

## Replay Closure

- identical admitted operand pairs must replay to the same shared-plane
  identity, local-frame identity, operand projection identities, and
  reduced-pair digest

## Diagnostics Closure

- denials must localize whether failure occurred at scope admission, plane
  agreement, posture agreement, precision agreement, frame selection, operand A
  projection, operand B projection, or reduced-pair assembly

## Determinism Closure

- shared-plane identity
- local-frame identity
- operand projection stage identities
- reduced-pair ordering
- reduced-pair digest

## Complexity / Proof Closure

- common-plane reduction should execute once per admitted operand pair and emit
  proof-bearing artifacts that downstream phases consume instead of
  re-certifying
- counters and evidence rows must expose reduction-stage presence explicitly
- no later phase may broaden work by silently rebuilding the common-plane proof

## Allowed Debt

- `7.1` may remain fail-closed on complex opposite-normal reconciliation if the
  denial is explicit and replay-stable
- `7.1` does not yet need event extraction, overlap extraction, or split
  topology proof
- `7.1` may keep the admitted operand-shape class narrower than the eventual
  planar boolean class as long as the exclusion boundary is explicit

## Milestone Done When

- every admitted planar boolean operand pair enters one canonical common-plane
  reduction boundary
- every denied pair fails with a typed and localized reduction-stage reason
- a successful reduction emits one canonical shared-plane identity, one
  canonical local frame, and one reduced operand-pair artifact
- replay and retained replay preserve the same reduction truth
- workload evidence and public anti-theatre fences make fake reduction proof
  harder than the real path
- `7.2` can be specified against the reduced-pair artifact without reopening
  plane or basis folklore
