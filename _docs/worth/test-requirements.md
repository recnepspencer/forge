# Worth Test Requirements, Part 1

## Purpose

This document defines the milestone-closeout proof bar for Worth roadmap
milestones `1` through `10`.

Worth proof is not complete when a demo works, when a happy-path model loads,
or when one benchmark part survives. Worth is only complete when the admitted
workflow surface for each milestone is certified as exact, explicit, or
clean-failing with machine-checkable evidence.

This document is part of the Worth test-document set:

- [test-requirements.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/worth/test-requirements.md)
  closes milestones `1` through `10`
- [test-requirements_pt2.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/worth/test-requirements_pt2.md)
  closes milestones `11` through `20`

## Governing Proof Rule

Every Worth milestone must prove all of the following at the boundary it
claims:

- authoritative truth legality
- derived recompute correctness where derivation exists
- branch and replay stability for the admitted workflow class
- persistent naming continuity or explicit continuity failure where naming is in
  scope
- explicit interaction intent preservation where intent is in scope
- diagnostics strong enough to explain the result mechanically
- exact success or exact structured failure, never crash, hang, or silent drift

## Acceptable Outcome Rule

Every hostile Worth test has exactly two acceptable outcomes:

- produce the correct result with machine-checkable proof artifacts
- fail cleanly with structured diagnostics that identify the exact trigger,
  affected scope, and boundary that rejected the operation

The following are never acceptable:

- crash
- hang
- non-deterministic replay
- silent topology corruption
- silent naming drift
- silent geometry drift
- silent intent reinterpretation
- silent heuristic auto-resolution where intent was ambiguous

## Milestone Mapping Rule

This proof document is milestone-shaped, not tier-shaped.

Each roadmap milestone must have a direct closeout section here.
Milestones are not closed by analogy, by neighboring suites, or by broad tier
claims. They are only closed by their own named proof section plus any
explicitly inherited lower-milestone obligations.

## Canonical Primitive Corpus Rule

Worth milestone proof may use example parts, but it may not be justified by
example parts.

To prove NMT, shells, surfaces, and solids honestly, Worth must maintain a
canonical primitive corpus made of parameterized families rather than a few
fixed shapes.

The rule is:

- a single tetrahedron does not prove solids
- a cube does not prove shell-building
- a triangle and quad do not prove loop handling
- one sheet face does not prove surface-topology closure
- one 3-face fan does not prove NMT

Instead, each primitive family below must be exercised as a family over
arbitrary admitted cardinality, valence, or face count within the milestone's
declared boundary.

### Topology primitive families

These are the minimum topology families Worth must eventually be able to
produce and certify to claim serious topology coverage:

- `WireOpen(n)`: an open wire chain with arbitrary admitted segment count
- `WireClosed(n)`: a closed wire loop with arbitrary admitted segment count
- `WireBranch(k)`: a wire branch or star with arbitrary admitted branch valence
- `SheetDisk(n)`: a single-face open sheet with one outer loop of arbitrary
  admitted cardinality
- `SheetAnnulus(n, h...)`: a single-face open sheet with one outer loop and one
  or more inner loops
- `SheetPatch(f)`: a multi-face open shell with arbitrary admitted face count
- `SolidShell(f)`: a closed genus-0 shell with arbitrary admitted face count
- `SolidWithVoid(f_outer, f_inner...)`: a closed outer shell with one or more
  inner void shells
- `MultiLumpBody(l)`: one body with multiple disconnected lumps
- `NmtEdgeFan(k)`: an edge shared by arbitrary admitted radial valence
- `NmtVertexPinch(d)`: a vertex supporting multiple admitted incident disks

### Surface and binding primitive families

These are the minimum carrier families needed before Worth can honestly claim
surface and geometry-binding coverage:

- `PlanarCarrier`: planar face / loop / shared-edge bindings
- `PeriodicAnalyticCarrier`: cylindrical or toroidal seam-bearing carriers
- `SingularAnalyticCarrier`: spherical or conical pole/tip-bearing carriers
- `TrimmedCarrier(o, h...)`: a carrier with one outer trim and one or more
  inner trims
- `SharedEdgeDualTrim`: two adjacent carriers sharing one bound edge with dual
  trims
- `ReboundCarrier`: one topology neighborhood rebound to a distinct geometry
  carrier while preserving or explicitly failing continuity

### Parameterized-family proof rule

