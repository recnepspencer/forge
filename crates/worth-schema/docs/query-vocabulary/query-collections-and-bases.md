# Query Collections And Bases

## What This Feature Is

`QueryCollection` and `QuerySchemaBasis` are the published names for two
different things:

- `QueryCollection` names the target collection
- `QuerySchemaBasis` names the schema-backed basis for a live or computed view

## Why You Use It

Use these when you need a stable target or basis name for:

- live view declarations
- computed declaration surfaces
- schema-backed diagnostics or derived views

This is the right surface when you are saying:

- "this live view reads topology entities"
- "this computed output belongs to topology validation"
- "this declaration should point at the persistent-name live basis"

## Stable Entry Points

From [facade.rs](C:/Users/Esther/Documents/Programming/forge_workspace/worktree_2/crates/worth-schema/src/facade.rs:1):

- `QueryCollection`
- `QuerySchemaBasis`

## Core Mental Model

The collection tells Query what broad record set you mean.

The schema basis tells Query what named schema-backed view or truth basis you
mean inside that collection family.

Examples:

- `QueryCollection::TopologyEntity`
- `QuerySchemaBasis::TopologyEntityLiveView`
- `QueryCollection::TopologyValidation`
- `QuerySchemaBasis::TopologyValidationComputed`

## How It Executes

This is vocabulary, not runtime admission.

The important behavior is:

- each enum variant has a stable string name
- the names line up with the schema-facing declaration surfaces used in tests
  and authoring code
- the basis names distinguish live surfaces from computed surfaces

## Small Example

```rust
use worth_schema::facade::{QueryCollection, QuerySchemaBasis};

assert_eq!(QueryCollection::TopologyEntity.as_str(), "TopologyEntity");
assert_eq!(
    QuerySchemaBasis::TopologyEntityLiveView.as_str(),
    ".schema.live.topology_entity"
);
```

## Real Example

```rust
use forge_query::facade::ForgeQueryLiveViewBuilder;
use worth_schema::facade::{QueryCollection, QuerySchemaBasis};

let declaration = ForgeQueryLiveViewBuilder::surface(".topology.entities")
    .from(QueryCollection::TopologyEntity.as_str())
    .schema_basis(QuerySchemaBasis::TopologyEntityLiveView.as_str())
    .build()?;
```

## How It Relates To Other Features

- Use [Query Aspect Paths](./query-aspect-paths.md) when you also need stable
  aspect names.
- Use [Live Fields](./live-fields.md) when you need the delivered field names
  inside a projection.
- Use `forge-query` builders when you are authoring actual declarations instead
  of just naming surfaces.

## Inspection And Debugging

If something looks wrong:

- inspect `as_str()` for the exact emitted name
- check whether you chose the live or computed basis variant
- make sure you are not using schema vocabulary as if it were runtime support

## Anti-Patterns

- Do not hardcode collection or basis strings when the published enums already
  name them.
- Do not treat collection names as declaration admission proof.
- Do not push declaration builder wrappers back into `worth-schema`.

## Current Limits

- This doc only covers naming the collection and basis.
- Runtime declaration authoring stays in `forge-query`.
- If you are guessing between two basis names, that usually means you should
  inspect the source truth you want before writing the declaration.

## Related Docs

- [Query Vocabulary](./README.md)
- [Query Aspect Paths](./query-aspect-paths.md)
- [Live Fields](./live-fields.md)
