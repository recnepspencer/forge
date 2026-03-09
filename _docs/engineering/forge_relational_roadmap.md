# Forge Relational Roadmap

## Purpose

This document turns the relational vision into an implementation roadmap.

It exists to prevent an MVP-style buildout that ships isolated features while quietly blocking the properties that are expensive to retrofit later:

- deterministic history
- first-class diagnostics
- `forge_harness`-driven parity and replay validation
- lineage and branch semantics
- diff/CDC correctness
- parallel read and planning surfaces
- future partition-aware execution

The target domains matter. This runtime is intended for geometry kernels, chip-design systems, and other demanding applications where truth corruption, replay drift, weak diagnostics, or nondeterministic behavior are unacceptable. The development bar therefore needs to match high-assurance infrastructure, not "good enough for v1" product code.

The operating rule is:

parallelize disposable work, serialize authority.

In practice that means:

1. parallel read, analyze, validate, and prepare against immutable snapshots
2. single-writer deterministic commit of authoritative truth
3. parallel downstream consumption over immutable commit artifacts

## Foundational Decisions We Refuse To Revisit Later

These are locked decisions for the runtime foundation:

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

Canonical ordering must be explicitly defined for:

- entity order
- relation order
- worker-intent merge order
- canonical merged-operation order
- authoritative apply order
- patch emission order
- diagnostics entry order
- replay record order

Internal worker scheduling may vary. Observable outputs may not.

### Snapshot publication semantics

- a successful commit produces exactly one committed visible snapshot
- failed commits publish nothing authoritative
- snapshot, patch, diagnostics, and replay artifacts are published as one coherent commit outcome
- if coherent publication cannot complete, the commit does not become visible
- publication is atomic from the user-visible contract perspective

### Replay artifact contract

- derived from canonical commit artifacts rather than internal heap state
- stable and serializable replay inputs
- canonical replay ordering
- local timing and worker scheduling excluded from replay semantics
- schema-versioned from day one
- schema/version mismatch is an explicit failure class

### Sidecar classification

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

Per-record metadata added to hot-path sidecars requires explicit justification.

### Lifecycle vocabulary

The runtime uses explicit lifecycle terms rather than a generic tombstone concept:

- `Live`
- `DeletedRetained`
- `PinnedBySnapshot`
- `PinnedByBranch`
- `PinnedByReplayRetention`
- `Reclaimable`
- `Reusable`

### Invariant categories

Every invariant must declare category and effect:

- `AlwaysOnStructural`
- `CommitBoundary`
- `SnapshotAudit`
- `HarnessHeavy`

Each invariant must declare whether failure blocks commit, blocks publication, or is audit-only.

### Diagnostics boundedness

- every commit emits a mandatory minimal structured summary
- detailed traces are optional by profile
- retention is bounded by policy
- diagnostics storage must not grow unbounded on hot paths
- commit-time diagnostics cannot depend on unlimited buffering

### Kind and schema registry discipline

- kinds use stable IDs
- kind registration is schema-governed
- kind identity is replay-stable
- kind mapping is portable across snapshots and branches
- schema/version mismatch is explicit and never silently tolerated

## Non-Negotiable Architectural Invariants

### Authority stays serialized

These are authoritative and must remain deterministic:

- final truth commit
- authoritative mutation order
- savepoint and rollback boundary transitions
- version graph advancement and visibility publication
- slot/generation reuse authority
- final lineage event recording
- final canonical patch/CDC emission order

### Parallelism happens around authority, not through it

These should be designed for parallel execution early:

- immutable snapshot reads
- query planning
- validation over immutable state
- diff fragment preparation
- secondary-index fragment preparation
- import staging
- post-commit bridge and downstream consumption
- retention and GC analysis

### Diagnostics are production infrastructure

Every major subsystem must emit structured, versioned, replay-comparable diagnostics for:

- success paths
- failure paths
- rollback paths
- branch/history transitions
- CDC publication
- validation findings
- retention and recovery decisions

The runtime must have one public diagnostics entrypoint instead of scattered debug helpers.

### `forge_harness` is required infrastructure

The `forge_harness` crate (`forge-harness` package) is the acceptance path for relational trust work.

Required harness roles:

- scenario fixture builders
- branch/history seeders
- replay and parity drivers
- diagnostics inspectors
- exportable comparison artifacts
- performance/profile sweeps where behavior needs proof

## Roadmap

### Phase 0: Contract and vocabulary lock-in

Goal:
Freeze the architectural language before implementation spreads.

Deliverables:

- public vocabulary for identity, version, commit, snapshot, lineage event, patch stream, diagnostics artifact, and correspondence
- explicit authority boundaries for commit, lineage, CDC, and version publication
- relational diagnostics contract shape and artifact families
- relational adapter plan for `forge_harness`
- canonical ordering classes and observable-surface definitions
- kind/schema registry policy
- lifecycle vocabulary and sidecar classification rules

### Phase 1: Identity and storage foundations

Goal:
Build storage and identity in a way that preserves future history and concurrency guarantees.

Deliverables:

- generational IDs
- typed handles and stale-handle safety
- separate entity and relation identity systems
- storage layout with predictable allocation behavior
- lifecycle-state model instead of generic tombstone flags
- hot-path and cold-path sidecar structure
- canonical live-record iterators
- adjacency storage for relation traversal
- structural identity hooks
- multi-layer identity vocabulary
- schema-governed stable kind IDs
- structural invariant subsystem

`forge_harness` expectations:

