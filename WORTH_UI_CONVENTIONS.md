# Worth UI Conventions For AI Agents

This document is the orientation map for AI agents building `worth-ui`.

It is not an API reference. Its job is to answer four questions:

1. What category of Worth UI thing am I touching?
2. Which subsystem owns that category?
3. What existing authority should I reuse before inventing anything?
4. Which mistakes are forbidden because they create local folklore?

If you need exact signatures, types, or examples, read the owning spec and code.
This file is the mental model and navigation layer.

## Read This First

Before meaningful Worth UI work, read these in order:

- `WORTH_UI_CONVENTIONS.md`
- `_docs/coding_guidelines/mentality.md`
- `_docs/coding_guidelines/arch_laws.md`
- `_docs/more_guidelines/dx_laws.md`
- `_docs/worth-ui/worth_ui_roadmap.md`
- `_docs/worth-ui/milestone-1.md`
- `_docs/worth-ui/milestone-2.md`
- `_docs/worth-ui/milestone-3.md`
- `_docs/worth-ui/milestone-4-hot-reloading.md` when doing hot reload work

The milestone specs are not background reading. They are the architecture. If
code and spec disagree, stop and reconcile the spec, the code, or both. Do not
silently create a third local path.

## Runtime Stack

Worth UI lives in this stack:

```text
application / validation app
-> worth-ui facade and app builder
-> capability registries and frozen snapshots
-> source lowering and canonical artifacts
-> active runtime state, facts, plans, projections, receipts
-> host renderer boundary
```

Worth UI is not a second Query, truth runtime, signal runtime, native runtime,
or bridge runtime. It is the UI platform layer. It owns UI capability
registration, surface/component vocabulary, structural shell semantics, source
authoring, canonical artifact lowering, execution-plan surfaces, hot reload
admission, UI runtime facts, projection contracts, diagnostics, and renderer
handoff receipts.

Lower runtimes may own truth, query, signal, persistence, native widgets, or
host drawing. Worth UI must integrate with them through typed boundaries rather
than copying their authority into local UI helpers.

## The Core Rule

The governing Worth UI rule is:

```text
author meaning once
lower it once
project it through runtime-owned facts and sealed receipts
render receipts without re-deciding meaning
```

That rule explains most of the architecture.

Worth UI wants source and registration authors to declare meaning once, let the
platform lower that meaning into canonical artifacts and runtime facts, and let
renderers consume sealed receipts. It does not want every component, page, or
validation-app renderer to invent local maps, local defaults, local prop
parsers, local style bags, local event emitters, local denials, or local layout
rules.

If you are about to add a map in the validation app, parse a prop in a renderer,
format an expected-value error locally, inspect authored prop names during
paint, or add a component-specific field that looks like a primitive, stop and
find the owning runtime lane first.

## Support And Honesty

Worth UI often exposes public vocabulary before the full production lane is
complete. Visibility is not support. A name compiling does not mean the surface
is admitted, scalable, or closed.

Use the specs and proof tests to determine support posture. If a surface is
not complete, name the missing authority and either build it or update the spec
to mark the exact blocker. Do not call an incomplete path a precedent.

Forbidden phrases in implementation plans unless backed by a real blocker:

- "v1"
- "temporary"
- "for now"
- "legacy compatibility"
- "we can clean this up later"
- "mostly right"
- "renderer shim"
- "local callback"
- "one-off proof"

Worth UI foundations must be built right the first time unless the spec records
a concrete blocked edge.

## Category Guide

Use this section when several Worth UI surfaces look plausible.

### Surface

A surface owns product-facing placement meaning and stable identity.

Use surfaces for mounted product areas, proof targets, panels, primary content,
auxiliary content, settings content, diagnostics content, overlays, and other
things that participate in shell placement and hot-reload identity.

Surfaces are not CSS divs. Do not use a surface as a generic local layout box.
Do not create a new surface abstraction when `SurfaceId`, surface descriptors,
and source-authored surface nodes already exist.

