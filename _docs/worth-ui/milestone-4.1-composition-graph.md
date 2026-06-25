# Milestone 4.1: Runtime Composition Graph

## Goal

Make arbitrary UI composition a runtime-owned graph of containers, content,
controls, interactions, overlays, collections, and diagnostics so authored UI
can be reorganized, resized, repeated, animated, and hot reloaded without
renderer-local layout folklore or Rust recompiles.

## Why This Milestone Exists

Milestone 4S proves hot reload can move authored primitive meaning through
runtime facts, graph obligations, sealed receipts, and mounted product views.
That is necessary but not sufficient for product-scale UI composition. A flat
mounted node list cannot express arbitrary nested rows, columns, stacks, cards,
toolbars, menus, forms, collections, popovers, or reusable content anatomy
without inviting renderer code to decide grouping, equal sizing, participation,
and placement.

Milestone 4.1 closes that gap before command, Query-bound view, form, and
component milestones broaden the product surface. The hard boundary is that
every visible UI structure must be a graph fact consumed through mounted
composition receipts. Host adapters may allocate pixels and report neutral
observations; they may not decide semantic composition.

## Governing Summaries

- `MENTALITY.md`: protects foundation-first, adversarial design. This
  milestone must solve arbitrary composition before building more product
  widgets on a flat proof path.
- `arch_laws.md`: protects proof-carrying phase progression, typed authority,
  graph-derived orchestration, and invalid-state unrepresentability. Mounted
  composition must carry what has been proven about topology, participation,
  sizing, allocation, and graph obligations.
- `composition_laws.md`: protects files and phases as semantic units. The
  milestone must split composition topology, allocation, content, overlays,
  collections, and motion into named responsibilities instead of another
  renderer file that knows everything.
- `domain_structure_laws.md`: protects physical structure as responsibility
  architecture. Composition graph, host observation, allocation, content,
  portals, collections, and reconciliation must live in predictable ownership
  locations.
- `perf_laws.md`: protects bounded breadth, explicit access structures, and
  counter-backed claims. Composition, collection, and layout reads must consume
  graph indexes and allocation receipts rather than loops, broad scans, or
  hidden N+1 traversal.
- `worth_ui_roadmap.md`: protects Worth UI as a real desktop platform, not an
  `egui` widget bundle. This milestone belongs between hot-reload foundation
  and broader command, Query, form, and component work because those milestones
  all depend on arbitrary runtime-owned composition.
- `milestone-4-hot-reloading.md`: protects the hot reload spine. Its former
  Phase 35+ work becomes stronger here because motion, overlays, collections,
  adaptive layout, button composition, and cross-component reuse require a
  generic composition graph rather than a flat primitive proof.

## Adversarial Constraint

A running Worth UI app must support arbitrary authored recomposition of a
product surface, including nested containers, equal-fill siblings, conditional
participation, content/control/interaction movement, overlays, collections,
adaptive alternatives, and motion, without renderer-local semantic branching,
per-frame source interpretation, broad graph scans, hidden N+1 traversal, or
Rust recompilation unless a genuinely new host primitive or compiled capability
is introduced.

The same guarantee must hold across app scale: adding pages, swapping page
roots, moving composed sections between pages or surfaces, reusing a composed
recipe in multiple places, and composing forms, cards, toolbars, inspectors,
menus, dialogs, and collection rows must not require new renderer conventions
or component-family layout code.

## Product Decision Lock

Composition is not a form abstraction, a card abstraction, a flexbox clone, a
DOM tree, or a renderer convenience. Composition is a Worth-owned runtime graph:
nodes declare identity and kind; edges declare membership, order, and
participation; container policies declare layout; child policies declare sizing;
Query and lower runtimes own their own truth; mounted receipts carry the proof
that host adapters may render.

Composition is also not a single-surface proof. Each mounted page slot,
surface, portal entry, collection item, diagnostic panel, or reusable component
instance may have a composition root. Page and shell placement still belongs to
mosaic and surfaces. Composition roots attach only through admitted page/content
slot, surface, component-instance, portal-entry, collection-item, or diagnostic
handoff receipts; they do not mount directly into mosaic regions or redefine
mosaic placement, sizing, clipping, scroll, persistence, or focus law.

## Ambitious Decisions Locked

- Composition identity uses dedicated `CompositionNodeId` values with typed
  backreferences to existing authorities. Existing surface, control,
  interaction, content, portal, collection, and diagnostic ids remain authority
  references, not composition identity substitutes.
- Authored composition syntax is explicit and Worth-native: `container`,
  `content`, `control`, `interaction`, `slot`, and `recipe`. There is no
  HTML/JSX/class spelling tier.
- Core composition reads ship with persistent graph indexes for
  root-to-children, node-to-parent, ancestors, descendants, participation, and
  affected consumers. Ephemeral scans are allowed only for rare bounded reads
  with receipt-backed counters.
- Component instances, page slots, surfaces, portal entries, collection items,
  and diagnostics use one composition-root receipt family with typed
  `CompositionRootKind`.
- Text direction ships as `ltr`, `rtl`, and `auto`; locale posture is modeled
  as typed metadata, with unsupported cases represented as typed posture rather
  than host folklore.
- Host measurements are stored in Worth UI through a formal observation lane.
  Host adapters observe bounds, text metrics, icon metrics, DPI, viewport,
  scroll, and time; they do not interpret them as semantic layout.
- Fill is weighted from the start. Equal fill is `fill(1)`, not a separate
  one-off row mode.
- Local scroll restoration, sticky headers, grouped sections, and compact
  cross-container move history are included because real product surfaces need
  them to avoid renderer exceptions.
- Image content starts with a narrow real registered local/static asset lane.
  Remote and async image loading remain typed unsupported posture until their
  own runtime lane exists.
- Accessibility participates in the graph from the start: role, name,
  description, enabled/disabled, focusable, tab order, label-for, and
  described-by are admitted and validated.
- Dropdowns, menus, tooltips, command palettes, toasts, diagnostics, popovers,
  and modals use one runtime portal host with typed lanes. There is no
  long-lived host-local popup exception.
- Expressions ship as an admitted capability system, not a closed built-in
  enum. The platform provides a bounded standard algebra for presence,
  equality, negation, `and`/`or`, `one_of`, empty/non-empty, conditional
  requiredness, simple normalization, payload object/nesting, field
  references, and literals; app authors may register custom expression
  operators through typed capability hooks with declared inputs, outputs,
  cost, purity, dependency contracts, diagnostics, and support posture.
  Arbitrary scripts, renderer callbacks, and source-time string evaluation are
  excluded.
- Adaptive proof covers width class, density, reduced motion, theme mode, and
  input modality. Broader OS-native postures are typed unsupported posture
  until integrated.
- Motion ships with named timing tokens and an admitted easing vocabulary:
  `linear`, `ease_in`, `ease_out`, `ease_in_out`, `emphasized`, and
  deterministic `spring`/`snappy` posture where replay-safe parameters are
  proven. Reduced-motion replacement posture is required.
- Recipes support named slots, collection item templates, and bounded repeated
  slots. They are shortcuts over canonical graph receipts, not macros or a
  second template runtime.
- The second reuse proof is a clickable card or row because it stresses nested
  content, interaction containment, collection readiness, and payload reuse
  more broadly than another button or menu item.

## Phase Plan

### Phase 1: Composition Graph Contract

This phase defines the universal graph vocabulary for composed UI. It freezes
composition as nodes, edges, policies, and receipts rather than component-local
layout branches.

**Relevant subsystems**
- runtime composition graph
- source lowering
- mounted product view
- Query graph obligation integration

**Relevant APIs**
- `WorthUiMountedProductViewReceipt`
- `WorthUiMountedNodeReceipt`
- runtime fact families for mounted topology and primitive receipts
- Query graph touch obligation authority consumed through Worth runtime lanes

**Warnings**
- Do not model composition as a DOM or generic child soup.
- Do not add form, card, toolbar, or button-specific child placement rules.
- Do not let renderer order, node kind, or vector position imply semantic
  parentage.
