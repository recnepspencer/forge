# Query Aspect Paths

## What This Feature Is

`QueryAspectPath` is the published way to name one query-facing slice of Worth
truth.

Examples:

- `QueryAspectPath::TOPOLOGY_STRUCTURE`
- `QueryAspectPath::GEOMETRY_BINDING`
- `QueryAspectPath::NAMING_PERSISTENT_NAME`

The helper functions:

- `query_aspect_paths(...)`
- `query_aspect_path_strings(...)`
- `query_aspect_paths_from_set(...)`

convert schema `platform::aspects::Aspect` values into these query-facing
names.

`QueryAspectFamily` is the small grouping enum behind those paths. It lets you
ask whether a path belongs to topology, geometry, lineage, naming, or
diagnostics without reparsing the string yourself.

## Why You Use It

Use this when you need a stable string-level or enum-level name for a truth
slice that a Query declaration, schema view, or diagnostic surface needs to
reference.

Use this when you are saying:

- "this declaration reads topology structure"
- "this live view exposes persistent names"
- "this touched-aspect set needs deterministic query-facing names"

Do not use it as a runtime admission or support API. It names truth. It does
not tell you whether the runtime will admit that truth.

## Stable Entry Points

From [facade.rs](C:/Users/Esther/Documents/Programming/forge_workspace/worktree_2/crates/worth-schema/src/facade.rs:1):

- `QueryAspectPath`
- `QueryAspectFamily`
- `query_aspect_paths(...)`
- `query_aspect_path_strings(...)`
- `query_aspect_paths_from_set(...)`

## Core Mental Model

`platform::aspects::Aspect` is the schema meaning.

`QueryAspectPath` is the query-facing name for that meaning.

`QueryAspectFamily` is the broad family label for that query-facing name.

For example:

- `Aspect::Topology(TopologyAspect::Structure)` means "topology structure"
- `QueryAspectPath::TOPOLOGY_STRUCTURE` is the query-facing path name for that
  same truth slice

This lets consumers preserve one stable mapping instead of inventing local
string names.

## How It Executes

There is no runtime workflow here. This is vocabulary conversion.

The important behaviors are:

- every published path has a stable `aspect.field` string
- every path also has a stable broad family through `family()`
- `from_aspect(...)` maps schema truth into that query-facing path
- `into_aspect(...)` reopens the schema meaning
- set conversion helpers preserve deterministic ordering

## Small Example

```rust
use worth_schema::facade::{
    QueryAspectPath,
};
use worth_schema::facade::platform::aspects::{Aspect, TopologyAspect};

let aspect = Aspect::Topology(TopologyAspect::Structure);
let path = QueryAspectPath::from_aspect(aspect);

assert_eq!(path.as_str(), "topology.structure");
assert_eq!(path.family(), worth_schema::facade::QueryAspectFamily::Topology);
```

## Real Example

```rust
use std::collections::BTreeSet;

use worth_schema::facade::{
    query_aspect_path_strings,
    query_aspect_paths_from_set,
};
use worth_schema::facade::platform::aspects::{
    Aspect, DiagnosticsAspect, NamingAspect, TopologyAspect,
};

let touched_aspects = BTreeSet::from([
    Aspect::Topology(TopologyAspect::Boundary),
    Aspect::Naming(NamingAspect::PersistentName),
    Aspect::Diagnostics(DiagnosticsAspect::Decisions),
]);

let paths = query_aspect_paths_from_set(&touched_aspects);
let path_strings = query_aspect_path_strings(touched_aspects);
```

## How It Relates To Other Features

- Use [Bootstrap Schema Registry](../schema-registry/bootstrap-schema-registry.md)
  when you need the full registered schema.
- Use [Topology Authoring](../topology-authoring/README.md) when you are
  authoring topology truth instead of naming a truth slice.
- Use `forge-query` once you need admission, orchestration, support, or
  recovery.

## Inspection And Debugging

If something looks wrong:

- inspect the `aspect.field` string from `as_str()`
- check the family with `family()`
- reopen the schema meaning with `into_aspect()`

## Anti-Patterns

- Do not hardcode local string names like `"topology.structure"` if
  `QueryAspectPath` already names the surface.
- Do not use `QueryAspectPath` as if it were a runtime support contract.
- Do not treat these names as proof that a Query workflow is available yet.

## Current Limits

- This page only covers aspect-path vocabulary.
- Other query vocabulary such as collections and bases should be documented as
  adjacent surfaces, not inferred from this one page.

## Related Docs

- [Query Vocabulary](./README.md)
- [Bootstrap Schema Registry](../schema-registry/bootstrap-schema-registry.md)
- [Moved Runtime Surfaces To forge-query](../migration/moved-runtime-surfaces-to-forge-query.md)
