# Forge Signal Scale Hardening Plan

## Purpose

This document is the strict performance and robustness hardening plan for `forge-signal` after the prepared execution cleanup and before we pretend the runtime is ready for very large CAD, aerospace, or other mission-critical workloads.

The standard here is not "works in tests."

The standard is:

- deterministic under extreme graph depth and width
- auditable without destroying throughput
- explicit about what is and is not parallel
- structured so hot-path state is cache-friendly
- hostile to hidden runtime assumptions

## Current Truth

`forge-signal` now has one truthful execution model:

- immutable execution snapshot
- task-local dependency capture
- prepared precompute
- deterministic serial apply
- optional same-stage parallel precompute

That is a good foundation.

It is not yet an aerospace-grade scale story.

## Readiness Matrix

### Safe To Treat As Real Foundation Now

- prepared execution model
- deterministic staged planner/executor backbone
- serial apply semantics
- bounded diagnostics profiles
- partition-aware invalidation and explanation
- transaction rollback semantics

### Works, But Not Aerospace-Grade Yet

- planner throughput on large graphs
- synchronous invalidation latency
- diagnostics cost under high-frequency execution
- same-stage parallel precompute
- `NodeEntry` memory density and locality
- dependency snapshot storage locality

### Convenience Only

- `forge-signal::easy`

`easy` is a UX layer. It is not the runtime surface that should carry kernel-grade or aerospace-grade workloads.

## The Parallelism Reality

We did not build "full parallel evaluation."

We built a narrower and honest version:

- planning is serial
- stage snapshot build is serial
- precompute within one stage can run in parallel
- apply is serial
- commit semantics remain deterministic

That is not a watered down mistake. It is the correct safe intermediate architecture.

What would be a mistake is pretending this means the runtime already has mature parallel execution.

It does not.

What still remains:

- executor policy based on stage width and estimated task cost
- thread-pool-backed dispatch instead of ad hoc stage thread spawning
- clearer cost model for when parallelism helps vs hurts
- patch-buffered or otherwise deterministic concurrent apply design if we ever want more than serial apply
- cache-line and write-amplification analysis before any parallel write-back

## Top Structural Risks

## 1. Planner Hot-Path Cost

The planner still pays for:

- `BTreeMap` / `BTreeSet`
- target sorting
- per-stage sorting
- recursive depth calculation
- cloned dependency lists for sorting

This is acceptable for current productization.
It is not the right end-state for very large graphs or frequent interactive replans.

### Required direction

- remove recursive depth calculation
- move toward arena-indexed vectors / bitsets on the hot path
- cache or maintain topological rank/depth on edge mutation
- preserve determinism without repeated tree-based sorting

## 2. Synchronous Push Invalidation

Invalidation still performs a synchronous downstream walk.

That is simple and correct.
It can also become the main hitch source for large fanout or deeply interactive editing.

### Required direction

- keep the deterministic push semantics
- add budgeted or deferred orchestration above the core runtime for interactive environments
- reduce cloning during downstream subscription checks
- preserve the current bitset-backed frontier model and extend that style where possible

## 3. Diagnostics Cost In Hot Loops

Diagnostics are core infrastructure, not optional polish.

That does not mean every diagnostics artifact should be rebuilt from scratch on every execution.

Current risk areas:

- whole-graph history summarization
- explanation allocation and sorting cost
- repeated string cloning in summaries and traces
- rich-profile overhead in high-frequency loops

### Required direction

- keep `Operational` cheap enough for always-on use
- make richer profiles explicitly more expensive and clearly documented as such
- incrementally maintain history/report summaries where possible
- continue separating trust surfaces from hot execution state

## 4. Node And Snapshot Locality

`NodeEntry` is still too fat for the long-term target.
`DependencySnapshot` is still per-node `Vec` storage.

That means:

- hot state and cold trace data live together
- many small heap-owned buffers exist for snapshots
- locality degrades as graphs scale

### Required direction

Do not jump straight to a massive rewrite unless measurement forces it.

Prefer this sequence:

1. split hot and cold node data
2. move dependency snapshots toward span-based storage in graph-owned buffers
3. then evaluate whether full SoA is justified

## 5. Dynamic And String-Heavy Convenience Paths

The runtime core is getting more explicit.
The convenience layers are still runtime-dynamic in places:

- `easy` uses `Box<dyn Any>`
- typed expectations still fail at runtime
- several public identity/token surfaces still wrap `String`

### Required direction

- keep dynamic convenience APIs quarantined from kernel-grade use
- prefer interned IDs or typed numeric handles in hot paths
- move implicit assumptions into typed wrappers or explicit host contracts when possible

## Prioritized Hardening Roadmap

## Tranche 1: Must-Harden Before Calling The Runtime Scale-Ready

1. Remove recursive planner depth calculation.
2. Add explicit planner/executor truth to docs: parallel precompute only, serial apply.
3. Introduce executor thresholds so narrow stages do not pay parallel overhead.
4. Audit and reduce cloned dependency scans in invalidation/planner hot paths.
5. Keep `Operational` diagnostics on an incremental-cost budget.

## Tranche 2: Required Before Very Large CAD / Aerospace Workloads

1. Hot/cold split for node state vs trace/causality/diagnostics metadata.
2. Span-based dependency snapshot storage owned by `SignalGraph`.
3. Cached topo rank / stage metadata maintained on edge mutation.
4. Replace tree-heavy planner internals with arena-indexed structures where safe.
5. Push string-backed identities out of hot execution structures.

## Tranche 3: Required Before Claiming Mature Parallel Execution

1. Replace per-stage scoped thread spawning with a real executor/pool strategy.
2. Add task-granularity policy for serial vs parallel stage dispatch.
3. Measure false sharing and write amplification before any concurrent apply work.
4. If concurrent apply is ever pursued, design deterministic patch-buffered merge first.
5. Add performance parity tests that prove parallelism helps on real stage shapes.

## Tranche 4: Compile-Time Contract Hardening

1. Keep convenience/runtime-dynamic APIs out of the critical kernel path.
2. Replace runtime string identity assumptions with interned or typed IDs where feasible.
3. Reduce `expect(...)`-based invariants in non-test code.
4. Prefer typed host integration contracts over implicit behavior.

## Non-Goals Right Now

- do not chase full async/non-blocking internals first
- do not parallelize apply just to claim a more advanced executor
- do not rewrite the entire graph into SoA before measuring where the wall is
- do not let `easy` define the performance shape of the runtime core

## Recommendation

The next work after the harness should not be "make everything async."

It should be:

1. planner hot-path hardening
2. diagnostics cost discipline
3. node/snapshot locality improvements
4. honest executor policy and parallelism maturation

That sequence keeps the runtime deterministic and auditable while moving it toward the scale envelope needed for very large engineering systems.