- Do not treat a flat mounted node list as a composition graph.

**Test requirements**
- Equivalent authored composition graphs lower to equivalent graph identities,
  node identities, edge identities, and mounted composition receipts.
- A renderer-local grouping attempt, such as "all controls first, then all
  interactions," fails boundary tests because membership must come from graph
  edges.
- Graph-obligation tests prove node kind, parent edge, order edge,
  participation edge, and mounted-topology obligations are selected from a
  declared composition graph touch.
- Invalid graph shapes, including cycles, missing roots, duplicate child order,
  and unsupported parent/child kinds, reject before mounted receipt
  construction.

**Engineering decisions**
- Composition nodes are authoritative runtime meaning after source lowering.
- Parentage, order, participation, and policy attachment are distinct graph
  facts because each can change independently and drive different projection
  rebinds.
- The first public target is nested local product composition, not shell
  mosaic topology. Mosaic continues to own page and shell region topology.
- Composition graph identity uses dedicated `CompositionNodeId` values with
  typed backreferences to existing authorities such as `SurfaceId`, control id,
  interaction id, content id, component instance id, portal entry id,
  collection item id, or diagnostic id. Raw existing ids are not reused as
  composition identity because parentage, order, participation, and
  reconciliation are separate proofs.

**Open questions**
- None.

### Phase 2: Authored Composition Source And Lowering

This phase admits source syntax for arbitrary nested containers and child nodes
while keeping authored meaning Worth-native and capability-backed.

**Relevant subsystems**
- Worth source parser
- live-view authoring
- primitive schema admission
- capability registry binding

**Relevant APIs**
- authored live-view source declarations
- primitive flow, appearance, event, and content schemas
- live-view state binding and control projection declarations
- interaction intent declarations

**Warnings**
- Do not make source look like HTML, JSX, CSS classes, or anonymous nested
  objects.
- Do not allow raw layout numbers when named measurement, density, or sizing
  facts should own the value.
- Do not allow source syntax that only works for forms, cards, or buttons.
- Do not let unrecognized composition keys pass through as inert metadata.

**Test requirements**
- A nested card containing a row of equal-fill controls and a separate
  right-aligned action row lowers through generic container and child
  declarations, not a form-specific parser.
- Unknown composition declarations, unsupported child kinds, invalid sizing
  tokens, and malformed parent references produce one typed admission report in
  canonical source order.
- Hot reloading a node from text input to dropdown preserves the state binding
  when identity policy admits preservation and changes only projection and
  participating layout facts.
- Equivalent Rust-authored and file-authored composition inputs lower to the
  same canonical composition graph where they declare the same meaning.

**Engineering decisions**
- Containers are generic composition nodes with layout policy attached. They are
  not surfaces unless they also declare product-facing placement identity.
- Existing controls, interactions, content items, diagnostics, and portal
  entries become child-capable node kinds through the same composition graph.
- Source spans must survive into graph-shape denials and mounted diagnostic
  rows.
- Authoring syntax uses explicit Worth-native declarations such as `container`,
  `content`, `control`, `interaction`, `slot`, and `recipe`. It must not use
  HTML-like tags, class bags, JSX-like component trees, or anonymous object
  nests as public composition truth.

**Open questions**
- None.

### Phase 3: Composition Graph Indexes And No-N+1 Reads

This phase makes composition lookup graph-owned and receipt-backed. Runtime
code must not rediscover parentage, children, ancestry, participation, or
layout consumers through recursive walks or local caches.

**Relevant subsystems**
- composition graph index
- Query graph read access planning
- projection dependency contracts
- performance counters

**Relevant APIs**
- graph read access planning and access-plan receipts consumed from Query
- runtime fact ids for composition nodes and edges
- mounted composition projection dependencies

**Warnings**
- Do not treat indexes as a later optimization.
- Do not build per-render or per-projection recursive graph walks.
- Do not add app-local maps from parent id to children or control id to slot.
- Do not call a graph read safe unless the access plan and counters prove it.

**Test requirements**
- Reading children, ancestors, participating descendants, and affected
  consumers uses admitted graph/index access plans with counters proving no
  caller-owned N+1 traversal.
- A broad recursive walk implementation fails residue or boundary tests even if
  it returns the right visible order.
- Changing one child edge invalidates only affected parent, allocation, and
  mounted-node projections rather than rebuilding every composition node.
- Unsupported graph access shapes return typed required-posture or denial
  results instead of silently scanning the whole graph.

**Engineering decisions**
- The graph/index view is part of the authority boundary because it proves both
  correctness and cost.
- Composition read plans must expose counters for child lookups, ancestor
  lookups, participation filters, access-plan posture, and affected-consumer
  breadth.
- Graph access planning is separate from graph touch obligations: one plans how
  to read graph-shaped data, the other selects which checks must run after a
  touch.
- Core composition reads ship with persistent graph indexes immediately:
  root-to-children, node-to-parent, node-to-ancestors/descendants,
  node-to-participation, and node-to-affected-consumers. Bounded ephemeral
  indexes are allowed only for rare derived reads whose access-plan receipts
  prove bounded cost.

**Open questions**
- None.

### Phase 4: Composition Roots, Page Slots, And Surface Mounts

This phase binds composition roots to the existing places product UI can appear:
page/content slots, registered surfaces, admitted component instances, portal
entries, diagnostics, collection items, and reusable component instances. It
prevents the composition graph from becoming a single validation-surface proof
or a replacement for mosaic/surface authority.

**Relevant subsystems**
- mosaic page and region topology as consumed placement authority
- surface descriptors and page/content slots as mount authority
- composition graph
- mounted product view

**Relevant APIs**
- surface and page/content slot fact families
- mosaic structural legality, region placement, scroll, focus, and sizing
  receipts as prerequisites
- mounted product view receipts
- composition graph root receipts

**Warnings**
- Do not make a page-local map from page id to renderer function.
- Do not let a surface imply a generic local layout box. Surface placement and
  composition-root content are distinct.
- Do not mount composition roots directly into mosaic regions. Mosaic admits
  regions and surfaces; composition attaches to the admitted content surface or
  slot that mosaic placed.
- Do not let composition roots choose region sizing, scroll ownership,
  persistence, clipping, hit-test posture, focus scope, or placement legality.
- Do not let reusable composition instances reconstruct identity from display
  labels, source path, or vector index.
- Do not let one active validation root become the hidden global composition
  root for all future pages.

**Test requirements**
- Adding a new page with a new composition root lowers through page/content slot
  facts plus composition-root facts, not a new renderer entrypoint.
- Moving a composed section from one surface or page slot to another preserves
  eligible state by composition identity and emits explicit move/rebind
  evidence.
- A stale page slot, unmounted composition root, duplicate root identity, or
  illegal root mount rejects before mounted product receipt construction.
- Composition root admission consumes already-proven mosaic legality and surface
  placement receipts. If mosaic placement, surface class, region scroll, region
  focus, or sizing legality is invalid, composition root construction must not
  proceed and must not restate the mosaic denial in local terms.
- Graph-index tests prove page root, surface root, portal root, diagnostic root,
  and collection-item root lookup consume graph/index receipts rather than
  page-local maps.

**Engineering decisions**
- Mosaic owns shell/page topology and surface placement; surfaces own
  product-facing placement identity; composition roots own local product
  anatomy inside admitted mounted content.
- A mounted product view may contain multiple composition roots, each with its
  own provenance, consumed facts, and rebind contract.
- Root identity is a proof-bearing runtime fact because state preservation,
  focus, diagnostics, and affected-consumer calculation depend on it.
- Page slot, surface, portal entry, diagnostic panel, collection item, and
  component instance roots share one composition root receipt family
  distinguished by a typed `CompositionRootKind`. Narrower internal helpers may
  exist, but the public proof family stays unified because the lifecycle is the
  same.

**Open questions**
- None.

### Phase 5: Semantic Context And Policy Propagation

