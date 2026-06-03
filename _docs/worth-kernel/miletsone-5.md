# Worth Kernel Milestone 5

This document is a first-pass implementation-order outline for Milestone 5.

It is intentionally not the full finished milestone spec yet.

The goal of this draft is to capture what Milestone 5 needs to cover in the
order we should probably build it, before we expand into:

- cross-crate changes
- new operator and validator families
- Forge Query usage details
- proof and certification structure

## Milestone Center

Milestone 5 freezes topology-to-geometry binding truth as a real authority
boundary.

Milestone 4 proved the construction-time birth seam.

Milestone 5 widens that into real binding and rebinding truth so later exact
planar work, booleans, continuity, and history surfaces inherit an honest
substrate instead of retrofitting one.

## Implementation Order

## 1. Freeze Binding Truth As Its Own Authority Surface

First, Milestone 5 must define authoritative topology-to-geometry binding truth
as its own surface rather than treating binding as incidental construction
metadata.

This means the system must have explicit truth for:

- face-to-surface bindings
- edge-to-curve bindings
- coedge-to-p-curve bindings
- vertex-to-geometry bindings

This is the root of the milestone. Everything else depends on it.

## 2. Freeze Binding Identity As Distinct From Topology And Naming

Once binding truth exists, Milestone 5 must make binding identity explicit and
separate from:

- topology identity
- naming identity
- later continuity conclusions

The system should not be allowed to quietly collapse:

- "same topology entity"
- "same persistent name"
- "same geometric binding"

into one blurry identity story.

Milestone 5 should freeze geometry-binding identity as its own truth family.

## 3. Replace Placeholder Anchor Denial With Real Carrier-Local Anchor Truth

Milestone 4 intentionally deferred robust `ParameterSpace(...)` support.

Milestone 5 must replace that typed unsupported placeholder with real
carrier-local anchor truth.

This needs to cover:

- parameter-space point anchors
- parameter-space direction anchors
- explicit carrier ownership for those anchors

This is the moment where carrier-local coordinates stop being implied and
become real truth.

## 4. Freeze Binding Completeness Rules

After binding truth and anchor truth exist, Milestone 5 must define what counts
as a complete admitted binding state.

This includes rules for:

- when an admitted entity is fully bound
- when a partially bound state is allowed
- when partial binding must fail
- when missing binding is typed unsupported versus illegal

Later milestones should not have to invent these rules case by case.

## 5. Freeze Motion-Aware Binding Semantics

Milestone 4 added real motion and anchor semantics.

Milestone 5 must therefore define what happens to binding truth under admitted:

- move workflows
- rotate workflows
- reorient workflows

Bindings must not be treated as static birth-only facts.

The system must know whether a binding is:

- preserved under motion
- transformed with its carrier
- invalidated by the workflow
- left unresolved pending a typed failure or ambiguity result

## 6. Freeze Rebinding Semantics For Local Topology Replacement

After motion-aware binding is explicit, Milestone 5 must define rebinding
semantics for local rebuild and topology replacement workflows.

This is the core "after an edit, what survives?" layer.

Milestone 5 must cover admitted workflows where:

- a face is replaced
- an edge is replaced
- a coedge trim path is replaced
- a vertex support situation changes

and prior binding truth must be:

- preserved
- reattached
- denied
- marked ambiguous

through explicit rules.

## 7. Freeze Typed Rebinding Outcome Classes

Rebinding results must be first-class truth, not prose or ad hoc booleans.

Milestone 5 should define explicit rebinding outcome classes such as:

- preserved
- exact reattachment
- continuity-justified reattachment
- ambiguous
- orphaned
- unsupported

These classes will be the vocabulary later milestones depend on.

## 8. Freeze Continuity And Rebinding Diagnostics

Once outcome classes exist, Milestone 5 must expose diagnostics that explain:

- what the prior binding meant
- what changed
- what candidates existed
- why a candidate was chosen
- why continuity was preserved or denied
- why a case was ambiguous or unsupported

The milestone should not stop at "worked" versus "failed."

## 9. Freeze Canonical Binding Identity And Digest Truth

Milestone 5 must harden the identity layer for binding truth itself.

This includes:

- canonical binding identity
- canonical anchor identity
- canonical rebinding identity
- geometry-committing digest truth for the binding surface

Later replay and certification work should inherit a real identity protocol
rather than soft or summary-only digests.

## 10. Freeze Historical Binding Inspection

Milestone 5 must make admitted binding history inspectable.

This should include the ability to inspect:

- a binding at a checkpoint
- a rebinding transition
- the continuity result attached to that rebinding decision
- the diagnostic explanation for that decision

