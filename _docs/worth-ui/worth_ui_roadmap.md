# Worth UI Future Roadmap

## Purpose

This document defines the future work for Worth UI.

It is a future-only roadmap. It does not treat Worth UI as a widget bundle or
an ornamental layer over `egui`. It exists to sequence the remaining work
required to turn Worth UI into a real desktop application platform with
hot-lowered iteration, canonical UI artifacts, Query-bound product surfaces,
platform-grade shell behavior, native desktop integration, and frame-efficient
execution lanes for both workbench UI and hostile real-time surfaces.

The governing rules remain:

- the canonical UI artifact is the source of runtime UI meaning
- compiled Rust defines capabilities; hot-reloadable UI source composes them
- file-authored UI and Rust-authored composition must converge on the same
  canonical artifact and execution-plan pipeline
- if `forge-query` already owns a stronger runtime-backed public lane for
  support or admission, typed bindings, projection consumption, async or result
  posture, recovery, inspection, explanation, or grouped/read/query products,
  Worth UI must consume that lane rather than rebuild a UI-local pseudo runtime
- app-shell behavior, interaction semantics, and execution plans must be
  platform-owned rather than app-local folklore
- semantic richness must lower before the hot path runs
- desktop UX, runtime honesty, and performance certification are all part of
  product completeness

Worth UI must remain strong enough for workbenches, editors, topology and CAD
tools, AI-native editing systems, operational apps, data-heavy consoles,
simulation tools, plugin-driven products, and real-time visualization or HUD
surfaces that need stronger guarantees than "egui plus some widgets."

## Current Roadmap Position

The current state is vision-first rather than product-complete.

The shipped baseline for Worth UI today is:

- the platform thesis captured in
  [worth-ui-vision.md](./worth-ui-vision.md)
- the DSL direction captured in
  [worth-ui-dsl-vision.md](./worth-ui-dsl-vision.md)
- the inspection and AI-diagnostics direction captured in
  [ai-diagnostics.md](./ai-diagnostics.md)
- the explicit decision to build above `egui` while keeping Worth-owned
  lowering, artifact, shell, interaction, and performance architecture
- the milestone ordering needed to avoid drifting into widget-first or
  application-local infrastructure before the platform foundations exist

This roadmap therefore tracks the work needed to turn the vision into a real
platform sequence.

## Roadmap Rules

Rules for every remaining Worth UI item:

- each milestone must describe a real platform capability, not just a component bundle
- each milestone must solve a structural problem before the dependent product features broaden
- each milestone must preserve the ownership boundary between Worth UI, Worth Query, the runtime bridge, truth/runtime authority, and lower native adapters
- each milestone must preserve hot-lowered composition, canonical UI artifacts, and no per-frame source interpretation
- each milestone that touches authored UI source, declaration artifacts, or
  runtime composition must preserve the DSL rule that authoring is a semantic
  language for canonical runtime declarations rather than a component,
  modifier, selector, or render-local widget language
- each milestone must treat the running Worth runtime as the primary host for
  hot reload, diagnostics, stable identity reconciliation, and safe plan swaps
- each milestone must preserve explicit accessibility, keyboard, focus, and diagnostics posture rather than treating them as polish
- each milestone must preserve frame-cost honesty through named counters and execution-plan boundaries
- frame-cost claims that cross diagnostic, report, or certification boundaries
  should lower Worth UI evidence into Forge Foundational performance claims,
  canonical bundles, counter-backed receipts, planned reports, and readiness
  envelopes instead of inventing local performance folklore
- each runtime milestone must ship the evidence, inspection, replay, and
  relevance surfaces required to explain the runtime families it introduces;
  explanation is not a late debug pass
- AI-facing inspection harnesses must arrive before, or at least alongside, the
  runtime families they need to inspect; a polished human inspector may arrive
  later, but formal AI entry points may not
- each milestone must preserve a structurally explicit layout model rather than drifting back toward DOM-shaped percentage, overflow, and implicit-parent folklore
- each milestone must define concrete acceptance evidence through platform scenarios, diagnostics artifacts, performance counters, replay-safe plan behavior, tooling evidence, or certification suites
- no milestone is complete until both implementation and trust evidence exist
- features that depend on stable identity, shell contracts, or interaction contracts must not ship before those foundations exist
- Worth UI must not become a second truth runtime, a second query runtime, or a web-runtime clone
- DSL sugar must follow admitted runtime lanes; authored syntax must not
  outrun declaration artifacts, aspect contracts, graph truth, measurement,
  Query binding, intent, services, or diagnostics support

## Foundation-First Critical Path

This section is the first build priority.

These are the milestones that determine whether Worth UI becomes a real
platform or a shallow collection of components:

- canonical lowering and artifact architecture
- hot composition runtime architecture
- frame-efficient execution lanes and cost certification
- app shell and command/interaction foundations
- Query-bound views and forms/workflow surfaces

If this section is weak, everything above it inherits the same failure mode:

- hot composition devolves into interpreted UI, Rust rebuild friction, or
  renderer-owned state carry-forward
- shell behavior becomes app-local glue
- components ship without coherent focus, accessibility, or runtime posture
- Query-bound surfaces drift back toward local caches and event plumbing
- performance claims remain vibes rather than certified contracts

## Milestone 1: Platform Skeleton, Facade, and Capability Registries

Detailed spec: [milestone-1.md](./milestone-1.md)

### Goal

Define Worth UI as one subsystem with a clean facade, stable vocabulary, and
mechanically visible capability boundaries before the rest of the platform is
built on top of ad hoc app-local abstractions.

### Must Ship

- the top-level Worth UI facade and crate topology
- explicit public vocabulary for UI source, canonical artifact, execution
  plan, capability registry, shell surface, command surface, view surface, and
  render surface
- capability registries for commands, components, domain-agnostic surfaces,
  mosaic region kinds, mosaic placement policies, mosaic sizing contracts,
  mosaic state slots, Query view bindings, runtime outcome projections,
  settings, task presentations, theme tokens, icons, command projections,
  plugin contribution slots, and native capability descriptors
- visibility boundaries that keep lower implementation topology private behind
  the facade
- typed registration contracts strong enough that later lowering can validate
  UI source without rediscovering platform meaning from strings or app-local
  code
- one small end-to-end registration path proving compiled Rust capabilities can
  be registered without hidden global state

### Must Preserve

- Worth UI remains above `egui` and does not fork or entangle low-level
  rendering ownership with platform ownership
- Worth UI does not become a second Query or truth runtime
- capability definition stays in compiled Rust rather than moving into
  untyped runtime source
- facade stability remains more important than internal topology convenience

### Acceptance Evidence

- one narrow public facade can build a minimal Worth UI app without deep
  imports into implementation modules
- adding a new registry family or lifecycle boundary forces explicit compiler
  updates at every construction site that must propagate it
