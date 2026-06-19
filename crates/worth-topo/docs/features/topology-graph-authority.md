<!-- worth-doc
crate: worth-topo
kind: feature
id: topology-graph-authority
query_integration_required: false
query_proof_required: false
touches_query: false
-->

# Topology Graph Authority

## What This Feature Is

This feature owns the core topology truth objects that the rest of
`worth-topo` interprets, validates, mutates, and certifies.

## Why You Use It

Use this when you need to understand the authoritative topology graph itself
rather than runtime admission, workload authoring, or executed read proof.

## Stable Entry Points

- `worth_topo::facade::{TopologyModel, TopologyBody, TopologyLump, TopologyRegion}`
- `worth_topo::facade::{TopologyShell, TopologyFace, TopologyLoop, TopologyWire}`
- `worth_topo::facade::{TopologyHalfEdge, TopologyEdge, TopologyVertex, TopologyView}`

## Common Path

1. Start from the topology graph types and `TopologyView`.
2. Use them as the authority inputs for validation, certification, or mutation.
3. Only then move outward into runtime-backed read or edit surfaces.

## Advanced Path

Use the advanced path when you need to reason about:

- body / lump / region ownership
- shell / face / loop / wire incidence
- half-edge / edge / vertex identity
- the difference between graph truth and derived interpretation

## Inspection And Debugging

Inspect this surface when a bug sounds like "what topology truth exists?" not
"was the runtime path admitted?" and not "what did the request prove?"

## Anti-Patterns

- treating interpreted topology as the same thing as topology truth
- explaining graph authority only through mutation workflows
- rediscovering graph ownership from diagnostics instead of the graph types

## Current Limits

This doc explains the stable topology graph substrate the Milestone 4 slice
inherits. It does not try to reteach every operator family.

## Related Docs

- [Topology Authority Overview](../foundations/topology-authority-overview.md)
- [Runtime Support](./runtime-support.md)
- [Topology Certification And Parity](./topology-certification-and-parity.md)

