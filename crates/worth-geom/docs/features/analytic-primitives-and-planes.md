<!-- worth-doc
crate: worth-geom
kind: feature
id: analytic-primitives-and-planes
query_integration_required: false
query_proof_required: false
touches_query: false
-->

# Analytic Primitives And Planes

## What This Feature Is

This feature owns the core analytic geometry primitives that later spatial,
topological, and realization workflows depend on.

## Why You Use It

Use this when you need the geometric substrate itself: points, planes, rays,
polygons, implicit vertices, and basic authored shapes.

## Stable Entry Points

- `worth_geom::facade::{Plane, PlaneRelation, Vertex, VertexGeom, VertexProvenance}`
- `worth_geom::facade::{ParameterSpacePoint, EdgeTieBreaker}`
- `worth_geom::facade::{block, cube, prism, pyramid, tetrahedron, wedge, dodecahedron}`

## Common Path

1. Choose the primitive or plane surface that matches the geometric fact.
2. Use the analytic operation directly.
3. Pass the resulting geometry upward only when a higher crate needs semantic
   meaning.

## Advanced Path

Use the advanced path when you need exact plane classification, implicit-vertex
selection, ray/plane resolution, or authored primitive generation.

## Inspection And Debugging

Inspect this surface when the bug is about geometry itself before any spatial
meaning or topology truth is attached.

## Anti-Patterns

- treating analytic primitives as if they already prove spatial meaning
- teaching shape creation only through higher-level workflows
- jumping straight to topology explanations for plane or polygon mistakes

## Current Limits

This doc backfills the foundational geometry primitives that the Worth runtime
stack already depends on.

## Related Docs

- [Geom To Spatial Authority Boundary](../boundaries/geom-to-spatial-authority-boundary.md)
- [Construction-Time Birth Bindings](../../../worth-spatial/docs/features/construction-time-birth-bindings.md)
- [Curve And Surface Schema](./curve-and-surface-schema.md)
