# forge-signal Performance Baseline

> **Status:** Baseline capture workbook.
>
> **Parent:** [signal_performance_architecture.md](./signal_performance_architecture.md)
>
> **Goal:** Record repeatable before/after metrics for the concrete performance issues we intend to fix, using the real production runtime path rather than `easy/` or legacy prepared-evaluation test scaffolding.

---

## How To Run

Run the ignored performance suite with output enabled:

```bash
cargo test -p forge-signal performance_profiles -- --ignored --nocapture
```

Each test emits one or more JSON lines. Record those results in the sections below before and after each performance phase.

Current suite:

- `perf_fintech_mixed_fanout_profile_matrix`
- `perf_topology_rewiring_churn_serial`
- `perf_suppression_wide_fanout_serial`
- `perf_harness_observability_profile_delta`

> [!IMPORTANT]
> These are baseline-capture tests, not hard real-time guarantees. They are ignored by default on purpose.

---

## Measurement Rules

To keep comparisons honest:

1. run on the same machine when possible
2. use the same build mode and feature flags
3. do not compare noisy one-off runs; capture at least 3 runs when making claims
4. compare both elapsed time and the emitted metric deltas
5. do not use `easy/` as evidence of engine performance

---

## Workload 1 — Fintech Mixed Fanout

**Purpose**

Targets:

- `P1` hot-path isolation
- `P5` observability cost by profile
- realistic mixed-fanout reads on the production evaluator path

**Source**

- fintech world fixture
- fanout scale
- serial executor
- runtime policy matrix: operational / development / forensic

### Baseline

| Profile | Elapsed (us) | Eval Calls | Nodes Evaluated | Nodes Recomputed | Plans Built | Tasks Scheduled | Stage Exec Count | Notes |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| operational | 397103 | 0 | 0 | 0 | 7 | 815 | 21 | First baseline from `performance_profiles`; 4 tasks pruned before execution. |
| development | 350715 | 0 | 0 | 0 | 7 | 815 | 21 | Same workload/profile family; 4 tasks pruned before execution. |
| forensic | 363392 | 0 | 0 | 0 | 7 | 815 | 21 | Same workload/profile family; 4 tasks pruned before execution. |

### After P1/P5

| Profile | Elapsed (us) | Eval Calls | Nodes Evaluated | Nodes Recomputed | Plans Built | Tasks Scheduled | Stage Exec Count | Notes |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| operational | | | | | | | | |
| development | | | | | | | | |
| forensic | | | | | | | | |

---

## Workload 2 — Topology Rewiring Churn

**Purpose**

Targets:

- `P3` topology mutation discipline
- especially batched subscriber reconciliation and clone/reinsert pressure

**Source**

- synthetic graph mutation workload
- repeated dependency rewiring
- serial mutation path

### Baseline

| Elapsed (us) | Rewiring Apply Count | Dependency Capture Updates | Compaction Count | Notes |
| ---: | ---: | ---: | ---: | --- |
| 94241 | 0 | 0 | 0 | Current workload isolates raw rewiring churn; subscriber/dependency segment rewrite counters are also 0 in the baseline output. |

### After P3

| Elapsed (us) | Rewiring Apply Count | Dependency Capture Updates | Compaction Count | Notes |
| ---: | ---: | ---: | ---: | --- |
| | | | | |

---

## Workload 3 — Suppression Wide Fanout

**Purpose**

Targets:

- `P4.4` suppression propagation scaling
- comparator-driven suppression behavior under wide fanout

**Source**

- one source
- one comparator-sensitive middle node
- many leaves
- serial executor

### Baseline

| Elapsed (us) | Skipped By Comparator | Suppressed Downstream Propagations | Nodes Evaluated | Notes |
| ---: | ---: | ---: | ---: | --- |
| 3234 | 0 | 0 | 0 | Current pipeline manifests the win as `tasks_pruned_before_execution=128` rather than comparator-skip counters. |

### After P4

| Elapsed (us) | Skipped By Comparator | Suppressed Downstream Propagations | Nodes Evaluated | Notes |
| ---: | ---: | ---: | ---: | --- |
| | | | | |

---

## Workload 4 — Harness Observability Profile Delta

**Purpose**

Targets:

- `P5` profile-gated observability
- prove harness is still using production code while different observation profiles impose different cost

**Source**

- harness scenario over production evaluator path
- development vs forensic observation profiles

### Baseline

| Profile | Elapsed (us) | Explanations | Provenance Records | Diagnostics Present | Tasks Executed | Tasks Pruned | Notes |
| --- | ---: | ---: | ---: | --- | ---: | ---: | --- |
| development | 1739 | 1 | 1 | true | 2 | 0 | First harness baseline using production bridge/runtime path. |
| forensic | 136 | 1 | 1 | true | 2 | 0 | Faster than development in this small run; rerun 3x before making any profile-cost claim. |

### After P5

| Profile | Elapsed (us) | Explanations | Provenance Records | Diagnostics Present | Tasks Executed | Tasks Pruned | Notes |
| --- | ---: | ---: | ---: | --- | ---: | ---: | --- |
| development | | | | | | | |
| forensic | | | | | | | |

---

## Open Follow-Ups

These are likely next suites after the first baseline pass:

- effect allocation discipline (`P2.4`)
- hash-first partition matching (`P2.5`)
- scratch retention policy (`P2.6`)
- profile-aware parallel scaling under full parallel executor

---

## Initial Capture Notes

- Captured on March 12, 2026 from:

```bash
cargo test -p forge-signal performance_profiles -- --ignored --nocapture
```

- Normal validation lane at capture time:

```bash
cargo test -p forge-signal --lib --quiet
```

- Result: `304 passed; 0 failed; 11 ignored`
- The suppression workload currently reports its benefit through `tasks_pruned_before_execution`, not `skipped_by_comparator` or `suppressed_downstream_propagations`. Future comparisons for that suite should keep using the same interpretation unless the metric model is intentionally changed.

Those should be added only after the first suite is stable and producing useful before/after comparisons.
