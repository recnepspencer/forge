# Forge Relational Vision

## Thesis

Forge needs a first-class truth runtime, not an ad hoc storage layer.

`forge-relational` is a standalone truth-state runtime library: the system that owns identity, mutation, history, diffs, lineage, and traversal over a host-managed graph or relational state model. This document is intentionally library-first. It defines what `forge-relational` must be in its own right, not how any one host application chooses to embed it.

For Forge, the specification graph is the product. More generally, the runtime exists for systems where authoritative graph state must outlive any single derived read model, evaluation layer, export format, or application session.

`forge-relational` should be designed as a standalone generic library. CAD and geometry kernels are major targets, but not the definition of the runtime. The same truth/runtime architecture should be useful in chip design, financial platforms, AI systems, and other domains that need transactional graph state with strong identity and history semantics.

Those domains are not forgiving. Geometry kernels, chip-design systems, and other critical engineering runtimes impose a high-assurance bar: determinism, diagnosability, replayability, and long-term architectural discipline are product requirements, not optional engineering quality work.

## Why This Runtime Is Different

These are not side features. They are the architectural bets that make the truth runtime strategically different:

- generational IDs and multi-layer identity
- sparse transactional mutation
- nested savepoints
- MVCC snapshots
- snapshot reads during active mutation
- branchable version graph
- deterministic replay
- patch streams / CDC
- aspect-tagged diffs
- relational aspect system
- lineage-aware identity evolution
- branch-aware correspondence hooks
- bulk relational queries

If these are treated as “nice to have later,” the runtime collapses back into ordinary graph storage and the larger Forge architecture loses its leverage.

## Mission

`forge-relational` exists to make truth-state graph operations safe, replayable, branchable, and inspectable at industrial scale.

It must be developed to the standard expected of high-consequence runtime infrastructure. This is not a place for MVP shortcuts, convenient ambiguity, or hidden behavior that "probably works" until scale, audit, or certification pressure arrives.

It must answer these questions as native runtime responsibilities:

- What is the authoritative identity of this entity?
- How do writes happen transactionally and rewind cheaply?
- How do multiple readers observe stable graph state while mutations continue elsewhere?
- How does committed truth emit structured diffs rather than opaque mutation fallout?
- How does an entity survive splits, merges, replacements, and branch divergence?
- How do downstream systems query the graph efficiently in bulk rather than one edge at a time?

This runtime is the substrate that makes mergeable history, AI-native editing, deterministic replay, and signal-driven derived computation possible across many domains.

## Architectural Model

### Runtime stack

| Layer | Responsibility | Owns |
| --- | --- | --- |
| `forge-relational` | Truth-state graph runtime | identity, mutation, history, diffs, lineage, traversal, integrity |
| `forge-signal` | Derived-computation runtime | invalidation, recomputation, conditions, scheduling, runtime self-inspection |
| Bridge / integration | Decoupled coordination | patch-to-invalidation, aspect mapping, snapshot evaluation, key mapping |

## Ownership Boundaries

### What `forge-relational` owns

- Stable identity and safe handle reuse
- Transactional mutation boundaries
- Snapshot/history/version graph behavior
- Structured diffs and change streams
- Lineage and correspondence foundations
- Query and traversal primitives
- Secondary index and derived-index hooks
- Integrity and schema enforcement hooks

### What `forge-relational` does not own

- Reactive invalidation scheduling
- Derived-computation policies or planners
- Domain-specific numeric solvers
- Projection execution policy
- Permanent fusion with signal execution
- Kernel feature execution or host-specific projection logic
- Mandatory use of the bridge for standalone relational use

### Structural rule

Truth mutation, truth history, and truth identity live here. Downstream runtimes may observe, project, or react to truth, but they do not redefine it.

## Principles

