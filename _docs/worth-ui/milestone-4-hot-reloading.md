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
- Law 41 is load-bearing for this milestone: a type must encode what has been
  proven about a value, not merely what the value contains. Every value named
  evidence, receipt, admitted, prepared, activated, certified, validated,
  dependency contract, changed facts, or rebind plan must be sealed,
  proof-bearing, and impossible for app code to synthesize.

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

The pessimistic form is stronger: if app code can forge a successful reload,
changed-fact basis, projection dependency contract, prepared rebind, activation
receipt, or runtime witness by assembling public fields or raw collections, the
milestone has failed even if behavioral tests pass. Proof-bearing values must
progress through typed transitions, and skipped or out-of-order transitions must
be compile-time errors wherever Rust can encode them.

## Product Decision Lock

- `WorthUiRuntimeHost` remains the owner of active artifact, active execution
  plan, capability snapshot, authoring snapshot, activation state, diagnostics,
  and projection rebind boundaries.
- `WorthUiValidationReloadEvidence` and `WorthUiCapabilityReloadEvidence` remain
  valid family evidence, but they must be able to project into one common
  runtime change envelope.
- `WorthUiRuntimeFactId` names runtime fact addresses. `WorthUiRuntimeFactSet`
  may collect addresses, but a plain fact set is not proof that a runtime change
  occurred.
- Changed-fact proof must cross runtime boundaries through sealed wrappers such
  as `WorthUiChangedRuntimeFacts`, `WorthUiCapabilityChangedFacts`,
  `WorthUiQueryBindingChangedFacts`, or equivalent family-specific admitted
  forms that only proving functions can construct.
- Projection dependencies must cross projection boundaries through sealed
  dependency contracts. A plain dependency set is declaration material, not an
  admitted projection-consumption proof.
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
- Compile-time enforcement is not a late cleanup phase. Every phase must add
  the strongest feasible enforcement at the point where the proof type is
  introduced: unrepresentable first, compile-fail second, structural guard
  third.
- The hot-reload runtime is a graph over truth, not a collection of renderer
  callbacks. Authored source and capability descriptors are authority inputs;
  authoring snapshots, primitive receipts, draw plans, event regions,
  observation receipts, diagnostics, and evidence rows are derived graph
  products.
- Every phase must state which graph facts it authors, which graph facts it
  consumes, which proof-bearing receipt it emits, and which impossible states
  become unrepresentable after that phase.
- Renderers may provide host observations such as pointer position, pressed
  button, available frame size, and elapsed time. They may not classify those
  observations into UI truth, interaction meaning, disabled posture, active
  appearance state, command readiness, layout legality, or diagnostic meaning.
- The runtime graph must classify host observations against active truth and
  lowered receipts before paint or interaction emission. If a surface is
  disabled, inert, readonly, hidden, unsupported, denied, stale, or otherwise
  non-operable, the graph must emit a typed receipt for that state instead of
  asking renderer code to remember which booleans to suppress.
- Graph/index views are part of the proof boundary, not performance cleanup.
  Any phase that repeatedly locates surfaces, ownership, parent/child
  containment, projection consumers, event regions, or changed-fact dependents
  must consume a runtime-owned graph/index view rather than rebuilding the same
  lookup through surface-local traversal.

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
- Add a common runtime change pipeline whose public types encode progression:
  raw family observations become classified family changes, classified changes
  become admitted changed facts, admitted changed facts become rebind-eligible
  runtime change evidence, and only activated evidence can drive projection
  rebuild.
- Add a common runtime change evidence type carrying runtime instance identity,
  activation status, admitted changed facts, family payload summary, denial
  posture, and counters.
- Let source reload evidence and capability reload evidence project into the
  common envelope without erasing their family-specific status or stages.
- Model each contributing reload family as its own row inside the common
  evidence, because one runtime observation boundary can contain a valid source
  activation, an equivalent command reload, and a denied theme reload at the
  same time.
- Derive the top-level activation posture from the family rows without hiding
  mixed outcomes. A mixed outcome must remain visibly mixed rather than being
  collapsed into `Activated` or `Denied`.
- Require all family rows in one evidence envelope to carry the same runtime
  instance witness. A foreign or stale family row must be rejected before the
  envelope can become rebind-eligible.
- Preserve `ReadyForFrameBoundary`, `Activated`, `EquivalentNoOp`, and `Denied`
  as distinct postures because projection rebind must treat them differently.
- Require every activated family row to carry a non-empty runtime-owned
  changed-fact proof. Empty changed facts are valid only for equivalent or
  denied rows.
- Distinguish raw fact sets from changed-fact proof. The common envelope may
  expose read-only fact addresses for diagnostics, but later rebind APIs must
  consume `WorthUiAdmittedRuntimeChangeEvidence` or an equivalent sealed proof
  type rather than a public `WorthUiRuntimeFactSet`.
- Keep envelope construction runtime-owned so app code cannot mint successful
  reload evidence.
- Give the evidence a stable digest derived from runtime identity, ordered
  family rows, family statuses, changed facts, denial posture, and family
  payload digests so later rebind and replay phases do not invent a second
  equivalence basis.

Target shape:

```rust
pub struct WorthUiClassifiedRuntimeChange {
    runtime_instance: WorthUiRuntimeInstanceWitness,
    family_rows: WorthUiClassifiedRuntimeChangeRows,
}

pub struct WorthUiAdmittedRuntimeChangeEvidence {
    runtime_instance: WorthUiRuntimeInstanceWitness,
    posture: WorthUiRuntimeChangeActivationPosture,
    changed_facts: WorthUiChangedRuntimeFacts,
    family_payloads: WorthUiRuntimeChangeFamilyPayloads,
    digest: WorthUiRuntimeChangeEvidenceDigest,
    counters: WorthUiRuntimeChangeCounters,
}

pub enum WorthUiRuntimeChangeActivationPosture {
    EquivalentNoOp,
    ReadyForFrameBoundary,
    Activated,
    Denied,
    Mixed(WorthUiRuntimeChangeMixedPosture),
}

pub struct WorthUiRuntimeChangeFamilyRow {
    family: WorthUiRuntimeChangeFamily,
    status: WorthUiRuntimeChangeFamilyStatus,
    changed_facts: WorthUiChangedRuntimeFacts,
    denial: Option<WorthUiRuntimeChangeDenial>,
    payload_digest: u64,
}
```

**Warnings**
- Do not flatten source and capability reload into one weak enum that loses
  typed denial stages.
- Do not flatten mixed-family reload outcomes into a single top-level success
  or failure bit.
- Do not let projections consume raw family evidence directly once the common
  envelope exists.
- Do not let any API that can trigger rebind accept a plain
  `WorthUiRuntimeFactSet` as change authority.
- Do not let app code build activated evidence from strings or copied counters.
- Do not promote a raw runtime instance number into a typed runtime identity
  outside the runtime-owned construction boundary.
- Do not put theme-specific names into common capability payloads; theme,
  command, command-projection, action, density, and appearance rows must remain
  family-neutral at the common evidence layer.

**Test requirements**
- Equivalent source reload and equivalent capability reload project to common
  evidence that preserves prior projections without rebuild.
- Mixed source and capability reload projects into one common evidence envelope
  whose family rows preserve each family status, changed-fact set, denial
  detail, and payload digest in canonical order.
- A mixed envelope with one activated family and one denied family remains
  visibly mixed; tests must fail an implementation that reports only
  `Activated` or only `Denied`.
- A prepared reload from one runtime cannot produce rebind-eligible evidence for
  a different runtime.
- Denied reload evidence cannot trigger projection rebuild.
- Activated reload evidence without a runtime-owned changed-fact basis fails at
  the construction boundary.
- A family row with empty changed facts is allowed only when its status is
  equivalent or denied; activated rows with empty changed facts fail runtime
  construction.
- Compile-fail coverage proves a caller cannot pass raw family evidence, a raw
  fact set, or a classified-but-unadmitted change into projection rebind.
- Common counters do not merge unlike family counters into fake totals. Tests
  must assert common boundary counters separately from typed source/capability
  payload counters.
- Evidence digests are stable across equivalent family-row construction order
  and differ when status, changed facts, denial posture, or payload digest
  differs.
- Compile-fail coverage proves app code cannot construct an activated common
  evidence envelope, a common family row, or a forged runtime instance witness.

**Engineering decisions**
- The shared envelope is a projection-consumption contract, not a replacement
  for family evidence.
- The common evidence pipeline is phase typed. Later APIs should accept the
  strongest proof type they need rather than accepting a weaker type and
  rechecking posture at runtime.
- Runtime instance identity must remain part of the envelope so stale or foreign
  evidence cannot drive local UI changes.
- The common envelope is the first durable boundary for projection rebind
  replay. Family evidence remains the authoritative source of family detail;
  common evidence carries the ordered, digestible, rebind-facing projection of
  those facts.
- Phase 1 must include mixed-family proof even before the coordinator exists,
  because later arbitrary hot reload depends on preserving partial activation,
  denial, and equivalence truth in one runtime observation boundary.

**Closeout requirement**
- Phase 1 is not complete until no rebind-facing API can accept raw family
  evidence, raw fact sets, classified-but-unadmitted changes, or forged runtime
  witnesses, and tests prove mixed valid/equivalent/denied family evidence keeps
  exact per-family status, changed facts, denial detail, counters, and digest.

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
  comparison outcomes into sealed changed-fact proof types where the proof
  already exists.
- Keep `WorthUiRuntimeFactSet` as an address collection. Add sealed
  proof-bearing wrappers for changed facts produced by runtime authority:
  capability deltas, source validation deltas, Query binding comparisons,
  authoring snapshot changes, and durable state family changes must not all be
  flattened into one public mutable set.
- Make the proof wrappers move through the later pipeline. Projection rebind
  should consume admitted changed facts through runtime change evidence, not
  reconstruct proof from visible fact addresses.
- Represent page templates and page instances as separate fact families because
  a template edit can affect many instances while an instance edit should
  usually remain scoped to one live page identity.

Target shape:

```rust
WorthUiRuntimeFactId::page_template(template_id)
WorthUiRuntimeFactId::page_instance(instance_id)
WorthUiRuntimeFactId::page_instance_template_binding(instance_id, template_id)

pub struct WorthUiChangedRuntimeFacts {
    facts: WorthUiRuntimeFactSet,
    proof: WorthUiChangedRuntimeFactsProof,
}

pub struct WorthUiCapabilityChangedFacts {
    changed_facts: WorthUiChangedRuntimeFacts,
    capability_snapshot_digest_before: u64,
    capability_snapshot_digest_after: u64,
}
```

**Warnings**
- Do not let `active_artifact` become the default invalidation answer for every
  edit once narrower facts can be derived.
- Do not introduce fact names that mix authority categories such as source
  origin, visual hierarchy, and runtime state in one string.
- Do not treat a public fact set as evidence. Fact addresses are diagnostic and
  intersection vocabulary; changed-fact wrappers are proof.
- Do not compute facts by reparsing source inside projection planning.
- Do not let conversion helpers accept raw strings or public counters as proof
  that a change occurred.

**Test requirements**
- A content-slot edit invalidates the page-host/content facts it touches but
  does not invalidate unrelated header theme facts.
- A theme-token edit invalidates theme consumers but not layout topology or
  content-mount consumers.
- Fact-set canonical ordering and digest behavior are stable across equivalent
  source declaration order.
- Malformed or unknown fact identity cannot be smuggled through the public
  facade.
- Compile-fail coverage proves app code cannot construct changed-fact proof
  wrappers, capability changed facts, Query binding changed facts, or durable
  state changed facts directly.
- Compile-fail coverage proves raw `WorthUiRuntimeFactSet` cannot be passed to
  APIs that require admitted changed-fact proof.

**Engineering decisions**
- Runtime facts are semantic contracts, not debug labels.
- Runtime facts are not themselves proof of change. The taxonomy gives the
  runtime a precise language; sealed changed-fact wrappers encode the authority
  claim that those facts actually changed.
- Broad facts remain valid for truly broad changes, but the ordinary path must
  prefer the narrowest honest fact set.
- Page-template facts and page-instance facts are separate families. The binding
  between them is its own fact because correspondence changes have different
  invalidation meaning than either template or instance edits.

**Closeout requirement**
- Phase 2 is not complete until the runtime fact vocabulary can name every
  source, capability, Query, layout, content, shell, page, component,
  appearance, action, and durable-state change required by this milestone, and
  every API that claims a change occurred requires sealed changed-fact proof
  instead of a public `WorthUiRuntimeFactSet`.

**Open questions**
- None.

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
- Split projection dependency declaration from admitted projection dependency
  contract. A projection may author dependency addresses, but the runtime must
  validate and seal them before the projection can enter the reloadable plan
  registry.
- Build the contract as a real platform projection interface with
  mechanically-enforced construction, not as a closed central enum. Header and
  page-host are first implementations, not the limit of the model.
- Migrate header menu, header theme, header frame, and page-host plans onto the
  shared contract while preserving their typed family-specific receipts.
- Make dependency declaration required at plan construction time.
- Make admitted dependency contracts required at plan admission time. A plan
  that only carries raw dependency sets is not reloadable and must be rejected
  before it can receive projection rebind evidence.
- Keep projection receipts derived from plans and runtime authority; app code
  may inspect receipts but must not construct them.
- Add enough family tagging that diagnostics can explain which projection
  family preserved, rebuilt, or denied under a change.
- Keep implementation extensibility behind runtime-owned proving constructors so
  downstream app code cannot implement the contract by hand and smuggle fake
  dependencies or receipts.

Target shape:

```rust
pub trait WorthUiProjectionPlanContract {
    type Request;
    type FrameReceipt;
    type RebuildDenial;

    fn projection_identity(&self) -> WorthUiProjectionIdentity;
    fn projection_family(&self) -> WorthUiProjectionFamily;
    fn dependencies(&self) -> &WorthUiValidatedProjectionDependencyContract;
    fn equivalence_basis(&self) -> &WorthUiProjectionEquivalenceBasis;
}

pub struct WorthUiAdmittedProjectionPlan<P: WorthUiProjectionPlanContract> {
    plan: P,
    construction_proof: WorthUiProjectionPlanProof,
}

pub struct WorthUiProjectionDependencyDeclaration {
    dependencies: WorthUiProjectionDependencySet,
}

pub struct WorthUiValidatedProjectionDependencyContract {
    dependencies: WorthUiProjectionDependencySet,
    validation_proof: WorthUiProjectionDependencyValidationProof,
}
```

**Warnings**
- Do not replace typed header or page-host receipts with an untyped projection
  bag.
- Do not allow projection dependencies to be appended later by the renderer.
- Do not let family planners or renderers self-certify dependency contracts.
  Runtime-owned admission must produce the validated dependency contract.
- Do not hide equivalence behind raw digest comparison without naming the basis.
- Do not let downstream app code implement the projection contract without a
  runtime-sealed admission path.

**Test requirements**
- A projection plan without declared dependencies cannot be registered or
  rebound.
- A projection plan with raw dependencies but no validated dependency contract
  cannot enter the admitted projection registry.
- A projection whose dependencies do not intersect changed facts preserves its
  frame digest and reports reuse.
- A projection whose dependencies intersect changed facts must rebuild or return
  a typed denial.
- Equivalent rebuilds report equivalent-after-activation rather than false
  changed output.
- Compile-fail coverage proves app code cannot construct an admitted projection
  plan, validated dependency contract, projection plan proof, or projection
  frame receipt.
- Compile-fail coverage proves render code cannot append dependencies or
  convert a raw dependency declaration into an admitted contract.

**Engineering decisions**
- Projection plans are derived state; they must be rebuildable from runtime
  authority and declared requests.
- The common contract standardizes lifecycle and evidence while family planners
  still own family-specific frame construction.
- Dependency validation is a proof transition, not a helper check. APIs after
  admission must consume the validated contract and should not defensively
  revalidate what the type guarantees.
- The primary model is a trait-like projection contract with proof-bearing,
  runtime-owned construction. A sealed enum may be used internally for current
  admitted families only if it remains an implementation detail and does not
  become the platform contract.

**Closeout requirement**
- Phase 3 is not complete until header menu, header theme, header frame, and
  page-host plans all carry admitted dependency contracts and equivalence bases
  through the shared projection contract, and app/renderer code cannot construct
  admitted plans, validated dependency contracts, or projection receipts.

**Open questions**
- None.

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
  admitted projection plans plus admitted runtime change evidence.
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
- Emit one batch receipt per rebind operation with per-projection row receipts
  so the runtime boundary has one counter surface while diagnostics still show
  individual projection outcomes.
- Keep coordinator input phase typed. Ready-but-unactivated evidence, denied
  evidence, activated evidence, and equivalent evidence may share reporting
  vocabulary, but the operation that performs rebuild must require the activated
  admitted evidence type. Preserve-only paths may accept narrower no-op or
  denial evidence types only when their signatures make rebuilding impossible.

Target shape:

```rust
pub struct WorthUiProjectionRebindBatchReceipt {
    runtime_instance: WorthUiRuntimeInstanceId,
    change_evidence_digest: WorthUiRuntimeChangeEvidenceDigest,
    counters: WorthUiProjectionRebindCounters,
    rows: Vec<WorthUiProjectionRebindRowReceipt>,
}

pub struct WorthUiProjectionRebindPlan {
    runtime_instance: WorthUiRuntimeInstanceWitness,
    evidence: WorthUiAdmittedRuntimeChangeEvidence,
    admitted_projections: WorthUiAdmittedProjectionRegistrySnapshot,
    affected_projection_count: usize,
}

pub struct WorthUiProjectionRebindRowReceipt {
    projection_identity: WorthUiProjectionIdentity,
    projection_family: WorthUiProjectionFamily,
    status: WorthUiProjectionRebindStatus,
    previous_frame_digest: u64,
    rebound_frame_digest: u64,
}
```

**Warnings**
- Do not keep duplicating `ReadyForFrameBoundary` / `Denied` / `Activated`
  logic inside each projection family.
- Do not let the coordinator call render code or hold `egui` state.
- Do not rebuild all registered projections when changed facts intersect only
  one projection family.
- Do not let the coordinator accept raw changed facts, raw dependency sets, raw
  projection plans, or runtime digests in place of proof-bearing inputs.
- Do not implement preserve and rebuild through one baggy method that accepts
  every status and then switches internally. The type signatures should expose
  which transitions can rebuild and which can only preserve.

**Test requirements**
- Header and page-host rebind through the same coordinator for one activated
  runtime change envelope.
- Ready-but-unactivated evidence is denied before any family projection planner
  runs.
- Denied and equivalent evidence preserve projection frames without rebuild.
- Multiple projections rebind with rebuild breadth equal to changed dependency
  intersections, not total registered projection count.
- Compile-fail coverage proves a raw fact set, raw evidence digest,
  classified-but-unadmitted evidence, or raw projection dependency declaration
  cannot enter the rebuild path.
- Compile-fail coverage proves a preserve-only evidence posture cannot call the
  activated rebuild API.

**Engineering decisions**
- The coordinator owns lifecycle classification; family planners own how to
  produce a new family plan when classification says rebuild.
- Rebind counters are part of the public proof because performance claims must
  be visible at the boundary they describe.
- Rebind is a proof-widening pipeline: admitted evidence plus admitted
  projections produce a rebind plan; a rebind plan produces receipts; receipts
  may be rendered. Later phases must not accept an earlier proof type just to
  make call sites easier.
- Coordinator output is a batch receipt with per-projection rows. The batch is
  the operation boundary; rows are the inspection surface.

**Closeout requirement**
- Phase 4 is not complete until header and page-host rebind through the shared
  coordinator from admitted runtime change evidence and admitted projection
  plans, rejected weaker inputs fail at compile time where possible, and tests
  prove rebuild breadth equals dependency intersections rather than registered
  projection count.

**Open questions**
- None.

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
- Expose the snapshot as one runtime-owned facade with typed catalog accessors,
  not as public fields and not as one generic map. The single snapshot preserves
  authority; typed accessors preserve semantic boundaries.
- Treat active authoring snapshot replacement as proof-bearing activation
  output. Candidate snapshots may be inspectable for diagnostics, but projection
  planners must consume only the active runtime-owned snapshot witness.

Target shape:

```rust
impl WorthUiRuntimeAuthoringSnapshot {
    pub fn workspace_shell(&self) -> &WorthUiWorkspaceShellCatalog;
    pub fn page_templates(&self) -> &WorthUiPageTemplateCatalog;
    pub fn page_instances(&self) -> &WorthUiPageInstanceCatalog;
    pub fn layout_topology(&self) -> &WorthUiLayoutTopologyCatalog;
    pub fn content_slots(&self) -> &WorthUiContentSlotCatalog;
    pub fn appearance_recipes(&self) -> &WorthUiAppearanceRecipeCatalog;
    pub fn runtime_bindings(&self) -> &WorthUiRuntimeBindingCatalog;
}
```

**Warnings**
- Do not turn the authoring snapshot into a second canonical app model.
- Do not let validation app code or renderers read parser structures directly.
- Do not update the active snapshot before the active artifact and execution
  plan have safely swapped.
- Do not let candidate or parsed snapshot forms enter projection planning APIs.

**Test requirements**
- Invalid source reload preserves the old active artifact, execution plan, and
  authoring snapshot.
- Activated source reload swaps artifact, execution plan, and authoring snapshot
  atomically.
- Page, shell, and content projections cannot import file-authored lowerer
  internals.
- Equivalent source with different declaration order converges to the same
  authoring snapshot meaning.
- Compile-fail coverage proves projection planners cannot consume parser
  structures, candidate snapshot structures, or unactivated authoring snapshot
  candidates.
- Compile-fail coverage proves app code cannot mint an active authoring
  snapshot witness or swap the active snapshot independently of runtime
  activation.

**Engineering decisions**
- Authoring snapshot breadth should grow only from already-proven lowering
  facts; projection planners should not re-derive source meaning.
- Snapshot replacement belongs to activation, not candidate preparation.
- The snapshot is one sealed runtime-owned facade with typed family accessors.
  This keeps the derived authoring view unified while preventing generic-map
  ambiguity.
