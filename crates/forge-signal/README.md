# forge-signal

Deterministic reactive computation runtime for host-managed state graphs.

`forge-signal` gives you:

- dependency DAG scheduling
- aspect-aware invalidation
- lazy recomputation
- conditional nodes
- transactional rollback
- deterministic behavior

It stays domain-free. Your application owns the truth state. `forge-signal` owns invalidation and recomputation.

## Hard Mode

The core API is explicit, but it should read cleanly.

```rust
use forge_signal::facade::*;

const PRICE: Aspect = Aspect::new(0);
const TAX: Aspect = Aspect::new(1);

let mut graph = SignalGraph::new();

let price = graph.node().build();
let total = graph
    .node()
    .depends_on_aspects([PRICE, TAX])
    .on_demand()
    .build();

graph.add_dependency(total, price, PRICE)?;

let mut runtime = SignalRuntime::builder(graph)
    .checkpoint_barrier(CheckpointBarrier::PerOperation)
    .build();

let mut latest_total = 0_u64;
runtime.transaction(&mut (), |transaction| {
    transaction.mark_dirty(price, PRICE)?;

    let mut compute = |_node: NodeId, _graph: &SignalGraph| {
        latest_total += 1;
        Ok(AspectVersion::from_updates([(PRICE, latest_total)]))
    };

    transaction.evaluate(total, &mut compute)?;
    Ok(())
})?;
```

### Core ideas

- `depends_on_aspects(...)` expresses which kinds of changes matter to a node
- `condition(...)` expresses when a node is allowed to run
- condition helpers like `on_demand()`, `debounce(...)`, `aspect_filter(...)`, `delta_threshold(...)`, and `custom_condition(...)` keep common policies readable
- `transaction(...)` gives one linear mutation/evaluation flow
- `graph.node()` replaces raw config-struct-heavy node creation for common cases

### Conditions

| Condition | Meaning |
| --- | --- |
| `EvaluationCondition::Always` | Run whenever the node is dirty |
| `EvaluationCondition::OnDemand` | Run only when explicitly requested |
| `EvaluationCondition::Debounce(ms)` | Run only after the quiet period |
| `EvaluationCondition::AspectFilter(mask)` | Run only when matching aspects changed |
| `EvaluationCondition::DeltaThreshold(value)` | Run only when change crosses the threshold |
| `EvaluationCondition::Custom(key)` | Host decides through a resolver |

## Easy Mode

If you just want reactive values, use the separate `easy` module.

```rust
use forge_signal::easy::*;

let mut graph = ReactiveGraph::new();
let price = graph.input(100.0_f64);
let tax = graph.input(0.08_f64);

let total = graph.computed(|context| {
    context.get(price) * (1.0 + context.get(tax))
});

assert_eq!(graph.get(total), 108.0);

graph.batch(|reactive| {
    reactive.set(price, 200.0);
    reactive.set(tax, 0.10);
});

assert_eq!(graph.get(total), 220.0);
```

Easy mode is a wrapper over the same runtime ideas. It is not a second execution engine.

## Why It Exists

Most reactive systems make invalidation easy but correctness hard.

`forge-signal` is built for workloads where you need:

- explicit mutation boundaries
- rollback on failure
- deterministic behavior
- precise aspect-level change tracking
- a runtime that can later grow into richer planners, memoization, and bridge integration

## Current Focus

The current public surface is centered on:

- `SignalRuntime`
- `SignalRuntime::builder(...)`
- `runtime.transaction(...)`
- `SignalGraph::node()`
- `forge_signal::easy::*`

Lower-level APIs still exist, but these are the intended front doors.