1. Truth is graph-native, not blob-native.
2. Identity must survive mutation, branching, and structural change.
3. Writes happen only through transactions.
4. Undo, branching, and replay are foundational, not add-ons.
5. Diffs are first-class outputs of commit, not side effects reconstructed later.
6. Lineage is a graph, not loose metadata.
7. Bulk traversal and bulk mutation matter as much as single-entity convenience APIs.
8. Deterministic ordering is a product feature, not a debugging aid.
9. The runtime remains generic; domain semantics arrive through schema, kinds, and host hooks.
10. The runtime must stand on its own as a reusable library, not as a kernel-specific storage helper.
11. `forge-relational` must expose standalone APIs for truth-state usage without requiring `forge-signal` or the bridge.
12. Integrity and schema validation are first-class runtime architecture, not accessory checks around the edge.
13. Diagnostics are a first-class runtime contract. Truth mutation, history, lineage, diffs, and replay must be inspectable, comparable, and auditable in production.
14. The relational harness is first-class infrastructure. Regression seeders, branch/history scenarios, and diff/replay parity drivers should evolve alongside the runtime, not after it.
15. Parallelism must be designed around immutable reads and immutable outputs. Preparation and downstream consumption may scale out, but authoritative truth mutation, authoritative order, and authoritative history must remain deterministic.

## Foundational Decisions We Refuse To Revisit Later

These are locked architectural decisions, not provisional defaults.

- single logical writer for authoritative truth commit
- generational identity as the only authoritative ID base
- separate entity and relation identity systems
- immutable committed snapshots
- canonical ordering for every observable output
- commit-native patch and CDC emission
- structured diagnostics as a production contract
- arena plus sidecar storage as the foundational memory model
- `forge_harness` as the default acceptance path
- parallelize preparation and consumption, serialize authority
- derived indexes remain non-authoritative and publish immutably
- no hidden mutation during reads
- no scheduler-dependent semantics

For this runtime, a slice is truth-grade only if it guarantees:

- all authoritative mutation flows through transactions only
- all observable outputs are deterministic and canonically ordered
- committed reads are immutable
- commit emits structured patch and diagnostics artifacts
- replay artifacts are derived from canonical commit artifacts
- harness parity exists for success and failure paths

### Explicit prohibitions

These are architectural prohibitions, not style guidance:

- no `HashMap` iteration in observable paths
- no read-triggered normalization, lazy repair, or cache mutation
- no shared mutable authoritative patch object across workers
- no entity-only runtime that defers relations to later
- no temporary pre-generational ID model
- no diagnostics-only freeform blobs without stable structured fields
- no replay artifact that is a dump of internal heap state
- no public API bias toward per-record ping-pong reads

### Observable order classes

Observable means:

- snapshots
- diagnostics artifacts
- patch artifacts
- replay artifacts
- public iteration surfaces
- harness comparison outputs

Internal worker scheduling may vary. Observable outputs may not.

Canonical ordering must be defined for:

- entity order
- relation order
- worker-intent merge order
- canonical merged-operation order
- authoritative apply order
- patch emission order
- diagnostics entry order
- replay record order

### Snapshot publication semantics

Snapshot publication is a single coherent user-visible boundary:

- a successful commit produces exactly one committed visible snapshot
- failed commits publish nothing authoritative
- snapshot, patch, diagnostics, and replay artifacts are published as one coherent commit outcome
- if coherent publication cannot complete, the commit does not become visible
- publication may have internal phases, but from the user-visible contract it is atomic

### Replay artifact contract

Replay artifacts must obey all of the following:

- derived from canonical commit artifacts rather than internal heap state
- stable and serializable inputs
- canonical replay ordering
- local timing and worker scheduling excluded from replay semantics
- schema-versioned from day one
- schema/version mismatch is an explicit failure class

### Invariant categories

Every invariant must declare both category and effect on execution:

- `AlwaysOnStructural`
- `CommitBoundary`
- `SnapshotAudit`
- `HarnessHeavy`

Each invariant must state whether failure blocks commit, blocks publication, or is audit-only.

### Diagnostics boundedness

Diagnostics are required by default, but they are not unbounded:

- every commit emits a mandatory minimal structured summary
- detailed traces are optional by profile
- retention is bounded by policy
- diagnostics storage must not grow unbounded on hot paths
- commit-time diagnostics cannot depend on unlimited buffering

### Kind and schema registry discipline

Kinds and schema identity are architectural contracts:

- kinds use stable IDs
- kind registration is schema-governed
- kind identity is replay-stable
- kind mapping is portable across snapshots and branches
- schema/version mismatch is explicit and never silently tolerated

## Pillars

### Identity Architecture

#### Generational IDs

Why it exists:
Handles must remain safe under deletion and slot reuse without leaking stale references.

Problem it solves:
Naive dense IDs create ABA-style hazards and make mutation-heavy graph systems brittle.