- invalid capability registration shapes fail mechanically rather than through
  documentation or runtime folklore
- one audit pass can name the exact public types that own capability
  registration, lowering input, lowering output, and execution-plan input

## Milestone 2: Canonical UI Source, Lowering, and Runtime Artifact

Detailed spec: [milestone-2.md](./milestone-2.md)

### Goal

Make repo-authored UI source lower into one canonical runtime artifact so the
platform owns UI meaning explicitly before hot reload, shell work, or component
growth broaden the surface area.

### Must Ship

- codebase-authored UI source format for shell composition, panels, menus,
  toolbars, tables, inspectors, forms, tokens, and bindings
- Rust-native composition API or macro path that can emit the same canonical
  artifact input as file-authored UI source
- authored syntax shaped by [worth-ui-dsl-vision.md](./worth-ui-dsl-vision.md):
  semantic lanes, explicit declaration families, and no component-local
  modifier/selector/view-builder authority
- parser and validator that consume source and capability registries
- one canonical UI artifact carrying stable IDs, component references, command
  bindings, Query/view bindings, layout intent, accessibility metadata, and
  diagnostics
- typed rejection for invalid component IDs, invalid command IDs, mismatched
  bindings, missing required props, and illegal artifact structure
- source-level layout declarations strong enough to name mosaic regions,
  nesting, split or stack or overlay or pinned behavior, scroll ownership, and
  grow or shrink rules without DOM-style percentage-height ambiguity
- artifact inspection surfaces usable by later tooling and diagnostics
- at least one sample app expressed through the source -> artifact pipeline

### Must Preserve

- the artifact remains the source of runtime UI meaning once lowering
  completes
- file-authored source and Rust-authored composition do not fork artifact
  meaning, diagnostics, or execution planning
- Query-facing runtime surfaces referenced by the artifact remain Query-owned;
  Worth UI may bind, route, inspect, and present them, but must not recreate
  local query, result-state, recovery, or explanation models
- source parsing and validation do not leak into the steady-state frame path
- artifact meaning remains independent of diagnostics richness
- the lowering pipeline does not bypass capability registries or app facade
  boundaries
- the source format does not smuggle structure, layout, appearance,
  operability, intent, or service meaning through modifier order, selector
  reach, or render-local builder semantics

### Acceptance Evidence

- semantically identical UI source lowers to identical canonical artifact
  identity and structure
- intentionally different source lowers to mechanically different artifact
  identity or structure
- invalid source fails with structured diagnostics while preserving the prior
  valid artifact
- artifact inspection can explain what source bound to what registered
  capabilities without reading Rust control flow
- Rust-authored composition and file-authored source that declare the same UI
  meaning lower to equivalent artifact identity and structure
- semantically equivalent authoring does not depend on modifier order,
  selector precedence, or ambient builder context to lower correctly

## Milestone 3: Hot Composition Architecture Series

This roadmap milestone is now a series rather than one overloaded runtime lump.

It co-develops with [worth-ui-dsl-vision.md](./worth-ui-dsl-vision.md) and
[ai-diagnostics.md](./ai-diagnostics.md). The DSL is the semantic source
boundary for the same runtime architecture; the AI/inspection substrate is the
runtime explanation boundary for that architecture. Neither should evolve as a
separate folklore layer ahead of admitted runtime lanes.

The sequencing rule is:

```text
3.1 establishes inspection authority and the formal harness contract
later 3.x slices continuously enrich that harness with real runtime evidence
```

The sequence is intentional:

```text
meaning
-> identity
-> aspects
-> indexes
-> obligations
-> inspection evidence
-> measurement
-> execution plans
-> receipts
-> visual snapshots
-> observations
-> rebind
-> Query binding
-> intent
-> services
-> diagnostics
-> visual evaluation
-> AI inspection tools
-> inspector surface
-> certification
```

Hot reload is not a standalone feature milestone. It is the behavior of this
entire authority pipeline under source, Query, measurement, host-observation,
and world changes.

### Goal

Turn canonical Worth UI artifacts into a runtime-owned hot composition
substrate whose declaration, aspect contract, graph, indexes, obligations,
inspection evidence, measurement, execution plans, receipts, observations,
rebind, diagnostics, and certification are real before shell, forms, and
richer product UX broaden the surface area.

### Must Preserve Across The Series

- semantic richness stays in lowering-time artifacts, admitted contracts, graph
  topology, and execution plans rather than leaking into per-frame host code
- invalid reloads never blank, corrupt, or silently replace the last admitted
  mounted truth
- identity changes remain explicit replacement events rather than accidental
  state loss or tree-position folklore
- Query-facing runtime surfaces preserve Query-owned support, admission, live,
  async/result, projection, recovery, inspection, and explanation posture
- aspects and indexes remain part of correctness and proof, not a later
  optimization pass
- host adapters remain native-mechanics translators rather than semantic owners
- diagnostics, inspection, and certification remain typed runtime artifacts
  rather than logs, strings, or demo-only helpers
- the AI harness remains a first-class runtime consumer rather than a late
  screenshot-and-log convenience layer
- the human inspector remains a projection over runtime evidence rather than a
  second explanation system

### Milestone 3.1: Runtime Boundary Closure and Crate Split

This slice gives future runtime work the correct architectural home before more
behavior lands. It also establishes the formal inspection authority boundary so
AI and human inspection never have to scrape logs or invent a second
explanation runtime later.

**Must ship**

- target architectural split for:
  - `worth-ui`
  - `worth-ui-dsl`
  - `worth-ui-runtime`
  - `worth-ui-inspection`
  - `worth-ui-query-binding`
  - `worth-ui-host-contract`
  - `worth-ui-host-egui`
  - `worth-ui-certification`
- `worth-ui-host-contract` as the stable native-host boundary and
  `worth-ui-host-egui` as only the first adapter implementation rather than
  the permanent rendering substrate
- sealed inspection and harness contract types:
  - `UiInspectionTarget`
  - `UiInspectionQuery`
  - `UiInspectionScope`
  - `UiEvidenceBudget`
  - `UiEvidenceRichness`
  - `UiInspectionReceipt`
- one formal inspection facade entry point that accepts typed inspection
  queries and returns typed receipts, even while later milestones broaden the
  supported targets, scopes, and evidence families
- typed unsupported/not-yet-admitted posture for inspection targets or scopes
  whose backing runtime families have not landed yet
- support snapshot shape on the public Worth UI side
- hard-prohibition audit for obvious forbidden imports, folders, and deep
  adapter/runtime coupling
- hard-prohibition audit for renderer-local debug helpers, panel-local
  diagnostic truth, and host-bypass explanation paths

**Acceptance evidence**

- public facade exists without reopening Milestone 1 as generic platform
  skeleton work
- DSL ownership exists as a first-class crate boundary from the start instead
  of being scattered through facade or runtime modules