Binding truth should be historically readable, not just live-state readable.

## 11. Freeze Branch-Local Binding Inspection

Milestone 5 must widen historical inspection into branch-local inspection for
the admitted surface.

That means branch-local workflows should be able to inspect:

- current binding truth
- prior binding truth
- branch-local rebinding outcomes
- branch-local continuity conclusions

This is needed so later branch and merge milestones inherit a real substrate.

## 12. Freeze Clean Kernel-To-Spatial Rebinding Seams

Milestone 4 proved a real kernel -> spatial -> topology construction path.

Milestone 5 must do the same for admitted binding and rebinding workflows.

`worth-kernel` should be able to consume:

- binding authoring
- binding lookup
- rebinding evaluation
- rebinding diagnostics

through clean spatial contracts instead of inventing local workaround logic.

## 13. Add Narrow Curved Carrier Pressure

Milestone 5 must not remain secretly planar-only.

It needs a narrow admitted curved-carrier surface sufficient to force honest:

- carrier-local anchors
- curved binding semantics
- curved rebinding semantics
- curved continuity diagnostics

This should stay narrow and intentional.

The purpose is not broad curved completion.

The purpose is to make the binding substrate real.

## 14. Add At Least One Asymmetric Curved Primitive Or Carrier Family

Milestone 5 must also pressure the system with at least one asymmetric curved
shape family.

A stretched blimp / ellipsoid-like family is the right kind of pressure because
it breaks:

- planar shortcuts
- circular symmetry shortcuts
- interchangeable-side assumptions
- fake continuity logic that only works on regular families

This family exists to harden the substrate before booleans and later curved
work depend on it.

## 15. Freeze Replay-Safe Binding And Rebinding Histories

Once the above surfaces exist, Milestone 5 must require admitted binding and
rebinding histories to replay identically.

This includes stable replay for:

- binding identity
- anchor identity
- rebinding outcome classes
- continuity conclusions
- diagnostics

## 16. Freeze Determinism And Certification For The Binding Layer

Milestone 5 should close by proving that the admitted binding layer is:

- deterministic
- historically inspectable
- replay-safe
- identity-stable
- explicit about ambiguity and unsupported cases

At minimum, the milestone must certify:

- rebinding determinism
- continuity classification determinism
- binding identity stability
- historical inspection parity

## Things Milestone 5 Does Not Need To Close

Milestone 5 does not need to close:

- full exact planar hostility
- full Boolean programs
- broad tangent-hostile curved certification
- broad freeform or NURBS support
- later feature regeneration closure
- merge-era semantics

Those depend on Milestone 5, but they do not belong inside its core scope.

## Current Summary

Milestone 5 should build in this order:

1. binding truth
2. binding identity separation
3. carrier-local anchor truth
4. binding completeness rules
5. motion-aware binding semantics
6. local replacement rebinding semantics
7. typed rebinding outcomes
8. rebinding diagnostics
9. canonical binding identity and digests
10. historical inspection
11. branch-local inspection
12. kernel-to-spatial rebinding seams
13. narrow curved-carrier pressure
14. one asymmetric curved family
15. replay-safe binding histories
16. determinism and certification closure

That is the current first-pass implementation sequence.

## First-Pass Operator Inventory

This section captures the concrete operator set Milestone 5 needs, grouped by
crate ownership.

This is not yet the full final crate spec.

It is the first-pass working inventory that follows from the milestone center.

## `worth-spatial` Core Binding Operators

These operators are the real center of Milestone 5.

### Direct binding authoring and replacement

- `AttachSurfaceToFace`
- `ReplaceSurfaceOnFace`
- `AttachCurveToEdge`
- `ReplaceCurveOnEdge`
- `AttachPCurveToCoedge`
- `ReplacePCurveOnCoedge`
- `AttachVertexGeometry`

`AttachVertexGeometry` is not cleanly present in the current operator list, but
Milestone 5 needs the vertex analogue of the face/edge/coedge binding family.

### Carrier-local anchor authoring

- `BindParameterSpacePointAnchor`
- `BindParameterSpaceDirectionAnchor`

These are effectively required Milestone 5 operators even if the current
inventory does not already name them directly.

### Binding evaluation and authority

- `EvaluateBindingCompleteness`
- `EvaluateBindingIdentity`
- `EvaluateRebindingCandidates`
- `ClassifyRebindingOutcome`
- `ExplainRebindingOutcome`

These operators freeze the truth surface around binding and rebinding instead of
leaving it implicit inside later workflows.

### Historical and branch-local inspection

- `InspectBindingAtCheckpoint`
- `InspectBindingTransition`
- `InspectBranchLocalBindingState`

