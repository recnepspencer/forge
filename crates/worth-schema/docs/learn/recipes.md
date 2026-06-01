# Recipes

## Pick The Right Surface

- Need to name a truth slice for Query-facing work:
  [Query Aspect Paths](../query-vocabulary/query-aspect-paths.md)
- Need to name a collection or schema basis:
  [Query Collections And Bases](../query-vocabulary/query-collections-and-bases.md)
- Need a registry loaded with Worth schema kinds:
  [Bootstrap Schema Registry](../schema-registry/bootstrap-schema-registry.md)
- Need to build a topology authoring batch:
  [Create Batch Builder](../topology-authoring/create-batch-builder.md)
- Need to understand where topology execution moved:
  [Verification](../topology-authoring/verification.md)
- Need to migrate an old schema-era runtime habit:
  [Moved Runtime Surfaces To forge-query](../migration/moved-runtime-surfaces-to-forge-query.md)

## Recipe: Name One Truth Slice

Use `QueryAspectPath` when you need a stable `aspect.field` name.

```rust
use worth_schema::facade::QueryAspectPath;
use worth_schema::facade::platform::aspects::{Aspect, TopologyAspect};

let aspect = Aspect::Topology(TopologyAspect::Structure);
let path = QueryAspectPath::from_aspect(aspect);

assert_eq!(path.as_str(), "topology.structure");
```

## Recipe: Bootstrap The Schema Registry

Use the published bootstrap entry when you need a real Worth
`RelationalSchemaRegistry`.

```rust
use worth_schema::facade::bootstrap_schema_registry;

let registry = bootstrap_schema_registry()?;
```

## Recipe: Declare The Registry Requirements Explicitly

Use `SchemaBuilder` when you want a small guard surface before full bootstrap.

```rust
use worth_schema::facade::SchemaBuilder;

let registry = SchemaBuilder::new()
    .with_topology_kinds()
    .with_naming_kinds()
    .build()?;
```

## Recipe: Build A Small Topology Intent

Use `TopologyCreateBatchBuilder` and `created_ref(...)` for same-batch authoring.

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

## Recipe: Run A Topology Intent Through Query

Use `worth-schema` to build the truth input, then hand that input to a
Query-owned runtime lane. This is the most important boundary in the crate.

```rust
use worth_schema::facade::{
    topology_authoring::TopologyCreateBatchBuilder,
};
use worth_schema::facade::platform::authority::MutationOrigin;
use worth_schema::facade::platform::entities::{EntityKind, TopologyEntityKind};

let intent = TopologyCreateBatchBuilder::new()
    .topology_entity("example.vertex", EntityKind::Topology(TopologyEntityKind::Vertex))
    .finish(MutationOrigin::Seed);

// Next step: hand `intent` to a Query-backed topology family or helper.
```

## Recipe: Decide Whether This Belongs In Schema Or Query

Use `worth-schema` when the question is:

- what truth is this?
- what is the stable name for this truth?
- what does the schema registry need?
- what topology truth batch do I want to author?

Use `forge-query` when the question is:

- can I run this?
- what declaration should I author?
- what blocked me?
- how do I inspect or recover?

## Related Docs

- [Feature Index](./feature-index.md)
- [Start Here](../start_here.md)
- [Query Vocabulary](../query-vocabulary/README.md)
- [Topology Authoring](../topology-authoring/README.md)
- [Moved Runtime Surfaces To forge-query](../migration/moved-runtime-surfaces-to-forge-query.md)
