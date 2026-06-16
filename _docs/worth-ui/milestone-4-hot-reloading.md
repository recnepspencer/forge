# Milestone 4S: Hot Reloading

## Goal

Build the runtime-owned hot reload spine that all Worth UI projections consume,
so source, capability, Query, action, state, theme, density, appearance, shell,
page, and component changes flow through declared changed facts, declared
projection dependencies, typed activation evidence, runtime-owned rebind
planning, and counter-backed certification.

This is a side-quest milestone for Milestone 4. It exists because the Shopify
dashboard proof cannot be trustworthy if each visible surface earns hot reload
through bespoke app-local glue.

## Why This Milestone Exists

Milestones 1 through 3 already built the lower substrate for canonical
artifacts, hot replacement, durable state reconciliation, Query binding
comparison, active execution plans, and frame-cost certification. Milestone 4
started proving that substrate through a native validation app, but the first
visible reload slices exposed a deeper problem: header, page-host, and
capability reload paths can work individually while still lacking one platform
contract for arbitrary projection rebind.

Hot reloading must become a runtime capability, not a collection of surface
patches. The product we are building is a UI platform. A page, shell, component,
menu, theme, action, or Query-bound view should hot reload because it is a
runtime-owned projection with declared dependencies over runtime facts, not
because the validation app remembers one more local case.

## Governing Summaries

`MENTALITY.md`

- The document protects foundation-first honesty.
- This side quest must solve the runtime reload foundation now rather than
  continuing dashboard breadth on ad hoc reload slices.

`arch_laws.md`

- The document protects proof-carrying runtime structure.
- Every mutation or reload must declare invalidated facts, every projection must
  declare consumed facts, and the runtime must compute their intersection.

`composition_laws.md`

- The document protects named semantic decomposition.
- Source admission, capability admission, change evidence, projection planning,
  rebind coordination, rendering, diagnostics, and certification must remain
  separate responsibilities.

`domain_structure_laws.md`

- The document protects authority boundaries in the tree and import graph.
- Source truth, runtime authority, derived authoring snapshots, projection
  receipts, diagnostics, and native rendering must be structurally separated.

`perf_laws.md`

- The document protects bounded execution tied to semantic delta.
- Reload breadth must be explained by changed facts and projection dependency
  intersections, not by broad scans over surfaces, artifacts, or registries.

`AI_README.md`

- The document protects Query as the ordinary runtime-backed public layer.
- Worth UI must consume Query-owned declaration, binding, projection
  consumption, async or result-state, recovery, inspection, and support posture
  through canonical runtime-owned artifacts rather than rebuilding local Query
  or local status models.

`worth_ui_roadmap.md`

- The roadmap protects Worth UI as a desktop application platform rather than a
  widget bundle over `egui`.
- This side quest belongs immediately inside the Milestone 4 work because the
  dashboard proof depends on platform-wide hot reload, not surface-local reload
  exceptions.

## Adversarial Constraint

A running native Worth UI app must survive arbitrary source, capability, Query,
and state edits without app-local shell logic, app-local dependency graphs,
local state hydration, broad runtime scans, or renderer-owned truth.

Changing a page slot, header label, theme token, dropdown selection mode,
component appearance, live-view binding, command posture, action projection, or
durable state family must produce runtime-owned change evidence, rebind only
projections whose declared dependencies intersect the changed facts, preserve
prior truth on denial, and expose receipts plus counters proving the path.

## Product Decision Lock

- `WorthUiRuntimeHost` remains the owner of active artifact, active execution
  plan, capability snapshot, authoring snapshot, activation state, diagnostics,
  and projection rebind boundaries.
- `WorthUiValidationReloadEvidence` and `WorthUiCapabilityReloadEvidence` remain
  valid family evidence, but they must be able to project into one common
  runtime change envelope.
- `WorthUiRuntimeFactId`, `WorthUiRuntimeFactSet`, and
  `WorthUiProjectionDependencySet` are the foundation for invalidation and
  projection consumption contracts.
- `WorthUiRuntimeAuthoringSnapshot` remains derived from the source and artifact
  pipeline. It must not become a second app model.
