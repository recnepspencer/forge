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

Milestone 1 closeout is recorded in:

- [milestone-1-closeout.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/worth/milestone-1-closeout.md)

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
- same-commit graph creation where topology entities and topology relations are
  authored together in one authoritative publish boundary using symbolic
  created-endpoint resolution rather than a second repair commit

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
- commit-boundary illegal topology attempts emit structured rejection artifacts
  with localized rejection class and diagnostic evidence
- same-commit graph creation preserves one coherent authoritative publish
  boundary for admitted topology creation rather than emitting orphan-entity
  intermediate truth
- derived topology interpretation classifies admitted shell, wire, boundary,
  wire-branch, solid-shell, and radial structure deterministically
- persistent names are authoritative truth, not regenerated labels
- certification emits named counter surfaces for the milestone-1 authority,
  interpretation, and replay boundaries
- closeout includes one machine-checkable bridge proof path from committed
  Worth truth through route and historical evaluation
- rejected and accepted paths both emit replayable diagnostic evidence

### Required verification output

- topology_truth_digest
- topology_validation_digest
- topology_validation_report
- naming_truth_digest
- topology_localization_report
- naming_attachment_report
- milestone_1_counter_report
- primitive_family_coverage_matrix
- primitive_corpus_parity_report
- admitted_range_sweep_report
- illegal_topology_rejection_report
- failure_locality_report
- bridge_family_coverage_report
- branch_local_topology_report
- milestone_1_replay_parity_report
- bridge_proof_report

These outputs must be emitted as direct closeout surfaces for the milestone,
not only as nested seeded or per-case reports that require downstream
reconstruction by convention.
The bridge proof output must also exercise multiple admitted topology families,
not just one showcase commit.
The closeout artifact should also expose aggregate validator-family coverage
and rejection-class distribution so the machine-checkable proof can explain
what passed and what rejected without per-case reconstruction.
Validator coverage must stay attributable by primitive family, not only in
global totals, so the proof surface can detect one admitted family lagging
while another family exercises the same validator successfully.
Failure locality must stay attributable by primitive family and role with
localized entity/relation counts, so admitted-family regressions cannot hide in
one generic rejection bucket.
The primitive-family coverage and parity matrices must keep explicit rows for
every canonical milestone-1 admitted family, including families that are
missing or currently failing, so a missing family cannot disappear silently
from the closeout artifact.
Branch-local parity is not complete unless the proof surface demonstrates a
real cross-branch partition; matching counts on a single branch are
insufficient.
Admitted-range sweeps are required closeout evidence, not optional secondary
checks, and each canonical family must keep an explicit out-of-class boundary
neighbor in the sweep proof surface.

### Closeout condition

Milestone `1` closes only when admitted topology truth and persistent naming
work generically across the milestone's admitted workflow surface, including
`WireBranch(k)` and `SolidShell(f)` as fully robust admitted families rather
than toy examples or partial subsets, when the required admitted-range sweeps
pass across every admitted family with visible out-of-class boundary rejection
evidence, and when admitted topology creation
publishes as one coherent same-commit graph mutation rather than a staged
authority workaround.

## Milestone 2: Derived Topology Materialization And Bridge-Causal Validation

### Purpose

Prove that topology materialization, invalidation, and derived topology
validation remain subordinate to truth and replay identically from the same
truth basis.

### Required workload surface

Run repeated executions over:

- primitive-corpus coverage for the milestone-1 admitted families under replay
  and branch-local reads, including `Smallest`, `Generic`,
  `HostileAdmitted`, and `OutOfClass` members for each admitted family
- arbitrary admitted seeded topology truth
- arbitrary admitted local authoritative truth mutations from milestone `1`
- arbitrary admitted shell, wire, and radial counts
- branch-local reads, mainline reads, and replayed reads of the same truth
  history

### Must verify

- identical truth histories produce identical topology materializations
- bridge invalidation remains parity-safe and deterministic
- derived topology validators classify admitted structure identically across
  replay and branch-local reads
- family-attributed derived coverage and parity remain explicit, so one
  admitted family cannot hide behind another family's totals
- reuse or rebuild-suppression claims are backed by an explicit equivalence
  contract
