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
- the explicit decision to build above `egui` while keeping Worth-owned
  lowering, artifact, shell, interaction, and performance architecture
- the milestone ordering needed to avoid drifting into widget-first or
  application-local infrastructure before the platform foundations exist

Milestones 1 through 3 have now closed the platform skeleton, canonical
artifact, and active runtime/execution-plan foundations. In particular,
Milestone 3 absorbed more of the original shell prerequisite work than the
initial roadmap expected: durable state reconciliation, panel and tab state
families, mosaic layout/state capability registries, command projections,
runtime diagnostics, hot replacement, lane execution, and frame-cost
certification are no longer future prerequisites.

The next gap is therefore not another invisible primitive layer. The next gap
is composed trust evidence: a real interactive validation app that can run,
display, replay, and manually validate the shell and reload scenarios that are
already mechanically supported by the runtime substrate.

Milestone 4 exposed one side-quest blocker inside that work: hot reload must
become a platform-wide projection rebind spine rather than a set of
surface-specific reload exceptions. Milestone 4S closes that blocker before
the Shopify dashboard proof broadens.

## Roadmap Rules

Rules for every remaining Worth UI item:

- each milestone must describe a real platform capability, not just a component bundle
- each milestone must solve a structural problem before the dependent product features broaden
- each milestone must preserve the ownership boundary between Worth UI, Worth Query, the runtime bridge, truth/runtime authority, and lower native adapters
- each milestone must preserve hot-lowered composition, canonical UI artifacts, and no per-frame source interpretation
- each milestone must treat the running Worth runtime as the primary host for
  hot reload, diagnostics, stable identity reconciliation, and safe plan swaps
- each milestone must preserve explicit accessibility, keyboard, focus, and diagnostics posture rather than treating them as polish
- each milestone must preserve frame-cost honesty through named counters and execution-plan boundaries
- frame-cost claims that cross diagnostic, report, or certification boundaries
  should lower Worth UI evidence into Forge Foundational performance claims,
  canonical bundles, counter-backed receipts, planned reports, and readiness
  envelopes instead of inventing local performance folklore
- each milestone must preserve a structurally explicit layout model rather than drifting back toward DOM-shaped percentage, overflow, and implicit-parent folklore
- each milestone must define concrete acceptance evidence through platform scenarios, diagnostics artifacts, performance counters, replay-safe plan behavior, tooling evidence, or certification suites
- no milestone is complete until both implementation and trust evidence exist
- features that depend on stable identity, shell contracts, or interaction contracts must not ship before those foundations exist
- Worth UI must not become a second truth runtime, a second query runtime, or a web-runtime clone

## Foundation-First Critical Path

This section is the first build priority.

These are the milestones that determine whether Worth UI becomes a real
platform or a shallow collection of components:

- canonical lowering and artifact architecture
- hot iteration and stable identity
- frame-efficient execution lanes and cost certification
- interactive validation app and composed shell acceptance evidence
- app shell and command/interaction foundations
- Query-bound views and forms/workflow surfaces

If this section is weak, everything above it inherits the same failure mode:

- hot reload devolves into interpreted UI or Rust rebuild friction
- shell behavior becomes app-local glue or invisible primitive-only proof
- components ship without coherent focus, accessibility, or runtime posture
- Query-bound surfaces drift back toward local caches and event plumbing
- performance claims remain vibes rather than certified contracts
- manual validation depends on developer memory instead of a replayable
  platform workbench

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

## Milestone 3: Hot Runtime, Stable Identity, Execution Plans, and Frame-Cost Certification

Detailed spec: [milestone-3.md](./milestone-3.md)

### Goal

Make canonical Worth UI artifacts become active, frame-executable runtime plans
that can be hot-replaced inside a running app without losing identity, durable
interaction state, Query-owned binding posture, diagnostics truth, or frame-cost
honesty.

### Must Ship

- runtime host authority over active artifact, active execution plan, last valid
  state, reload status, and diagnostics references
- runtime lifecycle, frame epoch, pending activation, pause/resume, shutdown,
  and failed-activation recovery receipts
- file-watch and debounce pipeline for UI source changes
- replaceable candidate envelopes for file-authored UI and admitted
  Rust-authored artifact inputs
- candidate admission, artifact equivalence, impact narrowing, identity
  matching, durable-state reconciliation, and Query binding rebind planning
- named impact surfaces for commands, tokens/themes, accessibility metadata,
  Query bindings, state families, lane assignment, renderer resources, and
  diagnostics policy
- atomic safe-frame activation that preserves the prior active plan on invalid
  candidates, failed lowering, failed reconciliation, or failed swap
- execution-plan lowering from canonical artifact into active egui-facing
  runtime plans
- compact runtime handles for commands, components, children, tokens, and view
  bindings rather than per-frame string or registry resolution
- admitted extension hooks for source ingress, debounce policy, identity seed
  contribution, durable state families, component lowering, lane adapters,
  canvas/spatial mechanics, real-time overlay mechanics, diagnostics
  projection, custom counters, and report materialization
- specialized execution lanes for ordinary widget/shell surfaces, virtualized
  data surfaces, canvas/spatial surfaces, and real-time overlay or HUD surfaces
- cross-lane parity proving lanes specialize mechanics and cost rather than
  canonical UI meaning
- typed diagnostics and in-app diagnostics projection for reload, plan,
  reconciliation, Query, lane, and frame-cost failures