- `CapabilitySnapshot`, `CapabilitySnapshotIndex`, command descriptors,
  projection descriptors, theme tokens, and future capability families remain
  capability authority.
- `ViewBindingDescriptor`, `WorthUiBoundBindingSemantics`,
  `WorthUiRuntimeDependencyHook`, `WorthUiQuerySupportReceipt`,
  `WorthUiQueryBindingComparison`, and `WorthUiQueryLiveRebindPlan` remain the
  Query-facing runtime substrate.
- Native validation app code may render receipts with `egui`, but it may not
  own shell, page, command, theme, Query, or reload truth.

## Phase Plan

### Phase 1: Runtime Change Evidence Envelope

This phase creates the shared evidence envelope consumed by projection rebind.
It freezes the distinction between family-specific reload details and the common
runtime fact contract every projection can understand.

**Relevant subsystems**
- runtime source reload
- runtime capability reload
- runtime fact sets
- activation lifecycle and diagnostics

**Relevant APIs**
- `WorthUiValidationReloadEvidence`
- `WorthUiCapabilityReloadEvidence`
- `WorthUiValidationReloadStatus`
- `WorthUiCapabilityReloadStatus`
- `WorthUiRuntimeFactSet`
- `WorthUiRuntimeHost`

**Build shape**
- Add a common runtime change evidence type carrying runtime instance identity,
  activation status, changed facts, family payload summary, denial posture, and
  counters.
- Let source reload evidence and capability reload evidence project into the
  common envelope without erasing their family-specific status or stages.
- Preserve `ReadyForFrameBoundary`, `Activated`, `EquivalentNoOp`, and `Denied`
  as distinct postures because projection rebind must treat them differently.
- Require activated change evidence to carry a valid changed-fact basis, even
  when that basis is intentionally empty for equivalent changes.
- Keep envelope construction runtime-owned so app code cannot mint successful
  reload evidence.

**Warnings**
- Do not flatten source and capability reload into one weak enum that loses
  typed denial stages.
- Do not let projections consume raw family evidence directly once the common
  envelope exists.
- Do not let app code build activated evidence from strings or copied counters.

**Test requirements**
- Equivalent source reload and equivalent capability reload project to common
  evidence that preserves prior projections without rebuild.
- A prepared reload from one runtime cannot produce rebind-eligible evidence for
  a different runtime.
- Denied reload evidence cannot trigger projection rebuild.
- Activated reload evidence without a runtime-owned changed-fact basis fails at
  the construction boundary.

**Engineering decisions**
- The shared envelope is a projection-consumption contract, not a replacement
  for family evidence.
- Runtime instance identity must remain part of the envelope so stale or foreign
  evidence cannot drive local UI changes.

**Open questions**
- None.

### Phase 2: Runtime Fact Taxonomy Expansion

This phase makes runtime facts rich enough to describe arbitrary Worth UI hot
reload without falling back to broad artifact or execution-plan invalidation for
every meaningful edit.

**Relevant subsystems**
- runtime fact identities
- authoring snapshot
- capability snapshot
- Query binding comparison
- durable state families

**Relevant APIs**
- `WorthUiRuntimeFactId`
- `WorthUiRuntimeFactFamily`
- `WorthUiRuntimeFactSet`
- `WorthUiProjectionDependencySet`
- `WorthUiRuntimeAuthoringSnapshot`
- `WorthUiQueryBindingComparison`

**Build shape**
- Extend runtime fact identity to cover layout topology, content mounts, shell
  surfaces, page templates, page instances, appearance recipes, density tokens,
  component declarations, action posture, live-view bindings, virtualized data
  frame targets, durable state families, overlays, toasts, and inspector
  surfaces.
- Keep public constructors typed and semantic; avoid caller-authored dotted
  string conventions where a domain type already exists.
- Preserve canonical ordering and digest behavior for fact sets so replay and
  equivalence tests are stable.
- Distinguish broad facts such as active artifact or execution plan from narrow
  facts such as one page slot or one theme token.
- Add conversion from existing impact, capability delta, and Query binding
  comparison outcomes into the expanded fact taxonomy where the proof already
  exists.

