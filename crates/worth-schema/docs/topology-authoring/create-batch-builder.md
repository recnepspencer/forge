# Create Batch Builder

## What This Feature Is

`TopologyCreateBatchBuilder` is the smallest published helper for authoring one
topology create batch.

It works together with:

- `created_ref(...)`
- `RawTopologyIntent`
- `MutationOrigin`

## Why You Use It

Use this when you want to:

- create topology entities in one batch
- create relations between those new entities
- attach a persistent-name pair with one helper
- lower the result into a `RawTopologyIntent`

## Stable Entry Points

From [facade.rs](C:/Users/Esther/Documents/Programming/forge_workspace/worktree_2/crates/worth-schema/src/facade.rs:1):

- `topology_authoring::TopologyCreateBatchBuilder`
- `topology_authoring::created_ref(...)`
- `RawTopologyIntent`
- `MutationOrigin`

## Core Mental Model

The builder owns one batch of create mutations.

`created_ref(...)` lets one new record point at another new record in the same
batch by create key, not by a live entity id.

`finish(...)` turns the authored batch into a `RawTopologyIntent`.

## How It Executes

The builder gives you three main authoring moves:

1. `topology_entity(...)`
2. `relation(...)`
3. `persistent_name_for(...)`

Then `finish(mutation_origin)` lowers the accumulated mutations into a real
`RawTopologyIntent`.

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

- Use [Verification](./verification.md) once you want to apply the authored
  intent through the Query-owned runtime path.
- Use [Bootstrap Schema Registry](../schema-registry/bootstrap-schema-registry.md)
  before verification when you need a runtime with Worth kinds loaded.

## Inspection And Debugging

If the authored batch is wrong:

- check the create keys you used with `created_ref(...)`
- verify the relation kind matches the source and target shapes
- verify whether `persistent_name_for(...)` is the helper you want instead of
  hand-adding naming records

## Anti-Patterns

- Do not synthesize fake live ids for same-batch references.
- Do not use this builder as a replacement for Query declaration orchestration.
- Do not hand-roll the persistent-name pair when the helper already owns that
  shape.

## Current Limits

- This is a create-batch helper, not a full topology edit DSL.
- It intentionally stops at authored truth input.
- Runtime execution, inspection, and recovery now belong to `forge-query`.

## Related Docs

- [Topology Authoring](./README.md)
- [Your First Topology Intent](./your-first-topology-intent.md)
- [Verification](./verification.md)