Milestone 5 should not treat binding truth as live-state-only information.

## `worth-spatial` Secondary Binding Operators

These belong after the basic bind/rebind path exists, but they are still part
of the admitted Milestone 5 substrate.

### Sense and parameterization control

- `ReverseEdgeCurveSense`
- `ReparameterizeEdgeCurve`
- `NormalizeEdgeCurveDomain`
- `ReversePCurveSense`
- `ReparameterizePCurve`
- `NormalizePCurveDomain`
- `SwapSurfaceParameterization`
- `NormalizeSurfaceParameterization`
- `SetFaceSurfaceSense`
- `SetEdgeCurveSense`
- `SetCoedgeSense`

These operators keep binding truth honest once carrier-local coordinates and
sense semantics become first-class.

## `worth-topo` Query And Navigation Helpers Milestone 5 Must Be Able To Consume

Milestone 5 does not move geometry-binding authority into `worth-topo`.

But it does require narrow topology query/navigation helpers that the spatial
binding layer can consume without recreating topology traversal logic locally.

These helpers are:

- `GetFaceLoops`
- `GetFaceEdges`
- `GetLoopCoedges`
- `GetCoedgeEdge`
- `GetEdgeVertices`
- `GetShellFaces`

These belong in `worth-topo` as topology-safe read helpers.

They are required support for Milestone 5 even though they are not the center
of the milestone.

## `worth-kernel` Workflow Operators

`worth-kernel` should stay thin here.

It should orchestrate admitted workflows and certification pressure, not own
binding legality or rebinding truth itself.

The first-pass workflow operators are:

- `AuthorPrimitiveBindingIntent`
- `PrepareAdmittedBindingWorkflow`
- `PrepareAdmittedRebindingWorkflow`
- `PrepareBindingInspectionWorkflow`
- `PrepareBindingCertificationScenario`

These operators exist so kernel workflows can consume Milestone 5 through clean
spatial contracts rather than inventing local binding logic.

## `worth-geom` Support Operators

`worth-geom` should support Milestone 5 through carrier math and inversion
utilities, not by owning binding authority.

The first-pass support set is:

- `InvertSurfacePointToUV`
- `InvertCurvePointToT`
- `EvalSurfacePoint`
- `EvalSurfaceDerivatives`
- `EvalCurvePoint`
- `EvalCurveDerivatives`
- `ClosestPointOnSurface`
- `ClosestPointOnCurve`

For the asymmetric curved pressure case, `worth-geom` will also need the
carrier support needed to represent and interrogate the admitted asymmetric
curved family.

## Operators Explicitly Deferred Out Of Milestone 5

Milestone 5 should not try to close the broad coupled surgery and Boolean-era
operator families yet.

The important deferred operators are:

- `SplitEdgeAndCurves`
- `MergeEdgesAndCurves`
- `SplitCoedgeAndPCurve`
- `MergeCoedgesAndPCurve`
- `SplitFaceAndTrimNetwork`
- `MergeFacesAndTrimNetwork`
- seam and pole surgery operators
- tangent-event normalization operators
- broad trim-network editing operators
- Boolean-specific imprint and intersection surgery operators

These depend on Milestone 5's binding substrate, but they do not belong inside
its first admitted closure.

## First-Pass Validator Inventory

This section captures the validator set Milestone 5 needs, again grouped by
crate ownership.

## `worth-spatial` Core Binding Validators

These validators define whether Milestone 5's new binding truth is honest.

### Existing validator families Milestone 5 must rely on or widen

- `ValidateCurveBoundToEdge`
- `ValidatePCurveBoundToCoedge`
- `ValidateSenseConsistency`
- `ValidateEveryCoedgeHasPCurveWhenRequired`
- `ValidatePCurveSenseMatchesCoedgeSense`
- `ValidateCurveSurfaceInversionResiduals`

### Milestone 5 validators that need to become explicit

- `ValidateBindingCompleteness`
- `ValidateBindingIdentitySeparation`
- `ValidateRebindingDeterminism`
- `ValidateRebindingOutcomeClassification`
- `ValidateHistoricalBindingInspectionParity`
- `ValidateBranchLocalBindingInspectionParity`
- `ValidateCanonicalBindingDigestStability`
- `ValidateAnchorCarrierOwnership`
- `ValidateParameterSpaceAnchorResolution`

These validators are the milestone-defining closure for the binding authority
surface.

## `worth-topo` Validators Milestone 5 Must Continue To Consume

Milestone 5 should use topology legality proof from `worth-topo` rather than
rebuild it locally.

The important topology validators that remain required support are:

- `ValidateTwinSymmetry`
- `ValidateNextPrevSymmetry`
- `ValidateLoopClosure`
- `ValidateOwnership`
- `ValidateNoOrphans`
- `ValidateShellWatertightness`
- `ValidateRadialCycleClosure`
- `ValidateEdgeManifoldStateMatchesUseCount`

These remain topology-owned even when spatial binding workflows depend on them.

## Curved-Pressure Validators Milestone 5 Should Add

Because Milestone 5 should include at least one asymmetric curved admitted
carrier or primitive family, it also needs explicit pressure validators for
that surface.

The first-pass set is:

- `ValidateCarrierLocalAnchorRoundtrip`
- `ValidateCurvedBindingContinuityClassification`
- `ValidateNonSymmetricBindingIdentityStability`
- `ValidateRebindingAcrossAsymmetricCarrierReplacement`
- `ValidateParameterDomainRespect`
- `ValidateNoPlanarShortcutFallback`

These validators exist specifically to prevent Milestone 5 from quietly
remaining planar-only or symmetry-dependent.

## What The Current Inventories Still Do Not Name Cleanly Enough

The current inventories are strong, but Milestone 5 still requires a few
operator and validator families to become more explicit than they are today.

The biggest missing or undernamed pieces are:

### Operators

- `AttachVertexGeometry`
- `BindParameterSpacePointAnchor`
- `BindParameterSpaceDirectionAnchor`
- `EvaluateBindingCompleteness`
- `EvaluateBindingIdentity`
- `EvaluateRebindingCandidates`
- `ClassifyRebindingOutcome`
- `ExplainRebindingOutcome`
- `InspectBindingAtCheckpoint`
- `InspectBindingTransition`
- `InspectBranchLocalBindingState`

### Validators

- `ValidateBindingCompleteness`
- `ValidateBindingIdentitySeparation`
- `ValidateRebindingDeterminism`
- `ValidateRebindingOutcomeClassification`
- `ValidateHistoricalBindingInspectionParity`
- `ValidateBranchLocalBindingInspectionParity`
- `ValidateCanonicalBindingDigestStability`
- `ValidateAnchorCarrierOwnership`
- `ValidateParameterSpaceAnchorResolution`
- `ValidateCarrierLocalAnchorRoundtrip`
- `ValidateCurvedBindingContinuityClassification`
- `ValidateNonSymmetricBindingIdentityStability`
- `ValidateRebindingAcrossAsymmetricCarrierReplacement`
- `ValidateParameterDomainRespect`
- `ValidateNoPlanarShortcutFallback`

## First-Pass Slice Order For Operators And Validators

If we implement the milestone in slices, the clean first-pass order is:

### Slice 1: `worth-spatial` binding authority

Operators:

- `AttachSurfaceToFace`
- `ReplaceSurfaceOnFace`
- `AttachCurveToEdge`
- `ReplaceCurveOnEdge`
- `AttachPCurveToCoedge`
- `ReplacePCurveOnCoedge`
- `AttachVertexGeometry`
- `EvaluateBindingCompleteness`
- `EvaluateBindingIdentity`

Validators:

- `ValidateCurveBoundToEdge`
- `ValidatePCurveBoundToCoedge`
- `ValidateBindingCompleteness`
- `ValidateBindingIdentitySeparation`

### Slice 2: `worth-spatial` anchor authority

Operators:

- `BindParameterSpacePointAnchor`
- `BindParameterSpaceDirectionAnchor`

Validators:

- `ValidateAnchorCarrierOwnership`
- `ValidateParameterSpaceAnchorResolution`
- `ValidateCurveSurfaceInversionResiduals`

### Slice 3: `worth-spatial` rebinding authority

Operators:

- `EvaluateRebindingCandidates`
- `ClassifyRebindingOutcome`
- `ExplainRebindingOutcome`

Validators:

- `ValidateRebindingDeterminism`
- `ValidateRebindingOutcomeClassification`

### Slice 4: history, branch, and curved pressure

Operators:

- `InspectBindingAtCheckpoint`
- `InspectBindingTransition`
- `InspectBranchLocalBindingState`

Validators:

- `ValidateHistoricalBindingInspectionParity`
- `ValidateBranchLocalBindingInspectionParity`
- `ValidateCanonicalBindingDigestStability`
- `ValidateCarrierLocalAnchorRoundtrip`
- `ValidateCurvedBindingContinuityClassification`
- `ValidateNonSymmetricBindingIdentityStability`
- `ValidateRebindingAcrossAsymmetricCarrierReplacement`
- `ValidateParameterDomainRespect`
- `ValidateNoPlanarShortcutFallback`

