<!-- worth-doc
crate: worth-topo
kind: feature
id: domain-reads
query_integration_required: true
query_proof_required: false
touches_query: false
-->

# Topology Domain Reads

## What This Feature Is

Topology domain reads are the public, topology-owned read facade on top of the
generic `forge-query` read-composition kernel.

## Why You Use It

Use this when you need executed topology-domain read meaning, per-request proof,
or closeout posture instead of only runtime admission.

## Stable Entry Points

- `topology_query_domain_entry(&query)`
- `TopologyCurrentHeadReadHandleExt::topology_reads(...)`
- `TopologySnapshotReadOnlyReadHandleExt::topology_reads(...)`

The topology crate owns the topology-facing read facade on top of the generic
`forge-query` read-composition kernel.

The public executed-read boundary is the admitted-handle read session built
from:

- `topology_query_domain_entry(&query)`
- `topology_current_head_authoritative_context()`
- `topology_snapshot_read_only_context()`
- `TopologyCurrentHeadReadHandleExt::topology_reads(...)`
- `TopologySnapshotReadOnlyReadHandleExt::topology_reads(...)`

Certification-owned proof aggregation is a separate internal support concern.
When a proof needs cross-workspace replay or branch-local accumulation, it may
use the certification harness in `crate::certification::support`. That harness
is not part of the topology read facade and must not replace admitted-handle
sessions for ordinary single-workspace read execution or proof authoring.

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

The historical executed-read path remains explicit: execution engine: `query_runtime_historical`.

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
blocked row. `phase_three_ready()` is true only when both the phase-three
blocker matrix is clear and every no-N-plus-one contract row is satisfied.

## Common Path

1. Admit a typed topology-domain read handle through Query domain entry.
2. Open the current-head or snapshot read session.
3. Issue the topology-domain read through the session.
4. Inspect the returned request report and the aggregate / proof / closeout
   reports when you need broader posture.

## Advanced Path

Use the advanced path when you need to inspect:

- execution engine versus fallback posture
- relationship-proof admission
- replay and branch-local parity
- no-N-plus-one closeout rows
- blocker rows that prevent phase-ready status

## Query Integration

Query owns the runtime read kernel, basis posture, and admitted-handle session
mechanics. `worth-topo` owns the topology-domain meaning of the returned read
surface and its executed proof boundary.

This is different from runtime support:

- runtime support answers whether a family is admitted
- domain reads answer what was actually executed and proved

## Inspection And Debugging

Inspect the request report first. Use the aggregate, proof, and closeout
surfaces when the bug looks structural rather than request-local.

If you need runtime admission rather than executed proof, read
[Runtime Support](./runtime-support.md) next.

## Anti-Patterns

- treating runtime admission as proof that a read executed honestly
- rebuilding topology neighborhoods through local row joins
- using certification-only harnesses as the ordinary public read path

## Current Limits

These docs describe the currently migrated public topology-domain read families
only. Unmigrated or fallback-backed neighbors must stay explicit.

## Related Docs

- [Topology Graph Authority](./topology-graph-authority.md)
- [Topology Certification And Parity](./topology-certification-and-parity.md)
- [Runtime Support](./runtime-support.md)
- [Topo Query Runtime Boundary](../boundaries/topo-query-runtime-boundary.md)
