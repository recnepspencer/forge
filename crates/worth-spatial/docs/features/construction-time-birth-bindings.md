<!-- worth-doc
crate: worth-spatial
kind: feature
id: construction-time-birth-bindings
query_integration_required: true
query_proof_required: false
touches_query: false
-->

# Construction-Time Birth Bindings

## What This Feature Is

Construction-time birth bindings are the spatial surfaces that explain how a
constructed result acquired its spatial meaning at birth.

## Why You Use It

Use this when you need to know what spatial facts were attached during
construction rather than after-the-fact derived interpretation.

## Stable Entry Points

- `worth_spatial::facade::binding`
- `worth_spatial::facade::anchor_binding`
- `worth_spatial::facade::placement`

## Common Path

Birth bindings are spatial semantics attached at construction time. They are
not caller-owned annotations and they are not topology authority.

The runtime lane comes from Query. Spatial consumes that lane and attaches the
binding or birth semantics that later artifact and proof surfaces inspect.

## Small Example

Use this when a kernel workflow needs to say not only "this primitive exists"
but also "this is the spatial meaning that was admitted at birth."

## Advanced Path

Birth bindings are where later replay, inspection, and rejection-locality
surfaces get their spatial grounding. If this surface is wrong, later proof can
be coherent but still semantically wrong.

## Query Integration

Query owns the runtime lane that carries construction into spatial birth truth.
`worth-spatial` owns the semantics attached on that lane, not the runtime
itself.

## How It Relates To Other Features

- [Birth Truth Artifacts](./birth-truth-artifacts.md)
- [Analytic Primitives And Planes](../../../worth-geom/docs/features/analytic-primitives-and-planes.md)
- [Curve And Surface Schema](../../../worth-geom/docs/features/curve-and-surface-schema.md)
- [Kernel To Spatial](../../../worth-kernel/docs/boundaries/kernel-to-spatial.md)

## Inspection And Debugging

Inspect birth bindings when a workflow reached spatial semantics but produced
the wrong admitted birth meaning.

## Anti-Patterns

- treating binding as a caller-owned metadata layer
- treating birth binding as topology certification

## Current Limits

This doc covers the admitted Milestone 4 birth-binding story only.

## Related Docs

- [Birth Completeness And Impossibility](./birth-completeness-and-impossibility.md)
- [Analytic Primitives And Planes](../../../worth-geom/docs/features/analytic-primitives-and-planes.md)
- [Curve And Surface Schema](../../../worth-geom/docs/features/curve-and-surface-schema.md)
- [Geom To Spatial Authority Boundary](../../../worth-geom/docs/boundaries/geom-to-spatial-authority-boundary.md)