- Active snapshot accessors are read-only proof surfaces. They expose what
  activation proved without allowing consumers to assemble an equivalent
  authority object.

**Closeout requirement**
- Phase 5 is not complete until projection planners consume only active,
  runtime-owned authoring snapshot views for workspace, page, layout, content,
  surface, component, runtime, and appearance meaning, and candidate/parser
  snapshots cannot enter active projection planning APIs.

**Open questions**
- None.

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
- Split raw capability reload requests from admitted capability reload
  candidates. A parsed package or request variant is not proof that the
  capability family can enter the active snapshot.
- Preserve existing theme, command, and command-projection reloads as the first
  family implementations.
- Add admitted request and evidence slots for density and appearance when the
  supporting descriptors exist.
- Report edited delta width separately from full capability-family rebuild
  breadth.
- Reject stale prepared reloads against active snapshot drift for every family.
- Activate multi-family capability reloads as one atomic candidate
  `CapabilitySnapshot` replacement. Evidence may contain per-family rows, but
  active capability truth must not partially commit by family.
- Carry capability changed facts as sealed family proof. The common runtime
  change envelope should consume admitted capability changed facts, not
  recompute deltas from descriptor strings.

Target shape:

```rust
pub struct WorthUiAdmittedCapabilityReloadBatch {
    candidate_snapshot: CapabilitySnapshot,
    family_rows: Vec<WorthUiCapabilityReloadFamilyRow>,
    changed_facts: WorthUiCapabilityChangedFacts,
}
```

**Warnings**
- Do not let capability reload become a bag of optional fields.
- Do not let unknown capability IDs create partial candidate snapshots.
- Do not flatten rich action, posture, density, or appearance meaning into
  booleans or local style state.
- Do not let a raw capability package, source path, or descriptor map masquerade
  as an admitted capability reload.

**Test requirements**
- A stale prepared reload for any admitted family cannot overwrite newer active
  capability truth.
- Multi-family capability reload reports exact touched facts and family rebuild
  breadth separately.
- Unknown capability IDs fail admission without mutating runtime truth.
- Duplicate or conflicting capability edits fail before candidate snapshot
  construction.
- Compile-fail coverage proves app code cannot construct admitted capability
  reload batches, capability changed facts, or activated capability evidence.
- Compile-fail coverage proves a raw capability request cannot be passed to the
  common runtime change envelope or projection rebind coordinator.

**Engineering decisions**
- Capability families share lifecycle and activation mechanics; they do not
  have to share payload structure.
- Future action and appearance reloads should plug into this family pipeline
  rather than copying theme reload.
- Multi-family capability reload activates as one atomic snapshot replacement.
  Ordered family rows are diagnostics and admission evidence, not partial commit
  boundaries.
- Capability reload follows the same proof chain as source reload: request,
  admitted candidate, changed-fact proof, activated evidence, projection rebind.
  Later APIs should never accept an earlier proof stage.

**Closeout requirement**
- Phase 6 is not complete until capability reload is genuinely family-dispatched
  and atomic across multi-family batches, theme/command/command-projection rows
  use the shared lifecycle, stale prepared reloads are rejected for every
  admitted family, and raw capability requests cannot enter runtime change or
  projection rebind APIs.

**Open questions**
- None.

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

**Query surfaces to use**
- Public runtime facade and workspace runtime handles from
  `crates/forge-query/docs/foundations/workspace-overview.md`.
- Support and admission posture from
  `crates/forge-query/docs/foundations/support-matrix-and-admission.md`.
- Aspect and authority-lane contracts from
  `crates/forge-query/docs/modeling/aspects-and-authority-lanes.md`.
- Runtime state snapshots from
  `crates/forge-query/docs/foundations/state.md`.
- Projection-consumption declarations and typed fact receipts from
  `crates/forge-query/docs/capabilities/projection-consumption.md`.
- Async/resource result-state posture from
  `crates/forge-query/docs/capabilities/async-resources-and-result-state.md`.
- Inspection surfaces from
  `crates/forge-query/docs/capabilities/inspection.md`.
- Recovery boundary and next-action surfaces from
  `crates/forge-query/docs/domain-capabilities/recovery-boundary.md` and
  `crates/forge-query/docs/domain-capabilities/recovery/recovery-requests-and-next-step-actions.md`.
- Effect posture and effect-intent receipts from
  `crates/forge-query/docs/execution/effects.md` and
  `crates/forge-query/docs/execution/writes-and-intents.md`.

**Build shape**
- Represent Query binding changes, live-view posture changes, projection fact
  changes, and virtualized frame target changes as runtime facts.
- Reuse `WorthUiQueryBindingComparison` and `WorthUiQueryLiveRebindPlan` instead
  of adding app-local hydration or local result-state reconciliation.
- Split Query-facing observations from admitted Query change proof. Worth UI may
  consume Query-owned receipts and comparisons, but only a runtime-owned lowerer
  may convert them into `WorthUiQueryBindingChangedFacts` or equivalent
  admitted changed-fact proof.
- Preserve Query-owned async/result-state, support, recovery, inspection, and
  projection-consumption posture through Worth UI receipts.
- Ensure Query-bound projections declare the exact bindings, projection facts,
  or virtualized data targets they consume.
- Add denial evidence when a requested Query-backed reload requires a Query
  support row that is not admitted.
- Treat Query projection-consumption receipts as first-class projection
  dependency entries where the Query support row is admitted. Binding facts can
  remain a migration aid, but the target path is typed Query materialized facts.
- Map Query support/admission denials into Worth UI reload denial evidence
  before projection rebind attempts run.
- Map Query aspect and authority-lane posture into Worth UI dependency facts so
  a projection declares whether it consumes live, computed, effect, derived
  state, recovery, or inspection meaning.
- Map Query runtime state snapshots and async/resource result-state posture into
  receipt-backed Worth UI projection facts instead of local `loading`, `retry`,
  `stale`, or `cancelled` enums.
- Map Query inspection and recovery outputs into diagnostics/evidence
  projections; do not use them as imperative local control paths.
- Preserve Query support and admission receipts as proof inputs. A binding ID,
  retained result handle, or local state label must not promote itself into
  reload authority without the corresponding Query-owned proof.

Target shape:

```rust
WorthUiRuntimeFactId::query_projection_fact(receipt_identity)
WorthUiRuntimeFactId::query_result_posture(binding_id)
WorthUiRuntimeFactId::query_live_view(binding_id)
WorthUiRuntimeFactId::query_computed_view(binding_id)
WorthUiRuntimeFactId::query_effect_posture(effect_id)
WorthUiRuntimeFactId::query_recovery_posture(recovery_id)
WorthUiRuntimeFactId::query_inspection_target(target_id)
WorthUiRuntimeFactId::virtualized_data_frame(frame_target_id)
```

Target lowering contract:

```rust
pub struct WorthUiQueryRuntimeFactLoweringInput {
    support_receipt: WorthUiQuerySupportReceipt,
    binding_comparison: WorthUiQueryBindingComparison,
    live_rebind_plan: WorthUiQueryLiveRebindPlan,
    projection_fact_receipts: Vec<WorthUiQueryProjectionFactReceipt>,
    state_snapshot_receipts: Vec<WorthUiQueryStateSnapshotReceipt>,
}

pub struct WorthUiQueryRuntimeFactLoweringReceipt {
    changed_facts: WorthUiQueryBindingChangedFacts,
    support_denials: Vec<WorthUiQuerySupportDenialReceipt>,
    consumed_projection_fact_count: usize,
    consumed_state_snapshot_count: usize,
}
```

**Warnings**
- Do not introduce a validation-app dependency on `forge-query`.
- Do not invent local loading, retry, stale, denied, or cancelled enums for
  states Query already represents.
- Do not materialize full collections merely to make UI reload bookkeeping easy.
- Do not accept raw Query binding IDs, local loading enums, retained result
  labels, or inspection targets in APIs that require admitted Query posture or
  changed-fact proof.

**Test requirements**
- Query binding posture drift rebinds only projections that consume the affected
  binding.
- Query support/admission denial becomes typed Worth UI reload denial evidence
  before any projection rebind is attempted.
- Query aspect and authority-lane changes map to distinct Worth UI runtime
  facts rather than a generic Query-changed flag.
- Async/result posture changes remain Query-owned facts and do not become local
  validation-app status enums.
- Query projection-consumption receipts become first-class dependency facts and
  cannot be replaced by local retained-row side maps.
- Query inspection and recovery outputs render through evidence projections but
  cannot mutate Worth UI runtime truth directly.
- Virtualized data reload preserves bounded frame target behavior and does not
  materialize off-screen rows.
- Query-bound projections cannot be updated by app-local hydration or direct
  `forge-query` dependency in the validation app.
- Compile-fail coverage proves app code cannot construct admitted Query
  changed-fact proof, Query support admission witnesses, or Query projection
  dependency contracts.
- Compile-fail coverage proves raw binding IDs or local result-state enums
  cannot enter the Worth UI hot-reload rebind path.

**Engineering decisions**
- Worth UI presents Query-owned meaning; Query remains the ordinary runtime
  owner of live, async, recovery, inspection, and projection-consumption
  semantics.
- Query-related changed facts should be derived from existing binding and live
  rebind planning evidence, not from UI-local observation.
- Query integration must carry Query proof forward rather than rediscovering or
  reclassifying it. Defensive reproving inside Worth UI is allowed only at
  trust-boundary ingress.
- Query projection-consumption receipts become first-class projection dependency
  entries when support is admitted. Binding facts are not the final abstraction
  for materialized Query facts.
- The Query public runtime facade, support/admission, aspects, state,
  projection-consumption, async/result-state, inspection, recovery, and effect
  surfaces are the named surfaces Worth UI must reach for. Generic references to
  "Query integration" are not sufficient implementation guidance.

**Closeout requirement**
- Phase 7 is not complete until Query-bound changes are lowered from Query-owned
  support, binding, live-rebind, projection-consumption, state, async/result,
  recovery, inspection, and effect receipts into sealed Worth UI changed facts,
  and raw binding IDs, local result enums, or UI-local Query state cannot drive
  reload or projection rebind.

**Open questions**
- None.

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
- Treat validation app launch and visible projection setup as public facade
  consumption. The app may hold read-only receipts and runtime-owned handles,
  but it must not hold proof constructors, admitted evidence internals, or
  projection dependency admission authority.
- Prove visible hot reload of header text, theme color, dropdown selection
  mode, page slot assignment, and at least one appearance or density-like
  projection when admitted.
- Add one Query or state-backed visible projection as soon as Phase 7 exposes a
  supported path.
- Make the first Query or state-backed projection small but product-shaped:
  a real native page section or status summary plus an adjacent evidence panel,
  not a diagnostics-only placeholder.
- Remove bespoke app-local reload branching wherever the coordinator can own
  the behavior.
- Keep raw `egui` usage inside the approved renderer/native boundary files.

Target shape:

```rust
struct ValidationProductSummaryProjection {
    frame_receipt: WorthUiProjectionFrameReceipt,
    evidence_receipt: WorthUiProjectionEvidenceReceipt,
}
```

**Warnings**
- Do not rebuild a harness, web app, or design prototype outside the native
  `egui` runtime.
- Do not let the validation app own a shell map, page-host map, menu authority,
  theme state, or reload state machine.
- Do not hide failed reloads by simply keeping the old pixels without evidence.
- Do not let validation fixtures use internal constructors merely because the
  product UI is local to the workspace. The validation app is still external to
  runtime authority.

**Test requirements**
- Manual-visible app can change text, color, dropdown mode, and page slot
  assignment through file edits without restart.
- App code cannot directly construct runtime change evidence or projection
  receipts.
- App code cannot store local page-host, shell, menu, or theme authority outside
  Worth UI runtime receipts.
- Renderer code can paint receipts but cannot mutate runtime truth.
- Compile-fail coverage proves validation app code cannot mint active authoring
  snapshot witnesses, changed-fact proof, admitted projection plans, projection
  rebind plans, or projection receipts.
- Structural guards prove the app does not import raw `egui` outside approved
  renderer boundaries and does not import runtime internals or `forge-query`
  directly.

**Engineering decisions**
- The validation app is an end-to-end product proof, not a harness and not a
  second runtime.
- Manual verification must observe the same receipts and evidence the tests
  assert.
- The first Query or state-backed visible projection should be a small real
  product surface with adjacent evidence, not a diagnostics-only panel.
- The validation app is intentionally a consumer of proof, not a producer of
  proof. If a test needs proof-bearing values, it must obtain them through the
  same public runtime path the app uses.

**Closeout requirement**
- Phase 8 is not complete until the native validation app renders visible
  header, page-host, and evidence surfaces from runtime receipts only, proves
  text/color/dropdown/page-slot hot reload through the same runtime path as
  automated tests, and contains no local shell, page-host, menu, theme, Query, or
  reload authority outside approved renderer boundaries.

**Open questions**
- None.

### Phase 9: Compiler Enforcement Certification Sweep

This phase audits and completes the compile-time enforcement introduced in the
earlier phases so future dashboard phases cannot reintroduce local truth or
local reload machines.

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
- Inventory every proof-bearing type introduced by Phases 1 through 8:
  evidence, receipt, admitted, prepared, activated, certified, validated,
  dependency contract, changed facts, rebind plan, active snapshot witness, and
  runtime witness.
- Add or verify compile-fail fixtures proving app code cannot mint runtime
  change evidence, source reload evidence, capability reload evidence, changed
  facts, active authoring snapshot witnesses, projection dependency contracts,
  projection plans, projection rebind plans, projection receipts, or prepared
  reloads.
- Add or verify compile-fail fixtures proving each proof-widening transition
  rejects weaker inputs: raw facts cannot become changed facts, classified
  changes cannot rebind projections, raw dependency declarations cannot become
  admitted projection contracts, preserve-only evidence cannot call rebuild,
  and candidate snapshots cannot enter active projection planning.
- Add or verify compile-fail or structural guards requiring projection plans to
  carry admitted dependency contracts and reload families to carry admitted
  changed-fact proof.
- Preserve the rule that validation app code cannot directly depend on
  `forge-query`.
- Preserve the rule that validation app code cannot deep-import runtime
  internals.
- Preserve the rule that raw `egui` usage stays in approved renderer/native
  boundary files.
- Make new hot-reload spine APIs unrepresentable when required contracts are
  missing. Structural guards are allowed only around legacy surfaces while they
  migrate to the new construction path.

Target shape:

```rust
pub struct WorthUiProjectionPlanBuilder<MissingDependencies> {
    projection_identity: WorthUiProjectionIdentity,
}

impl WorthUiProjectionPlanBuilder<MissingDependencies> {
    pub fn with_dependencies(
        self,
        dependencies: WorthUiProjectionDependencySet,
    ) -> WorthUiProjectionPlanBuilder<DependenciesDeclared>;
}

impl WorthUiProjectionPlanBuilder<DependenciesDeclared> {
    pub(crate) fn admit(self) -> WorthUiAdmittedProjectionPlan;
}
```

Certification matrix:

```text
raw family evidence -> classified runtime change -> admitted runtime change
raw fact set -> changed-fact proof -> admitted runtime change evidence
dependency declaration -> validated dependency contract -> admitted projection
active snapshot candidate -> activated snapshot witness -> projection planning
admitted evidence + admitted projections -> rebind plan -> rebind receipt
```

**Warnings**
- Do not rely on comments or roadmap prose to keep app code away from runtime
  internals.
- Do not expose internal module topology merely to make tests easier.
- Do not make compile-fail fixtures so synthetic that they no longer target real
  APIs a tired engineer might misuse.
- Do not defer compile enforcement for a proof type to this phase if the type is
  introduced earlier. This phase is certification and gap closure, not the first
  enforcement pass.
- Do not allow any public constructor or public field on a proof-bearing type
  unless the type is intentionally not proof-bearing and the name makes that
  explicit.

**Test requirements**
- A projection plan without a dependency contract fails to compile or fails a
  structural guard at construction.
- A reload family without changed-fact evidence cannot produce activated common
  change evidence.
- App code attempting to mint evidence or receipts fails compile-fail fixtures.
- App code attempting to store local page-host, shell, menu, or theme authority
  is caught by structural guards.
- Every proof-bearing type introduced by the milestone has either a compile-fail
  fixture proving it cannot be forged externally or a named explanation for why
  the compiler cannot encode that boundary.
- Every proof-widening transition in the certification matrix has a compile-fail
  fixture proving the weaker prior stage cannot skip ahead.

**Engineering decisions**
- Enforcement should move rules as high as possible: unrepresentable first,
  compile-fail second, structural guard third.
- Compile-fail fixtures should name real misuse paths, not abstract toy
  mistakes.
- New dependency and changed-fact contracts must be unrepresentable where they
  are introduced. Structural guards are transitional only and must name the
  legacy surface they protect.
- This phase should shrink over time. A healthy implementation arrives here with
  most compile-fail guards already added beside the proof type they protect.

**Closeout requirement**
- Phase 9 is not complete until every proof-bearing type introduced so far has
  either compile-fail coverage blocking external forgery and skipped-stage use or
  a named, mechanically-contained reason Rust cannot encode that boundary, and
  structural guards reject validation-app local reload authority.

**Open questions**
- None.

### Phase 10: Reload Storm, Replay, And Counter Certification

This phase closes the first certification layer with hostile proof that the
runtime-owned reload spine is deterministic, bounded, and honest under mixed
edit sequences. Later phases must still prove the full product-facing
appearance, density, component, dropdown, and native manual verification
surfaces.

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
- Add lightweight screenshot capture as supplementary manual/visual evidence,
  while keeping receipts, counters, and replay certification as the
  load-bearing correctness proof.

Target shape:

```rust
pub struct WorthUiHotReloadVisualCaptureReceipt {
    runtime_change_digest: WorthUiRuntimeChangeEvidenceDigest,
    projection_rebind_digest: WorthUiProjectionRebindBatchDigest,
    image_artifact_digest: String,
}
```

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
- Lightweight visual capture is supplementary evidence in this milestone.
  Formal screenshot-golden harness machinery remains a later developer tooling
  concern.

**Closeout requirement**
- Phase 10 is not complete until mixed reload storm certification proves replay
  convergence for active artifact, authoring snapshot, capability snapshot, and
  projection frame digests, proves changed-fact-bounded rebuild breadth, and
  proves steady frames after activation perform no source parse, artifact
  validation, registry lookup, broad artifact scan, or local hydration.

**Open questions**
- None.

## Active Implementation Status

Phases 1-10 are completed foundation phases for this side quest. They remain in
this spec as audit history and regression criteria because they define the
runtime-owned reload spine, proof progression, projection rebind coordinator,
compiler enforcement baseline, and first reload-storm certification layer.

Current implementation resumes at Phase 11. Do not replan or rebuild Phases
1-10 unless QA finds a regression against their closeout requirements. The
remaining mandatory product-completion work is Phases 11-19.

### Phase 11: Typed Appearance And Density Values

This phase makes style and density hot-reloadable runtime meaning instead of
renderer constants. It freezes the vocabulary that later component and header
projection work must consume.

**Relevant subsystems**
- capability theme tokens
- appearance descriptors
- density descriptors
- runtime fact taxonomy
- header theme planning
- native renderer boundary

**Relevant APIs**
- existing `ThemeTokenDescriptor`
- existing `ThemeTokenFamily`
- existing `ThemeTokenValue`
- existing `WorthUiRuntimeFactId`
- existing `WorthUiProjectionDependencySet`
- new `WorthUiAppearanceTokenDescriptor`
- new `WorthUiDensityTokenDescriptor`
- new `WorthUiAppearanceValue`
- new `WorthUiDensityValue`
- new typed value forms for color, length, font size, padding, spacing, border
  width, opacity where needed, corner radius where needed, and shadow or
  elevation
- new typed density forms for row spacing, container padding, hit-target
  minimums, compactness, comfortable or dense posture, and control internal
  spacing

**Build shape**
- Expand the appearance value vocabulary before widening the renderer. Values
  that mean color, length, font size, padding, spacing, border width, and shadow
  must be distinct typed values, not strings with comments.
- Keep color-compatible theme tokens as the existing theme path, but introduce
  dedicated appearance and density descriptors rather than stretching
  `ThemeTokenDescriptor` until it becomes a style bag. Theme tokens may bridge
  into appearance where the existing API already owns color truth; new
  non-color values must enter through appearance or density descriptors.
- Give each descriptor an identity, family, source, typed value, canonical
  digest basis, and declared semantic role. The descriptor model must be
  capable of representing the values needed by header/dropdown/page surfaces
  without renderer-local interpretation.
- Add exact runtime facts for appearance and density identities. A font-size
  edit must be represented as the exact font-size or appearance fact it touched,
  not as a generic theme-changed flag.
- Define canonical value normalization before reload support lands. Equivalent
  authored values such as `12px` and `12.0px`, or equivalent padding shorthands,
  must converge to the same typed value and digest; semantically different
  values must produce different digests.
- Extend header theme or appearance planning so header frame plans can declare
  dependencies on non-color appearance and density values.
- Remove renderer-owned authority for header font size, row padding, container
  padding, menu minimum width when it is appearance meaning, and shadow or
  elevation. The renderer may still perform egui painting mechanics from
  receipts.

Target shape:

```rust
pub enum WorthUiAppearanceValue {
    Color(ThemeColorValue),
    Length(WorthUiLengthValue),
    FontSize(WorthUiFontSizeValue),
    Padding(WorthUiPaddingValue),
    Spacing(WorthUiSpacingValue),
    BorderWidth(WorthUiBorderWidthValue),
    CornerRadius(WorthUiCornerRadiusValue),
    Shadow(WorthUiShadowValue),
}

pub struct WorthUiAppearanceTokenDescriptor {
    id: WorthUiAppearanceTokenId,
    family: WorthUiAppearanceFamily,
    source: WorthUiAppearanceTokenSource,
    value: WorthUiAppearanceValue,
}

pub struct WorthUiDensityTokenDescriptor {
    id: WorthUiDensityTokenId,
    family: WorthUiDensityFamily,
    value: WorthUiDensityValue,
}
```

