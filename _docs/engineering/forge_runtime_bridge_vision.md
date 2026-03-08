# Forge Runtime Bridge Vision

## Thesis

Forge does not want one fused runtime that mixes truth storage and derived computation into a single opaque system.

It wants two strong runtimes with a principled bridge between them:

- a truth runtime that owns identity, mutation, history, and diffs
- a computation runtime that owns invalidation, recomputation, scheduling, and observability
- a bridge layer that turns truth changes into deterministic reactive execution without collapsing the two together

The bridge is not glue code. It is the architectural boundary that keeps the system decoupled while still allowing precise, large-scale propagation from truth to computation.

The bridge exists because the two runtimes are separate libraries. Its job is to bring them together cleanly for hosts that use both, while preserving their separate ownership boundaries and independent runtime identities.

## Why This Bridge Is Different

These are the architectural bets that make the bridge first-class:

- dual-graph architecture
- patch-to-invalidation bridge
- aspect mapping layer
- snapshot evaluation
- bulk change propagation
- change stream protocol
- reactive source protocol
- relational-key to signal-node mapping
- field / lens subscriptions

The innovation is not “signals can read relational data.” The innovation is a protocol boundary that lets truth and computation remain separate systems while still composing precisely, efficiently, and causally.

## Mission

The bridge exists to answer these questions natively:

- How do committed truth changes become signal invalidations without hard-coding one runtime into the other?
- How do signal computations read stable truth snapshots without taking ownership of truth storage?
- How do relational aspects map onto signal aspects without lossy translation?
- How do large diff batches propagate efficiently?
- How do truth IDs stay decoupled from signal node IDs while still supporting stable linkage?
- How do fine-grained field/facet dependencies remain precise at the bridge boundary?

If the bridge is weak, truth and computation either fuse into one brittle system or devolve into manual invalidation code. The bridge therefore needs to be designed as a standalone integration architecture between libraries, not as internal glue inside one host application.

## Architectural Model

### Runtime stack

| Layer | Responsibility | Owns |
| --- | --- | --- |
| Truth runtime | authoritative graph state | identity, mutation, history, diffs, lineage |
| Derived runtime | reactive execution | invalidation, recomputation, conditions, scheduling |
| Bridge / integration | coordination boundary | protocols, mapping, routing, snapshot-backed reads |

### Dual-graph rule

Forge keeps two graphs on purpose:

- truth graph: the authoritative structural/versioned model
- computation graph: the dependency DAG for derived work

The bridge coordinates them. It does not erase the distinction.

## Ownership Boundaries

### What the bridge owns

- Change feed contracts
- Patch-to-invalidation routing
- Aspect mapping semantics
- Snapshot-backed read contracts
- Truth-key to signal-node mapping
- Fine-grained subscription shapes
- Bulk propagation policy surfaces

### What the bridge does not own

- Truth mutation or truth storage
- Signal scheduling internals
- Domain-specific semantic scoring
- Numeric compute logic
- Permanent ID fusion across runtimes
- Kernel-specific adaptation logic as the definition of the bridge

### Structural rule

The bridge translates and coordinates. It does not become a second truth runtime or a second scheduler.

## Principles

1. Truth and computation remain separate systems.
2. Change propagation starts from authoritative patch feeds, not ad hoc callbacks.
3. Aspects must remain explicit at the bridge boundary.
4. Snapshot-backed evaluation is mandatory for determinism.
5. Truth IDs and signal node IDs must remain decoupled.
6. Bulk propagation is a first-class scale problem, not an optimization after the fact.
7. Fine-grained subscriptions belong in the bridge contract, not by leaking truth internals into signal.
8. Protocol boundaries matter as much as runtime internals.
9. The bridge must be library-grade architecture, not a special-case Forge kernel adapter.
10. The bridge integrates the runtimes without collapsing their separate ownership boundaries.
11. End-to-end causality must survive the boundary from truth commit through invalidation and recomputation.

## Pillars

### Dual-runtime Foundation

#### Dual-graph architecture

Why it is first-class:
It preserves the distinction between authoritative state and derived computation.

What coupling problem it prevents:
Without dual-graph architecture, the system drifts toward a fused runtime where truth changes and reactive execution become inseparable.

Boundary ownership:
Truth runtime owns structural state. Signal runtime owns dependency scheduling. The bridge coordinates between them.

Assumptions:
Truth and compute have different lifecycles, different indexing needs, and different failure modes.