- inspection authority exists as a first-class runtime boundary from the start
- AI-facing and human-facing inspection must route through the same runtime
  inspection facade rather than separate ad hoc lanes
- unsupported inspection scopes fail through typed posture on the formal
  harness rather than through missing APIs or string errors
- host adapter cannot depend on runtime internals except through admitted host
  contracts
- a second host adapter can be introduced against `worth-ui-host-contract`
  without changing runtime truth ownership or moving host-neutral types into
  `worth-ui-host-egui`
- certification crate exists as the canonical anti-cheating home rather than
  test helpers drifting through unrelated modules

### Milestone 3.2: Canonical Declaration Artifacts and Aspect Contracts

This slice extends Milestone 2 from "canonical source lowers once" into
"runtime-owned declarations already carry the semantic contracts hot
composition needs later."

This is the first 3.x slice that must stay in lockstep with the DSL vision.

**Must ship**

- `UiDeclarationArtifact`
- `UiDeclarationIdentity`
- `UiDeclarationFamily`
- `UiAspectContract`
- support snapshot and support-row scaffolding for declaration families
- initial admitted declaration families:
  - `page`
  - `page-set`
  - `region`
  - `mosaic`
  - `local-composition`
  - `control`
  - `query-binding`
  - `intent`
  - `diagnostic-surface`

**Acceptance evidence**

- authored declaration lowers once into a canonical artifact with stable
  identity, family, aspect contract, topology, touch meaning, measurement
  policy, service usage, and Query binding posture
- no runtime phase rediscovering UI meaning from source text or renderer code
- aspect coverage report exists and can explain what semantic slices the
  declaration publishes
- declaration artifacts are rich enough to support lane-oriented DSL authoring
  without falling back to component-local meaning blobs

### Milestone 3.3: UI Authority Graph, Identity, Participation, and Core Indexes

This slice creates the runtime-owned graph that decides what exists, where it
lives, what participates, and how bounded lookup works.

**Must ship**

- `UiGraphNodeIdentity`
- `UiGraphSnapshot`
- parent/child/slot topology
- page/region/mosaic membership
- explicit presence and participation posture
- declaration-instance correspondence
- stable repeated-instance identity
- initial core indexes:
  - `graph node identity -> node`
  - `declaration identity -> graph node(s)`
  - `parent identity -> child set`
  - `slot identity -> occupant set`
  - `page identity -> participating node set`
  - `published aspect -> publishing node/receipt set`
  - `consumed aspect -> dependent node/receipt set`
  - `mounted receipt identity -> mounted receipt`

**Acceptance evidence**

- no recursive tree walk for ordinary lookup
- no tree-position identity
- graph mutation and index mutation are transactionally aligned
- participation axes stay explicit: mounted, visible, layout, hit-test, focus,
  accessibility, input, and diagnostic

### Milestone 3.4: Admission, Support, and Graph Touch Obligations

This slice closes the "who selects checks?" boundary. Callers declare touched
meaning; the runtime selects obligations.

**Must ship**

- `UiGraphTouchDescriptor`
- `UiSelectedObligationSet`
- `UiObligationDispatchPlan`
- `UiObligationVerdict`
- `UiAdmissionReport`
- world-aware support and admission posture
- initial obligation families:
  - `structural-legality`
  - `participation-legality`
  - `slot-contract`
  - `measurement-requirement`
  - `query-binding-requirement`
  - `intent-operability-requirement`
  - `diagnostic-surface-requirement`

**Acceptance evidence**

- declaration-change touches select obligations
- host-observation touches can select a different obligation neighborhood
- appearance-only touches do not trigger structure/focus obligations
- invalid structural edits deny before graph corruption
- denied admission preserves prior admitted mounted truth

### Milestone 3.5: Inspection Evidence Expansion and Relevance Indexes

This slice operationalizes the 3.1 inspection boundary by adding the first
substantial evidence families, relevance routing, and indexes over the runtime
truth already established by declarations, graph topology, and admission.

**Must ship**

- `UiEvidenceSlice`
- `UiRelevanceFilter`
- typed evidence families for:
  - declaration
  - admission
  - graph
  - aspects
  - obligations
- stable indexes for:
  - declaration identity -> evidence sets
  - source span -> declaration / admission evidence
  - graph node identity -> obligations / declaration / admission evidence
  - published aspect -> publishing nodes / receipts
  - consumed aspect -> dependent nodes / obligations / receipts
- the first formal AI harness path for targeted inspection by identity,
  source span, and scope before screenshot support exists

**Acceptance evidence**

- an agent can inspect a declaration artifact without receiving a giant dump
- an agent can inspect a graph node by identity and request only aspect-local
  evidence
- an agent can request `evidence_refs` before `materialized_detail`
- the 3.1 inspection facade can now answer real runtime questions instead of
  just carrying contract shape
- inspection responses emit typed receipts rather than strings or logs
- no renderer-local debug helper is required to explain declaration, admission,
  graph, or aspect posture

### Milestone 3.6: Measurement Semantics, Host Evidence Exchange, and Allocation Planning

This slice closes the planning side of measurement before continuous
interaction, committed allocation receipts, and churn-heavy replanning broaden
the runtime.

**Must ship**

- `UiMeasurementRequest`
- `UiMeasurementResult`
- `UiAllocationPlan`
- measurement vocabulary for:
  - `available-space`
  - `fixed`
  - `hug`
  - `fill`
  - `equal-share`
  - `min`
  - `max`
  - `bounded`
  - `content-measured`
  - `viewport-relative`
  - `scroll-owned`
  - `portal-anchored`
- typed host measurement-evidence exchange for intrinsic measurement, text
  measurement, viewport facts, and native sizing observations
- allocation-neighborhood planning semantics for parent/child constraint flow
  and sibling negotiation

**Acceptance evidence**

- host supplies measurement evidence only; it does not decide layout meaning
- intrinsic measurement stays evidence and does not become host-owned or
  cache-owned layout truth
- equivalent declaration + graph + measurement evidence inputs converge to the
  same allocation plan
- unsupported, cyclic, or not-yet-admitted measurement modes deny through typed
  posture instead of heuristic fallback

### Milestone 3.7: Allocation Receipts, Incremental Replanning, Scroll, Portal, And Continuous Interaction Measurement

This slice closes committed allocation truth and churn-heavy measurement
behavior after planning semantics already exist.

**Must ship**

- `UiAllocationReceipt`
- allocation equivalence and reuse basis
- invalidation and affected-neighborhood replanning rules for:
  - viewport resize
  - local resize and splitter drag
  - content growth
  - scroll-owned extent changes
  - portal-anchor changes
- committed runtime semantics for:
  - `viewport-relative`
  - `scroll-owned`
  - `portal-anchored`
