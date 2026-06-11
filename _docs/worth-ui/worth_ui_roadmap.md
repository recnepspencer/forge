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

This roadmap therefore tracks the work needed to turn the vision into a real
platform sequence.

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
- app shell and command/interaction foundations
- Query-bound views and forms/workflow surfaces

If this section is weak, everything above it inherits the same failure mode:

- hot reload devolves into interpreted UI or Rust rebuild friction
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

## Milestone 3: Hot Reload, Stable Identity, and Plan Swap

### Goal

Make the running Worth runtime the ordinary home of UI iteration so most
runtime-owned UI structure, presentation, binding, and shell changes can apply
without Rust recompilation while preserving enough identity that the app
remains usable during iteration. Compiled raw Rust remains the escape hatch for
edge cases, not the default authoring path for ordinary UI change.

### Must Ship

- file-watch and debounce pipeline for UI source changes
- runtime-hosted artifact-input watcher for file-authored UI and any
  Rust-authored composition outputs that are admitted as replaceable artifact
  input
- reload pipeline that reparses, revalidates, relowers, replans, and atomically
  swaps only at safe frame boundaries
- a default runtime-owned reload posture where ordinary changes to layout,
  shell composition, labels, tokens, inspector structure, tables, command
  placement, bindings, and other admitted UI artifact content flow through the
  same hot path
- stable identity rules for lowered nodes that own durable interaction state
- reconciliation logic for focus, scroll position, selection, panel visibility,
  splitter positions, tab state, text input state, and Query subscriptions
- explicit failure handling that keeps the last valid artifact and plan active
- in-app diagnostics surface for reload errors and rejected plan swaps
- explicit classification of what stays in the hot runtime lane versus what
  requires compiled raw Rust, with raw Rust framed as the edge-case escape
  hatch for behavior or platform work that cannot honestly be expressed as
  replaceable runtime artifact input
- one hostile reload scenario proving repeated edits do not collapse into state
  loss or app restart folklore

### Must Preserve

- reload work stays off the normal frame path where possible
- the running Worth runtime remains the ordinary owner of active artifacts,
  diagnostics, reconciliation state, and plan swap boundaries
- the default mental model stays "if it lives inside the runtime-owned UI
  artifact model, it hot reloads" rather than "reload exists for a narrow set
  of cosmetic edits"
- identity changes remain explicit replacement events rather than accidental
  state loss
- invalid reloads never blank or corrupt the active app shell
- hot reload remains composition reload, not arbitrary Rust-code hot patching
- the raw Rust escape hatch feeds canonical artifact input where possible and
  otherwise remains an explicit edge path rather than becoming a second UI
  runtime or bypassing capability registration
- state reconciliation remains compatible with nested layout structure rather
  than flattening regions into anonymous geometry

### Acceptance Evidence

- editing layout, shell composition, tokens, labels, table columns, inspector
  sections, command placement, view bindings, or other admitted runtime-owned
  UI artifact content updates a running app without a Rust rebuild
- the same running app can accept a valid replacement artifact produced from
  file-authored UI or Rust-authored composition through the same swap pipeline
- valid reloads preserve the declared stable state surfaces they should
- invalid reloads preserve the previous running plan while surfacing typed
  diagnostics
- the milestone names at least one concrete category that still requires
  compiled raw Rust and proves it is treated as an explicit escape hatch rather
  than a silent gap in the runtime reload model
- reload latency is observable and bounded enough to support ordinary UI
  iteration without feeling build-shaped

## Milestone 4: Execution Plans, Performance Lanes, and Frame-Cost Counters

### Goal

Compile canonical UI artifacts into frame-efficient execution plans and make
performance architecture explicit before broad shell or component work depends
on accidental hot-path behavior.

### Must Ship

- execution-plan lowering from canonical artifact into egui-facing runtime
  plans
- specialized execution lanes for ordinary widgets, virtualized data surfaces,
  canvas/editor surfaces, and real-time overlay or HUD surfaces
- compact runtime handles for commands, components, children, tokens, and view
  bindings rather than per-frame string resolution
- explicit rule that the steady-state frame loop does not parse or validate UI
  source
- named frame-cost counters for nodes visited, layout recompute breadth,
  hit-test breadth, text shaping, glyph uploads, allocations, draw batches,
  render passes, and virtualized rows touched
- one declared complexity-contract surface for hot paths and one explicit debt
  marker path where optimization is not yet closed

### Must Preserve

- semantic richness stays in lowering-time artifacts rather than poisoning the
  ordinary frame loop
- execution-lane specialization does not fork UI meaning into incompatible
  shadow runtimes
- performance instrumentation remains product-visible enough to certify claims
- no broad widget or shell milestone can force per-frame source interpretation
  back into the hot path

### Acceptance Evidence

- identical canonical artifacts produce identical execution plans where the
  lane and capability set are unchanged
- steady-state frame execution can be explained in counters rather than vague
  elapsed-time claims alone
- one hostile data-heavy surface and one hostile real-time surface prove that
  lane specialization changes mechanics without changing declared UI meaning
- accidental broad scans, per-frame registry lookups, or per-frame source
  interpretation can fail mechanically through counters or certification tests

## Milestone 5: Application Shell and Workspace Layout

### Goal

Make Worth UI able to host real desktop products by owning the app shell,
workspace model, and persisted layout semantics rather than leaving them to
every downstream application.

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

## Milestone 6: Command Spine, Focus, Selection, and Keyboard Routing

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

## Milestone 7: Query-Bound Views and Live Surface Binding

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

## Milestone 8: Forms, Validation, and Editing Workflows

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

## Milestone 9: Runtime UX, Preview, Recovery, and Explanation

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

## Milestone 10: Design System and Professional Component Set

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

## Milestone 11: Canvas, Spatial, and Real-Time Surface Lanes

### Goal

Make Worth UI honest about hostile frame surfaces by shipping first-class lanes
for canvases, spatial tools, real-time overlays, and renderer-integrated UI.

### Must Ship

- canvas and spatial surface lane with pan, zoom, hit testing, overlays,
  snapping, tool state, and command integration
- renderer-facing surface lane for custom draw passes and world or screen
  projection work
- real-time overlay and HUD lane with shader or material-backed surfaces
- enough execution-plan specialization that these lanes do not pay ordinary
  widget mechanics by default
- one sample hostile surface proving platform shell, diagnostics, and runtime
  binding can coexist with a high-frequency render surface
- explicit performance counters and debt markers for real-time lanes

### Must Preserve

- Worth UI does not attempt to own the full volatile scene renderer
- spatial and real-time lanes remain semantically integrated with commands,
  views, and inspection rather than becoming a disconnected side runtime
- performance claims remain counter-backed instead of anecdotal
- hostile surfaces do not pull broad shell or component work back into the hot
  path

### Acceptance Evidence

- one canvas-like product surface and one real-time overlay prove the lane
  split is real rather than aspirational
- UI structure remains hot-reloadable while the render surface maintains
  specialized mechanics
- frame counters expose where work is spent on spatial and real-time surfaces
- renderer-integrated surfaces still participate in platform command, focus,
  and diagnostics systems where applicable

## Milestone 12: Persistence, Settings, and Document or Project Workflows

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

## Milestone 13: Background Tasks, Diagnostics, and Recovery Tooling

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

## Milestone 14: Accessibility and Interaction Quality Completion

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

## Milestone 15: Native Platform Integration and Delivery

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

## Milestone 16: Plugin and Extension Architecture

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

## Milestone 17: Developer Tooling, Templates, and Platform Inspection

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

## Milestone 18: Worth UI Certification Program

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