**Warnings**
- Do not let `active_artifact` become the default invalidation answer for every
  edit once narrower facts can be derived.
- Do not introduce fact names that mix authority categories such as source
  origin, visual hierarchy, and runtime state in one string.
- Do not compute facts by reparsing source inside projection planning.

**Test requirements**
- A content-slot edit invalidates the page-host/content facts it touches but
  does not invalidate unrelated header theme facts.
- A theme-token edit invalidates theme consumers but not layout topology or
  content-mount consumers.
- Fact-set canonical ordering and digest behavior are stable across equivalent
  source declaration order.
- Malformed or unknown fact identity cannot be smuggled through the public
  facade.

**Engineering decisions**
- Runtime facts are semantic contracts, not debug labels.
- Broad facts remain valid for truly broad changes, but the ordinary path must
  prefer the narrowest honest fact set.

**Open questions**
- Whether page-template and page-instance facts should share one family with
  distinct identity tags or become separate fact families.

### Phase 3: Projection Plan Contract

This phase defines the common contract every reloadable projection must carry:
what it is, what runtime facts it consumes, what equivalence basis justifies
reuse, and what receipt proves a frame was produced.

**Relevant subsystems**
- header surface plans
- page-host plans
- projection dependency sets
- frame receipts
- facade exports

**Relevant APIs**
- `WorthUiProjectionDependencySet`
- `WorthUiHeaderFramePlan`
- `WorthUiHeaderMenuPlan`
- `WorthUiHeaderThemePlan`
- `WorthUiPageHostPlan`
- `WorthUiHeaderFrameReceipt`
- `WorthUiPageHostFrameReceipt`

**Build shape**
- Introduce a shared projection plan contract that carries projection identity,
  projection family, dependency set, frame digest or equivalence basis, and
  rebuild eligibility.
- Migrate header menu, header theme, header frame, and page-host plans onto the
  shared contract while preserving their typed family-specific receipts.
- Make dependency declaration required at plan construction time.
- Keep projection receipts derived from plans and runtime authority; app code
  may inspect receipts but must not construct them.
- Add enough family tagging that diagnostics can explain which projection
  family preserved, rebuilt, or denied under a change.

**Warnings**
- Do not replace typed header or page-host receipts with an untyped projection
  bag.
- Do not allow projection dependencies to be appended later by the renderer.
- Do not hide equivalence behind raw digest comparison without naming the basis.

**Test requirements**
- A projection plan without declared dependencies cannot be registered or
  rebound.
- A projection whose dependencies do not intersect changed facts preserves its
  frame digest and reports reuse.
- A projection whose dependencies intersect changed facts must rebuild or return
  a typed denial.
- Equivalent rebuilds report equivalent-after-activation rather than false
  changed output.

**Engineering decisions**
- Projection plans are derived state; they must be rebuildable from runtime
  authority and declared requests.
- The common contract standardizes lifecycle and evidence while family planners
  still own family-specific frame construction.

**Open questions**
- Whether the common projection plan contract should be trait-based internally
  or represented as a sealed enum over admitted projection families.

### Phase 4: Projection Rebind Coordinator

This phase moves rebind orchestration into one runtime-owned coordinator instead
of leaving every surface family to implement its own reload status logic.

**Relevant subsystems**
- runtime host
- header surface rebind
- page-host rebind
- runtime change evidence
- projection dependency intersection

**Relevant APIs**
- `WorthUiRuntimeHost`
- `WorthUiHeaderFrameRebindReceipt`
- `WorthUiPageHostRebindReceipt`
- `WorthUiProjectionDependencySet`
- runtime change evidence from Phase 1

**Build shape**
- Add a runtime-owned projection rebind coordinator that consumes current
  projection plans plus runtime change evidence.
- Let the coordinator classify each projection as preserved by equivalent
  reload, preserved by denial, denied because activation has not happened,
  equivalent after activation, or rebound after activation.
- Migrate `rebind_header_frame_after_reload` and
  `rebind_page_host_after_reload` onto the coordinator or shrink them into thin
  family adapters over it.
- Count inspected projections, dependency intersections, rebuild attempts,
  preserved frames, denied frames, and rebuilt frames.