### Required `worth-topo` support throughout these slices

- `GetFaceLoops`
- `GetFaceEdges`
- `GetLoopCoedges`
- `GetCoedgeEdge`
- `GetEdgeVertices`
- `GetShellFaces`

and the topology-owned validators:

- `ValidateTwinSymmetry`
- `ValidateNextPrevSymmetry`
- `ValidateLoopClosure`
- `ValidateOwnership`
- `ValidateNoOrphans`
- `ValidateShellWatertightness`
- `ValidateRadialCycleClosure`
- `ValidateEdgeManifoldStateMatchesUseCount`

This is the first-pass operator and validator map Milestone 5 needs.

## Crate Ownership Rules

Milestone 5 should follow these ownership rules strictly.

### `worth-spatial`

`worth-spatial` owns:

- binding truth
- binding identity
- carrier-local anchor truth
- binding completeness
- rebinding evaluation
- rebinding outcome classification
- continuity classification for binding workflows
- binding-facing diagnostics
- historical and branch-local binding inspection
- binding-layer certification and proof

If a surface answers a question like:

- "what geometry is this topology entity bound to?"
- "what anchor on this carrier does this mean?"
- "what happened to this binding after replacement?"
- "why was this rebinding preserved, denied, or marked ambiguous?"

then it belongs in `worth-spatial`.

### `worth-topo`

`worth-topo` owns:

- topology truth
- topology-safe navigation and query helpers
- topology legality validators
- topology-owned history/query support

If a surface answers a question like:

- "which loops belong to this face?"
- "which coedges belong to this loop?"
- "which vertices bound this edge?"
- "is this shell topologically closed?"

then it belongs in `worth-topo`.

`worth-topo` must not own geometry-binding meaning.

### `worth-kernel`

`worth-kernel` owns:

- workflow composition
- admitted authoring and orchestration
- kernel-facing entry surfaces that consume spatial contracts
- milestone-level scenario assembly and certification pressure

If a surface is about:

- preparing a binding workflow
- composing a rebinding workflow
- routing one admitted workflow through `worth-spatial` and `worth-topo`
- packaging a certification scenario

then it belongs in `worth-kernel`.

`worth-kernel` should not own binding legality, anchor truth, or rebinding
classification itself.

### `worth-geom`

`worth-geom` owns:

- carrier evaluation
- inversion
- closest-point and parameter-domain math
- asymmetric curved carrier support math
- witness-carrier construction support for admitted families

If a surface is about:

- evaluating a curve or surface
- inverting xyz to parameter space
- finding closest points
- representing or interrogating an admitted curved carrier

then it belongs in `worth-geom`.

`worth-geom` must not own binding authority.

## Forge Query Alignment And Runtime Boundary

The Query 9.3.7 and 9.3.8 surfaces make the runtime boundary clear:

- Query owns the public entry grammar
- Query owns declaration progression, route planning, receipts, envelopes,
  inspection, and readiness
- Query owns the generic graph-shaped write surface
- lower runtimes remain authoritative for truth identity, continuity, naming,
  verification, and writeback semantics

Milestone 5 should align to that shape instead of inventing a Worth-local
shadow runtime.

### What Milestone 5 should not create

- a Worth-local pseudo-Query declaration seam
- a Worth-local graph write engine
- a Worth-local receipt or envelope family for binding writes
- a Worth-local write inspection system that competes with Query's retained
  write artifacts

### What Milestone 5 should create

- Worth-owned binding semantics
- Worth-owned rebinding semantics
- Worth-owned topology and geometry legality inputs
- Worth-owned lowering from admitted binding intent into Query-owned graph
  authoring or later declaration-entry surfaces
- Worth-owned invariant hooks and certification pressure over those Query-owned
  surfaces

### Clean boundary by crate

`worth-kernel` should own:

- workflow assembly
- admitted binding authoring intent
- admitted rebinding workflow intent
- translation from Worth workflow meaning into Query-facing graph programs or
  declaration-entry requests

`worth-spatial` should own:

- the semantic truth that decides what binding or rebinding means
- candidate evaluation
- continuity and ambiguity classification
- the exact support facts a Query-lowered graph program is trying to preserve

`worth-topo` should own:

- topology navigation facts used before write authoring
- topology legality proof consumed by binding and rebinding decisions

`worth-geom` should own:

- carrier evaluation and inversion facts used before authoring or during
  validation

`forge-query` should own, when touched:

- the generic declaration-entry, route, receipt, envelope, readiness, and
  inspection seams
- the generic `compose_graph(...)` / graph-composition write family
- the generic graph-composition receipts, denial artifacts, and lifecycle
  evidence

