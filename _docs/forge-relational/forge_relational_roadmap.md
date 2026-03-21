# Forge Relational Future Roadmap

## Purpose

This document replaces the earlier buildout roadmap.

It tracks only the work that still remains for `forge-relational`.
Foundational architecture and Phase 8 are now shipped baseline. This roadmap
exists to define the remaining product milestones and the acceptance path that
must be satisfied before the runtime can be considered complete against
[test-requirements.md](/Users/spenstar/Documents/programming/forge%20workspace/Forge/_docs/forge-relational/test-requirements.md).

The operating rule remains:

parallelize disposable work, serialize authority.

That rule still governs every remaining milestone:

1. immutable read, planning, validation, and preparation may scale out
2. authoritative truth mutation and publication remain serialized and canonical
3. downstream consumption may scale out only over immutable published artifacts

## Shipped Baseline

The roadmap no longer tracks the already-shipped foundation as future work.

The current shipped baseline includes:

- identity and storage foundations
- transactional commit authority
- savepoint and rollback substrate
- MVCC snapshots and version/history substrate
- deterministic patch, diagnostics, and replay publication
- lineage, query, and index foundations
- proof-driven parallel preparation and post-commit scaling
- `forge-harness` parity and certification substrate

The shipped closeout reference for the latest major runtime milestone is
[phase-8.md](/Users/spenstar/Documents/programming/forge%20workspace/Forge/_docs/forge-relational/phase-8.md).

## Roadmap Rules

The remaining roadmap is organized around product-complete milestones first and
certification programs second.

Rules for every remaining item:

- each milestone must describe the missing runtime/product capability, not only a test label
- each milestone must preserve serialized authority, canonical observability, replay from canonical commit artifacts, and storage-visible fallback semantics
- each milestone must name the exact acceptance requirements in
  [test-requirements.md](/Users/spenstar/Documents/programming/forge%20workspace/Forge/_docs/forge-relational/test-requirements.md)
- the roadmap uses `test-requirements.md` as the authoritative source for what each named acceptance test demands
- no milestone is complete until both implementation and acceptance requirements are closed

## Milestone 1: CDC and Subscriber Recovery

Status: Closed on 2026-03-13. See
[milestone-1-closeout.md](/Users/spenstar/Documents/programming/forge%20workspace/Forge/_docs/forge-relational/milestone-1-closeout.md).

### Goal

Finish the stream product surface so CDC is not just emitted patch data, but a
durable downstream consumption contract.

### Must Ship

- subscriber-facing recovery model
- explicit resume/checkpoint semantics
- deterministic resumed publication order
- explicit subscriber-visible failure classes
- replay-to-CDC consistency as a runtime contract
- protocol-facing streaming patch feeds rather than internal-only patch emission
- diagnostics for resume tokens, checkpoints, recovery decisions, and stream rejection/failure

### Must Preserve

- commit-native canonical patch order
- coherent publication
- replay from canonical commit artifacts
- no abandoned savepoint or rollback work appearing in published CDC
- no scheduler-shaped stream semantics

### Explicit Boundary

Milestone 1 includes schema/version compatibility checking and explicit
subscriber-visible failure for incompatible checkpoints, recovery plans, or
runtime/schema combinations.

Milestone 1 does not include live schema-version transition inside an already
running subscriber contract. In other words, this milestone must fail
structurally on incompatible schema boundaries; it does not yet promise
mid-stream schema renegotiation, dual-schema CDC emission, or seamless
subscriber continuation across runtime schema changes.

### Acceptance Requirements

This milestone is complete only when the implementation satisfies the following
named requirements from
[test-requirements.md](/Users/spenstar/Documents/programming/forge%20workspace/Forge/_docs/forge-relational/test-requirements.md):

- `Diff/CDC truth parity test`
- `Hostile commit/replay equivalence test`
- `Durable recovery and schema mismatch test`
- `Snapshot-stable concurrent read vs hot rewrite test`

The roadmap intentionally references the requirements doc directly here. The
requirements doc remains the source of truth for the exact scenario,
verification output, and pass condition those tests impose.

## Milestone 2: Relational Aspect Semantics

Status: Closed on 2026-03-19. See
[milestone-2-closeout.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-relational/milestone-2-closeout.md).

### Goal

Finish the truth-layer aspect system so aspects are first-class relational
semantics for entities, relations, committed diffs, projections, historical
reads, and bulk queries, rather than only payload-derived change labels.

### Must Ship