- Keep family-specific rebuild functions narrow: they should rebuild family
  plans from runtime authority, not re-decide reload status.

**Warnings**
- Do not keep duplicating `ReadyForFrameBoundary` / `Denied` / `Activated`
  logic inside each projection family.
- Do not let the coordinator call render code or hold `egui` state.
- Do not rebuild all registered projections when changed facts intersect only
  one projection family.

**Test requirements**
- Header and page-host rebind through the same coordinator for one activated
  runtime change envelope.
- Ready-but-unactivated evidence is denied before any family projection planner
  runs.
- Denied and equivalent evidence preserve projection frames without rebuild.
- Multiple projections rebind with rebuild breadth equal to changed dependency
  intersections, not total registered projection count.

**Engineering decisions**
- The coordinator owns lifecycle classification; family planners own how to
  produce a new family plan when classification says rebuild.
- Rebind counters are part of the public proof because performance claims must
  be visible at the boundary they describe.

**Open questions**
- Whether coordinator output should be one receipt per projection or a batch
  receipt with per-projection rows.

### Phase 5: Authoring Snapshot Broadening

This phase broadens `WorthUiRuntimeAuthoringSnapshot` into the runtime-owned
derived authoring view that projections use for app, workspace, shell, page,
layout, content, surface, component, runtime, and appearance meaning.

**Relevant subsystems**
- source parsing and lowering
- runtime launch preparation
- active runtime state
- source ingress candidate submission
- projection planning

**Relevant APIs**
- `WorthUiRuntimeAuthoringSnapshot`
- `WorthUiRuntimeLaunchBuilder::prepare_for`
- `WorthUiParsedSourceToArtifactInputLowerer`
- `WorthUiArtifactInputResolver`
- `WorthUiCanonicalArtifactAssembler`
- `WorthUiRuntimeHost::active_authoring_snapshot`

**Build shape**
- Expand the snapshot beyond layout topology and content slots to include every
  authoring summary already lowerable from canonical source and artifact input.
- Preserve the snapshot as derived runtime state. It must always be rebuildable
  from source and canonical artifact lowering.
- Replace the active authoring snapshot only after successful runtime
  activation.
- Expose narrow runtime-owned read methods for projection planners rather than
  exporting parser or lowerer internals.
- Attach snapshot digest or equivalence basis to runtime change evidence where
  source reload changes authoring meaning.

**Warnings**
- Do not turn the authoring snapshot into a second canonical app model.
- Do not let validation app code or renderers read parser structures directly.
- Do not update the active snapshot before the active artifact and execution
  plan have safely swapped.

**Test requirements**
- Invalid source reload preserves the old active artifact, execution plan, and
  authoring snapshot.
- Activated source reload swaps artifact, execution plan, and authoring snapshot
  atomically.
- Page, shell, and content projections cannot import file-authored lowerer
  internals.
- Equivalent source with different declaration order converges to the same
  authoring snapshot meaning.

**Engineering decisions**
- Authoring snapshot breadth should grow only from already-proven lowering
  facts; projection planners should not re-derive source meaning.
- Snapshot replacement belongs to activation, not candidate preparation.

**Open questions**
- Whether the snapshot should expose separate typed catalogs per authoring
  family or one sealed inspection facade with family-specific accessors.

### Phase 6: Capability Reload Family Generalization

This phase turns current theme, command, and command-projection reload support
into a general capability-family reload pipeline that can admit density,
appearance, actions, and other capability-backed families without new local
reload machines.

**Relevant subsystems**
- capability reload
- capability snapshot replacement
- command and command projection descriptors
- theme token descriptors
- future density and appearance descriptors

**Relevant APIs**
- `WorthUiCapabilityReloadRequest`
- `WorthUiCapabilityPreparedReload`
- `WorthUiCapabilityReloadEvidence`
- `CapabilitySnapshot`
- `CapabilitySnapshotIndex`
- `ThemeTokenDescriptor`
- command descriptors and command projection descriptors

**Build shape**
- Keep `WorthUiCapabilityReloadRequest` enum-dispatched, but formalize each
  variant as a capability-family implementation with shared admission,
  equivalent-no-op, stale-prepared, activation, and changed-fact behavior.