This phase adds explicit propagation for inherited runtime context such as
theme, density, text direction, locale-ready posture, disabled or inert
subtrees, validation state, focus scope, and runtime mode. It prevents every
node family from reauthoring the same policy knobs.

**Relevant subsystems**
- composition graph
- appearance and density
- interaction/event posture
- diagnostics and validation state

**Relevant APIs**
- appearance and density receipts
- primitive event geometry receipts
- readiness and validation receipts
- composition graph context receipts

**Warnings**
- Do not clone CSS cascade semantics.
- Do not duplicate disabled, inert, validation, density, or theme props on
  every component family.
- Do not let context inheritance become invisible ambient global state.
- Do not let child nodes override inherited context without an explicit
  override receipt and eligibility proof.

**Test requirements**
- A disabled or inert parent composition node mechanically prevents descendant
  activation, hover/press state, focus entry, and cursor posture unless an
  admitted override exists.
- Changing a density or theme context rebinds only descendants whose receipts
  consume that context and preserves unrelated composition roots.
- Invalid context overrides, unsupported inheritance breaks, or ambiguous
  validation context produce typed denial reports with affected descendants.
- Graph-obligation tests prove context propagation, override eligibility,
  disabled/inert suppression, and validation-state participation are selected
  from graph touches rather than per-component checks.

**Engineering decisions**
- Context is a graph fact family with explicit source, scope, inherited value,
  override posture, and affected-consumer rows.
- Context does not replace authored primitive receipts. It supplies inherited
  inputs that primitive receipts may consume.
- Higher-order states such as disabled and inert belong here when they suppress
  whole subtrees; visual appearance for those states remains appearance
  meaning.
- Text direction ships in this milestone as admitted `ltr`, `rtl`, and
  `auto`/runtime-resolved posture because direction affects layout, alignment,
  truncation, focus order, and accessibility participation. Locale ships as
  posture-ready metadata with explicit unsupported/limited capability posture
  where full native locale behavior is not yet available.

**Open questions**
- None.

### Phase 6: Host Measurement Observation Lane

This phase formalizes host measurements as neutral observations, not renderer
layout authority.

**Relevant subsystems**
- host adapter boundary
- frame observation intake
- layout planning
- diagnostics and counters

**Relevant APIs**
- runtime host observation sink
- mounted product view receipts
- host-frame receipts for controls and text
- frame-cost counters

**Warnings**
- Do not let `available_width`, text metrics, DPI, or viewport size become
  renderer-owned layout policy.
- Do not let host observations mutate authoritative UI meaning.
- Do not compute adaptive alternatives or equal-fill distribution inside the
  adapter.
- Do not require source reload to process ordinary host resize observations.

**Test requirements**
- Given identical composition receipts and identical host measurement
  observations, layout allocation receipts are replay-equivalent.
- Host adapters that choose semantic layout from raw size, instead of reporting
  measurements to the runtime boundary, fail static guard tests.
- Resize or text-metric changes replan only projections that consume affected
  measurement facts.
- Missing or stale measurements produce typed measurement-readiness posture
  rather than renderer fallback layout.

**Engineering decisions**
- Host observations include available bounds, text metrics, icon metrics, DPI,
  scroll viewport facts, and elapsed time where motion later needs it.
- Observations are external inputs with their own basis and counters. They are
  not canonical authored truth.
- The runtime may produce layout allocation receipts from authored layout facts
  plus host observations; adapters execute those receipts.
- The first implementation stores host measurement observations in Worth UI
  behind a formal observation lane. They may later route through a lower
  runtime observation surface, but they are never renderer-owned semantic
  authority.

**Open questions**
- None.

### Phase 7: Layout Allocation Receipts

This phase adds item frame allocation as proof-bearing output. Equal sizing,
gap, padding, cross-axis alignment, baseline alignment, and per-child placement
become receipts, not renderer behavior.

**Relevant subsystems**
- flow layout primitive
- composition graph
- host measurement lane
- mounted product view

**Relevant APIs**
- `WorthUiFlowLayoutReceipt`
- flow measurement/density tokens
- mounted composition receipt
- item-frame presentation receipts

**Warnings**
- Do not expose CSS percentage, `calc`, `vh`, `vw`, or arbitrary unit strings
  as public truth.
- Do not let adapters divide space between siblings.
- Do not treat hidden conditional children as participating in fill
  distribution.
- Do not collapse sizing policy and layout policy into one style blob.

**Test requirements**
- Two participating siblings with `fill(1)` inside a row receive equal
  allocation receipts after gaps and hug children are accounted for.
- Conditional absence removes that child from fill distribution while retaining
  state where participation policy says so.
- Layout allocation changes caused by padding, gap, alignment, baseline,
  participation, or sizing edits rebind only layout-consuming projections.
- Invalid or unsupported sizing combinations reject through typed layout
  denials before adapter rendering.

**Engineering decisions**
- Child sizing policies include hug, fill, weighted fill, token-fixed,
  min/max-constrained, and absent-retaining-state posture where admitted.
- The adapter can place pixels according to allocation receipts but cannot
  derive sibling distribution rules.
- Layout allocation receipts carry authored token identity, resolved points,
  host measurement basis, participating child set, and counters.
- Weighted fill ships now. Equal fill is represented as `fill(1)`, and
  proportional allocation uses the same admitted weighted-fill family rather
  than a later ratio special case.

**Open questions**
- None.

### Phase 8: Scroll, Clip, Hit, And Viewport Boundaries

This phase makes scroll ownership, clipping, viewport participation, hit-test
boundaries, and local containment explicit composition facts instead of
renderer overflow behavior.

**Relevant subsystems**
- composition graph
- flow layout and allocation
- event geometry
- accessibility and focus participation

**Relevant APIs**
- mounted allocation receipts
- primitive event-region receipts
- host viewport observations
- scroll and clip posture receipts

**Warnings**
- Do not expose CSS `overflow` as public truth.
- Do not let scroll containers be renderer-local `ScrollArea` decisions.
- Do not let clipped visual regions still participate in hit testing,
  accessibility, or focus unless admitted policy says so.
- Do not let scroll ownership conflict with mosaic region scroll ownership.
- Do not reinterpret mosaic sizing overflow behavior as local composition
  overflow. Local scroll is legal only when the containing surface/region
  handoff admits surface-owned or composition-owned scroll posture.

**Test requirements**
- A local scroll container declares scroll owner, clipping posture, viewport
  basis, and hit-test participation through receipts.
- Clipped or off-viewport descendants do not participate in hit testing, focus,
  accessibility, or measurement unless an admitted policy says they do.
- Scroll-anchor edits and scroll-owner changes rebind only consumers of scroll,
  clip, viewport, and event-region facts.
- Invalid nested scroll ownership, unsupported clipping posture, or conflicting
  mosaic/local scroll authority rejects with typed diagnostics.
- A composition scroll declaration inside a mosaic region whose sizing or
  scroll contract forbids local scroll rejects by consuming the mosaic sizing
  and scroll receipts, not by a composition-local guess.

**Engineering decisions**
- Mosaic owns shell-region scroll. Composition owns local content scroll only
  inside an admitted composition root.
- Scroll, clip, hit, focus, and accessibility are shared consumers of viewport
  receipts, not separate renderer conventions.
- Collection virtualization later consumes this boundary for visible-window
  proof instead of inventing its own scroll model.
- Local scroll restoration ships in this milestone as durable state
  reconciliation over composition identity. Scroll ownership without
  restoration would force renderer memory or page-local state.

**Open questions**
- None.

### Phase 9: Content Anatomy Nodes

This phase makes text, icon, spacer, image, divider, badge, label, helper text,
error text, and adornment-like content ordinary composition children.

**Relevant subsystems**
- primitive content
- icon capability registry
- appearance and typography
- flow layout and baseline metrics

**Relevant APIs**
- primitive content receipts
- registered icon capabilities
- mounted text and icon node receipts
- appearance/style-value receipts

**Warnings**
- Do not solve icon/text, label/input, helper/error, or prefix/suffix anatomy
  with component-specific fields.