Boundary it imposes:
All persistent references must pass through typed, generation-aware identity rather than raw slot indices.

What depends on it:
Transactions, snapshots, replay, change feeds, and safe external handles.

#### Structural identity hooks

Why it exists:
Storage identity alone is not enough when the system needs to recognize “the same structure” across rebuilds or branches.

Problem it solves:
Without structural hooks, deduplication, correspondence, memoization, and merge reasoning become ad hoc.

Boundary it imposes:
The runtime must expose a place for host-provided structural signatures without hardcoding semantic meaning.

What depends on it:
Correspondence, lineage refinement, merge tooling, and future structural reuse.

#### Multi-layer identity model

Why it exists:
Forge must distinguish storage identity, lineage identity, structural identity, and optional semantic identity.

Problem it solves:
A single ID model cannot simultaneously handle slot lifetime, historical continuity, and cross-branch equivalence.

Boundary it imposes:
The runtime cannot collapse identity into one handle type plus comments.

What depends on it:
Lineage, correspondence, replay, merge, and downstream bridge mapping.

### Transaction Architecture

#### Transactional mutation

Why it exists:
Truth must never be mutated through scattered side effects.

Problem it solves:
Non-transactional writes make history, rollback, and diff emission unreliable.

Boundary it imposes:
All write paths flow through explicit transaction boundaries.

What depends on it:
Snapshots, CDC, rollback, bridge integration, and deterministic replay.

Authority rule:
Parallel workers may prepare candidate intents, validation summaries, index fragments, and diagnostics fragments, but final truth mutation must flow through one deterministic commit authority.

#### Sparse undo log

Why it exists:
Rollback must scale with touched state, not whole-graph size.

Problem it solves:
Full graph copying is too expensive for large mutable graphs and speculative workflows.

Boundary it imposes:
Mutation tracking must record touched subsets precisely.

What depends on it:
Cheap rollback, savepoints, speculative editing, and large-model throughput.

Diagnostics expectation:
Undo records must be inspectable as production truth artifacts. Rollback diagnostics need stable ordering, failure classification, and replay-comparable summaries.

#### Nested savepoints

Why it exists:
Complex operations need partial rollback without discarding an entire outer transaction.

Problem it solves:
Without savepoints, speculation inside a larger operation becomes too coarse and too expensive.

Boundary it imposes:
The transaction model must support scoped rewind points, not just all-or-nothing rollback.

What depends on it:
Compound mutation operators, AI-driven exploration, and future branch experimentation.

Concurrency boundary:
Savepoint creation, rollback-to-savepoint, commit, and abort are transaction-boundary state transitions. They must remain serialized and explicit even if some planning or validation work around them runs in parallel.

#### Bulk mutation APIs

Why it exists:
Large graph edits must be expressible as one structured operation, not a loop of tiny writes.

Problem it solves:
Per-entity mutation overhead destroys throughput and obscures intent.

Boundary it imposes:
The runtime must expose vectorized mutation surfaces as first-class APIs.

What depends on it:
Importers, graph transforms, migration tools, and large operator batches.

Parallel design rule:
Bulk mutation APIs should accept partitionable intent batches and worker-local staging outputs now, even if initial execution stays single-writer.

Formal planning layers:

- `WorkerIntentBatch`
- `MergedCommitPlan`
- `AuthoritativeApplyPlan`
- `CommitOutcome`

Staging objects must not quietly acquire authoritative semantics.

### History Architecture

#### MVCC snapshots

Why it exists:
Readers need stable views while mutation continues.

Problem it solves:
Single mutable state forces stop-the-world coordination or unsafe read semantics.

Boundary it imposes:
Committed state and mutable state must be representable as distinct versions.

What depends on it:
Snapshot-backed evaluation, replay, branch exploration, and parallel reads.

Parallel design rule:
Snapshots must be immutable, concurrently readable, and semantics-stable. Reads must not trigger lazy writeback, hidden normalization, or cache mutation that changes observable results.

#### Snapshot reads during active mutation

Why it exists:
Long-running reads and derived computation cannot block all writes.

Problem it solves:
Without stable snapshot reads, integration with signal evaluation becomes fragile and non-deterministic.

Boundary it imposes:
The runtime must let readers pin stable versions while writers continue elsewhere.

What depends on it:
Bridge contracts, signal evaluation, inspection tools, and audit surfaces.