- identity regression seeders
- stale-handle and slot-reuse parity cases
- exported diagnostics for identity faults

### Phase 2: Transaction and commit foundations

Goal:
Establish transaction semantics around a single deterministic commit authority.

Deliverables:

- transaction boundary model
- sparse undo log
- nested savepoints
- bulk mutation APIs
- `WorkerIntentBatch`
- deterministic merge model for staged write intents
- `MergedCommitPlan`
- `AuthoritativeApplyPlan`
- `CommitOutcome`
- coherent publication of snapshot, patch, diagnostics, and replay artifacts
- explicit failure taxonomy

`forge_harness` expectations:

- commit/rollback/savepoint scenarios
- injected failure cases
- replay parity for transactional diagnostics

### Phase 3: Snapshot, history, and replay foundations

Goal:
Make committed truth safely inspectable while mutation continues elsewhere.

Deliverables:

- immutable MVCC snapshot handles
- snapshot reads during active mutation
- version graph foundations
- deterministic replay artifact substrate
- history retention metadata
- pinned snapshot registry
- publication atomicity contract

`forge_harness` expectations:

- concurrent snapshot-read scenarios
- serial-authority replay parity suites
- branch/history export artifacts

### Phase 4: Diagnostics-first runtime surfaces

Goal:
Make truth behavior inspectable before the runtime becomes too large to reason about cleanly.

Deliverables:

- public diagnostics facade
- structured diagnostics artifacts for transactions, history, replay, lineage, and CDC
- deterministic diagnostics reduction rules
- production-safe artifact export story
- mandatory minimal commit summaries with bounded retention
- detailed trace profiles as optional overlays
- invariant-check entrypoints by invariant category

`forge_harness` expectations:

- diagnostics parity inspectors
- durable JSON export coverage
- named regression seeders for confirmed bugs

### Phase 5: Diff, CDC, and aspect foundations

Goal:
Make commit outputs precise, durable, and downstream-safe.

Deliverables:

- patch/CDC model
- aspect-tagged entity and relation diffs
- canonical patch ordering
- resume/checkpoint semantics
- subscriber-facing recovery model
- replay artifacts derived from canonical commit artifacts
- schema-versioned replay payloads

`forge_harness` expectations:

- diff parity suites
- replay-to-CDC parity suites
- subscriber recovery and resume tests

### Phase 6: Lineage and correspondence foundations

Goal:
Represent identity evolution explicitly instead of faking it through adds/removes.

Deliverables:

- lineage event types
- historical resolution APIs
- lineage graph storage
- branch-aware correspondence hooks
- deterministic lineage finalization policy

`forge_harness` expectations:

- split/merge/replace seeders
- cross-branch correspondence scenarios
- lineage diagnostics comparisons

### Phase 7: Query, indexes, and scale-ready surfaces

Goal:
Make the runtime usable for large truth workloads without painting the read side into a corner.

Deliverables:

- bulk relational queries
- relation-type scans
- secondary-index hooks
- derived-index generation model
- introspection APIs
- partition/work-packet vocabulary
- bulk packetized reads as the primary public read surface
- thin per-ID convenience only over packetized reads

`forge_harness` expectations:

- bulk-query parity cases
- index rebuild comparison suites
- diagnostics coverage for index publication and query planning

### Phase 8: Parallel preparation and post-commit scaling

Goal:
Add scalable parallel work without violating the trust model.

Deliverables:

- parallel planning over immutable snapshots
- parallel validation workers with deterministic reduction
- parallel diff/index fragment preparation
- parallel import staging
- parallel post-commit downstream consumption over immutable artifacts

`forge_harness` expectations:

- serial-vs-staged-parallel parity suites
- diagnostics parity suites
- replay parity suites
- performance acceptance suites proving the mode is useful

### Phase 9: Retention, GC, compaction, and long-life hardening

Goal:
Keep the runtime sustainable for long-lived projects.

Deliverables:

- retention policy framework
- snapshot and branch liveness analysis
- reclaim candidate discovery
- compaction planning artifacts
- audit-grade retention diagnostics

`forge_harness` expectations:

- retention and replay compatibility suites
- long-history regression scenarios
- compaction/reclamation diagnostics comparisons

## Weak Spots To Watch Now

1. Shared mutable commit structures.
2. Hidden mutation during reads.
3. Diagnostics bolted on per subsystem.
4. Diff/CDC treated as post-hoc reconstruction.
5. Lineage delayed until after history ships.
6. Query APIs designed as single-entity convenience wrappers.
7. Secondary indexes coupled to live mutable state.
8. Parallelism introduced without deterministic merge rules.
9. Harness coverage deferred until after the core "works."
10. Hot-path sidecars accumulating cold-path metadata.
11. Lifecycle-state semantics collapsed into a generic tombstone concept.
12. Snapshot publication treated as a best-effort multi-step side effect.

## Completion Standard

Forge Relational is only on a trustworthy path if:

- diagnostics remain first-class in every phase
- `forge_harness` is the default acceptance path for parity, replay, and regression work
- serial authority boundaries remain explicit
- all parallel-capable work is shaped around immutable inputs and deterministic merges
- roadmap decisions are evaluated on rewrite cost later, not just speed of initial implementation
- the implementation standard remains appropriate for geometry kernels, chip-design systems, and other high-consequence domains rather than ordinary MVP application work
- observable ordering is specified operationally, not only aspirationally
- snapshot, patch, diagnostics, and replay publication behave as one coherent commit outcome
