# Worth UI AI Diagnostics and Inspection Architecture

## Purpose

This document defines how AI inspection, human diagnostics, replay, visual
evaluation, and runtime explanation should work in Worth UI.

It is not a debug-feature wishlist, an implementation checklist, or a panel
mockup.

Its job is to lock the architecture early enough that:

- AI support does not become a pile of screenshots plus logs
- the human inspector does not become a second explanation system
- diagnostics do not become ad hoc text dumps
- replay does not become a test-only side lane
- visual evaluation does not depend on pixel guessing when runtime geometry is
  already available

The central question is:

```text
How should Worth UI expose runtime truth, evidence, and explanation so that
both AI agents and humans can inspect, debug, and validate the live UI without
creating a second runtime or a second diagnostic folklore layer?
```

## Thesis

The correct move is:

```text
build one runtime-owned evidence and inspection substrate
then expose it through two consumers:
  - AI inspection tools
  - a human inspector surface
```

Do not build "AI diagnostics" and "inspector diagnostics" as separate systems.

Do not treat screenshots as the main truth.

Do not treat logs as the public inspection contract.

The runtime already owns stronger semantic meaning than normal UI stacks:
declarations, admitted identities, aspect contracts, graph topology, Query
bindings, measurement plans, execution-plan decisions, frame-cost receipts,
mounted receipts, host observations, rebind causality, and denial posture.

The inspection architecture must expose that meaning directly.

## Core Rule

The governing rule is:

```text
host code may allocate pixels and report observations
only runtime-owned truth and runtime-owned evidence may explain visible UI
meaning
```

That means:

- renderer code does not explain why something exists, changed, denied, or
  aligned
- logs may present evidence, but logs are not the evidence contract
- screenshots may assist inspection, but screenshots are not the primary truth
- diagnostics panels may project evidence, but panels do not author diagnostic
  truth
- AI tools may query evidence, but tools do not reconstruct meaning from host
  behavior

The host corollary is:

```text
worth-ui-host-contract is the stable native-host boundary
worth-ui-host-egui is only the first adapter implementation
adapter-specific mechanics must not become runtime truth or public host law
```

## Relationship To The Existing Runtime

This document extends the architecture in
[Worth UI runtime orientation](../../workspaces/worth-ui/docs/worth-ui-readme.md) and
the roadmap in [worth_ui_roadmap.md](./worth_ui_roadmap.md).

It does **not** propose a second truth graph.

The better model is:

```text
authority graph = runtime truth about what exists and participates
inspection substrate = typed evidence and indexes over runtime truth,
declaration artifacts, obligations, receipts, observations, and rebind history
```

That distinction matters.

The authority graph is where UI truth lives.

The inspection substrate is how that truth becomes explainable, replayable,
queryable, and relevant to a specific question.

If the implementation creates a second graph-shaped authority system for
inspection, it has likely crossed the boundary and started rebuilding the
runtime.

## Why This Must Start Early

AI inspection and diagnostics are not end-of-roadmap polish.

They are runtime pressure.

If they arrive only after declarations, graph ownership, measurement, mounting,
services, and hot rebind are already built, then one of two bad things usually
happens:

- the architecture turns out not to be explainable without local hacks
- the team builds side channels that bypass runtime truth because the formal
  lanes were never made inspectable

Worth UI should take the opposite approach:

```text
every serious runtime family must ship with:
  - truth ownership
  - typed evidence artifacts
  - targeted inspection queries
  - replay hooks where relevant
  - relevance filters
```

This is especially important for AI.

An AI agent does not need the full human diagnostics panel on day one. It needs
a formal, typed, scoped entry point into runtime evidence.

That harness should exist in the first serious hot-composition milestone.

## Architectural Goal

The best possible end state is:

```text
AI sees the current frame
-> targets a node, point, or source edit
-> asks a typed inspection query
-> receives a scoped evidence slice with stable identities
-> optionally replays the change through stop points
-> patches source at the authored boundary
-> the runtime hot reloads
-> updated evidence proves whether the repair is correct
```

