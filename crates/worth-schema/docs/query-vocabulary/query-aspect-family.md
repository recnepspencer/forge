# Query Aspect Family

## What This Feature Is

`QueryAspectFamily` is the small grouping enum behind `QueryAspectPath`.

It tells you which broad truth family a query-facing path belongs to:

- topology
- geometry
- lineage
- naming
- diagnostics

## Why You Use It

Use this when you need to group or branch on query-facing truth families
without reparsing the `aspect.field` string yourself.

## Stable Entry Points

- `QueryAspectFamily`
- `QueryAspectPath::family()`

## Core Mental Model

`QueryAspectPath` is the exact path.

`QueryAspectFamily` is the broad bucket that path lives in.

Use the family when you need broad routing or filtering. Use the path when you
need the exact truth slice.

## How It Executes

There is no runtime workflow here. This is query vocabulary only.

## Small Example

```rust
use worth_schema::facade::{QueryAspectFamily, QueryAspectPath};

let path = QueryAspectPath::TOPOLOGY_BOUNDARY;

assert_eq!(path.family(), QueryAspectFamily::Topology);
```

## Real Example

```rust
use worth_schema::facade::{QueryAspectFamily, QueryAspectPath};

fn accepts_path(path: QueryAspectPath) -> bool {
    matches!(
        path.family(),
        QueryAspectFamily::Topology | QueryAspectFamily::Naming
    )
}
```

## How It Relates To Other Features

- Use [Query Aspect Paths](./query-aspect-paths.md) when you need the exact
  published path.
- Use [Query Collections And Bases](./query-collections-and-bases.md) when you
  are naming the surrounding live or computed surface.

## Inspection And Debugging

If a family-level decision looks wrong:

- inspect the exact path with `as_str()`
- check the path constant you started from
- make sure you actually wanted broad family routing instead of exact-path
  routing

## Anti-Patterns

- Do not parse `topology.*` or `geometry.*` by hand when `family()` already
  gives you the answer.
- Do not use the family when your logic actually depends on one exact path.

## Current Limits

- This page covers broad grouping only.
- Runtime admission, orchestration, and recovery belong to `forge-query`.

## Related Docs

- [Query Vocabulary](./README.md)
- [Query Aspect Paths](./query-aspect-paths.md)
- [Query Collections And Bases](./query-collections-and-bases.md)
