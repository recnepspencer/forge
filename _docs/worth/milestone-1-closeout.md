# Milestone 1 Closeout: NMT Topology Truth, Persistent Naming, And Validation Authority

## Status

Milestone 1 is complete.

This closeout records the proof surfaces and architectural decisions that now
constitute completion of:

- [milestone-1.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/worth/milestone-1.md)
- [test-requirements.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/worth/test-requirements.md)

## What Closed

Milestone 1 closes with one honest authoritative topology and naming substrate:

- authoritative Worth topology truth in `forge-relational`
- authoritative persistent naming truth
- same-commit graph creation for topology entities and relations
- commit-boundary runtime invariants for admitted topology legality
- derived topology interpretation as a separate, rebuildable layer
- branch-local and replay-aware certification over the admitted surface
- bridge-causal proof from committed truth into derived historical evaluation

## Closeout Evidence

The official Milestone 1 closeout proof surface is the machine-checkable
closeout artifact emitted by `worth-topo` certification.

Primary evidence classes:

- topology truth digest
- naming truth digest
- topology validation digest
- topology validation aggregate report
- topology localization aggregate report
- naming attachment aggregate report
- primitive family coverage matrix
- primitive corpus parity report
- admitted-range sweep report
- validator coverage report
- branch-local topology aggregate report
- replay parity aggregate report
- rejection class report
- failure locality report
- bridge family coverage report
- bridge proof report
- milestone counter report

## Primitive-Family Closure

Milestone 1 is closed over the admitted topology families:

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
- admitted-range sweep coverage
- branch-local parity
- replay parity
- family-attributed validator coverage
- family-attributed rejection and failure locality
- bridge-family proof coverage

## Architectural Outcome

Milestone 1 now freezes the first honest Worth topology architecture:

- shared harness layers own test grammar and execution structure
- Worth owns topology and certification meaning
- fixtures are phase-oriented rather than milestone-local convenience setup
- closeout is driven by declared certification requirements rather than helper drift

The Worth-side target shape is now:

- `fixtures/`
- `phase_harness/`
- `certification_core/`
- `domain_certification/`
- `milestones/`

Reference:
- [forge_test_architecture.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/engineering/forge_test_architecture.md)

## Allowed Debt After Closeout

Milestone 1 closes with explicit debt only in areas that were out of scope for
the milestone:

- broad topology edit-operator catalogs
- geometry binding and topology-to-geometry continuity
- boolean execution
- blends, features, and regeneration
- unsupported non-manifold classes outside the admitted class

These remain future roadmap work. They are not hidden incompleteness in the
Milestone 1 authority, validator, replay, or certification surfaces.

## Verification Snapshot

Closeout was verified against the active implementation and certification
surfaces with:

- `cargo test -p worth-topo`
- `cargo test -p worth-schema`
- `cargo test -p forge-relational`

## Next Active Milestone

With Milestone 1 closed, the active roadmap target becomes:

- [Milestone 2: Derived Topology Materialization And Bridge-Causal Validation](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/worth/worth_roadmap.md)
