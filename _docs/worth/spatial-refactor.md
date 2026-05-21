# Worth Spatial Refactor

## Goal

Define the refactor program that will make `worth-spatial` runtime-native
without lying about what still belongs to pre-runtime spatial semantics.

This document exists to drive a folder-by-folder rewrite of `worth-spatial`
until:

- pre-runtime spatial meaning is local, typed, explicit, and small
- installed/runtime-facing concerns are declared once and carried by Query
- numeric truth is admitted through `worth-math` instead of ambient `f64`
- spatial code stops rebuilding mini-runtime behavior locally
- the crate becomes a clean semantic partner to the Forge runtime instead of a
  second runtime-shaped system

This document was explored incrementally, but it should be implemented in a
clear dependency order rather than the order in which the sections were first
written.

## Why This Refactor Exists

`worth-spatial` is trying to become a serious spatial semantics layer for
Worth, but parts of it still behave like a self-contained kernel subsystem that
must personally carry:

- semantic admission
- numeric admission
- anchor interpretation
- execution posture
- local proof ceremony
- local diagnostics shaping
- and sometimes proto-runtime structure

That is the old pattern.

The runtime changes the rules.

Forge already has:

- Relational as graph and authoritative truth
- Signal as derived DAG and invalidation engine
- Bridge as truth-to-derived coherence seam
- Query as the ergonomic public operating language over admissions,
  workflows, receipts, lineage, diagnostics, subscriptions, and fact
  contracts

So `worth-spatial` should not try to become a second ambient runtime.

Its job is narrower and more powerful:

- own authored spatial meaning
- own local geometric and numeric admission before runtime truth exists
- lower domain-semantic asks into declared runtime-shaped asks
- preserve exactly the semantic facts the runtime should carry forward
- promote domain-originated semantic truth into Query-owned declarations,
  traces, hooks, receipts, diagnostics, and fact surfaces earlier than a
  normal kernel would

This refactor exists because `worth-spatial` is currently doing some of the
right semantic work with too much local ceremony, too much raw floating-point
meaning, and too little runtime-native declaration.

## Governing Summaries

- `MENTALITY.md`
  - Protects: solving the hard structural problem first instead of nibbling at
    features.
  - Main constraint here: spatial refactor must attack the real architectural
    failure mode first, which is duplicated runtime behavior and weak numeric /
    semantic boundaries, not cosmetic cleanup.

- `arch_laws.md`
  - Protects: proof-bearing phase progression, authority separation, declared
    effects, and explicit boundary artifacts.
  - Main constraint here: pre-runtime spatial phases must be explicit and
    honest, but runtime-facing lifecycle, evidence, and fact surfaces must be
    owned by Query rather than reinvented locally.

- `composition_laws.md`
  - Protects: named semantic steps instead of god files and helper swamps.
  - Main constraint here: the lowering folder must stop splitting one small
    semantic algebra across many ceremonial files and wrappers.

- `domain_structure_laws.md`
  - Protects: physical structure that preserves meaning, authority, and
    lifecycle distinctions.
  - Main constraint here: `worth-spatial` must separate authored semantics,
    local numeric admission, runtime-shaped lowering, and immediately
    runtime-facing
    operation families instead of flattening them into one broad lowering area.

- `perf_laws.md`
  - Protects: explicit cost surfaces, no repeated rediscovery, and no hidden
    breadth.
  - Main constraint here: spatial lowering must not repeatedly re-admit
    placements, re-resolve anchors, or re-discover numeric truth when that
    meaning could be admitted once and carried forward.

- `worth_roadmap.md`
  - Protects: Worth as a spec-native, AI-native, manufacturing-grade geometry
    system that uses Forge runtimes honestly rather than rebuilding them.
  - Main constraint here: `worth-spatial` owns topology/geometry interaction
    semantics, but runtime/query work must enter through Query and must not
    normalize local workaround architecture.

- `worth/test-requirements.md`
  - Protects: exact success or exact structured failure, replay honesty,
    diagnostics sufficiency, and workflow-class closure rather than toy demos.
  - Main constraint here: any spatial refactor must improve typed failure,
    replay honesty, and machine-checkable semantics rather than merely
    rearranging code.

- `forge_query_vision.md`
  - Protects: Query as the typed, aspect-aware, live-promotable public layer
    for runtime truth.
  - Main constraint here: once spatial meaning becomes runtime-facing, it
    should be declared into Query surfaces such as basis, admission, workflow,
    effect, fact consumption, and diagnostics instead of being wrapped again in
    Worth-local operational vocabulary.

- `forge_query_roadmap.md`
  - Protects: Query as the finished runtime-facing composition layer, including
    9.3.x public runtime grammar.
  - Main constraint here: the refactor should assume Query already owns the
    runtime-facing "pretty bows" for admission, effects, consumed facts,
    inspections, receipts, and boundary routing.

- `test-requirements-milestone-9_3-and-runtime-gates.md`
  - Protects: the public runtime-facing grammar for inspection, basis, effect
    execution, fact consumption, admission, and lower-runtime routing.
  - Main constraint here: when spatial semantics cross into runtime-facing
    operation families, the correct target is not a custom Worth wrapper but
    the Query-owned request -> admission -> handoff / plan -> receipt ->
    envelope -> certification progression.

## Adversarial Constraint

`worth-spatial` must survive this hostile condition:

> A long-lived branch-bearing geometry system with ambiguous anchors,
> feature-owned and tag-owned references, preview and replay posture,
> hostile numeric edge cases, identity evolution, and AI-authored spatial
> requests must preserve the same semantic admission result, the same numeric
> legality result, the same runtime-facing movement meaning, and the same
> clean-fail explanation regardless of whether the ask is evaluated as local
> authored intent, promoted immediately into runtime-backed workflow, replayed,
> consumed through maintained diagnostics.

If `worth-spatial`:

- lets raw `f64` values masquerade as admitted semantic truth
- lets anchor meaning stay disposable local glue instead of a declared semantic
  fact
- rebuilds runtime-facing admission, workflow, or evidence surfaces locally
- loses numeric failure meaning across movement families
- or forces downstream runtime-facing layers to rediscover what lowering
  already knew

then the refactor has failed.

## Runtime Mental Model

Relational is the truth-bearing graph and state authority. Signal is the
derived DAG and invalidation engine. Bridge keeps truth and derivation coherent
across runtime boundaries. Query is the ergonomic public operating layer that
turns all of that lower machinery into admissions, workflows, receipts,
provenance, lineage, diagnostics, subscriptions, and fact contracts.

This matters here because `worth-spatial` is not being asked to grow a second
truth, effect, or diagnostics runtime. It is being asked to produce the local
semantic and numeric truth that the runtime can then carry structurally. The
runtime does not need to invent spatial meaning in order to host it; Query
already has symbolic, extension-hook, invariant-pack, support-row, trace,
receipt, and inspection surfaces that can carry domain-originated semantics
once they stop being disposable local glue.

## Folder-By-Folder Program

This refactor will proceed folder by folder through `worth-spatial`.

For each folder we must answer:

1. what is true pre-runtime spatial meaning?
2. what is actually local numeric admission?
3. what runtime-facing concerns are being smuggled in locally?
4. what should be deleted, collapsed, or moved into `worth-math`?
5. what should transition into a Query-facing surface immediately rather than
   be rebuilt or rediscovered in Worth?

The implementation order for this refactor should be:

1. `spatial_intent/refs` + `spatial_intent/resolution`
   - authored symbolic vocabulary, witness admission, frame admission, and the
     immediate Query-facing symbolic/binding seam
2. `spatial_intent/lowering`
   - once refs and witness admission are cleaned up, lower against that tighter
     substrate instead of against today's duplicated helpers
3. `spatial_intent/arbitration`
   - make arbitration the canonical decision declaration before cleanup of
     preview and continuity
4. `spatial_intent/preview` + `spatial_intent/constraints` + `spatial_intent/continuity`
   - thin these into downstream declaration layers over the cleaned-up
     arbitration and lowering cores
5. `bindings`
   - primitive birth and its consequence posture after the core spatial intent
     slices are clearer
6. `certification` + `facade`
   - narrow and lock the public surface only after the internal slice seams are
     honest

The highest-leverage semantic knot is still `spatial_intent/lowering`, because
it currently combines:

- authored spatial intent admission
- local placement semantics
- anchor classification
- witness/catalog resolution
- movement and constraint application
- raw numeric normalization and rotation math
- and local proof/runtime ceremony

That makes it the right first boundary to cleanly split.

## Query API Mapping For This Refactor

This section exists so the refactor plan does not drift back into generic
"make it Query-shaped" language.

The exact adapter function names may evolve, but the family names below are
already strong enough to guide implementation now. If a spatial slice reaches a
runtime-facing seam and does not align to one of these Query families, that is
evidence we are about to invent a second operating surface instead of using the
one that already exists.

### `spatial_intent/refs` + `spatial_intent/resolution`

Use these Query families first:

- symbolic reference and target identity
  - `ForgeQuerySymbolicTargetReference`
  - `ForgeQuerySymbolicTargetReferenceFamily`
  - `ForgeQuerySymbolicTargetReferenceDenial`
  - `ForgeQuerySymbolicAspectReference`
  - `ForgeQuerySymbolicAspectReferenceFamily`
  - `ForgeQuerySymbolicAspectResolutionEvidence`
- existing-truth binding and probes
  - `ForgeQueryExistingTruthProbeRequest`
  - `ForgeQueryExistingTruthProbe`
  - `ForgeQueryExistingTruthProbeReceipt`
  - `ForgeQueryExistingTruthProbeRoutingPreflight`
  - `ForgeQueryExistingTruthTargetBinding`
  - `ForgeQueryExistingTruthBindingFamily`
- read and composition extension hooks when spatial refs must stay domain-owned
  - `ForgeQueryReadCompositionExtensionHookBoundary`
  - `ForgeQueryReadCompositionExtensionHookSupportRow`
  - `ForgeQueryGraphCompositionExtensionHookBoundary`
  - `ForgeQueryGraphCompositionExtensionHookSupportRow`

What this means in practice:

- feature-owned, tag-owned, parameter-space, frame-derived, and world-derived
  spatial references should stop being dead-end local categories
