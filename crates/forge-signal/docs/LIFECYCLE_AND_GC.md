# Lifecycle And GC

Most graph docs stop at "create node, add dependency, done forever."

Real systems are messier. Nodes go away. Editor objects disappear. Cached keyed entries expire. Temporary graph structure accumulates garbage.

`forge-signal` exposes lifecycle controls for that reality.

## Main surfaces

- `SignalGraph::unregister_node(...)`
- `SignalGraph::run_gc_epoch()`
- `SignalGraph::should_gc()`
- `SignalGraph::tombstone_count()`
- `SignalGraph::gc_threshold()`

These live in the lifecycle surface of the graph, not in the planner.

## What unregistering means

`unregister_node(...)` removes a node from the live graph surface and turns it into reclaimable structure.

That matters when:

- a keyed cache entry expires
- an editor entity is deleted
- a temporary derived node family shrinks again

## What GC means here

GC does not mean "the runtime has a secret tracing collector."

It means:

- tombstoned graph structure can be reclaimed in a controlled epoch
- you can observe when cleanup pressure is growing
- long-lived applications can avoid unbounded structural debris

## Example: remove a node and reclaim later

```rust
use forge_signal::facade::*;

let mut graph = SignalGraph::new();
let temporary = graph.node().build();

graph.unregister_node(temporary)?;

if graph.should_gc() {
    graph.run_gc_epoch();
}
# Ok::<(), SignalError>(())
```

## Practical guidance

Use this deliberately in:

- editors
- caches with churn
- long-running services with dynamic keyed nodes

If your graph is mostly static, you probably do not need to think about this every day.