**Warnings**
- Do not flatten all appearance values into strings or numbers.
- Do not treat non-color appearance as theme-token metadata unless the type
  system preserves the semantic distinction.
- Do not put appearance authority in `apps/worth-ui-validation-app`.
- Do not let density become a boolean compact flag if row spacing, padding, and
  posture are distinct meanings.
- Do not hide hardcoded renderer dimensions behind private constants and call
  that hot reload support.
- Do not choose convenient units that cannot round-trip canonically. Unit
  normalization is part of the platform contract.

**Test requirements**
- A font-size edit invalidates only projections that declare the exact
  font-size or appearance dependency; unrelated page slot and content
  projections are preserved.
- A row-padding edit and a container-padding edit remain distinguishable facts,
  with distinct digests and dependency intersections.
- Equivalent authored values normalize to one canonical descriptor digest, while
  semantically different length, padding, spacing, and shadow values produce
  different descriptor digests.
- Invalid length, padding, or shadow values are rejected before constructing a
  candidate capability snapshot.
- Descriptor identities are distinct across theme, appearance, and density
  domains even when their raw strings match.
- Renderer boundary tests prove header font size, row padding, container
  padding, shadow, and menu appearance are consumed from receipts rather than
  owned by app-local constants.

**Engineering decisions**
- Appearance and density are source-authored runtime meaning, not egui style
  customization.
- Color remains one appearance value, not the whole appearance system. Existing
  theme color support may bridge into appearance planning, but non-color
  appearance and density must not be forced into the theme-token model.
- Exact value facts matter because later rebuild breadth certification depends
  on narrow dependency intersection.
- Canonicalization is a Phase 11 foundation, not a Phase 12 parser cleanup. All
  later reload, digest, and replay proofs depend on it.

**Closeout requirement**
- Phase 11 is not complete until font size, row padding, container padding,
  spacing, border width, shadow or elevation, and color are represented as typed
  runtime appearance/density values with exact facts, canonical digest bases,
  descriptor identity boundaries, and projection dependencies, and the native
  header renderer no longer owns those values as hardcoded style authority.

**Open questions**
- None.

### Phase 12: Appearance And Density Capability Reload Families

This phase admits appearance and density edits through the same capability
reload proof chain as theme, command, and command projection reloads.

**Relevant subsystems**
- capability reload
- capability snapshot replacement
- appearance descriptors
- density descriptors
- changed-fact proof wrappers
- runtime change evidence envelope

**Relevant APIs**
- existing `WorthUiCapabilityReloadRequest`
- existing `WorthUiCapabilityPreparedReload`
- existing `WorthUiCapabilityReloadEvidence`
- existing `WorthUiCapabilityChangedFacts`
- existing `CapabilitySnapshot`
- existing `WorthUiAdmittedRuntimeChangeEvidence`
- new appearance reload package
- new density reload package
- new appearance and density family rows
- new appearance and density canonicalization receipts
- new appearance and density parse/admission denials

**Build shape**
- Add family-dispatched capability reload request variants or an equivalent
  sealed family implementation for appearance and density.
- Parse source-authored appearance and density packages into typed descriptors
  before candidate snapshot construction.
- Lower parsing into a canonical package before comparison. The package must
  preserve source identity for diagnostics while comparing and digesting through
  normalized descriptor values.
- Reuse the existing capability reload lifecycle: raw request, prepared
  candidate, stale-active-snapshot guard, activation, sealed changed facts,
  common admitted runtime change evidence, and projection rebind.
- Reject unknown appearance or density IDs, duplicate edits, conflicting edits,
  invalid values, and stale prepared reloads before mutating active capability
  truth.
- Report touched delta width separately from full family rebuild breadth for
  both appearance and density.
- Treat equivalent appearance or density edits as equivalent no-ops with
  evidence rather than rebuilds. Equivalent edits must not change active
  snapshot digest, projection frame digest, or rebuild counters.
- Carry parse counters, canonicalization counters, descriptor lookup counters,
  touched descriptor counts, changed descriptor counts, and full family breadth
  in the family evidence row.

Target shape:

```rust
WorthUiCapabilityReloadRequest::from_appearance(package)
WorthUiCapabilityReloadRequest::from_density(package)

pub struct WorthUiAppearanceReloadPackage {
    source_path: WorthUiCapabilitySourcePath,
    source_text: String,
}

pub struct WorthUiCanonicalAppearanceReload {
    descriptors: Vec<WorthUiAppearanceTokenDescriptor>,
    canonical_digest: u64,
}
```

**Warnings**
- Do not add app-local style maps to bridge missing capability support.
- Do not accept raw appearance or density maps in projection rebind APIs.
- Do not partially commit one family row from a multi-family capability reload.
- Do not copy the theme reload code into a parallel one-off reload machine.
- Do not compare source text when typed canonical descriptor meaning is
  available.
- Do not let invalid units, unsupported shorthands, duplicate aliases, or
  unknown descriptor IDs progress into candidate snapshot construction.

**Test requirements**
- A mixed appearance+density reload activates as one candidate snapshot and
  emits separate family rows with exact touched counts and rebuild breadth.
- A stale prepared appearance reload cannot overwrite a newer active density or
  appearance snapshot.
- Duplicate or conflicting appearance edits fail before candidate snapshot
  construction and preserve active truth.
- `12px`, `12.0px`, and an equivalent canonical length form produce equivalent
  reload evidence and do not rebuild projections.
- `padding: 4px 8px` and its canonical four-edge representation converge to
  one digest, while changing any one edge produces an exact changed fact.
- A shadow edit with invalid color, blur, spread, offset, or unsupported unit is
  rejected at parse/admission with prior active snapshot preserved.
- Compile-fail coverage proves raw appearance packages, raw density packages,
  and raw value maps cannot enter the runtime change envelope or projection
  rebind coordinator.

**Engineering decisions**
- Appearance and density join the generalized capability family pipeline; they
  are not special renderer controls.
- Multi-family capability reloads remain atomic snapshot replacements.
- The proof chain must stay Law 41-shaped: a later phase accepts only the proof
  type the prior phase produced.
- Canonical descriptor comparison is the equivalence basis for appearance and
  density reload. Source-text equality is diagnostics, not truth.
- Every family row must expose enough counters to prove no hidden full-registry
  scan or broad projection rebuild was smuggled into a convenient API.

**Closeout requirement**
- Phase 12 is not complete until appearance and density edits enter the same
  raw request -> prepared candidate -> stale guard -> activated evidence ->
  admitted runtime change -> projection rebind chain as other capability
  families, with unknown, duplicate, conflicting, invalid, and stale edits
  denied before active capability truth mutates, equivalent canonical edits
  proven as no-ops, and counters proving descriptor/canonicalization breadth.

**Open questions**
- None.

### Phase 13: Component Capability Reload Family

This phase turns component declarations into hot-reloadable capability truth
instead of static registration-only descriptors.

**Relevant subsystems**
- component capability registry
- capability reload
- capability snapshot replacement
- runtime fact taxonomy
- projection dependency contracts
- component rendering contracts

**Relevant APIs**
- existing `ComponentDescriptor`
- existing `ComponentPropSchema`
- existing `ComponentChildPolicy`
- existing `ComponentStateOwnership`
- existing `ComponentExecutionLane`
- existing `CapabilitySnapshot`
- existing `WorthUiRuntimeFactId::component`
- new component reload package
- new component family changed-fact proof
- new component compatibility classifier
- new component state reconciliation receipt
- new component shape denial

**Build shape**
- Add a source-authored component reload package that can replace admitted
  component descriptors inside a candidate capability snapshot.
- Lower component descriptor edits into exact component facts. A dropdown
  component edit must not become a generic capability-changed fact.
- Preserve component state-ownership, child-policy, focus, accessibility, theme
  token dependency, command binding slot, and execution-lane distinctions during
  reload admission.
- Introduce registered dropdown and multi-select dropdown component descriptors
  as product-shaped component capabilities, not validation-app local widgets.
- Reject component edits that would violate state ownership, required prop
  schema, child policy, focus posture, or execution-lane compatibility before
  snapshot mutation.
- Classify component descriptor replacement before activation as compatible,
  state-preserving shape change, state-dropping shape change, or denied
  incompatible shape. The classification must be typed evidence, not an inline
  branch in the renderer or validation app.
- Define state reconciliation rules for component replacement. Runtime-owned
  state may be preserved only when state ownership, prop schema compatibility,
  child policy, focus semantics, accessibility semantics, and execution lane
  allow it. State must be explicitly dropped or the reload denied when those
  contracts diverge.
- Carry component compatibility classification into projection rebind so a
  projection can explain whether it preserved state, dropped state, rebuilt
  frame only, or denied the replacement.

Target shape:

```rust
WorthUiCapabilityReloadRequest::from_components(component_package)

pub enum WorthUiComponentCompatibility {
    Equivalent,
    CompatiblePreserveState(WorthUiComponentStatePreservation),
    CompatibleDropState(WorthUiComponentStateDropReason),
    Denied(WorthUiComponentShapeDenial),
}
```

**Warnings**
- Do not let the validation app maintain a local component registry.
- Do not treat single-select and multi-select dropdown behavior as a renderer
  boolean when it is component or command-projection meaning.
- Do not erase focus, accessibility, state ownership, or execution lane while
  making component reloads convenient.
- Do not let component reloads bypass capability snapshot digest checks.
- Do not preserve component state by default. Preservation is a proven outcome,
  not the absence of a denial.
- Do not let a prop schema name match stand in for schema compatibility unless
  the schema compatibility proof says it is enough.

**Test requirements**
- Changing a dropdown component descriptor emits the exact component fact and
  rebinds only projections that declare that component dependency.
- A component edit that changes incompatible child policy or state ownership is
  denied and preserves the active component descriptor.
- A compatible prop-schema extension preserves eligible runtime-owned component
  state and reports a state-preservation receipt.
- A compatible but state-breaking component shape change drops only the affected
  component state and reports the drop reason; unrelated component state remains
  preserved.
- A focus/accessibility/execution-lane incompatibility is denied before
  candidate snapshot activation.
- A mixed component+appearance reload preserves atomic activation and reports
  both family rows.
- Compile-fail coverage proves app code cannot construct component changed
  facts, admitted component reload rows, or a replacement component registry.

**Engineering decisions**
- Components are capability truth first and renderer mechanics second.
- Dropdown and multi-select dropdown support must exercise the same component
  family pipeline future components will use.
- Component behavior that affects projection meaning must be represented in
  declared dependencies.
- Component compatibility is a runtime-owned planning proof. It belongs beside
  component reload/admission, not inside renderer widget selection.
- Component state reconciliation must reuse existing durable/runtime state
  reconciliation principles where they apply; it must not invent a validation
  app state cache.

**Closeout requirement**
- Phase 13 is not complete until dropdown and multi-select dropdown components
  are registered as Worth UI component capabilities, component descriptor edits
  produce sealed component changed facts, incompatible component shape edits are
  denied, compatible edits produce explicit state preserve/drop receipts, and
  app code cannot maintain or substitute a local component registry.

**Open questions**
- None.

### Phase 14: Dropdown Projection Contract

This phase makes dropdown behavior a first-class projection contract so
single-select and multi-select changes hot reload through runtime evidence
instead of incidental egui branching.

**Relevant subsystems**
- command projection registry
- component capability reload
- appearance and density reload
- projection contract
- projection rebind coordinator
- header surface planning

**Relevant APIs**
- existing `CommandProjectionSelectionMode`
- existing `CommandProjectionDescriptor`
- existing `WorthUiHeaderMenuProjectionRequest`
- existing `WorthUiHeaderMenuGroup`
- existing `WorthUiProjectionPlanContract`
- existing `WorthUiProjectionDependencyDeclaration`
- existing `WorthUiProjectionRebindPlan`
- existing durable state reconciliation surfaces
- new dropdown projection plan
- new dropdown frame receipt
- new dropdown selection-state reconciliation receipt

**Build shape**
- Define a dropdown projection identity that declares dependencies on command
  projection identity, selection mode, component identity, appearance values,
  density values, and any command facts it consumes.
- Build distinct receipt shapes for single-select and multi-select rendering
  posture while keeping the projection contract shared where the lifecycle is
  genuinely shared.
- Ensure `single_select -> multi_select` changes flow through command
  projection or component changed facts and rebind only affected dropdown or
  header projections.
- Ensure dropdown appearance edits such as font size, row padding, container
  padding, color, and shadow flow through appearance or density facts.
- Keep renderer logic paint-only: it may choose egui button or checkbox painting
  from the receipt posture, but it cannot own selection-mode truth.
- Separate selection-mode projection meaning from selected-value runtime state.
  `single_select` versus `multi_select` is projection/capability meaning;
  current selected item or selected set is durable runtime state owned by the
  runtime reconciliation path.
- Define mode-transition reconciliation. `single_select -> multi_select`
  preserves the selected value as a one-item set when the value remains valid.
  `multi_select -> single_select` must either deterministically choose the
  canonical surviving value through a declared policy or deny/drop state with a
  typed reason. The renderer may not decide this.
- Include command availability and retired-command handling in dropdown state
  reconciliation. A selected command that disappears or becomes invalid must not
  remain selected through stale local state.

Target shape:

```rust
pub struct WorthUiDropdownProjectionPlan {
    identity: WorthUiProjectionIdentity,
    dependencies: WorthUiProjectionDependencyDeclaration,
    selection_mode: CommandProjectionSelectionMode,
}

pub enum WorthUiDropdownSelectionStateReconciliation {
    PreservedSingle,
    PreservedAsMultiSet,
    NarrowedToSingle(WorthUiDropdownNarrowingPolicy),
    Dropped(WorthUiDropdownStateDropReason),
    Denied(WorthUiDropdownModeTransitionDenial),
}
```

**Warnings**
- Do not store dropdown mode in validation-app state.
- Do not make dropdown mode a header-only fact if the same dropdown projection
  can appear outside the header.
- Do not rebuild all header menus when only one dropdown projection changes.
- Do not let renderer widget choice decide runtime meaning.
- Do not confuse selection mode with selected values. One is projection meaning;
  the other is runtime state with reconciliation rules.
- Do not silently choose a multi-select value when narrowing to single-select.

**Test requirements**
- A single-select to multi-select edit rebinds the dropdown projection and
  preserves unrelated header groups and page-host projections.
- A single-select to multi-select edit preserves the current selected value as a
  one-item selected set when that command still exists.
- A multi-select to single-select edit with multiple selected values either
  applies a declared deterministic narrowing policy or produces a typed
  denial/drop receipt; tests must fail silent first-item selection.
- Removing or disabling a selected command during a dropdown mode reload drops or
  denies that selected value through runtime state reconciliation, not renderer
  cleanup.
- A dropdown row-padding edit rebinds the same dropdown projection without
  touching command descriptor facts.
- A forged dropdown frame receipt cannot be constructed by app code.
- Compile-fail coverage proves raw command projection descriptors or raw
  selection-mode values cannot enter projection rebind as admitted dropdown
  evidence.

**Engineering decisions**
- Dropdown projection is a reusable platform projection, not a header-specific
  special case.
- Header menus may compose dropdown projections, but they do not own dropdown
  reload semantics.
- Selection mode is command-projection or component meaning; painting posture
  is derived.
- Selection state reconciliation is part of the runtime hot-reload contract. It
  must remain deterministic under replay and visible through receipts.

**Closeout requirement**
- Phase 14 is not complete until `single_select -> multi_select` is proven as a
  runtime-owned dropdown projection rebind through declared command projection,
  component, appearance, and density dependencies, with renderer code limited to
  painting the resulting receipt, selected-value state reconciled
  deterministically through runtime receipts, and compile-fail guards blocking
  raw mode or descriptor shortcuts.

**Open questions**
- None.

### Phase 15: Header Product Proof Slice

This phase performs the exact manual header hot-reload proof: registered header
and dropdown components, runtime-owned appearance and density, and visible
change propagation without restart.

**Relevant subsystems**
- native validation app
- validation runtime workbench
- header renderer
- component capability reload
- command projection reload
- appearance and density reload
- projection rebind coordinator

**Relevant APIs**
- existing `ValidationRuntimeWorkbench`
- existing `WorthUiHeaderFramePlan`
- existing `WorthUiHeaderFrameRebindReceipt`
- existing `WorthUiCapabilityReloadEvidence`
- existing `WorthUiProjectionRebindBatchReceipt`
- new dropdown projection plan and receipt
- new appearance and density frame receipts
- source-authored header/menu input file
- source-authored command projection reload input
- source-authored appearance reload input
- source-authored density reload input
- source-authored component reload input

**Build shape**
- Register the header component, dropdown component, and multi-select dropdown
  behavior through Worth UI capability descriptors.
- Render the header from runtime projection receipts only. The validation app
  may hold receipts and runtime handles; it may not own menu, style, dropdown,
  or component authority.
- Support visible hot reload of header text, color, font size, row padding,
  container padding, shadow, and `single_select -> multi_select` mode.
- Show adjacent runtime evidence sufficient to inspect changed facts, admitted
  capability rows, projection intersections, rebind status, and rebuild counts.
- Keep the native renderer boundary explicit and egui-only painting local to
  approved renderer files.
- Drive the manual proof from file-like source inputs that mirror real
  authoring. Header text and menu labels must come from source/command
  projection input, dropdown mode from command projection or component input,
  font size/color/shadow/padding from appearance or density input, and component
  descriptor changes from component input.
- The validation app may expose controls to load or swap those authored inputs,
  but the controls must submit the same source packages that file watching would
  submit. They must not mutate runtime internals, capability snapshots, local
  style maps, or projection receipts directly.
- Display the pre-edit and post-edit projection identities and frame digests so
  manual verification can see whether the runtime preserved, rebuilt, or denied
  the right projection.

Target shape:

```rust
struct ValidationHeaderHotReloadProof {
    header_frame: WorthUiHeaderFrameRebindReceipt,
    dropdown_frame: WorthUiDropdownFrameReceipt,
    evidence: WorthUiProjectionRebindBatchReceipt,
}
```

**Warnings**
- Do not hide hardcoded style values in the renderer to make the demo look
  acceptable.
- Do not build a web, mock, or design-prototype version of the proof.
- Do not let the validation app decide whether a reload succeeded.
- Do not accept visual survival without evidence receipts.
- Do not implement manual verification as direct Rust setters on workbench
  fields. Manual proof must travel through source-shaped reload packages.
- Do not hide the source family that caused the change. The app must show
  whether a visible update came from source, command projection, component,
  appearance, density, or mixed evidence.

**Test requirements**
- Editing the header source from single-select dropdown to multi-select dropdown
  visibly changes the native header without restart and produces runtime-owned
  evidence.
- Editing font size, row padding, container padding, shadow, and color updates
  the native header from runtime receipts without app-local style state.
- Each visible header edit path uses the same file-like source package path as
  the reload loop: source/header input for text, command projection input for
  dropdown mode when applicable, appearance/density input for style, and
  component input for dropdown descriptor changes.
- Direct mutation of validation workbench header, style, dropdown, or component
  fields is rejected by structural guards or absent from the API.
- Invalid appearance input preserves prior visible header truth and emits denial
  evidence.
- Structural guards prove validation-app files outside renderer boundaries do
  not import raw egui, runtime internals, local style maps, local component
  registries, or local dropdown mode state.

**Engineering decisions**
- This phase is a product proof, not a diagnostics panel.
- Manual verification must use the same runtime path as automated tests.
- A successful proof requires both visible change and receipt-backed truth.
- File-like inputs are allowed as a native validation-app affordance only when
  they preserve the same authority boundary as file watching. They are not a
  second imperative editing API.

**Closeout requirement**
- Phase 15 is not complete until the running native validation app visibly hot
  reloads header text, color, font size, row padding, container padding, shadow
  or elevation, and single-select to multi-select dropdown mode without restart,
  while the visible evidence proves changed facts, admitted evidence, projection
  intersections, rebind status, and rebuild counters came from source-shaped
  runtime reload inputs rather than direct workbench mutation.

**Open questions**
- None.

### Phase 16: Page Slot And Component Interaction Proof

This phase proves the hot-reload spine is not header-specific by combining page
slot edits with component and appearance dependencies.

**Relevant subsystems**
- page host planning
- content slot catalog
- component capability reload
- appearance and density reload
- projection rebind coordinator
- native validation app page surface

**Relevant APIs**
- existing `WorthUiPageHostPlan`
- existing `WorthUiPageHostRebindReceipt`
- existing `WorthUiContentSlotCatalog`
- existing `WorthUiRuntimeFactId::page_content_slot`
- existing `WorthUiRuntimeFactId::surface_mount`
- existing `WorthUiProjectionRebindBatchReceipt`
- new component and appearance dependency facts

**Build shape**
- Add a visible page region that consumes a source-authored page slot,
  component mount, and appearance or density dependency.
- Prove a page slot reassignment rebinds page-host or content projections while
  preserving unrelated header appearance and dropdown projections.
- Prove a header appearance or dropdown mode edit preserves unrelated page-host
  projections.
- Prove moving a component mount through source-authored content changes emits
  content or component facts as appropriate and does not require a broad
  artifact scan in steady frame.

Target shape:

```rust
struct ValidationPageSlotInteractionProof {
    page_host: WorthUiPageHostRebindReceipt,
    projection_batch: WorthUiProjectionRebindBatchReceipt,
}
```

**Warnings**
- Do not add a page-local map to remember slot assignments.
- Do not use header-specific projection logic to handle page content.
- Do not count a broad rebuild as acceptable just because the visual result is
  correct.
- Do not treat injected reload inputs, manual queue insertion, or pre-solved
  reload packets as app-lane proof. Validation-app reload tests must observe
  real file-backed inputs through the reload loop and real app polling.
- Do not rely on summary strings or mirrored counters where typed proof can be
  projected. A formatted summary is not reload authority.
- Do not fall back to broad artifact-level changed facts for page-host/source
  interactions. Page-host bounded invalidation must be driven by exact layout,
  slot, surface, content, and component facts.