- named counters for reload/lowering work and steady-frame lane work
- Forge Foundational performance envelopes for shared claim vocabulary,
  canonical bundle comparison, counter-backed receipts, report materialization,
  certified performance bundles, and readiness closure
- hostile certification for reload storms, invalid reload preservation,
  identity/state carry-forward, Query drift, data-heavy lanes, real-time lanes,
  and no-source/no-registry/no-broad-scan steady frames

### Must Preserve

- semantic richness stays in lowering-time artifacts rather than poisoning the
  ordinary frame loop
- invalid reloads never blank or corrupt the active app shell
- identity changes remain explicit replacement events rather than accidental
  state loss
- hot reload remains composition reload, not arbitrary Rust-code hot patching
- the running Worth runtime remains the owner of active artifacts, active plans,
  diagnostics, reconciliation state, and plan swap boundaries
- state reconciliation remains compatible with nested layout structure rather
  than flattening regions into anonymous geometry
- Query-facing runtime surfaces referenced by active plans preserve Query-owned
  support, admission, live, async/result, projection, recovery, inspection, and
  explanation posture
- execution-lane specialization does not fork UI meaning into incompatible
  shadow runtimes
- extension hooks remain typed, admitted, receipt-bearing contributions rather
  than active-plan, Query, state, lane, or certification bypasses
- performance instrumentation remains product-visible enough to certify claims
- no broad widget, shell, canvas, data, or plugin milestone can force per-frame
  source interpretation, registry lookup, or broad artifact scans back into the
  hot path

### Acceptance Evidence

- the same running app can accept valid replacement artifacts produced from
  file-authored UI or Rust-authored composition through the same activation
  pipeline
- equivalent replacements classify as no-op and avoid needless plan swaps
- valid reloads preserve eligible durable state and explicitly replace or drop
  ineligible state
- invalid reloads preserve the previous active plan while surfacing typed
  diagnostics
- identical canonical artifacts produce identical execution plans where the lane
  and capability set are unchanged
- ordinary, virtualized data, canvas/spatial, and real-time overlay lanes all
  prove lane-specific execution through counters and receipts
- steady-state frame execution proves source parsing, artifact validation,
  registry string lookup, and broad artifact scans remain absent

## Milestone 4: Authoring DX Reset and Shopify Dashboard Product Hardening

Detailed spec: [milestone-4.md](./milestone-4.md)

### Goal

Replace the current low-level Worth UI authoring surface with a first-class
`app -> workspace -> page -> layout -> content -> surface -> component ->
appearance` model, then harden that model by building a serious native
Shopify-style admin dashboard on top of the existing Worth UI compiler,
runtime, Query-binding, and hot-reload substrate.

### Must Ship

- first-class authoring for `app`, `workspace`, `page`, `layout`, `content`,
  typed runtime families, `component`, and `appearance`
- a workspace-owned shell with typed page navigation and shared overlays,
  toasts, inspector, rail, and status surfaces
- mosaic-native layout DX for `fit`, `fill`, `share`, `clamp`, `ratio`,
  resizable regions, and explicit scroll ownership that lowers into the
  existing structural-facts pipeline
- typed runtime declaration families that consume existing `ViewBindingDescriptor`,
  bound-binding, runtime-hook, and Query-support substrate rather than
  inventing page-local hydration
- explicit iteration bindings for repeated live collections and virtualized
  execution
- seam arbitration and appearance/theme separation so touching boundaries dedupe
  by default while chrome remains distinct from structure
- one serious native Shopify-style admin workspace with overview, products,
  orders, and customers pages
- reload, restore, diagnostics, and counter evidence proving the dashboard uses
  the platform honestly

### Must Preserve

- the existing source -> artifact -> runtime proof chain and snapshot authority
  boundaries
- runtime ownership of active artifact, active plan, durable state, and Query
  posture drift
- no browser, DOM, CSS, React, or web-view implementation escapes
- no UI-local dependency graph, props runtime, hydration graph, or shadow shell
  runtime
- no broad lookup or scan regressions on the hot path
- no layout/style/runtime blob objects that collapse distinct responsibilities

### Acceptance Evidence

- equivalent old-form and new-form authoring converge on the same canonical
  artifact meaning
- the Shopify workspace runs inside one persistent shell with materially
  different pages and no page-local shell reimplementation
- repeated live data proves stable iteration identity and bounded visibility
  execution
- seams dedupe by default and only diverge when explicit posture requires it
- reload, restore, and theme/density changes preserve runtime truth and shell
  continuity
- receipts, counters, and diagnostics prove the dashboard consumes the existing
  substrate rather than bypassing it

## Milestone 4S: Hot Reloading

Detailed spec: [milestone-4-hot-reloading.md](./milestone-4-hot-reloading.md)

### Goal

Build the runtime-owned hot reload spine that all Worth UI projections consume,
so source, capability, Query, action, state, theme, density, appearance, shell,
page, and component changes flow through declared changed facts, declared
projection dependencies, typed activation evidence, runtime-owned rebind
planning, and counter-backed certification.

### Must Ship

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

### Must Preserve

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

### Acceptance Evidence

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

## Milestone 5: Command Spine, Focus, Selection, and Keyboard Routing

### Goal

Finish the core interaction substrate so actions, focus, selection, and
keyboard workflows are platform semantics instead of widget-local conventions.

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

Make Worth UI’s serious data surfaces bind to declared Query meaning instead of
app-local caches, host-shaped events, or widget-owned live-update folklore.

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

Make forms and editing a platform capability rather than a loose pile of input
widgets, local booleans, and submission folklore.

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
