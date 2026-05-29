# Topology Domain Reads

The topology crate owns the topology-facing read facade on top of the generic
`forge-query` read-composition kernel.

The public executed-read boundary is the admitted-handle read session built
from:

- `topology_query_domain_entry(&query)`
- `topology_current_head_authoritative_context()`
- `topology_snapshot_read_only_context()`
- `TopologyCurrentHeadReadHandleExt::topology_reads(...)`
- `TopologySnapshotReadOnlyReadHandleExt::topology_reads(...)`

External callers can:

- admit a typed topology configured handle through Query domain entry
- open a handle-bound read session against a `ForgeQueryWorkspace`
- issue topology-neighborhood reads through the typed session methods
- inspect per-request execution reports from the returned views
- inspect aggregate, proof, and closeout posture through the session
  `aggregate_report()`, `proof_report()`, and `closeout_report()` surfaces

The public closeout report now exposes:

- per-family executed-read closeout rows
- per-family closeout reasons and stable row digests
- explicit phase-three blocker rows
- no-N-plus-one contract rows named
  `topology_read_lowering_breadth`,
  `topology_read_fallback_posture`,
  `topology_read_view_parity`, and
  `topology_read_relationship_proof_posture`
- typed status lookup for family, blocker, and no-N-plus-one contract surfaces

The current migrated families are:

- `HalfEdgeSharedVertexNeighborhood`
- `HalfEdgeRadialNeighborhood`
- `LoopCycleNeighborhood`
- `LocalRewireNeighborhood`

The topology-domain boundary now also exposes one typed aggregate proof surface
over those migrated families:

- scope-class counts
- execution-engine counts
- fallback and debt rows
- locality-claim versus executed-scope matching
- replay / branch-local parity and decoded-view determinism counts

The no-N-plus-one contract rows are the machine-checkable closeout surface for
the side quest rule that recurring reads must not hide broad or repeated
rediscovery work behind domain-shaped helpers. A contract row is satisfied only
when the relevant proof was observed at the domain read boundary; missing
replay or branch-local decoded-view parity, fallback debt, scope mismatch, or
traversal without matching relationship-proof admission remains visible as a
blocked row.
`phase_three_ready()` is true only when both the phase-three blocker matrix is
clear and every no-N-plus-one contract row is satisfied.

The public runtime support boundary now also freezes the query-native
runtime posture through:

- `TopologyRuntimeSupport::runtime_posture_rows()`
- `TopologyRuntimeSupport::runtime_posture_status(...)`
- `TopologyRuntimeSupport::query_read_family_support_rows()`
- `TopologyRuntimeSupport::query_read_family_support_status(...)`
- `TopologyRuntimeSupport::query_edit_family_support_rows()`
- `TopologyRuntimeSupport::query_edit_lane_support_rows()`
- `TopologyRuntimeSupport::closeout()`

That surface exists so external callers can inspect which topology-domain read
and edit families are admitted on the surviving bridge-backed runtime and
whether the public deletion posture is satisfied, without reaching into
internal proof helpers. It is a public runtime admission/deletion contract,
not a replacement for the deeper domain proof ledger. The runtime-support
surface is documented directly in [runtime-support.md](runtime-support.md).

The runtime read-support matrix and the executed-read facade now share the same
typed family universe through `TopologyDomainQueryRequestFamily`, so
runtime admission and domain execution cannot drift by naming the same family
set twice.

The two public surfaces still answer different questions:

- `TopologyRuntimeSupport` answers whether a family is admitted on a
  runtime posture
- `TopologyDomainQuery` answers what execution, proof, parity, and
  closeout facts were actually observed on executed topology reads

## Current Posture

`HalfEdgeSharedVertexNeighborhood`, `HalfEdgeRadialNeighborhood`,
`LoopCycleNeighborhood`, and `LocalRewireNeighborhood` now lower to Query-owned
read families and execute
through the `forge-query` read kernel with:

- execution engine: `query_runtime_current`
- request fallback posture:
  - `HalfEdgeSharedVertexNeighborhood`: `none`
  - `HalfEdgeRadialNeighborhood`: `none`
  - `LoopCycleNeighborhood`: `none`
  - `LocalRewireNeighborhood`: `none`

When the same families execute on a snapshot read-only runtime, the domain
read path binds them to the runtime snapshot token through Forge Query's
historical/query-context surface and calls the basis-aware read-family executor.
Those request reports should expose:

- execution engine: `query_runtime_historical`
- executed basis digest: present
- executed snapshot token: the read-only runtime snapshot token
- fallback count: `0`

The underlying `TopologyDomainQuery` kernel remains request-only and does not
preload whole-view topology state, but its workspace-taking neighborhood
methods are now an internal adapter seam rather than the public executed-read
entry. The active topology families no longer depend on a hidden bootstrap
authority before they can decode their final views.

For the two local half-edge adjacency families:

- `HalfEdgeSharedVertexNeighborhood` executes through the operator-owned
  `SharedEndpoint` read surface over the vertex endpoint relations
- `HalfEdgeRadialNeighborhood` executes through the operator-owned
  `SharedAttachment` read surface over the radial-next and edge-attachment
  relations

In both cases, the executed scope, query digest, traversal breadth, and
relationship-proof admission are Query-backed at the returned boundary, and the
returned rows now retain the endpoint / edge relation materialization needed to
decode the final neighborhood directly from Query output.

For multi-hop cycle reads, the current migrated lane executes through the
operator-owned `FrontierSearch` read surface over the successor relation, and
the returned rows now retain the successor relation materialization needed to
decode the final cycle directly from Query output. The execution scope, query
digest, traversal breadth, relationship-proof admission, and final cycle decode
are now all Query-backed at the returned request boundary.

For local-rewire reads, the current migrated lane executes through the generic
anchored collection surface with the same successor-cycle plus predecessor
traversal shape the lowering artifact already certifies. The execution scope,
query digest, traversal breadth, and relationship-proof admission are
Query-backed at the returned boundary, and the returned rows now retain the
successor / predecessor relation materialization needed to decode the final
local-rewire neighborhood directly from Query output.

## Rules

- external callers should depend on the topology-domain facade, not on local
  row joins or explicit test/certification lookup helpers
- if a family is still fallback-backed, the returned request report must say so
  explicitly
- if a family is Query-backed, the returned request report must expose that
  query-native execution count directly
- if a caller needs the aggregate posture across multiple migrated reads, it
  should inspect the domain proof / closeout report instead of inferring from
  individual helper calls
- if a caller needs the Milestone 3 resume gate, it should inspect the
  hostile-suite return gate fields; scenario coverage and side-quest closeout
  readiness are separate inputs, and `milestone_three_return_gate_blocker_rows`
  names the remaining blocker rows directly
- if a caller needs to know why executed-read closeout is or is not ready, it
  should inspect the typed closeout family rows, phase-three blocker rows, and
  no-N-plus-one contract rows
  instead of reverse-engineering the answer from scalar counters alone; family
  rows now carry explicit reasons and structural row digests so callers do not
  have to reconstruct the meaning of `Unobserved`, `ExecutionGap`, or
  `QueryExecutedWithDebt` from raw counters
- if a caller only needs the public runtime admission/deletion contract, it
  should inspect `TopologyRuntimeSupport::query_read_family_support_rows()`
  and `TopologyRuntimeSupport::closeout()` rather than reaching into
  domain-only proof artifacts; callers that need the edit-side runtime
  contract should inspect the typed edit family/lane support rows on
  `TopologyRuntimeSupport`, and callers that need runtime-basis posture
  should inspect the typed runtime posture rows rather than relying on boolean
  convenience helpers
- decoded topology views remain derived and disposable; callers must issue a
  new request instead of mutating a returned neighborhood