- Do not introduce HTML-like tags or CSS classes.
- Do not draw one-off SVGs or text styles in component renderers.
- Do not let content own layout, appearance, or interaction semantics.

**Test requirements**
- Button-like, field-like, card-like, and toolbar-like anatomy all consume the
  same content node receipts and flow allocation receipts.
- Invalid icon names, unsupported image assets, malformed text bindings, and
  unknown content roles produce schema-owned denial reports with source spans.
- Baseline alignment between icon and text uses content metrics exposed in
  receipts, not renderer offsets.
- Hot reloading content order, icon choice, helper text, and content role
  changes only content and dependent layout projections.

**Engineering decisions**
- Content role is semantic presentation participation, not a component branch.
- Text and icon metrics are derived facts that layout consumes.
- Content nodes may be reused across components through stable composition
  identity and shared primitive receipts.
- Image asset admission ships as a narrow real lane: registered local/static
  image assets admit through typed asset receipts, while remote, async, or
  unsupported image sources produce explicit capability posture and denials
  rather than placeholders.

**Open questions**
- None.

### Phase 10: Accessibility And Focus Participation

This phase ensures every composition node has explicit accessibility and focus
participation posture before broader command, keyboard, and accessibility
milestones depend on it.

**Relevant subsystems**
- composition graph
- focus scopes
- accessibility semantics
- interaction/event posture

**Relevant APIs**
- focus scope receipts
- primitive content role receipts
- interaction operability receipts
- mounted composition node receipts

**Warnings**
- Do not wait until a later accessibility milestone to decide whether nodes
  participate in names, roles, tab order, disabled posture, or described-by
  relationships.
- Do not derive label/control relationships from visual adjacency alone.
- Do not let keyboard focus order be vector order unless the composition graph
  explicitly proves that order.
- Do not let hidden conditional nodes remain in focus or accessibility trees.

**Test requirements**
- Label, helper text, error text, control, and interaction nodes produce
  explicit name/description/role/focus participation receipts.
- Moving a field label or helper node preserves or rejects label/control
  association through graph identity, not visual location.
- Disabled, inert, hidden, clipped, and off-viewport nodes have deterministic
  focus and accessibility posture derived from context, participation, and
  viewport receipts.
- Boundary tests reject renderer-local tab order, label association, or
  accessibility role selection.

**Engineering decisions**
- Full accessibility completion remains a later milestone, but participation
  posture must exist here because composition topology creates or removes the
  relationships.
- Focus scope identity belongs on composition roots and containers where
  traversal boundaries matter.
- Accessibility names and descriptions are derived from content/control graph
  relationships with explicit receipts.
- Validation exercises host accessibility inspection for role, name,
  description, enabled/disabled posture, focusable posture, tab order,
  label-for, and described-by relationships. Host API gaps surface as typed
  capability posture, not as missing receipts.

**Open questions**
- None.

### Phase 11: Controls And Interactions As Generic Children

This phase removes special live-view treatment for controls and actions. Inputs,
dropdowns, buttons, menu items, command entries, and future controls mount as
ordinary composition nodes with primitive receipts attached.

**Relevant subsystems**
- live-view control projection
- interaction lane
- primitive appearance, event, and flow
- mounted product view

**Relevant APIs**
- `WorthUiLiveViewControlHostFrameReceipt`
- `WorthUiMountedInteractionNodeReceipt`
- live-view state binding receipts
- interaction intent and activation receipts

**Warnings**
- Do not create a form-specific action row or a card-specific button slot.
- Do not let controls choose their own label/input gap in renderer code.
- Do not let interaction readiness or disabled posture become local widget
  state.
- Do not group nodes by kind during rendering.

**Test requirements**
- A text input, dropdown input, and submit interaction can be moved between
  arbitrary containers without losing stable state or interaction identity
  where reconciliation admits preservation.
- Changing a control from text input to dropdown preserves binding identity and
  changes projection receipts without renderer branches.
- Disabled, read-only, hover, focus, cursor, and activation posture all flow
  through existing primitive and interaction receipts.
- Renderer-boundary tests prove adapters consume mounted node and host-frame
  receipts and do not inspect authored control or interaction declarations.

**Engineering decisions**
- The live-view scenario is a consumer of composition graph, not the owner of a
  form layout model.
- Label, control frame, helper text, and error text are composition children,
  so field anatomy scales to any component family.
- Interaction nodes declare intent and effect semantics separately from their
  layout parent.
- Dropdown closed controls may render as ordinary controls, but open dropdown
  popup entries wait for portal receipts. Host mechanics may execute selection
  only after the portal host admits popup composition; no long-lived host-local
  dropdown exception is allowed.

**Open questions**
- None.

### Phase 12: Standard Expression Algebra And Data Projection

This phase freezes the built-in expression vocabulary as ordinary registered
expression capabilities. It proves conditions, requiredness, computed
presentation state, payload projection, and simple derived UI facts without
making the standard algebra the only extensibility path.

**Relevant subsystems**
- live-view state binding
- readiness and payload projection
- composition graph participation
- Query projection consumption where data is Query-owned
- expression capability registry

**Relevant APIs**
- live-view state binding receipts
- conditional participation receipts
- readiness and payload projection receipts
- Query projection fact receipts
- standard expression operator descriptors

**Warnings**
- Do not introduce a form framework.
- Do not add component-local predicates such as `show_if_yes`,
  `disable_until_filled`, or `payload_data_wrapper`.
- Do not let expressions read renderer state or source text at frame time.
- Do not hard-code the standard algebra as a closed enum that prevents later
  registered operators.
- Do not let the standard algebra become arbitrary scripting.

**Test requirements**
- Operator parity tests prove every standard operator is represented by a
  registered expression capability descriptor, not by a built-in-only enum
  branch. The proof must compare descriptor id, arity, input value kinds,
  output value kind, purity, dependency contract, cost posture, diagnostics
  posture, and semantic slice.
- Expression lowering tests prove conditional participation, requiredness,
  validation presentation, enabled posture, normalization, and payload shape
  all consume one expression admission and projection lane. The test must fail
  if any case is implemented through a component-local predicate, form-local
  status flag, renderer branch, or separate payload-only parser.
- Payload-shape hot reload tests change a direct payload object to nested
  `data` shape and prove only expression output facts, payload projection
  consumers, and the next emitted interaction payload digest change. Control
  state, composition identity, layout allocation, and unrelated interaction
  receipts must preserve identity.
- Cross-field validation tests exercise advisory and blocking outcomes from
  the same expression lane, proving validation posture is a typed runtime fact
  consumed by diagnostics, appearance, composition participation, and
  interaction readiness without app-local form status.
- Query-backed expression tests consume typed Query projection fact receipts,
  Query support posture, and Query graph read access plans. The test must fail
  if the implementation reads retained rows, raw binding ids, bridge internals,
  local loading enums, or Query state maps.
- Canonical equivalence tests prove semantically equivalent expressions with
  different source ordering or harmless parentheses lower to equivalent
  operator plans, dependency contracts, output fact ids, and denial-free
  receipts where equivalence is declared.
- Batch denial tests submit invalid standard operator arity, value kind,
  dependency, payload path, and Query fact reference in one save and require
  one typed expression report in canonical declaration order with source span
  readiness, counters, stable denial-set digest, and no projection receipt
  construction.
- Replay tests rebuild derived expression output facts from admitted source,
  frozen capability snapshots, runtime input facts, and Query projection facts
  and prove byte-for-byte equivalent output receipts and payload digests.

**Engineering decisions**
- Expressions are typed, admitted, bounded, and capability-described. They are
  not general-purpose code execution.
- Expression outputs are derived runtime facts that participate in changed-fact
  and projection-rebind contracts.
- Validation state is not a form-specific surface; it is a runtime fact
  consumed by composition, diagnostics, appearance, and interaction.
- The standard expression algebra ships with presence, equality, negation,
  boolean `and`/`or`, `one_of`, empty/non-empty, conditional requiredness,
  simple normalization, payload object/nesting, field references, and literal
  values.

**Open questions**
- None.