Read next:

- `_docs/worth-ui/milestone-1.md`, Phase 8
- `_docs/worth-ui/milestone-2.md`, structural and canonical artifact phases
- `crates/worth-ui/src/capability/registry/surface/`
- `crates/worth-ui/src/source/`

### Component

A component owns reusable UI capability and content behavior. A component does
not own shell placement, mosaic topology, global state truth, or private style
vocabulary.

Components should consume shared primitive families for content, appearance,
layout, interaction, and motion. They should not mint private `button_*`,
`card_*`, or `menu_*` props when a shared primitive family should own the
meaning.

Read next:

- `_docs/worth-ui/milestone-1.md`, Phase 7
- `_docs/worth-ui/milestone-4-hot-reloading.md`, Phase 29 and later primitive phases
- `crates/worth-ui/src/capability/registry/component/`
- `crates/worth-ui/src/runtime/primitive/`

### Mosaic

Mosaic owns structural shell and page space allocation:

- region kinds
- surface placement policies
- shell layout vocabulary
- scroll ownership
- focus scopes
- clipping and hit-test posture
- sizing contracts for regions
- durable state slots for shell posture
- concrete page/region topology
- execution-plan topology derived from canonical structure

Mosaic should answer questions like:

- where does the editor region live?
- where does the inspector surface mount?
- who owns scrolling?
- can this surface dock, tab, split, float, or restore?
- what shell region identity survives hot reload?

Mosaic is not a generic flexbox or grid replacement. Do not use mosaic for
button anatomy, icon/text spacing, menu-row internals, or card content layout.

Read next:

- `_docs/worth-ui/milestone-1.md`, Phases 9-12
- `_docs/worth-ui/milestone-2.md`, Phase 5
- `_docs/worth-ui/milestone-3.md`, plan topology phases
- `crates/worth-ui/src/capability/registry/mosaic_region/`
- `crates/worth-ui/src/capability/registry/mosaic_placement/`
- `crates/worth-ui/src/capability/registry/mosaic_sizing/`
- `crates/worth-ui/src/source/lower/structural_legality/`

### Flow Layout

Flow layout owns local content arrangement inside components and primitive
surfaces:

- inline icon/text anatomy
- local rows and columns
- local stacks
- card internals
- menu item internals
- inspector field rows
- local spacers and simple local grids

Flow layout is not mosaic. It must not decide shell placement, scroll ownership,
surface movement, region containment, focus scope, or persistence.

Flow layout also must not invent a separate measurement universe. Gap, padding,
and local size values must resolve through shared Worth UI measurement, token,
or sizing authority. The draw plan may receive resolved points, but authored
truth should not be anonymous raw numbers when the milestone laws require named
measurement facts.

Read next:

- `_docs/worth-ui/milestone-4-hot-reloading.md`, Phase 30
- `crates/worth-ui/src/runtime/primitive/flow_layout/`
- `crates/worth-ui/src/runtime/primitive/presentation/`
- `crates/worth-ui/src/capability/registry/mosaic_sizing/descriptor/measurement/`

### Composition Graph

Composition graph owns arbitrary local product anatomy inside already-admitted
pages, surfaces, portal entries, diagnostic panels, collection items, reusable
component instances, and nested containers.

Use composition graph when the question is:

- which content/control/interaction/diagnostic nodes belong inside this
  container?
- what is the parent, order, participation, and stable identity of each child?
- which composition root is attached to this admitted page/content slot,
  surface, portal entry, diagnostic panel, collection item, or component
  instance?
- how do reusable field, card, toolbar, menu, or row recipes lower into
  ordinary graph nodes and slots?
- which children participate in layout, hit testing, focus, accessibility, and
  diagnostics?

