# Forge Signal Fintech Certification Matrix

## Purpose

This document defines the certification matrix for the `forge-signal` fintech test domain.

The goal is not to accumulate more tests. The goal is to ensure that the fintech domain closes specific coverage gaps in the real `forge-signal` API surface before we move on to another domain.

Each workflow we add should close an explicit uncovered cell in this matrix:

- API family
- bug class
- workflow expression

A workflow that cannot name those three things is probably filler.

---

## Scope

This matrix is specifically about what the fintech test domain should certify for `forge-signal`.

It is not the whole crate certification story.

Other existing crate-level suites still matter for:

- low-level graph contracts
- isolated planner semantics
- generic condition/comparator behavior
- event bus contracts
- keyed runtime internals
- diagnostics and observability specifics

The fintech domain exists to prove that those capabilities survive in realistic, hostile, world-shaped workflows.

---

## Signal Surface Reference

The fintech matrix should be aligned to the actual `forge-signal` public surface and vision, not just the currently convenient domain workflows.

Relevant sources:

- [`crates/forge-signal/src/lib.rs`](/Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-signal/src/lib.rs)
- [`crates/forge-signal/src/facade.rs`](/Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-signal/src/facade.rs)
- [`crates/forge-signal/BOUNDARY_CONTRACT.md`](/Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-signal/BOUNDARY_CONTRACT.md)
- [`crates/forge-signal/docs/API_SURFACE.md`](/Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-signal/docs/API_SURFACE.md)
- [`crates/forge-signal/docs/ADVANCED_PATTERNS.md`](/Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-signal/docs/ADVANCED_PATTERNS.md)
- [`_docs/engineering/forge_signal_vision.md`](/Users/spenstar/Documents/programming/forge%20workspace/Forge/_docs/engineering/forge_signal_vision.md)

The long-term target domains include CAD / geometry kernels and chip simulators. That means the fintech domain should explicitly exercise:

- aspect isolation
- condition gating
- comparator and tolerance behavior
- partition/locality behavior
- keyed and memoized computation
- tier policy behavior
- replay / lineage / branch / checkpoint semantics
- deterministic parallel overlap
- retained vs reconstructed artifacts

---

## API Families

These are the `forge-signal` API families that fintech can and should help certify.

### 1. World Assembly and Graph Construction

Primary surfaces:

- world assembly helpers
- `SignalGraph`
- `NodeBuilder`
- dependency wiring

Why it matters:

- bad world setup can hide real runtime bugs
- graph shape mistakes can make hostile workflows meaningless

### 2. Multi-Aspect Invalidation and Propagation

Primary surfaces:

- `Aspect`
- `AspectMask`
- `mark_dirty`
- multi-aspect node dependencies

Why it matters:

- future geometry and chip workloads will rely on aspect isolation heavily

### 3. Conditions

Primary surfaces:

- `EvaluationCondition`
- `on_demand`
- `debounce`
- `aspect_filter`
- `delta_threshold`
- `custom_condition`

Why it matters:

- fintech currently proves threshold pressure, but not the full condition family

### 4. Comparators and Tolerance Suppression

Primary surfaces:

- `VersionComparatorPolicy`
- node comparators
- tolerance-driven suppression behavior

Why it matters:

- tolerance mistakes are among the highest-risk future kernel bugs

### 5. Partitioned Outputs and Changed Regions

Primary surfaces:

- partitioned output nodes
- `ChangedRegion`
- partition and partition-detail subscriptions

Why it matters:

- this is crucial for geometry locality and simulator fanout

### 6. Transactions and Rollback

Primary surfaces:

- `SignalRuntime`
- `SignalTransaction`
- staged evaluation inside transactions
- hard rewind on failure

Why it matters:

- hostile workflows should never leak partial semantic state

### 7. Snapshots, Branches, Replay, and Lineage

Primary surfaces:

- snapshot capture and restore
- branch creation and switching
- replay slices
- lineage records