- measurement inspection evidence and allocation closeout receipts strong enough
  for later mounting and rebind milestones

**Acceptance evidence**

- viewport resize enters as host observation and replans only the affected
  allocation neighborhood
- mosaic resize and local composition allocation use the same measurement lane
- scroll-owned and portal-anchored measurement remain runtime-owned rather than
  adapter-owned
- continuous resize and drag pressure stay bounded without broad unrelated
  replanning

### Milestone 3.8: Execution-Plan Lowering, Equivalence, and Frame-Cost Surfaces

This slice ensures execution consumes lowered plans instead of reconstructing
strategy from graph or declaration artifacts every frame.

**Must ship**

- execution-plan lowering from canonical declaration + graph + allocation into
  active runtime plans
- equivalence and no-op classification for candidate replacements
- compact runtime handles for commands, components, children, tokens, and view
  bindings
- named counters for reload/lowering work and steady-frame execution work
- Forge Foundational performance-envelope integration for shared claim
  vocabulary and certified claim bundles

**Acceptance evidence**

- identical canonical artifacts produce equivalent execution plans where the
  lane and capability set are unchanged
- equivalent replacements avoid needless plan swaps
- steady-state frame execution proves source parsing, artifact validation,
  registry string lookup, and broad artifact scans remain absent

### Milestone 3.9: Mounted Receipts and Host Contract

This slice closes the host boundary: host code may render and observe, but may
not own visible UI meaning.

**Must ship**

- `UiMountedNodeReceipt`
- `UiMountedFrameReceipt`
- mounted receipt facts for paint intent, clip/layer, allocation box, input
  participation, focus participation, hit-test participation, accessibility,
  motion projection, and diagnostic projection
- host contract for viewport, pointer, keyboard, focus, scroll, time/tick, and
  text-measurement observations

**Acceptance evidence**

- egui adapter renders only mounted receipts
- egui adapter reports observations only
- host cannot receive authored declarations directly
- host cannot decide visible/disabled/valid/layout meaning

### Milestone 3.10: Visual Snapshot Receipts and Hit-Test Identity Bridge

This slice makes screenshots, hit testing, and visible-region targeting
identity-backed runtime evidence instead of loose image bytes.

**Must ship**

- `UiVisualSnapshotReceipt`
- frame capture by identity
- node capture by identity
- region capture by identity
- hit-test region map
- visible mounted node overlay support
- `screen point -> mounted receipt identity`
- `screenshot region -> mounted receipt identity`
- `mounted receipt identity -> declaration / graph / evidence` bridge

**Acceptance evidence**

- the runtime can capture the current frame without creating a second visual
  truth path
- an agent can ask what node is under a pixel and receive mounted identity
- a screenshot region can be traced back to mounted receipt, graph node,
  declaration, and evidence
- screenshot support is tied to frame identity rather than loose PNG bytes

### Milestone 3.11: Observation Intake and Hot Rebind Planner

This slice makes hot reload real as bounded rebind rather than renderer
refresh.

**Must ship**

- `UiHostObservation`
- `UiRebindPlan`
- `UiRebindReceipt`
- changed-fact classification
- affected-aspect detection
- consumed-fact and consumed-aspect index lookup
- preserve/remount decisions
- invalidated measurement, binding, and obligation sets

**Must handle**

- source declaration edits
- viewport resize
- host measurement results
- Query-backed fact changes
- service-event changes

**Acceptance evidence**

- local source edits do not rebuild the whole page
- appearance changes do not invalidate structure
- layout changes do not invalidate Query binding unless declared
- resize invalidates allocation without broad graph rebind
- invalid hot edits preserve the last admitted mounted truth

### Milestone 3.11: Query Binding and Projection Consumption Substrate

This slice broadens Milestone 2's declared binding references into a minimal
runtime binding substrate, but not yet the full product surface richness of
Milestone 6.

**Must ship**

- `UiProjectionBinding`
- `UiProjectionFactReceipt`
- schema/view-shape binding posture
- projected scalar value and projected collection lanes
- binding invalidation
- payload-shape requirement posture
- minimum binding postures:
  - `ready`
  - `pending`
  - `current`
  - `stale`
  - `revalidating`
  - `denied`
  - `unsupported`
  - `schema-mismatch`
  - `wrong-world`
  - `rebind-required`

**Acceptance evidence**

- selected-inspector fields come from Query projection facts
- schema-swap rebinding preserves compatible field identity where admitted
- invalid schema/payload posture emits typed mounted diagnostics
- no local loading/error enum replaces Query posture
- no renderer-side query builder exists

### Milestone 3.12: Intent, Operability, and Interaction Substrate

This slice turns host observations into runtime-routed intents instead of
widget callbacks.

**Must ship**

- `UiIntentDeclaration`
- `UiIntentAdmission`
- interaction families for:
  - `click`
  - `edit`
  - `select`
  - `submit`
  - `navigate-page`
  - `change-mosaic`
  - `open-portal`
  - `close-portal`
  - `invoke-command`
- operability postures for:
  - `operable`
  - `disabled`
  - `readonly`
  - `pending`
  - `denied`
  - `unsupported`
  - `wrong-world`
  - `stale`
  - `rebind-required`
  - `requires-confirmation`

**Acceptance evidence**

- controls do not own callback meaning
- submit payloads are assembled from runtime/control projection rather than
  renderer code
- disabled/readiness posture comes from runtime authority
- click success is not treated as mutation success
- invalid submits emit typed intent denials

### Milestone 3.13: Runtime Services Foundation

This slice closes the cross-cutting service lanes that the certification
vertical slice depends on.

**Must ship**

- first-class service lanes for:
  - `portal`
  - `focus`
  - `motion`
  - `command-routing`
  - `scroll`
  - `selection`
- minimal admitted service path for each family rather than feature-broad
  product richness

**Acceptance evidence**

- dropdowns open through the portal service with logical owner, anchor, layer
  posture, measurement plan, and focus/dismissal rules
- focus scopes, participant sets, route requests, host focus observations, and
  runtime focus receipts are real runtime artifacts
- motion projections derive from previous receipt + next receipt + motion
  declaration rather than host-local animation meaning

### Milestone 3.14: Diagnostics, Inspection, and Evidence Closure

This slice makes denials, support gaps, and rebind decisions typed, mountable,
and inspectable instead of spooky.

**Must ship**

- `UiDiagnosticArtifact`
- `UiCausalInspectionReport`
- diagnostic identity, stop class, world, source declaration identity,
  affected graph identity, selected obligations, admission evidence, binding
  evidence, measurement evidence, host capability evidence, rebind evidence,
  aspect-fit denials, and aspect-coverage reports

**Acceptance evidence**

- diagnostics mount through the mounted receipt path
- no string-only errors
- tests do not match diagnostic messages
- unsupported vs denied vs stale vs wrong-world vs rebind-required remain
  distinct