### Phase 13: Expression Capability Registration

This phase exposes the extension boundary for app-defined expression operators.
Custom expressions become registered capabilities with declared type contracts,
dependency contracts, purity, cost posture, diagnostics, and support posture
rather than renderer callbacks or source-language escape hatches.

**Relevant subsystems**
- capability registries
- expression capability registry
- semantic slice inventory
- Query graph obligation integration

**Relevant APIs**
- expression operator capability descriptor
- expression input and output type descriptors
- expression dependency contract receipts
- expression support posture receipts

**Warnings**
- Do not allow custom expressions to enter as raw closures without capability
  descriptors.
- Do not let custom operators read arbitrary runtime state, source files,
  renderer observations, or host widgets.
- Do not hide custom operator cost behind the same API shape as a literal or
  field reference.
- Do not let app code mint proof-bearing expression handles directly.

**Test requirements**
- Facade registration tests register a custom expression operator with declared
  operator id, input names, input value kinds, output value kind, dependency
  contract, purity, deterministic evaluation posture, cost posture, diagnostic
  posture, support posture, and semantic slice. The operator must appear in the
  frozen capability snapshot and the active runtime basis with stable digests.
- Registration denial tests reject missing dependency contracts, ambiguous
  output type, unsupported value kind, undeclared Query fact access, unbounded
  cost posture, impure mutation authority, duplicate operator id, unstable
  descriptor digest, and diagnostics-without-denial-code before authored source
  can reference the operator.
- Capability snapshot equivalence tests prove two registrations with identical
  descriptor meaning produce equivalent snapshot entries, while changing input
  kind, output kind, dependency contract, cost posture, or support posture
  changes the capability digest and affected semantic slice.
- Compile-fail coverage proves app code cannot construct admitted expression
  operator handles, dependency receipts, support posture receipts, evaluation
  plan receipts, or expression output facts directly.
- Graph-obligation tests prove expression support, capability gaps, dependency
  legality, Query access, deterministic-evaluation posture, and cost posture
  are selected from expression graph touches rather than local validator tables.
- Hidden-read tests register an operator that tries to read undeclared runtime,
  Query, source, renderer, or host-observation data and prove the registration
  or evaluation plan is denied before the read can affect output.
- Cross-app isolation tests prove two apps can register operators with the same
  local name under different capability authorities without id collision,
  shared mutable state, or cross-runtime leakage.
- Cost-contract tests require custom operators to publish exact work counters
  at registration or plan time, and prove scalar, batched, and Query-backed
  operators expose different cost posture honestly rather than hiding behind
  one uniform cheap API.

**Engineering decisions**
- Custom expressions are capability contributions, not scripts.
- Registration must declare the expression's truth inputs and cost before any
  authored composition can use it.
- The standard algebra is implemented as pre-registered platform expression
  capabilities so built-in and app-defined operators share the same lifecycle.
- Capability support posture is part of the frozen snapshot and active runtime
  basis, so hot reload can deny unsupported custom expression references
  without recompiling.

**Open questions**
- None.

### Phase 14: Expression Planning, Evaluation, And Rebind

This phase lowers admitted expression declarations into execution plans and
runtime facts. It makes custom and standard expression evaluation deterministic,
bounded, projection-aware, and hot-reloadable.

**Relevant subsystems**
- expression planner
- expression evaluator
- runtime fact lowering
- projection rebind coordinator

**Relevant APIs**
- expression admission receipt
- expression evaluation plan receipt
- expression output fact receipts
- changed-fact and projection dependency receipts

**Warnings**
- Do not evaluate expressions during rendering.
- Do not let custom operators bypass planning with direct runtime callbacks.
- Do not let expression evaluation mutate authoritative runtime state.
- Do not recompute unaffected expressions after unrelated composition edits.

**Test requirements**
- Plan parity tests prove standard and custom operators with equivalent
  declared inputs, dependency contracts, support posture, and output values
  produce equivalent evaluation plan receipts, output fact receipts, projection
  dependency receipts, and rebind decisions.
- Incremental rebind tests edit an expression operator reference, field
  reference, payload shape, literal value, Query fact reference, and dependency
  contract independently. Each edit must change only the affected expression
  output facts and declared consumers, while unrelated composition, appearance,
  control-state, interaction, portal, and collection projections preserve
  identity.
- Invalid-evaluation tests deny custom operator result kind mismatch,
  non-deterministic evaluation posture, missing input fact, stale Query
  projection fact, undeclared input read, unsupported cost posture, panic-like
  evaluator failure, and output serialization failure before output facts
  replace the prior valid plan.
- Mutation-isolation tests prove expression evaluation cannot mutate live-view
  state bindings, Query truth, composition graph topology, host observations,
  capability snapshots, or diagnostics policy. Evaluators can only return typed
  derived outputs or typed denials.
- Replay and convergence tests evaluate the same expression plan from fresh
  runtime reconstruction, hot reload candidate activation, and steady-frame
  reuse and require identical output facts, diagnostics posture, counters, and
  payload digests.
- Topological scheduling tests prove expression dependencies are evaluated in
  canonical dependency order, cycles deny before plan construction, duplicate
  dependency edges canonicalize, and independent expression subgraphs can be
  planned without hidden ordering dependence.
- Counter tests prove execution breadth is bounded by changed input facts,
  declared expression dependencies, and graph/index access plans. The proof
  must include exact counters for touched expression nodes, operator
  evaluations, Query fact reads, skipped unaffected expressions, and source or
  artifact parses, with source/artifact parses staying zero during execution.
- Prior-plan preservation tests submit a candidate with one invalid expression
  among several valid changed expressions and prove the previous active output
  facts remain mounted while diagnostics describe the denied candidate and
  affected consumers.

**Engineering decisions**
- Expression execution plans are lowered runtime artifacts. The evaluator
  consumes plans and input facts; it does not rediscover operator strategy.
- Expression outputs are derived facts and can be destroyed and rebuilt from
  admitted source, capability snapshots, and runtime input facts.
- Custom operators execute only through registered runtime evaluator entries
  whose input surface is the planned, typed expression packet.
- Rebind behavior is driven by expression dependency receipts and changed
  facts, not by form-local dirty flags.

**Open questions**
- None.

### Phase 15: Expression Diagnostics, Workbench Hooks, And Author Proof

This phase makes expression extensibility inspectable and teachable. It proves
that app authors can add expression capability without creating local folklore,
and that users can diagnose denied or changed expression behavior through
mounted evidence.

**Relevant subsystems**
- expression diagnostics
- mounted diagnostics
- validation workbench
- capability inspection

**Relevant APIs**
- expression denial receipts
- expression evidence row receipts
- capability inspection receipts
- validation workbench authored examples

**Warnings**
- Do not flatten custom expression failures into generic "invalid expression"
  strings.
- Do not hide custom operator provenance from diagnostics.
- Do not make author examples depend on validation-app-only registration paths.
- Do not let richer expression diagnostics alter evaluation output.

**Test requirements**
- Multi-denial tests submit invalid standard and custom expression
  declarations in one save and require one typed expression admission report
  with all denials in canonical declaration order, source span readiness,
  schema/operator ids, denial codes, expected syntax, examples, counters,
  stable denial-set digest, and affected-consumer rows derived from receipts.
- Mounted evidence tests use one custom expression for conditional
  participation, readiness, validation presentation, normalization, and payload
  projection. Evidence must name operator id, capability authority, input
  facts, Query facts where applicable, output fact, dependency contract, cost
  counters, semantic slice, projection consumers, and rebind decision.
- Inspection tests prove capability inspection can answer which standard and
  custom expression operators are registered, which source declarations consume
  them, which runtime facts they read, which output facts they publish, which
  projections rebind when they change, and which denials would block use.
- Renderer-boundary tests reject local formatting of expression denials,
  renderer-local evaluation, display-only predicate checks, and evidence rows
  built from debug strings instead of mounted expression receipts.
- Richness-policy tests switch expression diagnostics between compact and full
  presentation and prove admission, evaluation output, output fact identity,
  payload digest, and projection rebind decisions are unchanged.