Composition graph is not mosaic and not flow layout. Mosaic owns shell and page
region topology, surface placement, shell sizing, scroll ownership, clipping,
hit-test posture, persistence, and focus scopes. Surfaces own product-facing
placement identity. Flow layout owns a container's local arrangement policy.
Composition graph owns the graph of local product nodes inside admitted mounted
content and the facts that make flow layout, appearance, interaction,
accessibility, diagnostics, collection, portal, and motion consumers agree
about the same mounted structure.

Composition roots must consume existing page/content slot, surface,
component-instance, portal-entry, collection-item, or diagnostic mount receipts.
They must not mount directly into mosaic regions or redefine mosaic placement,
sizing, scroll, clipping, hit-test, persistence, or focus law.

Do not solve new pages, forms, cards, toolbars, menus, inspectors, or collection
rows by adding renderer branches, component-specific child lists, or page-local
maps. Add or consume composition roots, node receipts, edge receipts, semantic
context receipts, allocation receipts, and graph/index access receipts.

Read next:

- `_docs/worth-ui/milestone-4.1-composition-graph.md`
- `_docs/worth-ui/milestone-4-hot-reloading.md`
- Forge Query `docs/AI_README.md`, graph touch obligation authority and graph
  read access planning sections

### Measurement, Sizing, Tokens, And Density

Measurement authority is shared. Do not create a new number parser for every
primitive family.

Milestone 1 says raw layout numbers are invalid public artifact input outside
named token, sizing, or measurement definitions. That applies to shell layout
and local primitives. If a property means gap, padding, width, height, radius,
z-order, timing, breakpoint, or animation duration, look for the shared
measurement/token/sizing lane first.

Acceptable lower-level draw values are derived outputs, not authored truth.

Read next:

- `_docs/worth-ui/milestone-1.md`, Phase 11
- `crates/worth-ui/src/capability/registry/mosaic_sizing/`
- `crates/worth-ui/src/capability/registry/density/`
- `crates/worth-ui/src/capability/registry/style_value/`

### Primitive Boundary

Primitive families are the shared atom vocabulary that components consume.

The primitive boundary currently includes content, container, appearance, flow
layout, projection dependency evidence, admission reports, denial receipts, and
draw plans. It must grow into interaction and motion without creating
component-local folklore.

Primitive authoring must lower through schemas. Schemas own:

- schema id
- prop key or declaration key
- value kind
- default policy
- expected syntax
- examples
- semantic slice
- runtime fact family
- denial code

Admission is a batch boundary. One authored surface scan should produce one
accepted prop-set receipt or one denial set. Do not implement per-field
callbacks as the public proof surface.

Read next:

- `_docs/worth-ui/milestone-4-hot-reloading.md`, Phase 29
- `crates/worth-ui/src/runtime/primitive/schema.rs`
- `crates/worth-ui/src/runtime/primitive/admission.rs`
- `crates/worth-ui/src/runtime/primitive/receipt.rs`

### Content

Content owns visual anatomy: text, icon, image, spacer, groups, and local
content items.

Content is not layout and not appearance. Do not solve icon/text spacing by
creating an `icon_plus_text` component-specific combo. Model icon and text as
content items; model their spacing through flow layout; model their colors
through appearance/style values; model their identity through registered icon
capabilities.

Read next:

- `_docs/worth-ui/milestone-4-hot-reloading.md`, Phases 29-31+
- `crates/worth-ui/src/runtime/component_content/`
- `crates/worth-ui/src/runtime/component_visual/`
- `crates/worth-ui/src/runtime/primitive/content.rs`
- `crates/worth-ui/src/runtime/primitive/presentation/item_frame.rs`

### Appearance

Appearance owns visual facts: background, foreground, border, radius, chrome,
tone, typography, state appearance, and style-derived values.

Appearance is not a style blob. It must be separated from layout, content,
interaction, and motion. States such as hover, pressed, disabled, focused, and
selected should compose by state-layered appearance facts, not by duplicating
every style prop on every component.

