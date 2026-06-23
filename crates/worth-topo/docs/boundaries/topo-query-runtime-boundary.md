<!-- worth-doc
crate: worth-topo
kind: boundary
id: topo-query-runtime-boundary
query_integration_required: false
query_proof_required: false
touches_query: true
-->

# Topo Query Runtime Boundary

## Boundary

This boundary explains how topology-domain meaning sits on top of Query-owned
runtime posture.

## Allowed Upstream Inputs

- Query-owned runtime basis and admission posture
- admitted topology-domain read or edit requests

## Required Downstream Outputs

- topology-domain execution meaning
- typed runtime-support and executed-proof surfaces that do not drift into one
  another

## Stable Entry Points

- `worth_topo::runtime_support`
- `worth_topo::facade`

## Query Usage

Query owns runtime lifecycle and support posture. `worth-topo` owns the
topology-domain interpretation that sits on that runtime path.

## Forbidden Bypasses

- treating runtime admission as if it were executed topology proof
- building topology runtime folklore outside Query and the public topology
  support surfaces

## Binding Artifacts Or Receipts

The important retained artifacts at this boundary are:

- Query admission and support posture rows
- topology runtime-support rows
- topology domain-read request, aggregate, proof, and closeout reports

## Related Docs

- [Topology Authority Overview](../foundations/topology-authority-overview.md)
- [Domain Reads](../features/domain-reads.md)
- [Runtime Support](../features/runtime-support.md)