Every primitive family must be tested as a family, not as one fixed instance.

Examples:

- `WireClosed(3)` alone does not prove `WireClosed(n)`
- one four-sided sheet does not prove `SheetPatch(f)`
- one cube and one prism do not prove `SolidShell(f)`
- one 3-face radial fan does not prove `NmtEdgeFan(k)`

For each admitted family, Worth must prove:

- construction over arbitrary admitted size or valence
- validator closure over arbitrary admitted size or valence
- replay parity over arbitrary admitted size or valence
- clean failure when the workflow exits the admitted class

For each admitted family, the proof set must include at minimum:

- the smallest non-degenerate admitted member
- at least one larger generic member that is not a showcase shape
- at least one hostile admitted member near the milestone's admitted boundary
- at least one explicit out-of-class member that must fail cleanly

### Milestone coverage expectation

The early roadmap must cumulatively cover these families like this:

- milestones `1` through `3`: `WireOpen`, `WireClosed`, `SheetDisk`,
  `WireBranch`, `SheetPatch`, `SolidShell`, and `NmtEdgeFan` across each
  milestone's admitted class
- milestones `4` and `5`: hostile planar proof over `SheetDisk`,
  `SheetPatch`, `SolidShell`, `SolidWithVoid`, and admitted planar NMT cases
- milestones `6` through `8`: carrier and rebinding proof over
  `PlanarCarrier`, `TrimmedCarrier`, `SharedEdgeDualTrim`, `ReboundCarrier`,
  and admitted analytic curved carriers

No milestone may claim closure on shells, solids, surfaces, or NMT while the
required primitive families for that milestone are still represented only by
toy examples.

## Milestone 1: NMT Topology Truth And Naming Foundation

### Purpose

Prove that Worth can represent shell, wire, loop, halfedge, radial, and
persistent-name truth authoritatively without collapsing truth, validation, and
continuity into overlapping pseudo-authority layers.

### Required workload surface

Run deterministic truth workloads containing:

- primitive-corpus coverage for:
  - `WireOpen(n)`
  - `WireClosed(n)`
  - `WireBranch(k)`
  - `SheetDisk(n)`
  - `SheetPatch(f)`
  - `SolidShell(f)`
  - `NmtEdgeFan(k)`
- simple closed loops of arbitrary admitted cardinality
- wire branches of arbitrary admitted branch valence
- shells with arbitrary admitted face counts
- closed solid shells with arbitrary admitted face counts
- wires with arbitrary admitted segment counts
- radial fanout spanning the admitted NMT valence class
- branch-local seeded histories and local admitted truth edits

Milestone-1 workload generation must stay inside the milestone's admitted class:

- orientable, genus-0 `SolidShell(f)` only
- `WireBranch(k)` only within the admitted branch-topology class
- no `NmtVertexPinch(d)`, void-shell, higher-genus, or non-orientable cases
  except as explicit clean-fail out-of-class tests

### Must verify

- commit-boundary invariants reject impossible local topology before
  publication
- commit-boundary invariants reject illegal wire-branch and illegal solid-shell
  states before publication
- derived topology interpretation classifies admitted shell, wire, boundary,
  wire-branch, solid-shell, and radial structure deterministically
- persistent names are authoritative truth, not regenerated labels
- rejected and accepted paths both emit replayable diagnostic evidence

### Required verification output

- topology_truth_digest
- topology_validation_digest
- naming_truth_digest
- topology_localization_report
- naming_attachment_report

### Closeout condition

Milestone `1` closes only when admitted topology truth and persistent naming
work generically across the milestone's admitted workflow surface, including
`WireBranch(k)` and `SolidShell(f)` as fully robust admitted families rather
than toy examples or partial subsets.

## Milestone 2: Derived Topology Materialization And Bridge-Causal Validation

### Purpose

Prove that topology materialization, invalidation, and derived topology
validation remain subordinate to truth and replay identically from the same
truth basis.

### Required workload surface

Run repeated executions over:

- primitive-corpus coverage for the milestone-1 admitted families under replay
  and branch-local reads
- arbitrary admitted seeded topology truth
- arbitrary admitted local topology edits from milestone `1`
- arbitrary admitted shell, wire, and radial counts
- branch-local reads, mainline reads, and replayed reads of the same truth
  history

### Must verify

- identical truth histories produce identical topology materializations
- bridge invalidation remains parity-safe and deterministic
- derived topology validators classify admitted structure identically across
  replay and branch-local reads