- Workbench proof demonstrates one app-defined custom expression hook in the
  same running validation app used for ordinary composition hot reload. It must
  prove registration, authored use, hot reload, invalid edit preservation,
  mounted diagnostics, and payload/readiness effects without adding a form
  framework, renderer callback, source-time script, or component-local
  predicate.
- Negative workbench tests intentionally remove the custom operator
  registration, change its output type incompatibly, and make its dependency
  contract undeclared. Each case must deny through the expression capability
  path while preserving the prior mounted product view.
- Documentation-example tests compile or execute the public authoring examples
  for custom expression registration and use, proving the documented path is
  the real facade path rather than aspirational prose.

**Engineering decisions**
- Diagnostics are part of the expression capability contract, not an adapter
  afterthought.
- Author-facing extension hooks must be proven through the same facade and
  snapshot path expected for real apps.
- Expression richness policy controls mounted explanation breadth; it cannot
  change admission, evaluation, or projection truth.
- This phase closes the author-extensibility gap before reconciliation and
  mounted diagnostics broaden expression consumers.

**Open questions**
- None.

### Phase 16: Stable Reconciliation Across Moves And Shape Swaps

This phase makes state preservation and replacement graph-derived when nodes
move, projection kinds change, or conditional participation changes.

**Relevant subsystems**
- durable state reconciliation
- user intent target binding
- composition graph identity
- projection rebind coordinator

**Relevant APIs**
- durable state family reconciliation receipts
- live-view state binding receipts
- target binding receipts
- projection rebind receipts

**Warnings**
- Do not preserve state by vector index, display label, or renderer widget id.
- Do not drop state just because a node moved to a different parent.
- Do not preserve state across a shape swap unless identity and family policy
  prove compatibility.
- Do not let conditional absence become deletion unless participation posture
  says deletion.

**Test requirements**
- Moving an input from one container to another preserves its value when the
  state binding and reconciliation policy remain compatible.
- Changing text input to dropdown preserves or rejects state according to typed
  value-kind compatibility, with receipts explaining the decision.
- Conditional absence retaining state does not participate in layout, hit
  testing, focus, or accessibility until present again.
- A stale or mismatched user-intent target binding rejects before interaction
  activation or state mutation.

**Engineering decisions**
- Composition identity, state binding identity, projection kind, and visible
  participation are distinct facts with distinct rebind consequences.
- Reconciliation consumes proof-bearing prior and candidate receipts; it never
  reopens source text.
- State preservation is a runtime decision encoded in receipts, not an adapter
  behavior.
- Cross-container move history is retained as compact diagnostic evidence,
  including prior root/path, candidate root/path, preservation/replacement
  posture, and the receipts that justified the decision. It is not an
  unbounded history log.

**Open questions**
- None.

### Phase 17: Mounted Diagnostics As Composition

This phase makes diagnostics, evidence rows, inline field errors, launch
errors, and admission reports mounted composition nodes rather than renderer
strings.

**Relevant subsystems**
- diagnostics and denial receipts
- mounted evidence nodes
- composition graph
- product view rendering

**Relevant APIs**
- `WorthUiMountedEvidenceNodeReceipt`
- primitive and live-view admission reports
- runtime launch diagnostics
- mounted diagnostic panel receipts

**Warnings**
- Do not flatten typed denials into strings before mounting.
- Do not hand-format expected syntax or denial categories in adapters.
- Do not treat launch errors as outside the product rendering model unless they
  are explicitly marked as non-product host chrome.
- Do not make diagnostics richness alter operational truth.

**Test requirements**
- Invalid composition, primitive, control, interaction, and graph-access values
  mount as typed diagnostic nodes with source span readiness and stable
  denial-set digest.
- Inline field errors and global evidence panels consume the same denial
  receipts through different composition placements.
- Renderer-boundary tests reject local `format!("{denial:?}")` or colored-label
  diagnostic meaning in product renderers.
- Diagnostics richness policy changes presentation breadth without changing
  admission or projection outcomes.

**Engineering decisions**
- Diagnostics are ordinary mounted nodes with special provenance and
  presentation roles, not a separate renderer channel.
- Denial receipts own expected syntax, examples, source spans, counters, and
  affected consumers.
- Evidence panels should prove bounded graph reads and changed-fact
  intersections for composition work.
- The launch-error screen becomes a mounted diagnostic product view in this
  milestone unless the host process cannot create any Worth runtime at all. In
  that exceptional case, host chrome must explicitly mark itself as outside
  Worth product rendering.

**Open questions**
- None.

### Phase 18: Portal Host And Anchored Composition

This phase adds cross-surface overlay composition through a runtime portal host
owned as an ordered pancake list.

**Relevant subsystems**
- portal host
- anchored layout
- focus and interaction containment
- accessibility posture

**Relevant APIs**
- mounted portal host node receipt
- interaction activation receipts
- event containment receipts
- host measurement observations for anchors

**Warnings**
- Do not implement CSS-style `z-index`, absolute positioning, fixed
  positioning, or app-local popup stacks.
- Do not let paint order, hit order, focus order, dismissal order, and
  accessibility order diverge.
- Do not let component renderers own popover placement.
- Do not allow portals to bypass focus or event containment obligations.
- Do not let portal entries bypass existing surface classes, mosaic overlay,
  modal, floating, or transient-content placement posture. Portal entries are
  composition roots hosted by an admitted portal/surface handoff, not a
  replacement for structural overlay legality.

**Test requirements**
- Opening nested menus, popovers, modals, command palettes, and toasts updates
  one ordered portal-host receipt consumed by paint, hit testing, focus,
  dismissal, and accessibility.
- Invalid portal lane, modality, owner identity, anchor side, collision, or
  focus policy rejects before the portal list changes.
- A portal entry whose surface class, modality, or placement posture conflicts
  with mosaic/surface overlay legality rejects before the portal-host pancake
  list changes.
- Collision and flip behavior are deterministic from anchor observations and
  portal receipts.
- Graph-obligation tests prove portal reorder, modality, owner identity,
  dismissal, and focus-policy obligations are selected from portal graph
  touches rather than local overlay validators.

**Engineering decisions**
- Portal host owns cross-surface overlay order. Local draw order inside one
  container remains a composition/layout concern.
- Mosaic/surface placement owns whether a portal entry is legally overlay,
  modal, floating, transient, or status content. The portal host orders admitted
  entries; it does not grant placement authority.
- Anchored placement consumes host observations but not renderer policy.
- Dropdown popups, menus, tooltips, command palettes, and toasts all share the
  portal host with different admitted posture.
- Portal behavior uses one portal host with typed lanes such as tooltip,
  popover, menu, modal, command-palette, toast, and diagnostic. Lane policy is
  typed; separate portal hosts are not used to encode policy differences.

**Open questions**
- None.

### Phase 19: Collection Composition And Virtualized Windows

This phase admits repeated UI without turning collections into ordinary stacks
or app-local loops.

**Relevant subsystems**
- collection layout
- Query-bound projection consumption
- virtualized execution lanes
- composition graph indexes

**Relevant APIs**
- Query projection fact receipts
- Query graph read access planning receipts
- collection visible-range receipts
- mounted composition child allocation receipts

**Warnings**
- Do not materialize off-screen rows for friendly authoring.
- Do not use display text or index position as item identity.
- Do not build UI-local loading, stale, empty, or denied status maps.
- Do not implement scroll ownership as renderer `overflow` folklore.

**Test requirements**
- Visible-range counters prove off-screen items are not rendered or measured
  unless explicitly admitted.
- Query-bound collection windows consume Query-owned projection facts,
  support/admission posture, and async/result-state posture.
- Graph-read planning tests prove collection windows and item identity consume
  graph/index access plans rather than local row walks.
- Invalid iteration, identity, visible range, item sizing, sticky region, or
  scroll-anchor declarations produce one typed collection report.

**Engineering decisions**
- Collection composition is a separate family because huge repeated surfaces
  have different cost and visibility contracts from local flow layout.
