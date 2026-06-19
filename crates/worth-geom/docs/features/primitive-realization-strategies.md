<!-- worth-doc
crate: worth-geom
kind: feature
id: primitive-realization-strategies
query_integration_required: false
query_proof_required: false
touches_query: false
-->

# Primitive Realization Strategies

## What This Feature Is

This feature owns the geometry-side realization strategies, conditioning
witnesses, stability classes, and exhaustion truth for primitive realization.

## Why You Use It

Use this when you need to know how primitive geometry is realized and why one
sanctioned realization strategy succeeded, degraded, or exhausted.

## Stable Entry Points

- `worth_geom::facade::{PrimitiveRealizationStrategy, PrimitiveRealizationReport}`
- `worth_geom::facade::{PrimitiveConditioningWitness, PrimitiveStabilityClass}`
- `worth_geom::facade::{PrimitiveRealizationExhaustionReport, PrimitiveRealizationError}`
- `worth_geom::facade::{realize_block_support, realize_prism_support, realize_tetrahedron_support}`

## Common Path

1. Choose the sanctioned primitive realization family.
2. Realize the geometric support and inspect the conditioning / stability
   posture.
3. Hand the realized geometry upward to kernel, spatial, and topology layers.

## Advanced Path

Use the advanced path when you need to inspect exhaustion witnesses, alternate
tightening strategies, or direct realization reports for hostile proof work.

## Inspection And Debugging

Reach for this surface when a primitive workflow is geometrically unstable or
exhausted before the higher-layer proof says anything meaningful.

## Anti-Patterns

- teaching primitive realization as one hidden implementation path
- flattening exhaustion into generic unsupported errors
- diagnosing primitive failure only from kernel artifacts without checking the
  geometry strategy lane

## Current Limits

This doc backfills the geometry realization substrate that later Milestone 4
and 5.x kernel work already depends on.

## Related Docs

- [Primitive Construction](../../../worth-kernel/docs/features/primitive-construction.md)
- [Construction Results And Diagnostics](../../../worth-kernel/docs/features/construction-results-and-diagnostics.md)
- [Analytic Primitives And Planes](./analytic-primitives-and-planes.md)