- stable aspect identity at the truth layer
- explicit entity aspect semantics
- explicit relation aspect semantics
- aspect participation in committed diff and CDC output
- aspect-aware projection and bulk-query surfaces
- aspect-aware historical read surfaces
- aspect-aware and lineage-aware historical read surfaces where identity evolution and change surface both matter
- schema and kind driven aspect declarations where the runtime needs more than raw payload-key extraction
- diagnostics and artifacts that expose aspect-level committed change semantics canonically

### Must Preserve

- canonical patch ordering
- coherent publication
- deterministic replay from canonical commit artifacts
- no hidden mutation during reads
- authoritative storage-visible fallback semantics
- no scheduler-shaped observability

### Acceptance Requirements

This milestone is complete only when the implementation satisfies the following
named requirements from
[test-requirements.md](/Users/spenstar/Documents/programming/forge%20workspace/Forge/_docs/forge-relational/test-requirements.md):

- `Diff/CDC truth parity test`
- `Bulk query and traversal stress truth test`
- `Hostile commit/replay equivalence test`
- `Topology identity survival test`
- `Netlist rewiring identity and history test`

## Milestone 3: Structural Identity, Introspection, and Historical Inspection

### Goal

Finish the runtime surfaces that let consumers inspect structure, recent
mutation, and retained history as first-class truth capabilities rather than
ad hoc debug utilities.

### Must Ship

- explicit structural identity surfaces
- structural hashing or fingerprint surfaces that hosts can use without collapsing structural policy into storage identity
- graph introspection APIs over entities, relations, kinds, counts, connectivity classes, and change boundaries
- recent-mutation and transaction-surface introspection
- graph time-travel surfaces for retained or reconstructible historical truth
- explicit retention and reclaim product surfaces rather than only internal retention machinery
- historical inspection surfaces that compose structural identity, lineage, and aspects coherently

### Must Preserve

- permanent separation between storage identity, lineage identity, and structural identity
- canonical observability for historical reads
- explicit reclaim behavior under pinned readers and retained history
- no hidden mutation during inspection or time travel
- replay from canonical commit artifacts

### Acceptance Requirements

This milestone is complete only when the implementation satisfies the following
named requirements from
[test-requirements.md](/Users/spenstar/Documents/programming/forge%20workspace/Forge/_docs/forge-relational/test-requirements.md):

- `Hostile commit/replay equivalence test`
- `Snapshot pinning and reclaim correctness test`
- `Lineage/correspondence hardening test`
- `Bulk query and traversal stress truth test`
- `Topology identity survival test`
- `Netlist rewiring identity and history test`

## Milestone 4: Relation Integrity and Schema Contracts

### Goal

Finish the runtime contract layer for relation legality, schema-defined
invariants, and typed relation ergonomics so the runtime is enforcing graph
truth rules instead of merely storing graph-shaped data.

### Must Ship

- relation integrity as an explicit runtime capability
- schema-defined invariants for relation legality, cardinality, symmetry, uniqueness, and deletion effects where the schema declares them
- typed node and relation contract richness sufficient for industrial schema design
- integrity validation hooks that participate in authoritative commit
- failure diagnostics for relation-contract and invariant violations
- schema surfaces that remain generic while still being strong enough for topology, connectivity, workflow, and IR-style truth graphs

### Must Preserve

- serialized authority
- canonical failure reporting
- no partial publication after integrity failure
- schema-defined semantics instead of hardcoded domain semantics
- replay and recovery consistency for accepted and rejected commits

### Acceptance Requirements

This milestone is complete only when the implementation satisfies the following
named requirements from
[test-requirements.md](/Users/spenstar/Documents/programming/forge%20workspace/Forge/_docs/forge-relational/test-requirements.md):

- `Savepoint rollback fracture test`
- `Hostile commit/replay equivalence test`
- `Durable recovery and schema mismatch test`
- `Missing-twin / nonmanifold corruption localization test`

## Milestone 5: Schema Evolution, CDC Contract Evolution, and Schema Reconciliation

### Goal

Finish live schema evolution as an explicit runtime and CDC capability so
schema changes can participate in authoritative truth, replay, recovery, and
subscriber contracts without collapsing into ad hoc host coordination.

Additionally, finish classified schema reconciliation so that schema
divergence across branches, tenants, or evolution boundaries is handled by
explicit policy rather than blanket rejection or silent corruption.

### Must Ship

