# Schema Builder

## What This Feature Is

`SchemaBuilder` is the small guard surface around the full Worth schema
bootstrap.

The builder lets you say which kind families you expect before it emits the
full `RelationalSchemaRegistry`.

## Why You Use It

Use this when you want a callsite that says:

- "this setup expects topology kinds"
- "this setup expects naming kinds"

instead of calling the full bootstrap with no guard at all.

## Stable Entry Points

From [facade.rs](C:/Users/Esther/Documents/Programming/forge_workspace/worktree_2/crates/worth-schema/src/facade.rs:1):

- `SchemaBuilder`
- `SchemaBuildError`

## Core Mental Model

This is not a second schema system.

It is a small published wrapper that:

1. asks the caller to opt into required surfaces
2. fails early if those surfaces were not declared
3. then calls the full bootstrap

## How It Executes

Current builder flow:

1. call `SchemaBuilder::new()`
2. opt into `with_topology_kinds()`
3. opt into `with_naming_kinds()`
4. call `build()`

If one of those opt-ins is missing, `build()` returns `SchemaBuildError`.

## Small Example

```rust
use worth_schema::facade::SchemaBuilder;

let registry = SchemaBuilder::new()
    .with_topology_kinds()
    .with_naming_kinds()
    .build()?;
```

## Real Example

```rust
use forge_relational::facade::runtime::RelationalRuntimeApi;
use worth_schema::facade::SchemaBuilder;

let registry = SchemaBuilder::new()
    .with_topology_kinds()
    .with_naming_kinds()
    .build()?;

let runtime = RelationalRuntimeApi::builder()
    .schema_registry(registry)
    .build();
```

## How It Relates To Other Features

- Use [Bootstrap Schema Registry](./bootstrap-schema-registry.md) when you want
  the direct full bootstrap call.
- Use [Your First Topology Intent](../topology-authoring/your-first-topology-intent.md)
  after the runtime is configured.

## Inspection And Debugging

If `build()` fails:

- inspect `SchemaBuildError`
- check whether you forgot `with_topology_kinds()`
- check whether you forgot `with_naming_kinds()`
- remember that `build()` emits the standard bootstrap registry once the
  guard conditions are satisfied

## Anti-Patterns

- Do not treat `SchemaBuilder` as runtime readiness or Query admission.
- Do not treat it as a partial registry authoring DSL.
- Do not duplicate the published bootstrap guard logic locally.

## Current Limits

- The current published builder only guards topology and naming kinds.
- Full runtime workflow belongs to `forge-query`.

## Related Docs

- [Schema Registry](./README.md)
- [Bootstrap Schema Registry](./bootstrap-schema-registry.md)
- [Topology Authoring](../topology-authoring/README.md)