Do not make renderers choose colors, borders, or radius locally. Do not hard
code visual state defaults in component renderers when the runtime can admit
appearance receipts.

Read next:

- `_docs/worth-ui/milestone-4-hot-reloading.md`, primitive and appearance phases
- `crates/worth-ui/src/runtime/primitive/appearance.rs`
- `crates/worth-ui/src/runtime/component_visual/`
- `crates/worth-ui/src/capability/registry/appearance/`
- `crates/worth-ui/src/capability/registry/style_value/`

### Interaction

Interaction owns gestures, cursor posture, focus posture, state transitions,
and emitted runtime meaning.

Do not build local event emitters, local callbacks, or component-specific
submit systems. Interaction receipts must be runtime-owned, sealed, and
generic enough for buttons, cards, menu items, list rows, inspectors, and
future controls.

Renderer behavior should be thin:

- detect host input
- submit the generic interaction through runtime authority
- render current runtime receipts

Read next:

- `_docs/worth-ui/milestone-4-hot-reloading.md`, Phase 29 and interaction phases
- `crates/worth-ui/src/runtime/interaction_lane/`
- `crates/worth-ui/src/runtime/component_interaction/`

### Declarative Expressions

Declarative expressions own derived UI meaning such as conditional
participation, requiredness, validation posture, readiness, payload shape,
normalization, and other computed runtime facts.

Expressions are a capability system, not a closed enum and not a script
escape hatch. Worth UI ships standard expression operators, but app authors
must be able to register custom operators through typed capability descriptors
that declare input facts, output type, dependency contracts, purity, cost
posture, support posture, diagnostics, and evaluation authority.

Do not solve expression needs with component-local predicates like
`show_if_yes`, `disable_until_filled`, or `payload_data_wrapper`. Do not
evaluate expression strings in renderers. Do not expose arbitrary script
execution as the extension model.

Read next:

- `_docs/worth-ui/milestone-4.1-composition-graph.md`, expression phases
- `_docs/worth-ui/milestone-4-hot-reloading.md`, live-view and interaction
  phases
- Forge Query `docs/AI_README.md`, projection fact and graph obligation
  sections

### Motion

Motion owns admitted transitions and animations over declared properties.

Do not implement animation as renderer-local easing constants, timers, or
ad hoc state machines. Motion must consume typed property identities, timing
measurements, state transitions, and runtime receipts. Timing values follow
the same measurement/token rule as layout values.

Read next:

- `_docs/worth-ui/milestone-4-hot-reloading.md`, motion phases
- `_docs/worth-ui/milestone-3.md`, real-time and frame-boundary phases

### Query-Bound Surfaces

Query-bound UI must consume Query-owned typed artifacts and projection facts.
Worth UI should not invent local Query facades, loading taxonomies, support
postures, or recovery systems.

Use Query public facade surfaces and support/admission posture. Worth UI can
project Query meaning into UI, but it does not own Query truth.

Read next:

- `_docs/worth-ui/milestone-1.md`, Query integration phases
- `_docs/worth-ui/milestone-2.md`, binding and capability semantics
- Forge Query `docs/AI_README.md`

### Source Authoring

Source authoring is an input lane, not runtime authority. Parsed source lowers
into artifact input, structural legality, bound capability references, canonical
artifacts, active snapshots, runtime facts, and projection receipts.

Do not let renderers or validation-app code reopen source text to decide
meaning. Source spans should survive into denial receipts and diagnostics, but
the source file is not a paint-time database.

Read next:

- `_docs/worth-ui/milestone-2.md`
- `crates/worth-ui/src/source/`
- `crates/worth-ui/src/source/surface_component_authoring.rs`
- `crates/worth-ui/src/source/lower/`

### Hot Reload

Hot reload is a runtime admission and projection-rebind problem, not a renderer
remount trick.

A save should produce:

- authored delta summaries
- touched semantic slices
- changed runtime facts
- projection dependency intersections
- rebind or preserve decisions
- sealed receipts
- diagnostics and counters

Renderers should not decide whether a change is structural, visual, interactive,
or state-preserving. Mutations declare invalidated facts; projections declare
consumed facts; the runtime computes the intersection.

Read next:

- `_docs/worth-ui/milestone-4-hot-reloading.md`
- `crates/worth-ui/src/runtime/authored_delta/`
- `crates/worth-ui/src/runtime/runtime_fact/`
- `crates/worth-ui/src/runtime/projection_rebind/`
- `crates/worth-ui/src/runtime/validation_reload/`

### Diagnostics And Denials

Diagnostics are first-class runtime data. They are not strings, logs, or
renderer fallback messages.

Every denial should carry typed, machine-readable context:

- code
- owning schema or authority id
- raw authored value when applicable
- expected syntax
- examples
- semantic slice
- fact family
- source span readiness
- stable digest
- presentation rows derived from the receipt
- counters or breadth evidence when relevant

Renderers may display presentation rows. Renderers must not create expected
syntax, denial categories, or fallback explanations from local tables.

Read next:

- `_docs/worth-ui/milestone-4-hot-reloading.md`, Primitive Diagnostics Contract
- `crates/worth-ui/src/runtime/primitive/admission/denial_receipt.rs`
- `crates/worth-ui/src/runtime/primitive/flow_layout/denial_receipt.rs`
- `crates/worth-ui/src/facade/runtime_launch_diagnostics.rs`

### Renderer Boundary

The host renderer is a lower runtime contact surface. It may allocate host UI,
read sealed receipts, detect host input, and submit typed interactions. It must
not become semantic authority.

Renderers must not:

- inspect authored prop names
- parse raw authored values
- choose primitive defaults
- invent local layout maps
- create denial messages
- decide projection preservation
- scan artifact trees
- mint runtime facts or receipts

Read next:

- `_docs/worth-ui/milestone-3.md`, execution-plan and egui boundary phases
- `apps/worth-ui-validation-app/src/app/`
- `crates/worth-ui/src/runtime/primitive/presentation/`

## Decision Rules

Need shell/page structure?

- Use mosaic region, placement, sizing, state-slot, structural legality, and
  execution-plan topology. Do not use flow layout.

Need local icon/text/card/menu/field arrangement?

- Use composition graph for child membership and identity, then flow layout and
  content primitives for arrangement and anatomy. Do not use mosaic.

Need a new page, form, card, toolbar, menu, inspector, or collection row?

- Use composition roots, node/edge receipts, semantic context, and reusable
  composition recipes attached through existing surface/page/content-slot
  authority. Do not add page-local renderer maps or component-local layout
  child lists.

Need gap, padding, radius, duration, width, or size?

- Use shared measurement, token, density, sizing, or style-value authority.
  Do not add a raw number parser in the feature.

Need a button, card, row, menu item, or inspector control?

- Compose primitive content, appearance, flow layout, interaction, and motion.
  Do not create component-private style and event vocabularies.

Need a color, border, radius, typography, or state visual?

- Use appearance/style-value schemas and receipts. Do not hard code renderer
  values or duplicate every prop per component state.

Need an icon?

- Use registered icon capabilities and content icon items. Do not import an
  icon pack into a component-local registry or draw one-off SVGs as component
  meaning.

Need hover, pressed, focus, cursor, submit, select, or activate behavior?

- Use interaction state and sealed interaction receipts. Do not build local
  event emitters or callback maps.

Need conditional visibility, requiredness, validation state, readiness,
payload shape, or a custom computed UI fact?

- Use declarative expression capabilities and projection receipts. Register
  custom operators through capability descriptors when the standard algebra is
  insufficient. Do not add local predicates or renderer callbacks.

Need animation or transition?