- Preserve existing theme, command, and command-projection reloads as the first
  family implementations.
- Add admitted request and evidence slots for density and appearance when the
  supporting descriptors exist.
- Report edited delta width separately from full capability-family rebuild
  breadth.
- Reject stale prepared reloads against active snapshot drift for every family.

**Warnings**
- Do not let capability reload become a bag of optional fields.
- Do not let unknown capability IDs create partial candidate snapshots.
- Do not flatten rich action, posture, density, or appearance meaning into
  booleans or local style state.

**Test requirements**
- A stale prepared reload for any admitted family cannot overwrite newer active
  capability truth.
- Multi-family capability reload reports exact touched facts and family rebuild
  breadth separately.
- Unknown capability IDs fail admission without mutating runtime truth.
- Duplicate or conflicting capability edits fail before candidate snapshot
  construction.

**Engineering decisions**
- Capability families share lifecycle and activation mechanics; they do not
  have to share payload structure.
- Future action and appearance reloads should plug into this family pipeline
  rather than copying theme reload.

**Open questions**
- Whether multi-family capability reload should activate as one atomic snapshot
  replacement or as ordered family replacements inside one frame boundary.

### Phase 7: Query-Bound Runtime Change Integration

This phase defines how Query-owned live views, computed state, effects,
async/result posture, projection facts, and virtualized data participate in the
same hot reload spine.

**Relevant subsystems**
- Query binding semantics
- live rebind planning
- virtualized data lane
- runtime dependency hooks
- Query support receipts

**Relevant APIs**
- `ViewBindingDescriptor`
- `WorthUiBoundBindingSemantics`
- `WorthUiRuntimeDependencyHook`
- `WorthUiQuerySupportReceipt`
- `WorthUiQueryBindingComparison`
- `WorthUiQueryLiveRebindPlan`
- `WorthUiVirtualizedDataFrameTarget`
- Forge Query projection-consumption and runtime facade surfaces

**Build shape**
- Represent Query binding changes, live-view posture changes, projection fact
  changes, and virtualized frame target changes as runtime facts.
- Reuse `WorthUiQueryBindingComparison` and `WorthUiQueryLiveRebindPlan` instead
  of adding app-local hydration or local result-state reconciliation.
- Preserve Query-owned async/result-state, support, recovery, inspection, and
  projection-consumption posture through Worth UI receipts.
- Ensure Query-bound projections declare the exact bindings, projection facts,
  or virtualized data targets they consume.
- Add denial evidence when a requested Query-backed reload requires a Query
  support row that is not admitted.

**Warnings**
- Do not introduce a validation-app dependency on `forge-query`.
- Do not invent local loading, retry, stale, denied, or cancelled enums for
  states Query already represents.
- Do not materialize full collections merely to make UI reload bookkeeping easy.

**Test requirements**
- Query binding posture drift rebinds only projections that consume the affected
  binding.
- Async/result posture changes remain Query-owned facts and do not become local
  validation-app status enums.
- Virtualized data reload preserves bounded frame target behavior and does not
  materialize off-screen rows.
- Query-bound projections cannot be updated by app-local hydration or direct
  `forge-query` dependency in the validation app.

**Engineering decisions**
- Worth UI presents Query-owned meaning; Query remains the ordinary runtime
  owner of live, async, recovery, inspection, and projection-consumption
  semantics.
- Query-related changed facts should be derived from existing binding and live
  rebind planning evidence, not from UI-local observation.

**Open questions**
- Whether Query projection-consumption receipts should become first-class
  projection dependency entries immediately or remain represented through
  binding facts in the first implementation.

### Phase 8: Native Validation App Proof Slice

This phase rebuilds the validation app slice around runtime projection receipts
so manual verification exercises the same hot reload spine as automated tests.

**Relevant subsystems**
- validation app launch
- validation runtime workbench
- reload loop
- header renderer
- native renderer boundary

**Relevant APIs**
- `PreparedValidationWorkbenchLaunch`
- `ValidationRuntimeWorkbench`
- `WorthUiHeaderFramePlan`
- `WorthUiPageHostPlan`
- runtime change evidence from Phase 1
- projection rebind coordinator from Phase 4