Why it matters:

- this is the center of hostile workflow certification today

### 8. Planner and Executor Behavior

Primary surfaces:

- plan build and execute
- `StageExecutor`
- serial vs parallel overlap
- parallel admission behavior

Why it matters:

- determinism is a product contract

### 9. Tier Policy and Scheduling Policy

Primary surfaces:

- node tiers
- tier policies
- dependency modes
- evaluation triggers
- dirty propagation modes

Why it matters:

- mixed cheap/live and expensive/audit workflows are realistic in fintech and critical in future domains

### 10. Keyed Nodes and Structural Memoization

Primary surfaces:

- computation families
- keyed nodes
- structural memo keys

Why it matters:

- cache correctness will matter for scenario packs, geometry artifacts, and simulator subresults

### 11. Diagnostics, Explanation, and Artifact Materialization

Primary surfaces:

- replay
- lineage
- explanation
- retained vs reconstructed artifact access
- runtime policy retention differences

Why it matters:

- the signal vision is diagnostics-first, not diagnostics-later

### 12. Harness and Certification Honesty

Primary surfaces:

- workflow certification adapter
- overlap-aware comparison
- artifact capture
- failure bundle content

Why it matters:

- a dishonest certification layer produces false confidence

---

## Bug Classes

These are the signal bug classes the fintech domain should target.

### B1. Wrong Truth

Derived state is semantically wrong after a workflow.

### B2. Aspect Leakage

Too much or too little invalidation propagates across aspects.

### B3. Condition Gating Error

Work runs when it should not, or fails to run when it must.

### B4. Comparator / Tolerance Suppression Error

Meaningful change is suppressed, or non-meaningful churn propagates.

### B5. Partition / Locality Corruption

Local updates leak across partitions or fail to invalidate local dependents.

### B6. Transaction Atomicity / Rollback Failure

Aborted work leaks graph-visible or semantic state.

### B7. Branch / Replay / Recovery Drift

Branch-local truth, replay, lineage, or restored state diverges incorrectly.

### B8. Determinism / Parallel Drift

Serial and parallel runs disagree beyond guaranteed overlap.

### B9. Tier-Policy Inconsistency

Node scheduling behavior changes silently under tier policy differences.

### B10. Keyed Cache / Memo Corruption

Family-scoped or keyed computations leak, collide, or reuse stale results.

### B11. Diagnostics / Provenance Disagreement

Truth changed correctly, but replay, lineage, explanation, or artifact materialization lies.

### B12. Harness Overclaim / False Comparison

The adapter or certification runner compares or promises more than the runtime actually guarantees.

---

## Workflow Families

These are the workflow families the fintech domain should eventually provide.

Each family exists to close specific matrix cells.

### W1. Seeded World Assembly

Purpose:

- prove the default world comes up alive, seeded, branchable, and structurally correct

Primary API families:

- 1

Primary bug classes:

- B1

Current status:

- Covered

Current files:

- [`world_setup.rs`](/Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-signal/src/tests/domains/fintech/world_setup.rs)

### W2. Hostile Branch Isolation

Purpose:

- prove branch-local truth, replay, and branch head metadata remain isolated under churn

Primary API families:

- 6
- 7

Primary bug classes:

- B6
- B7

Current status:

- Covered baseline

Current files:

- [`branch_isolation.rs`](/Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-signal/src/tests/domains/fintech/branch_isolation.rs)

### W3. Threshold Flap and Rollback Storm

Purpose:

- pressure `DeltaThreshold` conditions plus rollback and restore

Primary API families:

- 3
- 6
- 7

Primary bug classes:

- B3
- B6
- B7

Current status:

- Covered for threshold only

Current files:

- [`threshold_flapping.rs`](/Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-signal/src/tests/domains/fintech/threshold_flapping.rs)

### W4. Partial Refresh Snapshot Recovery

Purpose:

- prove restore semantics after partial observation and hostile mutation

