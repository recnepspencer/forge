# forge-relational Architecture Roadmap

## Purpose

This document is the execution roadmap for the architectural refactor defined in [relational_architecture.md](./relational_architecture.md).

It is separate from [forge_relational_roadmap.md](./forge_relational_roadmap.md), which tracks the runtime's feature path. This document is only about architectural sequencing, milestone boundaries, and how the refactor should be planned and executed.

The architecture document is the design authority. This roadmap is the sequencing authority.

## Operating Rule

We do not invent milestone order ad hoc. We follow the dependency order already established in [relational_architecture.md](./relational_architecture.md):

1. Phase A: Foundation Types
2. Phase B: Internal Cleanup
3. Phase C: Runtime Decomposition
4. Phase D: Invariant Engine
5. Phase E: Commit Architecture
6. Phase F: API Surface

Breaking changes are acceptable throughout this program. Compatibility is not the goal. Structural correctness, semantic clarity, and long-term leverage are the goal.

## How This Roadmap Is Used

This roadmap is intentionally low-resolution and milestone-oriented.

The workflow is:

1. identify the next milestone from this document
2. map that milestone back to the relevant phase items in [relational_architecture.md](./relational_architecture.md)
3. write a high-resolution implementation plan for that milestone only
4. implement that milestone
5. verify acceptance before moving to the next milestone

We do not produce one giant implementation plan for the whole architectural program. Each milestone gets planned in high resolution only when it becomes active.

## Non-Negotiable Rules

These rules come from [forge_relational_vision.md](./forge_relational_vision.md) and [relational_architecture.md](./relational_architecture.md). Every milestone plan must preserve them.

- authoritative truth mutation remains serialized
- observable outputs remain deterministic and canonically ordered
- committed reads remain immutable
- replay remains derived from canonical commit artifacts
- derived indexes remain non-authoritative and retain storage-visible fallback semantics
- reads never perform hidden mutation, normalization, or repair
- breaking APIs is acceptable; semantic drift is not
- harness parity is part of completion, not follow-up cleanup
- if intermediate structural maintenance has no semantic value inside a batch,
  amortize it at the merged-batch boundary and reuse batch-derived summaries or
  deltas instead of recomputing structure in each subsystem

## Milestone Planning Template

Every milestone-specific plan should include:

- Objective
- Included architecture items
- Required code changes
- Acceptance criteria
- Risks to watch
- Must remain unchanged

This keeps planning local, decision-complete, and aligned with the architecture spec.

## Milestones

### Milestone 1: Foundation Types

Reference:
[relational_architecture.md](./relational_architecture.md), Phase A

Included items:

- A1 · Phantom-Tagged Identity Types
- A2 · Compiler-Enforced Slot Construction for `RecordArena`
- A3 · Unified Error Hierarchy with Structured Context

Objective:
Stabilize the primitive layer so downstream refactors build on better identity types, compiler-enforced storage initialization, and one runtime error taxonomy.

Why first:
These changes have the fewest architectural dependencies and the highest leverage on every later milestone.

Acceptance focus:

- identity duplication is removed in favor of domain-tagged generic primitives
- arena slot initialization becomes compiler-enforced at construction sites
- subsystem errors compose through a unified relational error surface

Must remain unchanged:

- SoA arena layout
- entity/relation identity separation
- deterministic observable behavior

### Milestone 2: Internal Cleanup

Reference:
[relational_architecture.md](./relational_architecture.md), Phase B

Included items:

- B1 · `MutationWorkspace` Combinator Audit
- B2 · `RelationalDraft` Delegation Cleanup
- B3 · Nested Config Sections
- B4 · Unified Intent Hierarchy
- B5 · Declarative Effect Assembly

Objective:
Reduce local structural friction so runtime decomposition happens on clearer internal contracts.

Why second:
This phase removes redundant hierarchies, overgrown config shape, and repetitive mutation plumbing that would otherwise amplify churn in Milestone 3.

Acceptance focus:

- closure-based split-borrow stays intact, but the combinator surface is pruned
- config organization aligns with real subsystem boundaries
- intent hierarchy is unified
- observability assembly is framework-driven rather than copied in every handler

Must remain unchanged:

- transaction-only mutation authority
- deterministic patch and diagnostics output
- current commit semantics

