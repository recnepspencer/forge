# Geometry Binding Vocabulary

## What This Feature Is

This feature covers the geometry-binding enums that belong to
`worth-schema` as write-side or interpretation vocabulary.

## Why You Use It

Use this when you need to classify the geometry shape or provenance attached to
topology truth.

## Stable Entry Points

- `SurfaceBindingKind`
- `CurveBindingKind`
- `CoedgeCurveKind`
- `CurveProvenanceKind`
- `VertexGeometryProvenanceKind`
- `VertexToleranceRegime`
- `SurfaceRelationKind`

## Core Mental Model

These enums do not run geometry work by themselves.

They give schema-owned names to:

- what kind of surface or curve binding something uses
- where a curve or vertex geometry came from
- how tolerant or exact the vertex geometry regime is
- how two surfaces relate

## How It Executes

This is classification vocabulary only.

## Small Example

```rust
use worth_schema::facade::platform::authority::{CurveBindingKind, SurfaceBindingKind};

let surface = SurfaceBindingKind::Cylinder;
let curve = CurveBindingKind::SurfaceIntersection;
```

## Real Example

```rust
use worth_schema::facade::platform::authority::{
    CurveProvenanceKind,
    SurfaceRelationKind,
    VertexGeometryProvenanceKind,
    VertexToleranceRegime,
};

let vertex_provenance = VertexGeometryProvenanceKind::EdgeSplit;
let tolerance = VertexToleranceRegime::Modeled;
let curve_provenance = CurveProvenanceKind::Imported;
let surface_relation = SurfaceRelationKind::General;
```

## How It Relates To Other Features

- Use [Topology Mutations](./topology-mutations.md) for the write-side topology
  batch that may refer to these classifications.
- Use [Interpretation Vocabulary](./interpretation-vocabulary.md) for the
  topology-side interpreted result names.

## Inspection And Debugging

If the chosen value feels ambiguous, that is usually a sign you need a more
precise domain classification rather than a local comment.

## Anti-Patterns

- Do not replace these enums with strings in consumer code.
- Do not treat geometry classification vocab as a runtime execution API.

## Current Limits

- These names classify geometry-adjacent truth. They do not by themselves
  express the full geometry payload.

## Related Docs

- [Authority](./README.md)
- [Topology Mutations](./topology-mutations.md)
- [Interpretation Vocabulary](./interpretation-vocabulary.md)
