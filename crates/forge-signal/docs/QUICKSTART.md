# Quickstart

This is the fast path.

The example below is the kind of thing you actually do in a real app:

- build a graph
- tell the runtime that some source data changed
- ask for a derived result
- let the runtime do the minimum work needed

## The mental model

Most days, you work with four things:

- `SignalGraph`
- `SignalRuntime`
- `transaction(...)`
- `runtime.diagnostics()`

`SignalGraph` is the dependency map.

`SignalRuntime` runs the graph, keeps history, and gives you diagnostics when
something looks off.

## Small example

```rust
use forge_signal::facade::*;

const PRICE: Aspect = Aspect::new(0);

let mut graph = SignalGraph::new();
let source = graph.node().build();
let total = graph.node().on_demand().build();

graph.set_dependencies(total, [DependencyEdge::new(source, PRICE)])?;

let mut runtime = SignalRuntime::build_for::<()>(graph);

runtime.transaction(&mut (), |tx| {
    tx.mark_changed(source, PRICE)?;
    tx.target(total).run(&|view| {
        let result = if view.node() == source {
            view.finish(NodeEvaluationResult::from_version(
                AspectVersion::from_updates([(PRICE, 1)]),
            ))
        } else {
            let version = view.read_aspect_version(source, PRICE)?;
            view.finish(NodeEvaluationResult::from_version(version))
        };
        Ok(result)
    })?;
    Ok(())
})?;

let version = runtime.target(total).read(&(), &|view| {
    let source_version = view.read_aspect_version(source, PRICE)?;
    Ok(view.finish(NodeEvaluationResult::from_version(source_version)))
})?;

let _ = version;
# Ok::<(), SignalError>(())
```

What happened here:

- `source` is a raw input node
- `total` is a derived node
- `tx.mark_changed(...)` tells the runtime the source changed
- `tx.target(total).run(...)` computes what needs to be computed
- `runtime.target(total).read(...)` asks for the current derived result

If you are building a web app, this is the same basic shape as:

- request data changes
- mark the affected source nodes
- read the derived view model or cache entry you care about

## What to learn next

- If you want the big picture, read [API_OVERVIEW.md](./API_OVERVIEW.md).
- If you want the normal everyday jobs, read [DAILY_WORKFLOWS.md](./DAILY_WORKFLOWS.md).
- If you want to debug why work happened, read [DIAGNOSTICS.md](./DIAGNOSTICS.md).
- If you want to tune execution, read [PARALLEL_EXECUTION.md](./PARALLEL_EXECUTION.md).
