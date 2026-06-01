# Field Selection Recipes

## What This Feature Is

This page is a task-first guide for combining the main query-vocabulary
surfaces in `worth-schema`:

- `QueryAspectPath`
- `QueryAspectFamily`
- `QueryCollection`
- `QuerySchemaBasis`
- `QueryLiveField`

## Why You Use It

Use this page when you know you need Query-facing names, but you want the
shortest path to the right combination.

## Stable Entry Points

- [Query Aspect Paths](./query-aspect-paths.md)
- [Query Aspect Family](./query-aspect-family.md)
- [Query Collections And Bases](./query-collections-and-bases.md)
- [Live Fields](./live-fields.md)

## Core Mental Model

These types name different parts of the same surface:

- `QueryAspectPath` names the truth slice
- `QueryAspectFamily` names the broad family
- `QueryCollection` names the target collection
- `QuerySchemaBasis` names the specific live or computed basis
- `QueryLiveField` names concrete delivered fields

## How It Executes

There is no runtime behavior here. The point is to choose honest names before
the Query runtime owns admission and orchestration.

## Small Example

```rust
use worth_schema::facade::QueryAspectPath;
use worth_schema::facade::platform::aspects::{Aspect, TopologyAspect};

let path = QueryAspectPath::from_aspect(Aspect::Topology(TopologyAspect::Structure));
assert_eq!(path.as_str(), "topology.structure");
```

## Real Example

```rust
use worth_schema::facade::{
    QueryAspectPath,
    QueryCollection,
    QueryLiveField,
    QuerySchemaBasis,
};

let collection = QueryCollection::TopologyRelation;
let basis = QuerySchemaBasis::TopologyRelationLiveView;
let aspect = QueryAspectPath::TOPOLOGY_STRUCTURE;
let fields = [
    QueryLiveField::IdentityId.delivered_name(),
    QueryLiveField::TopologyKind.delivered_name(),
    QueryLiveField::TopologySourceIdentity.delivered_name(),
    QueryLiveField::TopologyTargetIdentity.delivered_name(),
];
```

## How It Relates To Other Features

- Use [Recipes](../learn/recipes.md) for broader crate-level task routing.
- Use `forge-query` once you need to build or run the actual declaration.

## Inspection And Debugging

If you are unsure which name to pick:

- start with the truth slice using `QueryAspectPath`
- then decide whether the surface is live or computed
- then choose `QueryLiveField` only for concrete delivered field names

## Anti-Patterns

- Do not invent raw strings for names that already have published enums.
- Do not use schema vocabulary as a substitute for Query runtime readiness.
- Do not choose a computed basis when you actually mean a live view.

## Current Limits

- This page is about naming, not runtime behavior.

## Related Docs

- [Query Vocabulary](./README.md)
- [Query Aspect Paths](./query-aspect-paths.md)
- [Query Aspect Family](./query-aspect-family.md)
- [Query Collections And Bases](./query-collections-and-bases.md)
- [Live Fields](./live-fields.md)