- inspection explains why a node rebound, preserved, remounted, or denied

### Milestone 3.15: Visual Geometry, Design Invariants, and Perceptual Inspection

This slice lets the runtime answer alignment, spacing, symmetry, and visual
consistency questions from receipt-backed geometry first and screenshot pixels
second.

**Must ship**

- `UiTextRunReceipt`
- `UiTextBaselineReceipt`
- `UiGlyphBoundsReceipt`
- `UiVisualBoundsReceipt`
- `UiVisualAnchor`
- `UiAlignmentGroup`
- `UiSpacingGroup`
- `UiSymmetryAxis`
- `UiVisualInvariantDeclaration`
- `UiVisualEvaluationQuery`
- `UiVisualEvaluationReport`
- `UiVisualFinding`
- `UiVisualOverlayReceipt`
- declared invariant, advisory, and ad hoc inspection levels

**Acceptance evidence**

- the runtime can answer whether two labels align by baseline or leading edge
- the runtime can evaluate spacing rhythm or equal-width allocation without
  pixel-only guessing
- visual findings can link back to mounted receipt, graph node, declaration,
  and source span
- screenshot-confirmed visual inspection remains secondary to receipt-backed
  geometry rather than replacing it

### Milestone 3.16: AI Agent Inspection Tools and Replay Protocol

This slice turns the evidence substrate into a real agent-facing repair and
inspection interface.

**Must ship**

- formal AI tool registry over the runtime inspection substrate
- screenshot/frame capture tool
- inspect-target tool
- inspect-at-point tool
- diagnostic-relevance tool
- frame-diff tool
- rebind-explanation tool
- replay session, replay cursor, and stop-point selection
- replay stop points for parse, semantic lowering, declaration artifact,
  admission, graph touch, obligation selection, Query binding, measurement,
  mounted receipts, host observations, rebind planning, and diagnostics

**Acceptance evidence**

- an agent can debug a failed hot reload without reading giant dumps
- an agent can request only diagnostics relevant to a selected node, source
  edit, or denied rebind
- an agent can replay the last edit to the first denial point
- an agent can compare before/after frames or receipts by aspect scope

### Milestone 3.17: Worth Inspector Surface

This slice adds the human-facing runtime inspector as a projection over the
same evidence substrate the AI harness already uses.

**Must ship**

- visual tree view
- authority graph view
- aspect inspector
- rebind timeline
- measurement inspector
- Query binding inspector
- services inspector
- diagnostics feed
- replay timeline
- visual evaluation view

**Acceptance evidence**

- the inspector consumes the same evidence substrate as the AI tools
- clicking a visible node can navigate to declaration, graph, receipt, and
  diagnostic evidence
- the inspector can be authored through Worth UI where feasible
- the inspector does not become the source of diagnostic truth

### Milestone 3.18: Hot Composition Certification Vertical Slice

This slice is certification, not product scope broadening. It proves the
runtime architecture through one hostile, realistic workflow.

**Scenario**

```text
Workflow Editor Page
  left: step list
  center: workflow graph canvas
  right: selected step inspector
```

**Must prove**

- hot page/mosaic edits change graph topology through declarations
- resize invalidates allocation without broad unrelated rebind
- selected-step schema comes from Query projection facts
- field swaps preserve compatible identity where admitted
- dropdowns open through the runtime portal service
- focus is routed through the runtime focus service
- motion uses the runtime motion service
- submit payloads are admitted against Query/schema posture
- invalid payloads emit mounted diagnostics
- host adapters never decide disabled/visible/valid meaning
- receipts prove bounded rebind
- unrelated nodes remain untouched

### Acceptance Evidence For The Series

- the same running app can accept valid replacement declarations produced from
  file-authored UI or Rust-authored composition through the same activation and
  rebind pipeline
- equivalent replacements classify as no-op where the series says they should
- valid reloads preserve eligible durable state and explicitly replace or drop
  ineligible state
- invalid reloads preserve the previous admitted mounted truth while surfacing
  typed diagnostics
- execution-plan lowering, mounted receipts, observation intake, Query binding,
  intent routing, service routing, and diagnostics all prove their work through
  runtime-owned artifacts instead of renderer-local helpers
- steady-state frame execution proves source parsing, artifact validation,
  registry string lookup, and broad artifact scans remain absent

### Sequencing Notes

- Milestone 3.1 through 3.18 replace the old single Milestone 3 runtime lump
  with a narrower authority-first sequence
- `ai-diagnostics.md` co-develops across the full 3.x series; each runtime
  family must become inspectable as it lands instead of waiting for the end
- the formal AI inspection harness begins in Milestone 3.1; later milestones
  enrich it with real evidence families, visual capture, replay, and inspector
  projections
- the DSL vision must co-develop with Milestone 2 and Milestone 3.2 through
  3.17; sugar follows admitted runtime lanes instead of running ahead of them
- Milestones 4 through 7 now build on this substrate instead of reopening
  runtime authority, layout truth, Query posture, or interaction ownership
- detailed specs should split into milestone-3.x docs as each slice begins
  rather than trying to keep one giant Milestone 3 spec honest

## Milestone 4: Application Shell and Workspace Layout

### Goal

Build real desktop shell products on top of the Milestone 3.x hot composition
substrate by owning the app shell, workspace model, and persisted layout
semantics rather than leaving them to every downstream application.

### Must Ship

- mosaic as the primary structural layout model for shell and page composition
- multi-window application model
- nested mosaic regions that can split, stack, overlay, and pin
- region-level sizing contracts such as fixed, fill, ratio, bounded, and hug
- explicit scroll ownership and grow-then-scroll behavior at the region level
- dock, split, tab, sidebar, bottom-panel, and status-surface layout primitives
  expressed through or alongside the mosaic model where appropriate
- persisted workspace layout and restore semantics
- menu bar, toolbar, command palette, context menu, dialog, and modal-sheet
  shell surfaces
- active document, active panel, and active window routing contracts
- enough shell polish that one real workbench-style app can be built without
  custom layout infrastructure

### Must Preserve

- workspace layout remains a platform artifact, not a pile of widget-local
  geometry state
- mosaic remains the structural space-allocation language rather than
  collapsing into a grab bag of unrelated layout models for ordinary shell work
- shell meaning remains command-routed and identity-stable across reloads and
  restore flows
- shell does not force app authors to choose between multi-window support and
  hot-lowered composition
- persisted shell state remains distinct from authoritative runtime truth
- Milestone 4 must consume the page, mosaic, measurement, receipt, and rebind
  substrate from Milestone 3.x rather than reopening renderer-owned layout or
  shell-local topology state

### Acceptance Evidence

- one sample workbench can open, close, dock, split, tab, persist, and restore
  its shell without app-local shell logic
- one nested mosaic shell can express pinned sidebar, stacked scroll regions,
  and overlay surfaces without DOM-style height or overflow hacks
