# Your First Topology Intent

## What This Feature Is

This feature gives you the smallest published lane for building topology truth
intents in `worth-schema`.

The main surfaces are:

- `TopologyCreateBatchBuilder`
- `created_ref(...)`
- `RawTopologyIntent`
- `TopologyMutation`
- `TopologyMutationBatch`

## Why You Use It

Use this when you want to author a small batch of topology creates and
relations without hand-assembling every mutation record yourself.

This is the right surface when you are saying:

- "I need a real topology intent for a test or fixture"
- "I need same-batch symbolic references between created entities"
- "I need a clean authored topology input for a later runtime lane"

## Stable Entry Points

From [facade.rs](C:/Users/Esther/Documents/Programming/forge_workspace/worktree_2/crates/worth-schema/src/facade.rs:1):

- `topology_authoring::TopologyCreateBatchBuilder`
- `topology_authoring::created_ref(...)`
- `RawTopologyIntent`
- `TopologyMutation`
- `TopologyMutationBatch`

## Core Mental Model

`TopologyCreateBatchBuilder` helps you describe one authored batch.

`created_ref(...)` lets one new record point at another new record in the same
batch by create key instead of by live entity id.

`finish(...)` lowers the builder into a `RawTopologyIntent`.

## How It Executes

The authoring flow is:

1. declare topology entities
2. declare relations between them
3. optionally attach persistent-name truth with `persistent_name_for(...)`
4. finish into `RawTopologyIntent`
5. hand the authored input to a Query-backed runtime lane

That keeps authoring explicit while making same-batch graph construction
pleasant.

## Small Example

```rust
use worth_schema::facade::{
    topology_authoring::{created_ref, TopologyCreateBatchBuilder},
};
use worth_schema::facade::platform::authority::MutationOrigin;
use worth_schema::facade::platform::entities::{EntityKind, TopologyEntityKind};
use worth_schema::facade::platform::relations::{RelationKind, TopologyRelationKind};

let intent = TopologyCreateBatchBuilder::new()
    .topology_entity("example.face", EntityKind::Topology(TopologyEntityKind::Face))
    .topology_entity("example.loop", EntityKind::Topology(TopologyEntityKind::Loop))
    .relation(
        "example.face.outer_loop",
        RelationKind::Topology(TopologyRelationKind::FaceOuterLoop),
        created_ref("example.face"),
        created_ref("example.loop"),
    )
    .finish(MutationOrigin::LocalEdit);
```

## Real Example

```rust
use worth_schema::facade::{
    topology_authoring::{created_ref, TopologyCreateBatchBuilder},
};
use worth_schema::facade::platform::authority::MutationOrigin;
use worth_schema::facade::platform::entities::{EntityKind, TopologyEntityKind};
use worth_schema::facade::platform::relations::{RelationKind, TopologyRelationKind};

let intent = TopologyCreateBatchBuilder::new()
    .topology_entity("example.vertex", EntityKind::Topology(TopologyEntityKind::Vertex))
    .topology_entity("example.half_edge", EntityKind::Topology(TopologyEntityKind::HalfEdge))
    .relation(
        "example.half_edge.start_vertex",
        RelationKind::Topology(TopologyRelationKind::HalfEdgeStartsAtVertex),
        created_ref("example.half_edge"),
        created_ref("example.vertex"),
    )
    .persistent_name_for("example.vertex")
    .finish(MutationOrigin::Seed);
```

## How It Relates To Other Features

- Use [Bootstrap Schema Registry](../schema-registry/bootstrap-schema-registry.md)
  before this when you need a runtime loaded with Worth schema kinds.
- Use [Query Aspect Paths](../query-vocabulary/query-aspect-paths.md) when you
  need to name touched truth after authoring succeeds.
- Use `forge-query` for runtime-facing support, inspection, and recovery.

## Inspection And Debugging

If the authored intent looks wrong:

- verify that same-batch links use `created_ref(...)`
- confirm your entity and relation kinds match the topology shape you intend
- inspect the `MutationOrigin`

If you need a quick known-good shape, reach for `seed_minimal_topology(...)`
or the milestone-one primitive corpus in the same namespace.

## Anti-Patterns

- Do not hand-build same-batch entity references as fake live ids.
- Do not treat these helpers as the replacement for Query runtime entry.
- Do not use this lane to keep old schema-owned runtime support logic alive.

## Current Limits

- This page covers the smallest topology authoring lane, not every helper in
  the primitive corpus.
- runtime execution lives outside `worth-schema`

## Related Docs

- [Topology Authoring](./README.md)
- [Bootstrap Schema Registry](../schema-registry/bootstrap-schema-registry.md)
- [Moved Runtime Surfaces To forge-query](../migration/moved-runtime-surfaces-to-forge-query.md)
