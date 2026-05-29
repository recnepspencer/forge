# Live Fields

## What This Feature Is

`QueryLiveField` is the published way to name common delivered fields for a
live schema view.

Examples:

- `QueryLiveField::IdentityId`
- `QueryLiveField::TopologyKind`
- `QueryLiveField::TopologySourceIdentity`
- `QueryLiveField::NamingTargetIdentity`

## Why You Use It

Use this when a live view needs a field name that is more specific than a
whole aspect path.

This is the right surface when you are saying:

- "I need `identity.id` in the projection"
- "I need topology source and target identities"
- "I need the delivered name for a naming target identity field"

## Stable Entry Points

From [facade.rs](C:/Users/Esther/Documents/Programming/forge_workspace/worktree_2/crates/worth-schema/src/facade.rs:1):

- `QueryLiveField`

## Core Mental Model

`QueryAspectPath` names a truth slice.

`QueryLiveField` names a delivered field inside a live view projection.

`QueryLiveField::Aspect(path)` lets you lift a whole aspect path into the same
surface when that is the right projection shape.

## How It Executes

This feature gives you three stable reads:

- `aspect()`
- `field()`
- `delivered_name()`

That means you can inspect:

- which section the field belongs to
- which field name is used inside the section
- which final delivered name should go into a declaration projection

## Small Example

```rust
use worth_schema::facade::QueryLiveField;

assert_eq!(QueryLiveField::IdentityId.delivered_name(), "identity.id");
assert_eq!(QueryLiveField::TopologyKind.delivered_name(), "topology.kind");
```

## Real Example

```rust
use forge_query::facade::ForgeQueryLiveViewBuilder;
use worth_schema::facade::{QueryCollection, QueryLiveField, QuerySchemaBasis};

let declaration = ForgeQueryLiveViewBuilder::surface(".topology.relations")
    .select([
        QueryLiveField::IdentityId.delivered_name(),
        QueryLiveField::TopologyKind.delivered_name(),
        QueryLiveField::TopologySourceIdentity.delivered_name(),
        QueryLiveField::TopologyTargetIdentity.delivered_name(),
    ])
    .from(QueryCollection::TopologyRelation.as_str())
    .schema_basis(QuerySchemaBasis::TopologyRelationLiveView.as_str())
    .build()?;
```

## How It Relates To Other Features

- Use [Query Aspect Paths](./query-aspect-paths.md) when you want to project a
  whole aspect slice.
- Use [Query Collections And Bases](./query-collections-and-bases.md) when you
  need to name the target and basis around these fields.

## Inspection And Debugging

If a projection looks wrong:

- inspect `delivered_name()`
- verify the `aspect()` and `field()` split
- confirm that you used a live field rather than inventing a local string
- check whether you actually wanted `QueryAspectPath` for a whole truth slice
  instead of one delivered field

## Anti-Patterns

- Do not hardcode names like `"identity.id"` when `QueryLiveField` already owns
  them.
- Do not use live fields as a replacement for runtime inspection or recovery.

## Current Limits

- This page covers naming and projection support only.
- Runtime declaration behavior belongs to `forge-query`.

## Related Docs

- [Query Vocabulary](./README.md)
- [Query Aspect Paths](./query-aspect-paths.md)
- [Query Collections And Bases](./query-collections-and-bases.md)