**Build shape**
- Render header, page host, and evidence surfaces from runtime projection
  receipts only.
- Prove visible hot reload of header text, theme color, dropdown selection
  mode, page slot assignment, and at least one appearance or density-like
  projection when admitted.
- Add one Query or state-backed visible projection as soon as Phase 7 exposes a
  supported path.
- Remove bespoke app-local reload branching wherever the coordinator can own
  the behavior.
- Keep raw `egui` usage inside the approved renderer/native boundary files.

**Warnings**
- Do not rebuild a harness, web app, or design prototype outside the native
  `egui` runtime.
- Do not let the validation app own a shell map, page-host map, menu authority,
  theme state, or reload state machine.
- Do not hide failed reloads by simply keeping the old pixels without evidence.

**Test requirements**
- Manual-visible app can change text, color, dropdown mode, and page slot
  assignment through file edits without restart.
- App code cannot directly construct runtime change evidence or projection
  receipts.
- App code cannot store local page-host, shell, menu, or theme authority outside
  Worth UI runtime receipts.
- Renderer code can paint receipts but cannot mutate runtime truth.

**Engineering decisions**
- The validation app is an end-to-end product proof, not a harness and not a
  second runtime.
- Manual verification must observe the same receipts and evidence the tests
  assert.

**Open questions**
- Whether the first Query/state-backed visible projection should be a tiny
  diagnostic panel or a real dashboard-like data surface.

### Phase 9: Compiler Enforcement And Compile-Fail Guards

This phase locks the hot reload architecture so future dashboard phases cannot
reintroduce local truth or local reload machines.

**Relevant subsystems**
- facade visibility
- trybuild fixtures
- validation app native-boundary tests
- projection contract construction
- reload evidence construction

**Relevant APIs**
- facade exports in `crates/worth-ui/src/facade/mod.rs`
- internal runtime modules
- projection plans and receipts from Phase 3
- runtime change evidence from Phase 1
- existing validation app compile-fail fixtures

**Build shape**
- Add compile-fail fixtures proving app code cannot mint runtime change
  evidence, source reload evidence, capability reload evidence, projection
  receipts, or prepared reloads.
- Add compile-fail or structural guards requiring projection plans to carry
  dependencies and reload families to carry changed facts.
- Preserve the rule that validation app code cannot directly depend on
  `forge-query`.
- Preserve the rule that validation app code cannot deep-import runtime
  internals.
- Preserve the rule that raw `egui` usage stays in approved renderer/native
  boundary files.

**Warnings**
- Do not rely on comments or roadmap prose to keep app code away from runtime
  internals.
- Do not expose internal module topology merely to make tests easier.
- Do not make compile-fail fixtures so synthetic that they no longer target real
  APIs a tired engineer might misuse.

**Test requirements**
- A projection plan without a dependency contract fails to compile or fails a
  structural guard at construction.
- A reload family without changed-fact evidence cannot produce activated common
  change evidence.
- App code attempting to mint evidence or receipts fails compile-fail fixtures.
- App code attempting to store local page-host, shell, menu, or theme authority
  is caught by structural guards.

**Engineering decisions**
- Enforcement should move rules as high as possible: unrepresentable first,
  compile-fail second, structural guard third.
- Compile-fail fixtures should name real misuse paths, not abstract toy
  mistakes.

**Open questions**
- Whether dependency and changed-fact contracts can be made fully
  unrepresentable in the type system during this side quest or need one
  structural guard while the API is migrating.

### Phase 10: Reload Storm, Replay, And Counter Certification

This phase closes the side quest with hostile proof that arbitrary hot reload is
deterministic, bounded, and honest under mixed edit sequences.

**Relevant subsystems**
- reload storm certification
- reload counter boundary
- steady frame counter boundary
- runtime impact narrowing
- Forge Foundational performance evidence

**Relevant APIs**
- `WorthUiReloadStormCertification`
- `WorthUiReloadCertificationBundle`
- `WorthUiReloadLoweringCounterReceipt`
- `WorthUiCertifiedReloadLoweringCounterReceipt`
- `WorthUiSteadyFrameCounterReceiptBuilder`
- `WorthUiFoundationalCounterEvidence`
- `WorthUiRuntimeImpactNarrower`

