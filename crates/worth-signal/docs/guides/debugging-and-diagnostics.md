# Debugging And Diagnostics

Use this when something reruns when it should not, stays slow when it should
not, or looks different than expected.

Read these first if you need them:

- [../core-concepts/diagnostics-and-history.md](../core-concepts/diagnostics-and-history.md)
- [runtime-policy.md](./runtime-policy.md)
- [snapshots-branches-and-history.md](./snapshots-branches-and-history.md)

Start here:

```rust
let diagnostics = runtime.diagnostics();
```

Two grouped doors inside diagnostics are:

- `diagnostics.health_view()`
- `diagnostics.inspect()`

## Main Jobs

### Explain why this changed

Start with:

- `diagnostics.why(node)`
- `diagnostics.explain(node)`

`why(...)` is the normal first move.

### Check runtime health

Use:

- `diagnostics.health_now()`
- `diagnostics.health(profile)`
- `diagnostics.health_view().current_now()`
- `diagnostics.health_view().latest_flow()`
- `diagnostics.health_view().recent_history()`

Use this when the problem is broad instead of node-specific.

### Compare runs or summaries

When you need side-by-side comparison, start here:

- `diagnostics.compare().graphs(...)`
- `diagnostics.compare().flows(...)`
- `diagnostics.compare().reports(...)`
- `diagnostics.compare().histories(...)`

The flatter free functions still exist underneath. Start with the grouped entry
first.

### Inspect runtime state without dropping to raw helpers

Use:

- `diagnostics.inspect().graph()`
- `diagnostics.inspect().execution()`
- `diagnostics.inspect().plan(...)`
- `diagnostics.inspect().report(...)`

### When to jump to history

Use:

- `runtime.history()`
- `history.replay_for_branch(...)`
- `history.replay_for_node(...)`
- `history.lineage_for_node(...)`

Use history when you need the sequence of changes, not just the current answer.

## Practical Rule

The normal shape should be:

- `runtime.diagnostics()` for why, compare, health, and inspection
- `runtime.history()` for read-only replay, branch, and lineage inspection
- admitted runtime snapshot operations for capture and restore

Only drop into the flatter diagnostics module when you are doing tooling or
custom reporting.

In Worth Signal, debugging is part of running the system.
It is not something you bolt on later after the graph gets hard to reason
about.

That matters because "it reran" is not a side question here.
The runtime is expected to answer it.

## Profiles

The main diagnostics profiles are:

- `RuntimePolicy::operational()`
- `RuntimePolicy::development()`
- `RuntimePolicy::forensic()`

If you are unsure, start with `development()`.

If you need to jump back to one of those stock presets later, use
`reset_runtime_policy_to_tier(...)`.

If you already tuned custom retention or replay behavior, use
`set_runtime_policy(...)` instead so you keep the full bundle you meant.