### Practical rule

If a Milestone 5 write needs:

- symbolic handles
- mixed creation plus retarget/update/retire semantics
- identity-preserving rewrite evidence
- canonical write receipts
- graph-shaped denied-path diagnostics

then it should lower through Query graph composition rather than through a
Worth-local batch or relation-rewrite story.

If a Milestone 5 step is about:

- deciding whether a rebinding is legal
- deciding what continuity class applies
- deciding what carrier-local anchor survives

then that decision belongs in Worth before or alongside the Query-lowered
write, not inside Query.

## Proposed Milestone 5 Directory Skeletons

These skeletons are meant to keep the milestone scalable.

They are not a demand to create every file immediately.

They are the structure the work should grow toward so we do not end up with a
pile of unrelated helpers.

## `worth-spatial` Skeleton

Milestone 5 should deepen `worth-spatial` around binding authority as a
first-class domain.

```text
crates/worth-spatial/src/
  facade/
    bindings.rs
    diagnostics.rs
    history.rs
    identity.rs

  bindings/
    mod.rs

    authority/
      mod.rs
      face_surface.rs
      edge_curve.rs
      coedge_pcurve.rs
      vertex_geometry.rs
      completeness.rs

    anchors/
      mod.rs
      parameter_space_point.rs
      parameter_space_direction.rs
      carrier_ownership.rs
      resolution.rs

    rebinding/
      mod.rs
      candidate_evaluation.rs
      outcome_classification.rs
      continuity.rs
      diagnostics.rs

    identity/
      mod.rs
      binding_identity.rs
      anchor_identity.rs
      rebinding_identity.rs
      digest_protocol.rs

    history/
      mod.rs
      checkpoint_inspection.rs
      transition_inspection.rs
      branch_local_inspection.rs

    curved_pressure/
      mod.rs
      asymmetric_family.rs
      curved_binding_cases.rs

    certification/
      mod.rs
      completeness.rs
      rebinding_determinism.rs
      history_parity.rs
      curved_pressure.rs
```

### Why this shape

- `authority/` keeps the actual binding truth family together
- `anchors/` keeps carrier-local anchor semantics from dissolving into generic
  lowering helpers
- `rebinding/` gives replacement workflows one honest home
- `identity/` prevents digest and identity truth from being smeared into random
  files
- `history/` keeps inspection and branch-local work visible
- `curved_pressure/` keeps the asymmetric pressure cases explicit rather than
  hidden in generic fixtures

## `worth-topo` Skeleton

Milestone 5 should not widen `worth-topo` into geometry-binding logic.

It should only add or strengthen the topology-safe query/navigation support
that `worth-spatial` needs.

```text
crates/worth-topo/src/
  projection/
    read_views/
      domain/
        views/
          topology_navigation/
            mod.rs
            face_loops.rs
            face_edges.rs
            loop_coedges.rs
            coedge_edge.rs
            edge_vertices.rs
            shell_faces.rs

  certification/
    topology_navigation/
      mod.rs
      face_loops.rs
      face_edges.rs
      loop_coedges.rs
      coedge_edge.rs
      edge_vertices.rs
      shell_faces.rs

  validation/
    reference_integrity/
    loop_wiring/
    ownership/
    shell_closure/
    radial_rings/
```

### Why this shape

- the Milestone 5 support queries are read/navigation surfaces
- they should live near projection/read-view language, not mutation logic
- their certification should be separate from binding certification because the
  questions are topology-only

### Milestone 5 support queries that belong here

- `GetFaceLoops`
- `GetFaceEdges`
- `GetLoopCoedges`
- `GetCoedgeEdge`
- `GetEdgeVertices`
- `GetShellFaces`

## `worth-kernel` Skeleton

Milestone 5 should keep `worth-kernel` thin and workflow-shaped.

It should also be the only Worth crate that speaks directly to Query's public
write and orchestration seams for these workflows.

```text
crates/worth-kernel/src/
  binding/
    mod.rs

    authoring/
      mod.rs
      intents.rs
      workflow.rs

    rebinding/
      mod.rs
      workflow.rs

    inspection/
      mod.rs
      history.rs
      branch_local.rs

    certification/
      mod.rs
      scenarios.rs
      curved_pressure.rs

  facade/
    authoring/
    diagnostics/
    certification/
```

### Why this shape

- this keeps Milestone 5 work out of the existing construction subtree except
  where interoperability is required
- it makes "binding workflows" a named kernel responsibility instead of hiding
  them in generic authoring buckets
- it gives certification scenarios one clear home
- it keeps Query-facing lowering concentrated in one orchestration boundary
  instead of leaking runtime graph authoring across Worth

