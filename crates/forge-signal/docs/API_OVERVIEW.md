# API Overview

This is the map.

If you want the short version, the everyday center of gravity is:

- graph
- runtime
- transaction
- diagnostics

The intended import path is:

```rust
use forge_signal::facade::*;
```

## Core

This is the everyday graph surface.

- `SignalGraph`
- `NodeBuilder`
- `DependencyEdge`
- `mark_changed(...)`
- `mark_changed_with_regions(...)`
- `mark_dirty_batch(...)`

## Runtime

This is the production runtime surface.

- `SignalRuntime`
- `SignalRuntime::build_for::<Ctx>(...)`
- `SignalRuntime::operational_for::<Ctx>(...)`
- `SignalRuntime::forensic_for::<Ctx>(...)`
- `SignalRuntimePolicy`
- `SignalTransaction`
- `BatchChange`
- `BatchChangeResult`
- `Recipe`
- `RuntimeHistory`
- `RuntimeMerge`

When you want one durable derived definition instead of stitching the same
shape together repeatedly, use:

- `runtime.define(Recipe::new(...))`
- `.keyed(...)`
- `.run(...)`
- `.read(...)`

## Diagnostics

This is where you go when the runtime does something confusing.

- `runtime.diagnostics()`
- `diagnostics_for_graph(...)`
- `diagnostics_for_runtime(...)`

Primary jobs:

- `why(node)`
- `explain(node)`
- `health_now()`
- `compare().reports(...)`

## Advanced

Use this when you are taking deliberate control, not when you are just getting
work done.

- `EvaluationPlan`
- `EvaluationRequestMode`
- `StageExecutor`
- `ExecutionReadView`
- `PreparedEvaluation`
- `VersionComparatorPolicy`

## Integration

This is for bridge authors and specialist integration work.

- event subscribers
- effect mapping
- checkpoint evaluators
- specialist merge and proof-heavy forms

It matters. It just is not where most users should start.

## History

History is exposed through the runtime in friendlier terms:

- `runtime.history()`
- `runtime.merge()`
- `runtime.target(node)`
- `tx.target(node)`

You should not have to memorize lineage internals on day one.
