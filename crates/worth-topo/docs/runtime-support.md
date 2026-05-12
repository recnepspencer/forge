# Topology Runtime Support

`TopologyRuntimeSupport` is the public admission boundary for the
bridge-backed Query runtime that survives the rewrite.

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

This typed posture matrix replaces the older boolean helper layer. The public
runtime boundary now freezes posture using named capabilities such as:

- `CurrentHeadLiveReads`
- `CurrentHeadMaterialization`
- `PostWriteMaterialization`
- `HistoricalBasis`
- `AuthoritativeWrites`

These rows are exhaustive for the declared runtime posture capability set. The
runtime support boundary treats omission as a construction bug, not as an
implicit `Denied`.

## Public Read Support

Read support is exposed through:

- `TopologyRuntimeSupport::query_read_family_support_rows()`
- `TopologyRuntimeSupport::query_read_family_support_status(...)`

This matrix now uses the same public family identity as the executed-read
facade: `TopologyDomainQueryRequestFamily`. Runtime admission and domain
execution no longer maintain separate read-family taxonomies.

The current-head runtime admits:

- `HalfEdgeSharedVertexNeighborhood`
- `HalfEdgeRadialNeighborhood`
- `LoopCycleNeighborhood`
- `LocalRewireNeighborhood`

The snapshot read-only runtime admits those same public topology-domain
families through the existing Forge Query historical/query-context surface.
Execution is not treated as current-head: receipts are expected to report the
historical query runtime engine and retain the executed basis digest plus the
snapshot token that was admitted for the read-only runtime.

## Public Edit Support

Edit support is exposed through:

- `TopologyRuntimeSupport::query_edit_family_support_rows()`
- `TopologyRuntimeSupport::query_edit_family_support_status(...)`
- `TopologyRuntimeSupport::query_edit_lane_support_rows()`
- `TopologyRuntimeSupport::query_edit_lane_support_status(...)`

The runtime support boundary no longer exposes stringly lane checks or
secondary "is execution supported?" convenience helpers. Callers are expected
to inspect the typed family/lane rows directly.

Family support rows describe whether a family is:

- `Admitted`
- `PartiallyAdmittedByLane`
- `Denied`

Lane support rows additionally freeze:

- the lane identity
- whether the lane is admitted
- whether execution is `ScalarMutation` or `GraphComposition`

The current-head runtime currently admits composed operator lanes such as:

- `CreateInnerLoopOnExistingFace`
- `RehomeAllOwnedHalfEdgesToNewWire`
- `SplitConnectedHalfEdgeSetIntoNewWire`
- `SplitSingleFaceFromTwoFaceShellToNewShell`
- `RehomeAllOwnedFacesToNewShell`
- `RelocateHalfEdgeBeforeSuccessor`
- `RelocateHalfEdgeSpanBeforeSuccessor`

The snapshot read-only runtime denies all authoritative edit families and lanes.

The typed family and lane rows are exhaustive for the currently declared public
runtime support surfaces. Missing rows are treated as a runtime-support
construction bug rather than a soft denial.

## Closeout

`TopologyRuntimeSupport::closeout()` is the public runtime closeout
artifact for this boundary.

It currently freezes:

- `BridgeBackedRuntimePath`
- `QueryNativeTopologyReadFamilies`
- `QueryNativeTopologyEditFamilies`
- `QueryNativeGraphComposedEditLanes`
- `MirrorReadDeletion`

This closeout is derived from the admitted read/edit support matrices. It is a
public admission and deletion contract, not a replacement for the deeper
domain proof ledger or milestone certification reports.

That means this surface may say a family is admitted on a runtime posture
without restating the deeper execution-proof claims that belong to
`TopologyDomainQuery` request, proof, and closeout artifacts. For snapshot
read-only posture, those deeper request reports are the authority for whether
the executed receipt was actually basis-honest.

If a caller needs to know why the executed-read proof boundary is or is not
ready for phase-three closure, that caller must inspect the typed
`TopologyDomainQuery::closeout_report()` family rows and phase-three
blocker rows rather than asking the runtime admission surface to summarize
proof it does not own.

The bridge-backed runtime support profile still exposes live read declarations
on snapshot posture, but snapshot posture does not admit preview or
branch-local sessions; topology-domain reads are admitted only through the
historical snapshot query-basis context for that read-only runtime.
