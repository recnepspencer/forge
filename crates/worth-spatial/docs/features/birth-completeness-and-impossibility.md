<!-- worth-doc
crate: worth-spatial
kind: feature
id: birth-completeness-and-impossibility
query_integration_required: true
query_proof_required: false
touches_query: false
-->

# Birth Completeness And Impossibility

## What This Feature Is

This feature explains how `worth-spatial` distinguishes a complete birth path
from an impossible or incomplete one.

## Why You Use It

Use this when you need to understand why a construction workflow could not
honestly produce spatial birth truth.

## Stable Entry Points

- `worth_spatial::facade::binding`
- `worth_spatial::facade::recovery`
- `worth_spatial::facade::inspection`

## Common Path

Impossible is not the same as absent. Incomplete is not the same as rejected
topology. Spatial owns those distinctions.

The runtime-backed workflow reaches spatial semantics, and spatial returns a
typed completeness or impossibility posture rather than collapsing the outcome
into generic rejection.

## Small Example

Use this when a run failed after runtime admission and you need to know whether
the missing step was a spatial impossibility or a different lower-layer issue.

## Advanced Path

This is one of the main surfaces that protects Worth from turning spatial
meaning into topology-only truth or caller-owned heuristics.

## Query Integration

This feature depends on the Query-backed runtime path reaching spatial meaning
honestly. Query owns the runtime lane; spatial owns the completeness and
impossibility semantics returned on that lane.

## How It Relates To Other Features

- [Birth Truth Artifacts](./birth-truth-artifacts.md)
- [Spatial To Topo](../boundaries/spatial-to-topo.md)

## Inspection And Debugging

Inspect the typed impossibility or completeness posture before you debug
topology certification or replay drift.

## Anti-Patterns

- flattening impossibility into a generic "unsupported"
- hiding the missing spatial witness behind local helper logic

## Current Limits

Only the admitted Milestone 4 birth completeness surface is documented here.

## Related Docs

- [Construction-Time Birth Bindings](./construction-time-birth-bindings.md)
