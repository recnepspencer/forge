# Milestone 3: Construction Resolver And Research Compiler

## Goal

Build the first serious construction resolver for Hadwiger-Nelson exploration.

Milestone 3 turns retained evidence, tiling primitives, exact geometry,
screening certificates, suppression memory, and agent hypotheses into an
executable research compiler. A hypothesis is no longer a prose observation or
loose next-action note; it is a typed resolver policy with objective signals,
constraints, falsifiers, repair operators, equivalence contracts, and retained
outputs.

The target is not an AI runtime inside the crate. The target is a rigorous
environment that lets an external human or agent propose ideas, then forces
those ideas through the same kind of constraint-resolution discipline that a
serious CAD/MEP resolver applies to buildings: typed components, physics,
placement rules, conflict detection, repair, and replayable certification.

## Why This Milestone Exists

Milestone 1 made Hadwiger research artifacts Query-first, checker-backed,
aspect-aware, suppressible, and certifiable. Milestone 2 added a tiling
candidate language and iteration harness. Those are necessary, but they still
do not guarantee that an exploration loop is high-impact. Our first real
frontier drill-down exposed the gap: the system can mine interesting motifs,
but interesting motif archaeology is not the same as a path toward a stronger
chromatic result.

Milestone 3 closes that gap by making the loop optimize for executable research
progress. It gives the crate a construction-solving pipeline that can:

- declare what a candidate is trying to construct
- lower that intent through Query
- compile the intent into constraints and objectives
- resolve exact geometry and graph/colorability requirements
- reject or repair conflicts before expensive checker work
- suppress dead ends mechanically
- preserve every failure as reusable graph memory
- produce a replayable action/certification packet for the next iteration

## Governing Summaries

- `MENTALITY.md`: the strongest shaping constraint is to solve the hard
  structural problem first. For Milestone 3, that problem is not candidate
  generation; it is preventing low-signal hypothesis churn from masquerading as
  progress.
- `arch_laws.md`: semantic intent must compile into orchestration, policy and
  topology decisions must be pre-resolved, invalid states must be
  unrepresentable, and proof-bearing phase outputs must become the next phase's
  inputs.
- `composition_laws.md`: resolver vocabulary, constraint compilation,
  candidate assembly, repair, scoring, and certification must be separate
  responsibilities with names that predict their contents.
- `domain_structure_laws.md`: construction intent, exact constraints,
  candidate truth, derived indexes, speculative repairs, negative evidence, and
  reports must not share structural space or authority.
- `perf_laws.md`: broad exploration must be scoped, counted, and indexed.
  Expensive graph/geometry facts must be proven once and carried forward rather
  than rediscovered in helper loops.
- `milestone-2.md`: the prior milestone makes typed tiling iteration possible;
  Milestone 3 must turn that iteration into a resolver whose next moves are
  executable, falsifiable, and replayable.
- `AI_README.md`: ordinary Hadwiger work must still enter through Query,
  preserve canonical declaration/progression artifacts, use graph-owned
  lookup/index views, and let graph shape determine applicable validators and
  invariants wherever possible.

## Adversarial Constraint

The resolver must survive a high-volume external-agent loop that proposes many
plausible but overlapping, stale, underconstrained, visually tempting, or
already-falsified construction ideas. No idea may become a planned experiment
unless the system can prove what it is trying to improve, what exact constraints
it must satisfy, what would falsify it, which retained evidence it consumes,
which dead ends it is equivalent to, and which checker/proof lanes could admit
it later.

If any path can:

- re-run a dead-end construction without a typed reactivation condition
- score a hypothesis without naming its expected information gain
- build candidate geometry from approximate coordinates when exact replay is
  required
- decide which graph invariants apply by caller memory instead of graph shape
- hide broad corpus or graph walks behind a cheap-looking getter
- mutate a candidate without retaining the rewrite operator and repair reason
- treat advisory or speculative resolver output as checker, proof, theorem, or
  Query invariant authority

then this milestone has failed.

## Product Decision Lock

Milestone 3 is a Rust-facade and research-compiler milestone. It does not add a
web UI, TUI, CLI, in-crate AI model client, prompt orchestration engine, durable
store, or automatic theorem prover. External agents may drive it, but the crate
owns the typed construction language, resolver phase chain, authority
boundaries, replay evidence, and certification bundle.

