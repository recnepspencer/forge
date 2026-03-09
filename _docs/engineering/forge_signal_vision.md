# Forge Signal Vision

## Thesis

Forge uses two distinct runtimes for two distinct jobs:

- `forge-relational` is the source-of-truth runtime. It owns identity, mutation, history, diffs, and traversal over the model graph.
- `forge-signal` is the derived-computation runtime. It owns invalidation, recomputation, scheduling, conditions, and transactional refresh of computed state.
- The bridge layer connects them. Relational changes become signal invalidations, and signals evaluate against stable host snapshots without becoming the owner of truth.

This split is not optional. `forge-signal` is scheduling and execution infrastructure, never truth. It exists to make derived computation deterministic, transactional, aspect-aware, and auditable while remaining fully decoupled from domain-specific storage and semantics.

`forge-signal` should be designed as a standalone generic library. CAD is a major target, but not the definition of the runtime. The same core should make sense for game engines, financial platforms, chip simulation, AI systems, and other domains where dependency-aware incremental computation matters.

## Mission

`forge-signal` is the universal derived-computation substrate for Forge. More generally, it is a generic incremental computation runtime for systems that need dependency-aware recomputation over host-managed state. Every expensive or dependency-sensitive derived value can be modeled as a signal node: derived artifacts, validation summaries, simulation results, risk calculations, pricing outputs, query results, or analysis pipelines. The runtime exists to answer one question reliably:

> Given a set of upstream changes, what must recompute, in what order, under what conditions, and with what causal trace?

The runtime must answer that question with five non-negotiable properties:

1. Deterministic execution
2. Transactional rollback semantics
3. Aspect-aware invalidation granularity
4. Explicit separation from truth-state storage
5. First-class observability into why recomputation happened

Diagnostics are not optional polish. `forge-signal` must assume there will be runtime bugs, host bugs, policy mistakes, and hard-to-reproduce invalidation pathologies. Provenance, inspection, and metrics are therefore part of the product contract, not just developer support tooling.

## Architectural Model

### Runtime stack

| Layer | Responsibility | Owns |
| --- | --- | --- |
| `forge-relational` | Truth-state graph runtime | identity, transactions, history, diffs, traversal, integrity |
| `forge-signal` | Derived-computation runtime | dependency DAG, invalidation, recomputation, scheduling, conditions, observability |
| Bridge / integration | Decoupled coordination | patch-to-invalidation, aspect mapping, snapshot evaluation, node-key mapping |

### What `forge-signal` owns

- Evaluation dependency graph scheduling and invalidation
- Deterministic ordering
- Conditional and policy-aware evaluation gates
- Transactional invalidation and hard rewind on failure
- Node-scoped execution metadata such as aspects, conditions, comparator policy, and telemetry
- Query-style incremental execution semantics
- Future execution planning, staged execution, and parallel dispatch

### What `forge-signal` does not own

- Truth-state graphs, domain entities, or structural storage
- Domain numerics, geometry kernels, topology mutation, or schema rules
- Semantic meaning of aspects beyond host-defined slots
- Host identity models, diffs, or lineage semantics
- Permanent fusion with relational storage
- Mandatory use of the bridge for standalone signal use

### Structural rule

Signals consume host snapshots and emit derived-state refresh. They do not become a second source of truth, and they do not own the structural graph they observe.

## Provenance Model

`forge-signal` must make it possible to explain why a node is in its current state, not merely that it was evaluated.

The provenance ladder should stay explicit:

- invalidation provenance: which upstream changes dirtied or maybe-staled the node
- dependency provenance: which dependencies were considered and how their versions compared
- condition provenance: which evaluation conditions deferred or allowed work
- comparator provenance: which changes were suppressed as not meaningful
- recomputation provenance: whether work actually ran and what trace summary it produced
- host causality metadata: optional upstream provenance attached by host runtimes or future bridge integration

This is the baseline for trust, debugging, compliance, and hard-software diagnosis. The explanation surface, metrics surface, and inspection APIs should all reinforce this same causal model rather than invent separate diagnostic stories.

