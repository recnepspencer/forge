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

## Why This Is Harder Than `forge-signal`

`forge-signal` already requires deterministic runtime thinking, but its core problem is still narrower: present-state reactive execution over a constrained DAG.

`forge-relational` adds simultaneous pressure from:

- MVCC and historical visibility
- branch-aware history
- cyclic truth graphs
- lineage and identity evolution
- durable patch and replay contracts

That makes this runtime closer to a truth/history kernel than to a conventional Rust library.

The main danger is not borrow-checker pain by itself. The main danger is building semantically weaker shortcuts that look practical in the short term:

- replay reduced to patch reapplication
- lineage reduced to event logging
- derived indexes drifting into authority
- durability coupled to transient in-memory layout
- history assumptions collapsing around single-parent commits

The roadmap must therefore preserve correctness-first authority semantics before pursuing clever concurrency or memory-management machinery.

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
- replay is executed from canonical commit envelopes, not patch-only reconstruction
- history representation is merge-ready now: ordered parent commit lists, even before merge commits execute
- authoritative storage-visible reads always retain a non-index fallback path
- lineage is a constrained graph with explicit invariants, not event logging theater
- durability persists canonical truth artifacts rather than transient arena layout
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
- executed from canonical commit envelopes that include branch context, ordered parents, schema identity, merged apply semantics, patch artifact, and diagnostics summary
- stable and serializable replay inputs
- canonical replay ordering
- local timing and worker scheduling excluded from replay semantics
- schema-versioned from day one
- schema/version mismatch is an explicit failure class
- replay equivalence is defined over the observable surfaces promised by the active profile

### History shape

- commit references use ordered parent lists
- initial authoritative commit creation may still be restricted to zero or one parent
- replay, durability, lineage, and branch reasoning must remain compatible with future merge commits

### Derived index contract

- indexes are built from canonical truth outputs and version-visible storage
- index computation may parallelize; publication remains serialized and version-bound
- index absence, lag, mismatch, or failure must never change truth semantics
- authoritative reads must always retain a storage-visible fallback path

### Lineage contract

- storage identity and lineage identity remain permanently separate
- correspondence remains advisory until explicit promotion
- final lineage mutation is serialized and canonical
- lineage graph invariants must reject invalid references, ambiguous parentage, and silent advisory-to-authoritative promotion

### Durability contract

- durable format preserves canonical truth artifacts rather than transient arena layout
- recovery rebuilds authoritative truth from canonical envelopes and committed history
- snapshots are recoverable views rather than primary durable truth
- partial durable publication is invalid

### What We Will Not Do Prematurely

- no premature lock-free multi-writer truth mutation
- no `Arc`-everywhere persistent-state model as the default architecture
- no custom epoch reclamation or equivalent advanced memory machinery until profiling proves the simpler retention model insufficient
- no durability format that mirrors transient arena layout
- no optimization that weakens replay, coherent publication, or authoritative storage fallback

### Primary Early Performance Risks

The main early performance risks are:

- retained historical payload growth under pinned snapshots
- hot-record version-history growth
- chunk sizing mistakes that destroy scan locality
- pointer chasing and allocation overhead from abandoning SoA discipline
- replay and derived-index artifact growth beyond bounded policy

The first performance program should therefore focus on:

- sidecar and chunk discipline
- touched-state proportionality
- retention and reclaim efficiency
- storage-native historical visibility
- bounded diagnostics, patch, and replay artifacts by profile

### Semantic Risks Bigger Than Rust Risks

These are explicit runtime risks that deserve design, review, and test attention:

- replay defined too narrowly
- lineage reduced to event logging
- derived indexes treated as authority
- diagnostics that are verbose but operationally useless
- merge-ready history quietly collapsing back to single-parent assumptions
- retention, recovery, or replay semantics depending on incidental scheduler order

### Correctness-First MVCC Strategy

The intended MVCC path is:

- single-writer authoritative commit
- version-visible retained storage
- explicit snapshot pinning and release
- explicit retention and reclaim transitions
- chunk-aware analysis and retention planning
- storage-native historical reads before advanced reclamation machinery

If more advanced memory-management machinery is eventually required, it must be introduced without weakening deterministic truth semantics or coherent publication.

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