- the same refactor wave should shape them so Query can carry their symbolic
  identity, denial posture, and target-binding posture directly

### `spatial_intent/lowering`

Use these Query families first:

- runtime intent admission and decision posture
  - `ForgeQueryIntentDeclaration`
  - `ForgeQueryIntentAdmissionDecision`
  - `ForgeQueryIntentAdmissionSupportPosture`
  - `ForgeQueryIntentAdmissionSupportRow`
  - `ForgeQueryIntentDecisionTraceEnvelope`
  - `ForgeQueryIntentExecutionProvenance`
- effect lifecycle once admitted spatial motion or constraint families become
  runtime-facing acts
  - `AdmittedEffectIntent`
  - `AuthorityScopedEffectPlan`
  - `LoweredEffectExecutionPlan`
  - `EffectExecutionReceipt`
  - `SelfDescribingEffectEnvelope`
- projection / aftermath fact consumption
  - `declare_projection_consumption`
  - `AdmittedProjectionConsumption`
  - `CompletedProjectionFactConsumption`
  - `ConsumedProjectionFactSet`
  - `BoundProjectionFactFamily`

What this means in practice:

- lowering should originate anchor and numeric semantics locally
- the moment those semantics define a real runtime-facing move, rotate,
  reorient, lies-on, points-toward, or anchor-match posture, the same refactor
  wave should map them to intent admission, effect, and fact-consumption
  families instead of collapsing them into local placement-only aftermath

### `spatial_intent/arbitration`

Use these Query families first:

- intent admission and support discovery
  - `ForgeQueryIntentAdmissionEligibility`
  - `ForgeQueryIntentAdmissionSupportEligibility`
  - `ForgeQueryIntentAdmissionSupportMatrix`
  - `ForgeQueryIntentAdmissionSupportTraceabilityReport`
  - `ForgeQueryIntentAdmissionDecision`
  - `ForgeQueryIntentAdvisoryDecision`
  - `ForgeQueryIntentViolationDecision`
- graph-composition support and domain invariant posture
  - `ForgeQueryGraphCompositionCapabilitySupportRow`
  - `ForgeQueryGraphCompositionDomainInvariantSummary`
  - `ForgeQueryGraphCompositionDomainInvariantDenial`
  - `ForgeQueryGraphCompositionInvariantPackContext`
  - `ForgeQueryGraphCompositionInvariantPackViolation`

What this means in practice:

- `SpatialIntentArbitrationAnalysis` should become the local semantic source
- but support, blocked, deferred, conflict, and escalation posture should map
  immediately onto Query admission and graph-composition support surfaces
- preview and continuity should then consume that declared story instead of
  restating it in local-only summary structs

### `preview` + `constraints` + `continuity`

Use these Query families first:

- preview and scoped session workflow
  - `admit_preview_workflow_foundation`
  - `PreviewWorkflowFoundationArtifact`
  - `PreviewSessionPlanBinding`
  - `PromotionEligiblePreviewExecutionEnvelope`
  - `ReadOnlyPreviewExecutionEnvelope`
- continuity / identity evolution / lineage
  - `ForgeQueryContinuityMutationIntent`
  - `ForgeQueryContinuityMutationEvidence`
  - `ForgeQueryContinuityOutcomeClass`
  - `LineageContinuity`
  - `IdentityEvolutionResultBundle`
- basis lifecycle for preview, observation, replay, and continuation posture
  - `admit_preview_closeout_basis`
  - `admit_observation_basis`
  - `admit_replay_basis`
  - `BasisUseReceipt`
  - `SelfDescribingBasisEnvelope`

What this means in practice:

- preview should move toward Query preview workflow families immediately,
  instead of remaining a local simulation summary
- continuity should move toward Query continuity and lineage families
  immediately, instead of remaining a local classification table with booleans
- constraints should hand their admitted relation posture into the same
  admission / effect / fact-consumption grammar instead of stopping at local
  placement mutation

### `bindings`

Use these Query families first:

- graph composition and lifecycle aftermath
  - `ForgeQueryGraphCompositionBuilder`
  - `ForgeQueryGraphCompositionLifecycleOutcomes`
  - `ForgeQueryGraphCompositionLineageSummary`
  - `ForgeQueryGraphCompositionResolutionMap`
- effect lifecycle and mutation evidence
  - `EffectFamily`
  - `EffectReceiptTargetEvidence`
  - `ForgeQueryMutationTargetEvidence`
  - `ForgeQueryMutationCausalityEvidence`
  - `ForgeQueryMutationProvenanceEvidence`
- consumed fact aftermath
  - `ConsumedEntityIdentityFact`
  - `ConsumedRelationEndpointFact`
  - `ConsumedSourceReferenceFact`
  - `ConsumedEffectContinuityFact`

What this means in practice:

- primitive birth should keep its geometric/topology contract semantics local
- but completeness, mapping, created-identity, endpoint, source-reference, and
  continuity aftermath should be shaped for immediate Query graph-composition,
  mutation-evidence, and fact-consumption carriage

### `certification` + `facade`

Use these Query families first:

- public/runtime-facing admission and closure artifacts
  - `ForgeQueryIntentAdmissionCertificationBundle`
  - `ForgeQueryIntentAdmissionPublicBoundaryAudit`
  - `ForgeQueryIntentAdmissionProofShapeAudit`
- lower-runtime boundary and routing closure
  - `ForgeQueryLowerRuntimeBoundaryEnvelope`
  - `ForgeQueryLowerRuntimeBoundaryExecutionReceipt`
  - `ForgeQueryLowerRuntimeSupportMatrix`
  - `ForgeQueryLowerRuntimeCloseoutReport`
- diagnostics and inspection
  - `request_causal_inspection`
  - `AdmittedQueryCausalInspectionArtifact`
  - `AdvisoryQueryCausalInspectionArtifact`
  - `DeniedQueryCausalInspectionArtifact`

What this means in practice:

- `worth-spatial` certification should stop acting like the final runtime
  operating surface test harness
- it should instead prove that:
  - the local semantic core is honest
  - the Query transition seam is explicit
  - runtime-facing explanation, receipt, and certification posture already
    routes through Query families rather than through bespoke spatial wrappers

### Rule For Using This Mapping

When implementing a section of this refactor:

1. identify the irreducible local spatial semantic core
2. identify the first honest runtime-facing seam
3. bind that seam to one of the Query families above immediately
4. if no family fits, verify it carefully before inventing anything new

The burden of proof is now on local reinvention, not on Query use.

### `spatial_intent/lowering`

#### Current Problem

`spatial_intent/lowering` is doing several good semantic things, but the shape
is too large, too raw-float-heavy, and too ceremonial for what it actually is.

The current slice does preserve useful distinctions:

- authored spec versus admitted form
- point-like versus directional anchor meaning
- subject-owned versus external versus feature-owned versus geometric-tag
  meaning
- typed witness/tag failure classes
- separation between placement admission and placement mutation

But it currently pays for those distinctions with too much local scaffolding:

- raw `f64` / `[f64; 3]` values carrying many different semantic roles
- heavy `forge-proof` ceremony in anchor progression
- repeated `with_catalog` / non-catalog forks
- repeated re-admission of placement and frame truth
- repeated anchor classification and resolution paths
- operation-specific error remapping boilerplate
- local transform helpers and local semantic helpers split across too many
  files

The result is a folder that reads like a tiny runtime instead of a tight
pre-runtime lowering boundary.

The highest-signal examples are:

- [placement_anchor_progression.rs](C:/Users/shepworth/Documents/programming/forge-2/crates/worth-spatial/src/spatial_intent/lowering/placement_anchor_progression.rs)
  where proof artifacts and local authority minting dominate a semantic
  classification problem
- [placement_motion.rs](C:/Users/shepworth/Documents/programming/forge-2/crates/worth-spatial/src/spatial_intent/lowering/placement_motion.rs)
  where each motion family redoes anchor-lowering and placement-admission work
- [placement_constraints.rs](C:/Users/shepworth/Documents/programming/forge-2/crates/worth-spatial/src/spatial_intent/lowering/placement_constraints.rs)
  where constraint application repeats much of the same pattern
- [placement_anchor_directions.rs](C:/Users/shepworth/Documents/programming/forge-2/crates/worth-spatial/src/spatial_intent/lowering/placement_anchor_directions.rs)
  where raw vector normalization, `acos`, coincidence logic, and fallback axis
  math live directly in the semantic layer

This is exactly the kind of folder that should become much smaller and much
more truthful after refactor.

At the code level, the current public surface is much narrower than the
implementation beneath it. The folder currently exports only three real
families of behavior:

1. local admission
   - `admit_spatial_placement*`
   - `admit_spatial_move*`
   - `admit_spatial_rotate*`
   - `admit_spatial_reorient*`
   - `admit_spatial_offset`

2. placement motion application
   - `apply_admitted_move_to_placement*`
   - `apply_admitted_offset_to_placement*`
   - `apply_admitted_reorient_to_placement*`
   - `apply_admitted_rotate_to_placement*`

3. placement constraint application
   - `apply_admitted_lies_on_constraint_to_placement*`
   - `apply_admitted_points_toward_constraint_to_placement*`
   - `apply_admitted_anchor_match_constraint_to_placement*`

Everything else in this folder is support structure for those three behavior
families. That is why the implementation can and should shrink so much.

#### Runtime-Native Rewrite Direction

The rewrite should **not** turn `lowering` into a Query wrapper.

This folder still owns pre-runtime spatial meaning. What changes is:

- it should become smaller, sharper, and more typed locally
- it should stop rebuilding mini-runtime semantics
- it should emit admitted semantic facts that Query can carry immediately,
  through
  declarations, support posture, traces, hooks, receipts, inspections, and
  fact families, instead of waiting until all the meaning has been collapsed
  away

The target design is:

1. authored specs remain local Worth types
2. local numeric and semantic admission become tighter and more explicit
3. anchor meaning is resolved once through a central semantic algebra
4. placement and motion application operate over admitted local semantic forms
5. the resulting movement semantics are ready to be declared into Query-owned
   basis/admission/workflow/effect/fact lifecycles when they cross into
   runtime-facing operation families

The most important rewrite law for this folder is:

> Keep pre-runtime semantics local.  
> Promote runtime-facing meaning early.  
> Do not keep disposable local intermediates once Query has an honest place to
> carry them.

#### What Must Stay Local

The following still belong in `worth-spatial` and should not be pushed down
into Query:

- authored spatial placement, motion, and constraint intent
- anchor meaning before installed/runtime binding exists
- local ambiguity between point-like and directional meaning
- local geometric legality of authored placement and motion asks
- local topological/spatial lowering legality before runtime truth exists
- domain-specific interpretation of witness and tag semantics
- operator-specific spatial meaning such as:
  - move
  - offset
  - rotate
  - reorient
  - lies-on
  - points-toward
  - anchor-match

This folder is still the right home for:

- "what does this ask mean?"
- "is this ask coherent?"
- "what admitted local spatial object does it become?"

That is pre-runtime work and should remain here. But "remain here" does not
mean "stay invisible to the runtime forever." Once those answers start shaping
runtime-facing admission, execution, fact consumption, binding, or diagnostics,
the semantic result should transition into Query-owned carrying structures
immediately rather than being rediscovered from raw payloads.

#### What Must Move Down Into `worth-math`

The current raw `f64` surface is too weak for the laws we are trying to
enforce.

A raw `f64` or `[f64; 3]` is currently being used to mean too many things:

- point
- local point
- world point
- offset vector
- free vector
- direction candidate
- admitted direction
- rotation axis
- angle
- distance-like coincidence delta
- plane coefficients

Those are not the same kind of truth, and they do not fail the same way.

The rewrite should therefore push numeric primitives and checked operations into
`worth-math`, including at least:

- finite point and vector primitives
- admitted direction or unit-vector primitives
- admitted rotation-axis primitives
- admitted angle primitives
- finite nonnegative distance or coincidence primitives where needed
- checked normalization
- checked cross/dot/angle derivation
- checked plane embedding helpers

The law is:

- raw `f64` may still exist inside low-level numeric implementation
- semantic and admission boundaries should stop exposing raw `f64` as if it
  were already admitted spatial truth

This matters especially because movement families fail differently:

- move failures are not rotate failures
- rotate-axis failure is not coincidence failure
- reorient ambiguity is not offset invalidity
- plane embedding failure is not point-witness failure

The numeric layer should preserve that difference instead of erasing it into
anonymous arrays.

#### What Runtime Hooks Already Exist For These Semantics

This folder is not where Query starts, but it is a place where several
runtime-facing meanings should stop being ephemeral. Query already has real
surfaces for domain-originated semantics to attach to, including:

- symbolic references and symbolic target/aspect references
- graph-composition and read-composition extension-hook boundaries
- graph-composition domain-invariant summaries and denials
- invariant-pack violations and support rows
- intent declarations, admission traces, and support traceability
- existing-truth binding and target-evidence surfaces
- effect inspection evidence
- causal inspection artifacts, decision traces, and representative evidence
- receipts, envelopes, and certification bundles for runtime-facing outcomes

This means the choice is not just:

- keep semantics local forever
- or pretend Query should invent them from scratch after the fact

The real choice is:

- originate the semantic truth locally
- then attach it to the runtime at the first honest seam, as soon as it starts
  to matter outside the
  local lowering step

#### What Should Transition Into Query Immediately

Once a movement or constraint family is no longer purely local, the same
refactor wave should declare into Query:

- basis posture
- admitted movement family
- admitted anchor meaning
- admitted numeric contract
- workflow family
- effect family
- existing-truth binding or target-evidence requirements
- fact-consumption aftermath
- runtime-facing diagnostics and receipts

In practice that means this refactor should preserve and surface semantic facts
that Query must own immediately, such as:

- point-like versus directional anchor meaning
- feature-owned versus tag-owned versus external reference classes
- movement-family-specific numeric admission
- exact denial classes for ambiguity, unsupported meaning, and invalid numeric
  posture
- target-reference posture that binding and inspection surfaces will need
- aspect-relevant semantic slices that fact, invalidation, and
  diagnostics surfaces should preserve

The anti-pattern is:

- classify locally
- apply locally
- discard the semantic truth
- force runtime-facing code to rediscover it

The target is:

- classify once
- admit once
- carry forward as declared meaning

That is where the runtime opportunity lives for this folder. The runtime is not
too late for these semantics. The runtime is the place where they should stop
being disposable.

#### Concrete Rewrite: What We Should Actually Do

This folder should be rewritten in this order.

##### Step 1: Delete the local proof-artifact architecture

`placement_anchor_progression.rs` is the first target.

Today it creates:

- `ClassifiedAnchorArtifact`
- `LoweredPointAnchorArtifact`
- `LoweredSubjectAnchorArtifact`
- `LoweredTranslationAnchorArtifact`
- `LoweredReorientAnchorArtifact`
- local `AuthorityWitness`
- local proof markers and proof-set composition

That entire shape should be deleted.

The replacement should be ordinary semantic types, for example:

- `ResolvedPointAnchor`
- `ResolvedTranslationAnchor`
- `ResolvedReorientAnchor`
- `AnchorSemanticClass`

The point is not these exact names. The point is:

- no `forge-proof` in `lowering`
- no local proof authorities in `lowering`
- no artifact aliases in `lowering`
- no phase theater in `lowering`

The proof-bearing story belongs at the Query-facing seam, when these semantics
are promoted into runtime artifacts immediately after local admission.

##### Step 2: Replace anchor progression with one central anchor resolver

Right now anchor semantics are spread across:

- `placement_anchor_progression.rs`
- `placement_anchor_points.rs`
- `placement_anchor_directions.rs`
- `placement_anchor_resolution.rs`

That should collapse into one semantic center, probably `anchors.rs`, with one
context and a few direct entry points.

The new internal surface should look more like:

- `resolve_point_anchor(ctx, anchor) -> Result<ResolvedPointAnchor, PointAnchorError>`
- `resolve_translation_anchor(ctx, anchor) -> Result<ResolvedTranslationAnchor, PointAnchorError>`
- `resolve_reorient_anchor(ctx, anchor) -> Result<ResolvedReorientAnchor, ReorientAnchorError>`

Where `ctx` contains:

- the placement spec
- the admitted placement
- the admitted reference frame
- the witness catalog

That removes repeated:

- `classify_anchor(...)`
- `lower_*_with_catalog(...)` versus `lower_*_without_catalog(...)`
- payload extraction from artifacts
- re-resolution of placement/frame state

##### Step 3: Stop duplicating catalog and no-catalog paths internally

Internally, every path should use one lowering context with a catalog.

That means:

- public no-catalog helpers can remain as ergonomic wrappers
- internal logic should always receive a catalog, using
  `EmptySpatialWitnessCatalog` for the default path

This should eliminate the repeated twin functions across:

- `motion.rs`
- `placement.rs`
- `placement_motion.rs`
- `placement_constraints.rs`
- anchor-lowering helpers

The internal rule should be:

- one implementation path
- optional public wrapper layer
- no semantic forks by catalog posture

##### Step 4: Pull numeric meaning out of raw arrays

The rewrite should identify every numeric role now hidden in raw `f64` and
`[f64; 3]` and replace the semantic boundary with admitted numeric forms from
`worth-math`.

The first targets should be:

- move destination point
- offset vector
- rotate axis
- reorient facing direction
- points-toward facing delta
- lies-on projection offset
- plane embedding helpers

That means the current signatures and data fields that expose raw arrays should
be treated as rewrite targets, especially in:

- `placement.rs`
- `motion.rs`
- `placement_motion.rs`
- `placement_constraints.rs`
- `placement_anchor_directions.rs`
- `placement_motion_support.rs`

The rule is:

- low-level math may still use raw floats internally
- semantic structs and lowering outputs should not

##### Step 5: Merge repeated motion logic into one transform application layer

`placement_motion.rs` repeats the same pattern several times:

- resolve anchor
- extract world point or direction
- map operation-specific errors
- translate or rotate placement
- re-admit placement/frame truth in some paths

This should collapse into a single internal operation layer over the resolved
anchor semantics.

The clearest duplication to remove:

- `apply_admitted_move_to_placement` and `_with_catalog`
- `apply_admitted_offset_to_placement` and `_with_catalog`
- `apply_admitted_reorient_to_placement` and `_with_catalog`
- `apply_admitted_rotate_to_placement` and `_with_catalog`

The rotate path is especially important:

- it currently re-admits placement
- re-admits frame
- manually special-cases `ShapeOrigin`, `WorldOrigin`, `FrameOrigin`, and
  `FeatureOwned`
- duplicates nearly identical pivot rotation logic between catalog and
  non-catalog branches

That should become:

- resolve rotate anchor once
- branch only on resolved semantic kind
- apply one shared rotation transform pipeline

##### Step 6: Merge repeated constraint logic into the same semantic core

`placement_constraints.rs` is repeating the same underlying operations as the
motion layer:

- resolve anchor
- extract world point
- subtract vectors manually
- translate placement manually
- project onto a frame manually
- map slightly different copies of the same anchor errors

This should be rebuilt over the same resolved anchor types and the same
transform helpers used by motion.

The clearest cuts:

- `map_points_toward_anchor_error`
- `map_lies_on_anchor_error`
- `map_anchor_match_anchor_error`
- `map_anchor_match_target_error`

Those should stop being four almost-identical functions.

##### Step 7: Preserve semantic facts so runtime layers can declare them

The lowering rewrite should not end at cleaner local code.

Each admitted lowering result should make it obvious which facts the same
refactor wave must promote into Query.

For lowering, the important facts are:

- anchor semantic class
  - subject-owned point
  - external point
  - feature-owned point
  - geometric-tag point
  - directional axis
  - unsupported carrier-local reference
- movement family
  - move
  - offset
  - rotate
  - reorient
  - lies-on
  - points-toward
  - anchor-match
- numeric posture
  - admitted point
  - admitted direction
  - admitted axis
  - admitted offset
  - coincidence failure
  - invalid existing placement
- denial posture
  - unsupported anchor
  - ambiguous anchor meaning
  - anchor witness failure
  - anchor tag failure

If runtime-facing code still has to re-derive those categories from raw
payloads, the rewrite has failed.

#### What To Delete Or Collapse

The refactor should be aggressive here.

The clearest cuts are:

1. most of the local proof-ceremony in
   [placement_anchor_progression.rs](C:/Users/shepworth/Documents/programming/forge-2/crates/worth-spatial/src/spatial_intent/lowering/placement_anchor_progression.rs)
   - local authority minting
   - proof markers
   - proof-set composition
   - artifact aliases for small semantic cases
   - classified-artifact versus lowered-artifact ceremony

2. duplicated catalog and non-catalog execution paths
   - public wrappers may remain ergonomic
   - internal logic should unify around one lowering context with an explicit
     catalog

3. fragmented anchor semantics spread across:
   - `placement_anchor_points.rs`
   - `placement_anchor_directions.rs`
   - `placement_anchor_resolution.rs`
   - `placement_anchor_progression.rs`

4. repeated placement/frame re-admission and repeated anchor lowering inside
   motion and constraint operations

5. repetitive operation-specific error translation where a thinner shared error
   boundary would suffice

More concretely:

- delete `placement_anchor_progression.rs` entirely after replacing it with a
  simpler semantic resolver
- delete `placement_anchor_points.rs` as a separate file and move the useful
  behavior into the central anchor resolver
- fold `placement_anchor_directions.rs` into either the same resolver or a very
  small numeric-direction helper module
- fold `placement_anchor_resolution.rs` into the same resolver unless a tiny
  pure-resolution file still earns its keep
- shrink `placement_motion_support.rs` to pure transform math only, or merge it
  into `transforms.rs`
- keep `motion.rs` and `placement.rs`, but tighten their numeric boundaries
- rebuild `placement_motion.rs` and `placement_constraints.rs` over the same
  resolved anchor and transform core instead of letting each remain its own
  micro-framework

Expected cut:

- production code in this slice is currently about 2,400 lines
- healthy target after rewrite is roughly 1,250 to 1,600 production lines
- that implies approximately 800 to 1,100 lines should disappear

This is not cosmetic trimming. It is real structural deletion.

#### Target File Topology

The target shape for the first rewrite pass should be much smaller and more
centralized, for example:

- `placement.rs`
  - placement spec
  - admitted placement
  - placement geometry embedding
- `motion.rs`
  - motion specs
  - admitted motion families
- `anchors.rs`
  - lowering context
  - anchor semantic algebra
  - point and directional anchor lowering
  - witness/tag/feature resolution hooks
- `operations.rs`
  - apply move / offset / rotate / reorient
  - apply lies-on / points-toward / anchor-match
- `transforms.rs`
  - small pure transform helpers built over admitted numeric forms

The important rule is not the exact filenames. It is:

- one semantic center for anchor meaning
- one admitted numeric boundary
- one lowering context
- one operation layer

That is much more honest than the current split.

The concrete target after rewrite should be approximately:

- `placement.rs`
  - still the home for authored placement and admitted placement
  - but with admitted numeric forms instead of casual arrays at the semantic
    boundary
- `motion.rs`
  - still the home for authored motion and admitted motion families
  - but with movement-specific admitted numeric types
- `anchors.rs`
  - all anchor semantic classification and resolution
  - one context
  - no proof artifacts
  - no public fork by catalog posture
- `operations.rs`
  - all placement mutation and constraint application over shared resolved
    semantics
- `transforms.rs`
  - pure geometry helpers only
  - no semantic branching

If that exact topology is not used, the replacement must still satisfy the same
shape:

- no progression theater
- no duplicated catalog path internals
- no separated point versus direction versus resolution micro-files
- no duplicated motion versus constraint anchor logic
- no raw numeric meaning leaking through semantic boundaries

#### Acceptance Evidence

The lowering refactor is only honest if it proves all of the following:

- authored spatial meaning is preserved across the rewrite
- point-like versus directional ambiguity remains explicit
- witness and tag failure classes remain typed and operation-specific
- movement-family-specific numeric failures stay distinct
- no raw `f64` semantic boundary remains where an admitted numeric type is
  required
- internal catalog / non-catalog lowering logic unifies without semantic drift
- runtime-facing meaning is already shaped for immediate Query declaration
- runtime hooks for carrying anchor meaning, numeric posture, and denial class
  are identified earlier and no longer assumed absent
- file and function topology becomes more predictable and less ceremonial

Concrete proof should include:

- parity tests across the current admitted workflow surface for move, offset,
  rotate, reorient, and placement constraints
- failure-topology tests proving ambiguity, unsupported-anchor, witness-failure,
  tag-failure, non-finite, and coincidence families remain distinct
- compile-time or construction-time enforcement wherever numeric admission can
  be moved into `worth-math`
- architectural QA proving the folder no longer behaves like a mini runtime
- explicit Query-transition notes for the semantic facts runtime-facing layers
  must consume immediately
- a code-structure diff proving:
  - `placement_anchor_progression.rs` is gone
  - internal catalog and no-catalog logic no longer fork repeatedly
  - motion and constraint application share the same anchor and transform core
  - rotate no longer duplicates pivot logic across catalog posture

## Next Categories

Once `spatial_intent/lowering` is specified and rewritten, the next
categories to walk should be:

- `spatial_intent/refs` and `spatial_intent/resolution`
  - reference vocabulary, witness catalogs, frame admission, witness
    resolution, and what should remain local versus become runtime-native
    symbolic, binding, fact, and diagnostic surfaces
- immediate runtime-facing spatial operation surfaces
  - where Worth should stop at semantic declaration and let Query own
    admission, workflow, receipts, lineage, and diagnostics directly

This document should grow one category at a time. Each added section should be
specific enough that an engineer can tell:

- what is being deleted
- what is being preserved
- what becomes runtime-native
- and what the acceptance evidence must prove

### `preview` + `constraints` + `continuity`

These three slices should be reviewed together because they are all trying to
answer the same larger question:

- what does a spatial ask imply before commit?
- what should happen if it is accepted?
- what identity, policy, and explanation consequences should survive?

Right now those answers are split across three separate local subsystems. That
is useful as a prototype, but too small and too local for the runtime we have.

#### Current Problem

This combined slice currently has three different shapes:

1. `constraints`
   - authored constraint specs
   - local admitted constraint wrappers
   - frame and point witness admission
   - target witness resolution for points-toward

2. `preview`
   - a local preview struct
   - local commit disposition derivation from arbitration
   - local warning synthesis
   - local profile-driven presentation of what would happen

3. `continuity`
   - a local continuity assessment struct
   - local continuity class mapping from arbitration or chosen resolution
   - local explanation class mapping

All three are conceptually downstream of arbitration and upstream of any real
runtime-facing workflow or execution. But none of them are yet shaped as
runtime-native declarations.

That causes three problems:

- the same semantic decision gets restated in multiple local wrappers
- preview and continuity remain "simulation objects" instead of becoming
  runtime-carryable posture
- constraints stop at local admission even though they are natural candidates
  for Query-owned effect, fact, and diagnostic lifecycles

#### How These Categories Should Be Reimagined

These three slices are not the same kind of thing.

They should be treated as:

1. `constraints`
   - authored spatial operator vocabulary plus local admission

2. `preview`
   - a pre-commit workflow declaration and explanation surface

3. `continuity`
   - an identity and lineage posture declaration surface

That distinction matters because the runtime can help them in different ways.

#### `constraints`: What We Should Actually Do

`constraints/constraints.rs` currently does very little real work:

- `SpatialLiesOnConstraintSpec`
- `SpatialPointsTowardConstraintSpec`
- `SpatialAnchorMatchConstraintSpec`
- admitted wrappers around those specs
- frame admission for lies-on
- point witness resolution for points-toward
- essentially no admission logic for anchor-match

That means the current constraint admission layer is extremely thin. Most of
the real work currently happens downstream in
`lowering/placement_constraints.rs`.

So the concrete plan should be:

##### Step 1: keep authored constraint vocabulary, shrink admitted wrappers

The authored constraint specs are fine in spirit and should stay:

- lies-on
- points-toward
- anchor-match

But the admitted wrappers are too shallow to justify their current shape.

The rewrite should move toward:

- one authored constraint vocabulary surface
- one local admitted constraint family with explicit admitted target/frame
  posture where needed
- no unnecessary wrapper types that simply hold the same spec without adding
  real semantic admission

Concretely:

- `AdmittedSpatialAnchorMatchConstraint` probably should not remain just
  `spec: SpatialAnchorMatchConstraintSpec`
- either add real admitted meaning or keep it authored until the shared
  lowering layer consumes it

##### Step 2: merge constraint admission and constraint lowering around one core

Right now the split is awkward:

- `constraints/constraints.rs` admits a little
- `lowering/placement_constraints.rs` does the real semantic work

After the lowering rewrite, these should be treated as one pipeline:

- authored constraint intent
- local target/frame/witness admission
- shared anchor semantic lowering
- placement transform application
- runtime-facing constraint consequence facts

That means the constraint section of the plan should explicitly depend on the
shared anchor and transform core from the lowering refactor.

##### Step 3: classify which constraints stay local and which become Query-native

Constraints are one of the clearest places where Query should help more.

Local-only pieces:

- semantic meaning of the constraint
- local frame and witness admission
- local ambiguity and numeric posture

Query-facing pieces that should appear soon after:

- declared workflow family
- declared effect family or composition family
- admitted target-binding posture
- admitted basis posture
- post-constraint fact contracts
- denial and explanation posture
- continuity implications if relevant

That means the output of constraint handling should stop being "new placement
spec only" and start becoming "constraint meaning plus runtime-carryable
posture."

##### Step 4: preserve explicit constraint aftermath facts

For these constraints, the same refactor wave should expose Query-facing facts
that carry:
such as:

- which anchor family was constrained
- which target family was consumed
- whether the operation was projection-like, facing-like, or match-like
- whether the target was frame-derived, point-witness-derived, or anchor-derived
- whether continuity was preserved, reinterpreted, or blocked

If downstream runtime-facing layers must infer those from the resulting
placement only, the
constraint pipeline is still too lossy.

#### `preview`: What We Should Actually Do

`preview/simulation.rs` is doing useful work, but it is currently too local and
too presentation-shaped.

Today it:

- runs arbitration
- maps escalation to `SpatialIntentPreviewCommitDisposition`
- invents a warning list
- stores the policy profile and preview richness