Testing rule:
`forge_harness` parity suites must treat snapshot reads during active mutation as a first-class acceptance path, not a stress-test add-on.

#### Branchable version graph

Why it exists:
Forge models must fork and merge like code, not just walk a linear undo chain.

Problem it solves:
Linear history cannot represent concurrent design evolution.

Boundary it imposes:
Version history must be graph-shaped, not stack-shaped.

What depends on it:
Merge, counterfactuals, collaboration, and AI branch exploration.

Authority boundary:
Version graph advancement is authoritative history publication. Candidate work may be prepared in parallel, but parent selection, version identity assignment, and visibility publication must be canonical and serialized.

#### Deterministic replay

Why it exists:
A truth runtime for engineering must reproduce history exactly.

Problem it solves:
Without replay, debugging, certification, and regression minimization all become weaker.

Boundary it imposes:
Commit ordering, mutation records, and observable outputs must be deterministic.

What depends on it:
Audit, debugging, merge diagnostics, and validation harnesses.

Harness rule:
Replay acceptance for `forge-relational` must run through `forge_harness` (`forge-harness` package). The runtime should not rely on bespoke replay test scaffolding that drifts from the production diagnostics contract.

#### Graph time travel

Why it exists:
Users and systems need to inspect historical graph states directly.

Problem it solves:
Replay alone is not enough when inspection must target prior committed states on demand.

Boundary it imposes:
The history model must support reconstruction or retention of prior states intentionally.

What depends on it:
Debugging, visualization, branch comparison, and audit tooling.

Lifecycle vocabulary:

- `Live`
- `DeletedRetained`
- `PinnedBySnapshot`
- `PinnedByBranch`
- `PinnedByReplayRetention`
- `Reclaimable`
- `Reusable`

The architecture must use explicit lifecycle terminology rather than treating "tombstone" as a catch-all concept.

#### Version garbage collection

Why it exists:
History retention must remain sustainable as the graph evolves.

Problem it solves:
Snapshots and branches accumulate until they become a storage and memory liability.

Boundary it imposes:
Version retention and reclamation cannot be an afterthought.

What depends on it:
Long-lived projects, collaboration, and industrial-scale persistence.

### Diff Architecture

#### CDC / patch streams

Why it exists:
Every commit should emit structured change data as a native runtime output.

Problem it solves:
Reconstructing diffs after the fact is slower, less precise, and harder to integrate.

Boundary it imposes:
Commit must produce machine-readable patchsets, not just mutate state silently.

What depends on it:
Bridge integration, audit, streaming tooling, and downstream invalidation.

Authority boundary:
Diff preparation may be parallelized over immutable snapshots and worker-local fragments. Final emitted authoritative patch order must remain deterministic and canonical.

#### Stream correctness semantics

Why it exists:
Patch feeds become operational infrastructure the moment other systems consume them continuously.

Problem it solves:
Without explicit ordering, resume, checkpoint, idempotence, and coalescing semantics, change streams become fragile and hard to trust in financial, AI, distributed, or large interactive systems.

Boundary it imposes:
The runtime must define deterministic stream ordering and consumption semantics as part of the architecture, not leave them implicit.

What depends on it:
Bridge replay, durable subscribers, large-scale integrations, and robust downstream recovery.

Diagnostics expectation:
Resume tokens, checkpoints, publication order, coalescing decisions, and subscriber-visible failures all need structured diagnostics and replay-visible records.

#### Aspect-tagged diffs

Why it exists:
Truth changes must say what changed, not merely that something changed.

Problem it solves:
Coarse patch streams create over-invalidation and poor runtime self-inspection.

Boundary it imposes:
Diffs must carry typed aspect metadata.

What depends on it:
Signal invalidation routing, incremental tools, and targeted analysis.

#### Relational aspect system

Why it exists:
Nodes and relations need generic aspect masks so the runtime can speak in precise change semantics.

Problem it solves:
Without a relational aspect system, bridge-layer aspect mapping becomes lossy and ad hoc.

Boundary it imposes:
Aspect support must exist at the truth layer, not only in signals.

What depends on it:
Patch-to-invalidation mapping, diff precision, and field/lens subscriptions.

#### Relation aspect tagging

Why it exists:
Relations themselves often change in meaningful ways distinct from node payload changes.

Problem it solves:
If only nodes carry aspects, edge-level changes become hard to describe precisely.