#### Explicit truth/compute separation

Why it is first-class:
Each runtime can optimize for its real job rather than carrying the other runtime’s constraints.

What coupling problem it prevents:
Fused designs make truth storage harder to branch and make compute scheduling harder to reason about.

Boundary ownership:
The bridge enforces separation by making protocol surfaces explicit instead of implicit shared state.

Assumptions:
Projection and derived computation are disposable; truth is not.

### Change Propagation

#### Patch-to-invalidation bridge

Why it is first-class:
Truth commits should drive recomputation automatically through structured patchsets.

What coupling problem it prevents:
Manual invalidation logic becomes fragmented, lossy, and impossible to audit at scale.

Boundary ownership:
Truth emits structured patch data; bridge translates it into signal invalidations; signal executes the resulting recompute work.

Assumptions:
The truth runtime emits precise diffs rather than opaque mutation events.

#### Bulk change propagation

Why it is first-class:
Large truth diffs must propagate efficiently into the computation runtime.

What coupling problem it prevents:
Per-change invalidation routing creates overhead cliffs and hides the real scale model.

Boundary ownership:
The bridge owns routing strategies for large patchsets, but not signal executor internals.

Assumptions:
Industrial workloads will routinely touch many entities and aspects at once.

#### Deterministic invalidation routing

Why it is first-class:
The same diff must drive the same invalidation set and ordering semantics every time.

What coupling problem it prevents:
Nondeterministic routing undermines replay, debugging, and trust in downstream results.

Boundary ownership:
Bridge contract defines the mapping and routing semantics; signal runtime preserves deterministic execution semantics after invalidation arrives.

Assumptions:
Truth diffs and aspect mappings are stable and reproducible.

#### End-to-end causality propagation

Why it is first-class:
The bridge should preserve causal identity across truth commit, patch emission, invalidation routing, and derived recomputation.

What coupling problem it prevents:
Without explicit causality propagation, provenance fragments at the runtime boundary and explanations become local rather than end-to-end.

Boundary ownership:
Truth runtime owns commit provenance; bridge carries and maps causal references; signal runtime consumes them for explanation and trace surfaces.

Assumptions:
Lineage/provenance matter operationally, not only as debugging metadata.

### Semantic Mapping

#### Aspect mapping layer

Why it is first-class:
Relational aspects and signal aspects are distinct spaces that must be mapped intentionally.

What coupling problem it prevents:
Without a dedicated mapping layer, aspect translation becomes scattered and inconsistent.

Boundary ownership:
Truth runtime owns relational aspects; signal runtime owns signal aspects; bridge owns the mapping contract.

Assumptions:
Both runtimes support aspect-level precision and those aspect systems are not identical by default.

#### Relational-key to signal-node mapping

Why it is first-class:
Truth IDs and signal IDs should not collapse into one namespace.

What coupling problem it prevents:
Hard-coupled IDs make each runtime hostage to the storage/indexing choices of the other.

Boundary ownership:
Bridge owns mapping and lookup. Truth and signal each keep their own identity models.

Assumptions:
One truth entity may map to multiple signal nodes and one signal node may depend on multiple truth keys.

#### Field / lens subscriptions

Why it is first-class:
Signals often depend on a specific field or facet, not the whole truth object.

What coupling problem it prevents:
Whole-entity subscriptions create coarse invalidation and throw away the value of aspect-precise truth diffs.

Boundary ownership:
Bridge exposes fine-grained subscription shapes; truth provides the field/facet view; signal consumes mapped invalidation.

Assumptions:
Precision at the boundary matters for both correctness and scale.

### Read Contracts

#### Snapshot evaluation

Why it is first-class:
Signal computation must read stable truth-state while writes continue elsewhere.

What coupling problem it prevents:
Direct live reads from mutable truth state make computation nondeterministic and fragile.

Boundary ownership:
Truth runtime provides snapshots; bridge supplies snapshot-backed read surfaces; signal evaluates against them.

Assumptions:
Truth runtime supports stable snapshots and snapshot reads during concurrent mutation activity.

#### Reactive source protocol

Why it is first-class:
Signals need a generic way to consume truth-backed data without embedding storage internals.

What coupling problem it prevents:
If signal code must know relational storage details directly, the layering collapses.

Boundary ownership:
Bridge owns the read contract; truth runtime implements it; signal runtime consumes it.

Assumptions:
The protocol describes reads and subscriptions, not fused execution ownership.