That is a good prototype, but it is not yet a runtime-native preview surface.

##### Step 1: stop treating preview as a local simulation object

Preview should be reframed as:

- pre-commit workflow posture
- policy-shaped intent explanation
- Query-ready preview declaration

The current local preview type is okay as a temporary shell, but its fields
should be treated as inputs to immediate Query-facing workflow, inspection, and
diagnostic artifacts, not as the final story.

##### Step 2: collapse duplicated preview entrypoints

The current entrypoints:

- `prepare_spatial_intent_preview`
- `prepare_spatial_intent_preview_with_capabilities`
- `prepare_spatial_intent_preview_with_profile`
- `prepare_spatial_intent_preview_with_capabilities_and_profile`

should collapse internally into one implementation path with one preview
request/input struct.

Public ergonomic wrappers can remain if they help, but internally the rule
should be:

- one request type
- one arbitration result
- one preview posture derivation

##### Step 3: stop inventing warning posture only as local UI fluff

The warnings today:

- clarification required
- preserved candidate set
- blocked future candidate
- profile-driven auto-resolve
- high-fidelity preview

are actually useful declared semantics.

They should immediately feed Query-facing:

- workflow explanation posture
- advisory / decision traces
- preview diagnostics
- support posture
- preview fact slices

So the plan should treat preview warnings as semantic declarations, not just
a local vector for presentation.

##### Step 4: tie preview to runtime-native basis and workflow families

Preview is one of the places where the runtime should do much more.

This slice should align immediately with:

- preview basis posture
- speculative workflow family
- continuation / promotion / discard posture
- inspection and explanation artifacts
- preview facts and receipts

The local preview layer should therefore preserve:

- authored act kind
- arbitration analysis
- policy profile
- commit disposition
- preview richness
- explicit warning posture

as declared, carryable facts rather than one-shot simulation output.

#### `continuity`: What We Should Actually Do

`continuity` is currently a local classification table over arbitration
candidates and resolutions.

That is useful, but too narrow.

Today it:

- maps candidates to continuity classes
- maps candidates to explanation classes
- tracks subject and anchor preservation booleans
- tracks blocked capability posture

The current implementation is compact, but it is also almost entirely a local
translation layer.

##### Step 1: stop treating continuity as a local afterthought

Continuity should become one of the main runtime-facing spatial declarations.

It is not just a convenience label. It is a candidate input to:

- lineage posture
- binding posture
- continuity facts
- downstream mutation / merge strategy
- diagnostics and explanation
- preview and commit posture

That means the continuity mapping should be preserved as a real semantic output,
not just a local helper answer.

##### Step 2: unify analysis-based and resolution-based continuity entry

Right now there are two entrypoints:

- `assess_spatial_identity_continuity_from_analysis`
- `assess_spatial_identity_continuity_from_resolution`

Those should probably remain conceptually distinct, but they should be driven by
one shared continuity-declaration core.

The rule should be:

- one continuity decision model
- multiple sources of evidence into it

instead of two top-level APIs that happen to converge internally.

##### Step 3: prepare continuity for Query-facing lineage and fact surfaces

Continuity is an obvious place where Query should carry more.

The current classifications:

- identity preserved
- anchor continuity preserved
- identity reinterpreted
- identity split
- identity merged
- identity blocked pending choice

should immediately become candidates for:

- continuity facts
- lineage and correspondence posture
- preview explanation
- effect aftermath classification
- merge / commit strategy selection
- diagnostics and support posture

That means the continuity output should be treated as part of the runtime story,
not just a local post-analysis verdict.

##### Step 4: replace bare booleans with stronger semantic posture

The current fields:

- `preserves_subject_identity: bool`
- `preserves_anchor_identity: bool`

are useful, but too weak as the final long-term surface.

The rewrite should move toward a more explicit continuity posture vocabulary,
because booleans collapse too much meaning.

The plan should therefore treat those booleans as transitional and identify
them as rewrite targets.

#### Combined Runtime-Native Direction

Taken together, `preview`, `constraints`, and `continuity` should be pushed
toward one larger runtime-native story:

- constraints declare what spatial relation is being requested
- preview declares what would happen under current basis, capability, and
  policy posture
- continuity declares what identity and anchor consequences would follow

Then Query should increasingly carry:

- the workflow posture
- the basis posture
- the diagnostic posture
- the continuity posture
- the fact aftermath
- the receipts and explanations

The anti-pattern is:

- constraint code computes a new placement
- preview code makes a local summary
- continuity code makes a separate local summary
- runtime-facing code has to restitch all of it

The target is:

- constraint meaning declared once
- preview posture declared once
- continuity posture declared once
- runtime carries those declarations forward

#### What To Delete Or Collapse

The clearest cuts here are:

1. collapse preview entrypoints internally into one request-driven path
2. stop leaving `AdmittedSpatialAnchorMatchConstraint` as a nearly empty wrapper
3. merge constraint admission and lowering more honestly around the shared
   lowering core
4. unify continuity decision logic around one declaration model
5. treat warning vectors and continuity booleans as transitional forms, not
   final architecture

#### Target File Topology

A healthier target shape would look something like:

- `constraints/specs.rs`
  - authored constraint vocabulary
- `constraints/admission.rs`
  - local frame / witness / target admission
- shared lowering / operations core
  - actual application using anchor and transform semantics
- `preview/request.rs`
  - preview request / input posture
- `preview/evaluation.rs`
  - derive preview declaration from arbitration + policy + capabilities
- `continuity/declaration.rs`
  - continuity posture and explanation vocabulary
- `continuity/evaluation.rs`
  - continuity derivation from analysis/resolution inputs

Again, exact names may vary. The structural requirements are:

- preview is workflow declaration, not UI fluff
- continuity is a semantic posture surface, not an afterthought
- constraints are one pipeline, not half in one folder and half in another

#### Acceptance Evidence

This refactor is only honest if it proves:

- preview disposition and warning posture are preserved
- continuity class and explanation posture are preserved
- constraint admission and lowering preserve current success/failure semantics
- preview, continuity, and constraint posture are easier to promote into
  Query-facing workflow, fact, binding, and diagnostic surfaces
- empty or decorative wrappers have been removed
- these slices no longer restate the same decision in multiple local forms

Concrete proof should include:

- parity tests for current preview dispositions and warnings
- parity tests for current continuity classifications
- parity tests for current constraint admission and placement-application
  outcomes
- QA proving preview and continuity are framed as declaration surfaces
  rather than local summary sidecars

### `spatial_intent/refs` + `spatial_intent/resolution`

These folders should be treated as one refactor slice because they currently
form one pipeline:

- authored references and witness vocabulary in `refs`
- local frame and witness admission in `refs`
- witness resolution and policy profile shaping in `resolution`
- downstream consumption by `lowering`

Right now that pipeline is real, but it is not arranged honestly enough.

#### Closure Status

This slice is now structurally closed through the parameter-admission seam.
The following work is complete:

- `SpatialFrameRef` now lives as authored vocabulary separate from frame
  admission and basis math
- point and direction witness admission now share one resolution core
- raw parameter arrays at the witness seam have been replaced by
  `ParameterSpacePoint`
- geometry-domain parameter admission, canonicalization, and polygonal trimmed
  posture now exist below the spatial witness seam
- resolved witnesses now preserve parameter-admission evidence instead of
  discarding it
- fixture catalog machinery has left the normal `refs` module surface and lives
  under explicit test support

The remaining work for this area is no longer `refs + resolution` cleanup. Any
future changes here should be in service of later slices such as `lowering` or
the final `certification + facade` narrowing pass.

#### Current Problem

This slice currently mixes three different jobs:

1. authored symbolic vocabulary
   - anchor refs
   - frame refs
   - point and direction witness refs
   - carrier roles and feature roles

2. local numeric and witness admission
   - frame normalization
   - frame basis projection / embedding
   - finite point checks
   - direction normalization
   - fallback perpendicular derivation

3. pseudo-runtime lookup and fixture infrastructure
   - witness catalog trait
   - empty catalog
   - fixture catalog
   - geometric-tag classification
   - parameter-space and feature-owned lookup conventions

Those jobs are not the same thing.

Today the code lets them blur together:

- `frames.rs` is both authored frame vocabulary and low-level numeric basis math
- `witness_catalog.rs` is both the interface contract and a big test/fixture
  implementation surface
- `resolution.rs` and `point_resolution.rs` duplicate the same resolution shape
  for direction and point witnesses
- local witness failure classes are doing good work, but they are still too
  detached from Query-facing symbolic, binding, and diagnostic surfaces
- `profiles.rs` is sitting inside `resolution` even though it is really policy
  vocabulary, not witness resolution

The result is a slice that is semantically important but structurally too flat.

#### What `refs` Should Actually Be

`refs` should become the authored symbolic reference vocabulary.

That means `refs` should mostly define:

- anchor references
- frame references
- point witness references
- direction witness references
- carrier roles
- symbolic reference families and authored naming posture

It should **not** also be the place where we do most of the numeric work or
where runtime-shaped binding posture quietly accumulates.

Concretely:

- `anchors.rs` is fundamentally correct in spirit
  - it is authored symbolic reference vocabulary
  - and it should align explicitly with Query symbolic references in the same
    refactor wave
- `witnesses.rs` and `point_witnesses.rs` are also mostly correct in spirit
  - they are authored witness vocabulary
  - but they are too split and should likely be unified under one witness
    vocabulary surface
- `frames.rs` is overloaded
  - `SpatialFrameRef` belongs with authored references
  - `SpatialFrameBasis` and projection / embedding math do not

#### What `resolution` Should Actually Be

`resolution` should become the local admission and interpretation layer for
authored witness vocabulary.

That means it should answer:

- what does this witness mean locally?
- can this witness resolve under current local/catalog posture?
- what admitted local point/direction/frame meaning does it become?
- what exact failure class occurred?

It should **not** be:

- a half-runtime binding layer
- a generic fixture bag
- a policy bucket for unrelated postures

#### Concrete Rewrite: What We Should Actually Do

##### Step 1: Split authored reference vocabulary from admitted numeric basis

`frames.rs` should be split.

Keep in `refs`:

- `SpatialFrameRef`

Move out of `refs`:

- `SpatialFrameBasis`
- `AdmittedSpatialFrameRef`
- `admit_spatial_frame(...)`
- local normalize helpers
- projection / embedding math

Those belong either:

- in a dedicated local admission module
- or in a small `frame_admission.rs` / `frame_basis.rs` area backed by
  `worth-math`

Reason:

- `SpatialFrameRef` is authored symbolic intent
- `SpatialFrameBasis` is admitted numeric geometry

Those are different phases and should not live as one concept.

##### Step 2: Unify point and direction witness vocabulary

`point_witnesses.rs` and `witnesses.rs` should be treated as one authored
witness language.

Keep the semantic distinctions:

- point witnesses
- direction witnesses
- carrier kinds
- point roles
- direction roles

But stop scattering them across small files that make the whole witness system
harder to reason about.

Concrete target:

- one `witness_refs.rs` or similar
- one place for authored witness vocabulary
- one place for role enums

This should make it much easier to see:

- which references are truly symbolic
- which are ambiguous by design
- which are asking for carrier-local, feature-owned, or frame-derived meaning

##### Step 3: Merge point and direction resolution into one witness admission core

`point_resolution.rs` and `resolution.rs` are the same architectural shape:

- inspect authored witness variant
- resolve direct world / frame / catalog-backed posture
- normalize or finite-check
- emit requested + resolved + resolution class
- return typed failure class

They should become one coherent witness admission layer with shared internal
helpers and parallel point/direction subfamilies instead of two nearly
independent micro-systems.

The target shape should look more like:

- `admit_point_witness(...)`
- `admit_direction_witness(...)`
- shared resolution classes
- shared failure posture
- shared catalog-routing conventions

The important thing is not the exact names. It is:

- one witness admission system
- not a separate point world and direction world

##### Step 4: Move numeric contracts down into `worth-math`

This slice still contains too much local raw-float admission logic:

- `normalize(...)` in `frames.rs`
- `normalize(...)` in `resolution.rs`
- `finite_point(...)` in `point_resolution.rs`
- fallback perpendicular derivation in `resolution.rs`
- axis extraction / basis projection behaviors tied directly to raw arrays

Those are numeric and geometric contracts, not authored reference vocabulary.

The rewrite should move toward:

- admitted finite point types
- admitted direction types
- admitted frame-normal / axis types
- checked perpendicular derivation
- checked basis projection helpers

`refs` and `resolution` should consume those admitted numeric forms. They
should not continue to mint them ad hoc with repeated local helpers.

##### Step 5: Shrink `witness_catalog.rs` to the real contract

`witness_catalog.rs` currently contains:

- the real trait contract
- resolved witness result types
- geometric-tag resolution classification
- empty catalog
- a large fixture implementation
- a lot of fixture entry structs

That is too much.

The production contract should become much smaller:

- `SpatialWitnessCatalog`
- production-facing resolved catalog witness result types
- geometric-tag resolution contract, if it still belongs here
- `EmptySpatialWitnessCatalog`

The fixture-heavy implementation should move to test support.

That means:

- `SpatialFixtureWitnessCatalog`
- `DirectionParameterEntry`
- `GeometricTagEntry`
- `DirectionFeatureEntry`
- `PointParameterEntry`
- `PointFeatureEntry`

should stop living in production code.

If a reusable fake catalog is still valuable, it should live under test support,
not in the main production module.

##### Step 6: Reclassify what must transition into Query immediately

This slice begins locally, but several of its authored and admitted concepts
are exactly the things runtime-facing layers should promote into Query
immediately.

The key ones are:

- symbolic anchor and witness identities
- feature-owned versus parameter-space versus frame-derived versus world-derived
  posture
- geometric-tag class
- witness resolution class
  - direct world
  - frame derived
  - carrier derived
  - fallback derived
  - exhausted
- witness failure class
  - non-finite
  - ambiguous
  - undefined
  - unsupported
  - degenerate
  - coincident
  - exhausted

These should not remain mere local debugging details.

They are strong candidates for immediate Query-facing:

- symbolic references
- existing-truth binding posture
- support rows
- admission traces
- fact families
- diagnostics and inspection artifacts

##### Step 7: Move policy profiles out of witness resolution

`profiles.rs` does not belong in `resolution`.

It is policy vocabulary:

- threshold posture
- preview richness
- arbitration posture
- named intent profiles

That should move to a policy-focused area, because it is not performing
resolution. It is defining declared semantic policy inputs that immediate Query
admission and support posture should consume.

#### What Must Stay Local

The following still belong in `worth-spatial`:

- authored anchor, frame, and witness vocabulary
- local meaning of feature-owned, parameter-space, and geometric-tag asks
- local witness ambiguity
- local witness failure classification
- local frame and witness admission before installed/runtime binding exists

This slice is still the home of:

- "what symbolic thing is being asked for?"
- "can that symbolic ask resolve locally?"
- "what admitted local spatial witness does it become?"

#### What Should Move Toward Query

Once these references and admitted witnesses start shaping runtime-facing work,
the same refactor wave should promote into Query:

- symbolic references
- target-binding posture
- reference-family support posture
- witness resolution class
- witness denial class
- fact and diagnostic slices based on witness family and resolution outcome

The anti-pattern is:

- resolve a witness locally
- use it once
- discard the symbolic and resolution posture
- rebuild binding, diagnostics, and support interpretation from raw values

The target is:

- author symbolically
- admit locally
- carry forward the symbolic and resolution posture structurally

#### What To Delete Or Collapse

The clearest cuts here are:

1. split `frames.rs`
   - keep authored frame refs separate
   - move admitted basis and numeric work elsewhere

2. merge `point_witnesses.rs` and `witnesses.rs`
   - one witness vocabulary surface

3. merge `point_resolution.rs` and `resolution.rs`
   - one witness admission core

4. move fixture catalog code out of `witness_catalog.rs`
   - production contract only in production
   - big fake catalog in test support

5. move `profiles.rs` out of `resolution`
   - policy vocabulary deserves its own home

#### Target File Topology

A healthier target shape would look something like:

- `refs/anchors.rs`
  - authored anchor refs
- `refs/frames.rs`
  - authored frame refs only
- `refs/witness_refs.rs`
  - point + direction witness vocabulary
  - carrier kinds and roles
- `resolution/frame_admission.rs`
  - admitted frame basis
  - frame numeric admission
- `resolution/witness_admission.rs`
  - point + direction witness admission
  - shared resolution and failure posture
- `resolution/catalog.rs`
  - production catalog contract only
- `policy/profiles.rs` or equivalent
  - spatial policy profile vocabulary
- test support
  - fixture catalogs and large fake lookup scaffolding

Again, exact names can vary. The structural requirements are:

- authored symbolic refs separate from admitted numeric basis
- one witness vocabulary surface
- one witness admission surface
- production catalog contract separate from fixture catalog
- policy profiles not buried under resolution

#### Acceptance Evidence

This refactor is only honest if it proves all of the following:

- authored symbolic reference vocabulary remains explicit and stable
- point and direction witness resolution preserve current success and failure
  classes
- direct world, frame-derived, carrier-derived, fallback-derived, and exhausted
  resolution posture remain typed
- local ambiguity and unsupported posture remain exact
- frame admission no longer leaks raw numeric helpers through authored ref code
- fixture catalog scaffolding leaves production code
- runtime-facing layers can carry symbolic and resolution posture without
  rediscovering it from raw points and directions
- policy vocabulary is easier to route into Query admission immediately

Concrete proof should include:

- parity tests for current point and direction witness resolution behavior
- parity tests for geometric-tag point/direction/unsupported distinctions
- QA proving authored refs and admitted numeric basis are structurally split
- QA proving fixture catalog code no longer inflates production modules
- explicit notes on which symbolic and resolution facts Query-facing layers must
  promote immediately

### `arbitration`

After reviewing the code, arbitration should have come before preview and
continuity because it is the actual upstream semantic decision engine.

`preview` and `continuity` are mostly downstream interpretations of arbitration
results. If arbitration stays too local or too weak, the downstream layers will
keep inventing their own parallel summaries.

#### Current Problem

`arbitration` is doing the core semantic work, but it is still shaped like a
local chooser instead of a runtime-ready declaration layer.

Today it has:

- authored act vocabulary in `conflicts.rs`
- observed relation facts in `conflicts.rs`
- candidate vocabulary in `candidates.rs`
- blocked capability vocabulary in `blocked.rs`
- candidate ranking and explanation class in `ranking.rs`
- escalation analysis in `escalation.rs`
- chosen resolution and resolution error in `resolution.rs`

That is already the brain of this subsystem.

The problem is not that it lacks concepts. The problem is that those concepts
are still spread across small local tables and boolean capability gates rather
than being treated as one declared decision model that runtime-facing layers
can carry immediately.

The main issues are:

- capability support is modeled as local booleans in `SpatialIntentCapabilitySet`
- candidate ranking is mostly a local priority sort with handwritten
  push/insert logic
- policy posture influences escalation, but the result is still mostly a local
  summary object
- preview and continuity then restate parts of the same decision in their own
  local shapes
- the whole slice is very close to a Query admission and declaration model, but
  it stops short of thinking that way

#### What Arbitration Actually Is

Arbitration is not just a helper that picks the best candidate.

It is the declaration point for:

- authored act family
- observed relation facts
- candidate set
- support / blocked posture
- conflict class
- escalation posture
- chosen resolution authority

That means arbitration should be treated as the semantic source for:

- preview posture
- continuity posture
- workflow posture
- support and denial posture
- diagnostics and explanation posture

If those downstream layers have to rediscover the decision, arbitration is too
weakly shaped.

#### Concrete Rewrite: What We Should Actually Do

##### Step 1: collapse local arbitration tables into one explicit declaration model

Right now arbitration is spread across:

- `blocked.rs`
- `candidates.rs`
- `conflicts.rs`
- `ranking.rs`
- `escalation.rs`
- `resolution.rs`

That is too fragmented for the small amount of real semantic state involved.

The target should be:

- one authored conflict vocabulary area
- one candidate/support vocabulary area
- one arbitration declaration / decision area
- one chosen-resolution area

More concretely:

- `ranking.rs` should probably disappear as a separate concept
- `SpatialIntentExplanationClass` and `SpatialIntentCandidateRank` are too thin
  to deserve a whole file
