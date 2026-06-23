<!-- worth-doc
crate: worth-geom
kind: feature
id: boundary-certification-and-intersection
query_integration_required: false
query_proof_required: false
touches_query: false
-->

# Boundary Certification And Intersection

## What This Feature Is

This feature owns the geometry-side intersection, projection, overlap, and
boundary-certification substrate used before topology or spatial layers claim
their own truth.

## Why You Use It

Use this when you need projected boundaries, intersection lines, polygon
overlap, or geometry-side boundary certification artifacts.

## Stable Entry Points

- `worth_geom::facade::{certify_boundary, build_projection_frame, project_boundary_to_2d}`
- `worth_geom::facade::{BoundaryArrangement, BoundaryCertError, BoundaryRejectReason}`
- `worth_geom::facade::{clip_line_to_face_polygon, compute_intersection_line, polygons_overlap_3d}`

## Common Path

1. Project or intersect the geometric boundary.
2. Certify the boundary arrangement or overlap facts.
3. Pass the resulting geometry-side artifact upward for topology or spatial
   interpretation.

## Advanced Path

Use the advanced path when you need arrangement vertices, atomic segments,
polygon-overlap classification, or exact projected-boundary reasoning.

## Inspection And Debugging

Use this layer when the question is whether the geometry-side boundary fact was
computed correctly before a higher crate consumed it.

## Anti-Patterns

- treating geometry-side certification as topology legality
- rebuilding intersection facts from higher-layer artifacts
- hiding projection-frame assumptions inside downstream adapters

## Current Limits

This doc backfills the public geometry proof substrate rather than the full
runtime-backed topology proof lane.

## Related Docs

- [Topology Certification And Parity](../../../worth-topo/docs/features/topology-certification-and-parity.md)
- [Curve And Surface Schema](./curve-and-surface-schema.md)
- [Geom To Spatial Authority Boundary](../boundaries/geom-to-spatial-authority-boundary.md)