### Milestone 3: Runtime Decomposition

Reference:
[relational_architecture.md](./relational_architecture.md), Phase C

Included items:

- C1 · God Struct to Subsystem Split
- C2 · Visibility Cache Encapsulation
- C3 · `SnapshotGuard` Scope Narrowing
- C4 · Fork-Safe Runtime Construction

Objective:
Replace `RelationalRuntime` as the dominant mutable boundary with explicit subsystem ownership and narrow runtime borrows.

Why third:
This is the keystone milestone. Later invariant, commit, and API work all depend on the runtime no longer being centered on one oversized authority object.

Acceptance focus:

- runtime state is split into coherent subsystems with focused APIs
- commit phases borrow only the subsystems they need
- visibility/cache locking is encapsulated semantically
- snapshot guards no longer monopolize the entire runtime

Must remain unchanged:

- coherent publication semantics
- immutable snapshot semantics
- single-writer commit authority

### Milestone 4: Invariant Engine

Reference:
[relational_architecture.md](./relational_architecture.md), Phase D

Included items:

- D1 · Bitmask Invariant Groups
- D2 · Invariant Cost Classification
- D3 · Invariant Policy Runtime
- D4 · Intent Contracts
- D5 · Three-State Invariant Verdicts
- D6 · State-Derived Invariant Context

Objective:
Turn invariants into explicit contracts with targeted execution rather than broad runtime passes.

Why fourth:
Once subsystem boundaries are stable, invariant execution can become precise, cheap to dispatch, and directly tied to mutation intent.

Acceptance focus:

- invariants declare group, cost, execution point, and failure effect
- mutation intents declare what invariant groups they can affect
- invariant execution becomes targeted and policy-driven
- verdicts distinguish pass, advisory, and violation cases
- merged-plan structural facts are hoisted and reused rather than repeatedly
  recomputed inside invariant paths

Must remain unchanged:

- always-on structural safety
- deterministic diagnostics
- explicit failure classes
- batch-final truth semantics: intermediate structural maintenance remains
  non-authoritative unless it is itself an explicit batch artifact

### Milestone 5: Commit Architecture

Reference:
[relational_architecture.md](./relational_architecture.md), Phase E

Included items:

- E1 · Commit Decision Log
- E2 · Commit Result Envelope

Objective:
Make commit outcomes first-class artifacts with one coherent result surface and explicit decision tracing through the authority path.

Why fifth:
This phase depends on the type, runtime, and invariant work being settled enough to formalize commit artifacts cleanly.

Acceptance focus:

- commit decisions are explicitly recorded
- callers receive one coherent commit result envelope
- success and failure paths expose structured machine-usable context
- commit phases consume batch-derived summaries or deltas rather than
  rediscovering the same structural facts independently

Must remain unchanged:

- atomic publication contract
- canonical patch/replay relationship
- branch/history determinism
- semantic focus on final batch truth rather than intermediate per-intent
  structural states

### Milestone 6: API Surface

Reference:
[relational_architecture.md](./relational_architecture.md), Phase F

Included items:

- F1 · Facade Namespace Organization
- F2 · Type-Driven Read Surface (`RecordProjection`)

Objective:
Expose a public surface that reflects the final runtime contracts instead of leaking low-level traversal mechanics.

Why sixth:
The external API should be stabilized last, after the underlying contracts are actually correct.

Acceptance focus:

- facade layout matches real domain boundaries
- type-driven projection becomes the primary ergonomic read surface
- low-level read primitives remain subordinate infrastructure, not the main contract

Must remain unchanged:

- storage-visible fallback semantics
- deterministic query outputs
- read/write aspect contract duality

## Continuous Acceptance Expectations

These run across all milestones:

- keep `forge-relational` tests green after each completed slice
- add or update parity coverage when contracts change
- verify deterministic ordering on every new observable surface
- treat semantic regressions as more severe than API breakage
- prefer harness-backed validation for replay, publication, snapshots, diagnostics, and branch/history behavior

## Closeout Rule

A milestone is not complete just because code compiles.

A milestone is complete when:

- the included architecture items are implemented
- acceptance criteria for that milestone are verified
- no prohibited semantic drift was introduced
- the next milestone can be planned against a cleaner, more stable system