Primary API families:

- 6
- 7
- 11

Primary bug classes:

- B6
- B7
- B11

Current status:

- Covered baseline

Current files:

- [`snapshot_recovery.rs`](/Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-signal/src/tests/domains/fintech/snapshot_recovery.rs)

### W5. Serial vs Parallel Hostile Overlap

Purpose:

- prove guaranteed overlap under executor variation

Primary API families:

- 8
- 12

Primary bug classes:

- B8
- B12

Current status:

- Covered baseline

Current files:

- [`executor_overlap.rs`](/Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-signal/src/tests/domains/fintech/executor_overlap.rs)
- [`certification.rs`](/Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-signal/src/tests/domains/fintech/certification.rs)

### W6. High-Fanout Tolerance Pressure

Purpose:

- pressure threshold and recovery behavior under larger fanout

Primary API families:

- 4
- 6
- 7

Primary bug classes:

- B4
- B6
- B7

Current status:

- Covered baseline

Current files:

- [`fanout_tolerance.rs`](/Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-signal/src/tests/domains/fintech/fanout_tolerance.rs)

### W7. Aspect Isolation Workflow

Purpose:

- prove one market aspect can change without fabricating unrelated invalidation

Primary API families:

- 2

Primary bug classes:

- B2

Current status:

- Missing in fintech

### W8. Aspect Filter Condition Workflow

Purpose:

- prove aspect-filtered nodes wake only on intended aspects

Primary API families:

- 2
- 3

Primary bug classes:

- B2
- B3

Current status:

- Missing in fintech

### W9. On-Demand Workflow

Purpose:

- prove on-demand nodes defer and then evaluate when explicitly pulled

Primary API families:

- 3

Primary bug classes:

- B3

Current status:

- Missing in fintech

### W10. Debounce Workflow

Purpose:

- prove quiet-period gating is respected under repeated updates

Primary API families:

- 3

Primary bug classes:

- B3

Current status:

- Missing in fintech

### W11. Custom Condition Workflow

Purpose:

- prove host-driven condition decisions behave deterministically in a world-shaped case

Primary API families:

- 3

Primary bug classes:

- B3

Current status:

- Missing in fintech

### W12. Comparator Policy Workflow

Purpose:

- prove exact vs tolerance comparator behavior with realistic source churn

Primary API families:

- 4

Primary bug classes:

- B4

Current status:

- Missing in fintech

### W13. Partition-Local Bucket Workflow

Purpose:

- prove bucket-local or scenario-local changes stay local under partition-aware subscriptions

Primary API families:

- 5

Primary bug classes:

- B5

Current status:

- Missing in fintech

### W14. Changed-Region Workflow

Purpose:

- prove changed-region propagation aligns with actual local invalidation promises

Primary API families:

- 5

Primary bug classes:

- B5
- B11

Current status:

- Missing in fintech

### W15. Wrong-Branch / Wrong-Snapshot Recovery Workflow

Purpose:

- prove invalid recovery paths fail cleanly and preserve forensic evidence

Primary API families:

- 6
- 7
- 12

Primary bug classes:

- B6
- B7
- B12

Current status:

- Missing in fintech

### W16. Invalid Transition Workflow

Purpose:

- prove state-machine-invalid workflow transitions are rejected explicitly

Primary API families:

- 7
- 12

Primary bug classes:

- B7
- B12

Current status:

- Missing in fintech

### W17. Tiered Live-vs-Audit Workflow

Purpose:

- prove cheap live nodes and expensive audit nodes obey distinct tier policy semantics

Primary API families:

- 8
- 9

Primary bug classes:

- B8
- B9

Current status:

- Missing in fintech

### W18. Keyed Scenario Pack Workflow

Purpose:

- prove keyed or memoized scenario bundles stay family-scoped and branch-safe

Primary API families:

- 10

Primary bug classes:

- B10

Current status:

- Missing in fintech

### W19. Artifact Materialization Workflow