For humans, the same substrate should make it possible to:

- click a visible element and jump to its declaration
- see why a node is visible, hidden, disabled, denied, or remounted
- inspect what aspects were published or consumed
- inspect why a layout box was allocated as it was
- inspect what Query projection the UI consumed
- inspect why a replay widened or stayed local
- inspect whether alignment, symmetry, spacing rhythm, or focus topology are
  correct

## Non-Goals

This architecture must not devolve into:

- a browser-DevTools clone with web baggage
- a console log surface with better formatting
- a screenshot-only AI workflow
- a renderer-local debug overlay that bypasses runtime receipts
- a second truth runtime for diagnostics
- an inspector that owns diagnostic truth
- a one-off debug surface that only works for the current demo

The goal is not:

```text
teach the AI to read pixels better
```

The goal is:

```text
let the AI ask semantically precise runtime questions using the same language
the architecture already uses
```

### Milestone 3.1 Non-Goals

Milestone 3.1 specifically does not implement:

- DSL parsing
- canonical UI declarations
- authority graph topology
- aspect contracts
- measurement/allocation
- mounted receipts
- visual snapshots
- replay
- AI screenshot tools
- human inspector UI
- Query projection binding
- portal/focus/motion services

Milestone 3.1 only creates the enforced homes, public lifecycle, inspection
contract shape, support posture, unsupported posture, and anti-bypass proof
that later milestones must use.

## Boundary Matrix

| Crate | Owns | Must Not Own |
| --- | --- | --- |
| `worth-ui` | product facade | runtime internals |
| `worth-ui-dsl` | source/semantic DSL boundary | graph truth |
| `worth-ui-runtime` | hot-composition truth | host mechanics |
| `worth-ui-inspection` | inspection contracts/evidence | panel UI truth |
| `worth-ui-query-binding` | Query consumption boundary | Query authority |
| `worth-ui-host-contract` | native host boundary facts | egui mechanics |
| `worth-ui-host-egui` | egui translation | UI meaning |
| `worth-ui-certification` | anti-bypass proof | production runtime truth |

## Evidence Substrate

The inspection substrate should own typed evidence families such as:

- declaration artifact evidence
- admission and denial evidence
- authority-graph identity and topology evidence
- aspect publication and consumption evidence
- graph-touch and obligation-selection evidence
- Query binding and projection-consumption evidence
- measurement and allocation evidence
- execution-plan lowering, equivalence, activation, and frame-cost evidence
- mounted receipt evidence
- host-observation evidence
- rebind and preservation evidence
- diagnostic evidence
- visual snapshot evidence
- visual geometry and visual invariant evidence
- replay timeline and replay-step evidence

In the earliest support-bearing slice, the substrate should also own support
and closure artifacts such as:

- `UiInspectionSupportReport`
- `UiInspectionScopeSupportRow`
- `UiInspectionClosureReport`

Each family must have:

- stable identity
- provenance
- semantic category
- causal links to upstream and downstream evidence
- source-span readiness where relevant
- relevance filtering support
- materialized detail that can be loaded lazily

Support reporting is not optional garnish.

The runtime should be able to say, structurally:

```text
scope: measurement
status: unsupported
reason: belongs-architecturally-not-yet-admitted
milestone_expected: 3.6 or later
```

That is stronger than returning `unsupported` from a query without saying what
the architecture expects.

### Evidence Is Not Logs

Logs are presentation.

Evidence is typed runtime output.

The public lane should be evidence first, with logs, test output, and UI panels
rendering projections of that evidence.

This prevents the usual failure mode where tools start matching strings instead
of consuming typed structure.

### Evidence Must Be Indexed

The substrate should expose indexes that make targeted questions cheap and
honest.

Important indexes include:

- declaration identity -> evidence sets
- source span -> declaration / diagnostics / rebind evidence
- graph node identity -> obligations / bindings / receipts / diagnostics
- active plan identity -> lowering basis / equivalence decision / activation /
  lane receipts / frame-cost evidence
- plan handle -> exact plan generation / handle family / admitted target
- mounted receipt identity -> visible region / source / graph node / services
- published aspect -> publishing nodes / receipts
- consumed aspect -> dependent nodes / obligations / receipts
- changed fact -> affected rebind evidence
- diagnostic identity -> attached evidence neighborhood
- screen point or region -> mounted receipt identity
- frame identity -> visible snapshot / mounted receipts / replay step

Without these indexes, inspection drifts back toward scans, dumps, and
hand-built explanations.

## AI Inspection Protocol

The AI should not receive a random bag of debug commands.

It should receive a typed inspection query surface.

Example shape:

```text
UiInspectionQuery {
  target:
    - declaration(identity)
    - graph_node(identity)
    - mounted_receipt(identity)
    - source_span(file, range)
    - screen_point(x, y)
    - screenshot_region(identity)
    - diagnostic(identity)
    - frame(identity)
    - replay_step(identity)

  scope:
    - declaration
    - admission
    - graph
    - aspects
    - obligations
    - query_binding
    - measurement
    - execution_plan
    - plan_equivalence
    - frame_cost
    - mounting
    - host_boundary
    - services
    - rebind
    - diagnostics
    - replay
    - visual_geometry

  richness:
    - summary
    - evidence_refs
    - materialized_detail
    - causal_trace

  relevance:
    - attached_only
    - nearest_cause
    - denied_only
    - unsupported_only
    - changed_since(frame)
    - source_edit(change_identity)

  budget:
    max_nodes
    max_edges
    max_bytes
}
```

This is the right shape because it forces inspection to stay:

- target-aware
- scope-aware
- budget-aware
- relevance-aware
- identity-backed

The companion support/reporting lane should be able to answer:

- does this target/scope belong architecturally?
- is it admitted yet?
- if not, why not?
- when is it expected to become admitted?

The point is to make “not yet” machine-checkable instead of forcing clients to
guess from missing behavior.

### AI Tools

The formal tool lane should support capabilities like:

- `capture_frame`
- `capture_node`
- `inspect_at_point`
- `inspect_target`
- `inspect_source_span`
- `inspect_diagnostic`
- `inspect_rebind`
- `inspect_layout`
- `inspect_query_binding`
- `inspect_execution_plan`
- `inspect_host_observation`
- `explain_visibility`
- `explain_operability`
- `explain_allocation`
- `explain_plan_equivalence`
- `explain_frame_cost`
- `explain_rebind`
- `diff_frames`
- `list_relevant_diagnostics`
- `start_replay`
- `step_replay`
- `compare_replay_points`
- `evaluate_alignment`
- `evaluate_spacing`
- `evaluate_symmetry`
- `show_visual_overlay`

The important part is not the tool count.

The important part is that each tool must query the same evidence substrate
instead of scraping logs, reading private host fields, or interpreting pixels
in isolation.

Inspection receipts must be sealed but projectable.

That means:

- public consumers may inspect identity, target, scope, posture, budget, and
  evidence refs
- public consumers may project those receipts into AI responses, tests, logs,
  or inspector UI
- only the inspection runtime may construct receipt identities, posture
  witnesses, support rows, or evidence claims

The wrong alternatives are:

- externally WORTHable receipts, which make proof meaningless
- privately unreadable receipts, which make the runtime impossible to consume

## Visual Snapshots

Worth UI supports screenshots, but a screenshot is not the public truth
object. The implemented public object is
`UiVisualSnapshotReceipt<ArtifactPosture>`: a bounded, immutable evidence
bundle for one exact presentation basis.

Its implemented shape includes:

```text
UiVisualSnapshotReceipt<ArtifactPosture> {
  UiVisualSnapshotIdentity
  UiVisualSnapshotAffinity {
    presentation_attempt
    frame
    semantic_surface
    host_surface
    binding_generation
    presentation_epoch
    relation
  }
  UiVisualCoordinateObservation
  UiVisibleRegionIndexIdentity
  UiHitTestRegionIndexIdentity
  optional or required UiVisualPixelArtifact
  UiVisualInspectionCostReceipt
  UiVisualSnapshotEvidence
}
```

Coordinates created through `UiVisualCoordinateScope` cannot escape their
snapshot. Point and region adjudication keep visible contributors distinct from
the total-ordered hit-test target. Each result may carry
`UiVisualIdentityTrace`, which follows mounted receipt and incarnation through
graph and declaration identity to authored source provenance and typed evidence
references.

The host supplies capture and presentation observations. The runtime supplies
the meaning those observations explain. AI and human consumers project the
same receipt, omission, denial, indeterminate posture, cost, and lifecycle
evidence; neither consumer reconstructs authority from pixels.

### Implemented Visual Snapshot Closure: Milestone 3.11

The 3.11 snapshot lane is honest because the system can:

- capture the current frame, a retained presentation, a selected mounted node,
  or a snapshot-scoped region;
- map a client physical point or region to distinct visible and hit-test
  outcomes;
- map mounted receipt identity to graph, declaration, authored source, and
  evidence references;
- retain, supersede, expire, cancel, dispose, and shut down snapshots through
  typed bounded lifecycle outcomes;
- publish and clear a fixed identity overlay through successor mounted frames;
  and
- keep optional pixels as disposable evidence rather than truth.

### Committed Successor: Milestone 3.12

Identity-aware predecessor/successor snapshot comparison is not a 3.11
capability. Milestone 3.12 owns the first admitted comparison that can relate
changed facts, preserved identity, remount decisions, and bounded hot rebind.
Until that contract exists, neither raw pixel diff nor local identity matching
may be presented as semantic rebind evidence.

## Replay

Replay should be a first-class runtime protocol, not a testing afterthought.

The AI and the human inspector both need it.

Replay lets the runtime answer:

- what changed?
- when did the meaning diverge?
- where was the first denial introduced?
- why did this rebind widen?
- why did this node preserve identity?
- why did this node remount?

### Replay Stop Points

Replay should support meaningful stop points such as:

- after parse
- after semantic lowering
- after canonical declaration artifact
- after admission
- after graph touch
- after obligation selection
- after Query binding
- after measurement planning
- after committed allocation
- after execution-plan lowering
- after plan equivalence/no-op classification
- after plan activation
- after frame execution
- after mounted receipts
- after host observation intake
- after rebind planning
- after diagnostics

That enables targeted questions like:

- replay the last edit until admission
- replay the failed change until the first denial point
- compare before/after mounted receipts by aspect
- stop at rebind planning and explain why breadth widened

## Relevance Filtering

One of the most important parts of this architecture is relevance.

The AI almost never wants "all diagnostics."

It wants:

- diagnostics relevant to this node
- diagnostics relevant to this source edit
- diagnostics relevant to this failed rebind
- diagnostics relevant to this screenshot region
- diagnostics relevant to unsupported aspects
- diagnostics relevant to this denied service attachment

The same rule applies to humans using the inspector.

The inspection substrate must therefore provide relevance filters as first-class
runtime features rather than leaving the panel or tool client to guess.

## Human Inspector Surface

The human-facing surface should not be framed as "browser DevTools for Worth."

That mental model is too web-shaped.

This surface is better described as:

- Worth Inspector
- Runtime Inspector
- Evidence Inspector
- Composition Inspector

Its job is to inspect runtime authority and runtime evidence, not HTML nodes or
CSS style cascades.

### Inspector Views

Useful first-class views include:

1. Visual Tree  
   Mounted receipt topology, visible region identity, declaration identity,
   graph node identity, and diagnostic badges.

2. Authority Graph  
   Runtime topology for pages, page sets, mosaics, regions, controls, portals,
   focus scopes, services, and diagnostics attachments.