- shell state restore is deterministic enough that restart and recovery do not
  invent layout drift
- command palette, menus, and context surfaces all project the same command
  backbone
- workspace layout edits can survive hot reload when stable IDs remain intact

## Milestone 5: Command Spine, Focus, Selection, and Keyboard Routing

### Goal

Broaden the interaction substrate closed in Milestones 3.12 and 3.13 so
actions, focus, selection, and keyboard workflows become rich platform
semantics instead of widget-local conventions.

### Must Ship

- canonical command registry with stable identifiers, labels, shortcuts, icons,
  and readiness or posture hooks
- focus model and traversal rules for shell, widgets, dialogs, and command
  surfaces
- selection primitives strong enough for tables, trees, inspectors, canvases,
  and multi-panel workflows
- keyboard routing and shortcut conflict handling
- undo and redo presentation surfaces tied to command identity
- command-projection surfaces for menu items, toolbar items, palette entries,
  and context actions

### Must Preserve

- command meaning stays canonical across all projections
- focus and selection state remain identity-bound rather than implicit widget
  side effects
- keyboard ergonomics do not bypass accessibility semantics
- command readiness does not collapse into generic booleans when runtime
  posture can remain structured
- Milestone 5 must deepen command, focus, selection, and keyboard richness on
  the admitted interaction and service substrate rather than reintroducing
  host-local callback routing or native-widget focus truth

### Acceptance Evidence

- the same command can be invoked consistently through button, menu, palette,
  shortcut, and context-entry surfaces
- focus traversal and selection behavior remain deterministic across reload,
  restore, and multi-window flows
- command conflicts and invalid routes surface structured diagnostics rather
  than silent precedence accidents
- undo and redo presentation can name what action is being reversed or replayed

## Milestone 6: Query-Bound Views and Live Surface Binding

### Goal

Broaden the minimal Query binding and projection-consumption substrate from
Milestone 3.11 into serious data surfaces that bind to declared Query meaning
instead of app-local caches, host-shaped events, or widget-owned live-update
folklore.

### Must Ship

- table or grid surface bound to collection queries with typed columns,
  ordering, cursor semantics, virtualization, and query-shaped patches
- tree or graph-navigation surface bound to bounded traversal or grouped
  neighborhood semantics
- inspector or detail surface bound to detail and inspector view shapes
- timeline or history surface bound to historical and diff-capable Query views
- live view binding that promotes one-shot declared meaning into ongoing
  runtime-backed delivery without raw event handling in the widget
- typed surface contracts for selection, sort, filter, and visible-state
  bindings that remain subordinate to Query meaning rather than replacing it

### Must Preserve

- Worth UI does not become the owner of query legality, basis semantics, or
  truth authority
- table, detail, grouped, timeline, and inspector semantics, plus query
  planning, saved-query meaning, projection consumption, and typed fact
  receipts remain Query-owned runtime lanes rather than UI-local data-source
  abstractions
- live updates remain query-shaped rather than raw CDC or raw widget events
- view surfaces remain honest about policy masks, unsupported families, denied
  basis combinations, and deferred capability rows
- app authors do not need local caches to keep core surfaces usable
- Milestone 6 must build richer table, tree, inspector, and timeline surfaces
  on the admitted binding substrate instead of reopening local Query clones,
  local result-state models, or renderer-owned live semantics

### Acceptance Evidence

- one table, one tree, and one inspector surface can be driven from declared
  Query meaning with live updates and no app-local cache repair layer
- equivalent declared views receive equivalent live update behavior across
  reload and restart boundaries where the runtime supports it
- unsupported or denied view bindings fail explicitly and typed
- Query-bound surfaces can explain what view meaning, basis, or runtime posture
  they are currently presenting

## Milestone 7: Forms, Validation, and Editing Workflows

### Goal

Broaden the interaction, Query-binding, and diagnostic substrate from
Milestones 3.11, 3.12, and 3.14 so forms and editing become a platform
capability rather than a loose pile of input widgets, local booleans, and
submission folklore.

### Must Ship

- form surface model with field binding, draft values, dirty or touched state,
  reset or revert behavior, and validation presentation
- local validation and runtime-backed validation or admission presentation
- typed submit, retry, cancel, and reset flows
- inline, grouped, and panel-level error presentation
- editing flows that can participate in Query-bound detail or inspector
  surfaces without collapsing into local truth ownership
- one editing example proving that a serious form can be built without app-local
  form framework code

### Must Preserve

- forms remain subordinate to platform interaction and runtime posture models
- draft or editing state does not masquerade as authoritative truth
- runtime validation or admission stays structured rather than flattened into
  one error string or one boolean
- async or result-state posture, recovery, preview, and ordinary outcome
  semantics compose with existing Query/runtime lanes rather than a Worth-UI-
  owned form status model
- form behavior remains accessible, keyboard-usable, and hot-reload-safe
- Milestone 7 must consume the runtime-owned intent, operability, binding, and
  evidence substrate instead of inventing a Worth-UI-local form framework above
  it

### Acceptance Evidence

- one nontrivial form supports edit, validation, submit, retry, reset, and
  runtime error presentation without custom app-local form infrastructure
- local validation and runtime-backed validation remain distinguishable in the
  UI and diagnostics
- form reloads preserve the stable state they should and explicitly replace the
  state they should not preserve
- form submission results can surface structured success, advisory, violation,
  and recoverable outcomes

## Milestone 8: Runtime UX, Preview, Recovery, and Explanation

### Goal

Make runtime posture visible as ordinary UX so Worth UI can present previews,
recoverable failures, structured denials, and explanations without app-local
status folklore.

### Must Ship

- structured runtime-state surfaces for loading, denied, advisory, violation,
  stopped, recoverable, stale, failed, and completed outcomes
- preview, before/after, accept/discard, and commit-oriented workflow surfaces
- runtime explanation and inspection panels for command readiness, view
  posture, mutation evidence, and cross-runtime "why" answers where admitted
- recovery affordances and recovery-surface presentation for typed failures
- one end-to-end preview workflow showing staged or speculative state, review,
  acceptance, and discard
- enough reusable runtime UX that downstream apps do not have to invent their
  own runtime-state taxonomy

### Must Preserve

- Worth UI does not invent a second mutation, recovery, or explanation runtime
- runtime posture stays structured through the UI boundary
- preview and staged states remain distinct from authoritative truth
- recovery briefs, async or result-state posture, projection-consumption facts,
  Query inspection, and cross-runtime causal explanation remain runtime-owned
  contracts that Worth UI presents rather than redefines
- richer diagnostics do not change the operational outcome being presented

### Acceptance Evidence

- the same structured runtime posture can be presented in a button, panel, form,
  preview flow, and diagnostics surface without reinterpretation