## Target DX

The public rhythm should teach one path: Query-admitted Hadwiger handle,
construction library, executable hypothesis policy, checked resolution, retained
result.

```rust
use hadwiger_research::facade::{
    admit_hadwiger_research_handle,
    ConstructionPrimitiveLibrary,
    ConstructionResolverRequest,
    HadwigerResearchOperatingContext,
    VirtualEdgeClampCompositionHypothesis,
    resolve_hadwiger_construction_candidate_checked,
};

let handle =
    admit_hadwiger_research_handle(HadwigerResearchOperatingContext::finite_lower_bound_real())?;

let library = ConstructionPrimitiveLibrary::hadwiger_frontier_default()
    .with_virtual_edge_primitives()
    .with_clamp_composition_primitives()
    .with_periodic_patch_primitives();

let hypothesis = VirtualEdgeClampCompositionHypothesis::new("clamp-shell-pass-a")
    .with_terminal_pair("left-terminal", "right-terminal")?
    .with_objective("increase not-5-colorability pressure after exact unit replay")?
    .with_falsifier("candidate remains 5-colorable after checked CNF replay")?
    .with_repair_operator("replace flexible spoke with rigid terminal clamp")?;

let resolved = resolve_hadwiger_construction_candidate_checked(
    &handle,
    ConstructionResolverRequest::new("candidate-pass-a", library)
        .with_hypothesis(hypothesis)
        .with_evidence_corpus(&corpus)
        .with_screening_catalog(&screening_catalog)
        .with_research_graph_catalog(&catalog),
)?;

assert!(resolved.legality_report().is_enforced());
assert!(resolved.counters().query_readiness_checks() > 0);
assert!(!resolved.admits_theorem_authority());
```

Resolver output should be reusable by the cockpit without losing why the next
move exists.

```rust
use hadwiger_research::facade::{
    derive_research_cockpit_action_packet_checked,
    ResearchCockpitSession,
};

let session = ResearchCockpitSession::builder("resolver-session-a")
    .with_corpus(corpus)
    .with_frontier(frontier)
    .with_resolved_candidate(resolved)
    .finish()?;

let packet = derive_research_cockpit_action_packet_checked(&handle, &session)?;

assert!(packet.actions().iter().any(|action| action.is_checker_ready()));
assert!(!packet.actions().iter().any(|action| action.is_dead_end_rehash()));
```

## Phase Plan

### Phase 1: Construction Declaration Families

Freeze the Query-facing intent vocabulary for construction resolution.

**Relevant subsystems**

- `domain_declarations`
- `tiling_candidates`
- `agent_advisory`
- `research_cockpit`

**Relevant APIs**

- `ForgeQueryDeclarationInput`
- `ForgeQueryDeclarationFamilyMarker`
- `ForgeQueryDeclarationCanonicalEntry`
- `ForgeQueryAdmittedConfiguredDomainHandle::declare_checked(...)`
- `declaration_entry_readiness::<I>()`
- `declaration_entry_crossing_inventory::<I>()`

**Warnings**

- Construction declarations are intent, not evidence. They cannot admit
  geometry, colorability, proof, or invariant authority.
- Resolver declarations must not duplicate Phase 2 tiling declarations; they
  point at primitive libraries, hypotheses, objectives, and evidence bases.

**Test requirements**

- Equivalent resolver declarations converge despite primitive, objective, and
  evidence insertion order.
- Raw construction requests cannot resolve, mutate, or plan themselves without
  an admitted Hadwiger handle and Query declaration path.

**Engineering decisions**

- Add declaration families for construction resolver requests, primitive
  library snapshots, executable hypothesis policies, rewrite attempts, repair
  attempts, and resolver certification bundles.
- Canonical declaration identity includes source corpus digest, primitive
  library digest, hypothesis digest, objective digest, and resolver budget
  scope.

**Open questions**

- None.

### Phase 2: Construction Primitive Library

Define the reusable pieces the resolver is allowed to compose.

**Relevant subsystems**

