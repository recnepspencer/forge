<!-- worth-doc
crate: worth-topo
kind: feature
id: runtime-support
query_integration_required: true
query_proof_required: false
touches_query: false
-->

# Topology Runtime Support

## What This Feature Is

`TopologyRuntimeSupport` is the public topology admission boundary over the
surviving bridge-backed Query runtime posture.

## Why You Use It

Use this when you need to know whether a topology-domain family or lane is
admitted on the current runtime basis before you ask what an executed read or
edit actually proved.

## Stable Entry Points

- `TopologyRuntimeSupport::runtime_posture_rows()`
- `TopologyRuntimeSupport::query_read_family_support_rows()`
- `TopologyRuntimeSupport::query_edit_family_support_rows()`
- `TopologyRuntimeSupport::query_edit_lane_support_rows()`

`TopologyRuntimeSupport` is the public admission boundary for the bridge-backed
Query runtime that survives the rewrite.

Use it when you need to know:

- which runtime postures are admitted on the current runtime basis
- whether a public topology-domain read family is admitted
- whether a public topology-domain edit family is admitted, partially admitted
  by named lanes, or denied
- which named query-native edit lanes are admitted
- whether the surviving runtime posture satisfies the public closeout contract

## Runtime Posture

Runtime posture is exposed through:

- `TopologyRuntimeSupport::runtime_posture_rows()`
- `TopologyRuntimeSupport::runtime_posture_status(...)`

This typed posture matrix replaces the older boolean helper layer.

## Public Read Support

Read support is exposed through:

- `TopologyRuntimeSupport::query_read_family_support_rows()`
- `TopologyRuntimeSupport::query_read_family_support_status(...)`

This matrix uses the same public family identity as the executed-read facade:
`TopologyReadRequestFamily`.

The snapshot read-only runtime admits those same public topology-domain read
families through the historical basis-aware posture.

## Public Edit Support

Edit support is exposed through:

- `TopologyRuntimeSupport::query_edit_family_support_rows()`
- `TopologyRuntimeSupport::query_edit_family_support_status(...)`
- `TopologyRuntimeSupport::query_edit_lane_support_rows()`
- `TopologyRuntimeSupport::query_edit_lane_support_status(...)`

Family rows answer whether a family is admitted. Lane rows answer which lane is
admitted and what execution shape it uses.

## Common Path

1. Inspect runtime posture rows.
2. Inspect read-family or edit-family support rows.
3. Inspect lane rows if a family is only partially admitted by lane.
4. Use `closeout()` when you need the public runtime closeout contract.

## Advanced Path

Use the advanced path when you need to distinguish:

- current-head versus snapshot posture
- family admission versus lane admission
- public runtime support versus executed domain proof

## Closeout

`TopologyRuntimeSupport::closeout()` is the public runtime closeout artifact
for this boundary.

It freezes:

- `BridgeBackedRuntimePath`
- `QueryNativeTopologyReadFamilies`
- `QueryNativeTopologyEditFamilies`
- `QueryNativeGraphComposedEditLanes`
- `MirrorReadDeletion`

If you need executed proof, parity, or decoded-view closeout rather than
runtime admission, read [Domain Reads](./domain-reads.md) next.

## Query Integration

Query owns the runtime lifecycle and support posture substrate. `worth-topo`
owns the topology-domain admission view published on top of that substrate.

This doc is about admission, not executed proof.

## Inspection And Debugging

Use this surface first when the question is "was this family admitted?" rather
than "what did the request prove?"

## Anti-Patterns

- inferring support from visible method names
- treating runtime-support admission as the same thing as executed proof
- rebuilding lane support folklore outside the typed rows

## Current Limits

This doc covers the currently declared public topology runtime-support surfaces
only. Unsupported or denied postures must stay explicit.

## Related Docs

- [Topology Graph Authority](./topology-graph-authority.md)
- [Topology Certification And Parity](./topology-certification-and-parity.md)
- [Domain Reads](./domain-reads.md)
- [Topo Query Runtime Boundary](../boundaries/topo-query-runtime-boundary.md)