- no derived cache becomes authority

### Required verification output

- materialized_topology_digest
- interpreted_topology_digest
- derived_validation_digest
- derived_truth_basis_digest
- bridge_routing_digest
- bridge_historical_evaluation_digest
- derived_family_coverage_matrix
- derived_family_parity_matrix
- derived_validator_coverage_report
- derived_invalidation_report
- derived_rebuild_report
- derived_equivalence_contract_report
- derived_fallback_report
- derived_failure_locality_report
- derived_branch_local_parity_report
- derived_replay_parity_report
- derived_bridge_family_coverage_report
- milestone_2_counter_report

### Closeout condition

Milestone `2` closes only when derived topology is a deterministic, rebuildable
layer over the admitted topology truth workflows, with corpus-shaped proof over
the canonical admitted families rather than one recompute showcase path. These
outputs must be emitted as direct closeout surfaces for the milestone, not
only as nested helper artifacts or Milestone-1 compatibility fields.
Derived validator coverage must stay attributable by admitted primitive family,
validator family, and derived phase, so one validator can not silently stop
exercising one admitted family while aggregate derived family coverage still
looks healthy.

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

## Milestone 7: Planar B-Rep Boolean Foundation

### Purpose

Prove that the first admitted planar boolean workflow class is real on the
B-rep lane, while preserving the Query-owned declaration, admission, readiness,
route, receipt, and envelope contract that future EMBER execution must
inherit.

### Required workload surface

Run admitted planar boolean workloads containing:

- arbitrary admitted planar body pairs
- arbitrary admitted coplanar and near-coplanar planar interactions within the
  milestone's class
- replayed, checkpointed, and non-checkpointed planar boolean histories
- the real planar metaboss families through the B-rep lane
- Query-owned declaration artifacts that classify workloads into:
  - admitted to B-rep now
  - reserved for future EMBER execution
  - denied or policy-gated before execution

### Must verify

- the B-rep boolean lane lowers through one canonical Query declaration family
- split / classify / assemble / postprocess behavior is deterministic on the
  admitted planar surface
- topology failure, spatial/predicate failure, Query-lane denial, and B-rep
  execution failure remain distinct
- admitted B-rep histories replay identically and checkpoint parity holds
- the same admitted planar workload remains legible to a future EMBER lane
  without changing the public boolean entry contract

### Required verification output

- boolean_route_plan_digest
- brep_boolean_phase_trace
- boolean_failure_localization_report
- brep_boolean_replay_parity_report

### Closeout condition

Milestone `7` closes only when admitted planar boolean workflows are generic
across the admitted B-rep workflow class, including the planar metaboss suite,
rather than passing on a handful of sample unions or box pairs.

## Milestone 8: EMBER Lane And Dual-Pipeline Hostile Boolean Proof

### Purpose

Prove that hostile planar boolean workloads either converge honestly across the
EMBER and B-rep lanes or fail with exact, replay-safe localization and explicit
typed divergence.

### Required workload surface

Run hostile planar boolean workloads containing:

- coplanar apocalypse and overlap storms
- thin labyrinth and micro-feature avalanche cases
- cancellation chains and deep boolean histories
- scale-separated and halfspace-storm cases
- singularity-star and ultimate-degeneracy cases
- accepted and rejected hostile workloads across both EMBER and B-rep lanes
- the same admitted workload lowered through the same Query declaration family
  before lane-specific execution

### Must verify

- parity or explicit typed divergence between EMBER and B-rep is deterministic
- corruption-localization remains exact and replay-safe
- checkpointed and non-checkpointed hostile histories converge to the same
  outcome class
- accepted and rejected hostile boolean cases replay identically
- no hostile boolean workload crashes, hangs, or corrupts silently across
  either lane

### Required verification output

- ember_brep_parity_digest_series
- boolean_corruption_localization_report
- hostile_boolean_failure_localization_report
- hostile_boolean_replay_report

### Closeout condition

Milestone `8` closes only when hostile boolean proof covers the admitted planar
workflow class across both EMBER and B-rep lanes rather than isolated hard
cases or one-lane demonstrations.

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