**Build shape**
- Certify mixed source, capability, Query, projection, and state reload
  sequences.
- Prove invalid reloads preserve active truth and prior projections.
- Prove equivalent reloads do not churn projection frames.
- Prove changed-fact intersections bound projection rebuild breadth.
- Emit counter-backed evidence consumable by the validation app and existing
  diagnostics surfaces.
- Prove steady frames after reload do not reintroduce source parsing, registry
  string lookup, broad artifact scans, or local hydration.

**Warnings**
- Do not accept visual survival as certification.
- Do not hide broad scans behind cheap-looking coordinator APIs.
- Do not couple diagnostics richness to the operational hot path.

**Test requirements**
- Randomized reload storms converge to the same active artifact, authoring
  snapshot, capability snapshot, and projection frame digests under replay.
- Alternating valid, invalid, equivalent, and stale reloads never blank the app
  or drift active runtime truth.
- Projection rebuild count equals changed dependency intersection, not total
  registered projection count.
- Steady-frame certification proves no source parsing, artifact validation,
  registry string lookup, broad artifact scan, or local hydration occurs after
  activation.

**Engineering decisions**
- Certification must include both automated hostile tests and product-visible
  evidence surfaces.
- Counter evidence belongs at the operation boundary it claims to certify.

**Open questions**
- Whether screenshot-golden capture should be added here or left to the later
  developer tooling milestone once the runtime proof is complete.

## Must Ship

- common runtime change evidence over source, capability, Query, and state
  reload families
- expanded runtime fact taxonomy for source, capability, Query, layout,
  content, shell, page, component, appearance, action, and durable-state changes
- common projection plan contract with declared dependencies and equivalence
  basis
- runtime-owned projection rebind coordinator
- broadened runtime authoring snapshot derived from the existing source and
  artifact pipeline
- generalized capability reload family pipeline
- Query-bound reload integration that consumes Query-owned posture rather than
  rebuilding local state
- native validation app proof slice using runtime receipts only
- compiler enforcement and compile-fail guards against local reload authority
- reload storm, replay, and counter certification

## Must Preserve

- `WorthUiRuntimeHost` ownership of active artifact, active execution plan,
  capability snapshot, authoring snapshot, diagnostics, and activation state
- existing source -> artifact -> runtime proof chain
- Query ownership of live, async/result, recovery, inspection, projection
  consumption, and support posture
- prior-valid runtime truth on denied, stale, unreadable, or invalid reloads
- renderer boundary as paint-only consumption of runtime receipts
- no app-local dependency graph, hydration graph, reload state machine, shell
  map, page map, command map, or theme state
- no per-frame source interpretation, registry string lookup, broad artifact
  scan, or broad projection rebuild hidden behind convenient APIs

## Acceptance Evidence

- header, page-host, theme, command, command-projection, and at least one
  source-authored page/content projection rebind through one runtime change and
  projection coordinator path
- a running native validation app visibly hot reloads text, color, dropdown
  mode, page slot assignment, and one broader projection without restart
- denied, stale, equivalent, valid, and mixed reloads preserve runtime truth and
  produce typed evidence
- projection rebuild breadth is bounded by changed-fact and dependency
  intersection, with counters proving the claim
- Query-bound reload evidence preserves Query-owned posture and does not create
  validation-app local status models
- compile-fail guards prevent app code from minting reload evidence, projection
  receipts, direct Query dependency, or local shell/page/menu/theme authority
- reload storm certification proves deterministic replay and no steady-frame
  broad scans after activation

## Sequencing Notes

- This side quest should land before expanding the Shopify dashboard proof
  beyond the current minimal native validation app slice.
- Milestone 4 remains the authoring and product-hardening milestone; this side
  quest clears the platform reload blocker that Milestone 4 exposed.
- The first implementation phases should migrate existing header and page-host
  reload support rather than adding new visible surfaces first.
- If a phase discovers that an existing lower artifact cannot express the needed
  changed facts or projection dependencies, the correct response is to widen the
  side quest and fix that lower structure before returning to dashboard breadth.