- no derived cache becomes authority

### Required verification output

- topology_view_digest
- bridge_routing_digest
- topo_diagnostics_digest
- replay_parity_report

### Closeout condition

Milestone `2` closes only when derived topology is a deterministic, rebuildable
layer over the admitted topology truth workflows.

## Milestone 3: Topology Editing Core

### Purpose

Prove that the first admitted topology-edit workflows preserve truth legality,
naming legality, and replay parity over general admitted edit classes.

### Required workload surface

Run admitted edit workloads containing:

- primitive-corpus edit coverage over:
  - `WireOpen(n)`
  - `WireClosed(n)`
  - `SheetDisk(n)`
  - `SheetPatch(f)`
  - admitted `SolidShell(f)`
- primitive lifecycle edits over arbitrary admitted entity counts
- loop and boundary rewiring over arbitrary admitted loop cardinalities
- shell and wire membership edits
- radial splice or reseating workflows within the admitted NMT class

### Must verify

- edit application preserves milestone `1` and `2` validator closure
- naming preservation or typed ambiguity is explicit for admitted edit classes
- rejected edits localize to the exact invariant or continuity boundary
- admitted edit histories replay identically

### Required verification output

- topology_edit_digest
- naming_edit_continuity_matrix
- rejected_edit_scope_report
- edit_replay_parity_report

### Closeout condition

Milestone `3` closes only when admitted topology edits operate generically
across the milestone's workflow surface instead of only on hand-built examples.

## Milestone 4: Planar Exactness And Structural Identity

### Purpose

Prove that exact-planar decisions and structural identity remain stable,
distinct, and trustworthy across the admitted planar workflow class.

### Required workload surface

Run planar workloads containing:

- primitive-corpus planar coverage for:
  - `SheetDisk(n)`
  - `SheetAnnulus(n, h...)`
  - `SheetPatch(f)`
  - admitted `SolidShell(f)`
  - admitted `SolidWithVoid(f_outer, f_inner...)`
- arbitrary admitted planar loop cardinalities
- arbitrary admitted shell face counts
- coplanar and thin-feature cases within the admitted exact-planar class
- replayed and branch-local planar histories

### Must verify

- admitted planar classifications remain exact
- structural identity remains distinct from naming and lineage
- impossible or policy-gated planar cases fail cleanly
- planar identity digests replay identically

### Required verification output

- exact_planar_decision_digest
- structural_identity_digest
- planar_clean_fail_report
- planar_identity_replay_report

### Closeout condition

Milestone `4` closes only when exact-planar and identity claims hold across the
admitted planar workflow surface, not just cubes, prisms, and sample booleans.

## Milestone 5: Hostile Planar Proof Program

### Purpose

Prove that hostile planar workloads either succeed exactly or fail with exact
localized proof.

### Required workload surface

Run hostile planar workloads containing:

- hostile primitive-corpus coverage for:
  - admitted `SheetPatch(f)` families
  - admitted `SolidShell(f)` families
  - admitted `SolidWithVoid(f_outer, f_inner...)` families
  - admitted planar `NmtEdgeFan(k)` families
- long planar histories of arbitrary admitted length
- cancellation and return-to-prior-state workflows
- coplanar overlap storms
- high-valence and scale-separated planar pressure within the admitted class

### Must verify

- topology drift is detected at the causal step where it begins
- accepted and rejected hostile planar cases replay identically
- no hostile planar workload crashes, hangs, or corrupts silently
- cancellation workflows return to parity-equivalent truth where justified

### Required verification output

- chain_truth_digest_series
- cancellation_parity_matrix
- planar_hostility_report
- causal_trigger_report

### Closeout condition

Milestone `5` closes only when hostile planar proof covers the admitted planar
workflow surface rather than a few benchmark chains.

## Milestone 6: Geometry Binding And Topology-To-Geometry Identity

### Purpose

Prove that topology-to-geometry binding is an honest truth surface and does not
collapse topology identity, geometry identity, and naming continuity together.

### Required workload surface

Run binding workloads containing:

- primitive-corpus carrier coverage for:
  - `PlanarCarrier`
  - `TrimmedCarrier(o, h...)`
  - `SharedEdgeDualTrim`
  - `ReboundCarrier`
- arbitrary admitted shell and face counts
- arbitrary admitted edge, coedge, and vertex binding counts
- local topology replacement and rebinding workflows
- historical and branch-local binding inspection workflows

