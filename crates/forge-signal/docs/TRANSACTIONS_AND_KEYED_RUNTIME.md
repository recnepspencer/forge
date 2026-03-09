# Transactions And Keyed Runtime

This is the part people forget until they badly need it.

## Transactions

Use a transaction when graph mutation, evaluation, diagnostics, replay, and rollback must move together.

Primary surface:

- `runtime.begin()`
- `runtime.transaction(ctx, |tx| ...)`
- `tx.mark_dirty(...)`
- `tx.mark_dirty_with_regions(...)`
- `tx.evaluate_with_plan(...)`
- `tx.evaluate_with_plan_and_executor(...)`
- `tx.read(...)`
- `tx.commit(...)`
- `tx.rollback(...)`

### Example: staged mutation and read

```rust
use forge_signal::facade::*;

const PRICE: Aspect = Aspect::new(0);

let mut graph = SignalGraph::new();
let source = graph.node().build();
let total = graph.node().on_demand().build();
graph.add_dependency(total, source, PRICE)?;

let mut runtime = SignalRuntime::builder(graph).build();
let mut ctx = ();

runtime.transaction(&mut ctx, |tx| {
    tx.mark_dirty(source, PRICE)?;
    let result = tx.read(total, &|node, view| {
        let prepared = if node == source {
            view.finish(NodeEvaluationResult::from_version(
                AspectVersion::from_updates([(PRICE, 1)]),
            ))
        } else {
            let version = view.read_aspect_version(source, PRICE)?;
            view.finish(NodeEvaluationResult::from_version(version))
        };
        Ok(prepared)
    })?;

    let _ = result;
    Ok(())
})?;
# Ok::<(), SignalError>(())
```

Why use a transaction instead of mutating the graph directly:

- rollback is part of the contract
- replay and diagnostics stay coherent with the graph outcome
- failure paths do not leak partial semantic artifacts

## Keyed nodes and computation families

Keyed runtime surfaces matter when one logical family produces many stable node instances.

Main surfaces:

- `register_computation_family(...)`
- `keyed_node(...)`
- `ComputationFamily`
- `ComputationKey`
- `StructuralMemoKey`
- `PreparedKeyedContext`

Use this when:

- you maintain a family of cached derived values
- you need repeatable node identity per business key
- you want memoization and replay to see stable keys instead of anonymous node churn

### Example: register a family and create a keyed node

```rust
use forge_signal::facade::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum FamilyTier {
    Live,
}

let mut runtime = SignalRuntime::builder(SignalGraph::new())
    .with_tiers::<FamilyTier>()
    .build();

let family = ComputationFamily::new("positions");
runtime.register_computation_family(family);

let keyed = runtime.keyed_node(
    family,
    ComputationKey::from("AAPL"),
)?;

runtime.set_node_tier(keyed, FamilyTier::Live);
# Ok::<(), SignalError>(())
```

## When transactions and keyed nodes belong together

They pair well when:

- you create or touch keyed nodes incrementally
- you need mutation batches to commit atomically
- you want replay and rollback to describe stable business-key-level changes

That combination is common in trading books, editor entity graphs, and long-lived caches.