Boundary it imposes:
Aspect tagging must apply to both entities and relations.

What depends on it:
Topology/state diffs, bridge routing, and precise downstream invalidation.

#### Streaming patch feeds

Why it exists:
Other systems need live access to truth changes without polling snapshots.

Problem it solves:
Batch-only diff extraction limits responsiveness and decoupled integrations.

Boundary it imposes:
Patch emission must be protocol-friendly, not only internal bookkeeping.

What depends on it:
Bridge subscriptions, external tooling, and collaborative workflows.

#### Lineage-aware diffs

Why it exists:
Diffs must capture identity evolution, not only adds/removes.

Problem it solves:
Plain patch feeds cannot express replacement, split, and merge semantics cleanly.

Boundary it imposes:
Diffs must be able to reference lineage and correspondence data.

What depends on it:
Merge tooling, branch reasoning, and downstream identity-sensitive systems.

### Lineage Architecture

#### Lineage events

Why it exists:
Replace, split, and merge are first-class structural events, not awkward combinations of delete/add.

Problem it solves:
Without explicit lineage events, identity evolution becomes ambiguous.

Boundary it imposes:
The runtime must model transformation history directly.

What depends on it:
Historical resolution, correspondence, and lineage-aware diffs.

Authority boundary:
Workers may discover correspondence or lineage candidates in parallel, but final lineage event recording must be canonical and serialized.

#### Historical ID resolution

Why it exists:
Consumers need to ask what an older entity became.

Problem it solves:
Versioned systems become difficult to query if old identifiers simply disappear.

Boundary it imposes:
Identity lookup must support forward resolution across history.

What depends on it:
Audit, merge, replay, and bridge consumers that track evolving entities.

#### Lineage graph

Why it exists:
Lineage must be queryable as its own graph.

Problem it solves:
Loose metadata cannot support serious reasoning about identity evolution.

Boundary it imposes:
Lineage relationships require first-class storage and traversal.

What depends on it:
Historical resolution, merge, branch comparison, and AI-assisted correspondence.

#### Branch-aware correspondence hooks

Why it exists:
Different branches need a way to express “these are probably the same thing” even when identity diverged.

Problem it solves:
Lineage alone is insufficient for cross-branch matching once history forks independently.

Boundary it imposes:
The runtime must allow branch-aware matching hooks without baking in domain scoring rules.

What depends on it:
Merge, conflict resolution, and structural comparison across revisions.

#### Correspondence policy hooks

Why it exists:
Hosts need to plug in matching policy when lineage is insufficient.

Problem it solves:
A generic runtime cannot hardcode semantic equivalence policy for every domain.

Boundary it imposes:
The runtime provides hooks and structure, not semantic scoring logic.

What depends on it:
Advanced merge, cross-branch matching, and future semantic reconciliation.

### Query and Scale Architecture

#### Single-entity traversal primitives

Why it exists:
The graph still needs sharp primitives like `targets_of` and `sources_of`.

Problem it solves:
Higher-level systems need reliable building blocks for precise traversal.

Boundary it imposes:
The query layer must expose clear low-level operations, not only bulk APIs.

What depends on it:
Inspection tools, mutation operators, and adapter layers.

#### Bulk relational queries

Why it exists:
This is one of the runtime’s defining strengths. Large graph workloads need vectorized queries, not per-entity loops disguised as APIs.

Problem it solves:
Single-entity traversal alone does not scale to industrial graph analysis or broad invalidation planning.

Boundary it imposes:
Bulk query surfaces must be treated as primary design targets, not convenience wrappers.

What depends on it:
Bridge propagation, analysis, imports, large traversal workloads, and system-scale performance.

Parallel design rule:
Bulk APIs should be expressible as deterministic work packets over stable snapshots so future partition-aware execution does not require redesigning the query surface.

#### Relation-type scans

Why it exists:
Systems frequently need to scan all edges of a kind efficiently.

Problem it solves:
Type-filtered relation access is too expensive if it requires full graph walks.

Boundary it imposes:
Storage and indexing must support efficient relation-kind access paths.

What depends on it:
Validation, analytics, bridge mapping, and schema-level tooling.

#### Graph introspection APIs

Why it exists:
A truth runtime must be inspectable beyond raw storage access.

Problem it solves:
Counts, structure summaries, references, and recent mutations should not require custom debug code every time.

