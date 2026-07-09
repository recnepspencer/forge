# Lifecycle And GC

Most graph docs stop at "create node, add dependency, done forever."

Real systems are messier. Nodes go away. Cached keyed entries expire. Temporary
graph structure accumulates garbage.

`worth-signal` exposes lifecycle controls for that reality.

## Main Surfaces

- `SignalGraph::unregister_node(...)`
- `SignalGraph::run_gc_epoch()`
- `SignalGraph::should_gc()`
- `SignalGraph::tombstone_count()`
- `SignalGraph::gc_threshold()`

## Practical Rule

Use this deliberately in:

- editors
- caches with churn
- long-running services with dynamic keyed nodes