**Test requirements**
- A page slot reassignment preserves header dropdown and appearance projections
  with rebind counters proving no header rebuild.
- A header appearance edit preserves page-host projection frames.
- A component mount movement changes only the content and component facts it
  touches.
- Compile-fail coverage proves raw content-slot declarations and raw component
  maps cannot enter page-host rebind APIs.
- Validation-app app-lane tests must drive reload through real observed files
  and reload polling rather than injected `ValidationReloadInput` shortcuts.
- Visible page-slot proof must expose typed dependency proof sufficient for
  tests to assert exact token, slot, surface, component, and changed-fact
  meaning without relying on formatted strings alone.

**Engineering decisions**
- Header and page-host projections must prove bounded invalidation against each
  other.
- Source-authored content remains the authority for page slot assignment.
- Component interaction proof belongs before mixed storm closeout so breadth
  certification has a real product surface to certify.
- Phase 16 must leave behind honest product-facing proof surfaces because Phase
  17 storm certification depends on those surfaces already proving real reload
  behavior rather than synthetic harness behavior.

**Closeout requirement**
- Phase 16 is not complete until a page slot reassignment, a component mount
  movement, and a header appearance/dropdown edit each prove bounded
  invalidation against the other surfaces, with receipts showing preserved
  projections remain preserved and rebuilt projections are limited to exact
  changed-fact intersections.
- Phase 16 is also not complete until the validation app proves the same
  behavior through real file-backed reload observation, and the proof surfaces
  expose typed meaning rather than summary-only evidence.

**Open questions**
- None.

### Phase 17: Mixed Reload Product Storm

This phase runs a product-shaped mixed reload storm that combines source,
capability, component, appearance, density, command projection, and denial
outcomes in one evidence path.

**Relevant subsystems**
- reload loop
- capability reload
- source ingress
- Query reload evidence where available
- projection rebind coordinator
- reload storm certification
- native validation app evidence surface

**Relevant APIs**
- existing `WorthUiCapabilityReloadRequest::batch`
- existing `WorthUiValidationReloadEvidence`
- existing `WorthUiCapabilityReloadEvidence`
- existing `WorthUiAdmittedRuntimeChangeEvidence`
- existing `WorthUiReloadReplayCertification`
- existing per-step `WorthUiProjectionRebindBatchReceipt`
- new `ValidationMixedReloadStormProof`
- new `ValidationMixedReloadStormReplayCertification`
- new appearance, density, and component reload rows

**Build shape**
- Build one mixed reload scenario containing a header text change, dropdown mode
  change, appearance or density change, page slot assignment change, one denied
  family row, and one equivalent row.
- Preserve visible mixed status. The evidence must not collapse the scenario
  into a single success or failure label.
- Certify active truth after the storm: valid admitted changes apply,
  equivalent rows avoid churn, denied rows preserve prior truth, and projection
  frames converge under replay.
- Surface counters proving rebuild breadth follows changed-fact/dependency
  intersections rather than total registered projection count.

Target shape:

```rust
pub struct ValidationMixedReloadStormProof {
    posture: ValidationMixedReloadStormPosture,
    steps: Vec<ValidationMixedReloadStormStep>,
    projection_counters: ValidationMixedReloadStormProjectionCounters,
    final_active_artifact_digest: u64,
    final_active_plan_digest: u64,
    final_capability_snapshot_digest: u64,
    final_authoring_snapshot_digest: Option<u64>,
    final_last_valid_artifact_digest: u64,
    final_last_valid_plan_digest: u64,
}
```

- The mixed-storm proof must preserve real per-step header/page-host projection
  receipts instead of forging one cross-storm `WorthUiProjectionRebindBatchReceipt`.
  Distinct admitted reload rows carry distinct change-evidence digests, so a
  single aggregated projection batch across the whole storm would be
  architecture-dishonest.

**Warnings**
- Do not allow partial capability snapshot commits unless the family model has
  explicitly proven a partial-commit boundary.
- Do not hide denied rows inside a successful mixed reload summary.
- Do not treat equivalent edits as rebuilds.
- Do not let a mixed storm reintroduce app-local state reconciliation.

**Test requirements**
- A mixed valid+equivalent+denied reload preserves per-family status and
  produces top-level mixed evidence.
- Replaying the same mixed storm converges to the same active artifact,
  capability snapshot, authoring snapshot, and projection frame digests.
- Projection rebuild count equals changed dependency intersection across
  header, dropdown, page-host, component, appearance, and density projections.
- Invalid family rows preserve prior visible truth while admitted family rows
  still update through the runtime path allowed by the family model.

**Engineering decisions**
- Product storm certification must exercise real product projections, not only
  synthetic proof objects.
- Mixed evidence is a first-class posture, not a logging detail.
- Equivalent, denied, stale, and valid outcomes remain distinct because
  projection rebind depends on the distinction.

**Closeout requirement**
- Phase 17 is not complete until one product-shaped mixed storm containing valid,
  equivalent, stale or denied, source, capability, appearance, density,
  component, command-projection, and page-slot edits replays deterministically,
  keeps per-family status visible, preserves prior truth on denied rows, and
  certifies bounded rebuild breadth across real product projections.

**Open questions**
- None.

### Phase 18: Appearance And Component Compiler Enforcement Sweep

This phase extends Law 41 compile-time enforcement to every new appearance,
density, dropdown, and component proof surface added after the original reload
spine.

**Relevant subsystems**
- runtime authority compile tests
- validation app native boundary tests
- facade import tests
- appearance reload
- density reload
- component reload
- dropdown projection

**Relevant APIs**
- existing `runtime_reload_authority_compile` suite
- existing validation-app native boundary guards
- new appearance changed-fact proof types
- new density changed-fact proof types
- new component changed-fact proof types
- new dropdown projection receipts
- new appearance and component admitted reload rows

**Build shape**
- Add compile-fail fixtures beside each new proof-bearing type. Field privacy,
  constructor privacy, skipped-stage rejection, and weaker-input rejection must
  all be represented where Rust can encode them.
- Treat Phase 18 as a certification sweep, not a place to first add ordinary
  enforcement. Phases 11-17 must add local compile-fail coverage beside each
  proof type when it is introduced; Phase 18 verifies the matrix is complete and
  fills only true gaps.
- Add validation-app structural guards that reject local style maps, local
  component registries, local dropdown mode state, raw egui outside renderer
  files, runtime internal imports, and direct Query dependency.
- Add facade pass fixtures proving only intended public facade types are
  importable by app code.
- Ensure each new rebind-facing API accepts admitted proof-bearing inputs, not
  raw declarations, raw requests, raw fact sets, raw dependency sets, or
  classified-but-unadmitted changes.
- Add a coverage matrix mapping every new proof-bearing type to: public facade
  import posture, field privacy fixture, constructor privacy fixture,
  skipped-stage fixture, weaker-input fixture, validation-app misuse fixture
  where applicable, and the runtime API that legitimately constructs it.

**Warnings**
- Do not rely on documentation to protect proof transitions.
- Do not combine unrelated compile-fail cases into one broad fixture that hides
  which proof boundary failed.
- Do not expose internal constructors to make validation-app tests easier.
- Do not leave new proof surfaces without matching weaker-input rejection tests.
- Do not let this phase become a dumping ground for missing enforcement that
  should have been added when the proof type was introduced.
- Do not accept broad "cannot use internals" fixtures when the misuse can be
  expressed as a precise skipped-stage or weaker-input compile-fail case.

**Test requirements**
- Compile-fail coverage proves app code cannot construct appearance, density,
  or component changed facts or admitted reload evidence.
- Compile-fail coverage proves raw appearance, density, component, command
  projection, and dropdown declarations cannot enter projection rebind APIs.
- Compile-fail coverage proves app code cannot construct dropdown, header, page
  host, appearance, density, or component projection receipts.
- Structural guards prove validation-app code cannot own local style maps,
  dropdown mode state, component registries, menu authority, page maps, shell
  maps, or theme state.
- The enforcement matrix has no blank cells for proof-bearing types introduced
  in Phases 11-17 unless a blank cell is paired with a named reason the compiler
  cannot encode that specific boundary and a structural guard covers it.

**Engineering decisions**
- Compiler enforcement is part of the feature, not a cleanup pass.
- Each proof family must have a local fixture that explains what weaker input is
  rejected.
- App-facing facade importability and internal constructor denial are paired
  proof obligations.
- The matrix is the closeout artifact for this phase. It prevents us from
  claiming enforcement in prose while leaving one proof transition unguarded.

**Closeout requirement**
- Phase 18 is not complete until every appearance, density, component, dropdown,
  header, and page-host proof-bearing type introduced by Phases 11-17 has
  compile-fail coverage for external construction and skipped-stage use, and
  validation-app structural guards reject local style, component, dropdown,
  menu, page, shell, theme, runtime, egui, and Query authority leaks, with a
  complete enforcement matrix documenting each proof transition.

**Open questions**
- None.

### Phase 19: Native Manual Verification App Closeout

This phase makes the native validation app a real end-to-end hot-reload product
proof with manual verification surfaces for every mandatory reload family.

**Relevant subsystems**
- native validation app
- validation runtime workbench
- evidence projection
- reload storm certification
- appearance and density rendering
- component and dropdown rendering
- page-host rendering

**Relevant APIs**
- existing `ValidationRuntimeWorkbench`
- existing `WorthUiRuntimeHost`
- existing `WorthUiProjectionRebindBatchReceipt`
- existing `WorthUiReloadReplayCertification`
- existing `WorthUiHotReloadVisualCaptureReceipt`
- new header/dropdown/product proof receipts
- new appearance, density, and component evidence rows
- new manual verification flow matrix
- native visual evidence projection rows for changed facts, rebuilt projections,
  preserved projections, denial posture, replay posture, and counter posture

**Build shape**
- Provide a native egui app surface for applying file-like reload inputs to
  source, command projection, appearance, density, component, and mixed storm
  scenarios.
- Display changed facts, admitted evidence rows, projection rebind rows, rebuild
  and preserve counters, denial reasons, and replay certification status.
- Use a tasteful VS Code-like dark theme expressed through Worth UI runtime
  appearance values, not app-local egui constants.
- Include manual flows for the exact header/dropdown/style/page-slot acceptance
  proof: text, color, font size, row padding, container padding, shadow,
  single-select to multi-select mode, and page slot reassignment.
- Keep the app public-facade only. The app validates that external consumers can
  use Worth UI honestly.
- Add a manual verification flow matrix in the app and in tests. Each row must
  name the authored input, expected changed facts, expected projections rebuilt,
  expected projections preserved, expected visible result, expected denial or
  equivalent posture where applicable, replay expectation, and counter
  expectation.
- Include one row for each mandatory surface: header text, header color, header
  font size, dropdown row padding, dropdown container padding, dropdown shadow,
  single-select to multi-select mode, multi-select to single-select state
  reconciliation, component descriptor change, page slot reassignment, invalid
  appearance denial, equivalent canonical appearance edit, and mixed product
  storm.
- The app must show enough before/after evidence that a human can confirm a
  preserved projection remained preserved and a rebuilt projection rebuilt for
  the declared reason. A screenshot alone is never closeout evidence.

Target shape:

```rust
struct ValidationHotReloadProductApp {
    workbench: ValidationRuntimeWorkbench,
    visible_evidence: ValidationRuntimeEvidenceProjection,
}

struct ValidationManualReloadFlowExpectation {
    authored_input: ValidationAuthoredReloadInputKind,
    expected_changed_facts: WorthUiRuntimeFactSet,
    expected_rebuilt_projections: Vec<WorthUiProjectionIdentity>,
    expected_preserved_projections: Vec<WorthUiProjectionIdentity>,
    expected_visible_result: ValidationVisibleReloadExpectation,
    expected_counter_posture: ValidationCounterExpectation,
}
```

**Warnings**
- Do not call this a harness if it behaves like a product validation app.
- Do not make manual verification depend on developer memory or terminal logs.
- Do not hide acceptance evidence in tests only; the running native app must
  show it.
- Do not add web, DOM, CSS, browser, or webview tooling to prove a native egui
  platform capability.
- Do not make the final app an unstructured playground. It must be a product
  proof with named flows and expected evidence.
- Do not let manual verification rely on "it looked different" without showing
  facts, projection rows, counters, and replay/certification posture.

**Test requirements**
- The native app can manually demonstrate text, color, font size, padding,
  shadow, dropdown mode, component, and page slot hot reload without restart.
- The visible evidence panel matches the same receipts and counters asserted by
  automated tests.
- Invalid, stale, equivalent, valid, and mixed reloads are all visible and
  preserve or update runtime truth according to their typed evidence.
- Structural guards prove the app consumes facade APIs only and renderer files
  remain paint-only.
- Every manual flow matrix row has an automated counterpart asserting changed
  facts, rebuilt projections, preserved projections, visible projection digest,
  denial/equivalence posture, replay posture, and counters.
- The app exposes a VS Code-like dark theme through runtime appearance/density
  values and proves changing that theme uses the same appearance/density reload
  path as the header proof.

**Engineering decisions**
- The validation app is an end-to-end product proof, not a separate testing
  runtime.
- Manual evidence is supplementary to typed receipts, but it must display the
  same truth the receipts certify.
- Milestone 4S is not complete until this app can demonstrate every mandatory
  hot-reload surface.
- The manual verification matrix is the final product-grade acceptance artifact
  for this side quest. It makes the remaining bar inspectable without relying on
  thread memory or developer narration.

**Closeout requirement**
- Phase 19 is not complete until a human can launch the native egui validation
  app and manually verify every mandatory hot-reload surface in this milestone
  from visible controls and evidence panels, while automated tests assert the
  same receipts, counters, denials, preserved truth, rebind rows, and replay
  certifications shown in the app, row by row through the manual verification
  flow matrix.

**Open questions**
- None.

### Phase 20: Semantic Slice Inventory And Authority Boundary

This phase catches Worth UI up to the aspect model before we harden the rest of
the hot-reload pipeline. The main deliverable is not code motion by itself. The
deliverable is an explicit semantic inventory of what kinds of authored meaning
can change, which runtime facts should represent those changes, which
projections consume those facts, and whether each slice belongs to product
meaning or platform meaning.

The reminder from `forge-signal` is simple and important:
- dependencies answer who cares about whom
- aspects answer what part changed
- conditions and comparators answer different questions entirely

For this milestone, that means broad labels like "header reload", "component
reload", or "source reload" are operational breadcrumbs, not semantic truth.
Semantic truth must be expressed in slices that projections can depend on
independently. Examples include theme token value, appearance field, density
field, layout padding, shell-slot assignment, content-slot assignment, mount
target, component-selection mode, dropdown selection mode, and authored
Query-binding shape.

The reminder from `forge-relational` is stricter: once canonical semantic truth
exists, later systems must not reconstruct it through a second interpretation
path. This phase therefore requires an authority map that names where each
semantic slice is declared and which layer is allowed to own it.

It also needs one explicit Query reminder so we do not rediscover it later.
Where Query is already the authority, Worth UI should preserve Query-owned
semantic slices rather than inventing parallel local ones. The baseline slices
to preserve are:
- live-view binding identity and binding preservation/rebind/retirement posture
- async/result posture
- projection fact changes
- state snapshot changes
- recovery posture
- inspection posture
- virtualized data frame target changes

Those are not all authored structure, but they are canonical semantic slices
that the Worth UI touched graph should consume honestly where Query owns them.

**Requirements**
- Produce an explicit semantic slice inventory for Milestone 4 hot reload and
  make it normative inside this spec rather than tribal knowledge.
- Classify every slice as product meaning or platform meaning.
- Map each slice to its owning authority layer: authored source, capability
  authority, Query authority, runtime-owned interaction state, or another named
  runtime authority already present in Worth UI.
- Identify which slices already have exact runtime fact families and which still
  collapse into broad operational families.
- Name the Query-owned slices Worth UI must preserve verbatim rather than
  re-model locally.

**Mechanical model**
- Start from visible hot-reload targets and runtime-owned surfaces.
- Decompose them into semantic change slices rather than file groups or
  surface groups.
- Assign each slice one owner and one intended runtime fact family or family
  gap.
- Record the compile boundary explicitly: new Rust component implementations,
  new capability families, and new runtime subsystem behavior remain platform
  meaning and are therefore outside hot reload.

**Warnings**
- Do not let the inventory devolve into UI-screen taxonomy.
- Do not classify one broad "component" or "header" slice when multiple
  responsibilities can change independently.
- Do not duplicate Query posture with Worth UI local status slices where Query
  is already canonical.

**Test requirements**
- Spec and code references agree on the named semantic slices used in current
  hot-reload fact lowering.
- Guardrails or compile-fail coverage prove platform-meaning slices still do not
  enter authored hot-reload lanes.
- A focused audit test or assertion matrix proves the documented Query-owned
  slices still map to existing Query reload families without local duplication.

**Engineering decisions**
- This phase is the semantic contract pass that later hardening depends on.
- Query-owned slices should be preserved as upstream semantic truth, not
  rewritten into new Worth UI folklore.

**Open questions**
- Whether the semantic slice inventory should live only in this milestone spec
  or also become a near-code reference table once the implementation settles.

### Phase 21: Canonical Authored Delta Lowering

This phase introduces the first hard guardrail we were missing: authored
changes must lower once into canonical semantic delta before the rest of the
runtime decides what to preserve, rebind, or rebuild. That is the hot-reload
version of the `forge-relational` rule against a second aspect-set construction
path.

The goal is not merely "better source ingress." The goal is that there is one
authoritative authored-delta lane from source-package observation to touched
semantic declarations. No later stage gets to reopen raw source files and
reconstruct its own change meaning. If any rebind planner, renderer helper, or
surface-specific adapter still needs raw external authored documents after this
phase output exists, that is architectural debt and should be treated as a
regression.

Mechanically, this phase introduces a proof-bearing authored-delta summary at
the source-package boundary. That summary is not a bag of changed paths and it
is not yet a generic runtime fact set. It is canonical delta truth over touched
authored declarations and their semantic slice rows. Transitional loaders may
still exist while we migrate, but they must funnel through this canonical
lowering path.

**Requirements**
- Add one canonical source-ingress phase output that represents authored delta
  truth after observed source-package edits are loaded.
- Preserve explicit proof progression from raw watcher observation to authored
  delta summary; later phases must not accept raw paths or raw file contents in
  place of the authored-delta proof type.
- Ensure later consumers do not rescan authored files or local artifact text to
  rediscover touched semantic slices once authored-delta proof exists.
- Expose counters proving authored-ingress breadth: observed modules, parsed
  modules, authored declarations inspected, authored declarations marked
  touched, and semantic slices emitted.

**Mechanical model**
- Watchers or manual edits submit observed source-module changes.
- The runtime compiles a candidate source package at the existing source
  boundary.
- Canonical authored declarations are compared against the active authored
  boundary.
- The runtime emits a typed touched-authored proof with inspectable semantic
  slice rows rather than per-family string tags.
- Later phases consume that canonical authored-delta proof to derive changed
  facts and runtime evidence.

**Warnings**
- Do not treat file names as architectural reload families.
- Do not let source ingress become a second runtime replacement lane beside the
  existing source -> artifact -> runtime proof chain.
- Do not permit callers to select later reload families before authored delta
  has been lowered.
- Do not widen every source edit into full package rebuild proof just because
  the ingress boundary is the whole source package.

**Test requirements**
- A touched source-module observation lowers into typed authored-delta proof
  before any family-specific rebind lane runs.
- Compile-fail or structural-guard coverage proves raw file observations cannot
  enter APIs that require authored-delta proof.
- Counters prove the ingress path reports touched authored breadth and emitted
  semantic-slice breadth rather than only raw watcher breadth.
- Replay of the same observed source edit produces the same authored-delta
  summary digest.

**Engineering decisions**
- Source ingress remains file-backed operationally, but the architectural
  boundary is canonical authored semantic delta, not file-family dispatch.
- Transitional family-specific reload fixtures are acceptable only if they feed
  the canonical authored-ingress path and do not become a second authority lane.

**Open questions**
- Whether the initial authored-delta proof type should be per-module plus
  aggregate summary or only aggregate summary with inspectable member rows.

### Phase 22: Touched-Graph Propagation Over Semantic Facts

This phase makes the graph own invalidation breadth at the semantic resolution
defined in Phase 20 and lowered in Phase 21. An authored edit is a mutation
against authored truth. That mutation must declare exactly what it invalidates.
A projection or visible runtime surface must declare exactly what it consumes.
The framework computes their intersection. That is the core aspect model we
want Worth UI to actually obey before we move into broader hardening.

Mechanically, this phase widens the runtime fact taxonomy where needed so
semantic authored responsibilities become exact changed-fact families rather
than broad source-reload placeholders. The precise taxonomy may evolve, but the
rule is fixed: each fact family must correspond to one semantic responsibility
that projections can depend on independently. For authored structure, likely
families include shell-slot assignment, page-host page declaration, layout
topology region or padding selection, content-slot to surface assignment,
surface-to-component mount choice, command projection component choice, and
authored Query-binding shape where that binding is authored product meaning.

This phase also includes the Query reminder we should not lose: when a touched
slice comes from Query authority, the Worth UI graph should preserve the exact
Query-owned slice rows already available in Query reload evidence. Do not
replace them with a coarser Worth UI-local umbrella like "query changed" if the
existing Query proof already distinguishes live binding, result posture,
projection facts, state snapshots, recovery, inspection, and virtualized frame
targets.

**Requirements**
- Add or refine runtime fact families so Milestone 4 semantic slices can change
  independently at reload time.
- Lower authored-delta proof into exact changed-fact proof rather than broad
  source-reload placeholders.
- Keep capability-authority fact families separate from authored-structure fact
  families even when both are touched by the same save.
- Preserve Query-owned slice granularity when lowering Query evidence into Worth
  UI changed facts.
- Ensure changed-fact breadth matches semantic delta and can be inspected row by
  row.

**Mechanical model**
- The authored-delta summary identifies touched authored declarations and slice
  rows.
- Lowering maps those declarations and rows to semantic runtime fact ids.
- Each changed fact id is grouped into a family whose semantics match one
  responsibility.