## API Strategy

`forge-signal` should expose two public faces built on one runtime.

The product requirement is not only "easy mode for simple cases." The full-power surface must also be beautiful for expert users who need explicit control. In Forge, "beautiful" means explicit without boilerplate, powerful without generic noise, and predictable without ambient magic.

Both surfaces must remain directly usable without `forge-relational` or the bridge. The bridge is an integration layer, not the required entrypoint to the signal runtime.

### `forge-signal-core`

This is the low-level runtime surface. It keeps all control explicit:

- `SignalGraph`
- `SignalRuntime`
- transactions
- aspects
- evaluation conditions
- comparator policies
- explicit invalidation and evaluation entrypoints
- future planners and schedulers

This surface exists for kernel internals, integration layers, performance-sensitive paths, and any host that needs precise runtime control.

The design goal for this surface is not raw capability alone. The core API should read as a clean execution pipeline:

- configure runtime and policy once
- open a transaction or execution scope explicitly
- mark dirty inputs and emit staged changes
- evaluate targets under named policies
- commit or rollback deterministically

The hard-mode surface should therefore prefer:

- builder-driven construction over generic-heavy constructors
- fluent node and policy configuration over bulky config structs
- defaulted generics and obvious defaults for common advanced cases
- explicit extension points for comparators, conditions, planners, and bridge hooks
- local reasoning at the call site about what mutates, what evaluates, and what commits
- stable query-style execution surfaces for keyed computations and cache validation
- accessible, intention-revealing names such as `depends_on_aspects(...)` and `condition(...)` over compressed or insider-oriented terminology

### `forge-signal-easy`

This is the ergonomic signal surface. It is a core product pillar because the runtime should feel simple for common use, not only powerful for specialists.

The easy surface should provide:

- input signals
- computed signals
- effects/watchers
- batched updates
- automatic dependency capture
- implicit lazy pull on read for common workflows

The easy surface must compile down to the same runtime primitives as the core layer. It is a UX layer, not a separate execution engine.

### Design rule

Easy syntax must not imply easy internals. The low-level contract remains explicit, deterministic, transactional, and auditable even when the user-facing syntax feels Angular/Solid-style simple.

### Core API quality rule

The hard version must not feel like internal scaffolding. Advanced users should be able to express transactions, policy wiring, evaluation, and inspection in short linear flows without repeatedly spelling type noise or assembly-like setup.

### Naming rule

API language should optimize for first-read comprehension. Prefer accessible, slightly longer, intention-revealing names over compressed, overly technical, or clever names. If a longer name teaches the runtime model more clearly, prefer the longer name.

## Capability Matrix

Status meanings:

- `Implemented`: real in the current crate and covered by code/tests
- `Next`: intended near-term productization or runtime work
- `Later`: important, but deferred until the foundation above it exists

### Forge Signal

#### Foundation runtime

| Capability | Status | Notes |
| --- | --- | --- |
| Dependency DAG | Implemented | `SignalGraph` owns the evaluation graph and rejects cycles at runtime |
| Aspect / granularity system | Implemented | Aspects, masks, and per-aspect versions are present |
| Transactional invalidation | Implemented | Runtime-backed transactions hard-rewind via sparse patching |
| Maybe-stale states | Implemented | `Clean` / `MaybeStale` / `Dirty` are part of the core contract |
| Lazy recomputation | Implemented | Pull-based evaluation recomputes only when requested |
| Aspect-based invalidation | Implemented | Subscribers are dirtied according to changed aspect paths |
| Dynamic dependency discovery | Implemented | `EvaluationContext` records upstream reads explicitly |
| Conditional nodes | Implemented | `OnDemand`, `Debounce`, `AspectFilter`, `DeltaThreshold`, `Custom` |
| Tolerance / epsilon gates | Implemented | Comparator policies include exact and tolerance-based suppression |
| Deterministic execution behavior | Implemented | Current traversal/order semantics are intentionally deterministic |
| Telemetry baseline | Implemented | Runtime counters exist for evaluation, invalidation, rollback, GC, and gating |
| Serialization-friendly graph state | Implemented | Core graph and node data are serializable, though snapshot API is not yet first-class |

