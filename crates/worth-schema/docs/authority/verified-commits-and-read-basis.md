# Verified Commits And Read Basis

## What This Feature Is

This page explains a boundary change.

`VerifiedTopologyCommit`, `DerivedTopologyReadBasis`, and
`DerivedTruthBasisIdentity` are no longer part of the broad public
`worth-schema` facade.

## Why You Use It

Use this page when you are migrating old code or docs that expected schema to
publish authority-shaped runtime result surfaces directly.

## Stable Entry Points

There are no stable entrypoints for these result types on the broad
`worth-schema` facade anymore.

## Core Mental Model

These were authority-shaped result surfaces.

The normal public story now is:

- schema authors truth input
- Query owns runtime execution, retained artifacts, inspection, and recovery

## How It Executes

If you were previously consuming these results directly, the migration question
is not "where is the same type now?" It is "which Query surface should own the
thing I was really trying to do?"

## Small Example

```rust
// Old instinct:
// "I need a schema-owned verified commit result."
//
// New question:
// "Do I actually need a Query outcome, inspection artifact, or recovery brief?"
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
let inspection = handle.inspect_declaration_entry(&outcome)?;
```

## How It Relates To Other Features

- Use [Topology Mutations](./topology-mutations.md) when you need the
  schema-owned write vocabulary.
- Use [Verification](../topology-authoring/verification.md) for the migration
  note on the removed verification lane.
- Use `forge-query` for the runtime lane.

## Inspection And Debugging

If your old code depended on one of these result types, inspect the callsite
and identify whether it really needs:

- runtime success/failure
- retained inspection
- support/explanation
- recovery
- or whether it was only using these types as a convenient carrier for authored
  truth that should stay in schema

## Anti-Patterns

- Do not re-export these types back onto the broad schema facade for
  convenience.
- Do not treat fixture-oriented authority payloads as the normal runtime
  consumer story.

## Current Limits

- This page documents the public boundary only. It is not a guide to internal
  support helpers.

## Related Docs

- [Authority](./README.md)
- [Verification](../topology-authoring/verification.md)
- [Moved Runtime Surfaces To forge-query](../migration/moved-runtime-surfaces-to-forge-query.md)