### Protocol Surfaces

#### Change stream protocol

Why it is first-class:
Patch feeds need a stable contract if multiple downstream systems are going to consume them.

What coupling problem it prevents:
Ad hoc internal event shapes hardwire consumers to implementation details.

Boundary ownership:
Truth runtime produces the stream; bridge defines the consumption contract; downstream systems subscribe through it.

Assumptions:
The change stream needs to be general enough for more than one consumer, not custom-built only for signal.

#### Stream correctness semantics

Why it is first-class:
Continuous integration between runtimes needs more than just an event shape; it needs correct delivery semantics.

What coupling problem it prevents:
Without explicit ordering, cursor/checkpoint, replay/resume, idempotence, and coalescing semantics, the bridge becomes hard to reason about under failure and scale.

Boundary ownership:
Truth runtime provides deterministic stream material; bridge defines how consumers resume, checkpoint, and interpret the stream; signal runtime consumes it consistently.

Assumptions:
Large systems will need durable streaming behavior rather than fire-and-forget callbacks.

#### Stable bridge contracts for host integrations

Why it is first-class:
The kernel and future integrations need a stable coordination surface.

What coupling problem it prevents:
Without explicit contracts, every integration path invents its own mapping and propagation logic.

Boundary ownership:
Bridge defines the contract shapes; host systems adapt to them explicitly.

Assumptions:
Bridge protocols should survive internal refactors of truth and compute runtimes.

## Roadmap

This roadmap sequences the bridge features as named milestones so they cannot quietly disappear into vague “integration later” work.

### Phase 1: Dual-graph contract

Breakthrough features:

- truth graph vs compute graph separation
- ownership boundaries

Outcome:
The architecture locks in explicit separation between truth storage and derived computation before deeper integration work expands.

### Phase 2: Change feed contract

Breakthrough features:

- change stream protocol
- stream correctness semantics
- patch-to-invalidation bridge

Outcome:
Truth commits become structured inputs to reactive invalidation rather than bespoke kernel callbacks.

### Phase 3: Aspect integration

Breakthrough features:

- aspect mapping layer
- invalidation routing semantics

Outcome:
Truth-side aspects propagate into signal invalidation precisely and predictably.

### Phase 4: Snapshot-backed execution and causality

Breakthrough features:

- snapshot evaluation
- reactive source protocol
- end-to-end causality propagation

Outcome:
Signals evaluate against stable truth snapshots while preserving causal traceability from truth commit to derived explanation.

### Phase 5: Identity decoupling

Breakthrough features:

- relational-key to signal-node mapping

Outcome:
Truth identity and signal identity remain independent while still supporting durable linkage.

### Phase 6: Fine-grained subscriptions

Breakthrough features:

- field / lens subscriptions

Outcome:
Subscriptions become precise enough to preserve the value of relational aspects and fine-grained diffs.

### Phase 7: Scale path

Breakthrough features:

- bulk change propagation
- protocol maturity for large diffs

Outcome:
The bridge can route large change sets without collapsing into per-event overhead or integration-specific hacks.

## Non-goals

- Turning the bridge into a fused truth+compute runtime
- Requiring signal code to understand truth storage internals directly
- Collapsing truth IDs and signal IDs into one handle space
- Treating patch-to-invalidation or snapshot evaluation as optional later polish
- Making the bridge a grab bag of bespoke kernel glue

## Public Vocabulary

These are conceptual surface areas, not immediate crate split requirements:

- dual-graph architecture
- patch-to-invalidation bridge
- aspect mapping layer
- snapshot evaluation contract
- change stream protocol
- stream correctness semantics
- end-to-end causality propagation
- reactive source protocol
- relational-key to signal-node mapping
- field / lens subscriptions
- bulk change propagation

Companion documents:

- [_docs/engineering/forge_relational_vision.md](/Users/spenstar/Documents/programming/forge%20workspace/Forge/_docs/engineering/forge_relational_vision.md) for truth-state architecture
- [_docs/engineering/forge_signal_vision.md](/Users/spenstar/Documents/programming/forge%20workspace/Forge/_docs/engineering/forge_signal_vision.md) for derived computation

Dual-graph architecture, patch-to-invalidation routing, snapshot evaluation, and ID decoupling are what make integration clean rather than fragile. If those are weak, the system either fuses into a monolith or falls back to manual invalidation and leaky abstractions. The bridge should therefore be designed as a strong coordination layer between generic libraries that need to work together.
