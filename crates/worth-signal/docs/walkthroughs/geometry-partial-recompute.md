# Geometry Partial Recompute

This walkthrough shows the region-aware case:

- one wing panel changes
- the wing updates
- the tail does not
- diagnostics keep the changed region visible

The runnable example is:

- [`../../examples/geometry_partial_recompute.rs`](../../examples/geometry_partial_recompute.rs)

## The Dependency Shape

```rust
use worth_signal::facade::*;

const AIRFRAME: Aspect = Aspect::new(0);

let mut graph = SignalGraph::new();
let airframe = graph.node().partitioned_output().build();
let wing_skin = graph.node().on_demand().build();
let tail_skin = graph.node().on_demand().build();

graph.set_dependencies(
    wing_skin,
    [DependencyEdge::with_partition_scope(
        airframe,
        AIRFRAME,
        PartitionSubscription::whole_partition("wing"),
    )],
)?;
graph.set_dependencies(
    tail_skin,
    [DependencyEdge::with_partition_scope(
        airframe,
        AIRFRAME,
        PartitionSubscription::whole_partition("tail"),
    )],
)?;
# Ok::<(), SignalError>(())
```

This is the key move:

- wing work subscribes to wing changes
- tail work subscribes to tail changes

That is how one local edit stays local.

## Mark A Region Change

```rust
runtime.transaction(&mut state, |tx| {
    tx.mark_changed_with_regions(airframe, AIRFRAME, &[ChangedRegion::new("wing")])?;
    tx.read_many(&[wing_skin, tail_skin], &evaluate)?;
    Ok(())
})?;
```

That one line is doing real work.
It tells the runtime this was not a full-airframe blast radius.
It was a wing-only change.

## Check The Result

```rust
let versions = runtime.read_many(&[wing_skin, tail_skin], &state, &evaluate)?;
assert_eq!(versions[0].get(WING_SKIN), 101);
assert_eq!(versions[1].get(TAIL_SKIN), 200);
```

Wing moved.
Tail stayed put.

That is the kind of selective recompute people actually care about.

## Ask For The Explanation

```rust
let explanation = runtime.diagnostics().explain(wing_skin)?;
let rendered = format!("{explanation}");
assert!(rendered.contains("wing"));
```

The changed region is not lost after recompute.
The runtime can still tell you what part moved.

Read next:

- [../core-concepts/aspects-and-dependencies.md](../core-concepts/aspects-and-dependencies.md)
- [../guides/defining-computation.md](../guides/defining-computation.md)
