<!-- worth-doc
crate: worth-spatial
kind: boundary
id: spatial-to-topo
query_integration_required: false
query_proof_required: false
touches_query: false
-->

# Spatial To Topo

## Boundary

This boundary explains where spatial semantics stop and topology authority
begins.

## Allowed Upstream Inputs

- spatial birth bindings and spatial truth artifacts
- typed impossibility or completeness postures

## Required Downstream Outputs

- topology-safe inputs for `worth-topo`
- no ambiguity about whether the next authority question is spatial or topology

## Stable Entry Points

- upstream: `worth_spatial::facade`
- downstream: `worth_topo::facade`

## Query Usage

Query may carry the runtime lane, but this handoff is not a Query-owned
semantic decision. It is the spatial-to-topology authority transition.

## Forbidden Bypasses

- smuggling topology authority into spatial helper logic
- reconstructing spatial meaning from topology-only artifacts

## Binding Artifacts Or Receipts

The important retained outputs are the spatial truth artifacts that explain
what the topology authority is consuming.

## Related Docs

- [Construction-Time Birth Bindings](../features/construction-time-birth-bindings.md)
- [worth-topo Topology Authority Overview](../../../worth-topo/docs/foundations/topology-authority-overview.md)