- preview flows can be abandoned without authoritative residue
- explanation surfaces can answer what command or view was bound, why it was
  denied or advised, and what recovery path exists where admitted
- app authors can build one nontrivial review or recovery flow without creating
  a second status model above the runtime

## Milestone 9: Design System and Professional Component Set

### Goal

Turn Worth UI from shell-plus-binding infrastructure into a coherent,
production-grade component system that downstream products can use without
rebuilding basic desktop UX.

### Must Ship

- semantic token system for surfaces, text, borders, accent, warning, danger,
  success, selection, focus, overlays, and runtime states
- light, dark, high-contrast, and custom-theme support
- density modes and component-state contracts
- layout composition semantics for seam ownership, spacing resolution,
  scroll-container behavior, resize behavior, and region-edge behavior within
  the mosaic model
- professional workbench component set including tables, lists, trees,
  inspectors, split panes, tab bars, toolbars, searchable selects, breadcrumbs,
  notifications, logs, progress views, and file or project browser surfaces
- visual-state consistency rules across the component library
- one sample app proving components compose into a product rather than a demo
  gallery

### Must Preserve

- component growth must not outrun shell, interaction, or accessibility
  foundations
- semantic tokens remain meaning-bearing rather than raw color piles
- specialized inner surfaces such as forms, tables, lists, trees, and canvases
  remain region content inside the mosaic structure rather than forcing the
  mosaic model to impersonate every inner content layout
- components remain compatible with hot-lowered composition and stable identity
- workbench-grade components stay stronger than ornamental one-off widgets

### Acceptance Evidence

- a realistic data-heavy workbench can be composed from the shipped component
  set without major custom infrastructure
- theme and density changes propagate coherently through the same component set
- layout composition rules are strong enough that common shell and page layouts
  do not require margin or padding folklore, percentage-height hacks, or
  accidental overflow behavior
- components preserve focus, keyboard, accessibility, and runtime-state
  semantics under real product composition
- component surfaces remain narrow enough that platform tooling can inspect them

## Milestone 10: Canvas, Spatial, and Real-Time Product Surfaces

### Goal

Build product-grade canvas, spatial-tool, real-time overlay, and
renderer-integrated UI surfaces on top of the execution lanes and frame-cost
certification established in Milestone 3.

### Must Ship

- canvas and spatial product primitives with pan, zoom, hit testing, overlays,
  snapping, tool state, and command integration
- renderer-facing product surfaces for custom draw passes and world or screen
  projection work
- real-time overlay and HUD product primitives with shader or material-backed
  surfaces
- higher-level tool-state, selection, overlay, and command workflows over the
  Milestone 3 lane substrate
- one sample hostile surface proving platform shell, diagnostics, and runtime
  binding can coexist with a high-frequency render surface
- expanded performance counters and certification scenarios for real-time and
  spatial product workflows

### Must Preserve

- Worth UI does not attempt to own the full volatile scene renderer
- spatial and real-time lanes remain semantically integrated with commands,
  views, and inspection rather than becoming a disconnected side runtime
- performance claims remain counter-backed instead of anecdotal
- hostile product surfaces consume the Milestone 3 execution lanes rather than
  redefining lane mechanics

### Acceptance Evidence

- one canvas-like product surface and one real-time overlay prove the lane
  substrate can support real product interaction
- UI structure remains hot-reloadable while the render surface maintains
  specialized mechanics
- frame counters expose where work is spent on spatial and real-time surfaces
- renderer-integrated surfaces still participate in platform command, focus,
  and diagnostics systems where applicable

## Milestone 11: Persistence, Settings, and Document or Project Workflows

### Goal

Finish the user-visible persistence model so settings, layout, projects,
documents, and recovery state become platform capabilities rather than app-local
storage conventions.

### Must Ship

- typed settings model with user, workspace, and project scopes
- settings-surface composition strong enough for real settings panels
- persisted workspace layout, recent files, and recent project flows
- document or project workflows with dirty tracking, autosave, and recovery
  snapshots
- restore semantics for panels, tabs, and relevant in-progress UI state
- migration posture for persisted platform state

### Must Preserve

- persisted UI and project state remain separate from authoritative domain
  truth unless explicitly routed through runtime contracts
- restore behavior remains deterministic rather than best-effort folklore
- settings do not become an untyped bag disconnected from the platform facade
- autosave and recovery behavior remain explicit rather than ambient

### Acceptance Evidence

- one sample app can restore layout, tabs, settings, and recent project context
  without app-local persistence plumbing
- autosave and recovery snapshots restore a meaningful working session after a
  forced interruption
- settings remain typed and scoped through the same platform contracts used by
  UI composition
- persistence migrations fail explicitly when incompatible rather than drifting
  silently

## Milestone 12: Background Tasks, Diagnostics, and Recovery Tooling

### Goal

Make long-running work, operational diagnostics, and supportability visible as
platform behavior so desktop apps do not freeze, hide failures, or depend on
log archaeology.

### Must Ship

- task model for progress, cancellation, retry, completion, and failure
- status-bar, panel, and notification surfaces for task presentation
- diagnostics surfaces for errors, traces, command history, task history, and
  performance panels
- recovery presentation for task or workflow failures where recovery exists
- one support-oriented diagnostic flow proving a user or developer can inspect
  a failure without raw logs
- one performance panel or profiling surface consuming the platform's named
  counters

### Must Preserve

- task state remains distinct from authoritative truth
- diagnostics richness does not change the task or workflow result
- support surfaces remain platform-native rather than special-purpose app code
- recovery actions remain explicit instead of silent retries or hidden cleanup

### Acceptance Evidence

- one long-running task can be started, observed, cancelled, retried, and
  diagnosed through platform surfaces alone
- one failure can be explained through structured diagnostics without scraping
  implementation logs
- counters and traces remain connected back to named platform operations
- app authors can expose support-grade diagnostics without inventing a second
  infrastructure layer

## Milestone 13: Accessibility and Interaction Quality Completion

### Goal

Close the accessibility and interaction-quality bar as a real product
completion milestone rather than a deferred compliance sweep.

### Must Ship

- accessible roles, names, descriptions, and state semantics across the core
  component and shell set
- focus visibility, keyboard traversal, reduced motion, scaling, contrast, and
  comfort rules enforced through platform behavior
- accessibility inspection tooling sufficient to audit platform surfaces
- screen-reader and keyboard-only support path for core product patterns
- one hostile accessibility pass over shell, command, form, and view surfaces

### Must Preserve

- accessibility remains built into platform primitives rather than layered on
  as opt-in app-local metadata
- keyboard ergonomics and accessibility semantics reinforce rather than fight
  each other
- accessibility completion does not fork the component model into a "special"
  accessibility-only path
- quality rules remain compatible with hot reload and high-density product use

### Acceptance Evidence