- `domain_artifacts`
- `candidate_screening`
- `tiling_candidates`
- `frontier_seeds`

**Relevant APIs**

- Hadwiger canonical artifact/reference contracts
- tile/contact equivalence witness surfaces
- candidate screening invariant catalog
- exact graph and algebraic embedding artifacts

**Warnings**

- A primitive is not a theorem fragment. It is a typed construction component
  with constraints, ports, and evidence requirements.
- Primitive libraries must reject ambiguous terminals, unowned boundaries,
  floating-only coordinates, and implicit color assumptions.

**Test requirements**

- Equivalent libraries converge despite primitive order and alias insertion
  order.
- A primitive with a missing terminal, invalid port role, or unsupported
  coordinate field is rejected before candidate assembly.

**Engineering decisions**

- Add primitives for virtual edges, bichromatic virtual edges, terminal clamps,
  rigid spokes, periodic quotient patches, boundary-owned tiles, known
  obstruction cores, and seed-derived graph fragments.
- Every primitive exposes ports, exact geometry requirements, coloring
  requirements, allowed rewrite operators, and suppression equivalence scope.

**Resolved decisions**

- Use three fixture tiers. Tiny ordinary tests use Moser-spindle-scale,
  Golomb-like, and small virtual-edge/clamp primitives. Medium ignored tests
  use trimmed Heule/Polymath/de Grey-derived motifs and minimized cores.
  Full frontier assets such as Heule 510/517 graphs and large proof
  certificates remain retained, digest-bound heavyweight assets outside the
  ordinary suite.

### Phase 3: Executable Hypothesis Policy

Turn hypotheses into typed policies that can be falsified, repaired, and scored.

**Relevant subsystems**

- `discovery_loop`
- `agent_advisory`
- `research_cockpit`
- `explanations`

**Relevant APIs**

- `InvariantHypothesis`
- `AgentExperimentProposal`
- `ExperimentPlan`
- `ExperimentSuppressionProof`
- Query contribution-composed advisory/support surfaces

**Warnings**

- Hypotheses without objectives, falsifiers, repair operators, and expected
  information gain are advisory notes only.
- A hypothesis policy cannot directly create an `ExperimentPlan`; it must pass
  suppression, legality, and resolver eligibility.

**Test requirements**

- Equivalent hypothesis policies converge when objective/falsifier/repair rows
  are inserted in different order.
- A policy missing any of objective, falsifier, repair operator, evidence basis,
  or expected signal is denied resolver eligibility with typed blockers.

**Engineering decisions**

- Add policy types for virtual-edge clamp composition, periodic quotient
  pressure balancing, obstruction-core grafting, boundary repair, and
  color-pressure transfer.
- Add lifecycle states: `Advisory`, `ResolverEligible`, `Resolved`,
  `Falsified`, `Repaired`, `Retired`, and `Reactivated`.
- Ship objectives as named descriptors first, paired with typed scoring
  profiles. Do not introduce a general weighted expression language until the
  resolver has produced enough retained score reports to justify the extra
  semantic surface.

**Resolved decisions**

- Objective functions ship as named enum-like descriptors such as
  `IncreaseNotKColorabilityPressure`, `ReduceExactGeometryRisk`,
  `MaximizeMotifReuse`, and `CloseProofGap`. Weighting belongs to typed
  `ResolverScoringProfile` values, not to a free-form expression language.

### Phase 4: Constraint Compilation

Compile construction intent into exact geometry, graph, colorability,
boundary, and invariant obligations.

**Relevant subsystems**

- `candidate_screening`
- `algebraic_geometry`
- `research_graph_invariants`
- `domain_artifacts`

**Relevant APIs**

- exact geometry replay operations
- algebraic unit-distance verification
- candidate screening operations
- Query custom invariant registration and graph-shaped obligation reports

**Warnings**

- Constraint compilation is not execution. It produces a proof-carrying plan
  that later phases consume without rediscovering applicability.
- Constraint applicability must be derived from primitive shape, graph shape,
  aspect posture, and registered invariant descriptors, not caller-selected
  checklists.

**Test requirements**

- Equivalent primitive/hypothesis inputs compile to the same constraint plan and
  counters despite insertion order.