- Mixed saves may emit multiple changed-fact families in one admitted runtime
  change envelope.
- Later rebind planning uses family rows plus changed-fact intersection, not
  source-file category dispatch or surface-local heuristics.

**Warnings**
- Do not use "source changed" as a substitute for semantic fact specificity
  when narrower authored or Query truth is available.
- Do not collapse authored structure and compiled capability authority into one
  fact family.
- Do not let projections depend on string paths or parser-local tokens when they
  should depend on semantic fact ids.
- Do not replace distinct Query slices with one coarse "Query reload" family
  where existing proof already supports narrower lowering.

**Test requirements**
- A structural authored edit such as repointing a content slot, changing layout
  padding, or changing a mount target produces exact semantic changed facts.
- Equivalent authored edits preserve zero changed facts where semantics do not
  change.
- Mixed authored saves produce multiple fact families while preserving row-level
  status and changed-fact counts.
- Query-bound reload tests prove live-binding, result-posture, projection-fact,
  state-snapshot, recovery, inspection, and virtualized-target slices preserve
  exact lowering where those slices are already available upstream.
- Projection breadth counters prove semantic fact intersection narrows rebuilds
  compared with broad source-reload fallback.

**Engineering decisions**
- Runtime facts remain the shared dependency vocabulary; changed-fact wrappers
  remain proof.
- Fact-family granularity should follow semantic responsibilities, not parser
  convenience, incidental file organization, or UI-screen boundaries.

**Open questions**
- Whether shell-slot assignment and surface-mount choice deserve separate fact
  families or a shared authored-mount family with typed sub-identities.

### Phase 23: Authoring Delta Ingress Unification

This phase resets the hot-reload entry boundary around authored truth rather
than file-family folklore. The important architectural change is not "watch
more files." It is that the runtime must treat an authored source-package edit
as the ordinary ingress boundary and derive the typed touched scope from that
edit once, up front, before any later phase decides what reload family,
projection, or visible surface might care.

Today the side quest still carries transitional ingress surfaces such as theme,
command, component, appearance, and density reload files because they were the
fastest path to proving the hot-reload spine. That transitional split is not
the end state. The end state is one authored-source ingress path that can
answer: which authored modules changed, which canonical authored declarations
changed, which lower artifact structures changed, and which typed runtime fact
families those authored deltas touch. Family-specific loaders may still exist
as adapters or fixtures, but they must lower through the same source-ingress
truth rather than defining independent reload authority.

Mechanically, this phase introduces a typed authored-delta summary at the
source-package boundary. That summary is not a bag of changed file paths and it
is not a plain fact set. It is proof-bearing phase output that records the
canonical authored units touched by the edit and the candidate lower structures
those units affect. Later phases consume that authored-delta summary exactly as
they currently consume admitted runtime change evidence: as a stronger type than
raw observations. The runtime must no longer require callers to decide "this is
a theme reload" or "this is a component reload" before lowering. The authored
delta shape itself must determine which later lanes may apply.

The key performance rule here is that ingress breadth must still be bounded by
semantic delta. "All source edits go through one source-package boundary" does
not mean "all source edits widen to full package invalidation." The source
package boundary is where the system learns touched authored meaning once; it is
not permission to replace narrow impact with global rebuild.

**Requirements**
- Add one canonical source-ingress phase output that represents authored delta
  truth after observed source-package edits are loaded.
- Preserve explicit proof progression from raw watcher observation to authored
  delta summary; later phases must not accept raw paths or raw file contents in
  place of the authored-delta proof type.
- Allow transitional family-specific reload fixtures only if they lower into the
  same authored-ingress path or are clearly marked as temporary proof adapters.
- Expose counters proving authored-ingress breadth: observed modules, parsed
  modules, authored declarations inspected, and authored declarations marked
  touched.

**Mechanical model**
- Watchers or manual edits submit observed source-module changes.
- The runtime compiles a candidate source package at the existing source
  boundary.
- Canonical authored declarations are compared against the active authored
  boundary.
- The runtime emits `TouchedAuthoredDeclarationSet`-style proof rather than
  per-family string tags.
- Later phases consume that touched authored set to derive typed changed facts.

**Warnings**
- Do not treat file names as architectural reload families.
- Do not let source ingress become a second runtime replacement lane beside the
  existing source -> artifact -> runtime proof chain.
- Do not permit callers to select later reload families before the authored
  delta has been lowered.
- Do not widen every source edit into full package rebuild proof just because
  the ingress boundary is the whole source package.

**Test requirements**
- A touched source-module observation lowers into a typed authored-delta summary
  before any family-specific rebind lane runs.
- Compile-fail or structural-guard coverage proves raw file observations cannot
  enter APIs that require authored-delta proof.
- Counters prove the ingress path reports touched authored breadth rather than
  only raw watcher breadth.
- Replay of the same observed source edit produces the same authored-delta
  summary digest.

**Engineering decisions**
- Source ingress remains file-backed operationally, but the architectural
  boundary is authored semantic delta, not file-family dispatch.
- Transitional family-specific reload fixtures are acceptable only if they feed
  the canonical authored-ingress path and do not become a second authority lane.

**Open questions**
- Whether the initial authored-delta proof type should be per-module plus
  aggregate summary or only aggregate summary with inspectable member rows.

### Phase 24: Authored Structural Fact Lowering

This phase turns authored delta into graph-owned invalidation truth. The goal is
not just "support more changed facts." The goal is that authored structure such
as shell topology, page topology, layout slots, surface mounts, component
selection, Query/view bindings, and other Milestone 4 authoring targets lower
into explicit runtime fact families with the same rigor already applied to
theme, appearance, density, source content slots, and dropdown interaction
state.

The important architectural rule is read/write duality. An authored edit is a
mutation against authored truth. That mutation must declare exactly what it
invalidates. A projection or visible runtime surface must declare exactly what
it consumes. The framework computes their intersection. This phase therefore
adds new authored-structural fact families where the current system still
collapses meaning into lower-level source replacement or code-owned wiring.

Mechanically, this phase widens the runtime fact taxonomy with authored
structure-specific families. Examples include shell-slot assignment, page-host
page declaration, layout topology region selection, content-slot to surface
assignment, surface-to-component mount choice, command projection component
choice, and Query/view binding shape where those are authored facts rather than
compiled capability authority. The precise taxonomy may evolve, but the rule is
fixed: each fact family must correspond to one semantic authored responsibility
that projections can depend on independently.

The lowering path for these facts must remain proof-bearing. Raw parsed
structures, raw artifact-input fragments, or raw mount maps are not changed-fact
proof. This phase must produce sealed authored-structural changed-fact proof
that later runtime change evidence can carry without re-derivation.

**Requirements**
- Add authored-structural runtime fact families for the Milestone 4 authoring
  responsibilities that can change independently at reload time.
- Lower authored-delta proof into exact authored-structural changed-fact proof
  rather than broad source-reload placeholders.
- Keep capability-authority fact families separate from authored-structure fact
  families even when both are touched by the same save.
- Ensure changed-fact breadth matches semantic authored delta and can be
  inspected row by row.

**Mechanical model**
- The authored-delta summary identifies touched authored declarations.
- Lowering maps those declarations to structural runtime fact ids.
- Each changed fact id is grouped into a family whose semantics match one
  authoring responsibility.
- Mixed saves may emit multiple changed-fact families in one admitted runtime
  change envelope.
- Later rebind planning uses family rows plus changed-fact intersection, not
  source-file category dispatch.

**Warnings**
- Do not use "source changed" as a substitute for authored-structural fact
  specificity when narrower authored truth is available.
- Do not collapse authored structure and compiled capability authority into one
  fact family.
- Do not let projections depend on string paths or parser-local tokens when they
  should depend on semantic authored fact ids.

**Test requirements**
- A structural authored edit such as repointing a content slot or changing a
  mount target produces exact authored-structural changed facts.
- Equivalent authored edits preserve zero changed facts where semantics do not
  change.
- Mixed authored saves produce multiple fact families while preserving row-level
  status and changed-fact counts.
- Projection breadth counters prove authored-structural fact intersection
  narrows rebuilds compared with broad source-reload fallback.

**Engineering decisions**
- Runtime facts remain the shared dependency vocabulary; authored-structural
  changed-fact wrappers remain proof.
- Fact-family granularity should follow semantic authoring responsibilities, not
  parser convenience or incidental file organization.

**Open questions**
- Whether shell-slot assignment and surface-mount choice deserve separate fact
  families or a shared authored-mount family with typed sub-identities.

### Phase 25: Move Code-Owned Structural Wiring Into Authored Truth

This phase removes the remaining app-code-owned structural choices that block
honest Milestone 4 authoring reload. The specific problem is not components in
the abstract. The problem is any live structural choice that is currently fixed
in Rust even though it belongs to the Milestone 4 authoring hierarchy: which
surface a slot mounts, which component a live authored mount selects, which
projection uses which runtime-owned component kind, and similar structure that a
developer should be able to express in authored source without recompiling the
app.

The hot-reload target here is precise. Changing the compiled behavior of a brand
new Rust component implementation still requires a compile because that changes
platform meaning. But selecting, mounting, or rebinding already-registered
capabilities inside authored structure is product meaning and should therefore
be hot reloadable. This phase must encode that distinction directly in the spec
instead of relying on memory.

Mechanically, authored source must become the authority for live structural
bindings that are currently chosen in code. The runtime should no longer ask
validation-app Rust or header-local helpers which component id or mount target a
live authored structure uses when that choice can be expressed in authored
truth. Once that authored truth exists, the authored structural fact families
from Phase 24 become the dependency basis and generic rebind machinery from
earlier phases can do the rest.

**Requirements**
- Audit live structural choices still hardcoded in app code and classify each as
  either compiled capability authority or authored product structure.
- Move authored product-structure choices into authored truth and lower them
  through the existing source -> artifact -> runtime proof chain.
- Preserve the hard boundary that new Rust component implementations, new
  capability families, and new runtime subsystem behavior still require compile.
- Ensure existing runtime projections and visible surfaces consume the authored
  structural truth rather than local code-owned maps.

**Mechanical model**
- Already-registered components remain capability authority.
- Authored source chooses where those components are mounted or rebound.
- A source edit that changes mount/component choice produces authored-structural
  changed facts.
- Rebind paths intersect those facts against declared dependencies and rebuild
  only affected projections or surfaces.
- No additional watcher registration is required because the source-package
  ingress already owns authored change discovery.

**Warnings**
- Do not "solve" this by adding more validation-app-only config seams that sit
  beside the Milestone 4 authoring model.
- Do not let code-owned helpers remain hidden structural authority for mounts,
  shell slot selection, or projection component choice when those are authored
  responsibilities.
- Do not weaken the compiled-authority boundary for genuinely new Rust-rendered
  behavior.

**Test requirements**
- A running app can repoint an existing live authored mount or projection to a
  different already-registered component through authored source only.
- No Rust edit is required for that repointing proof.
- Evidence shows authored-structural changed facts, affected projections, and
  preserved unaffected projections.
- Structural-guard coverage proves code-owned renderer or app glue cannot
  synthesize the resulting authored truth locally.

**Engineering decisions**
- This phase is where Milestone 4 authoring and Milestone 4S hot reload become
  one story again: authored structure lowers once, runtime consumes it once.
- Compile boundaries remain declarations of platform meaning; hot reload
  boundaries remain declarations of product meaning.

**Open questions**
- Which existing validation-app structural choices should move first to maximize
  proof value with minimum transitional churn.

### Phase 26: Delta-Driven Rebind Phase Selection

This phase makes orchestration structural. Once the runtime can ingest authored
delta and lower authored-structural changed facts, it must stop relying on
surface-local branching or caller-selected reload modes to decide what rebind
work runs. The mutation shape must determine phase execution.

The mechanical target is a coordinator that consumes admitted runtime change
evidence whose family rows may now include source, capability, Query,
interaction, and authored-structural changes. The coordinator then determines
which rebind lanes are eligible by intersecting changed-fact proof against
declared projection dependencies and by honoring any phase-specific admissibility
proof already established earlier in the pipeline. This is the moment where the
touched graph truly owns invalidation breadth.

The coordinator must remain explicit about two separate things:
- what changed
- what consumed that changed meaning

That means later paths must not hide family-specific heuristics such as "a
component reload probably means header rebuild" or "a source edit means page
host and header both rerun." Those shortcuts are architectural regressions. The
only honest reason to rebuild is that a projection or visible surface declared a
dependency that intersects admitted changed-fact proof.

**Requirements**
- Add a common coordinator path that maps admitted runtime change evidence to
  eligible rebind phases through dependency intersection and phase-specific
  admissibility.
- Make skipped phases explicit and measurable.
- Preserve common handling of denied, equivalent, activated, and mixed family
  rows without surface-local divergence.
- Expose counters for inspected projections, phase rows considered, dependency
  intersections, skipped phases, rebuild attempts, preserved projections, and
  rebuilt projections.

**Mechanical model**
- One save may emit multiple family rows.
- Each rebind lane asks the coordinator for admitted rebuild planning based on
  its dependency contract.
- If no dependency intersects changed-fact proof, the lane is preserved.
- If dependency intersects and the lane's rebuild proof is admitted, the lane
  rebuilds.
- Equivalent and denied rows may still flow through the same coordinator, but
  they preserve unless another activated family row intersects the dependency
  contract.

**Warnings**
- Do not reintroduce header-specific, page-specific, or product-specific
  rebuild shortcuts once authored-structural facts exist.
- Do not allow the executor or renderer to rediscover changed scope that the
  coordinator should already know.
- Do not let source-edit breadth silently widen because the coordinator lacks
  authored-structural fact granularity.

**Test requirements**
- Appearance-only, density-only, component-only, authored-mount-only, and mixed
  deltas each prove that untouched lanes are skipped.
- A mixed save that touches two unrelated authoring responsibilities rebuilds
  only the projections consuming those responsibilities.
- Counters prove skipped phase breadth and rebuild breadth.
- Replay shows identical phase-selection outcomes for the same admitted runtime
  change evidence.

**Engineering decisions**
- Later visible surfaces may still expose family-specific receipts, but phase
  selection itself belongs to the common runtime-owned coordinator.
- Rebind planning remains a proof-widening pipeline: admitted change evidence
  plus admitted dependency contract plus admitted projection plan plus admitted
  rebuild or preserve outcome.

**Open questions**
- Whether the phase-selection coordinator should expose one public aggregate
  receipt or only family/lane-specific receipts plus shared counters.

### Phase 27: Runtime-Owned Structural Hot Reload Product Proof

This phase broadens the validation app from "value reload proof" into "authored
structure reload proof." The app must visibly demonstrate that authored product
structure can change through the runtime-owned hot-reload spine without local
glue pretending to be framework semantics.

The proof target is deliberately narrower than arbitrary custom Rust component
implementation. A brand-new compiled component implementation still requires a
compile because that changes capability authority. But once a component is
registered, the app should be able to mount, remount, or repoint it through
authored source only if that structural choice belongs to product meaning. The
validation app must make that visible through the same evidence surfaces used
for prior phases: changed facts, rebind rows, projection digests, counters, and
visible result summaries.

Mechanically, this phase should add at least one authored structural proof flow
where a running app changes a live mount or projection choice entirely through
authored source. The visible result must not rely on developer inference. The
proof app must show the previous component or mount target, the next component
or mount target, the exact authored-structural changed facts, the rebuilt
projection rows, the preserved rows, and the resulting visible structure.

**Requirements**
- Add validation-app proof slices for authored structural remounting or
  reprojection over already-registered capability authority.
- Extend the visible evidence panel to name authored-structural changed facts
  and the projections they rebuilt.
- Keep renderer files paint-only; authored structural truth must come from
  runtime receipts and proof snapshots only.
- Ensure manual verification uses the same runtime spine and evidence as
  automated tests.

**Warnings**
- Do not fake structure reload by editing compiled Rust and then calling it hot
  reload.
- Do not hide authored-structural proof in logs or tests only.
- Do not accept local renderer booleans, local mount maps, or local component
  selectors as proof of authored structural reload.

**Test requirements**
- A running app visibly remounts or reprojects an already-registered component
  through authored source only.
- The visible proof panel shows exact authored-structural changed facts.
- Automated tests assert the same rebuilt and preserved projection rows the app
  displays.
- Invalid or unsupported authored structural edits preserve prior-valid runtime
  truth with typed denial evidence.

**Engineering decisions**
- The validation app remains product proof, not a special debug harness.
- Visible structure reload proof must consume the same runtime receipts as the
  rest of the milestone rather than inventing app-local state stories.

**Open questions**
- Which authored structural proof flow best represents the Shopify dashboard
  target while staying minimal enough for this side quest.

### Phase 28: Authoring-Truth Hot Reload Final Boss Certification

This phase closes the side quest with the proof we actually care about: runtime-
owned composition reload over authored product meaning. The final boss must not
be another isolated family reload. It must demonstrate that one save can change
multiple authored responsibilities, that the runtime lowers those changes into
typed changed facts, that only consuming projections rebuild, and that the
result is deterministic under replay.

The strongest scenario is a mixed authored save that does all of the following
without recompiling Rust:
- introduces or activates a new authored structural declaration over already-
  registered capability authority
- repoints a live mount, surface, or projection to that declaration
- changes one unrelated authored value such as theme, appearance, density, or
  another localized authored responsibility
- preserves unaffected projections and runtime truth elsewhere in the app

This proves the full intended model:
- authored source is the ordinary reload boundary
- changed meaning lowers into exact touched facts
- the touched graph drives rebind breadth
- visible product structure changes through runtime-owned receipts
- compiled platform meaning remains the non-reloadable boundary

Mechanically, the final boss should record one named end-to-end certification
artifact that captures authored delta digest, changed-fact families, rebuilt
projection identities, preserved projection identities, visible result digest,
counter posture, replay posture, and the compile-boundary explanation for why
the same proof does not apply to brand-new Rust component implementations.

**Requirements**
- Add a named final-boss certification scenario that combines authored
  structural change with at least one non-structural authored value change in
  one save.
- Prove the resulting runtime change is admitted through one mixed runtime
  evidence envelope.
- Prove rebuild breadth follows changed-fact intersection, not source-edit
  breadth.
- Prove deterministic replay of authored delta, changed-fact rows, projection
  rows, and visible result digest.
- Record the compiled-authority boundary explicitly: new Rust-rendered component
  behavior still requires compile and is not part of hot reload completion.

**Warnings**
- Do not call the milestone complete with a final boss that only changes one
  style token.
- Do not make the final boss rely on special-case validation-app glue instead of
  ordinary runtime-owned authored truth.
- Do not blur the boundary between product meaning and platform meaning.

**Test requirements**
- One save changes authored structure and one unrelated authored value.
- The visible app result changes in both places without restart or Rust edit.
- The evidence log shows mixed family rows, exact changed facts, rebuilt
  projections, preserved projections, and counter posture.
- Replay produces identical evidence digest and projection digest.
- Structural guards or compile-boundary proof make explicit that adding a new
  Rust component implementation still requires compile.

**Engineering decisions**
- The final boss is a certification program, not a flashy demo.
- The side quest is only complete when this scenario proves runtime-owned
  composition reload over authored truth rather than watcher theater over file
  categories.

**Open questions**
- Whether the final boss should live entirely inside the validation app or
  also receive a separate certification harness for replay-focused debugging.

### Phase 29: Primitive Boundary Contract

This phase replaces the old component-by-component proof direction with a
shared primitive boundary. The validation app must stop teaching the platform
that a button, panel, row, or card owns its own private style and interaction
vocabulary. Worth UI needs one hot-reloadable primitive stack that later
components consume.

The boundary for this phase is:

- `surface` owns product-facing placement meaning and stable identity.
- `mosaic` owns shell/page structural space allocation.
- `container` owns local primitive alignment and containment posture.
- `measurement` owns padding, radius, gap, size, and duration values through
  named density, sizing, or measurement facts. Anonymous layout numbers are not
  valid public primitive truth.
- `content` owns visual anatomy such as text, icon, image, spacer, and groups.
- `appearance` owns chrome, tones, typography, seams, and visual states.
- `interaction` owns gestures, cursor posture, focus posture, and emitted
  runtime meaning.
- `motion` owns admitted transitions and animations over declared properties.
- `primitive graph` owns dependency edges between authored truth, capability
  truth, lowered primitive receipts, event regions, active observations, paint
  plans, interaction receipts, diagnostics, and presentation evidence.

**Requirements**
- Define the shared authored and lowered vocabulary for surface, mosaic,
  container, measurement, content, appearance, interaction, and motion.
- Keep the vocabulary as authored meaning that lowers into the existing
  source-to-artifact-to-runtime chain rather than a second runtime model.
- Add runtime fact families and projection dependency contracts for the first
  centered primitive proof, including primitive measurement, primitive
  interaction, and primitive motion facts.
- Add the primitive graph contract that every primitive family extends:
  authored truth and capability truth enter as authority inputs, admitted
  family values lower into sealed receipts, receipts publish graph facts,
  projections declare consumed facts, changed facts select projection rebinds,
  and renderer plans consume only graph-derived receipts.
- Render one centered surface whose visual result comes only from the lowered
  primitive receipts.
- Hot reload text content, named padding measurement, background, alignment,
  interaction payload, and motion timing through authored facts.
- Make authored primitive prop schemas the authority for value admission, not
  metadata. Schemas must own defaults, value kind, expected shape, examples,
  semantic slice, fact family, and denial code.
- Add a primitive prop admission boundary that converts raw authored surface
  props into sealed validated primitive values or sealed primitive value denial
  receipts before any primitive proof receipt is constructed.
- Primitive prop admission must be a batch boundary, not a per-prop callback
  path. One authored surface scan produces one admission report with accepted
  prop-set receipt or a denial set.
- Typed primitive denials must wrap first-class denial receipts carrying schema
  identity, raw value, expected shape, examples, semantic slice, fact family,
  denial code, and stable denial digest.
- Admission reports must carry counters for schema count, authored props seen,
  defaults applied, values validated, and denials emitted.