#### Incremental computation semantics

| Capability | Status | Notes |
| --- | --- | --- |
| Query-style incremental execution | Implemented | Keyed computation families and family-scoped lookup now exist; broader query ergonomics can deepen later |
| Reactive diff propagation / result diffing | Implemented | Output identity, output change reporting, and downstream suppression are now first-class |
| Partial recomputation boundaries | Implemented | Partition-aware outputs, changed-region metadata, and partition-scoped subscriptions now exist |
| Structural memoization | Implemented | First-pass explicit memoization exists through host-supplied structural keys and family-scoped caches |
| Speculative evaluation / branching | Later | Depends on branchable execution state and discard semantics |
| Fixed-point / convergence nodes | Later | Valuable for solver-style workloads, but not foundation-critical |

#### Introspection and debugging

| Capability | Status | Notes |
| --- | --- | --- |
| Execution trace / provenance baseline | Implemented | Trace summary, structured explanations, and optional causality metadata now exist |
| Explain / provenance API | Implemented | `explain(node)` is now a first-class structured surface with human-readable display |
| Diagnostics-first inspection model | Implemented | Explanation, inspection, DOT export, and metrics now reinforce one causal debugging story |
| End-to-end causality integration | Next | Signal explanation should be able to connect cleanly back through bridge-carried truth provenance |
| Graph inspection tools | Implemented | Direct graph export and dependency-chain inspection exist; hot-path analysis can deepen later |
| Dependency inspection | Implemented | Direct APIs now answer “who depends on what” explicitly |
| Execution metrics | Implemented | Telemetry is now surfaced intentionally through graph/runtime metrics snapshots |

#### Scheduling and execution

| Capability | Status | Notes |
| --- | --- | --- |
| Deterministic execution mode | Implemented | Current runtime behavior is deterministic by design |
| Core API ergonomics | Implemented | Full-power API now uses builders, explicit transactions, and accessible naming |
| Builder-based runtime ergonomics | Implemented | Runtime builder, transaction helpers, and node builders are now first-class |
| Explicit execution planner | Next | Needed before reusable staged execution and more advanced scheduling |
| Parallel evaluation | Later | Depends on planner/stage model and executor separation |
| Cost-aware scheduling | Later | Requires per-node cost metadata and planner integration |
| Priority propagation | Later | Requires explicit scheduling model and prioritization semantics |

#### State / replay / evolution

| Capability | Status | Notes |
| --- | --- | --- |
| First-class signal graph snapshots | Next | Data is serializable, but snapshot/restore is not yet a first-class API |
| Replay-oriented evaluation state capture | Next | Needs explicit runtime snapshot surface and metadata framing |
| Branchable evaluation paths | Later | Depends on snapshot/branch semantics rather than current in-place flow |
| Signal lineage | Next | Track how computed artifacts evolve across evaluations, cache refresh, replacement, snapshot restore, and branch switches |

#### Easy API / developer experience

| Capability | Status | Notes |
| --- | --- | --- |
| Easy-mode signal API | Implemented | `forge_signal::easy::*` now exists as a separate surface over the same runtime |
| Angular-style computed ergonomics | Implemented | Input/computed/get/set/batch are now available without changing the core contract |
| Automatic dependency capture in easy API | Implemented | Easy-mode computed closures now discover dependencies automatically |
| Effects/watchers | Next | Ergonomic layer should expose subscription/effect patterns |
| Batch ergonomics | Implemented | Easy API now provides explicit batching over the same runtime semantics |

#### Bridge / dual-runtime integration