- explicit schema evolution model for authoritative runtimes
- canonical publication of schema transition boundaries
- subscriber-facing schema transition semantics for checkpoint/resume and
  active stream continuation
- typed transition policies such as fail, require renegotiation, or continue
  under an explicitly compatible schema bridge where supported
- recovery and replay semantics that preserve schema transition boundaries as
  canonical artifacts
- diagnostics for schema transition decisions, rejection, and subscriber
  contract renegotiation
- compatibility rules that distinguish:
  - incompatible schema mismatch
  - resumable compatible evolution
  - transitions that require a new subscriber contract

#### Schema Reconciliation

Schema reconciliation provides classified handling of schema divergence during
merge, deployment, and tenant migration. The runtime must classify every schema
difference into a reconciliation category and apply the appropriate policy
rather than treating all incompatibilities as fatal.

Required reconciliation categories:

- **additive reconciliation**: new aspects, fields, entity types, or relation
  types introduced on one side but absent on the other must be
  auto-reconcilable with explicit default or null semantics; this covers the
  common case of one branch or tenant evolving the schema while another has not
  yet adopted the change
- **narrowing reconciliation**: aspects or fields removed on one side but still
  present on the other must be handled by caller-supplied policy (prefer
  richer, prefer target, or reject); the runtime must not silently drop or
  silently preserve without an explicit policy decision
- **type-incompatible rejection**: same-named aspect or field with incompatible
  type on each side must be classified as a genuine conflict and fail-explicit
  with structured diagnostics; the runtime must not attempt implicit coercion
- **structural-incompatible rejection**: fundamentally different relation
  topology or entity modeling between sides must be classified as structural
  conflict and fail-explicit with structured diagnostics

Required reconciliation behaviors:

- schema reconciliation must participate in the merge commit pipeline when
  branches carry divergent declared schemas
- reconciliation decisions must be emitted as canonical artifacts for replay,
  diagnostics, and audit
- reconciliation must compose with Milestone 7 merge execution so that
  data-level merge and schema-level reconciliation are resolved in the same
  transactional commit
- reconciliation must support caller-supplied policies rather than hardcoded
  resolution strategies
- reconciliation must never silently drop schema structure, silently coerce
  types, or silently adopt incompatible topology

#### Motivation

Schema reconciliation is especially critical for web development with custom
workflows, per-tenant customization, plugin/extension systems, and any domain
where schema divergence is the norm rather than an exceptional event. Without
classified reconciliation, fail-explicit rejection on schema mismatch creates
unacceptable friction for deployments, tenant migrations, and feature rollouts
in these domains.

### Must Preserve

- serialized authority for schema-affecting truth mutation
- canonical CDC and patch ordering across schema boundaries
- replay from canonical commit artifacts
- no hidden host-side schema repair during resume or recovery
- explicit failure instead of silent drift when transition semantics are not
  supported
- reconciliation decisions as canonical artifacts, not silent internal behavior

### Acceptance Requirements

This milestone is complete only when the implementation satisfies the
following named requirements from
[test-requirements.md](/Users/spenstar/Documents/programming/forge%20workspace/Forge/_docs/forge-relational/test-requirements.md),
and any additional schema-evolution-specific certification requirements added
there:

- `Diff/CDC truth parity test`
- `Hostile commit/replay equivalence test`
- `Durable recovery and schema mismatch test`

Additionally, this milestone must add and satisfy an explicit schema-evolution
CDC certification requirement if
[test-requirements.md](/Users/spenstar/Documents/programming/forge%20workspace/Forge/_docs/forge-relational/test-requirements.md)
does not yet contain one.

Additionally, this milestone must add and satisfy an explicit schema
reconciliation certification requirement covering additive, narrowing,
type-incompatible, and structural-incompatible classification with
policy-driven resolution.

## Milestone 6: Lineage and Correspondence Completion

### Goal

Finish identity-evolution semantics so lineage is authoritative truth, not just
a base graph plus advisory metadata.

### Must Ship

- explicit replace/split/merge-like lineage event coverage
- authoritative promotion flow from advisory correspondence
- invalid and ambiguous correspondence rejection
- branch-local identity-evolution isolation
- historical ID resolution across legitimate lineage chains
- lineage graph as a named query surface, not only internal bookkeeping
- historical resolution ergonomics that downstream systems can consume directly
- lineage-aware committed change surfaces where identity evolution must appear

### Must Preserve

- permanent separation between storage identity and lineage identity
- advisory correspondence stays non-authoritative until explicit promotion
- canonical serialized lineage finalization
- branch-local lineage does not leak across branches
- deterministic replay and diagnostics of lineage behavior