- the candidate/rank/support relationship should live next to the arbitration
  decision model itself

##### Step 2: replace boolean capability posture with declared support posture

`SpatialIntentCapabilitySet` is currently just a bool bag:

- `merge_boolean: bool`
- `subtract_boolean: bool`
- `cut_opening: bool`
- `join: bool`
- `host_attach: bool`

That is fine as a prototype, but too weak for the runtime-native direction.

This should move toward:

- declared support posture
- capability family identity
- explicit support / blocked / deferred semantics
- immediate Query-facing support rows and admission posture

The immediate rewrite target is:

- stop treating capability as a local feature-toggle bag
- start treating it as an admitted support declaration candidate

Even if the first rewrite still uses a simple local struct, the plan should
treat those booleans as transitional.

##### Step 3: make arbitration output the canonical semantic declaration

`SpatialIntentArbitrationAnalysis` is already close to the right center.

It carries:

- authored act
- observed relation facts
- candidates
- conflict class
- escalation
- chosen candidate

That should become the canonical source that preview and continuity consume.

The rewrite rule should be:

- preview derives from arbitration declaration
- continuity derives from arbitration declaration or chosen resolution
- no downstream layer re-ranks, re-classifies, or re-explains the candidate set

##### Step 4: preserve candidate and explanation posture for immediate Query promotion

The following arbitration facts should be treated as carryable, not local-only:

- authored act kind
- observed relation fact set
- candidate identity
- candidate availability posture
- blocked capability identity
- explanation class
- conflict class
- escalation posture
- chosen authority posture

These are excellent candidates for immediate Query-facing:

- intent declarations
- admission traces
- support rows
- preview explanations
- continuity facts
- workflow posture
- runtime-facing diagnostics

If downstream layers only see `chosen_candidate` and lose the rest, the runtime
opportunity is being wasted.

##### Step 5: tighten the resolution boundary

`resolve_spatial_intent_conflict_by_policy` and
`resolve_spatial_intent_conflict_by_choice` are conceptually fine, but the
resolution layer should be understood as:

- one resolution entry from escalation
- one explicit authority posture
  - policy auto-resolve
  - explicit choice
- one typed failure posture

The downstream rule should be:

- resolution consumes the declared arbitration story
- resolution does not recompute arbitration
- preview and continuity then consume arbitration and resolution, not the other
  way around

##### Step 6: make preview and continuity thinner after arbitration cleanup

This slice changes the downstream categories immediately.

After arbitration is cleaned up:

- preview should mostly map arbitration posture into preview-specific workflow
  and warning posture
- continuity should mostly map arbitration and resolution posture into lineage
  and identity posture

That means preview and continuity should likely shrink after this refactor.

#### What Must Stay Local

The following still belong in `worth-spatial`:

- authored act family semantics
- observed spatial relation fact semantics
- candidate family semantics
- local policy-family interpretation over those candidates

This is still domain semantics, not something Query should invent for us.

#### What Should Move Toward Query

Arbitration is one of the clearest places where Query should carry more
immediately.

The immediate Query-facing surfaces are:

- intent declaration
- support posture
- admission trace
- advisory / denied / auto-resolved posture
- preview workflow posture
- continuity and lineage posture
- resolution authority posture
- diagnostics and explanation surfaces

So the target is not:

- local arbitration forever

It is:

- local spatial semantics
- declared arbitration outcome
- runtime-carried decision story

#### What To Delete Or Collapse

The clearest cuts here are:

1. collapse `ranking.rs` into the real arbitration declaration layer
2. treat `SpatialIntentCapabilitySet` booleans as transitional and prepare for a
   stronger support posture
3. stop letting preview and continuity restate arbitration decisions in parallel
4. reduce file fragmentation where it is separating tiny enums from the actual
   decision model without buying clarity

#### Target File Topology

A healthier target shape would look something like:

- `arbitration/vocabulary.rs`
  - authored act, relation facts, candidate families, blocked capability
- `arbitration/decision.rs`
  - arbitration declaration, support posture, conflict class, escalation
- `arbitration/resolution.rs`
  - chosen authority, chosen resolution, typed resolution failure

Again, exact names can vary. The structural requirements are:

- one clear arbitration declaration center
- support posture not hidden as a bool bag forever
- preview and continuity downstream from arbitration, not sibling brains

#### Acceptance Evidence

This refactor is only honest if it proves:

- current candidate selection behavior is preserved
- current blocked capability behavior is preserved
- current conflict-class and escalation behavior is preserved
- preview and continuity can become thinner because arbitration carries more of
  the shared decision story
- arbitration outputs are easier to promote into Query-facing admission and
  support artifacts immediately

Concrete proof should include:

- parity tests for current arbitration candidate sets, escalations, and
  resolution outcomes
- QA proving preview and continuity no longer restate arbitration semantics
  unnecessarily
- explicit notes on which arbitration fields Query-facing layers must promote
  into declarations, traces, and support posture immediately

### `bindings`

The `bindings` folder is really one concentrated primitive-birth subsystem.

That makes it easier to reason about, but it also makes the problems more
obvious: this slice is carrying real domain semantics alongside a lot of local
authority, digest, completeness, mapping, and rejection ceremony that should
either become stronger domain types or flow directly into Query/runtime artifacts
instead of multiplying local report families.

#### Current Problem

This slice currently contains:

- a local construction-birth authority in `authority.rs`
- primitive birth authored input and birth plan in `primitive_birth.rs`
- family contract checks in `primitive_birth_contract.rs`
- input validation in `primitive_birth_validation.rs`
- completeness certification in `primitive_birth_completeness.rs`
- mapping report construction in `primitive_birth_mapping.rs`
- rejection rows in `primitive_birth_rejection.rs`

The real domain core is:

- primitive birth family
- scaffold input
- admitted birth plan
- family contract semantics

But around that core, the folder also contains:

- `DefaultHasher` digests
- stringly topology birth class identity
- wide count clusters
- local authority branding
- multiple local report layers that restate the same truth at slightly
  different abstraction levels

So the shape is:

- strong kernel instinct
- weak identity substrate
- weak numeric substrate
- too many local report objects

#### What This Slice Actually Is

This folder is not just “bindings.”

It is really:

- primitive birth authored specification
- primitive birth family contract
- primitive birth local admission
- primitive birth completeness and mapping consequences
- primitive birth rejection posture

That means it should be treated as one primitive-birth pipeline, not as a bag
of related report files.

#### Concrete Rewrite: What We Should Actually Do

##### Step 1: replace raw digest and authority identity immediately

This slice is still using `DefaultHasher` in:

- `authority.rs`
- `primitive_birth.rs`

That should be treated as immediate rewrite debt, not a nice-to-have.

The rewrite should move to the same stronger digest posture we have already
been pushing elsewhere:

- stable digest protocol
- typed digest wrappers
- no raw `String` digest identity as the primary substrate

The same applies to:

- `topology_birth_class: &'static str`
- `authority_digest: String`
- `scaffold_digest: String`
- `birth_digest: String`
- `completeness_digest: String`
- `report_digest: String`
- `row_digest: String`

Those should move toward:

- typed identity wrappers
- typed birth-class vocabulary
- digest values that are projections of stronger truth, not the main semantic
  carrier

##### Step 2: replace count clusters with one value object

`PrimitiveConstructionBirthScaffoldInput` is carrying:

- `expected_vertex_count`
- `expected_edge_count`
- `expected_loop_count`
- `expected_wire_count`
- `expected_face_count`
- `expected_shell_count`
- `expected_body_count`

And those are mirrored again in:

- `SpatialConstructionBirthPlan`
- `PrimitiveConstructionBirthContractCounts`
- completeness and mapping reports

That should collapse into one named topology-count object.

The rewrite target should be something conceptually like:

- one `TopologyCounts` value object
- scaffold input carries it once
- plan carries admitted supported counts once
- contract, completeness, mapping, and rejection logic consume it

That will eliminate a lot of repeated getter surfaces and constructor noise.

##### Step 3: separate authored scaffold input from admitted birth contract

`PrimitiveConstructionBirthScaffoldInput` currently does too much:

- authored family and topology class
- scaffold digest
- support planes
- vertex positions
- expected counts
- realization report

It is halfway between:

- authored input
- admitted numeric/topology witness set
- runtime-facing birth evidence

Those phases should be sharper.

The rewrite should move toward:

- authored primitive birth spec
- admitted scaffold witness set
- admitted birth contract
- admitted birth plan

This does not mean adding more wrapper clutter. It means making the existing
big struct stop collapsing several phases together.

##### Step 4: move numeric and geometric witness truth down

This slice still exposes raw:

- `vertex_positions: Vec<[f64; 3]>`
- support planes directly

Those are real geometric witnesses and should not remain casual raw-array
payloads forever.

The rewrite should move toward:

- admitted point collections
- admitted support-plane collections
- numeric admission through `worth-math` and geometry admission through
  lower layers before birth planning treats them as trustworthy

The immediate plan does not need to solve every type here, but it should mark
these raw arrays as rewrite targets, not stable architecture.

##### Step 5: collapse local report proliferation into one consequence story

Right now there are several local consequence/report families:

- `SpatialConstructionBirthCompletenessReport`
- `SpatialConstructionBirthMappingReport`
- `SpatialConstructionBirthMappingRow`
- `SpatialConstructionBirthRejectionRow`

They are each reasonable, but together they suggest the subsystem is generating
multiple local report-shaped artifacts because it does not yet trust a more
coherent lifecycle.

The rewrite should ask:

- what is the canonical admitted primitive birth artifact?
- what are its direct consequence families?
- which of those are real domain consequences and which are local report
  scaffolding?

My expectation is:

- completeness and mapping should probably be consequence views over one
  stronger primitive-birth artifact
- rejection posture should be a typed failure/consequence family, not merely
  another local digest row

##### Step 6: make family contracts declarative instead of match forests

`primitive_birth_contract.rs` is the start of the right idea, but it is still a
big handwritten `match` with count formulas.

That should move toward a more declarative family contract model.

The plan does not need a full trait lattice immediately, but it should be
headed toward:

- one named family contract surface
- counts and support-plane requirements expressed through named contract rules
- diagnostics that can expose which contract clause failed