Purpose:

- prove retained vs reconstructed explanation/provenance expectations under different runtime policies

Primary API families:

- 11

Primary bug classes:

- B11

Current status:

- Missing in fintech

### W20. Coarse-vs-Fine Truth Trap Workflow

Purpose:

- prove lower-level truth can diverge even when coarse audit surfaces converge

Primary API families:

- 2
- 11
- 12

Primary bug classes:

- B2
- B11
- B12

Current status:

- Partially covered informally, not formalized

### W21. Event Ordering Workflow

Purpose:

- prove alert or audit subscribers flush deterministically and roll back cleanly

Primary API families:

- 6
- 11

Primary bug classes:

- B6
- B11

Current status:

- Missing in fintech

---

## Matrix Summary

This table is intentionally coarse. It exists to show where fintech is already strong and where it is still thin.

| Workflow Family | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| W1 Seeded World Assembly | X |  |  |  |  |  |  |  |  |  |  |  |
| W2 Hostile Branch Isolation |  |  |  |  |  | X | X |  |  |  |  |  |
| W3 Threshold Flap and Rollback Storm |  |  | X |  |  | X | X |  |  |  |  |  |
| W4 Partial Refresh Snapshot Recovery |  |  |  |  |  | X | X |  |  |  | X |  |
| W5 Serial vs Parallel Hostile Overlap |  |  |  |  |  |  |  | X |  |  |  | X |
| W6 High-Fanout Tolerance Pressure |  |  |  | X |  | X | X |  |  |  |  |  |
| W7 Aspect Isolation |  | X |  |  |  |  |  |  |  |  |  |  |
| W8 Aspect Filter Condition |  | X | X |  |  |  |  |  |  |  |  |  |
| W9 On-Demand |  |  | X |  |  |  |  |  |  |  |  |  |
| W10 Debounce |  |  | X |  |  |  |  |  |  |  |  |  |
| W11 Custom Condition |  |  | X |  |  |  |  |  |  |  |  |  |
| W12 Comparator Policy |  |  |  | X |  |  |  |  |  |  |  |  |
| W13 Partition-Local Bucket |  |  |  |  | X |  |  |  |  |  |  |  |
| W14 Changed Region |  |  |  |  | X |  |  |  |  |  | X |  |
| W15 Wrong-Branch / Wrong-Snapshot |  |  |  |  |  | X | X |  |  |  |  | X |
| W16 Invalid Transition |  |  |  |  |  |  | X |  |  |  |  | X |
| W17 Tiered Live-vs-Audit |  |  |  |  |  |  |  | X | X |  |  |  |
| W18 Keyed Scenario Pack |  |  |  |  |  |  |  |  |  | X |  |  |
| W19 Artifact Materialization |  |  |  |  |  |  |  |  |  |  | X |  |
| W20 Coarse-vs-Fine Truth Trap |  | X |  |  |  |  |  |  |  |  | X | X |
| W21 Event Ordering |  |  |  |  |  | X |  |  |  |  | X |  |

Interpretation:

- fintech is currently strongest in transactions, branch/recovery, and executor overlap
- fintech is weak in partitions, keyed caches, tier policy, non-threshold conditions, and artifact materialization

---

## Helper and Infrastructure Plan

Before filling the missing rows, we should build helper layers that make new workflows cheap and honest.

### H1. Case Truth Capture Helpers

Needed because:

- top-level audit truth is too coarse on its own
- several workflows need both coarse and fine probes

Should capture:

- primary market source truth
- threshold truth
- coarse audit truth
- bucket aggregate truth
- scenario aggregate truth
- branch head snapshot metadata
- replay summary
- lineage summary

Suggested location:

- fintech test domain support module, likely split by truth responsibility rather than a generic `helpers.rs`

### H2. Case Comparison Helpers

Needed because:

- several workflows should compare the same shaped artifacts
- we already learned that the wrong comparison surface creates false assumptions