### Acceptance Requirements

This milestone is complete only when the implementation satisfies the following
named requirements from
[test-requirements.md](/Users/spenstar/Documents/programming/forge%20workspace/Forge/_docs/forge-relational/test-requirements.md):

- `Lineage/correspondence hardening test`
- `Topology identity survival test`
- `Netlist rewiring identity and history test`
- `Hostile commit/replay equivalence test`

## Milestone 7: Merge-Ready History and Merge Execution

This milestone is intentionally split so the roadmap stays honest about what is
already structurally supported versus what may still be missing as product
behavior.

### Milestone 7A: Merge-Ready History Certification

#### Goal

Prove that merge-ready history shape is operationally real across replay,
durability, diagnostics, and ancestry reasoning.

#### Must Ship

- ordered multi-parent commit-envelope fixtures as accepted certification inputs
- ordered parent persistence through durability
- replay handling for ordered parent lists
- diagnostics and ancestry reasoning that remain correct on ordered parents
- explicit evidence that observable surfaces do not quietly assume “single parent or none”

#### Acceptance Requirements

This sub-milestone is complete only when the implementation satisfies the
following named requirement from
[test-requirements.md](/Users/spenstar/Documents/programming/forge%20workspace/Forge/_docs/forge-relational/test-requirements.md):

- `Merge-ready history shape test`

### Milestone 7B: Authoritative Merge Execution

#### Goal

If authoritative multi-parent merge commit creation is not yet complete,
finish it as a first-class runtime feature.

#### Must Ship

- authoritative multi-parent commit execution
- deterministic parent ordering at merge commit creation time
- merge conflict classification and structured diagnostics
- replay semantics for merge commits
- durable publication semantics for merge commits
- branch-head advancement semantics for successful merges

#### Explicit Future Merge/Reconciliation Requirements

Milestone 7 must be treated as more than "make merge commits exist."

The roadmap needs to be explicit that several merge/reconciliation behaviors are
not fully supported yet but are architecturally required for a production-grade
relational runtime. These are not optional refinements to append later if
someone notices them. They are part of the real merge product surface.

Required future merge behaviors include:

- persistent identity matching across branches when corresponding records are
  not the same raw record id
- aspect-aware merge into an existing target record when one branch carries
  richer declared aspect structure than another
- deletion/removal semantics as a first-class merge result
- relation endpoint rewiring merge semantics when one branch changes topology
  and another changes payload/aspects
- typed three-way merge semantics over base/source/target for record state,
  aspect deltas, and relation structure
- policy-driven conflict resolution beyond honest conflict detection and
  fail-closed rejection
- partial mergeability where non-conflicting regions can converge while
  conflicting regions remain isolated
- rich merge explanation surfaces that can answer why a target branch ended up
  with a particular adopted, rejected, or reconciled aspect shape

One especially important required behavior is this:

- merging richer aspect structure into an existing poorer target structure must
  eventually be modeled as explicit reconciliation semantics, not silently
  collapsed into ordinary introduction, ordinary replacement, or lineage
  continuity alone

The roadmap is intentionally explicit here because the likely failure mode would
otherwise be one of:

- duplicating logically corresponding records instead of reconciling them
- treating richer aspect structure as unrelated introduction
- overloading lifecycle or lineage continuity with merge identity meaning
- bolting on later heuristic matching without a first-class identity contract

Milestone 2 is only a prerequisite for this work. Milestone 7 is where the
runtime must become explicit about these merge/reconciliation behaviors as real
product requirements.

#### Must Preserve

- single serialized authority for final truth commit
- canonical observability and replay
- coherent publication
- explicit failure rather than partial merge truth

#### Acceptance Requirements

This sub-milestone is complete only when:

- the roadmap is paired with an explicit merge-execution certification test or suite if
  [test-requirements.md](/Users/spenstar/Documents/programming/forge%20workspace/Forge/_docs/forge-relational/test-requirements.md)
  does not already contain one
- `Hostile commit/replay equivalence test` is satisfied for merge-bearing histories
- `Durable recovery and schema mismatch test` is satisfied for merge-bearing histories
- `Merge-ready history shape test` remains satisfied on real merge-produced histories, not only fixtures

## Milestone 8: Parallel Read, Bulk Mutation, and Scale Query Completion

### Goal

Finish the scale side of the vision so the runtime can honestly claim
first-class industrial bulk query and bulk mutation behavior, not just strong
commit-time preparation.

