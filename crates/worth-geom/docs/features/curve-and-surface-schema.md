<!-- worth-doc
crate: worth-geom
kind: feature
id: curve-and-surface-schema
query_integration_required: false
query_proof_required: false
touches_query: false
-->

# Curve And Surface Schema

## What This Feature Is

This feature owns the public curve, coedge, and surface schema layer for
geometry-bearing authored and evaluated surfaces.

## Why You Use It

Use this when you need geometric carriers, parameter-space admission, trim
surfaces, or curve/surface identity rather than only primitive solids.

## Stable Entry Points

- `worth_geom::facade::{CurveGeom, CurveKind, CurveProvenance, SpCurveApproximation}`
- `worth_geom::facade::{Coedge, ParametricCurve2D, SurfaceIndex}`
- `worth_geom::facade::{SurfaceData, SurfaceKind, SurfaceRelation, ParameterDomain}`
- `worth_geom::facade::{CanonicalParameterPoint, DomainParameterPoint, PolygonalTrimmedParameterRegion}`

## Common Path

1. Start from the curve or surface schema type.
2. Admit or evaluate parameter-space positions as needed.
3. Hand the resulting geometry upward only when spatial or topology semantics
   need to interpret it.

## Advanced Path

Use the advanced path when you need trimmed parameter regions, surface-pair
classification, or explicit curve provenance and approximation posture.

## Inspection And Debugging

Inspect this surface when the geometric carrier or parameter semantics are
wrong before the issue becomes a spatial anchor or topology problem.

## Anti-Patterns

- flattening all geometric carriers into point clouds
- teaching parameter admission only as local helper logic
- confusing curve/surface schema with higher-level runtime semantics

## Current Limits

This doc explains the public geometry schema layer that existing Worth surfaces
already inherit. It does not claim a full downstream semantics story.

## Related Docs

- [Construction-Time Birth Bindings](../../../worth-spatial/docs/features/construction-time-birth-bindings.md)
- [Analytic Primitives And Planes](./analytic-primitives-and-planes.md)
- [Boundary Certification And Intersection](./boundary-certification-and-intersection.md)
