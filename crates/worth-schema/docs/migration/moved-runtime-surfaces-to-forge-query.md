# Moved Runtime Surfaces To forge-query

## What This Feature Is

This is the migration guide for consumers who previously reached into
`worth-schema` for runtime-facing answers.

The short version:

- keep using `worth-schema` for vocabulary and schema-authority support
- move runtime-facing work to `forge-query`

## Why You Use It

Use this guide if you are looking for a removed `worth-schema` facade export or
if you are unsure whether a public surface belongs to schema or Query now.

## Stable Entry Points

Keep using `worth-schema` for:

- `platform::aspects::Aspect`
- `platform::entities::EntityKind`
- `platform::relations::RelationKind`
- `platform::authority::RawTopologyIntent`
- `platform::authority::TopologyMutationBatch`
- `QueryAspectPath`
- `QueryCollection`
- `QuerySchemaBasis`
- `bootstrap_schema_registry()`
- topology authoring helpers

Move to `forge-query` for:

- configured handle entry
- declaration readiness
- declaration orchestration
- support and inspection
- invariant registration and denials
- recovery

## Core Mental Model

`worth-schema` names truth.

`forge-query` owns the public runtime workflow around that truth.

If a surface answers:

- "can I run this?"
- "what blocked this?"
- "how do I inspect this runtime artifact?"
- "what do I do next?"

that answer belongs in Query now.

## Surface Mapping

Old schema instinct:

- mutation admission/support contract
- runtime invariant rollout plan
- boundary envelope / boundary failure
- narrated runtime trace
- topology verification helper
- schema-owned verified authority result

New home:

- Query readiness and support surfaces
- Query invariant registration and invariant denial surfaces
- Query inspection, support, explanation, and recovery surfaces
- Query-owned topology execution, outcome, inspection, and recovery surfaces

The practical translation is:

- if you need a stable name for truth, stay in schema
- if you need to run, inspect, or recover runtime work, go to Query

## Small Example

Old instinct:

```rust
// Old schema-era instinct:
// look for a schema-owned readiness or support helper here.
```

New direction:

```rust
use forge_query::facade::ForgeQueryRuntime;

let handle = query
    .domain(MyDomain)
    .with_operating_context(context)
    .validate()?
    .admit()?;

let readiness = handle.declaration_entry_readiness::<MyDeclarationFamily>();
```

## Real Example

If you previously wanted schema to tell you whether a workflow was blocked by
runtime posture, the replacement question is now:

1. what declaration family am I trying to run?
2. what admitted operating world am I in?
3. what does Query readiness, support, or recovery say?

That keeps one public runtime story instead of two competing ones.

## How It Relates To Other Features

- Use [Start Here](../start_here.md) to reset the basic crate boundary.
- Use [Query Aspect Paths](../query-vocabulary/query-aspect-paths.md) if you
  need schema-owned query vocabulary after the migration.
- Use [Your First Topology Intent](../topology-authoring/your-first-topology-intent.md)
  if your work is topology authoring support rather than runtime
  orchestration.

## Inspection And Debugging

If you are not sure whether a missing surface should be in schema, ask:

- is this naming truth?
- is this schema bootstrap?
- is this topology authoring support?

If the answer is no, and it sounds runtime-facing, it probably belongs in
`forge-query`.

## Anti-Patterns

- Do not rebuild a local schema-owned support matrix.
- Do not reintroduce schema-owned runtime rollout docs just because consumers
  ask where the old APIs went.
- Do not push Query declaration builders back into `worth-schema` just because
  multiple domain crates need them.

## Related Docs

- [Start Here](../start_here.md)
- [Feature Index](../learn/feature-index.md)
- [Query Vocabulary](../query-vocabulary/README.md)