### Must Ship

- deterministic packetized planning for bulk reads over immutable snapshots
- explicit parallel read execution where supported
- large-surface bulk traversal/query APIs as primary surfaces
- low-level bulk mechanical primitives for relation traversal and scans, with semantic layering remaining above them
- parity between index-assisted reads and authoritative storage fallback
- read-side locality, partition, and work-packet metrics
- partition-aware query surfaces and partitioning hints where the runtime can expose them generically
- memory stability guarantees strong enough for long-lived high-scale workloads
- bulk mutation as a scale product surface rather than only repeated single-write composition
- graph introspection surfaces strong enough to inspect read and mutation scale behavior
- snapshot-stable read behavior under active mutation and high churn

### Must Preserve

- immutable snapshot semantics
- no hidden read mutation or repair
- non-authoritative derived indexes
- authoritative storage fallback always available
- deterministic observable query order

### Acceptance Requirements

This milestone is complete only when the implementation satisfies the following
named requirements from
[test-requirements.md](/Users/spenstar/Documents/programming/forge%20workspace/Forge/_docs/forge-relational/test-requirements.md):

- `Bulk query and traversal stress truth test`
- `Index non-authority corruption test`
- `Deterministic observability under hostile scheduling test`
- `Snapshot-stable concurrent read vs hot rewrite test`

## Milestone 9: Generic Certification Program

### Goal

Run the full generic truth-grade certification program after the remaining
product milestones are implemented.

### Scope

This milestone is not for discovering missing features. It is for proving the
completed runtime under the hostile scenarios already defined in
[test-requirements.md](/Users/spenstar/Documents/programming/forge%20workspace/Forge/_docs/forge-relational/test-requirements.md).

### Acceptance Requirements

This milestone is complete only when all ten generic named requirements in
[test-requirements.md](/Users/spenstar/Documents/programming/forge%20workspace/Forge/_docs/forge-relational/test-requirements.md)
are satisfied:

- `Hostile commit/replay equivalence test`
- `Savepoint rollback fracture test`
- `Snapshot pinning and reclaim correctness test`
- `Deterministic observability under hostile scheduling test`
- `Index non-authority corruption test`
- `Diff/CDC truth parity test`
- `Lineage/correspondence hardening test`
- `Merge-ready history shape test`
- `Bulk query and traversal stress truth test`
- `Durable recovery and schema mismatch test`

Each certification run must emit canonical machine-checkable artifact bundles,
not only human-readable logs, exactly as required by the requirements doc.

## Milestone 9: Domain Certification Program

### Goal

Prove that the generic runtime is actually fit for the stated target domains.

### Acceptance Requirements

This milestone is complete only when all four domain-specific named
requirements in
[test-requirements.md](/Users/spenstar/Documents/programming/forge%20workspace/Forge/_docs/forge-relational/test-requirements.md)
are satisfied:

- `Topology identity survival test`
- `Missing-twin / nonmanifold corruption localization test`
- `Netlist rewiring identity and history test`
- `Snapshot-stable concurrent read vs hot rewrite test`

The requirements doc remains the authoritative source for the exact CAD and
chip scenarios, verification output, and pass conditions these tests impose.

## Per-Milestone Format

For consistency and readability, every milestone in this roadmap uses the same
shape:

- `Goal`
- `Must Ship`
- `Must Preserve`
- `Acceptance Requirements`

The acceptance section deliberately references
[test-requirements.md](/Users/spenstar/Documents/programming/forge%20workspace/Forge/_docs/forge-relational/test-requirements.md)
explicitly rather than trying to restate those tests in weaker or shorter form.

## Completion Standard

Forge Relational is roadmap-complete only when:

- all remaining product milestones are shipped
- all ten generic named requirements in
  [test-requirements.md](/Users/spenstar/Documents/programming/forge%20workspace/Forge/_docs/forge-relational/test-requirements.md)
  are satisfied
- both CAD named requirements in
  [test-requirements.md](/Users/spenstar/Documents/programming/forge%20workspace/Forge/_docs/forge-relational/test-requirements.md)
  are satisfied
- both chip named requirements in
  [test-requirements.md](/Users/spenstar/Documents/programming/forge%20workspace/Forge/_docs/forge-relational/test-requirements.md)
  are satisfied
- all certification runs emit canonical machine-checkable artifacts for truth,
  patches, diagnostics, lineage, replay, branch heads, and query surfaces