### What should live here

- `AuthorPrimitiveBindingIntent`
- `PrepareAdmittedBindingWorkflow`
- `PrepareAdmittedRebindingWorkflow`
- `PrepareBindingInspectionWorkflow`
- `PrepareBindingCertificationScenario`

### What should not live here

- binding legality rules
- parameter-space anchor authority
- rebinding outcome classification
- topology navigation logic

Those belong in `worth-spatial` or `worth-topo`.

## `forge-query` Touch Surface If Milestone 5 Exposes A Generic Gap

Milestone 5 is not primarily a `forge-query` milestone.

But if Worth pressure exposes a missing generic runtime seam, the fix should
land in Query's existing generic subsystems rather than as Worth-specific
runtime glue.

```text
crates/forge-query/src/
  declaration_entry/
    seam/
    inspection/
    readiness/
    orchestration/

  runtime/
    mutation/
      graph_composition/
        builder/
        declarations/
        lifecycle/
        lowering/
        denial/
        hooks/

    support/
      graph_composition/

    surface/
      graph_composition/

    inspection/
      unified/
```

### What belongs here if Worth exposes a real generic gap

- new generic graph-composition capability families
- new generic lifecycle-outcome or denial distinctions
- new generic inspection or readiness projections for graph composition
- new generic lowering hooks where a domain contributes meaning but Query keeps
  runtime artifact authority

### What does not belong here

- Worth-specific binding legality
- Worth-specific rebinding taxonomies
- Worth-specific carrier geometry logic
- Worth-specific topology navigation helpers

If a change only matters to Worth binding semantics, it should stay in Worth.

If the change is truly "serious downstream domains need Query to express this
graph-shaped write honestly," then it belongs in Query.

## `worth-geom` Skeleton

Milestone 5 should use `worth-geom` for carrier math and admitted curved-family
support, not for binding authority.

```text
crates/worth-geom/src/
  curve/
    eval.rs
    inversion.rs
    closest_point.rs

  surface/
    eval.rs
    inversion.rs
    closest_point.rs
    parameter_domains.rs

  primitives/
    shape_realization/
    asymmetric_curved/
      mod.rs
      stretched_spheroid.rs
      capsule_ovaloid.rs

  certification/
    carrier_roundtrip/
      mod.rs
      curve_parameter_roundtrip.rs
      surface_parameter_roundtrip.rs
      asymmetric_curved_roundtrip.rs
```

### Why this shape

- evaluation and inversion stay geometry-owned
- asymmetric curved support becomes an explicit admitted family instead of
  leaking into generic primitive helpers
- roundtrip certification stays near the math substrate that owns it

## Recommended File Placement By Operator Family

This section maps the first-pass operator inventory into the proposed
directories.

## `worth-spatial`

### `bindings/authority/`

- `AttachSurfaceToFace`
- `ReplaceSurfaceOnFace`
- `AttachCurveToEdge`
- `ReplaceCurveOnEdge`
- `AttachPCurveToCoedge`
- `ReplacePCurveOnCoedge`
- `AttachVertexGeometry`
- `EvaluateBindingCompleteness`
- `EvaluateBindingIdentity`

### `bindings/anchors/`

- `BindParameterSpacePointAnchor`
- `BindParameterSpaceDirectionAnchor`

### `bindings/rebinding/`

- `EvaluateRebindingCandidates`
- `ClassifyRebindingOutcome`
- `ExplainRebindingOutcome`

### `bindings/history/`

- `InspectBindingAtCheckpoint`
- `InspectBindingTransition`
- `InspectBranchLocalBindingState`

### `bindings/authority/` or `bindings/rebinding/` after basic closure

- `ReverseEdgeCurveSense`
- `ReparameterizeEdgeCurve`
- `NormalizeEdgeCurveDomain`
- `ReversePCurveSense`
- `ReparameterizePCurve`
- `NormalizePCurveDomain`
- `SwapSurfaceParameterization`
- `NormalizeSurfaceParameterization`
- `SetFaceSurfaceSense`
- `SetEdgeCurveSense`
- `SetCoedgeSense`

## `worth-topo`

### `projection/read_views/domain/views/topology_navigation/`

- `GetFaceLoops`
- `GetFaceEdges`
- `GetLoopCoedges`
- `GetCoedgeEdge`
- `GetEdgeVertices`
- `GetShellFaces`

## `worth-kernel`

### `binding/authoring/`

- `AuthorPrimitiveBindingIntent`
- `PrepareAdmittedBindingWorkflow`

### `binding/rebinding/`

