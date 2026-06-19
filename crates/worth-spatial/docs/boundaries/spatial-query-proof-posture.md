<!-- worth-doc
crate: worth-spatial
kind: boundary
id: spatial-query-proof-posture
query_integration_required: false
query_proof_required: false
touches_query: true
-->

# Spatial Query Proof Posture

## Boundary

This boundary explains how `worth-spatial` depends on Query proof posture
without claiming Query runtime authority as a spatial-owned concern.

## Allowed Upstream Inputs

- Query-owned runtime execution and support posture
- Query-owned proof posture when a spatial surface needs to prove Query
  consumption honestly

## Required Downstream Outputs

- spatial semantics that remain attached to the admitted Query runtime lane
- explicit proof ownership when a doc or artifact depends on Query evidence,
  hard prohibitions, support pinning, or adoption closure

## Stable Entry Points

- `worth_spatial::facade::query_adoption`
- `worth_spatial::facade::support`

## Query Usage

Use Query-owned proof surfaces for:

- evidence-report identity
- hard-prohibition and boundary-audit posture
- support snapshots and support pinning
- reference-consumer adoption closure

## Forbidden Bypasses

- local synthetic proof that duplicates Query-owned lanes
- documenting Query posture as if it were just another spatial helper family

## Binding Artifacts Or Receipts

The important retained proof artifacts at this boundary are:

- Query-owned evidence-report identities
- Query-owned boundary-audit outputs
- Query-owned support snapshots and support-pin rows

## Related Docs

- [Spatial Overview](../foundations/spatial-overview.md)
- [Worth To Query](../../../worth-kernel/docs/boundaries/worth-to-query.md)
