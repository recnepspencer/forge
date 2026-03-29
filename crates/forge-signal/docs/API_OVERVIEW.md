# API Overview

This is the map of the public surface.

There are two normal entry paths:

- `forge_signal::easy` for the shortest path
- `forge_signal::facade::*` for the broader runtime surface

They are not two different systems.
`easy` is a simpler way into the same runtime story.

If you only remember five areas, remember these:

- easy
- graph
- runtime
- transaction
- diagnostics

The intended import path is:

```rust
use forge_signal::facade::*;
```

If you want the shortest path in, start with `forge_signal::easy::*`.

The short-path names to remember are:

- `SignalApp`
- `InputSignal<T>`
- `ComputedSignal<T>`
- `SignalContext`

## Read This With The Docs Tree

If you are learning the library in order:

- `GETTING_STARTED.md`
- `core-concepts/`
- `guides/`
- `reference/`

## Core

This is the graph surface you use all the time.

- `SignalGraph`
- `NodeBuilder`
- `DependencyEdge`
- `mark_changed(...)`
- `mark_changed_with_regions(...)`
- `mark_dirty_batch(...)`

If you are in `easy`, this lower layer is still what the system runs on top of.
You just do not have to start here.

## Runtime

This is the main runtime surface.

- `SignalRuntime`
- `SignalRuntime::build_for::<Ctx>(...)`
- `SignalRuntime::operational_for::<Ctx>(...)`
- `SignalRuntime::forensic_for::<Ctx>(...)`
- `RuntimePolicy`
- `SignalTransaction`
- `BatchChange`
- `BatchChangeResult`
- `Recipe`
- `History`
- `RuntimeMerge`

If you want one stable computation shape instead of rebuilding the same thing by
hand, use:

- `runtime.define(Recipe::new(...))`
- `.keyed(...)`
- `.run(...)`
- `.read(...)`

If you started in `easy`, this is the surface you grow into.

## Diagnostics

This is where you go when the runtime does something you did not expect.

- `runtime.diagnostics()`
- `diagnostics_for_graph(...)`
- `diagnostics_for_runtime(...)`

Start with:

- `why(node)`
- `explain(node)`
- `health_now()`
- `health_view().current_now()`
- `inspect().graph()`
- `compare().reports(...)`

## Advanced

Use this when you are taking deliberate control, not when you are still trying
to get the system working.

- `EvaluationPlan`
- `RunMode`
- `StageExecutor`
- `ReadView`
- `PlannedRun`
- `ComparatorPolicy`

If you need to tune advanced behavior, look for the grouped edit points first:

- `builder.adjust_runtime_policy(...)`
- `builder.adjust_checkpoints(...)`
- `builder.adjust_fallback_comparator(...)`
- `runtime.adjust_runtime_policy(...)`
- `runtime.adjust_checkpoint_policy(...)`
- `runtime.adjust_tier_policy(...)`
- `runtime.adjust_fallback_comparator(...)`

## Integration

This is for bridge authors and deeper integration work.

- event subscribers
- effect mapping
- checkpoint evaluators
- deeper merge and proof-heavy forms

It matters. It just is not where most users should start.

## History

History is exposed through the runtime:

- `runtime.history()`
- `runtime.merge()`
- `runtime.target(node)`
- `tx.target(node)`

Do not start by memorizing lineage internals.

## Policy shape

The short version:

- choose a runtime preset first
- refine `RuntimePolicy` only when you need more control
- treat `reset_runtime_policy_to_tier(...)` as the stock preset reset, not the
  main advanced owner