Should provide:

- exact truth comparison
- guaranteed-overlap comparison
- coarse-vs-fine mismatch detection
- branch-local replay comparison
- restore-before/after truth comparison

### H3. Condition Pressure Verbs

Needed because:

- current world verbs focus on threshold pressure only

Should provide:

- on-demand trigger verbs
- debounce timing or resolver hooks
- aspect-filter mutation verbs
- custom-condition resolver hooks

### H4. Partition and Locality Verbs

Needed because:

- partition and changed-region support is one of the biggest uncovered signal surfaces

Should provide:

- local bucket mutation verbs
- local scenario mutation verbs
- changed-region emission helpers
- partition-specific truth capture

### H5. Recovery Error Injection Verbs

Needed because:

- wrong-branch and wrong-snapshot workflows should be explicit, not hand-built

Should provide:

- restore wrong snapshot onto active branch
- restore stale checkpoint after newer checkpoint
- query replay around missing or wrong anchor
- branch-head mismatch creation where valid

### H6. Tier Policy World Profiles

Needed because:

- fintech currently does not express mixed scheduling policy well

Should provide:

- live-risk tier profile
- audit/replay tier profile
- explicit node tier assignment helpers
- comparator-by-tier helpers

### H7. Keyed Scenario and Cache Verbs

Needed because:

- keyed families and structural memoization are otherwise untested in a world-shaped context

Should provide:

- scenario-pack family creation
- keyed derived artifact lookups
- branch-safe keyed reuse checks
- rollback-safe keyed mutation checks

### H8. Artifact Materialization Helpers

Needed because:

- retained vs reconstructed artifact expectations should be asserted explicitly

Should provide:

- runtime policy selection for `development`, `forensic`, and `kernel`
- expected artifact availability helpers
- retained-vs-reconstructed comparison probes

### H9. Event Subscriber Support

Needed because:

- event ordering and rollback are part of the real signal contract

Should provide:

- domain-shaped alert subscriber
- domain-shaped audit publication subscriber
- deterministic capture of flush order

### H10. Certification Row Naming Rules

Needed because:

- this suite will get large quickly

Naming rule should freeze:

- workflow family IDs
- artifact aliases
- invariant IDs
- failure bundle labels
- profile labels

Without this, the suite will drift into ad hoc naming noise.

---

## Recommended Build Order

These are the highest-leverage next moves.

### Phase 1. Truth and Comparison Layer

Build first:

- H1 case truth capture
- H2 case comparison
- H10 naming rules

Reason:

- nearly every missing workflow depends on them

### Phase 2. Invalid and Locality Pressure

Build next:

- H4 partition/locality verbs
- H5 recovery error injection verbs

Then add:

- W13 partition-local bucket
- W14 changed-region
- W15 wrong-branch / wrong-snapshot
- W16 invalid transition
- W20 coarse-vs-fine truth trap

### Phase 3. Condition and Tier Expansion

Build next:

- H3 condition pressure verbs
- H6 tier policy world profiles

Then add:

- W8 aspect filter condition
- W9 on-demand
- W10 debounce
- W11 custom condition
- W17 tiered live-vs-audit

### Phase 4. Keyed and Artifact Materialization

Build next:

- H7 keyed scenario/cache verbs
- H8 artifact materialization helpers
- H9 event subscriber support

Then add:

- W18 keyed scenario pack
- W19 artifact materialization
- W21 event ordering

---

## Exit Criteria

The fintech domain is not “done” when it has many tests.

It is done when:

- every API family that fintech can honestly express has at least one hostile workflow
- every workflow names a primary bug class
- coarse and fine truth probes are both used where needed
- serial-vs-parallel overlap remains explicit and capability-honest
- invalid paths are certified, not just successful ones
- remaining uncovered cells are explicitly documented as requiring another domain rather than silently ignored

Until then, new workflow additions should be justified by this matrix instead of added ad hoc.