Boundary it imposes:
Introspection belongs in the runtime surface, not only in external tooling.

What depends on it:
Debugging, runtime self-inspection, and admin/inspection interfaces.

#### Secondary index and derived-index hooks

Why it exists:
Arena layout and direct traversal are not enough for every large-scale query workload.

Problem it solves:
Bulk relational queries often need host-tunable index surfaces rather than forcing every system to rebuild indexing ad hoc outside the runtime.

Boundary it imposes:
The runtime should expose hooks for maintained indexes and derived lookup structures without hardcoding every domain-specific index strategy.

What depends on it:
High-volume queries, relation-type scans, bridge lookups, and large host-specific query acceleration.

Parallel design rule:
Derived index maintenance should separate authoritative commit from parallel rebuild assistance. Parallel workers may compute index fragments or new immutable generations, but publication of read-visible index state must remain ordered and explicit.

Storage classification rule:

Hot-path sidecars contain only data required for:

- slot validity and lifecycle state
- canonical iteration participation
- commit apply
- snapshot reads
- adjacency traversal
- aspect/version gating required by core read/commit paths

Cold-path sidecars contain data required for:

- lineage refinement
- extended diagnostics
- replay enrichment
- correspondence hints
- branch metadata not needed by core apply/read
- audit-only or harness-heavy metadata

Any per-record metadata placed in hot-path sidecars requires explicit justification.

#### Parallel read access

Why it exists:
Many systems need to inspect stable graph state concurrently.

Problem it solves:
Read serialization becomes a bottleneck once models and downstream consumers grow.

Boundary it imposes:
Snapshot and storage design must support concurrent immutable reads.

What depends on it:
Analysis, signal evaluation, tooling, and distributed workflows.

Determinism rule:
Query results, diagnostics summaries, and introspection output must not depend on thread count, worker timing, or scheduler order.

#### Partitioning hints

Why it exists:
Large models eventually need guidance for splitting work across threads or systems.

Problem it solves:
Blind partitioning can destroy locality and correctness assumptions.

Boundary it imposes:
The runtime should expose hints without requiring distributed execution to exist on day one.

What depends on it:
Future scale-out, bulk query planning, and cross-worker decomposition.

Future-proofing rule:
Even before full partitioning exists, APIs should preserve the ability to address entity ranges, relation ranges, branch-local work packets, and snapshot-local partitions.

#### Memory stability guarantees

Why it exists:
Very large graph systems fail when allocation behavior is unpredictable.

Problem it solves:
Fragmentation and unstable layouts erode performance and make scaling fragile.

Boundary it imposes:
Storage design must prioritize predictable allocation and locality.

What depends on it:
Long-lived graph workloads, bulk queries, and industrial-scale model sizes.

## Roadmap

The ordering here is architectural, not cosmetic. The early phases lock in the properties that are expensive to retrofit later.

Diagnostics and the harness are cross-cutting requirements, not a final cleanup phase. Every phase above should ship with:

- deterministic summaries and diffs for the new subsystem
- failure-path diagnostics rather than success-only reporting
- scenario-driven `forge_harness` coverage with named regression seeders for confirmed bugs
- bounded retained history suitable for long-running truth runtimes
- serial-vs-parallel parity checks wherever a phase introduces parallel-capable preparation or read paths
- coherent publication semantics for snapshot, patch, diagnostics, and replay artifacts
- explicit invariant categories and declared failure effects

### Phase 1: Identity and storage foundations

Breakthrough features:

- generational IDs
- arena layout and storage discipline
- structural identity hooks
- multi-layer identity model

Outcome:
Truth has stable handles, predictable storage behavior, and a clean identity story that can support branching and lineage later.

Must also lock in:

- separate entity and relation identity systems
- lifecycle-state vocabulary instead of generic tombstone flags
- hot-path versus cold-path sidecar discipline
- schema-governed stable kind IDs

### Phase 2: Transaction foundations

Breakthrough features:

- transactional mutation
- sparse undo log
- nested savepoints
- bulk mutation APIs

Outcome:
Mutation becomes explicitly scoped, rewindable, and scalable for large graph edits.

Must also lock in:

- thread-local intent staging rather than shared mutable patch construction
- deterministic intent merge rules before any attempt at concurrent apply
- rollback and savepoint diagnostics that survive replay and branch comparison
- formal separation between `WorkerIntentBatch`, `MergedCommitPlan`, `AuthoritativeApplyPlan`, and `CommitOutcome`

