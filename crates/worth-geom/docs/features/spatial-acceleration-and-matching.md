<!-- worth-doc
crate: worth-geom
kind: feature
id: spatial-acceleration-and-matching
query_integration_required: false
query_proof_required: false
touches_query: false
-->

# Spatial Acceleration And Matching

## What This Feature Is

This feature owns the geometry-side spatial acceleration, local-space, and
matching substrate used before higher crates attach semantic meaning.

## Why You Use It

Use this when you need geometric coincidence, edge matching, welding, BVH/BSP
support, or characteristic-scale reasoning.

## Stable Entry Points

- `worth_geom::facade::{BspConfig, BspNode, BspSolid, PlaneSet}`
- `worth_geom::facade::{BvhNode, query_overlapping_pairs}`
- `worth_geom::facade::{LocalCoordinateSpace, ScaleAnalysis, compute_characteristic_scale}`
- `worth_geom::facade::{CoincidenceGraph, CoincidenceKind, EpsilonWelder, EdgeMatch}`

## Common Path

1. Choose the acceleration or matching surface that fits the geometric task.
2. Produce the geometric neighborhood or coincidence result.
3. Let higher crates interpret the meaning of that result rather than mixing in
   runtime or topology truth locally.

## Advanced Path

Use the advanced path for BSP splitting, BVH overlap queries, fuzzy edge
matching, radial candidate selection, or local-coordinate normalization.

## Inspection And Debugging

Reach for this layer when the problem is geometric locality or coincidence, not
yet a topology certification or runtime-support problem.

## Anti-Patterns

- hiding geometric locality decisions inside topology helpers
- treating acceleration structures as a semantic authority
- skipping scale analysis and then blaming higher crates for noisy geometry

## Current Limits

This doc backfills the geometric locality and matching substrate that later
Worth crates already rely on.

## Related Docs

- [Runtime Support](../../../worth-topo/docs/features/runtime-support.md)
- [Boundary Certification And Intersection](./boundary-certification-and-intersection.md)
- [Geom To Spatial Authority Boundary](../boundaries/geom-to-spatial-authority-boundary.md)