| Capability | Status | Notes |
| --- | --- | --- |
| Dual-graph architecture | Next | Architectural direction is clear, but dedicated bridge surface is not yet formalized |
| Patch-to-invalidation bridge | Next | Relational diffs should drive signal invalidation directly |
| Aspect mapping layer | Next | Needed to map relational aspects onto signal aspects cleanly |
| Snapshot evaluation | Next | Signals should evaluate against immutable relational snapshots |
| Bulk change propagation | Next | Large patchsets should invalidate efficiently without per-change overhead |
| Change stream protocol | Later | Generic protocol should exist before tighter integration scales up |
| Reactive source protocol | Later | Generic read contract for signal consumers without fusion |
| Relational-key to signal-node mapping | Later | Needed to keep truth IDs and signal IDs decoupled |
| Field / lens subscriptions | Later | Fine-grained subscriptions belong in the bridge, not core graph ownership |

### Forge Relational Context

`forge-relational` is not the main subject of this document, but the signal vision depends on the truth-side runtime being explicit.

| Capability family | Role in the stack | Status framing |
| --- | --- | --- |
| Identity / storage | Truth-side handles, arena layout, structural identity hooks | Core truth-runtime concern |
| Transactions / savepoints | Authoritative mutation boundary | Core truth-runtime concern |
| History / MVCC / replay | Stable snapshots and deterministic history | Required for long-term bridge maturity |
| Diffs / CDC / patch feeds | Drives signal invalidation inputs | Direct bridge dependency |
| Traversal / introspection | Read-side access for host snapshots | Required integration surface |

## Roadmap

This roadmap is product-oriented. The existing foundation execution plan remains the detailed crate-level implementation plan for the current base runtime, and the dedicated Phase 1 plan captures the concrete productization pass. This document defines what the product becomes after that foundation exists.

### Phase 1: Productize the current runtime (Completed)

**Outcome:** `forge-signal` is both powerful and approachable for direct use.

Major additions:

- hard-mode API cleanup for transactions, builders, and common execution flows
- accessible naming cleanup for core APIs, builders, and examples
- `forge-signal-easy` ergonomic API
- runtime builder for `SignalRuntime`
- transaction closure helpers
- node builder ergonomics
- better crate-level documentation of core vs easy usage

Phase 1 established the public runtime surface. The current runtime is now legible and humane enough to build on without carrying forward the old public API clutter.

See [_docs/engineering/forge_signal_phase1_plan.md](/Users/spenstar/Documents/programming/forge%20workspace/Forge/_docs/engineering/forge_signal_phase1_plan.md) for the concrete execution plan.

### Phase 2: Observability and dependency inspection (Completed)

**Outcome:** users can see why work ran, what depends on what, and where recomputation cost is going.

Major additions:

- first-class execution provenance surface through `explain(node)`
- dependency inspection APIs
- graph inspection and debug export tools
- richer surfaced metrics on top of existing telemetry
- bridge-ready causality metadata hooks

Phase 2 is the baseline provenance phase for the runtime. It should be treated as core product infrastructure, not optional debugging sugar. The goal is to make current-state causality explicit: why a node is dirty, maybe-stale, deferred, recomputed, or unchanged in the moment.

### Phase 3: Smarter propagation and diff-aware execution (Completed)

**Outcome:** the runtime suppresses more unnecessary work and becomes more semantically aware of unchanged outputs.

Major additions:

- query-style keyed computation surfaces
- output identity / result diffing
- downstream suppression when outputs are unchanged
- partition-aware output reporting and partition-scoped subscriptions
- first structural memoization layer with explicit host keys

Phase 3 established the smarter propagation substrate while preserving future snapshot and signal-lineage semantics. Output diffing, partition-aware subscriptions, and memoization now exist in forms that keep artifact continuity describable later.

### Phase 4: Execution planning and parallelism

**Outcome:** evaluation is planned explicitly, then dispatched efficiently.

Major additions:

- reusable execution planner
- staged execution model
- executor abstraction
- deterministic staged scheduling
- optional parallel execution after planning exists

Parallel execution is intentionally not earlier than this phase. Planner/stage semantics come first.

### Phase 5: Snapshots, replay, and branchable evaluation

**Outcome:** signal state becomes explicitly captureable, inspectable, and replay-friendly.

Major additions:

- first-class snapshot/restore API
- evaluation-state persistence model
- signal lineage foundations so computed artifacts can be tracked across refresh, replacement, restore, memoized reuse, and branch switches
- replay-oriented inspection tooling
- branchable evaluation-path foundations