3. Aspect Inspector  
   Published aspects, consumed aspects, changed aspects, aspect coverage,
   aspect-fit denials, and aspect-sensitive obligations for the selected target.

4. Rebind Timeline  
   Changed fact, affected aspects, invalidated obligations, preserved nodes,
   remounted nodes, denial artifacts, and updated mounted receipts.

5. Measurement Inspector  
   Constraints, sizing mode, intrinsic requests, host measurements, allocation
   plan, overflow posture, and scroll ownership.

6. Query Binding Inspector  
   Query artifact consumed, basis/world posture, projection facts, schema
   posture, async/result posture, payload shape, and invalidation posture.

   The current projection evidence family correlates a Query transition or
   attempt, shape-specific affine fact, application generation, mounted node
   and frame, presentation attempt, and visible pixels. Availability,
   current/stale activity, stop kind, compatibility, native family,
   collection continuation, and structural cost remain separate typed fields.
   Compact identities are retained first; detail is materialized lazily under
   explicit evidence, disclosure, and retention budgets.

   Support reporting distinguishes the installed product backend and consumer
   contract from unsupported, remasked, wrong-world, stale-generation,
   incompatible-schema, and expired-detail posture. A matching identity,
   digest, diagnostic string, pixel, or inspection record cannot construct a
   binding, fact, Query operation, rebind plan, or publication authority.

7. **Execution Plan And Frame Cost** -- lowering authority, active plan
   generation, host-neutral lane partitions, typed handle families, exact
   equivalence/no-op decision, affected closure, activation receipt, and
   ordinary versus reconstructive cost counters.

8. **Services Inspector** -- portal topology, focus routing, motion, command
   routing, selection, scroll, and other runtime services.

9. **Diagnostics Feed** -- typed, filterable diagnostics grouped by relevance,
   not a console.

10. **Replay Timeline** -- source edits, artifacts, admissions, graph mutations,
    observations, rebinds, mounted frames, and diagnostics.

11. **Visual Evaluation** -- alignment groups, baselines, spacing rhythm,
    symmetry axes, visual bounds, overlays, invariant violations, and
    perceptual advisories.

### Dogfooding Rule

The inspector should, where feasible, be authored and rendered through Worth
UI itself.

That proves:

- diagnostics can mount through the standard path
- inspection can be product-facing rather than side-loaded
- Query bindings can power complex support surfaces
- services such as portals and focus survive pressure

But the inspector must never become the source of truth.

It consumes runtime evidence. It does not mint it.

## Visual Evaluation And Design Invariants

Worth UI should support a category stronger than "eyeball the screenshot."

This document calls that category:

```text
visual evaluation and design invariants
```

This family owns questions like:

- do these text baselines align?
- do these labels share a leading edge?
- do these inputs occupy equal allocated width?
- is spacing rhythm consistent?
- is this icon optically centered with its text?
- are these controls symmetric around the parent centerline?
- do visual bounds match declared or allocated expectations?
- did geometry stay correct while paint visually drifted?

### Two Kinds Of Visual Evaluation

Worth UI should distinguish:

1. Runtime-semantic checks  
   These are based on receipts, anchors, baselines, bounds, groups, and
   declared tolerances. They should be deterministic and inspectable.

2. Perceptual checks  
   These are based on screenshot pixels, rasterization, anti-aliasing, and
   visual mass. They are valuable, but usually advisory unless promoted into a
   declared invariant.

The rule is:

```text
receipt-backed first
screenshot-confirmed second
```

### Visual Evidence Families

Examples of useful evidence families:

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

### Advisory vs Blocking

Not every visual finding should fail the world.

Worth UI should support at least three levels:

- declared invariant violation
- design advisory
- ad hoc inspection result

This allows authors to say:

- "these labels must align" as a declared invariant
- "this region feels optically heavy" as an advisory
- "show me near misses within 1px" as an ad hoc inspection query

