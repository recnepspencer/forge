# forge-signal

`forge-signal` is a deterministic incremental runtime for derived work.

Your app owns the real state. `forge-signal` owns:

- dependency tracking
- invalidation
- recompute
- rollback
- diagnostics

The main import path is:

```rust
use forge_signal::facade::*;
```

Most days, the shape is simple:

- build a `SignalGraph`
- build a `SignalRuntime`
- mark changes in a transaction
- read the derived node you care about
- use diagnostics when something smells off

## What it is good at

- web backends and reactive views
- finance and risk pipelines
- ML feature and scoring flows
- geometry or compiler-style partial recompute

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
- [Quickstart](./docs/QUICKSTART.md)
- [Daily workflows](./docs/DAILY_WORKFLOWS.md)
- [Diagnostics](./docs/DIAGNOSTICS.md)

## Examples

- [`examples/web_live_search.rs`](./examples/web_live_search.rs)
- [`examples/finance_risk_refresh.rs`](./examples/finance_risk_refresh.rs)
- [`examples/ml_feature_pipeline.rs`](./examples/ml_feature_pipeline.rs)

## Reality check

This crate is meant to feel clean on the normal path and still have real power
when you need more control.

If you are just getting started, stay in:

- `SignalGraph`
- `SignalRuntime`
- `transaction(...)`
- `runtime.diagnostics()`