- Item composition uses the same child node vocabulary as static composition,
  but identity, visible range, and virtualization posture are collection facts.
- Collections must expose bounded counters in the product-visible evidence path.
- Sticky headers and grouped sections ship in this phase as admitted collection
  features because real lists, tables, inspectors, chats, and rails need them
  to avoid renderer-local scroll/layout exceptions.

**Open questions**
- None.

### Phase 20: Adaptive Composition Alternatives

This phase admits responsive and posture-based alternatives as canonical
runtime plans, not renderer width checks.

**Relevant subsystems**
- adaptive layout
- density and platform posture
- state reconciliation
- projection rebind coordinator

**Relevant APIs**
- named measurement and density facts
- host measurement observation receipts
- composition graph alternative receipts
- durable-state reconciliation receipts

**Warnings**
- Do not model adaptive behavior as CSS media queries or `vh`/`vw` unit soup.
- Do not let renderer code choose alternatives from raw width checks.
- Do not preserve state across alternatives unless identity and eligibility
  prove it.
- Do not collapse density/theme changes into broad app rebuilds.

**Test requirements**
- Width, density, platform, and runtime-posture alternatives lower to
  deterministic canonical graph facts.
- Resizing selects an admitted alternative through host observation facts and
  adaptive receipts, not adapter conditionals.
- Eligible focus, scroll, splitter, selection, and input state carry forward
  across alternatives with receipts.
- Invalid breakpoint, density, platform posture, or carry-forward declarations
  produce one adaptive admission report.

**Engineering decisions**
- Adaptive alternatives are authored composition alternatives with explicit
  basis and preservation posture.
- Host width is an observation. Alternative selection is runtime meaning.
- Adaptive layout composes with mosaic, flow, overlay, collection, appearance,
  interaction, and motion facts.
- Validation exercises width class, density, reduced-motion, theme mode, and
  input modality (`pointer`/`keyboard`) platform postures in this milestone.
  Broader OS-native postures wait for native integration but must expose typed
  unsupported posture where referenced.

**Open questions**
- None.

### Phase 21: Motion Over Composition Changes

This phase adds time as authored runtime meaning over admitted composition,
appearance, and geometry facts without allowing renderer-local animation
policy.

**Relevant subsystems**
- motion recipes
- appearance recipes
- layout allocation receipts
- host time observations

**Relevant APIs**
- stateful appearance receipts
- layout allocation receipts
- host elapsed-time observations
- graph obligation receipts for motion activation

**Warnings**
- Do not introduce arbitrary animation callbacks or script.
- Do not make motion mutate authoritative state.
- Do not let reduced-motion policy be renderer folklore.
- Do not let visual interpolation desync from hit testing, focus, scroll
  anchoring, or event regions.

**Test requirements**
- Hover, press, focus, disabled, presence, expansion, collapse, and appearance
  transitions consume admitted motion recipes and host-time observations.
- Press interrupting hover, hot reload during active motion, and reduced-motion
  policy changes resolve through deterministic interruption, retarget, cancel,
  or replacement receipts.
- Layout-motion geometry reconciles draw frames, hit testing, focus, scroll
  anchoring, and diagnostics through one geometry receipt.
- Unsupported animated fields, malformed duration, invalid easing, or unsafe
  geometry motion reject before active receipts change.

**Engineering decisions**
- Appearance motion ships before or beside layout motion only when geometry
  implications are explicit; both consume the same composition graph identity.
- Active motion is derived runtime state over authored recipes, prior active
  receipts, reduced-motion policy, and host time observations.
- Renderer code may interpolate from receipts but cannot choose easing,
  interruption, retarget, cancellation, reduced-motion, or legality.
- The admitted easing vocabulary ships with `linear`, `ease_in`, `ease_out`,
  `ease_in_out`, `emphasized`, and deterministic `spring`/`snappy` posture
  where the runtime can prove replay-safe parameters. Duration and delay use
  named timing tokens, and reduced-motion replacement posture is required.

**Open questions**
- None.

### Phase 22: Reusable Composition Recipes And Defaults

This phase admits reusable composition recipes for common authored patterns
without turning them into component-local layout code. Recipes can provide
default composition graphs, primitive bindings, context hooks, and slot
contracts while still lowering into ordinary composition nodes and edges.

**Relevant subsystems**
- component capability registry
- composition graph lowering
- primitive schemas and defaults
- source authoring and diagnostics

**Relevant APIs**
- component capability descriptors
- primitive family default receipts
- composition graph root and instance receipts
- source admission reports

**Warnings**
- Do not create a second template language.
- Do not make recipes opaque renderer functions.
- Do not let recipe defaults bypass primitive schemas, graph facts, or
  composition receipts.
- Do not let app authors copy/paste large graph fragments because there is no
  reusable recipe boundary.

**Test requirements**
- A reusable field recipe, action-row recipe, card-section recipe, and
  toolbar-section recipe lower to ordinary composition nodes, edges, context,
  and primitive receipts.
- Overriding recipe slots, primitive values, context inputs, or child
  participation changes only the affected graph facts and preserves recipe
  identity where eligible.
- Invalid recipe slot fills, unsupported default overrides, duplicate slot
  identities, and missing required slot content reject through typed reports
  with affected instance rows.
- Cross-instance tests prove one recipe definition can be used on multiple
  pages or surfaces without shared mutable state, id collisions, or broad
  rebinds.

**Engineering decisions**
- Recipes are authored shortcuts over the canonical graph, not new runtime
  authority.
- Component capabilities may supply default recipes, but the resulting
  instance still consumes ordinary composition, primitive, context, and
  allocation receipts.
- Recipe identity and instance identity are distinct facts so shared edits and
  instance-specific edits have different rebind breadth.
- Recipes support named slots, collection item templates, and bounded repeated
  slots in this milestone. Arbitrary recursion, unconstrained macros, and
  general-purpose template execution are excluded.

**Open questions**
- None.

### Phase 23: Atom And Molecule Reuse Proof

This phase proves generic composition by building button-like and second
component-like surfaces from the same composition, content, appearance,
interaction, sizing, motion, and diagnostics receipts.

**Relevant subsystems**
- component capability registry
- primitive content, flow, appearance, interaction, and motion
- composition graph
- projection rebind coordinator

**Relevant APIs**
- component capability descriptors
- mounted composition receipts
- primitive family admission reports
- interaction activation receipts

**Warnings**
- Do not make button the primitive architecture.
- Do not add `button_*`, `card_*`, or `row_*` prop explosions for shared
  primitive meaning.
- Do not make reuse visual only; interaction, event geometry, state,
  diagnostics, and motion must reuse the same families too.
- Do not turn this into a broad component library milestone.

**Test requirements**
- A button-like atom and a second row/card/menu-item-like component consume the
  same primitive and composition receipt types.
- Shared content and appearance recipes affect all declared consumers through
  dependency contracts and bounded affected-consumer rows.
- Nested interaction containment emits distinct parent and child receipts
  without component-local event code.
- One invalid shared recipe consumed by both components produces one
  schema-owned denial basis with all affected consumers reported through
  dependency/rebind evidence.

**Engineering decisions**
- Button is a proof of composition, not the source of composition.
- Component capabilities add capability support posture and default recipes;
  they do not own private layout, content, interaction, or motion universes.
- This phase is the acceptance bridge from primitive/live-view proof to later
  product component milestones.
- The second proof component is a clickable card or row, not a menu item. A
  card/row exercises nested content, shared appearance, hover/pressed/disabled
  state, nested interaction containment, collection readiness, and payload
  reuse more broadly; menu behavior is already covered by portal/dropdown
  phases.

**Open questions**
- None.

### Phase 24: Composition Workbench Proof

This phase builds one serious running validation surface that exercises the
composition graph in human-visible terms and certifies hot reload, diagnostics,
and bounded execution.

**Relevant subsystems**
- validation app
- mounted product view
- source hot reload
- diagnostics and performance counters