- Primitive schema declarations must be self-certifying: unique schema ids,
  unique prop keys, declared default policy, valid defaults, examples, expected
  syntax, semantic slice, fact family, and denial code.
- Unknown `primitive_`-prefixed authored props must reject as schema denials.
  Non-primitive surface props must pass through without contaminating primitive
  admission.
- Primitive measurement props must admit named tokens or named measurements and
  lower to resolved point values only inside sealed measurement receipts.
  Renderer-visible points are derived outputs, not authored truth.
- Primitive interaction props must lower to a sealed interaction receipt that
  names supported interaction kind, cursor posture, focus posture, interaction
  id, and submit payload. Component submit must consume this receipt instead of
  a button-local callback or button-local payload parser.
- Primitive interaction and appearance state must be modeled as proof-bearing
  graph states, not public boolean bags. Enabled, disabled, readonly, inert,
  selected, hover, press, focus, and future posture states must enter through
  typed transitions whose output types make illegal combinations
  unrepresentable before Phase 31 and Phase 33 consume them.
- Primitive motion props must lower to a sealed motion receipt that names the
  motion kind, property target, duration measurement, easing, and resolved
  duration. Renderer animation behavior may consume this receipt, but may not
  invent easing or timing defaults locally.
- Denial receipts must be ready for source spans and presentation projection.
  Presentation rows are derived from receipts for renderers, while digest and
  proof identity remain independent of display formatting.
- The first centered proof must expose graph evidence showing the authored
  surface, admitted primitive fact families, projection dependency edges,
  selected changed facts, resolved primitive receipt, and renderer draw plan as
  separate graph products derived from one active truth basis.

**Warnings**
- Do not introduce a generic "style blob" that collapses layout, content,
  interaction, and appearance into one bag.
- Do not treat mosaic as flexbox, and do not treat local flow/container layout
  as shell topology.
- Do not accept anonymous raw numbers for primitive padding, radius, gap, size,
  or duration when a named measurement, sizing, density, or style-value fact is
  the intended authority.
- Do not create a new surface abstraction; surfaces already exist and keep
  their Milestone 1 meaning.
- Do not let the renderer interpret authored property names.
- Do not expand button-specific props as the platform primitive model.
- Do not hard-code rejection copy in validation renderers. Renderers may show a
  typed denial receipt, but the expected-value language belongs to the runtime
  primitive schema.
- Do not let primitive resolvers parse raw authored prop values after primitive
  prop admission exists. Resolvers consume validated prop sets.
- Do not let component-specific interaction or motion props become the
  platform path.
- Do not expose proof-named primitive receipts with public constructors that
  accept raw strings, raw booleans, raw fact sets, or renderer observations.
- Do not let validation-app code assemble active visual state, operability
  state, interaction eligibility, or event targeting by combining local
  booleans. The app may submit host observations to the runtime graph and draw
  the receipts it receives back.

**Test requirements**
- Editing authored text, named padding, background, alignment, interaction
  payload, and motion duration produces exact changed facts for content,
  measurement, appearance, container, interaction, and motion slices.
- Projection dependency contracts name which primitive slices they consume.
- The renderer receives resolved receipts and does not inspect authored prop
  names.
- Malformed primitive values produce typed denials whose displayed guidance is
  derived from the authored prop schema for the rejected prop.
- Tests assert denial receipt fields and denial digest/equivalence basis, not
  display-string substrings.
- Batch admission tests cover all declared defaults, multiple invalid values in
  canonical schema order, unknown primitive-prefixed prop rejection,
  non-primitive prop pass-through, stable denial-set digest, admission counters,
  and receipt-derived presentation rows.
- Schema certification tests fail if primitive schemas are duplicated,
  incomplete, or declare defaults that do not admit under their value kind.
- Measurement admission tests prove raw numeric padding rejects, valid named
  density tokens resolve to points, missing tokens deny before receipt
  construction, and changed density-backed primitive values rebind only
  consuming primitive projections.
- Interaction admission tests prove submit payload edits affect the next
  sealed interaction receipt without rebuilding component code, and unsupported
  interaction kinds deny through the primitive schema/report path.
- Motion admission tests prove transition target, duration, and easing admit
  into a sealed motion receipt, and invalid duration or target values deny
  through schema-derived expected syntax.
- Primitive graph tests prove authored changes enter as graph facts, projection
  dependencies are declared, and only intersecting consumers rebind.
- Typestate tests prove illegal primitive progressions cannot be expressed:
  renderer observations cannot become paint posture directly, app code cannot
  mark a denied family as accepted, and disabled operability cannot be consumed
  as enabled interaction eligibility.
- Compile-fail coverage prevents app code from minting primitive receipts,
  validated primitive values, prop admission reports, prop admission receipts,
  admission counters, denial sets, value denial receipts, changed-fact proof,
  dependency contracts, graph-derived observation receipts, active paint
  posture receipts, or interaction activation receipts directly.

**Manual verification**
- Launch the validation app and open the primitive proof page.
- Change the centered proof text, named padding token, background, alignment,
  interaction payload, and motion duration in source.
- Confirm each edit hot reloads without restart.
- Confirm the evidence panel names the exact primitive fact family that changed
  and shows bounded projection rebind rows.
- Confirm the evidence panel names the graph basis for the proof: authored
  surface, admitted primitive families, projection consumers, active draw plan,
  and any active observation or interaction receipt.
- Enter invalid values such as a named color, invalid alignment, and nonnumeric
  padding. Confirm the running app stays alive, preserves the typed rejection
  boundary, and shows schema-derived expected syntax for the exact prop that
  was rejected.
- Enter multiple invalid primitive values in one save. Confirm the running app
  shows all rejected primitive values from one admission report, including
  counters and receipt-derived presentation rows.
- Add a typo such as `primitive_backgrounnd`; confirm it rejects as an unknown
  primitive prop. Add a non-primitive surface prop; confirm primitive admission
  ignores it.
- Try `primitive_padding 32`; confirm it rejects as a raw measurement and shows
  the schema-owned named-token syntax.

**Engineering decisions**
- This phase establishes the vocabulary later phases grow, so every later proof
  should reuse these primitive families instead of creating local component
  props.
- A surface is placement/identity meaning, not a CSS div.
- The first proof is intentionally tiny because it must prove the architecture
  boundary before proving visual breadth.
- The primitive graph is part of Phase 29, not a later optimization. Later
  phases may add graph fact families, but they must not introduce local
  mini-graphs in renderers or validation-app code.
- `container` lowers as a distinct local primitive family. It does not replace
  mosaic, and mosaic does not own primitive anatomy.
- `measurement`, `interaction`, and `motion` are required primitive families in
  this phase, not later polish.

### Primitive Diagnostics Contract For Phases 30+

Every future primitive family added after Phase 29 inherits the primitive
admission and diagnostics boundary. New layout, appearance, interaction,
content, motion, overlay, collection, adaptive, or component-composition
authoring must extend the same pattern rather than inventing local parse errors
or renderer prose.

Every future primitive family also inherits the graph-owned proof boundary.
The runtime maintains a DAG from authored truth and capability truth to
admitted primitive facts, lowered receipts, draw plans, event regions,
observation receipts, interaction receipts, diagnostics, and presentation
evidence. A phase is incomplete if it ships a visible behavior by letting
renderer code classify raw observations, re-run graph lookup, or stitch
booleans into a state that should have been a graph-derived proof type.

Every future primitive family must also translate web/CSS vocabulary into
Worth-owned runtime meaning before it becomes public DX. CSS and HTML may be
useful as a problem inventory, but they are not an acceptable authority model
for Worth UI. The public authoring surface must not grow selectors, cascade,
specificity, class bags, pseudo-classes, DOM order semantics, `display`,
`visibility`, `overflow`, `pointer-events`, arbitrary `position`, arbitrary
percent/viewport units, media-query rules, `z-index`, or tag-soup composition.
When Worth UI needs the same capability, it must enter as a named runtime
family: recipe binding, participation posture, scroll ownership, event policy,
mosaic topology, flow adjacency, portal-host ordering, adaptive alternatives,
motion receipts, collection windows, or typed component composition.

**Requirements**
- Each authored primitive value family must register schemas with schema id,
  prop key or declaration key, value kind, default policy where applicable,
  expected syntax, examples, semantic slice, fact family, and denial code.
- Admission must run as a batch over the relevant authored boundary and return
  one report with an accepted receipt or a denial set. Per-field rejection
  helpers may exist internally, but they must not become the public proof
  surface.
- Reports must expose counters for schemas considered, authored values seen,
  defaults applied where relevant, values validated, denials emitted, and any
  family-specific breadth needed to prove no hidden broad scan.
- Denial receipts must carry stable digest material, schema identity, raw
  authored value, expected syntax, examples, semantic slice, fact family, denial
  code, source-span readiness, and receipt-derived presentation rows.
- Renderer and validation-app surfaces may display report and receipt
  presentations, but must not format expected-value guidance from their own
  tables, inspect authored prop names, or parse raw authored values.
- Unknown primitive-family declarations must have an explicit policy. Unknown
  keys inside a primitive namespace reject as schema denials; unrelated product
  surface props pass through unless the family explicitly owns them.
- Each primitive family must publish its graph contract: authored truth inputs,
  capability truth inputs, derived fact family, projection consumers, receipt
  type, changed-fact identity, and counter boundary.
- Each phase must define the proof progression for its family as typed
  transitions. Raw authored values or raw host observations become admitted
  values, admitted values become lowered receipts, lowered receipts become
  graph facts, and graph facts become renderer-consumable plans or receipts.
  Later phases may consume only the proof-bearing output of the previous
  transition.
- Each phase must name the invalid states it makes unrepresentable. If the
  state can still be assembled from public fields, raw booleans, raw strings,
  or local enums in app code, the phase has not satisfied Law 41.
- Each phase that depends on surface lookup, parent/child containment, region
  ordering, projection consumer selection, or event targeting must use a
  runtime-owned graph/index view with counters. Surface-local recursion,
  renderer-local hit tables, and per-call registry scans are not acceptable
  proof boundaries.
- Each phase that is tempted to expose a CSS-like property must name the
  Worth-native family that owns the meaning instead. The spec and public APIs
  must make clear whether the capability belongs to source truth, capability
  truth, a graph fact, a projection receipt, a renderer draw plan, or host
  observation.

**Test requirements**
- Every phase that adds schemas also adds schema certification for uniqueness,
  completeness, valid defaults where applicable, denial-code consistency, and
  examples.
- Every phase that adds admitted values tests multi-denial batch behavior in
  canonical order, stable denial-set digest, report counters, unknown-key
  policy, and receipt-derived presentation rows.
- Tests assert structured receipt/report fields and digest behavior. They must
  not pass by matching renderer strings.
- Compile-fail coverage expands with each new proof-bearing report, receipt,
  counter, denial set, validated value, or admitted plan type.
- Graph-contract tests prove each phase declares consumed facts and invalidated
  facts, and that changed-fact evidence rebinds only declared consumers.
- Law 41 tests prove renderer/app code cannot construct graph-derived proof
  receipts directly, cannot skip the admitted-to-lowered transition, and cannot
  express the invalid states the phase claims to ban.
- Counter tests prove graph/index consumption remains bounded by semantic
  delta, selected regions, or declared consumer sets rather than broad scans.
- Anti-CSS boundary tests reject selector-like, class-like, raw style-bag,
  `z-index`, display/visibility, overflow, pointer-events, and arbitrary
  unit-soup declarations unless the relevant phase has translated that meaning
  into a Worth-native schema with proof receipts and graph consumers.

**Manual verification**
- Each visible proof with invalid authored input must show all denials from the
  same runtime report, including counters and presentation rows derived from
  receipts.
- Manual steps must include at least one multi-denial save and one unknown-key
  save for the newly introduced primitive family when that family admits
  authored keys.
- Manual proof evidence must name graph facts and projection consumers, not
  renderer branches or component-local state names.

### Phase 30: Flow Layout Primitives

This phase adds local flow layout so components can compose anatomy without
reimplementing size, position, padding, adjacency, alignment, and gap rules. It
is not the full mosaic shell layout system; it is the inner content-layout
family used by atoms, rows, cards, menus, and inspector sections.

**Requirements**
- Add local flow layout primitives for `row`, `column`, `stack`, `inline`,
  `grid`, `spacer`, main-axis `align`, cross-axis/baseline alignment, named
  measurement-backed `gap`, named measurement-backed `padding`, `fit`, and
  `fill`.
- Model outside separation as flow adjacency, slot spacing, mosaic region
  spacing, or collection item spacing. Do not add CSS-style `margin` or margin
  collapse semantics to primitive authoring.
- Lower flow layout declarations into runtime layout/container facts and
  resolved layout receipts.
- Publish flow layout as a graph fact family consumed by draw-plan generation,
  event-region derivation, content baseline placement, and projection rebind.
  Draw plans must consume `WorthUiFlowLayoutReceipt` or equivalent
  proof-bearing layout receipts, not renderer-local gap, padding, fit, fill, or
  alignment choices.
- Resolve flow gap and padding from named density, sizing, or measurement facts;
  renderer-visible point values are derived receipt outputs, not authored truth.
- Keep mosaic responsible for shell/page region topology and flow layout
  responsible for local content arrangement.
- Extend primitive authored schemas for every new flow-layout value kind, with
  expected-value syntax owned by schema metadata and reused by typed denials.
- Add flow-layout admission reports and denial sets that reuse the primitive
  diagnostics contract, including counters, stable denial-set digest, source
  span readiness, and receipt-derived presentation rows.
- Hot reload the centered proof between text-only, inline icon/text, and
  stacked icon/text compositions without renderer code changes.
- Preserve exact layout identity and changed-fact evidence when only local
  alignment, gap, or padding changes.

**Warnings**
- Do not make flow layout a hidden mini-DOM.
- Do not force mosaic to impersonate every inner component layout.
- Do not let component renderers choose gap, padding, or alignment locally.
- Do not expose CSS `margin` as child-owned outside space. External spacing is
  owned by the parent layout relation that places siblings or regions.
- Do not use raw anonymous numbers where named values or authored measurement
  facts are required by the current milestone law.
- Do not add one-off denial messages for each flow prop; invalid `gap`, `fit`,
  `fill`, and alignment values must format from the same authored schema that
  lowered them.
- Do not let event hit geometry rediscover local flow frames from authored
  props. Event regions consume graph-derived draw plans that already consumed
  flow receipts.

**Test requirements**
- Editing named `gap`, named `padding`, main-axis `align`, cross-axis/baseline
  alignment, `fit`, or `fill` changes only the consuming flow-layout
  projections.
- Switching from inline to stack changes the local layout family without
  rebuilding unrelated surface or interaction projections.
- Equivalent named flow measurement declarations lower to equivalent resolved
  runtime layout receipt values while preserving authored token identity.
- Draw-plan tests cover row, column, inline, stack, grid, spacer, fit, fill,
  main-axis alignment, and baseline alignment from resolved receipt metrics.
- Graph-consumption tests prove flow receipts feed draw-plan facts and
  event-region facts through declared dependency edges, with no renderer-local
  parsing or layout recomputation from authored `flow_*` props.
- Invalid flow-layout values produce typed schema-referenced denials without
  renderer-local parsing or string matching.
- A single save with multiple invalid flow-layout values produces one report
  with all denials in canonical schema/declaration order.
- Unknown flow-layout primitive keys reject through the same denial-set path,
  while unrelated surface props pass through.
- Steady-frame counters prove no source parsing or broad artifact scan is
  needed to execute the local flow layout.

**Manual verification**
- Change the proof from text-only to icon plus text in an inline row.
- Change the same proof to a stacked icon and text.
- Change gap, padding, main-axis alignment, and cross-axis/baseline alignment
  values.
- Confirm each visible layout change appears live and the evidence panel names
  local flow-layout facts rather than component-specific branches.
- Confirm the evidence panel shows draw-plan and event-region consumers of the
  changed flow facts when local layout affects geometry.
- Change one flow value to an invalid token and confirm the visible rejection
  names the exact authored prop and the expected syntax from the flow schema.
- Change several flow values to invalid tokens in one save and confirm the
  evidence panel shows one flow admission report with all denials and counters.

**Engineering decisions**
- Flow layout is the default anatomy layout family for ordinary components.
- Mosaic remains the structural workspace/page layout family.
- The proof must make it obvious that layout behavior is reusable before the
  milestone starts rebuilding button, row, or card variants.

**Engineering decisions**
- Baseline alignment belongs in Phase 30. Until exact font metrics land in the
  universal content primitive phase, baseline planning uses deterministic
  metrics derived from resolved text size and icon size in content receipts.

### Phase 31: Stateful Appearance Recipes

This phase replaces one-off `pressed_*`, `hover_*`, and component-local style
fields with a reusable stateful appearance model. Any visual node should be
able to resolve rest, hover, pressed, focus, disabled, and selected appearance
through the same runtime-owned state map.

**Requirements**
- Add authored appearance recipes with state maps for `rest`, `hover`,
  `pressed`, `focus`, `disabled`, and `selected`.
- Bind appearance recipes explicitly through admitted primitive facts or
  capability defaults. Do not introduce selectors, class names, specificity, or
  cascade as the mechanism that decides which recipe applies.
- Define deterministic state precedence and inheritance rules.
- Lower stateful appearance recipes into resolved appearance receipts consumed
  by renderers.
- Resolve active appearance from graph-owned observation receipts. Renderer
  code may report host observations, but it must not decide whether the active
  state is rest, hover, pressed, focus, disabled, selected, or any combination.
- Register appearance-state prop schemas for state names, color-like values,
  border values, radius, opacity, typography, and token references before
  admitting those values.
- Admit appearance recipes through batch reports with denial sets, counters,
  stable digests, source-span readiness, and receipt-derived presentation rows.
- Prove that appearance state maps can style background, foreground, border,
  border radius, opacity, focus ring, typography, and icon/text tones where the
  primitive supports them.
- Hot reload hover color, pressed border, focus ring, disabled opacity, and
  selected posture on the centered proof.
- Define the active-state typestate family that downstream phases consume:
  enabled observations may carry hover, pressed, focus, and selected posture;
  disabled/inert observations may not carry hover, pressed, or focus activation
  posture. If a state combination is mechanically impossible, it must be
  unrepresentable rather than suppressed during paint.

**Warnings**
- Do not repeat every style primitive under every component-specific state
  prefix.
- Do not model stateful appearance as CSS pseudo-classes or selector matching.
  Hover, pressed, focus, disabled, selected, and future states are graph-owned
  observation receipts consumed by appearance recipes.
- Do not introduce cascade, specificity, or "last declaration wins" semantics.
  Recipe precedence must be declared, typed, and testable.
- Do not let disabled, focus, selected, hover, and pressed precedence become
  renderer folklore.
- Do not make appearance own structure, runtime truth, or interaction emission.
- Do not make raw color piles the preferred end-state over semantic tokens and
  appearance recipes.
- Do not fork per-state diagnostics. Rest, hover, pressed, focus, disabled, and
  selected denials must all use the same schema-derived expected-value path.
- Do not accept a public `observed(hovered, pressed, focused, disabled, ...)`
  style constructor as proof. That is a value bag, not a Law 41 state
  transition.

**Test requirements**
- State precedence is deterministic for combinations such as hover+focus,
  pressed+hover, disabled+hover, and selected+focus.
- Disabled, inert, or readonly observation tests prove hover, pressed, focus,
  and activation-only style overlays cannot be represented on the disabled
  branch. This must be enforced by types or compile-fail boundaries, not by a
  late runtime normalization branch.
- Editing a state recipe changes only appearance facts and rebinds only
  projections that consume that recipe.
- Removing a state override falls back to the correct inherited/base value.
- Renderers consume a resolved active-state appearance receipt, not authored
  state keys.
- Renderer-boundary tests prove app code cannot construct active appearance
  posture from raw booleans and cannot pass raw host observations directly to
  paint planning.
- Invalid state names, malformed color/token values, bad radius values, and
  invalid opacity values produce typed denials whose displayed expectation is
  derived from the appearance-state schema.
- Multiple invalid state recipe values in one save produce one appearance
  admission report with all denials and stable denial-set digest.
- Tests prove appearance-state presentation rows are derived from denial
  receipts, not renderer-owned copy.

**Manual verification**
- Hover, press, focus, disable, and select the centered proof where applicable.
- Change each state's authored visual recipe while the app is running.
- Confirm active visual state updates live and the evidence panel names
  appearance-state changed facts.
- Disable the proof while hovering or pressing and confirm the evidence names a
  disabled observation receipt, not a disabled+pressed or disabled+hover
  boolean combination.
- Enter invalid values for at least one state name, one color/token field, and
  one numeric field. Confirm the rejection is visible in the running app and
  the expected syntax comes from schema metadata rather than renderer prose.
- Enter several invalid appearance-state values in one save and confirm the
  running app shows one report with counters and all denial rows.

**Engineering decisions**
- Stateful appearance is a primitive family, not a button feature.
- State maps compose with interaction posture but do not emit interactions.
- The proof intentionally stays on one centered surface so failures are easy to
  attribute.

**Open questions**
- Whether appearance-state inheritance should admit named state groups such as
  `interactive`, `selectable`, or `danger` in this milestone or leave that for
  the design-system milestone.

### Phase 32: Generic Interaction Lane

This phase turns interaction from local callbacks into runtime-owned authored
meaning. Clickable, submit, command, toggle, open, and focus behavior should
all lower into a generic interaction lane with sealed receipts.

**Requirements**
- Add generic authored interaction declarations for `click`, `submit`,
  `command`, `toggle`, `open`, and `focus`.
- Lower interactions into runtime-owned interaction facts with declared
  dependency contracts.
- Emit sealed generic interaction receipts from mounted surfaces and
  components.
- Lower interaction readiness, primitive disabled posture, command support,
  focusability, containment, and capability support into graph-owned
  operability receipts. Enabled, disabled, readonly, inert, unsupported, and
  denied interaction paths must be distinct receipt variants or typestates, not
  a set of booleans callers interpret.
- Admit interaction declarations through structured reports with denial sets for
  invalid kind, payload, target, command, toggle, open, focus, or readiness
  values.