- core shell, command, form, table, tree, and inspector surfaces pass a named
  accessibility audit path
- keyboard-only flows remain usable across the same scenarios
- accessibility tooling can inspect platform-generated semantics without
  implementation archaeology
- contrast, scaling, and reduced-motion rules are proven in real sample apps

## Milestone 14: Native Platform Integration and Delivery

### Goal

Make Worth UI shippable as real desktop software with strong platform
integration and delivery mechanics instead of stopping at "the app runs on my
machine."

### Must Ship

- native menus, dialogs, notifications, clipboard, drag and drop, tray, and OS
  theme integration adapters
- file associations, URL handlers, single-instance behavior, and app metadata
  surfaces
- packaging, installers, update-channel support, crash capture, and session
  restore infrastructure
- keychain or credential integration and explicit permission surfaces where the
  platform owns them
- enough release and runtime behavior that one real app can be packaged and
  maintained through platform tooling

### Must Preserve

- native integration remains adapter-shaped and explicit rather than ambient
  host knowledge spread through the app layer
- packaging and delivery work do not redefine app shell or runtime semantics
- crash and update infrastructure remain distinct from authoritative truth
- platform differences stay behind named boundaries rather than leaking across
  app code

### Acceptance Evidence

- one sample app can be packaged and launched with native integration features
  working through platform adapters
- restart after update or crash can restore enough state to feel like a real
  desktop product
- native integration failures surface explicitly and diagnosably
- delivery surfaces remain stable enough to support real release channels

## Milestone 15: Plugin and Extension Architecture

### Goal

Let Worth UI apps grow into platforms without collapsing their shell, runtime
honesty, or security model under extension pressure.

### Must Ship

- typed plugin contribution points for commands, panels, inspectors, settings,
  query views, themes, toolbars, and project templates
- capability and permission model for filesystem, network, credentials,
  commands, panels, runtime mutation, and project access
- runtime-aware extension hooks that consume platform commands, views, and
  structured outcomes instead of lower-runtime internals
- inspection surfaces showing what each plugin contributed and why
- one sample plugin host proving multiple extensions can coexist through the
  same platform contracts

### Must Preserve

- plugin power remains capability-bounded and inspectable
- extensions do not bypass Query-facing or command-facing platform surfaces
- host apps retain shell, accessibility, and diagnostics coherence under
  extension growth
- plugin contribution points remain part of the public platform facade rather
  than deep imports into internals

### Acceptance Evidence

- one host can load multiple plugins that contribute commands, panels, or views
  through typed contribution points without custom per-plugin glue
- capability violations fail explicitly and typed
- plugin-contributed surfaces still participate in focus, accessibility, theme,
  and diagnostics systems
- host inspection can explain what a plugin added and what authority it holds

## Milestone 16: Developer Tooling, Templates, and Platform Inspection

### Goal

Make Worth UI teachable, inspectable, and self-hosting enough that teams can
understand the platform visually instead of learning it through source diving
alone.

### Must Ship

- component gallery
- theme editor
- layout debugger
- command registry inspector
- accessibility inspector
- Query or view inspector
- profiler or frame-counter inspection surface
- screenshot-test harness
- sample templates for workbench, data app, graph editor, runtime inspector,
  dashboard, and plugin host shapes
- one end-to-end platform inspection story that uses the same runtime artifacts
  the platform itself owns

### Must Preserve

- tooling consumes canonical platform artifacts rather than shadow metadata
- templates remain examples of real platform usage rather than special internal
  paths
- inspection surfaces remain diagnostic and educational rather than becoming a
  second imperative editing runtime
- tooling breadth does not dilute facade clarity or runtime ownership

### Acceptance Evidence

- a new team can start from a template and stay within the ordinary platform
  path
- platform tooling can explain what a shell, command, artifact, view, or plan
  is doing without source spelunking
- screenshot and inspection tooling can certify real product examples
- sample apps expose roadmap gaps honestly rather than hiding them

## Milestone 17: Worth UI Certification Program

### Goal

Run the full platform certification pass after the remaining product milestones
exist, and create the missing Worth UI test-requirements contract if it does
not yet exist.

### Must Ship

- `_docs/worth-ui/test-requirements.md` as the authoritative acceptance source
  if it does not already exist by this milestone
- named certification suites for hot reload, stable identity carry-forward,
  shell restore, Query-bound view parity, forms and validation behavior,
  preview or recovery flows, accessibility, native integration, plugin
  isolation, and frame-budget certification
- canonical machine-checkable artifact bundles for certification results where
  the platform claims structured proof rather than visual impressions
- explicit distinction between verified platform paths and intentional debt
  paths in certification output

### Must Preserve

- certification remains a proof program, not a feature-discovery bucket
- the platform is judged against its declared ownership boundaries rather than
  app-local workarounds
- end-to-end claims remain tied back to named counters, artifact identities,
  and structured outcomes
- hostile cases remain part of the bar rather than optional extended tests

### Acceptance Evidence

- every prior milestone has named certification coverage, either through the
  Worth UI requirements doc or through milestone-native acceptance programs
- certification runs can prove that hot-lowered composition, platform shell,
  Query-bound views, forms, accessibility, native integration, plugins, and
  frame-budget surfaces behave according to the roadmap claims
- certification artifacts are sufficient for offline review without leaning on
  private host memory or narrative-only logs
- all declared high-value debt paths are either verified closed or still
  explicitly marked as debt

## Per-Milestone Format

For consistency and readability, every milestone in this roadmap uses the same
shape:

- `Goal`
- `Must Ship`
- `Must Preserve`
- `Acceptance Evidence`

## Completion Standard

Worth UI is roadmap-complete only when:

- all foundation-first critical-path milestones are shipped
- all product-complete platform milestones are shipped
- hot-lowered composition, stable identity, and execution-plan swaps are
  proven under hostile edit and reload scenarios
- frame-cost counters and performance certification exist for high-frequency
  surfaces rather than only best-effort profiling
- shell, command, focus, accessibility, Query binding, forms, preview, and
  recovery behavior are platform-owned rather than app-local
- native integration, delivery, plugins, tooling, and certification are strong
  enough that teams can ship and maintain real desktop products on top of the
  platform
- a Worth UI test-requirements program exists and has closed the platform's
  claimed hostile cases rather than leaving them as implied future work

## Companion Documents

- [worth-ui-vision.md](./worth-ui-vision.md)
- [_docs/forge-query/forge_query_vision.md](../forge-query/forge_query_vision.md)
- [_docs/forge-runtime-bridge/forge_runtime_bridge_vision.md](../forge-runtime-bridge/forge_runtime_bridge_vision.md)
- [_docs/forge-relational/forge_relational_vision.md](../forge-relational/forge_relational_vision.md)
- [_docs/forge_signal/forge_signal_vision.md](../forge_signal/forge_signal_vision.md)