- Unsupported algebraic fields, ambiguous boundary conventions, missing graph
  obligations, and unchecked advisory evidence stop before assembly.

**Engineering decisions**

- Add a `ConstructionConstraintPlan` carrying exact geometry obligations,
  colorability obligations, boundary obligations, screening obligations,
  graph-legality obligations, and expected checker routes.
- Carry counters for primitive count, port count, constraint count, Query
  readiness checks, graph index touches, and rejected unsupported constraints.
- Use certificate-first algebraic geometry. Constraint compilation may accept
  exact rational, quadratic-field, and retained algebraic replay certificates.
  Native solving is limited to small sealed operators whose field closure and
  replay obligations are declared in the operator type.

**Resolved decisions**

- Milestone 3 prioritizes algebraic replay over general solving. Arbitrary
  algebraic coordinate solving is out of scope. Tiny native solves are allowed
  only for shape-specific sealed operators that produce replayable coordinate
  certificates before any authority-sensitive use.

### Phase 5: Candidate Assembly And Exact Replay

Assemble resolver-eligible primitives into candidate artifacts and replay the
cheap exact constraints before expensive solver work.

**Relevant subsystems**

- `tiling_candidates`
- `frontier_seeds`
- `candidate_screening`
- `domain_artifacts`

**Relevant APIs**

- `GraphVersion` builders
- `ExactGraphEmbedding`
- `AlgebraicGraphEmbedding`
- exact unit-distance and interval screening operations
- boundary ownership and conflict graph construction screening operations

**Warnings**

- Assembly must not allocate a large candidate before cheap rejection checks
  prove primitive compatibility.
- Geometry replay and conflict extraction must consume the compiled constraint
  plan, not re-derive obligations from raw candidate fields.

**Test requirements**

- Candidate assembly is deterministic across equivalent primitive ordering,
  port ordering, and terminal aliasing.
- A same-color unit conflict, ambiguous boundary, non-unit edge, missing
  coordinate, or unsupported algebraic field rejects assembly without creating
  checker or theorem authority.

**Engineering decisions**

- Add `ResolvedConstructionCandidate`, `ConstructionAssemblyPlan`,
  `ConstructionAssemblyReport`, and `ConstructionAssemblyBlocker`.
- Emit retained partial candidates when useful for repair, but mark them
  support/advisory only unless all required replay obligations pass.
- First assembly supports one-level primitive composition with explicit graft
  points. Arbitrary nesting is deferred until equivalence, repair, projection,
  and suppression semantics are proven on one-level composition.

**Resolved decisions**

- Ship one-level composition plus explicit graft points. This still supports
  serious candidates while keeping locality, equivalence, suppression, and
  repair evidence mechanically explainable.

### Phase 6: Checker Routing And Proof-Ready Action Production

Prepare checker and proof work only after exact replay and graph legality pass.

**Relevant subsystems**

- `mathematical_verification`
- `proof_claims`
- `research_cockpit`
- `domain_declarations`

**Relevant APIs**

- `verify_unit_distance_embedding_checked(...)`
- `verify_algebraic_unit_distance_embedding_checked(...)`
- `verify_k_colorability_checked(...)`
- `verify_k_colorability_with_certificate_checked(...)`
- `admit_plane_lower_bound_claim_checked(...)`
- Query declaration progression, route, receipt, and envelope surfaces

**Warnings**

- Resolver output can be checker-ready or proof-ready; it cannot itself admit
  checker or theorem authority.
- SAT solver output without independent replay remains advisory even when the
  resolver predicts it is promising.

**Test requirements**

- A checker-ready action packet preserves Query declaration digest, compiled
  constraint digest, candidate digest, and exact replay digest.
- Missing certificate replay, stale Query readiness, or unsupported checker
  lane blocks proof-ready action and emits a typed recovery/repair row.

**Engineering decisions**

- Add `ConstructionCheckerAction`, `ConstructionProofAction`,
  `ConstructionActionEligibility`, and `ConstructionActionBlocker`.
- Action packets include route expectations, authority requirements, and the
  minimal retained evidence needed by the next checker/proof operation.