## Diagnostics Are Not Special-Cased Text

Diagnostics must remain typed runtime artifacts.

They should preserve:

- identity
- source declaration identity
- world and support posture
- aspect posture
- denial or degraded reason family
- affected artifact identities
- evidence references
- relevance metadata
- presentation rows or projections

Tests and tools should consume those typed artifacts.

They should not match the final display string and pretend that is proof.

## Formal AI Harness

The first serious hot-composition milestone should establish a formal AI entry
point.

That harness is more important than a polished panel in the early sequence.

### Harness Requirements

At minimum, the AI harness should expose:

- frame capture by identity
- point and region hit testing
- target inspection queries with scoped evidence
- relevant-diagnostics lookup
- replay session creation and stepping
- rebind explanation
- frame diff by identity and aspect scope
- execution-plan lowering, equivalence/no-op, activation, and frame-cost
  inspection without exposing executable plan ownership
- visual evaluation queries for alignment, spacing, and symmetry

The harness should be:

- typed
- budgeted
- identity-backed
- replay-capable
- independent of ad hoc console output

### Harness Rule

If an AI repair workflow cannot be expressed through the formal harness without
reading a giant dump, the harness is not mature enough yet.

## Per-Milestone Integration Rule

The right roadmap posture is not:

```text
build the runtime first
add AI/diagnostics at the end
```

The right posture is:

```text
every milestone adds the evidence, inspection, and replay surfaces for the
runtime families it introduces
```

That means:

- declaration milestones add declaration inspection
- admission milestones add denial and support inspection
- graph milestones add topology and aspect inspection
- measurement milestones add allocation and geometry inspection
- execution-plan milestones add lowering, equivalence, activation, handle/lane,
  and frame-cost inspection
- mounting milestones add visible-region and mounted-receipt inspection
- services milestones add service topology inspection
- rebind milestones add change-diff and preservation inspection
- visual milestones add visual-evaluation inspection

This keeps inspection honest and keeps the runtime explainable while it is
still forming.

## Suggested Structural Boundaries

The final crate topology may vary, but the conceptual boundaries should remain
separate:

```text
worth-ui-inspection
  runtime-owned evidence/query/replay substrate

worth-ui-agent-tools
  AI-facing tool protocol over the inspection substrate

worth-ui-inspector
  human-facing inspector projections over the same substrate
```

The key is not the exact crate count.

The key is preserving the lifecycle boundaries:

- runtime evidence substrate
- AI tool surface
- human panel surface

They should not collapse into one implementation blob.

## Acceptance Standard

This architecture is only real if it can prove all of the following:

- AI can inspect a declaration artifact without a dump
- AI can ask why a node is visible, hidden, disabled, denied, rebound, or
  remounted
- AI can move from screenshot region to mounted receipt identity
- AI can move from mounted receipt identity to declaration, source, graph, and
  evidence
- AI can replay a change to the first denial point
- AI can explain why a candidate was a semantic no-op, required a bounded plan
  replacement, or was denied; which exact plan generation is active; and what
  work an ordinary frame performed
- humans can inspect the same evidence through the inspector
- diagnostics are relevant, typed, identity-backed, and filterable
- visual alignment and spacing can be evaluated from runtime geometry rather
  than screenshots alone
- the inspector consumes evidence but does not author truth
- plan and Query inspection cannot mint handles, activate candidates, promote
  receipts/digests into authority, or submit an executable plan
- projection evidence cannot be reassembled into a binding or fact, and lazy
  detail cannot widen Query, application, mounted, or disclosure authority
- no explanation path requires renderer-local semantic reconstruction

## Final Rule

Worth UI should aim for something better than conventional UI tooling:

```text
not screenshots plus logs
not devtools plus folklore
not AI guessing from pixels

but a live, replayable, semantically indexed runtime that can explain itself
through the same declaration, graph, aspect, Query, measurement, plan,
mounting, and service language that already defines product truth
```

That is the standard.