That will matter a lot once more primitive families arrive.

##### Step 7: identify where Query should carry this immediately

This slice still begins locally, but it clearly wants immediate runtime help
at the first honest seam.

The immediate Query-facing layers should be able to carry:

- primitive birth family identity
- admitted scaffold identity
- admitted family contract posture
- completeness posture
- mapping aftermath facts
- rejection posture
- support and diagnostics around impossible or blocked birth pathways

So the target should not be:

- forever-local primitive birth reporting

It should be:

- local birth semantics and geometric/topology admission
- then runtime-carried declaration, consequence, and diagnostics posture

#### What Must Stay Local

The following still belong in `worth-spatial`:

- primitive family semantics
- topology birth class semantics
- local family contract meaning
- local geometric/topology admission before installed/runtime truth exists
- local primitive birth planning

That is real domain truth and should not be invented by Query.

#### What Should Move Toward Query

Once primitive birth becomes part of a runtime-facing workflow, the same
refactor wave should promote into Query:

- primitive birth declaration
- support posture
- rejection posture
- completeness and mapping consequence facts
- diagnostics and explanation posture
- identity/lineage implications for created topology

The anti-pattern is:

- do primitive birth locally
- generate several local report types
- rebuild workflow, support, and diagnostics posture outside them

The target is:

- admit birth once
- declare its consequence posture once
- let Query-facing runtime layers carry that truth structurally immediately

#### What To Delete Or Collapse

The clearest cuts here are:

1. remove `DefaultHasher` identity from `authority.rs` and `primitive_birth.rs`
2. collapse count clusters into one topology-count object
3. split the giant scaffold input into clearer phases instead of one overloaded
   struct
4. reduce the number of local report families by centering the primitive-birth
   artifact first
5. prepare `primitive_birth_contract.rs` to become a more declarative contract
   surface instead of a growing match forest

#### Target File Topology

A healthier target shape would look something like:

- `bindings/primitive_birth/spec.rs`
  - authored primitive birth vocabulary
- `bindings/primitive_birth/counts.rs`
  - topology-count value object
- `bindings/primitive_birth/admission.rs`
  - local numeric and topology admission
- `bindings/primitive_birth/contract.rs`
  - declarative family contract rules
- `bindings/primitive_birth/plan.rs`
  - admitted birth plan
- `bindings/primitive_birth/consequences.rs`
  - completeness, mapping, and rejection consequence families

Again, exact names can vary. The structural requirements are:

- no weak digest substrate
- no giant count cluster duplication
- no overloaded scaffold input doing every phase at once
- no report proliferation without a centered primitive-birth artifact

#### Acceptance Evidence

This refactor is only honest if it proves:

- current primitive family planning behavior is preserved
- current family contract behavior is preserved
- current completeness and mapping consequences are preserved
- current rejection posture remains typed
- digest and authority identity are stronger and more stable
- count handling is centralized instead of repeated
- runtime-facing layers can promote primitive-birth posture without
  rediscovering it from scattered report types

Concrete proof should include:

- parity tests for current primitive-birth planning and contract outcomes
- parity tests for completeness, mapping, and rejection consequences
- QA proving count duplication has been collapsed
- QA proving `DefaultHasher` and stringly digest identity no longer define the
  semantic core

### `certification` + `facade`

This slice is a little unusual.

`worth-spatial/src/certification` is not a rich domain-certification subsystem
the way some other crates have. It is mostly a public-boundary test harness for
`facade.rs`, plus one compile-fail check around the construction-birth
authority constructor.

That means the real refactor target is not "move certification logic onto the
runtime" in the abstract.

The real target is:

- shrink and clarify the public surface
- make the public surface transition to Query-facing seams immediately where it
  should
- keep only the local semantic admission surface in `worth-spatial`
- keep certification as a hostile public-boundary harness that proves the new
  seams are honest

#### Current Problem

Today the facade exports almost everything directly:

- primitive birth planning surfaces
- reference vocabulary
- witness vocabulary
- witness catalogs
- frame basis and frame admission
- point and direction witness resolution
- motion and placement admission
- constraint admission and constraint application
- arbitration
- preview
- continuity

And the certification harness mostly proves that all of this is public and
works end-to-end.

That is valuable as a prototype, but it means the current public surface is:

- too wide
- too local
- too tied to implementation-shaped types
- not yet expressing the Query transition cleanly

The test tree confirms this:

- `public_api.rs` is a giant all-in-one boundary test
- `public_api_anchor_lowering.rs`
- `public_api_arbitration.rs`
- `public_api_carrier_witnesses.rs`
- `public_api_preview.rs`
- `public_api_continuity.rs`

Those tests are doing honest work, but they are also revealing that the crate
still thinks the final public story is "export the whole local semantic machine
directly."

That is not the runtime-native target.

#### What Certification Should Actually Do Here

For `worth-spatial`, certification should do two jobs:

1. prove the local semantic boundary is honest
2. prove the Query-facing seam is honest

It should not mainly prove that every local intermediate type is public forever.

That means this slice should be refactored together with `facade.rs`.

#### Concrete Rewrite: What We Should Actually Do

##### Step 1: split the facade into local-semantic and runtime-facing strata

Right now `facade.rs` is one flat export layer.

That should stop.

The public story should become more stratified, for example:

- local semantic admission surfaces
  - authored refs
  - authored specs
  - local admitted motion / placement / witness semantics
- runtime-transition surfaces
  - arbitration declaration
  - preview declaration
  - continuity declaration
  - primitive-birth declaration and consequence posture
- Query-facing integration seams
  - the exact shapes intended to cross into Query immediately

The exact filenames can vary, but the main rule is:

- stop exporting the whole local semantic machine as one flat permanent facade

##### Step 2: stop treating every local type as permanent public contract

Several currently exported types should be treated as likely transitional:

- wide local admitted wrappers that only exist to bridge internal steps
- local preview and continuity summary types in their current shape
- fixture-heavy catalog surfaces
- internal consequence/report layers from primitive birth

That does not mean “hide everything.”

It means:

- decide which types are true long-term semantic contracts
- decide which types are only local refactor stepping stones
- narrow the public contract accordingly

The certification harness should then enforce that narrower contract.

##### Step 3: make certification prove the Query transition, not just local exports

The current tests mostly prove:

- this is exported
- this type behaves correctly
- this end-to-end local flow works

The refactor should shift certification toward:

- this local semantic surface is honest
- this Query-facing declaration seam is explicit
- this runtime transition preserves semantic facts
- this public surface does not expose unnecessary local machinery

For example, certification should increasingly test:

- arbitration as the canonical decision declaration
- preview and continuity as downstream declared postures
- primitive birth as a centered artifact with consequence views
- witness and binding posture preserved at the public seam

instead of simply proving that many local functions are callable.

##### Step 4: reduce giant public API test aggregation

`public_api.rs` currently carries too much at once.

It should likely shrink and become more role-based:

- birth and topology surface
- reference and witness surface
- spatial admission surface
- arbitration / preview / continuity surface
- runtime-transition seam surface

This will make it much easier to see what the crate is really promising.

##### Step 5: add certification for what should no longer be public

There is already one compile-fail check for the construction-birth authority
constructor.

That pattern should grow wherever we intentionally tighten the public surface.

As the facade narrows, compile-fail checks should prove:

- local implementation-only constructors are not public
- transitional local helper types are not accidentally exported as durable
  contracts
- runtime-transition seams are used instead of bypassing back into local
  internals

##### Step 6: make Query the immediate public runtime-facing door

This is the most important structural change.

For runtime-facing behavior, `worth-spatial` should stop behaving like the final
public operating surface.

Instead:

- `worth-spatial` should expose the irreducible local semantic core
- Query should become the immediate public runtime-facing door for:
  - admission posture
  - support posture
  - workflow posture
  - preview posture
  - continuity / lineage posture
  - diagnostics and explanation posture
  - consequence and fact posture

That means the facade refactor is not just cleanup. It is part of the runtime
transition.

#### What Must Stay Local

The following still belong in `worth-spatial` public semantics:

- authored spatial vocabulary
- irreducible local admission surfaces
- primitive family semantics
- witness and frame meaning
- spatial arbitration semantics

But those should be exposed as the local semantic core, not as the whole
runtime story.

#### What Should Transition To Query Immediately

The runtime-facing public story should move immediately toward Query for:

- arbitration outcome posture
- support and blocked capability posture
- preview posture
- continuity and lineage posture
- primitive-birth consequence posture
- diagnostics and explanation posture
- binding and fact aftermath posture

The anti-pattern is:

- export everything from `worth-spatial::facade`
- make certification prove all of it publicly forever
- only later figure out how Query becomes the real operating surface

The target is:

- export the true local semantic core
- expose an explicit immediate Query-facing seam
- let certification prove that seam and guard against widening the surface

#### What To Delete Or Collapse

The clearest cuts here are:

1. shrink `facade.rs` so it is not one flat export dump
2. reduce `public_api.rs` as a giant all-surface contract test
3. add more compile-fail checks as the public surface narrows
4. stop treating fixture-heavy or transitional local types as permanent facade
   promises

#### Target File Topology

A healthier target shape would look something like:

- `facade/local_semantics.rs`
  - authored refs, specs, admitted local semantics
- `facade/runtime_transition.rs`
  - the explicit types and functions that should cross into Query immediately
- `certification/public_facade_contracts/contracts/...`
  - split by public promise family
- `certification/public_facade_contracts/compile_fail/...`
  - prove intentionally hidden local internals stay hidden

Again, exact names can vary. The structural requirements are:

- certification proves the real public promises
- facade stops being a flat export of everything
- Query is treated as the runtime-facing door immediately

#### Acceptance Evidence

This refactor is only honest if it proves:

- the local semantic core remains public where it truly should
- the public surface is narrower and more intentional
- runtime-facing posture transitions to Query at explicit seams
- certification tests are aligned to those seams rather than to every local
  helper or intermediate type
- compile-fail coverage grows where the surface is intentionally tightened

Concrete proof should include:

- updated public API contract tests grouped by real surface family
- compile-fail tests for newly hidden local implementation details
- QA proving `facade.rs` no longer behaves like a flat export dump
- explicit notes on which public surfaces are local semantic core and which are
  Query-transition seams