- Milestone 3 produces checker-ready and proof-ready action packets. It may
  call existing in-process checked Hadwiger operations, but it does not own
  external checker scheduling or long-running resource lifecycle.

**Resolved decisions**

- External checker scheduling belongs in a later bridge-backed milestone.
  Resolver action packets name the required checker/proof lane and retained
  evidence; runtime bridge or signal-backed scheduling should own external
  execution lifecycle later.

### Phase 7: Rewrite Operators And Repair Search

Make construction failure actionable by producing bounded, typed repairs.

**Relevant subsystems**

- `explanations`
- `discovery_loop`
- `research_graph_invariants`
- `research_cockpit`

**Relevant APIs**

- `HadwigerReusableNegativeEvidence`
- `GraphResidentFailure`
- `FailureScope`
- `ExperimentSuppressionProof`
- Query recovery briefs where stops are Query-owned

**Warnings**

- Repair operators must be named domain actions, not arbitrary mutation
  callbacks.
- A repair cannot erase the failure it repairs. It must retain the negative
  evidence and explain what changed enough to avoid suppression.

**Test requirements**

- A repaired candidate that does not change the dead-end signature remains
  suppressed and cannot become a new experiment.
- A repair with a typed reactivation condition and qualifying new evidence can
  produce a distinct resolver-eligible policy with changed digest.

**Engineering decisions**

- Add operators for terminal clamp replacement, spoke rigidification,
  conflict-edge relocation, boundary ownership split, periodic-cell expansion,
  quotient-lattice perturbation, obstruction-core graft, and color-pressure
  balancing.
- Every operator declares its input scope, output scope, invariants touched,
  expected signal, and rollback/retention evidence.
- Coordinate-creating repairs are sealed and certificate-producing. Operators
  may create new algebraic coordinates only when the operator declares the
  field, field closure, coordinate construction rule, and exact replay
  obligations.

**Resolved decisions**

- Most repair operators select from retained certified coordinates or modify
  graph/tile structure. Coordinate creation is allowed only for sealed
  shape-specific constructions such as exact midpoint, reflection, named
  rotation, graft-point reuse, or quadratic-field construction whose replay
  certificate proves all required distances.

### Phase 8: Objective Scoring And Information-Gain Accounting

Score resolver candidates by expected research value, not visual plausibility.

**Relevant subsystems**

- `discovery_loop`
- `research_cockpit`
- `candidate_screening`
- `agent_advisory`

**Relevant APIs**

- `DiscoveryScorecard`
- `ResearchCockpitCounters`
- screening evaluation reports
- agent advisory contribution records

**Warnings**

- Scores are prioritization evidence only. They cannot satisfy mathematical,
  checker, proof, or invariant authority.
- Hidden full-corpus scans behind score getters are forbidden. Broad scoring
  must be a named recomputation operation with counters.

**Test requirements**

- Equivalent evidence and candidate inputs produce identical score reports,
  including counter snapshots.
- Changing objective weights, expected checker cost, suppression hits,
  novelty/equivalence posture, or proof-gap posture changes the score digest.

**Engineering decisions**

- Score dimensions include expected chromatic pressure, exact-geometry risk,
  SAT/certificate cost, novelty, suppression risk, repairability, motif reuse,
  proof-gap closure, and counterexample value.
- Every score report carries explicit counters for graph index touches,
  candidate breadth, screening rows consumed, suppression hits, and unsupported
  lanes skipped.
- Provide built-in scoring profiles for the ordinary resolver lanes and allow
  caller-supplied typed profiles with bounded weights and canonical digest
  identity.

**Resolved decisions**

- Ship fixed defaults and typed custom profiles. Defaults include
  `frontier_lower_bound_default`, `periodic_upper_bound_default`, and
  `geometry_repair_default`. Custom profiles must be bounded, canonicalized,
  and non-authoritative.

### Phase 9: Resolver Corpus Projection And Queryable Research Graph

Materialize resolver output as a compact queryable graph for external agents.

**Relevant subsystems**

- `frontier_exploration`
- `research_graph_invariants`
- `research_cockpit`
- `agent_advisory`

**Relevant APIs**