- Hot reload submit payloads, command targets, toggle values, and open targets
  without recompiling component code.
- Keep renderers responsible only for gesture detection and runtime submission
  through the generic host method.
- Runtime submission must consume an enabled activation receipt produced by the
  graph. A disabled, inert, readonly, unsupported, or denied interaction
  receipt may be displayed and diagnosed, but it must not satisfy the activation
  API.

**Warnings**
- Do not add component-local callback systems.
- Do not let payload parsing or action dispatch live in validation renderer
  code.
- Do not flatten command/readiness/runtime posture into one boolean.
- Do not let app code mint sealed interaction receipts.
- Do not let app code decide that a hit gesture is actionable by checking
  `can_activate` or `disabled` booleans. The graph must emit either an
  activation-eligible receipt or a typed non-activation outcome.

**Test requirements**
- Clicking the centered primitive emits a sealed generic submit receipt with
  the active authored payload.
- Editing the payload or interaction target changes the next emitted receipt
  without rebuilding component code.
- Command/toggle/open declarations share the same proof lane rather than
  branching into local systems.
- Operability typestate tests prove a disabled/readiness-disabled interaction
  cannot produce an enabled activation request and cannot emit a submit,
  command, toggle, open, or focus receipt through the mounted interaction API.
- Invalid interaction declarations produce one batch report with typed denials,
  counters, stable digest, and receipt-derived presentation rows.
- Compile-fail coverage proves app code cannot mint receipts or bypass the
  runtime host method.
- Compile-fail coverage proves disabled or non-operable interaction receipts
  cannot be passed to APIs that require enabled activation.

**Manual verification**
- Click the centered proof and inspect the emitted interaction receipt.
- Edit the payload and click again without restart.
- Confirm the next receipt carries the new payload and the evidence panel names
  interaction changed facts.
- Disable the interaction and click it. Confirm the visible evidence shows a
  typed non-activation outcome and no successful interaction receipt can be
  emitted.
- Enter multiple invalid interaction declarations in one save and confirm the
  app shows one runtime interaction admission report rather than local callback
  errors.

**Engineering decisions**
- Interactions are runtime facts and receipts, not closures.
- Component renderers detect gestures; runtime owns the meaning and receipt.
- This phase is the behavioral twin of Phase 31's visual state map.

**Open questions**
- Whether double-click and long-press should enter in this phase or wait until
  gesture arbitration broadens.

### Phase 33: Cursor, Hit Testing, and Event Containment

This phase adds the event geometry required for real components. Cursor
behavior, hit area, pointer capture, and nested containment must be authored
and runtime-owned rather than component-local accident.

**Requirements**
- Add cursor recipes tied to interaction posture, including pointer, text,
  grab, grabbing, resize, and default cursors where supported by the host.
- Add hit-area contracts for visual bounds, padded bounds, explicit hit slop,
  and disabled hit behavior.
- Add participation and event-policy receipts for pass-through, capture,
  disabled-hit, inert, and blocked-below behavior where the event family owns
  those meanings. Do not expose CSS-style `pointer-events` as a paint-layer
  switch.
- Add nested event containment rules for parent and child interactive content.
- Add pointer-capture basics for press/drag continuity where the current host
  can prove them.
- Admit cursor, hit-area, capture, and containment declarations through the
  shared diagnostics contract with typed denial receipts and counters.
- Build event dispatch from the runtime graph: active draw plans and interaction
  operability receipts produce event-region graph facts; host pointer
  observations are classified against those facts; classification emits typed
  dispatch receipts such as no hit, enabled hit, disabled hit, bubbled,
  captured, emitted, or denied. Renderer code must not assemble event state by
  combining hit, selected, emitted, disabled, and can-activate booleans.
- Cursor posture must be a graph-derived dispatch receipt. A disabled or inert
  region may still produce a diagnostic hit and `not_allowed` cursor receipt,
  but it must not produce an enabled hover, pressed, focus, capture, or
  activation observation.
- Prove a clickable card-like surface containing an inner button-like primitive
  where inner and outer clicks emit distinct receipts.

**Warnings**
- Do not let child interactions accidentally trigger parent clicks unless
  authored bubbling/containment says so.
- Do not hide hit slop in renderer constants.
- Do not let cursor arbitration be decided by paint order alone.
- Do not solve nested click behavior with component-name special cases.
- Do not implement interaction participation as `pointer-events` folklore.
  Hit testing, cursor posture, capture, propagation, disabled hits, and
  pass-through behavior are separate runtime facts with typed dispatch
  receipts.
- Do not let event dispatch expose candidate receipts as raw boolean bags when
  the combinations have legal meaning. Candidate, selected, emitted,
  disabled-hit, bubbled, captured, and no-hit states must be typed outcomes
  where downstream APIs can accept only the legal variants.
- Do not let validation-app code perform graph lookup for parent/child
  containment or hit routing. It submits host pointer observations and consumes
  dispatch receipts.

**Test requirements**
- Inner click emits only the inner interaction unless containment allows
  propagation.
- Outer click emits only the outer interaction.
- Cursor arbitration resolves deterministically when nested regions overlap.
- Dispatch typestate tests prove disabled hits cannot produce enabled hover,
  pressed, capture, or activation receipts, and that disabled cursor evidence
  remains diagnosable without becoming actionable.
- Graph-index tests prove nested containment, region ordering, and bubbling are
  selected from event-region graph facts rather than renderer traversal or
  component-name branches.
- Editing hit padding or cursor posture changes only interaction-geometry facts
  and consuming projections.
- Invalid cursor names, hit-area values, containment policies, or capture
  postures produce schema-derived denial sets with receipt presentations.
- Compile-fail coverage proves renderer/app code cannot construct dispatch
  receipts, event-region graph facts, or enabled activation receipts directly.

**Manual verification**
- Hover the outer surface and inner primitive and confirm cursor changes match
  authored posture.
- Click the inner primitive and confirm only the inner receipt emits.
- Click the outer surface and confirm only the outer receipt emits.
- Hot reload cursor and hit-padding values and confirm behavior changes live.
- Disable the inner primitive while hovering or pressing it. Confirm cursor
  evidence remains disabled/not-allowed, no active outline or pressed state can
  appear, and evidence names a disabled-hit dispatch receipt.
- Enter invalid cursor and hit-area values in one save and confirm the app shows
  one interaction-geometry admission report with both denials.

**Engineering decisions**
- Event geometry is a primitive family because every serious component needs
  it.
- Hit testing consumes layout and interaction facts but must not redefine
  layout or interaction meaning.
- This phase protects later cards, rows, menus, overlays, splitters, and drag
  handles from bespoke event code.
- Event dispatch is where the primitive graph pays for itself: mechanically
  impossible interaction states must be banned by graph-derived receipt variants
  before appearance or interaction emission consumes them.

**Open questions**
- How much pointer-capture behavior can be certified against the current egui
  adapter without waiting for lower native adapter work.

### Phase 34: Universal Content Primitives

This phase makes text, icons, images, spacers, badges, and dividers reusable
content anatomy rather than button-only or card-only fields. Content should be
hot-reloadable independently from appearance and interaction while still
consuming their resolved state where appropriate.

**Requirements**
- Add universal content primitives for text, icon, image, spacer, badge,
  divider, inline group, stack group, and slot where supported by current
  lowering.
- Support independent text and icon size, color/tone, alignment, accessibility
  name, and measurement posture.
- Keep icon references through registered icon IDs, not raw asset paths.
- Support native vector icon rendering where the host adapter can provide it,
  while preserving fallback posture as explicit capability support.
- Publish content anatomy as graph facts consumed by flow layout, baseline
  planning, accessibility projection, appearance-tone resolution, event-region
  derivation, and renderer draw plans. Content receivers must consume content
  receipts rather than rediscovering text/icon/image/spacer structure from
  authored keys.
- Model content presence as typed participation posture where needed: present,
  absent, hidden from paint, hidden from accessibility, inert, denied,
  loading, or unsupported. Do not reuse CSS `display` or `visibility` as
  paint-only style flags.
- Admit content declarations through batch reports, including invalid icon IDs,
  unsupported native vector posture, text measurement values, image references,
  slot declarations, and accessibility names.
- Prove the same content receipt can render inside a button-like primitive,
  row-like primitive, and card-like primitive.

**Warnings**
- Do not model this as "icon plus text" inside button.
- Do not make text/icon spacing a component-specific field.
- Do not let raw icon paths stand in for `IconId`.
- Do not let content primitives emit interactions directly; interactions attach
  through the interaction family.
- Do not use `display: none` or `visibility` semantics as shorthand for
  content lifecycle. Presence, layout participation, accessibility
  participation, event participation, and retained state must be separate
  receipt-backed meanings where they differ.
- Do not let a button, card, row, or menu own a private "icon plus text" branch.
  The same content receipt and flow receipt must compose regardless of the
  consuming component family.

**Test requirements**
- Editing icon ID, icon size, icon color, text size, text color, gap, and
  content order changes content or appearance facts only as appropriate.
- The same content declaration can be consumed by multiple component families
  without renderer-specific prop plumbing.
- Text and icon baseline/alignment behavior is deterministic across inline and
  stacked layouts.
- Registered icon capability support determines native SVG/vector rendering
  admission rather than renderer-local guesses.
- Graph-consumption tests prove the same content receipt feeds multiple
  primitive/component draw plans and accessibility projections without
  component-local content parsing.
- Invalid content declarations produce typed denial sets with schema/capability
  identity, counters, and receipt-derived presentation rows.

**Manual verification**
- Change text-only content to icon/text content.
- Swap plus/check or another registered icon.
- Change icon size independently from text size and change gap independently
  from both.
- Render the same content in at least two different proof surfaces and confirm
  both update live.
- Enter an unknown icon id and one malformed content measurement in the same
  save; confirm both denials appear in one content admission report.

**Engineering decisions**
- Content primitives are local anatomy, not data plumbing.
- Icons are capabilities and content references, not renderer literals.
- Text measurement and baseline behavior are part of content/layout proof, not
  button polish.

**Open questions**
- Whether rich text/code text should enter here or wait for the text/editor
  milestone.

### Phase 35: Motion and Transition Recipes

This phase adds time as authored runtime meaning without allowing arbitrary
per-frame script. Motion should be a recipe over admitted properties with
deterministic interruption, reduced-motion posture, and reload behavior.

**Requirements**
- Add transition recipes over admitted appearance fields such as background,
  foreground, border color, border width, radius, opacity, shadow, and
  transform where supported.
- Define easing, duration, delay, interruption, retarget, cancel, and
  reduced-motion posture.
- Lower motion recipes into runtime motion receipts consumed by renderers.
- Publish active motion as graph facts derived from admitted motion receipts,
  active observation receipts, reduced-motion policy, elapsed host time, and
  prior motion continuity receipts. Renderer code may supply elapsed time and
  draw interpolated plans, but it may not choose easing, interruption,
  reduced-motion behavior, retarget policy, or active motion legality.
- Admit motion recipes through batch reports with typed denials for invalid
  easing, duration, delay, animated field, interruption, retarget, cancel, and
  reduced-motion values.
- Hot reload duration, easing, and animated field sets.
- Prove hover/press/focus transitions on the centered primitive without
  renderer-local animation policy.

**Warnings**
- Do not introduce arbitrary animation callbacks or script.
- Do not make motion alter authoritative state.
- Do not let reduced-motion policy be renderer folklore.
- Do not animate fields whose layout or hit-testing implications have not been
  admitted.
- Do not let active motion become per-frame mutable renderer state. Motion
  continuity, interruption, cancellation, and retargeting are graph-derived
  receipts over active truth and host time observations.

**Test requirements**
- Editing transition duration, easing, and animated fields changes motion facts
  and rebinds consuming projections.
- Press interrupting hover and hover leaving pressed resolve deterministically.
- Reduced-motion policy disables, shortens, or replaces motion according to
  runtime policy receipts.
- Hot reload during an active transition retargets or cancels through explicit
  receipts.
- Graph-contract tests prove motion consumes active observation facts,
  appearance facts, reduced-motion facts, and time observations through declared
  dependency edges rather than renderer-side policy.
- Invalid motion recipes preserve prior active truth and expose one denial set
  with stable digest, counters, and receipt-derived presentation rows.

**Manual verification**
- Hover and press the proof primitive and observe admitted transitions.
- Change duration and easing while the app is running.
- Toggle reduced-motion policy where available and confirm behavior changes
  through runtime receipts.
- Enter multiple invalid motion values in one save and confirm the visible
  report shows all motion denials from runtime receipts.

**Engineering decisions**
- Motion is runtime-owned recipe execution, not component-local animation code.
- Appearance motion comes before layout motion because it has narrower
  geometry consequences.
- Active motion must remain compatible with invalid reload preserving the
  prior valid plan.

**Open questions**
- Which easing vocabulary should be admitted now versus deferred to the design
  system milestone.

### Phase 36: Layout Motion and Geometry Reconciliation

This phase admits motion over size and position only where the runtime can
prove stable identity, deterministic hit testing, and safe geometry
reconciliation. Layout animation is stronger than appearance animation because
it changes where interactions land.

**Requirements**
- Add admitted layout-motion recipes for size, position, expansion, collapse,
  and transform where the layout family can prove stable identity.
- Define geometry reconciliation for hit testing during transitions.
- Publish layout-motion geometry as graph facts consumed by draw-plan
  interpolation, event-region derivation, scroll anchoring, focus preservation,
  and diagnostics. Hit testing during motion consumes reconciled geometry
  receipts, not stale draw frames or renderer-local interpolation choices.
- Define scroll anchoring and focus preservation behavior when animated layout
  changes affect visible geometry.
- Prove a panel or card expansion/collapse that hot reloads size, radius,
  position, and transition posture while preserving interaction state.
- Admit layout-motion and geometry reconciliation declarations through batch
  reports with typed denials and family counters.
- Keep unsupported layout-motion requests denied with typed diagnostics rather
  than silently falling back to renderer behavior.

**Warnings**
- Do not animate arbitrary layout changes whose identity or hit geometry is
  ambiguous.
- Do not let visual interpolation desync from hit testing.
- Do not make scroll jumps acceptable side effects of layout motion.
- Do not treat layout motion as a reason to bypass mosaic or flow layout facts.
- Do not let renderer animation state become the authority for current
  geometry. The graph must own the relationship between visual interpolation,
  event regions, focus, and scroll anchoring.

**Test requirements**
- Admitted expansion/collapse preserves eligible focus, hover, and press state
  through explicit reconciliation receipts.
- Hit testing during layout motion follows the declared geometry posture.
- Graph-contract tests prove interpolated draw frames, event regions, focus
  preservation, and scroll anchoring consume the same layout-motion geometry
  receipt.
- Unsupported layout-motion requests fail before activation and preserve the
  prior active plan.
- Editing layout-motion parameters rebinds only projections that consume the
  motion and geometry facts.
- Invalid or unsupported layout-motion declarations expose denial sets whose
  presentation rows come from receipts, not renderer warnings.

**Manual verification**
- Trigger the expansion/collapse proof and interact with it during motion.
- Hot reload the size, position, and duration values.
- Confirm hit testing and focus stay coherent and evidence names layout-motion
  facts.
- Enter one unsupported geometry field and one malformed duration in a single
  save and confirm one layout-motion report shows both denials.

**Engineering decisions**
- Layout motion is admitted only where geometry proof exists.
- The phase exists to prevent later panels, inspectors, accordions, and
  adaptive shells from inventing their own animation rules.
- Geometry reconciliation is runtime proof, not a renderer convenience.

**Open questions**
- Whether route/page transitions should wait for the workspace-shell milestone
  or share this layout-motion family now.

### Phase 37: Overlay and Anchored Layout

This phase adds the runtime portal host needed for dropdowns, menus, popovers,
tooltips, command palettes, overlays, anchored inspectors, modals, and toasts.
Overlay behavior must be authored/runtime-owned instead of accidental absolute
positioning or numeric stacking in a renderer.

**Requirements**
- Add overlay and anchored layout primitives for anchor target, side, alignment,
  offset, collision, flip, clamp, portal kind, portal lane, modality, dismissal,
  focus policy, event policy, and owner identity.
- Model anchored placement through anchor and portal receipts, not CSS
  `position: absolute`, `fixed`, or `sticky`. Sticky-like behavior belongs to
  collection or scroll-anchor families when it is tied to scrolling; anchored
  cross-surface behavior belongs to the portal host.
- Add a runtime-owned portal host that lowers admitted overlay declarations into
  an ordered pancake list of portal entries. The list is the single truth for
  cross-surface overlay order; paint order, hit testing, focus routing, escape
  handling, outside-click dismissal, accessibility posture, and diagnostics all
  consume the same portal-host receipt.
- Lower overlay declarations into runtime overlay layout facts, portal-host
  graph facts, ordered portal-entry receipts, and projection dependencies.
- Integrate overlay focus and dismissal with the generic interaction lane.
- Prove an authored button-like primitive opens a popover/menu through runtime
  receipts.
- Admit overlay declarations through batch reports with typed denials for
  anchor, side, alignment, offset, collision, flip, clamp, portal kind, portal
  lane, modality, dismissal, focus policy, event policy, and owner identity
  values.
- Hot reload anchor side, offset, width, collision/flip policy, dismissal
  posture, focus policy, event policy, and portal lane where the runtime can
  prove a legal reorder.

**Warnings**
- Do not build DOM-style portals, CSS-style stacking, or a second overlay
  runtime.
- Do not expose absolute/fixed positioning as public overlay DX. Authors
  declare anchor, owner, modality, lane, event policy, and collision behavior;
  the runtime computes placement.
- Do not let popover placement live in component code.
- Do not allow overlays to bypass focus, accessibility, or interaction
  containment.
- Do not make z-order an app-local integer pile. Authors declare portal meaning;
  the runtime portal host owns ordered portal entries.
- Do not let paint order, focus order, hit order, or dismissal order diverge.
  They are different consumers of the same portal-host pancake receipt, not
  separate renderer conventions.

**Test requirements**
- Overlay anchor geometry remains stable under equivalent layout source.
- Collision and flip behavior is deterministic and receipt-backed.
- Dismissal and focus-trap posture are runtime-owned and hot reloadable.
- Editing overlay placement facts rebinds only overlay-consuming projections.
- Portal-host ordering tests prove opening nested popovers, menus, modals,
  command palettes, and toasts appends or replaces entries according to runtime
  portal policy, not authored numbers.
- Shared-consumer tests prove paint order, hit testing, focus routing,
  dismissal, and accessibility consume the same ordered portal-host receipt.
- Reorder-denial tests prove an authored portal lane or modality edit that would
  violate focus, containment, or owner identity rejects before activation and
  preserves the prior portal list.
- Invalid overlay declarations produce one denial report with counters and
  receipt-derived presentation rows.

**Manual verification**
- Open the authored popover/menu from the centered proof.
- Open a nested menu/popover and confirm the newest portal appears above its
  owner while outside-click, escape, and focus behavior follow the same ordered
  portal list.
- Change anchor side, offset, width, collision posture, dismissal behavior,
  focus policy, event policy, and portal lane.
- Confirm the overlay changes live and evidence names overlay-layout facts,
  portal-host facts, and ordered portal-entry receipts.
- Enter invalid anchor side and focus policy values together and confirm both
  appear in one overlay admission report.

**Engineering decisions**
- Overlay layout is a layout family plus a runtime portal-host topology, not a
  component branch and not a CSS `z-index` clone.
- Anchored surfaces consume interaction, layout, appearance, and focus facts
  together, so this phase should reuse prior primitive receipts.
- The portal host is a pancake list because cross-surface overlay order is a
  runtime topology fact. Local draw order inside one surface remains a draw-plan
  concern; cross-surface portals belong to the portal host.
- Overlay proof is required before dropdown/menu behavior can be considered
  platform-owned.

**Open questions**
- Whether command palette and system toasts should be portal kinds in the same
  host or separate portal lanes with stricter modality and focus policies.

### Phase 38: Collection Layout and Virtualization

This phase admits repeated content and large collections without turning them
into full-materialized stacks. Lists, grids, table-like rows, timelines, and
chat logs need stable item identity, visible-range windows, scroll ownership,
scroll anchoring, and bounded frame evidence.

**Requirements**
- Add collection layout primitives for list, grid, table-like row windows, and
  timeline/chat-log style windows where current runtime lanes can support them.
- Lower iteration, item identity, visible range, estimated/known item sizing,
  sticky region, and scroll-anchor facts into runtime receipts.
- Lower clipping and scroll ownership into runtime receipts. Do not expose CSS
  `overflow` as a local style; the collection or scroll container must own
  whether content clips, scrolls, virtualizes, anchors, or participates in hit
  testing outside the visible range.
- Consume existing virtualized data lane posture where collection size or
  invalidation breadth requires it.
- Admit collection declarations through batch reports with typed denials for
  iteration, identity, visible range, item sizing, sticky regions, scroll
  anchors, and collection posture values.
- Prove hot reload of row/card appearance, item gap, list/grid mode, and
  visible-range posture without full collection materialization.
- Preserve collection-level loading, empty, denied, partial, and stale posture
  as typed runtime meaning rather than local placeholder branches.

**Warnings**
- Do not model large collections as ordinary stacks.
- Do not materialize off-screen rows for friendly authoring.
- Do not hide item identity in display text or index position.
- Do not create a UI-local collection status model.
- Do not use viewport/unit language as authoring truth for collection windows.
  Visible ranges, scroll anchors, and sticky regions are runtime receipts with
  counters.
- Do not implement `overflow: auto` behavior as renderer convenience. Scroll
  ownership and clipping are graph facts consumed by layout, hit testing,
  accessibility, and virtualization.

**Test requirements**
- Visible range counters prove off-screen items are not rendered or measured
  unless explicitly admitted.
- Item identity is stable across reorder, filter, scroll, and partial
  invalidation where runtime identity remains stable.
- Editing collection layout or item appearance facts rebinds only the relevant
  collection projections.
- Query-bound collection posture preserves upstream Query-owned live and
  async/result semantics.