- `PrepareAdmittedRebindingWorkflow`

### `binding/inspection/`

- `PrepareBindingInspectionWorkflow`

### `binding/certification/`

- `PrepareBindingCertificationScenario`

## `worth-geom`

### `curve/`, `surface/`, and `primitives/asymmetric_curved/`

- `InvertSurfacePointToUV`
- `InvertCurvePointToT`
- `EvalSurfacePoint`
- `EvalSurfaceDerivatives`
- `EvalCurvePoint`
- `EvalCurveDerivatives`
- `ClosestPointOnSurface`
- `ClosestPointOnCurve`

## Recommended File Placement By Validator Family

## `worth-spatial`

### `bindings/certification/completeness.rs`

- `ValidateCurveBoundToEdge`
- `ValidatePCurveBoundToCoedge`
- `ValidateBindingCompleteness`

### `bindings/certification/identity.rs`

- `ValidateBindingIdentitySeparation`
- `ValidateCanonicalBindingDigestStability`

### `bindings/certification/anchors.rs`

- `ValidateAnchorCarrierOwnership`
- `ValidateParameterSpaceAnchorResolution`
- `ValidateCurveSurfaceInversionResiduals`

### `bindings/certification/rebinding.rs`

- `ValidateRebindingDeterminism`
- `ValidateRebindingOutcomeClassification`

### `bindings/certification/history.rs`

- `ValidateHistoricalBindingInspectionParity`
- `ValidateBranchLocalBindingInspectionParity`

### `bindings/certification/curved_pressure.rs`

- `ValidateCarrierLocalAnchorRoundtrip`
- `ValidateCurvedBindingContinuityClassification`
- `ValidateNonSymmetricBindingIdentityStability`
- `ValidateRebindingAcrossAsymmetricCarrierReplacement`
- `ValidateParameterDomainRespect`
- `ValidateNoPlanarShortcutFallback`

### `bindings/certification/sense.rs`

- `ValidateSenseConsistency`
- `ValidateEveryCoedgeHasPCurveWhenRequired`
- `ValidatePCurveSenseMatchesCoedgeSense`

## `worth-topo`

### keep in the existing topology-owned validation families

- `ValidateTwinSymmetry`
- `ValidateNextPrevSymmetry`
- `ValidateLoopClosure`
- `ValidateOwnership`
- `ValidateNoOrphans`
- `ValidateShellWatertightness`
- `ValidateRadialCycleClosure`
- `ValidateEdgeManifoldStateMatchesUseCount`

These should not be rehomed into `worth-spatial`.

## `worth-geom`

### `certification/carrier_roundtrip/`

- roundtrip and inversion correctness checks supporting:
  - parameter-space point fidelity
  - parameter-space direction fidelity
  - asymmetric curved carrier roundtrip behavior

These should stay geometry-owned even when `worth-spatial` consumes them.

## Scalability Rules For Milestone 5 Implementation

To avoid a mess later, the implementation should follow these rules:

1. Do not create generic `helpers`, `utils`, or `misc` buckets.
2. Keep binding truth, anchor truth, rebinding, identity, history, and
   certification as visibly separate folders.
3. Do not hide topology navigation inside `worth-spatial`; consume it from
   `worth-topo`.
4. Do not hide carrier math inside `worth-kernel`; consume it from
   `worth-geom`.
5. Do not let `worth-kernel` become a second binding authority.
6. Do not let any Worth crate create a shadow Query runtime, graph-write, or
   receipt layer.
7. Put new validators beside the truth family they certify.
8. Put curved-pressure support in an explicitly named subtree so it remains an
   admitted pressure lane, not an accidental generic curved system.

## Immediate Structural Recommendation

If we start implementing Milestone 5 soon, the first directories worth adding
deliberately are probably:

### `worth-spatial`

- `bindings/authority/`
- `bindings/anchors/`
- `bindings/rebinding/`
- `bindings/identity/`
- `bindings/history/`
- `bindings/certification/`

### `worth-kernel`

- `binding/authoring/`
- `binding/rebinding/`
- `binding/inspection/`
- `binding/certification/`

### `forge-query` only if generic runtime pressure truly appears

- `runtime/mutation/graph_composition/`
- `runtime/support/graph_composition/`
- `runtime/surface/graph_composition/`
- `runtime/inspection/unified/`
- `declaration_entry/orchestration/`

### `worth-topo`

- `projection/read_views/domain/views/topology_navigation/`
- `certification/topology_navigation/`

### `worth-geom`

- `primitives/asymmetric_curved/`
- `certification/carrier_roundtrip/`

That is the cleanest first-pass crate and directory map for the milestone.