**Relevant APIs**
- runtime candidate admission and activation receipts
- mounted composition receipts
- graph/index access-plan receipts
- runtime evidence and diagnostic rows

**Warnings**
- Do not prove only a centered primitive or a single happy-path form.
- Do not hide missing composition support behind validation-app renderer
  branches.
- Do not require restart for any authored composition, style, layout,
  participation, content, payload, or diagnostic presentation edit.
- Do not manually inspect the screen as the only proof of success.

**Test requirements**
- A running app hot reloads a card with two equal-fill controls, a dropdown
  projection swap, conditional third input, right-aligned submit action,
  payload-shape edit, inline errors, and disabled/readiness posture without
  Rust recompilation.
- The same app hot reloads nested card, toolbar, menu/popover, collection row,
  and adaptive alternative scenarios through one composition graph path.
- Evidence rows show changed facts, consumed facts, graph obligations, access
  plans, allocation counters, affected consumers, preserved state, and denied
  candidate posture.
- Static guards and compile-fail coverage prove validation app code cannot
  mint composition receipts, allocation receipts, graph proof, or renderer-local
  semantic layout authority.

**Engineering decisions**
- Manual proof scenarios must be written in realistic product language:
  "make inputs equal width and move submit to the action row," not abstract
  graph jargon.
- The workbench is a consumer of the platform, not an alternate architecture.
- The proof must include rejection and preservation cases, not only valid hot
  reloads.

**Open questions**
- None.

## Must Ship

- runtime-owned composition graph with typed nodes, edges, participation,
  order, policy attachments, graph facts, and mounted receipts
- composition roots bound to page slots, surfaces, portal entries, diagnostic
  panels, collection items, and reusable component instances through existing
  surface, page/content slot, and component-instance mount authority
- authored source and Rust-authored composition inputs that lower to the same
  canonical composition graph
- graph/index access planning and no-N+1 receipt proof for composition reads
- semantic context propagation for theme, density, disabled/inert posture,
  validation state, focus scope, and runtime mode
- host measurement observation lane for bounds, text metrics, icon metrics,
  DPI, viewport, scroll, and time where needed
- layout allocation receipts for nested containers, fill/hug/fixed sizing,
  gap, padding, baseline, alignment, and conditional participation
- scroll, clip, hit-test, viewport, focus, and accessibility participation
  boundaries derived from composition receipts
- content anatomy nodes for text, icon, spacer, image-ready content, divider,
  labels, helper/error text, and reusable groups
- controls and interactions mounted as generic composition children
- declarative expression, validation, condition, readiness, and payload
  projection receipts over runtime state and Query projection facts
- expression capability registration for app-defined operators with declared
  inputs, outputs, dependency contracts, purity, cost posture, support posture,
  diagnostics, planning, evaluation, and mounted evidence
- reconciliation receipts for moves, projection swaps, conditional absence,
  and target binding drift
- mounted diagnostics as ordinary composition nodes
- runtime portal host for overlays and anchored composition
- collection composition with visible-range, identity, virtualization, and
  Query-backed posture
- adaptive composition alternatives selected through admitted runtime posture
  facts
- motion recipes over composition, appearance, and admitted geometry changes
- reusable composition recipes and defaults that lower to ordinary graph nodes,
  slots, primitive receipts, and instance receipts
- atom and molecule reuse proof using shared primitive/composition receipts
- validation workbench proving arbitrary composition hot reload without local
  renderer meaning

## Must Preserve

- Worth UI remains above `egui` and does not become a web runtime, DOM clone,
  CSS clone, or app-local widget bundle
- mosaic remains responsible for shell and page topology; composition graph
  owns local product anatomy and nested content arrangement
- composition graph consumes mosaic structural legality, region placement,
  sizing, scroll, clipping, hit-test, persistence, and focus receipts rather
  than replacing or restating them
- surfaces remain product-facing placement identity, not generic local layout
  boxes
- page, surface, portal, diagnostic, collection item, and component-instance
  roots remain distinct authority facts rather than one hidden global root
- Query remains owner of Query truth, graph read access planning, projection
  facts, support/admission posture, async/result posture, and recovery posture
  where those surfaces are involved
- reusable recipes remain authored shortcuts over canonical composition, not
  opaque renderer functions or a second template runtime
- expressions remain typed and bounded runtime projections with capability
  extension hooks, not arbitrary scripts, renderer callbacks, or form-local
  validators
- authored meaning lowers once into canonical artifacts, graph facts, and
  sealed receipts before renderer execution
- host adapters allocate pixels, execute host mechanics, and report neutral
  observations without deciding semantic layout, state, diagnostics,
  interaction, or motion meaning
- no per-frame source interpretation, registry string lookup, broad artifact
  scan, broad graph scan, or hidden N+1 traversal returns through convenience
  APIs
- invalid candidate composition preserves prior valid runtime truth and
  surfaces typed diagnostics
- performance claims remain counter-backed and visible to product proof

## Acceptance Evidence

- a running validation app can be started once and then hot reload arbitrary
  nested composition edits across multiple pages and surfaces, including
  equal-width rows, separate action rows, conditional controls, projection
  swaps, payload-shape edits, reusable recipes, overlays, collections,
  adaptive alternatives, content changes, and motion recipes
- denied edits preserve prior mounted receipts and display receipt-derived
  diagnostics with source spans, counters, stable denial-set digests, and
  affected-consumer rows
- page/root move tests prove eligible state, focus, diagnostics, and
  interaction identity survive movement between composition roots without
  renderer remount folklore
- mosaic/surface integration tests prove composition roots cannot mount into
  illegal regions, bypass surface placement classes, override shell scroll or
  focus ownership, or reinterpret mosaic sizing overflow behavior
- semantic context tests prove disabled/inert, density, theme, validation,
  focus scope, and override posture propagate through graph facts and can deny
  illegal overrides
- graph/index receipts prove composition reads use admitted access plans rather
  than recursive walks, app-local maps, or broad scans
- layout allocation receipts prove equal fill, hug, fixed, min/max,
  participation, baseline, gap, padding, and alignment decisions are runtime
  meaning
- expression tests prove conditional participation, validation state,
  readiness, payload shape, custom expression operators, and Query-backed
  derived values consume typed runtime facts and capability descriptors instead
  of form-local code
- expression capability tests prove app-defined operators register through the
  facade, lower into evaluation plans, publish bounded output facts, expose
  mounted diagnostics, and rebind only declared consumers
- expression hostile tests prove invalid custom operator registration,
  undeclared reads, non-deterministic output, mutation attempts, broad
  recompute, missing registration, incompatible output changes, and local
  renderer evaluation are denied or made uncompilable before they can affect
  mounted product truth
- scroll/clip/focus/accessibility tests prove off-viewport, hidden, disabled,
  inert, clipped, and absent-retaining-state nodes have consistent layout,
  hit-test, focus, accessibility, and diagnostic posture
- recipe tests prove shared recipes can be reused across pages and surfaces
  without shared mutable state, id collisions, or broad rebinds
- renderer-boundary tests prove semantic composition APIs are unavailable
  outside approved adapters and approved adapters consume receipts rather than
  authored declarations
- compile-fail tests prove app code cannot mint composition, allocation,
  motion, portal, collection, diagnostic, graph proof, or changed-fact
  dependency receipts
- replay and equivalence tests prove semantically equivalent composition inputs
  lower to equivalent graph and mounted receipts
- counter tests prove steady-frame rendering does not parse source, scan
  artifacts, or rediscover graph topology

## Sequencing Notes

Milestone 4.1 follows Milestone 4S because the hot reload spine must exist
before arbitrary composition can rebind honestly. It precedes command, focus,
Query-bound view, form, and component-library milestones because those
milestones all need nested composition, stable reconciliation, layout
allocation, diagnostics, portals, collections, and adaptive alternatives to be
platform facts rather than per-feature conventions.

The former Milestone 4S Phase 35-41 material belongs here because motion,
overlay, collection, adaptive layout, button composition, and cross-component
reuse are not tail-end hot reload patches. They are consumers and proofs of the
runtime composition graph.
