# Diagnostics

Diagnostics is not side tooling in `forge-signal`.

It is part of the product. If something recomputes when it should not, or stays
slow when it should not, this is where you go.

Start here:

```rust
let diagnostics = runtime.diagnostics();
```

## Main jobs

### Explain why this changed

Start with:

- `diagnostics.why(node)`
- `diagnostics.explain(node)`

`why(...)` is the normal first move.

Use it for questions like:

- "Why did this node run again?"
- "Why did changing one source fan out this far?"
- "Why is this output different than last time?"

### Check runtime health

Use:

- `diagnostics.health_now()`
- `diagnostics.health(profile)`

Use this when the problem is broad instead of node-specific.

Example:

- "Is this runtime carrying too much history?"
- "Are we retaining too much detail for this workload?"
- "Did we accidentally turn on a heavier profile than we meant to?"

### Compare runs or summaries

When you need side-by-side comparison, start here:

- `diagnostics.compare().graphs(...)`
- `diagnostics.compare().flows(...)`
- `diagnostics.compare().reports(...)`
- `diagnostics.compare().histories(...)`

The flatter free functions still exist underneath, but the guided comparison
entry is the better place to start.

### Inspect replay, history, and lineage

Use:

- `runtime.history()`
- `history.replay_for_branch(...)`
- `history.replay_for_node(...)`
- `history.lineage_for_node(...)`

Example:

- replay a bad batch update
- inspect what happened on one branch
- trace how one artifact changed over time

## Practical rule

Start with the guided diagnostics object.

Only drop into the flatter diagnostics module when you are doing specialist
comparison, reporting, or tooling work.

## Profiles

The main diagnostics profiles are:

- `SignalRuntimePolicy::operational()`
- `SignalRuntimePolicy::development()`
- `SignalRuntimePolicy::forensic()`

In practice:

- `operational()` keeps overhead lower
- `development()` is the best default while building
- `forensic()` keeps more detail when you need to really dig in

If you are unsure, start with `development()`.
