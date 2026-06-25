# Milestone 3 Closeout: Topology Editing Core

## Status

Milestone 3 is complete.

This closeout records the proof surfaces and architectural decisions that now
constitute completion of:

- [milestone-3.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/worth/milestone-3.md)
- [test-requirements.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/worth/test-requirements.md)
- [topo-test-requirements.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/worth/topo-test-requirements.md)

## What Closed

Milestone 3 closes with one honest topology-only edit substrate over the
Milestone 1 authority and Milestone 2 derived-read foundations:

- one topology-operator boundary in `worth-topo`
- typed topology edit contracts and proof-carrying edit batches
- admitted operator-family closure for lifecycle, boundary wiring, shell/wire
  membership, and radial adjacency edits
- explicit naming continuity outcomes for preserved, ambiguous, and rejected
  edit cases
- direct rejection locality surfaces for blocked topology edits
- accepted and rejected replay parity over every required hostile scenario
- accepted and rejected branch-local parity over every required hostile
  scenario, with accepted branch evidence tied to authority projection
- direct derived fallout, reuse, fallback, validation breadth, query traversal,
  primitive-family, operator-family, scale-pressure, and return-gate evidence

## Closeout Evidence

The official Milestone 3 closeout proof surface is the machine-checkable
closeout artifact emitted by `worth-topo` certification.

Primary evidence classes:

- topology edit digest rows
- naming edit continuity matrix rows
- rejected edit scope report rows
- edit replay parity rows
- edit branch-local parity rows
- replay and branch breadth rows
- validator-family coverage rows
- validation breadth rows
- changed-scope coverage rows
- derived-region coverage rows
- determinism rule rows
- edit breadth counter rows
- edit fallout breadth rows
- derived fallback policy denial rows
- derived reuse legality rows
- derived work breadth rows
- edited topology query traversal rows
- operator-family closure rows
- primitive-family closure rows
- scale-pressure rows
- hostile certification category rows
- failure-locality rows
- Milestone 3 return-gate blocker rows

## Hostile Scenario Closure

Milestone 3 is closed over the required hostile scenario families:

- `BowtieAdjacentRewire`
- `CancellationChainParity`
- `SplitCollapseChurn`
- `AmbiguousLocalRewireContinuity`
- `BrokenRadialLocalization`

The suite proves both accepted and rejected paths:

- accepted scenarios replay with matching edit and materialized topology
  evidence
- rejected scenarios preserve typed rejection and unchanged branch-local truth
- branch-local accepted rows are scenario-specific rather than count-only
- ambiguous continuity remains explicit instead of being silently upgraded to
  preserved continuity
- broken or illegal local topology localizes to typed rejection evidence

## Primitive And Operator Closure

Milestone 3 closes edit-family proof over the admitted primitive families:

- `WireOpen(n)`
- `WireClosed(n)`
- `SheetDisk(n)`
- `SheetPatch(f)`
- admitted `SolidShell(f)`

It also emits direct operator-family rows for the admitted Milestone 3 edit
surface:

- `CreateTopologyEntity`
- `RetireTopologyEntity`
- `AttachBoundaryMembership`
- `DetachBoundaryMembership`
- `RewireLoopSuccessor`
- `RewireLoopEndpoint`
- `AttachShellOrWireMembership`
- `DetachShellOrWireMembership`
- `SpliceRadialAdjacency`
- `DetachRadialAdjacency`

## Architectural Outcome

Milestone 3 now freezes the first trustworthy topology editing architecture:

- authority remains in relational truth
- topology editing remains geometry-free
- derived topology remains inspection and fallout evidence, not mutation
  authority
- branch-local and replay proofs are direct closeout rows, not helper-only
  nested artifacts
- projection surfaces expose authorized truth, derived products, and
  diagnostics without performing topology interpretation
- certification and validation remain separate: validators answer whether a
  topology state or edit result satisfies invariant families, while
  certification proves the edit, derived, projection, replay, branch, hostile,
  scale, and closeout system survives the required corpus

The Worth-side topology skeleton now preserves the domain story:

- `brep/`
- `derived_topology/`
- `validation/`
- `topology_operators/`
- `projection/`
- `certification/`
- `test_support/`

## Allowed Debt After Closeout

Milestone 3 closes with explicit debt only in areas outside the milestone:

- primitive and body construction programs
- topology-to-geometry binding
- spatial classification
- planar exactness
- boolean operators
- curve, surface, and trim semantics
- broader operator catalogs that require later milestone meaning

These are future roadmap surfaces. They are not hidden incompleteness in the
Milestone 3 topology-edit authority, replay, branch, diagnostics, or closeout
proof surfaces.

## Verification Snapshot

Closeout was verified against the active implementation and certification
surfaces with:

- `cargo fmt --check`
- `cargo test -p worth-topo branch_local_acceptance --quiet`
- `cargo test -p worth-topo replay_branch_breadth --quiet`
- `scripts/ci/check_worth_topo_milestone3_slow_certification.ps1`
- `cargo test -p worth-topo broad_direct_file_clusters_stay_explicitly_reviewed --quiet`
- `cargo test -p worth-topo --quiet`

The Milestone 3 topology-operator closeout is an explicit slow certification
gate, not part of the default worth-topo unit-test iteration lane. Default
iteration should use focused tests and `cargo test -p worth-topo --lib`; the
full closeout gate must still run before declaring topology-operator closeout
coverage intact after touching operator semantics, replay, branch-local
behavior, validation breadth, scale pressure, or closeout row contracts.

## Next Active Milestone

With Milestone 3 closed, the active roadmap target becomes:

- [Milestone 4: Topology-Certified Primitive Construction](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/worth/worth_roadmap.md)
