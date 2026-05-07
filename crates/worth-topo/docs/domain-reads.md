# Worth Topology Domain Reads

`worth-topo` owns the topology-facing read facade on top of the generic
`forge-query` read-composition kernel.

The current migrated families are:

- `HalfEdgeSharedVertexNeighborhood`
- `HalfEdgeRadialNeighborhood`
- `LoopCycleNeighborhood`

## Current Posture

`HalfEdgeSharedVertexNeighborhood`, `HalfEdgeRadialNeighborhood`, and
`LoopCycleNeighborhood` now lower to Query-owned read families and execute
through the `forge-query` read kernel with:

- execution engine: `query_runtime_current`
- fallback posture: `whole_view_debt`

For the two local half-edge adjacency families:

- `HalfEdgeSharedVertexNeighborhood` executes through the operator-owned
  `SharedEndpoint` read surface over the vertex endpoint relations
- `HalfEdgeRadialNeighborhood` executes through the operator-owned
  `SharedAttachment` read surface over the radial-next and edge-attachment
  relations

In both cases, the executed scope, query digest, traversal breadth, and
relationship-proof admission are Query-backed at the returned boundary, and the
remaining debt is explicit: final neighborhood classification still uses a
Worth-owned whole-view decode helper after Query execution has already
succeeded.

For multi-hop cycle reads, the current migrated lane executes through the
operator-owned `FrontierSearch` read surface over the successor relation, so the
execution scope, query digest, traversal breadth, and relationship-proof
admission are now Query-backed at the returned boundary. The remaining debt is
explicitly narrower: final cycle ordering still uses a Worth-owned whole-view
decode helper after Query execution has already succeeded.

The remaining topology-domain family is not migrated yet and remains explicit
snapshot fallback debt:

- `LocalRewireNeighborhood`

## Rules

- external callers should depend on the topology-domain facade, not on local
  row joins or direct `snapshot_index` helpers
- if a family is still fallback-backed, the returned request report must say so
  explicitly
- if a family is Query-backed, the returned request report must expose that
  query-native execution count directly
- decoded topology views remain derived and disposable; callers must issue a
  new request instead of mutating a returned neighborhood
