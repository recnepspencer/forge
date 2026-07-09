# Snapshots, Branches, And History

Use this when "what is true right now?" is not enough.

Read these first if you need them:

- [../core-concepts/diagnostics-and-history.md](../core-concepts/diagnostics-and-history.md)
- [debugging-and-diagnostics.md](./debugging-and-diagnostics.md)

This guide is for:

- snapshots
- replay
- branch inspection
- seeing how something changed over time

## Main Surfaces

- `runtime.history()`
- `history.snapshot()`
- `history.branch_snapshot(...)`
- `history.branches()`
- `history.replay_for_branch(...)`
- `history.replay_for_node(...)`
- `history.lineage_for_node(...)`
- `runtime.merge()`

## The Main Rule

`runtime.history()` is the starting point. Do not start from the lower-level
history types unless you have a real reason.

## Example

```rust
use worth_signal::facade::*;

let mut runtime = SignalRuntime::build_for::<()>(SignalGraph::new());

let snapshot = runtime.history().snapshot();
let branches = runtime.history().branches();

let _ = (snapshot, branches);
```

## Replay vs lineage

- replay answers what happened over time
- lineage answers how one result changed over time

If you are starting from a runtime question, begin with `runtime.history()`.

Concrete questions:

- "What happened during that bad import job?"
- "What changed on this branch after the refactor?"
- "Why does this result look different now than it did yesterday?"

This is another place where WORTH Signal shows its shape.
The runtime keeps the sequence of changes, not just the latest answer.
If a branch diverges, restores, or rolls back, that should still be visible
after the fact.

## Merge

Guided merge flow:

```rust
# use worth_signal::facade::*;
# let mut runtime = SignalRuntime::build_for::<()>(SignalGraph::new());
# let source = runtime.history().current_branch();
# let target = runtime.history().current_branch();
let planned = runtime.merge()
    .from(source)
    .into(target)
    .plan()?;

let _result = planned.execute()?;
# Ok::<(), SignalError>(())
```