- Query projection consumption receipts
- graph-owned lookup/index guidance from `AI_README.md`
- Hadwiger research graph custom invariant registrations
- research cockpit equivalence classes

**Warnings**

- The agent-readable graph is derived state. It must be rebuildable from
  retained authority, not become a second source of truth.
- Projection nodes must retain typed references back to source artifacts; no
  raw string-only edges.

**Test requirements**

- Recomputing the resolver projection from the same retained corpus converges
  exactly despite insertion order.
- Projection facts used by a later hypothesis or score retain real Query
  projection-consumption receipts where Query owns the materialized fact.

**Engineering decisions**

- Add nodes for primitives, ports, constraints, candidates, failures, repairs,
  objectives, checker actions, proof gaps, equivalence classes, and score rows.
- Add edges for consumes, satisfies, violates, repairs, suppresses, reactivates,
  equivalent-for, checker-ready-for, and proof-gap-for.
- Emit both a Rust projection artifact and an optional JSONL sidecar. The Rust
  artifact is the authority-preserving derived form; JSONL is an agent-browsing
  projection bound to the Rust projection digest.

**Resolved decisions**

- Ship both. The compact Rust artifact remains the source derived artifact.
  JSONL is a rebuildable, digest-bound sidecar for agent exploration and must
  never become an independent authority store.

### Phase 10: Closed-Loop Iteration Runner

Run bounded resolver iterations while preserving suppression, legality, and
replay.

**Relevant subsystems**

- `research_cockpit`
- `discovery_loop`
- `research_graph_invariants`
- `candidate_screening`

**Relevant APIs**

- `ResearchCockpitSession`
- `ResearchCockpitActionPacket`
- `ExperimentBatch`
- `update_discovery_frontier(...)`
- graph-shaped invariant legality reports

**Warnings**

- The runner coordinates a bounded research loop; it does not run open-ended AI
  generation or unattended theorem search.
- Each iteration must consume the prior iteration's retained artifacts and
  suppression state rather than reconstructing history from logs.

**Test requirements**

- A five-iteration run is deterministic, has stable digest output, and exposes
  per-iteration counters.
- If iteration `n` produces a dead-end signature, iteration `n + 1` cannot
  re-plan the same construction unless a typed reactivation condition is
  satisfied by new evidence.

**Engineering decisions**

- Add `ConstructionResolverRunRequest`, `ConstructionResolverIterationReport`,
  and `ConstructionResolverRunReport`.
- Iteration sequence is: declare -> compile constraints -> assemble -> replay
  cheap exact checks -> produce checker/proof actions -> retain explanations ->
  attach failures -> repair/rewrite -> rescore -> update cockpit packet.
- Public iteration uses explicit budget profiles. Five iterations is the
  certification/default smoke loop, not the hard public ceiling.

**Resolved decisions**

- Do not hard-code five as the public maximum. Public requests accept bounded
  iteration counts through a `ResolverIterationBudget`; certification requires
  a deterministic five-iteration fixture.

### Phase 11: Milestone 3 Certification Bundle

Certify that the resolver can produce high-signal next actions without leaking
authority.

**Relevant subsystems**

- all Milestone 3 resolver subsystems
- Query declaration/progression/readiness
- Hadwiger certification bundle

**Relevant APIs**

- `certify_hadwiger_milestone_one_bundle_checked(...)`
- candidate screening certification reports
- research cockpit certification artifacts
- Query declaration inventory/readiness

**Warnings**

- Certification is not a success claim about Hadwiger-Nelson. It proves the
  resolver is honest, replayable, and ready for real iteration.
- The bundle must include both successful candidate preparation and hostile
  failed/blocked/repair/suppression scenarios.

**Test requirements**

- Certification includes at least one virtual-edge/clamp composition scenario,
  one periodic quotient scenario, one boundary failure scenario, one SAT
  certificate blocker scenario, one dead-end suppression scenario, and one
  repair/reactivation scenario.
- Certification proves advisory agent input, heuristic scores, and derived
  projections never admit theorem, checker, proof, or Query invariant
  authority.

**Engineering decisions**

- Extend the digest inventory with construction declaration, primitive library,
  hypothesis policy, constraint plan, assembly plan, candidate, exact replay,
  checker action, repair operator, score report, resolver projection, iteration
  report, and run report digests.