- Invalid collection declarations produce one denial report with counters,
  stable digest, source-span readiness, and receipt-derived presentation rows.
- Tests prove collection denial reports preserve Query-owned posture instead of
  turning upstream status into local placeholder copy.

**Manual verification**
- Open the collection proof with enough items to require virtualization.
- Change row/card appearance, item gap, and list/grid posture.
- Scroll during and after edits and confirm scroll anchoring remains coherent.
- Confirm evidence shows visible-range counters rather than full collection
  materialization.
- Enter invalid visible-range and item-sizing values together and confirm one
  collection admission report shows both denials.

**Engineering decisions**
- Collection layout is its own family because ordinary flow layout cannot
  honestly certify huge surfaces.
- The phase is required before Shopify/Codex-style lists, chats, and project
  rails can scale.
- Collection posture consumes Query/runtime lanes where they already own
  stronger truth.

**Open questions**
- Whether sticky headers and grouped collection sections are required in this
  phase or can land as the first follow-up once basic virtualization is proven.

### Phase 39: Adaptive Layout Alternatives

This phase adds canonical adaptive alternatives for width, density, platform,
and runtime posture. Adaptive behavior must lower into explicit alternative
plans rather than renderer if-statements, CSS media queries, or arbitrary unit
expressions.

**Requirements**
- Add authored adaptive layout alternatives for width ranges, density modes,
  platform posture, and workspace/page posture.
- Express ranges through named host/posture facts and admitted measurement
  thresholds. Do not expose arbitrary `vh`, `vw`, percentage, or `calc(...)`
  unit soup as public adaptive truth.
- Lower alternatives into canonical layout facts with equivalence and impact
  metadata.
- Support common shell transformations such as rail expanded/collapsed,
  inspector docked/overlay, toolbar inline/overflow, and panel hidden/shown.
- Hot reload breakpoint or density rules and prove runtime swaps the affected
  layout plan without recompiling.
- Preserve stable identity and durable state across eligible adaptive
  alternatives.
- Admit adaptive alternatives through batch reports with typed denials for
  invalid ranges, density modes, platform posture, runtime posture, and state
  carry-forward eligibility.

**Warnings**
- Do not implement responsive behavior as renderer-local width checks.
- Do not model adaptive behavior as CSS media queries. Conditions are admitted
  runtime posture facts that choose between typed alternatives.
- Do not let percent, viewport, or calculated strings smuggle layout authority
  around measurement/adaptive schemas.
- Do not let adaptive alternatives become arbitrary hidden page maps.
- Do not preserve state across alternatives unless identity and eligibility
  prove it.
- Do not collapse density/theme changes into broad app rebuilds.

**Test requirements**
- Width/density/platform alternatives lower to deterministic canonical layout
  facts.
- Changing a breakpoint or density rule rebinds only consuming adaptive layout
  projections.
- Eligible focus, scroll, splitter, and selection state carries forward across
  alternatives with receipts.
- Ineligible state replacement/drop is explicit.
- Invalid adaptive declarations produce one report with denial sets, counters,
  digest, and receipt-derived presentation rows.

**Manual verification**
- Resize or otherwise trigger the adaptive proof.
- Change a breakpoint or density rule while the app is running.
- Confirm the rail, inspector, toolbar, or panel alternative changes live.
- Confirm evidence names adaptive-layout changed facts and state carry-forward
  receipts.
- Enter invalid breakpoint and density values in one save and confirm one
  adaptive admission report shows both denials.

**Engineering decisions**
- Adaptive layout is canonical authored meaning, not host-local layout code.
- This phase protects desktop product shells from becoming compile-bound when
  designers change responsive behavior.
- Adaptive alternatives reuse mosaic, flow, overlay, appearance, interaction,
  and motion facts.

**Open questions**
- Which platform postures can be honestly tested in the validation app before
  native integration broadens.

### Phase 40: Button Atom as Primitive Composition

This phase reintroduces the button only after the primitive families exist. The
button must be a composition of layout, content, appearance, interaction, and
motion receipts, not a special case that grows its own props forever.

**Requirements**
- Define `worth.component.button` as a component capability that consumes the
  shared primitive receipts.
- Express the centered submit button proof through authored surface, flow
  layout, content, appearance, interaction, and motion declarations.
- Express button anatomy as typed primitive composition, not as HTML-like tag
  structure, class bags, or generic child soup. A button consumes slots,
  content receipts, flow receipts, appearance receipts, interaction receipts,
  and motion receipts through declared contracts.
- Preserve the generic submit/click receipt path from Phase 32.
- Prove color, size, content, icon, payload, pressed state, cursor, and motion
  hot reload through shared primitive facts.
- Remove or quarantine any button-specific style plumbing that duplicates the
  primitive families.
- Button admission must aggregate the shared primitive family reports it
  depends on. It may summarize layout, content, appearance, interaction, cursor,
  hit-test, and motion denials, but it must not replace them with button-local
  messages.

**Warnings**
- Do not make button the primitive architecture.
- Do not reintroduce `button_*` prop explosions for appearance, content,
  cursor, or motion.
- Do not make button authoring look like `<button><span>...</span></button>`
  with classes. Worth UI authoring names component capability, primitive
  slots, content anatomy, recipes, and interactions directly.
- Do not allow the button renderer to choose spacing, icon/text treatment,
  cursor, or pressed styling.
- Do not preserve legacy compatibility paths from the proof scaffolding once
  the primitive composition path exists.

**Test requirements**
- Button proof consumes only shared primitive receipts for layout, content,
  appearance, interaction, and motion.
- Editing button color, size, content, icon, payload, pressed state, cursor,
  and transition changes the corresponding primitive facts.
- No button-specific renderer path interprets authored prop names.
- Invalid button composition that touches multiple primitive families reports
  all family denials through their original receipts and counters.
- Compile-fail coverage prevents app code from minting button interaction
  receipts directly.

**Manual verification**
- Render the centered submit button.
- Hot reload background, width/height or padding, icon choice, icon size, text,
  gap, pressed style, cursor, transition duration, and submit payload.
- Click the button after payload edits and confirm the next receipt carries
  active authored data.
- Enter invalid icon, pressed appearance, and submit payload declarations in
  one save and confirm the app shows structured family denials rather than a
  button-specific error.

**Engineering decisions**
- Button is the first serious atom proof, not the source of the architecture.
- This phase should delete temporary proof-only APIs that would otherwise
  fossilize into public shape.
- A new Rust button implementation still requires compile; authored
  composition over registered capability does not.

**Open questions**
- Whether button anatomy should be fully author-authored or use a small
  platform-default content slot recipe when authors only supply label and
  action.

### Phase 41: Cross-Component Reuse Proof

This phase proves the primitive families are universal by building a second
component family with the same receipts. A clickable row, card, or menu item
should be able to reuse the same content, appearance, interaction, cursor, hit
testing, and motion primitives without new local plumbing.

**Requirements**
- Build at least one second atom or small molecule, preferably a clickable row
  or card, using the same primitive families as the button.
- Reuse content declarations across button and the second component where
  possible.
- Reuse appearance recipes or state groups across button and the second
  component where possible.
- Prove nested interactions between the second component and an inner button or
  action using Phase 33 containment rules.
- Hot reload card/row background, radius, gap, cursor, nested icon, payload,
  and motion without new component-specific style fields.
- Cross-component proof surfaces must display shared primitive family admission
  reports when reused recipes fail, preserving the original schema identity and
  counters.

**Warnings**
- Do not declare success if the second component copies button-specific fields
  under new names.
- Do not let row/card behavior create a parallel interaction or event geometry
  lane.
- Do not make reuse only visual; interaction receipts and changed facts must
  reuse the same families too.
- Do not turn this into a broad component library milestone.

**Test requirements**
- The second component consumes the same primitive receipt types as the button.
- Shared content and appearance edits affect all declared consumers through
  dependency contracts.
- Nested button-inside-card or action-inside-row containment emits the correct
  receipts.
- Projection rebuild breadth remains bounded when only one shared primitive
  recipe changes.
- One invalid shared recipe consumed by both components produces one
  schema-owned denial basis with all affected consumers reported through
  dependency/rebind evidence, not duplicate component-local messages.

**Manual verification**
- Render the button and second component in the validation app.
- Hot reload one shared appearance recipe and confirm both consumers update.
- Hot reload a component-specific layout or content change and confirm only the
  intended consumer updates.
- Click nested and parent interaction zones and confirm receipts are distinct.
- Break a shared recipe and confirm both consumers preserve prior valid truth
  while the evidence shows one shared denial report and bounded consumer rows.

**Engineering decisions**
- This is the phase that proves the primitive stack is not button-local.
- Cross-component reuse replaces the old final-boss proof style with scalable
  complexity built one phase at a time.
- The milestone can broaden to Codex-style workspace slices only after this
  proof shows primitives compose honestly.

**Open questions**
- Whether the second component should be a row/card for collection readiness or
  a menu item for overlay/dropdown readiness.

## Must Ship

- common runtime change evidence over source, capability, Query, and state
  reload families
- sealed proof-widening types for raw observations, classified changes,
  admitted changed facts, admitted runtime change evidence, activated evidence,
  projection dependency contracts, admitted projection plans, rebind plans, and
  rebind receipts
- expanded runtime fact taxonomy for source, capability, Query, layout,
  content, shell, page, component, appearance, action, and durable-state changes
- common projection plan contract with admitted dependency contracts and
  equivalence basis
- runtime-owned projection rebind coordinator
- broadened runtime authoring snapshot derived from the existing source and
  artifact pipeline
- generalized capability reload family pipeline
- typed appearance and density values for color, font size, length, spacing,
  padding, border width, and shadow or elevation
- appearance, density, and component capability reload families with sealed
  changed-fact proof
- dropdown projection contracts for single-select and multi-select behavior
- Query-bound reload integration that consumes Query-owned posture rather than
  rebuilding local state
- native validation app proof slices using runtime receipts only, including the
  header/dropdown/style/page-slot manual acceptance path
- explicit semantic slice inventory and authority map covering authored,
  capability, Query, and runtime-owned hot-reload meaning
- canonical authored source-package ingress and typed authored-delta proof
- one canonical authored-delta lowering lane with no second interpretation path
- semantic changed-fact propagation that preserves Query-owned slices for live
  binding, async/result posture, projection facts, state snapshots, recovery,
  inspection, and virtualized frame targets
- authored-structural runtime fact families for shell, page, layout, content,
  mount, component-selection, primitive composition, and binding meaning where
  those belong to product authoring rather than compiled capability authority
- shared primitive authoring and runtime receipt families for surface, layout,
  container, content, appearance, interaction, motion, cursor, hit testing,
  overlay layout, collection layout, and adaptive layout
- Worth-native replacements for common CSS/HTML-shaped concerns: explicit
  appearance recipe binding instead of selectors/cascade, participation posture
  instead of display/visibility, scroll ownership instead of overflow, flow
  adjacency instead of margin, event policy instead of pointer-events,
  portal-host ordering instead of z-index, adaptive alternatives instead of
  media queries, and typed composition instead of tag/class soup
- schema-owned expected-value metadata for every authored primitive prop family,
  with typed denials carrying schema identity, source-span readiness, stable
  digest basis, counters, denial sets, and renderable receipt-derived guidance
- local flow layout primitives for row, column, stack, inline, grid, spacer,
  alignment, gap, padding, fit, and fill
- stateful appearance recipes for rest, hover, pressed, focus, disabled, and
  selected states with deterministic precedence and inheritance
- generic interaction declarations and sealed receipts for click, submit,
  command, toggle, open, and focus behavior
- cursor, hit-area, pointer-capture, and nested event containment contracts
- universal content primitives for text, icon, image, spacer, badge, divider,
  inline group, stack group, and slot where supported
- motion and transition recipes over admitted appearance fields, plus layout
  motion only where geometry reconciliation can be proven
- runtime portal host and anchored layout contracts for popovers, menus,
  dropdowns, tooltips, command palettes, modals, toasts, portal ordering,
  overlay focus, event policy, and dismissal behavior
- collection layout and virtualization contracts for list, grid, table-like,
  timeline, and chat-log surfaces with stable item identity and visible-range
  evidence
- adaptive layout alternatives for width, density, platform, and runtime
  posture with state carry-forward or replacement receipts
- authored product-structure hot reload over already-registered capability
  authority, including primitive composition, live remount, or reprojection
  proof without Rust edits
- delta-driven rebind phase selection over the touched graph rather than
  surface-local reload branching or file-family dispatch
- centered primitive validation proofs that scale phase by phase from simple
  visual facts through flow layout, state, interaction, event geometry,
  content, motion, overlays, collections, adaptive layout, button composition,
  and cross-component reuse
- per-family primitive admission reports that scale with the proof ladder:
  every new authored primitive family has schema certification, batch
  admission, denial sets, counters, unknown-key policy, source-span readiness,
  stable digest, and receipt-derived presentation rows
- button atom proof built from shared primitive receipts rather than
  button-specific style, content, cursor, or motion plumbing
- cross-component reuse proof showing the same primitive receipts drive a
  second row, card, or menu-item family
- compiler enforcement and compile-fail guards against local reload authority
- per-phase compile-fail guards beside each proof type, plus a final
  certification sweep that verifies every proof transition rejects weaker input
- reload storm, replay, and counter certification

## Must Preserve

- `WorthUiRuntimeHost` ownership of active artifact, active execution plan,
  capability snapshot, authoring snapshot, diagnostics, and activation state
- existing source -> artifact -> runtime proof chain
- Law 41 proof progression: values named evidence, receipt, admitted,
  prepared, activated, certified, validated, dependency contract, changed facts,
  rebind plan, active snapshot witness, or runtime witness cannot be publicly
  forged or skipped
- the aspect rule: dependency contracts say who consumes meaning, changed facts
  say what semantic slice changed, and later phases must not blur those jobs
- Query ownership of live, async/result, recovery, inspection, projection
  consumption, and support posture
- no second interpretation path that rescans authored files or lower artifacts
  after canonical authored-delta proof exists
- prior-valid runtime truth on denied, stale, unreadable, or invalid reloads
- renderer boundary as paint-only consumption of runtime receipts
- no CSS/HTML authority model: no selectors, cascade, specificity, class bags,
  pseudo-class state authority, display/visibility shortcuts, overflow
  shortcuts, pointer-events shortcuts, absolute/fixed positioning shortcuts,
  media-query shortcuts, z-index shortcuts, or tag-soup component composition
- no app-local dependency graph, hydration graph, reload state machine, shell
  map, page map, command map, component registry, dropdown mode authority,
  style map, or theme state
- built-in structural and primitive semantics stay platform-owned:
  region sizing, gap, padding or inset, scroll ownership, slot mounting,
  surface placement identity, flow layout contracts, content primitive
  contracts, appearance-state resolution, interaction semantics, cursor and
  hit-test arbitration, motion admission, runtime portal host ordering, overlay
  layout, collection layout, adaptive layout, and built-in component contracts
- app-authored product meaning stays app-owned:
  which already-registered built-ins are mounted, how primitives are composed,
  what authored content they display, which appearance recipes they consume,
  which interactions they emit, and what authored values they use
- no code-owned structural mount or projection authority where Milestone 4
  authoring truth should own the choice
- no component-specific style, content, cursor, event, or motion prop explosion
  where shared primitive receipts can express the meaning
- no renderer-local rejection prose, prop parsing, expected-value tables, or
  denial aggregation for primitive authoring; validation renderers may display
  typed reports and receipt presentations, but schema metadata and runtime
  admission reports own diagnostic expectations
- no local component callback, animation, overlay, portal ordering, collection,
  or responsive layout system beside the runtime-owned primitive families
- no per-frame source interpretation, registry string lookup, broad artifact
  scan, or broad projection rebuild hidden behind convenient APIs
- compile boundary between product meaning and platform meaning: new Rust
  component implementations, new primitive algorithms, new capability families,
  and new runtime subsystem behavior remain non-hot-reloadable

## Acceptance Evidence

- header, page-host, theme, command, command-projection, and at least one
  source-authored page/content projection rebind through one runtime change and
  projection coordinator path
- a running native validation app visibly hot reloads text, color, dropdown
  mode, page slot assignment, and one broader projection without restart
- the same native validation app visibly hot reloads header font size, row
  padding, container padding, shadow or elevation, and component-backed dropdown
  behavior through runtime receipts rather than app-local style or component
  state
- authored structural and primitive edits lower into exact touched runtime fact
  families rather than broad source-edit fallback
- Query-bound reload lowering preserves exact upstream Query slice granularity
  where available rather than collapsing to one coarse local Query change flag
- a running native validation app visibly repoints or remounts at least one
  live authored structure to another already-registered component or surface
  target through authored source only
- centered validation proofs visibly hot reload primitive text, padding,
  background, alignment, flow layout, state appearance, interaction payload,
  cursor, hit padding, content anatomy, motion, overlay placement, collection
  layout, adaptive alternatives, and button composition without restart
- centered validation proofs also visibly reject invalid authored primitive
  values without crashing, preserving prior runtime truth where applicable and
  showing expected syntax derived from the matching primitive schema
- every post-29 primitive proof includes a multi-denial manual and automated
  case where one save produces one runtime admission report with all denials,
  counters, stable digest, and receipt-derived presentation rows
- one mixed authored save changes structure and one unrelated primitive value
  in the same running app, with evidence proving only consuming projections
  rebuilt
- the visible validation app proves a rising proof ladder rather than saving
  primitive integration for one final boss
- denied, stale, equivalent, valid, and mixed reloads preserve runtime truth and
  produce typed evidence
- raw fact sets, raw dependency sets, raw reload requests, candidate snapshots,
  and classified-but-unadmitted changes cannot enter APIs that require changed
  facts, admitted dependency contracts, active snapshot witnesses, or activated
  rebind evidence
- projection rebuild breadth is bounded by changed-fact and dependency
  intersection, with counters proving the claim
- Query-bound reload evidence preserves Query-owned posture and does not create
  validation-app local status models
- compile-fail guards prevent app code from minting reload evidence,
  changed-fact proof, admitted projection plans, projection dependency
  contracts, rebind plans, projection receipts, direct Query dependency, or
  local shell, page, menu, theme, component, dropdown, or style authority
- compile-fail guards prove every proof-widening transition rejects skipped or
  out-of-order stages wherever the compiler can encode the distinction
- mixed product storm certification proves deterministic replay, bounded
  projection rebuilds, typed denied/equivalent/valid family evidence, and no
  steady-frame broad scans after activation
- certification makes explicit that brand-new Rust component implementations,
  primitive algorithms, capability families, or runtime subsystems still
  require compile because they change platform meaning, while authored
  composition over existing capability authority does not
- manual verification instructions are human-readable and describe what the
  tester should look for on screen, not just which internal receipt changed

## Sequencing Notes

- This side quest should land before expanding the Shopify dashboard proof
  beyond the current minimal native validation app slice.
- Milestone 4 remains the authoring and product-hardening milestone; this side
  quest clears the platform reload blocker that Milestone 4 exposed.
- Phases 1-10 have already migrated the existing header/page-host reload spine
  and established the first certification layer. Active implementation now
  resumes at Phase 11.
- The milestone is not complete after reload storm certification alone.
  Appearance, density, component, dropdown projection, and native manual
  verification phases are mandatory completion gates because they are named
  acceptance evidence, not future component-system polish.
- Phases 20-22 are an intentional catch-up pass on aspect semantics before
  later hardening. They exist to force explicit semantic slices, one canonical
  authored-delta lowering lane, and touched-graph propagation over exact facts
  before we harden the broader authored-truth reload path.
- Post-19 work must shift from reload-family completion into authored-truth
  completion. The remaining bar is not "more watched files"; it is runtime-owned
  authored structural reload over the Milestone 4 authoring hierarchy.
- If a later phase can only succeed by rediscovering meaning from raw authored
  files or by coarsening Query-owned slices into local umbrella facts, stop and
  fix the authority boundary instead of hardening the shortcut.
- Delay aggressive DX compression until the authored-truth proof is closed.
  Proof-first completion is the correct order because nice syntax over code-owned
  structural wiring would fossilize the wrong authority boundary.
- If a phase discovers that an existing lower artifact cannot express the needed
  changed facts or projection dependencies, the correct response is to widen the
  side quest and fix that lower structure before returning to dashboard breadth.
- Post-28 work is intentionally a primitive proof ladder instead of a single
  late final-boss shell. The point is to prove that Worth UI can author and hot
  reload reusable primitive families first, then compose recognizable workspace
  shapes such as a Codex-style shell from those families without renderer glue.

## Verification Discipline

- Run tests conservatively and widen only when the current proof slice is
  stable. This milestone has enough compile-fail, boundary, and validation-app
  coverage that broad `cargo test -p worth-ui` runs after every edit waste time
  without improving local signal.
- During active implementation, prefer the narrowest honest proof target:
  one touched unit or boundary test module, one touched compile-fail harness
  such as `runtime_reload_authority_compile`, one touched validation-app
  structural guard, or one touched native proof test.
- Do not use full workspace test runs as a first response to a local change in
  this milestone. Escalate breadth in this order only when the narrower slice is
  green and the change crossed another authority boundary:
  targeted unit or boundary tests, targeted compile-fail or structural guards,
  touched crate test run, then milestone-relevant full `worth-ui` verification.
- Broad `cargo test -p worth-ui` runs are reserved for slice closeout,
  cross-phase refactors, or pre-handoff verification. Full workspace runs are
  reserved for changes that genuinely cross crate boundaries or CI parity gates.
- When a change introduces or edits one proof-bearing type, the default
  verification should include the owning targeted behavior test and the owning
  skipped-stage or weaker-input compile-fail fixture before any broader suite.
- When a change is renderer-only, prove it through the owning renderer boundary
  tests and the nearest product-proof test before expanding into unrelated hot
  reload suites.
- When a change is validation-app-only, prove it through native-boundary guards
  and the touched visible proof flow before rerunning unrelated runtime
  certification suites.
- If broad runs become necessary, record why the narrower proof target was not
  enough. This milestone should teach future implementers to spend test time
  where authority actually changed.