### Must verify

- topology remains authoritative when geometry bindings change
- binding legality is distinct from topology legality
- naming continuity remains inspectable independently from binding state
- admitted rebinding histories replay identically

### Required verification output

- topology_geometry_binding_digest
- rebinding_history_report
- identity_separation_report
- binding_replay_parity_report

### Closeout condition

Milestone `6` closes only when rebinding and identity separation work
generically over the admitted binding workflow surface.

## Milestone 7: Curved Geometry Foundation

### Purpose

Prove that admitted curved geometry classes are bounded, visible, and
auditable, without letting approximation silently become truth.

### Required workload surface

Run admitted curved workloads containing:

- primitive-corpus carrier coverage for admitted analytic curved carriers,
  including:
  - `PeriodicAnalyticCarrier`
  - `SingularAnalyticCarrier`
  - curved `TrimmedCarrier(o, h...)`
- arbitrary admitted curved edge and trim counts
- tangent and near-tangent events within the admitted curved class
- local chained curved rebuilds of bounded admitted depth
- anti-drift workflows over the admitted anchored geometry surface

### Must verify

- approximation and escalation remain visible and bounded
- policy-required unresolved cases fail cleanly
- anti-drift and anchoring outcomes are deterministic
- admitted curved histories replay to the same bounded outcomes

### Required verification output

- approximation_decision_digest
- escalation_trace_report
- anti_drift_report
- curved_replay_parity_report

### Closeout condition

Milestone `7` closes only when admitted curved workflows are generic across the
admitted class rather than passing on a handful of analytic examples.

## Milestone 8: Curved Hostile Proof Program

### Purpose

Prove that hostile curved workloads either stay within declared bounds or fail
with exact, replay-safe localization.

### Required workload surface

Run hostile curved workloads containing:

- hostile primitive-corpus coverage for admitted analytic curved carrier
  families and admitted `ReboundCarrier` histories under tangent pressure
- tangent-heavy curved histories of arbitrary admitted length
- scale-separated curved cases within the admitted class
- anchored or symbolic drift-sensitive histories

### Must verify

- degradation is detected where it begins
- drift-localization remains exact and replay-safe
- accepted and rejected hostile curved cases replay identically
- no hostile curved workload crashes, hangs, or drifts silently

### Required verification output

- curved_truth_digest_series
- drift_localization_report
- curved_failure_localization_report
- curved_hostile_replay_report

### Closeout condition

Milestone `8` closes only when hostile curved proof covers the admitted curved
workflow class rather than isolated hard cases.

## Milestone 9: Feature Intent Core And Spec Truth

### Purpose

Prove that feature intent is authoritative spec truth rather than host-side
convention.

### Required workload surface

Run feature-truth workloads containing:

- arbitrary admitted feature counts
- arbitrary admitted dependency fan-in and fan-out
- parameter rewrite workflows
- branch-local feature-history inspection

### Must verify

- identical feature-truth histories replay identically
- dependency legality and dependency ordering are deterministic
- feature truth remains distinct from derived regeneration
- diagnostics tie feature intent to affected truth domains

### Required verification output

- feature_truth_digest
- dependency_determinism_report
- feature_domain_affect_report
- feature_truth_replay_report

### Closeout condition

Milestone `9` closes only when feature truth works generically across the
admitted feature workflow surface, not only for a couple of authored examples.

## Milestone 10: Regeneration, Dependency Execution, And Replay Parity

### Purpose

Prove that admitted feature-driven regeneration is deterministic, replay-safe,
and bridge-causal without becoming a second runtime.

### Required workload surface

Run regeneration workloads containing:

- arbitrary admitted feature-graph sizes
- arbitrary admitted dependency depths
- topology-only and geometry-only delta workflows
- live, replayed, and branch-local regeneration over the same admitted feature
  history

### Must verify

- live and replayed regeneration yield the same admitted conclusions
- topology-only and geometry-only recompute remain distinguishable
- dependency execution is deterministic
- derived outputs remain disposable and rebuildable

### Required verification output

- regenerated_model_digest
- topo_vs_geom_delta_report
- dependency_execution_report
- regeneration_replay_parity_report

### Closeout condition

Milestone `10` closes only when admitted regeneration workflows operate
generically across the admitted feature-graph workflow surface with replay
parity and honest recompute boundaries.
