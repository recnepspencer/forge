# Milestone 2 Closeout: Derived Topology Materialization, Bridge-Causal Invalidation, And Rebuildable Interpretation

## Status

Milestone 2 is complete.

This closeout records the proof surfaces and architectural decisions that now
constitute completion of:

- [milestone-2.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/worth/milestone-2.md)
- [test-requirements.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/worth/test-requirements.md)

## What Closed

Milestone 2 closes with one honest derived-topology read pipeline over the
Milestone 1 authoritative truth substrate:

- one explicit proof-bearing derived read basis
- one schema-owned truth-to-derived invalidation vocabulary
- one bridge-lowered invalidation mapping pack
- one explicit materialized topology view layer
- one explicit interpreted topology view layer
- one phase-attributed derived-validator layer
- one explicit equivalence-contract surface for parity and reuse claims
- one explicit invalidation, rebuild, fallback, and failure-locality diagnostic surface
- one corpus-shaped bridge, replay, and branch-local parity proof program

## Closeout Evidence

The official Milestone 2 closeout proof surface is the machine-checkable
closeout artifact emitted by `worth-topo` certification.

Primary evidence classes:

- materialized topology digest
- interpreted topology digest
- derived validation digest
- derived truth-basis digest
- bridge routing digest
- bridge historical evaluation digest
- derived family coverage matrix
- derived family parity matrix
- derived validator coverage report
- derived invalidation aggregate report
- derived rebuild aggregate report
- derived equivalence-contract aggregate report
- derived fallback aggregate report
- derived failure-locality report
- derived branch-local parity report
- derived replay parity report
- derived bridge family coverage report
- milestone 2 counter report

## Derived-Family Closure

Milestone 2 is closed over the admitted derived-read families inherited from
Milestone 1:

- `WireOpen(n)`
- `WireClosed(n)`
- `WireBranch(k)`
- `SheetDisk(n)`
- `SheetPatch(f)`
- `SolidShell(f)`
- `NmtEdgeFan(k)`

For each admitted family, closeout includes:

- canonical role coverage:
  - `Smallest`
  - `Generic`
  - `HostileAdmitted`
  - `OutOfClass`
- materialization / interpretation / validation execution
- family-attributed derived parity
- family-attributed derived validator coverage
- family-attributed failure locality
- bridge-family proof coverage

## Architectural Outcome

Milestone 2 now freezes the first honest Worth derived-topology architecture:

- authority remains in relational truth
- invalidation vocabulary remains schema-owned
- bridge routing owns truth-to-derived causality
- materialization, interpretation, validation, parity, and diagnostics are separate derived phases
- derived topology remains destroyable and rebuildable from authority plus declared contracts alone
- milestone closeout is still driven by declared certification requirements rather than helper drift

The Worth-side target shape remains:

- `fixtures/`
- `phase_harness/`
- `certification_core/`
- `domain_certification/`
- `milestones/`

Reference:
- [forge_test_architecture.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/engineering/forge_test_architecture.md)

## Allowed Debt After Closeout

Milestone 2 closes with explicit debt only in areas that were out of scope for
the milestone:

- topology editing semantics
- geometry binding and topology-to-geometry identity
- feature graphs and regeneration
- bridge writeback authority
- broad optimization beyond explicit, counted fallback policies
- unsupported non-manifold classes already excluded by Milestone 1

These remain future roadmap work. They are not hidden incompleteness in the
Milestone 2 derived read basis, invalidation, parity, diagnostic, or closeout
surfaces.

## Verification Snapshot

Closeout was verified against the active implementation and certification
surfaces with:

- `cargo test -p worth-topo`
- `cargo test -p worth-schema`

## Next Active Milestone

With Milestone 2 closed, the active roadmap target becomes:

- [Milestone 3: Topology Editing Core](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/worth/worth_roadmap.md)
