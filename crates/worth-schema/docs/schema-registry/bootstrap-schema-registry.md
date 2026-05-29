# Bootstrap Schema Registry

## What This Feature Is

This feature gives you the published way to build a Worth
`RelationalSchemaRegistry`.

The main entrypoints are:

- `bootstrap_schema_registry()`
- `SchemaBuilder`
- `SchemaBuildError`
- `SCHEMA_ID`
- `SCHEMA_VERSION_ID`

## Why You Use It

Use this when you need a relational schema registry that already knows Worth
topology, geometry, lineage, naming, and diagnostics kinds.

This is the right surface when you are saying:

- "I need a runtime with the Worth schema loaded"
- "I need tests or fixtures with the real schema registration"
- "I need the schema identity/version for the authoritative truth basis"

## Stable Entry Points

From [facade.rs](C:/Users/Esther/Documents/Programming/forge_workspace/worktree_2/crates/worth-schema/src/facade.rs:1):

- `bootstrap_schema_registry()`
- `SchemaBuilder`
- `SchemaBuildError`
- `SCHEMA_ID`
- `SCHEMA_VERSION_ID`

## Core Mental Model

`bootstrap_schema_registry()` gives you the full Worth schema registry.

`SchemaBuilder` is the smaller "make me declare what I expect" wrapper around
that registry bootstrap.

Today the builder makes you opt into:

- topology kinds
- naming kinds

That keeps callers honest about the minimum surfaces they are asking for before
the full bootstrap runs.

## How It Executes

`bootstrap_schema_registry()` registers the Worth schema families in sequence:

1. topology
2. geometry
3. lineage
4. naming
5. diagnostics

`SchemaBuilder` then adds a small guard layer that fails early if the caller
did not request the required kind families first.

## Small Example

```rust
use worth_schema::facade::bootstrap_schema_registry;

let registry = bootstrap_schema_registry()?;
```

## Real Example

```rust
use worth_schema::facade::SchemaBuilder;

let registry = SchemaBuilder::new()
    .with_topology_kinds()
    .with_naming_kinds()
    .build()?;
```

## How It Relates To Other Features

- Use [Query Aspect Paths](../query-vocabulary/query-aspect-paths.md) when you
  need query-facing names after the schema is loaded.
- Use [Topology Authoring](../topology-authoring/README.md) when you want to
  build or verify topology truth against a runtime that uses this schema.

## Inspection And Debugging

If schema bootstrap fails:

- inspect `SchemaBuildError`
- confirm you opted into the required builder surfaces
- confirm you are using the published bootstrap entry rather than rebuilding
  registrations by hand

## Anti-Patterns

- Do not rebuild the Worth schema registration manually if the published
  bootstrap surface already does it.
- Do not treat `SchemaBuilder` as a runtime planner. It is a schema bootstrap
  guard.
- Do not confuse schema registration with Query runtime admission. They solve
  different problems.

## Current Limits

- This doc covers the published bootstrap lane, not every lower registration
  helper.
- Query-owned runtime readiness, orchestration, and invariant registration live
  in `forge-query`, not here.

## Related Docs

- [Start Here](../start_here.md)
- [Query Vocabulary](../query-vocabulary/README.md)
- [Topology Authoring](../topology-authoring/README.md)