- Use motion over declared properties and measurement-owned timing. Do not use
  renderer-local timers as semantic state.

Need to reject invalid authored input?

- Add or reuse a schema, batch admission report, denial set, denial receipt,
  and presentation rows. Do not return a string error.

Need to explain what hot reload changed?

- Use authored delta, changed runtime facts, semantic slices, dependency
  contracts, and projection rebind receipts. Do not diff visible UI or inspect
  component branches.

Need validation-app proof?

- Prove the runtime path. The validation app may render and exercise receipts,
  but it must not become the source of truth.

Need to use Query meaning?

- Use Query-owned artifacts and support posture. Do not build pseudo-Query
  surfaces inside Worth UI.

## Hard Prohibitions

- Do not create local authority for shell/page maps.
- Do not create local authority for component remount decisions.
- Do not create local style, theme, density, or visual-state maps.
- Do not create local dropdown, command, interaction, or submit truth.
- Do not create local query loading/status truth.
- Do not create local dependency graphs.
- Do not infer changed facts in the renderer.
- Do not parse authored prop names in renderers.
- Do not parse raw authored values after an admission boundary exists.
- Do not close expression semantics around only the built-in operators. Custom
  expression operators must enter through typed capability registration, not
  local renderer callbacks or scripts.
- Do not let validation-app code become the production architecture.
- Do not treat mosaic as flexbox or grid for local component internals.
- Do not treat flow layout as shell topology.
- Do not treat flow layout as composition topology. Flow arranges children;
  composition graph proves which children exist, where they belong, and how
  they participate.
- Do not implement new pages, forms, cards, toolbars, menus, inspectors, or
  collection rows as renderer branches or component-private child lists.
- Do not mount composition roots directly into mosaic regions or let
  composition override mosaic placement, sizing, scroll, clipping, hit-test,
  persistence, or focus contracts.
- Do not put raw layout numbers in public authored input when a named
  measurement, token, sizing, density, or style-value fact should own them.
- Do not build component-specific props when a primitive family should own the
  meaning.
- Do not collapse content, layout, appearance, interaction, and motion into a
  generic style blob.
- Do not duplicate state props for every visual state when state-layered
  appearance facts should compose.
- Do not hand-format denials in renderer code.
- Do not flatten typed denials into diagnostic strings.
- Do not mint proof-bearing receipts outside the authority that proves them.
- Do not expose internal modules because they might be useful later.
- Do not add legacy compatibility while building a new foundation.
- Do not call incomplete work a precedent.

## AI Checklist Before Editing Code

Before changing Worth UI code, answer these:

1. What category am I touching: surface, component, mosaic, flow, content,
   appearance, interaction, motion, measurement, source, hot reload,
   diagnostics, or renderer?
2. Which subsystem owns that category today?
3. Which milestone phase defines the intended contract?
4. What existing type, schema, receipt, fact family, or registry already owns
   the authority I am about to add?
5. Am I adding authored meaning, lowered artifact meaning, runtime fact meaning,
   projection meaning, or renderer presentation?
6. Is this value authoritative or derived?
7. What proof-bearing type carries the guarantee to the next phase?
8. What changed facts does this mutation invalidate?
9. What facts does the consuming projection declare?
10. What counters prove this path did not hide a broad scan or source parse?
11. If this can fail, what typed denial receipt carries the failure?
12. If this changes UI live, what manual proof and automated proof certify hot
    reload behavior?
13. Is any code I am adding local folklore that duplicates an existing lane?

If you cannot answer those, stop and read the owning specs before writing code.

## When In Doubt

Use this decision order:

1. Worth UI conventions
2. milestone specs
3. architectural laws and mentality
4. existing facade and runtime receipts
5. existing source lowering and capability registries
6. validation app only as proof surface
7. host renderer only as receipt consumer

If the current public lane cannot do the job honestly, do not invent a local
runtime path. Name the missing authority, update the plan or spec, and build
the real boundary.
