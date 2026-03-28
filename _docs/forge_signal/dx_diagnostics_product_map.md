# Forge Signal Diagnostics Product Map

## Purpose

Diagnostics should be productized around user jobs, not around flat export
families.

---

## Core Jobs

The diagnostics product should be organized around:

- explain why this changed
- compare two runs
- inspect runtime health
- trace replay / lineage / history

---

## Canonical Entry Shape

Diagnostics should be entered through one obvious access point.

Target direction:

```rust
let diagnostics = runtime.diagnostics();
```

or the graph equivalent for graph-only usage.

Users should not need to begin by memorizing free-floating helper families.

---

## Job 1: Explain Why This Changed

Primary output:

- explanation object
- optional render / summary view

What should be discoverable first:

- `explain(node)`

What should be secondary:

- raw fact and provenance types
- raw renderer entry points

---

## Job 2: Compare Two Runs

Primary output:

- comparison result with summary-first access

What should be discoverable first:

- compare reports / plans / explanations / histories through one comparison area

What should be secondary:

- raw diff types

---

## Job 3: Inspect Runtime Health

Primary output:

- health-oriented summaries:
  - planning summary
  - precompute summary
  - apply / rollback / failure summaries

What should be discoverable first:

- summary and health entry points

What should be secondary:

- low-level counters and internal diagnostic materialization details

---

## Job 4: Trace Replay / Lineage / History

Primary output:

- history-oriented access point
- replay slices
- lineage chains
- restore-related historical views

What should be discoverable first:

- history / replay / lineage as a coherent family

What should be secondary:

- raw event and lineage record internals

---

## Product Rule

Diagnostics should feel like:

- one premium subsystem
- a few crisp user jobs
- progressively disclosed machinery

Not:

- a taxonomy of helper families

---

## Design Requirement

The user should not need to first understand:

- compare functions
- diff functions
- render functions
- summary structs
- lineage record internals

Instead, the public diagnostics experience should lead with the jobs above and
progressively disclose the underlying machinery.
