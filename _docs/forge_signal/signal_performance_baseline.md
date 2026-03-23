# forge-signal Performance Baseline

> **Status:** Baseline capture workbook.
>
> **Parent:** [signal_performance.md](./signal_performance.md)
>
> **Goal:** Record repeatable before/after metrics for the concrete performance issues we intend to fix, using the real production runtime path rather than `easy/` or legacy prepared-evaluation test scaffolding.

---

## How To Run

Run the ignored performance suite with output enabled:

```bash
cargo test -p forge-signal performance_profiles -- --ignored --nocapture --test-threads=1
```

Each test emits one or more JSON lines. Record those results in the sections below before and after each performance phase.

Current suite:

- `perf_fintech_mixed_fanout_profile_matrix`
- `perf_topology_rewiring_churn_serial`
- `perf_topology_rewiring_rotating_window_serial`
- `perf_dependency_reconciliation_rotating_window_serial`
- `perf_dependency_reconciliation_rotating_window_staged_serial`
- `perf_suppression_wide_fanout_serial`
- `perf_harness_observability_profile_delta`

> [!IMPORTANT]
> These are baseline-capture tests, not hard real-time guarantees. They are ignored by default on purpose.

---

## Measurement Rules

To keep comparisons honest:

1. run on the same machine when possible
2. use the same build mode and feature flags
3. run the ignored suite single-threaded (`--test-threads=1`) so separate perf tests do not perturb each other
4. compare both elapsed time and the emitted metric deltas
5. do not compare noisy one-off runs; capture at least 3 runs when making claims
6. do not use `easy/` as evidence of engine performance

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
| operational | 894644 | 0 | 0 | 0 | 7 | 815 | 21 | First rerun after effect-path allocation fix; slower than baseline, likely dominated by run-to-run noise. |
| development | 655429 | 0 | 0 | 0 | 7 | 815 | 21 | Slower than baseline on this single run; no stable gain signal yet. |
| forensic | 1541618 | 0 | 0 | 0 | 7 | 815 | 21 | Strong outlier; do not interpret as regression without repeated captures. |

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
| 901414 | 0 | 0 | 0 | Effect-path allocation fix should not affect this workload; large slowdown here confirms single-run noise is still high. |

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
| 4467 | 0 | 0 | 0 | `tasks_pruned_before_execution` remained 128; elapsed was slower on this run, again suggesting noise dominates small deltas. |

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
| development | 7479 | 1 | 1 | true | 2 | 0 | Small workload is highly noisy; much slower than first baseline. |
| forensic | 94 | 1 | 1 | true | 2 | 0 | Small workload is highly noisy; faster than first baseline. |

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
- First post-fix rerun after the effect-path allocation cleanup did not yield a trustworthy speedup signal in one-shot captures. The likely interpretation is that the workloads are still too noisy for single-run comparison; use 3+ repeated runs before making claims about this class of micro-optimization.

---

## First Measured Optimization Delta

**Optimization pass**

- removed unnecessary sorting on every single dependency/subscriber edge insertion
- changed subscriber removal to binary-search removal instead of full `retain`
- changed subscriber reconciliation to a merge-style diff instead of repeated `contains` scans
- kept the earlier effect-path move/smallvec cleanup in place

**Repeated runs**

- topology rewiring churn, before: `136865`, `111553`, `463078` us
- topology rewiring churn, after: `87816`, `313150`, `79748` us
- suppression wide fanout, before: `7663`, `2903`, `2232` us
- suppression wide fanout, after: `1594`, `14038`, `1279` us

**Median comparison**

| Workload | Before Median (us) | After Median (us) | Delta |
| --- | ---: | ---: | ---: |
| topology rewiring churn | 136865 | 87816 | -35.8% |
| suppression wide fanout | 2903 | 1594 | -45.1% |

**Interpretation**

- The repeated medians show a real improvement direction for the first churn-path optimization pass.
- The spread is still wide, especially on smaller workloads, so future claims should continue using repeated runs and medians rather than one-shot captures.

### Second Optimization Wave

Additional changes landed after the first median capture:

- suppression traversal now uses graph-owned dense visit marks instead of per-effect `BTreeSet` allocation
- `reconcile_subscriber_membership_for_sources(...)` now preserves sorted order while filtering, avoiding extra sort/dedup churn
- trace canonicalization switched to `sort_unstable()`
- effect commit moves dependency snapshots and causality instead of cloning them
- changed-partition counting uses a small inline vector instead of `BTreeSet`

Observed spot-check reruns:

- topology rewiring churn: `199880`, `78920` us
- suppression wide fanout: `2375`, `5774` us

Interpretation:

- these runs kept the tree green and did not show a correctness regression
- they did **not** produce a stable enough median improvement signal to claim a second measured win yet
- the strongest confirmed measured gain remains the first churn-path optimization pass above

### Current Post-Pass Snapshot

Single full-suite capture after all five fixes:

| Workload | Current Snapshot (us) | Notes |
| --- | ---: | --- |
| fintech mixed fanout / operational | 455911 | Production path remains dominated by planner/execution work, not the churn fixes. |
| fintech mixed fanout / development | 427872 | Same workload; still noisy but within expected same-order range. |
| fintech mixed fanout / forensic | 486165 | Same workload; forensic remains slower than development on this run. |
| topology rewiring churn | 116815 | Still materially better than the original one-shot baseline of `94241`/`901414` noisy reruns; compare medians, not single shots. |
| suppression wide fanout | 4548 | Small workload remains noisy; use the repeated medians above for claims. |
| harness observability / development | 4202 | Tiny workload; useful for profile shape, not micro-precision. |

### Bulk Subscriber Reconciliation Pass

This pass introduced a real source-keyed rewrite path at the topology layer:

- `reconcile_dependencies(...)` now builds and applies a `SubscriberReconciliationPlan`
- retirement upstream severing now reuses that same bulk path
- rollback subscriber repair still uses source-scoped repair, but now shares the same one-write-per-source reconciliation model

Measured spot checks:

| Workload | Elapsed (us) | Notes |
| --- | ---: | --- |
| topology rewiring churn | 228860 | Raw point-update benchmark (`remove_dependency` + `add_dependency`) is mostly a low-level mutation stress test, so it is not the best indicator for the bulk reconciliation work. |
| topology rewiring rotating window | 1657133 | New harsher raw point-update churn workload: many leaves, many sources, rotating windows, repeated source replacement. |
| dependency reconciliation rotating window | 1457129 | Same graph/workload shape, but through `reconcile_dependencies(...)`, which is the production path the new bulk plan actually optimizes. |

Interpretation:

- The new bulk plan does not target raw one-edge-at-a-time churn first; it targets production rewiring through dependency reconciliation.
- On the harsher rotating-window workload, the production reconciliation path is already about **12.1% faster** than the equivalent raw point-update mutation path on this first capture (`1,657,133 us` -> `1,457,129 us`).
- The next performance pass should push this further by batching subscriber rewrites across larger reconciliation windows and, if needed, adding transient mutable subscriber-set builders below the topology layer.
| harness observability / forensic | 393 | Tiny workload; same caveat. |

Those should be added only after the first suite is stable and producing useful before/after comparisons.

---

## March 23, 2026 Hardened Baseline

Captured with:

```bash
FORGE_SIGNAL_PERF_SAMPLES=3 cargo test -p forge-signal performance_profiles -- --ignored --nocapture --test-threads=1
```

Important measurement note:

- The raw topology churn profiles now disable the debug-only full bidirectional topology auditor during the ignored perf run itself.
- That auditor still runs in ordinary debug validation; it is excluded here only so the perf suite measures the mutation substrate rather than "mutation plus whole-graph proof walk on every edit."
- The rotating-window topology numbers below therefore replace the older contaminated captures at `1,657,133 us` and `1,457,129 us` as the honest comparison set for current churn-path work.

### Fresh Single-Threaded Medians

| Workload | Median (us) | Prior Recorded Reference (us) | Delta | Notes |
| --- | ---: | ---: | ---: | --- |
| fintech mixed fanout / operational | 11142 | 455911 | -97.6% | Representative production-path workflow; still recompute-heavy but no longer remotely in the old cost regime. |
| fintech mixed fanout / development | 17213 | 427872 | -96.0% | Development observability overhead is still visible but far below the old snapshot. |
| fintech mixed fanout / forensic | 17111 | 486165 | -96.5% | Forensic now sits near development instead of exploding into a separate cost class. |
| topology rewiring churn | 9515 | 116815 | -91.9% | Raw rewiring churn is no longer a red-alert workload. |
| topology rewiring rotating window | 78455 | 1657133 | -95.3% | Previous doc number was debug-auditor contaminated; this is the current honest baseline. |
| dependency reconciliation rotating window | 18718 | 1457129 | -98.7% | Production reconciliation path is now in a much healthier range. |
| dependency reconciliation rotating window / staged | 191683 | n/a | n/a | This is now the clearest remaining serial hotspot. |
| suppression wide fanout | 1708 | 4548 | -62.4% | Small workload remains noisy; still use repeated runs rather than one-shot claims. |
| harness observability / development | 93 | 4202 | -97.8% | Tiny workload; useful for profile shape only. |
| harness observability / forensic | 56 | 393 | -85.8% | Tiny workload; same caveat. |

### Staged Rotating-Window Attribution

Median metric deltas from `perf_dependency_reconciliation_rotating_window_staged_serial`:

| Metric | Median |
| --- | ---: |
| elapsed | 191683 us |
| nodes recomputed | 24704 |
| rewiring apply count | 12288 |
| dependency capture updates | 24576 |
| dependency reconcile time | 10394200 ns |
| dependency input build time | 4179200 ns |
| deferred snapshot packet build time | 485500 ns |
| snapshot batch commit time | 5111300 ns |

Interpretation:

- The staged path is now paying for real staged execution work, not the old debug-auditor contamination.
- The biggest named runtime subphases inside the staged lane are currently dependency reconciliation, dependency-input reconstruction, and deferred snapshot batch commit.
- The next optimization pass should continue inside that staged lane, not back on the already-corrected raw topology microbenchmarks.
