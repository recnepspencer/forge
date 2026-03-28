# History And Replay

Use this when "what is true right now?" is not enough.

This is for:

- snapshots
- replay
- branch inspection
- seeing how something changed over time

## Main surfaces

- `runtime.history()`
- `history.snapshot()`
- `history.branch_snapshot(...)`
- `history.branches()`
- `history.replay_for_branch(...)`
- `history.replay_for_node(...)`
- `history.lineage_for_node(...)`
- `runtime.merge()`
- `runtime.merge().from(...).into(...).plan()?`
- `planned.execute()?`

## What history is for

`runtime.history()` is the friendly starting point over the lower-level lineage
surface.

## Example

```rust
use forge_signal::facade::*;

let mut runtime = SignalRuntime::build_for::<()>(SignalGraph::new());

let snapshot = runtime.history().snapshot();
let branches = runtime.history().branches();

let _ = (snapshot, branches);
```

Specialist merge flow:

```rust
# use forge_signal::facade::*;
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

## Replay vs lineage

- replay answers what happened in execution history
- lineage answers how an artifact changed over time

If you are starting from a runtime question rather than a data-model question,
begin with `runtime.history()`.

Concrete examples:

- "What happened during that bad import job?"
- "What changed on this branch after the refactor?"
- "Why does this artifact look different now than it did yesterday?"
