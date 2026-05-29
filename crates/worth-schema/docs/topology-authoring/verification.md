# Verification

## What This Feature Is

This page explains a removal and the remaining curated authoring boundary.

`worth-schema` no longer publishes topology verification as a normal consumer
runtime lane.

## Why You Use It

Use this page when you were looking for:

- `verify_topology_intent(...)`
- `verify_topology_intent_on_branch(...)`
- schema-owned verified commit execution helpers
- the old "run this topology intent for me" lane

## Stable Entry Points

There are no stable verification entrypoints for this lane in
`worth-schema` anymore.

Use:

- `worth-schema` to build the authored truth input
- `forge-query` for admitted runtime entry, orchestration, inspection, and
  recovery

## Core Mental Model

The old schema verification lane mixed two responsibilities:

- schema-owned truth authoring
- runtime-owned execution and result handling

## How It Executes

The new intended flow is:

1. author a `RawTopologyIntent` in `worth-schema`
2. enter through a configured/admitted Query runtime lane
3. hand the intent to a Query-backed topology family/helper
4. inspect the Query result, receipt, envelope, or recovery surface there

## Small Example

```rust
use worth_schema::facade::{
    topology_authoring::TopologyCreateBatchBuilder,
};
use worth_schema::facade::platform::authority::MutationOrigin;
use worth_schema::facade::platform::entities::{EntityKind, TopologyEntityKind};

let intent = TopologyCreateBatchBuilder::new()
    .topology_entity("example.vertex", EntityKind::Topology(TopologyEntityKind::Vertex))
    .finish(MutationOrigin::Seed);

// Next step: pass `intent` to a Query-backed topology runtime surface.
```

## Real Example

```rust
use forge_query::facade::ForgeQueryRuntime;

let handle = query
    .domain(MyDomain)
    .with_operating_context(context)
    .validate()?
    .admit()?;

let outcome = handle.orchestrate_declaration_entry_outcome(input);
```

This is intentionally generic. The exact family/helper lives with the
Query-backed topology runtime surface, not in `worth-schema`.

## How It Relates To Other Features

- Use [Create Batch Builder](./create-batch-builder.md) before this when you
  want a pleasant authoring lane.
- Use [Verified Commits And Read Basis](../authority/verified-commits-and-read-basis.md)
  for the migration note on the old authority result story.
- Use `forge-query` for the actual runtime lane.

## Inspection And Debugging

If you were depending on the old verification helpers, audit the callsite and
split it into:

- schema-owned truth input authoring
- Query-owned runtime execution and inspection
- Query-owned recovery if the old code expected a next-step answer

## Anti-Patterns

- Do not reintroduce local schema verification wrappers.
- Do not rebuild a second runtime execution story beside Query.

## Current Limits

- `worth-schema` does not provide a public verification lane for this work.
- Query owns the runtime execution path.
- `worth_schema::facade::topology_authoring` owns only authored truth inputs
  and fixture-oriented support artifacts.

## Related Docs

- [Topology Authoring](./README.md)
- [Create Batch Builder](./create-batch-builder.md)
- [Verified Commits And Read Basis](../authority/verified-commits-and-read-basis.md)
