# forge-signal

`forge-signal` is a deterministic incremental runtime for derived work.

Your app owns the real state.
`forge-signal` owns:

- dependency tracking
- invalidation
- recompute
- rollback
- diagnostics
- replay and history

This crate is not just trying to rerun less work.
It is trying to keep updates, transactional truth, explanation, and history in
one system.

There are two normal entry paths:

```rust
use forge_signal::easy::*;
use forge_signal::facade::*;
```

Use `easy` for the shortest path.
Use `facade` when you want the broader runtime surface from the start.

## What Makes It Different

The important line is this:

- not "reactive graph plus some debug helpers"
- not "incremental cache plus a separate audit layer"
- not "rerun less work and figure out the rest later"

Forge Signal keeps change propagation, transactions, diagnostics, and history
in the same runtime.

That means:

- updates should land as one unit
- rollback should leave the runtime in a sane state
- diagnostics should explain why work happened
- replay and history should keep the trail

## Fast Mental Model

Most days, the shape is:

- build a `SignalGraph`
- build a `SignalRuntime`
- mark changes in a transaction
- read the derived node you care about
- use diagnostics when something smells off

If you start in `easy`, that is still the same system.
You are not signing up for a toy path you need to throw away later.

## Where It Fits

- web backends and reactive views
- finance and risk pipelines
- ML feature and scoring flows
- geometry or compiler-style partial recompute

## One Continuous Story

The flagship story looks like this:

- a source file changes
- a transaction lands the update
- only the right downstream targets rerun
- diagnostics explain why the bundle moved
- replay keeps the trail

That full version lives here:

- [Compiler targeted rebuild walkthrough](./docs/walkthroughs/compiler-targeted-rebuild.md)

## Small example

```rust
use forge_signal::facade::*;

const PRICE: Aspect = Aspect::new(0);
const TOTAL: Aspect = Aspect::new(1);

#[derive(Default)]
struct CheckoutState {
    price_version: u64,
    total_version: u64,
}

let mut graph = SignalGraph::new();
let price = graph.node().build();
let total = graph.node().on_demand().build();

graph.set_dependencies(total, [DependencyEdge::new(price, PRICE)])?;

let mut runtime = SignalRuntime::build_for::<CheckoutState>(graph);

let mut state = CheckoutState {
    price_version: 2,
    total_version: 5,
};

let evaluate = |view: &mut EvaluationContext<'_, CheckoutState>| {
    let result = if view.node() == price {
        view.finish(NodeEvaluationResult::from_version(
            AspectVersion::from_updates([(PRICE, view.domain().price_version)]),
        ))
    } else {
        let _upstream = view.read_aspect_version(price, PRICE)?;
        view.finish(NodeEvaluationResult::from_version(
            AspectVersion::from_updates([(TOTAL, view.domain().total_version)]),
        ))
    };
    Ok::<_, SignalError>(result)
};

runtime.transaction(&mut state, |tx| {
    tx.mark_changed(price, PRICE)?;
    tx.target(total).read(&evaluate)?;
    Ok(())
})?;

let version = runtime.target(total).read(&state, &evaluate)?;
assert_eq!(version.get(TOTAL), 5);
# Ok::<(), SignalError>(())
```

## Start here

- [Docs index](./docs/README.md)
- [Getting started](./docs/GETTING_STARTED.md)
- [API overview](./docs/API_OVERVIEW.md)
- [Compiler targeted rebuild walkthrough](./docs/walkthroughs/compiler-targeted-rebuild.md)
- [Running the runtime](./docs/guides/running-the-runtime.md)
- [Debugging and diagnostics](./docs/guides/debugging-and-diagnostics.md)

## Examples

- [`examples/easy_task_board.rs`](./examples/easy_task_board.rs) for the short path
- [`examples/compiler_targeted_rebuild.rs`](./examples/compiler_targeted_rebuild.rs) for targeted rebuilds, diagnostics, and replay
- [`examples/geometry_partial_recompute.rs`](./examples/geometry_partial_recompute.rs) for region-aware invalidation

## Walkthroughs

- [Easy task board](./docs/walkthroughs/easy-task-board.md)
- [Compiler targeted rebuild](./docs/walkthroughs/compiler-targeted-rebuild.md)
- [Geometry partial recompute](./docs/walkthroughs/geometry-partial-recompute.md)

## Reality check

If you are just getting started, stay in:

- `SignalGraph`
- `SignalRuntime`
- `transaction(...)`
- `runtime.diagnostics()`

Or start in `easy` and move out only when you need more room.