### Phase 3: History and branching foundations

Breakthrough features:

- MVCC snapshots
- snapshot reads during active mutation
- branchable version graph
- deterministic replay

Outcome:
Truth becomes branchable and inspectable without sacrificing safe mutation flow.

Must also lock in:

- immutable snapshot handles safe for concurrent reads
- serialized authoritative publication of commits and version graph advancement
- `forge_harness` parity coverage for snapshot reads during active mutation and replay fidelity
- coherent publication of snapshot, patch, diagnostics, and replay artifacts

### Phase 4: Diff and aspect foundations

Breakthrough features:

- CDC / patch streams
- stream correctness semantics
- aspect-tagged diffs
- relational aspect system
- relation aspect tagging

Outcome:
Commits emit precise structured change data suitable for bridge routing and audit.

Must also lock in:

- worker-local diff fragment preparation with deterministic final merge
- canonical stream ordering and durable resume/checkpoint semantics
- diagnostics surfaces for patch publication, subscriber recovery, and audit export

### Phase 5: Lineage and correspondence foundations

Breakthrough features:

- lineage events
- historical ID resolution
- lineage graph
- branch-aware correspondence hooks

Outcome:
Identity evolution becomes queryable and usable across history and branches.

Must also lock in:

- deterministic lineage finalization rules
- parallel candidate discovery without parallel authoritative lineage mutation
- harness seeders for split/merge/replace/correspondence regressions

### Phase 6: Query and scale foundations

Breakthrough features:

- bulk relational queries
- secondary index and derived-index hooks
- relation-type scans
- parallel read access
- memory stability guarantees

Outcome:
The truth runtime becomes powerful enough to feed projections, analyses, and bridge consumers at scale.

Must also lock in:

- partition-aware job descriptions even before distributed execution exists
- deterministic map-reduce style aggregation for validation, diagnostics, index fragments, and metrics
- explicit separation between serial truth commit and parallel post-commit derived work

### Runtime trust infrastructure

This work is intentionally cross-phase:

- production diagnostics contract for truth mutation, history, lineage, replay, and CDC
- one public diagnostics entrypoint instead of scattered debug utilities
- lifecycle-aware truth artifacts that can later compose with bridge and signal diagnostics
- a relational adapter for `forge_harness`, built on production diagnostics, with branch/history/diff/replay seeders and regression scenarios
- acceptance suites that compare serial-authority execution against any staged-parallel preparation or post-commit parallel mode
- invariant categories with declared execution effects
- schema-versioned replay artifacts derived from canonical commit artifacts

If this work is postponed, the truth runtime will be much harder to trust once branching, replay, and correspondence become real.

## Non-goals

- Turning the truth runtime into a reactive scheduler
- Fusing truth storage and signal evaluation into one system
- Treating bulk queries, MVCC, lineage, or CDC as optional polish
- Baking domain-specific semantic meaning into the generic runtime core
- Reducing identity to a single storage handle model

## Public Vocabulary

These are conceptual surface areas, not immediate crate split requirements:

- truth runtime
- identity model
- transaction boundary
- snapshot/version graph
- patch/change feed
- stream correctness semantics
- relational aspect system
- lineage graph
- correspondence hooks
- traversal/query layer

Companion documents:

- [_docs/engineering/forge_signal_vision.md](/Users/spenstar/Documents/programming/forge%20workspace/Forge/_docs/engineering/forge_signal_vision.md) for derived computation
- [_docs/engineering/forge_runtime_bridge_vision.md](/Users/spenstar/Documents/programming/forge%20workspace/Forge/_docs/engineering/forge_runtime_bridge_vision.md) for dual-runtime integration
- [_docs/engineering/forge_relational_roadmap.md](/Users/spenstar/Documents/programming/forge%20workspace/Forge/_docs/engineering/forge_relational_roadmap.md) for the implementation roadmap and future-proofing constraints

The truth runtime’s MVCC, CDC, lineage, aspects, and bulk query architecture are what make the rest of the stack viable. If these are weak, every projection, bridge, and reactive layer built on top of them becomes weaker too. `forge-relational` should therefore be designed as an independent runtime library with its own direct API surface, not as a kernel-tied implementation detail or as something that must be accessed through the bridge.
