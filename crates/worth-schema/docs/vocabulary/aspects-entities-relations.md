# Aspects, Entities, And Relations

## What This Feature Is

This feature covers the most basic truth vocabulary in `worth-schema`:

- `platform::aspects::Aspect`
- `platform::entities::EntityKind`
- `platform::relations::RelationKind`

## Why You Use It

Use this when you need the canonical Worth names for:

- what kind of truth slice something belongs to
- what kind of entity something is
- what kind of relation something is

## Stable Entry Points

- `platform::aspects::Aspect`
- `platform::entities::EntityKind`
- `platform::relations::RelationKind`

And their family enums, such as:

- `platform::aspects::TopologyAspect`
- `platform::aspects::GeometryAspect`
- `platform::entities::TopologyEntityKind`
- `platform::relations::TopologyRelationKind`

## Core Mental Model

`platform::aspects::Aspect` answers: what kind of truth is this?

`platform::entities::EntityKind` answers: what kind of record is this?

`platform::relations::RelationKind` answers: what kind of edge or ownership
link is this?

These are the base names the rest of the crate builds on.

## How It Executes

These enums are pure vocabulary.

## Small Example

```rust
use worth_schema::facade::{
};
use worth_schema::facade::platform::aspects::{Aspect, GeometryAspect};
use worth_schema::facade::platform::entities::{EntityKind, TopologyEntityKind};

let aspect = Aspect::Geometry(GeometryAspect::Binding);
let entity = EntityKind::Topology(TopologyEntityKind::Face);
```

## Real Example

```rust
use worth_schema::facade::platform::entities::{EntityKind, TopologyEntityKind};
use worth_schema::facade::platform::relations::{RelationKind, TopologyRelationKind};

let face = EntityKind::Topology(TopologyEntityKind::Face);
let loop_kind = EntityKind::Topology(TopologyEntityKind::Loop);
let outer_loop = RelationKind::Topology(TopologyRelationKind::FaceOuterLoop);
```

## How It Relates To Other Features

- Use [Query Vocabulary](../query-vocabulary/README.md) when you need the
  schema-facing query names derived from these concepts.
- Use [Schema Registry](../schema-registry/README.md) when you need these names
  registered into a real `RelationalSchemaRegistry`.
- Use [Topology Authoring](../topology-authoring/README.md) when you want to
  build write-side truth with these names.

## Inspection And Debugging

If a callsite looks ambiguous:

- inspect the family enum first
- then inspect the specific variant
- avoid collapsing the meaning into local strings

## Anti-Patterns

- Do not replace these enums with raw strings in consumer code.
- Do not skip the family distinctions when they carry real domain meaning.

## Current Limits

- This page covers the core names only.
- Runtime behavior belongs to Query-backed surfaces, not to these enums.

## Related Docs

- [Core Vocabulary](./README.md)
- [Query Vocabulary](../query-vocabulary/README.md)
- [Topology Authoring](../topology-authoring/README.md)
