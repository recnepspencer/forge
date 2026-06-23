<!-- worth-doc
crate: worth-geom
kind: boundary
id: geom-to-spatial-authority-boundary
query_integration_required: false
query_proof_required: false
touches_query: false
-->

# Geom To Spatial Authority Boundary

## Boundary

This boundary explains where pure geometry ends and spatial semantics begin.

## Allowed Upstream Inputs

- geometry primitives and algorithm outputs from `worth_geom::facade`

## Required Downstream Outputs

- spatially meaningful interpretation in `worth-spatial`
- no confusion between geometry output and topology or runtime authority

## Stable Entry Points

- upstream: `worth_geom::facade`
- downstream: `worth_spatial::facade`

## Forbidden Bypasses

- treating geometric results as if they already prove spatial birth semantics
- treating geometry outputs as topology truth

## Binding Artifacts Or Receipts

The important retained outputs are the geometry primitives and measurements
that `worth-spatial` later interprets into birth semantics.

## Related Docs

- [Geometry Overview](../foundations/geometry-overview.md)
- [worth-spatial Spatial Overview](../../../worth-spatial/docs/foundations/spatial-overview.md)
