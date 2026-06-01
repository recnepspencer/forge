# Derived Invalidation Declarations

## What This Feature Is

This feature covers the schema-owned declarations that map authoritative truth
changes onto derived invalidation targets.

The main surfaces are:

- `platform::authority::DerivedInvalidationTarget`
- `platform::authority::DerivedTruthSurfaceKind`
- `platform::authority::TruthToDerivedInvalidationDeclaration`
- `platform::authority::milestone_two_invalidation_declarations()`

## Why You Use It

Use this when you need a stable schema-owned mapping from one truth-side patch
field onto one derived invalidation target.

## Stable Entry Points

- `DerivedInvalidationTarget`
- `DerivedTruthSurfaceKind`
- `TruthToDerivedInvalidationDeclaration`
- `milestone_two_invalidation_declarations()`

## Core Mental Model

The truth side says what changed.

The invalidation declaration says which derived surface should be treated as
stale because of that change.

## How It Executes

This is declaration data, not runtime invalidation itself.

## Small Example

```rust
use worth_schema::facade::platform::authority::{
    milestone_two_invalidation_declarations, DerivedInvalidationTarget,
};

let declarations = milestone_two_invalidation_declarations();

assert!(declarations.iter().any(|declaration| {
    declaration.target == DerivedInvalidationTarget::TopologyStructure
}));
```

## Real Example

```rust
use worth_schema::facade::platform::authority::milestone_two_invalidation_declarations;

let declaration = milestone_two_invalidation_declarations()
    .into_iter()
    .find(|declaration| declaration.truth_patch_field == "topology.boundary")
    .expect("topology boundary invalidation declaration");

assert_eq!(declaration.declaration_id, "topology-boundary");
```

## How It Relates To Other Features

- Use [Topology Mutations](./topology-mutations.md) when you are on the
  write-side truth vocabulary.
- Use [Query Vocabulary](../query-vocabulary/README.md) when you need the
  schema-facing query names around those truth slices.

## Inspection And Debugging

If a mapping looks wrong:

- inspect `truth_patch_field`
- inspect `truth_surface_kind`
- inspect `target.bridge_scope()`

## Anti-Patterns

- Do not treat this declaration table as a runtime orchestration surface.
- Do not invent new local invalidation ids when a stable declaration already
  exists here.

## Current Limits

- The published helper carries the historical `milestone_two_` name.
- Query owns the runtime orchestration around these declarations.

## Related Docs

- [Authority](./README.md)
- [Topology Mutations](./topology-mutations.md)
- [Query Vocabulary](../query-vocabulary/README.md)
