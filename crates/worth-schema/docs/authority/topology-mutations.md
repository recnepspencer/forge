# Topology Mutations

## What This Feature Is

This feature covers the public write-side topology truth vocabulary in
`worth-schema`.

The main surfaces are:

- `RawTopologyIntent`
- `TopologyMutation`
- `TopologyMutationBatch`
- `MutationOrigin`
- `CreateKey`
- `EntityReference`

## Why You Use It

Use this when you need to describe topology truth changes explicitly.

This is the right surface when you are saying:

- "I need a batch of topology mutations"
- "I need to distinguish created references from live ids"
- "I need a raw topology intent before Query-owned execution"

## Stable Entry Points

From [facade.rs](C:/Users/Esther/Documents/Programming/forge_workspace/worktree_2/crates/worth-schema/src/facade.rs:1):

- `platform::authority::RawTopologyIntent`
- `platform::authority::TopologyMutation`
- `platform::authority::TopologyMutationBatch`
- `platform::authority::MutationOrigin`
- `platform::authority::CreateKey`
- `platform::authority::EntityReference`

## Core Mental Model

`RawTopologyIntent` is the authored write-side batch.

`TopologyMutation` is one topology truth change inside that batch.

`MutationOrigin` records why the batch exists, such as `Seed` or `LocalEdit`.

This is truth-authoring vocabulary, not a runtime support contract.

## How It Executes

There is no orchestration here by itself.

These types become useful when:

1. you author them directly
2. or a helper like `TopologyCreateBatchBuilder` authors them for you
3. then a Query-backed topology runtime lane applies them

## Small Example

```rust
use worth_schema::facade::{
    CreateKey,
    MutationOrigin,
    RawTopologyIntent,
    TopologyMutation,
};
use worth_schema::facade::platform::entities::{EntityKind, TopologyEntityKind};
use worth_schema::facade::platform::authority::{
    CreateKey, MutationOrigin, RawTopologyIntent, TopologyMutation,
};

let intent = RawTopologyIntent::new(
    vec![TopologyMutation::CreateEntity {
        create_key: CreateKey::new("example.vertex"),
        kind: EntityKind::Topology(TopologyEntityKind::Vertex),
    }],
    MutationOrigin::Seed,
);
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

## How It Relates To Other Features

- Use [Create Batch Builder](../topology-authoring/create-batch-builder.md)
  when you want a pleasant authoring lane.
- Use [Verification](../topology-authoring/verification.md) when you want the
  migration note for where runtime execution went.

## Inspection And Debugging

If your batch is hard to read:

- inspect the `MutationOrigin`
- inspect whether your `EntityReference` values are live ids or created refs
- decide whether a helper should own the authoring instead of your callsite

## Anti-Patterns

- Do not confuse write-side truth vocabulary with Query declaration grammar.
- Do not attach new schema-owned runtime readiness logic here.
- Do not use fake live ids when a created reference is the honest shape.

## Current Limits

- This doc covers topology write vocabulary, not the full authority execution
  story.
- Query owns the broad public runtime workflow.

## Related Docs

- [Authority](./README.md)
- [Create Batch Builder](../topology-authoring/create-batch-builder.md)
- [Verification](../topology-authoring/verification.md)