- Add golden DX transcripts for resolver request, candidate resolution,
  checker-ready action production, repair search, and five-iteration run.

**Open questions**

- None.

## Must Ship

- Query declaration families for construction resolver requests, primitive
  libraries, hypothesis policies, rewrite attempts, repair attempts, and
  resolver certification bundles.
- `ConstructionPrimitiveLibrary` with first-class Hadwiger primitives:
  virtual edges, bichromatic virtual edges, terminal clamps, rigid spokes,
  periodic quotient patches, boundary-owned tiles, known obstruction cores, and
  seed-derived fragments.
- Executable hypothesis policy types with objective, falsifier, repair
  operators, expected information gain, evidence basis, and lifecycle posture.
- `ConstructionConstraintPlan` carrying exact geometry, graph/colorability,
  boundary, screening, and research-graph legality obligations.
- Candidate assembly and cheap exact replay before expensive checker/proof
  action production.
- Checked action packets for geometry, SAT/colorability, proof-admission, and
  invariant-denial work.
- Rewrite and repair operators with typed reactivation and suppression
  interaction.
- Objective scoring reports with visible counters and no authority promotion.
- Queryable resolver projection graph that is derived from retained authority.
- Bounded closed-loop iteration runner with deterministic five-iteration
  certification.
- Milestone 3 certification bundle.

## Must Preserve

- Query remains the public entry point.
- Hadwiger owns construction, tiling, motif, and resolver domain meaning.
- Checkers and proof-admission functions remain the only mathematical authority
  lanes.
- Resolver output remains candidate/support/advisory unless later checker/proof
  lanes admit stronger authority.
- Graph-shaped invariants and validators apply mechanically from graph shape
  wherever possible.
- Failed resolver attempts remain reusable graph memory.
- Suppression and reactivation remain proof-bearing, not heuristic.
- Derived projections and score reports remain rebuildable from retained
  authority.

## Acceptance Evidence

- `cargo fmt -p hadwiger-research --check`
- `cargo check -p hadwiger-research`
- `cargo test -p hadwiger-research --jobs 1`
- scoped compile-boundary suite proving no deep imports, no unchecked resolver
  execution, no advisory-to-authority promotion, no raw primitive bypass, and
  no mutable resolver internals.
- certification bundle proves the target DX, hostile failure paths, suppressed
  duplicate paths, repair/reactivation paths, and deterministic iteration.
- line-cap and directory-cap audits remain clean.
- public docs explain that Milestone 3 is a resolver/research compiler, not an
  AI runtime or UI.

## Sequencing Notes

Milestone 3 belongs immediately after Milestone 2. Milestone 2 makes tiling
ideas typed and replayable; Milestone 3 makes their exploration disciplined.
Building a UI or model-driven generator before this resolver would create
faster low-signal churn. Building the resolver now gives external agents and
humans a playground that is actually useful: every idea becomes a typed
construction policy, every failure becomes reusable evidence, and every next
move is explainable and replayable.

After Milestone 3, the project is ready for sustained frontier exploration with
external agents driving high-impact hypothesis loops through the Hadwiger
facade.

## Self-Check

- Does the milestone solve a real structural problem? Yes: it turns
  exploration from motif commentary into executable construction resolution.
- Is the adversarial constraint precise and load-bearing? Yes: it prevents
  stale, duplicate, advisory, underconstrained, or visually plausible ideas from
  becoming planned work without proof-bearing eligibility.
- Does the roadmap justify this milestone now? Yes: Milestone 2 makes tiling
  candidates available; the next needed capability is disciplined construction
  resolution.
- Does the spec preserve crate authority boundaries? Yes: Query owns entry and
  orchestration, Hadwiger owns resolver meaning, checkers/proof functions own
  mathematical authority, and projections remain derived.
- Are the phases carrying most of the real design information? Yes.
- Is each phase centered on one conceptual detail or boundary? Yes.
- Does each phase contain at least two adversarial tests? Yes.
- Could a competent engineer map this spec into honest types, modules, and
  tests? Yes.
- Does the milestone belong in this roadmap sequence? Yes.