See [_docs/engineering/forge_signal_state_lineage_design.md](/Users/spenstar/Documents/programming/forge%20workspace/Forge/_docs/engineering/forge_signal_state_lineage_design.md) for the concept lock on snapshots, replay, provenance, and signal lineage.

Phase 5 is the historical-provenance phase. It should extend provenance from current-state explanation into replayable, restorable, and time-aware understanding of how evaluation state and computed artifacts evolved.

### Phase 6: Dual-runtime bridge completion

**Outcome:** relational truth and signal computation compose cleanly through stable protocols.

Major additions:

- patch-to-invalidation bridge
- aspect mapping layer
- snapshot-backed evaluation surface
- bulk diff propagation
- integration contracts for node-key mapping and source reads

Phase 6 is the end-to-end provenance phase. It should allow truth-side commit and patch causality to flow cleanly through the bridge into signal-side explanation and diagnostics without collapsing the runtimes into one system.

### Phase 7: Advanced semantics

**Outcome:** the runtime supports more demanding generic execution patterns without becoming domain-specific.

Major additions:

- fixed-point / convergence nodes
- speculative evaluation / branching
- mature structural memoization
- cost-aware and priority-aware scheduling policy surfaces

## Principles

1. `forge-signal` is never truth.
2. Easy syntax is a product requirement, not a runtime simplification.
3. Hard-mode syntax quality is a product requirement, not a cosmetic afterthought.
4. Determinism is a product feature, not just a test harness property.
5. Transactions and rollback are mandatory semantics, not optional safety sugar.
6. Bridge protocols must preserve decoupling between truth and compute.
7. Structural and domain semantics remain outside the runtime unless exposed as generic hooks.
8. `forge-signal` must stand on its own as a reusable library with standalone APIs, not only as one half of the Forge stack.
9. Accessible naming is a product feature; prefer intention-revealing names over insider shorthand.
10. Diagnostics are first-class runtime architecture; explanation, inspection, provenance, and metrics must ship as core capabilities.
11. Signal lineage is a real runtime concern distinct from host truth lineage and should be modeled explicitly when replay and branching mature.

## Non-goals

- Embedding geometry- or topology-specific semantics directly into `forge-signal`
- Collapsing relational truth storage and signal execution into one fused runtime
- Replacing explicit core APIs with only an ergonomic wrapper
- Treating the easy API as permission to weaken transactional or deterministic guarantees
- Shipping parallel execution before an explicit planner/stage model exists

## Public Surface Vocabulary

These names are conceptual API categories, not necessarily immediate crate splits:

- `forge-signal-core`: low-level runtime surface
- `forge-signal-easy`: ergonomic signal surface
- bridge / integration layer: relational-to-signal coordination surface

`forge-signal-core` should be optimized for expert readability, not only raw power.
Both `forge-signal-core` and `forge-signal-easy` should be directly usable without going through the bridge layer.

The current concrete vocabulary remains the anchor for the runtime contract:

- `SignalGraph`
- `SignalRuntime`
- transactions
- aspects
- evaluation conditions
- comparator policies

## Current-State Notes

- The current foundation is already strong enough to justify this vision: DAG scheduling, aspects, conditional evaluation, comparator policies, deterministic behavior, telemetry, and transactional rewind are real.
- The current observability baseline is now real too: structured explanations, dependency inspection, DOT export, surfaced metrics, richer trace summaries, and generic causality hooks are part of the runtime.
- The current foundation execution plan should be treated as the implementation hardening path for the base runtime, not as the final product vision.
- The next major leap is not inventing a new core model. It is pushing beyond baseline observability into smarter propagation, planning, snapshots, lineage, and bridge-grade causality.
- The state/replay/lineage concepts are now locked separately in [_docs/engineering/forge_signal_state_lineage_design.md](/Users/spenstar/Documents/programming/forge%20workspace/Forge/_docs/engineering/forge_signal_state_lineage_design.md) so later phases do not drift into ad hoc semantics.
