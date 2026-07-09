# Milestone 2: Tiling Candidate Language And Iteration Harness

## Goal

Build the first concrete tiling and motif exploration layer on top of the
Milestone 1 Hadwiger research stack.

Milestone 2 turns external tiling ideas into typed, Query-declared,
replayable candidate languages: motifs with terminals, periodic quotient
cells, exact tile/contact geometry, generated-pattern certificates, conflict
graphs, failure classifications, and iteration packets. It must make the
closed-loop research workflow ready for real use without embedding an AI
runtime, trusting prose, or letting visual tiling plausibility become
mathematical authority.

## Why This Milestone Exists

Milestone 1 made Hadwiger research evidence durable, aspect-aware,
Query-native, checker-backed, explainable, suppressible, and certifiable. That
foundation is necessary but not sufficient for fast research iteration: the
system still needs a disciplined language for producing and comparing tiling
candidates, motifs, quotient cells, terminal-forcing gadgets, and pattern
failures.

The useful research loop is not "draw a tiling and try coloring it." It is:

motif seed -> certified geometry -> exact conflict graph -> coloring/proof
analysis -> core/failure extraction -> motif library update -> next candidate.

Milestone 2 owns the candidate language and iteration shape for that loop.
Query remains the public declaration/progression/recovery substrate, and
Milestone 1 checker/proof/screening/discovery surfaces remain the authority
lanes.

## Governing Summaries

- `MENTALITY.md`: protect against false authority and dead-end repetition by
  solving the hard structural problem first. This milestone must make tiling
  search proof-carrying before it makes search convenient.
- `arch_laws.md`: semantic intent must compile into orchestration, boundaries
  must emit self-describing envelopes, and equivalence contracts must be
  explicit. Tiling candidates must move through named proof-bearing phases.
- `composition_laws.md`: candidate generation, geometry certification,
  conflict construction, proof analysis, motif extraction, and iteration
  planning are separate responsibilities with predictable files and names.
- `domain_structure_laws.md`: generated proposals, certified geometry,
  conflict projections, failure evidence, motif memory, and iteration packets
  must not share structural space or authority.
- `perf_laws.md`: broad candidate search must expose scope, counters,
  equivalence basis, and bounded lookup indexes rather than hiding graph walks
  behind cheap-looking calls.
- `hadwiger-research` roadmap: Milestone 1 establishes Query-first artifact
  authority and discovery memory; the next milestone should supply the tiling
  candidate substrate that can feed that pipeline.

## Adversarial Constraint

A visually plausible, agent-suggested, floating-point, partially checked, or
near-duplicate tiling candidate must never become checker, proof, theorem, or
Query invariant authority. Every candidate must either carry the exact typed
evidence required by its current phase, stop with a retained explanation, or be
suppressed by a proof-bearing equivalence/dead-end record.

If any path can:

- treat a drawn tile adjacency as a unit-distance conflict without exact replay
- plan a repeated dead-end candidate without a typed reactivation condition
- trust a floating coordinate, vague boundary convention, or generated pattern
  rule as final evidence
- collapse lower-bound motifs and upper-bound periodic cells into one generic
  pattern type
- rebuild graph meaning through local helper traversal instead of consuming a
  canonical graph/index view
- let advisory tiling notes become experiment plans without Phase 7 and Phase 8
  checks

then this milestone has failed.

## Product Decision Lock

Milestone 2 is a Rust-facade and artifact-language milestone. It does not add a
CLI, TUI, web UI, in-crate AI generation, prompt orchestration, durable store,
or long-running solver scheduler. External agents and humans propose ideas;
Hadwiger canonicalizes, checks, explains, suppresses, and prepares them.

## Phase Plan

### Phase 1: Tiling Candidate Declarations

Freeze the public Query entry vocabulary for tiling and motif research
intents.

**Relevant subsystems**

- `domain_declarations`
- `candidate_screening`
- `agent_advisory`
- `research_cockpit`

**Relevant APIs**

- `WORTHQueryDeclarationInput`
- `WORTHQueryDeclarationFamilyMarker`
- `WORTHQueryDeclarationCanonicalEntry`
- `WORTHQueryAdmittedConfiguredDomainHandle::declare_checked(...)`
- `declaration_entry_readiness::<I>()`
- `declaration_entry_crossing_inventory::<I>()`

**Warnings**

- Tiling declarations express intent only; they do not certify geometry,
  conflict graphs, coloring, or theorem claims.
- Lower-bound obstruction candidates and upper-bound periodic coloring
  candidates must not share one vague declaration family.

**Test requirements**

- Equivalent declaration payloads converge across insertion order and helper
  vs direct Query declaration paths.
- Raw tiling/motif request types cannot progress, certify, or execute
  themselves without an admitted Hadwiger handle.

**Engineering decisions**

- Add declaration families for motif seeds, terminal-forcing studies, periodic
  quotient cells, generated-pattern closures, tile-contact witnesses, conflict
  graph extraction requests, and core-extraction requests.
- Reuse descriptive/advisory Query contribution lanes for external tiling notes
  and relational/checker lanes for replayable candidate artifacts.

**Open questions**

- None.

### Phase 2: Motif And Terminal-Forcing Language

Build motifs as reusable color-pressure modules instead of full-pattern blobs.

**Relevant subsystems**

- `domain_artifacts`
- `candidate_screening`
- `discovery_loop`
- `research_cockpit`

**Relevant APIs**

- Hadwiger canonical artifact/digest contracts
- Phase 4 colorability verification artifacts
- Phase 7 motif observations, pattern signatures, and experiment suppression
- Phase 9 agent exploration intake

**Warnings**

- A motif can be advisory, candidate, blocked, or checker-supported; it cannot
  claim terminal-forcing authority without replayed coloring evidence.
- Terminal semantics must be typed. A free-form note like "A and B probably
  differ" is advisory only.

**Test requirements**

- A motif with the same terminal relation and source evidence has the same
  digest despite vertex/terminal insertion order.
- A terminal-forcing claim without checked SAT/model/refutation evidence stays
  non-authoritative and cannot seed a proof claim.

**Engineering decisions**

- Add motif artifacts for geometry template reference, parameters, terminals,
  declared unit-distance edges, forbidden same-color pairs, source family,
  novelty signature, and proof status.
- Add terminal-relation certificates for checked relations such as
  `must_differ`, `cannot_share_color_subset`, and `requires_distinct_color_count`.

**Open questions**

- Which known seed corpora are bundled as fixtures vs retained externally?

### Phase 3: Exact Tiling Geometry And Boundary Ownership

Define the exact special-case geometry language for tiles, cells, boundaries,
and region ownership.

**Relevant subsystems**

- exact geometry subsystem from Phase 4
- candidate screening rectangular/periodic-cell certificates
- research graph invariant catalog

**Relevant APIs**

- `ExactPoint2`
- `ExactGraphEmbedding`
- exact arithmetic/interval screening operations
- boundary ownership screening operation
- tile equivalence witness operations

**Warnings**

- Visual adjacency is not a conflict graph. Conflict edges come from certified
  unit-distance possibility.
- Boundary ownership must be part of the candidate, not an after-the-fact
  convention.

**Test requirements**

- Ambiguous boundary ownership, uncovered boundary samples, and overlapping
  ownership reject or stop as typed shape errors.
- Reordered tile definitions, boundary rules, and contact samples converge to
  the same canonical geometry digest.

**Engineering decisions**

- Start with rational polygonal/rectangular cells and explicit open/closed
  boundary ownership labels.
- Require exact or interval certificates for tile diameter, same-color
  separation, Minkowski-difference/unit-circle contact, and boundary conflicts.

**Open questions**

- Whether arbitrary rational polygons ship in the first implementation slice or
  remain behind rectangular/polygonal-special-case markers.

### Phase 4: Periodic Quotient And Generated Pattern Replay

Make upper-bound-style periodic and generated tiling candidates replayable.

**Relevant subsystems**

- whole-plane coloring construction verification
- generated-pattern screening certificates
- research cockpit equivalence and action packets

**Relevant APIs**

- periodic quotient graph screening
- monodromy/color-holonomy screening
- translation/rotation closure screening
- substitution consistency screening
- finite patch boundary-extension screening

**Warnings**

- A valid finite cell is not a valid plane coloring unless wraparound,
  translations, boundary ownership, and generated closure all replay.
- Generated rules may produce evidence, blockers, or unsupported posture; they
  do not become theorem authority.

**Test requirements**

- A quotient cell that passes inside-cell checks but fails across a translated
  boundary is rejected with translation-vector-local evidence.
- A loop/generator certificate that returns a tile to an incompatible color is
  rejected, while identity-compatible loops replay deterministically.

**Engineering decisions**

- Represent periodic cells with lattice basis, region references, colors,
  boundary rules, translation rules, quotient edges, and conflict certificates.
- Treat arbitrary infinite tiling languages as unsupported until expressed as
  finite replay certificates.

**Open questions**

- How much rotation support should be exact in the first slice versus limited
  to named algebraic rotations.

### Phase 5: Conflict Graph Extraction And Core Minimization

Lower certified geometry and motifs into exact conflict graphs and reusable
cores.

**Relevant subsystems**

- `candidate_screening`
- finite graph index
- Phase 4 colorability subsystem
- Phase 7 discovery loop

**Relevant APIs**

- `ScreeningFiniteGraphIndex`
- exact unit-distance conflict screening lanes
- `verify_k_colorability_checked(...)`
- critical subgraph extraction screening
- known obstruction containment screening

**Warnings**

- Conflict graphs must be built from `1 in Delta(A, B)` or exact point
  unit-distance replay, not adjacency folklore.
- A large UNSAT graph is not the useful artifact; the minimized, replayable
  core and its ancestry are.

**Test requirements**

- Conflict graph extraction is stable across tile/region ordering and changes
  when a distance certificate, boundary ownership, or translation vector
  changes.
- Core extraction refuses to claim minimality when any removal check is missing
  or unsupported.

**Engineering decisions**

- Add typed conflict facts, translated conflict facts, coloring proof facts,
  parameter failure facts, and extracted motif/core artifacts.
- Reuse SAT/model/refutation checker authority from Milestone 1 rather than
  creating a local coloring proof lane.

**Open questions**

- Whether first critical-core minimization is vertex-only, edge-only, or both.

### Phase 6: Candidate Equivalence, Suppression, And Reactivation

Make repeated tiling dead ends mechanically unplannable.

**Relevant subsystems**

- `discovery_loop`
- `research_graph_invariants`
- `research_cockpit`
- candidate screening novelty/equivalence lanes

**Relevant APIs**

- `DeadEndSignature`
- `ExperimentSuppressionProof`
- `ReactivationCondition`
- `ResearchGraphInvariantCatalog`
- tile/contact equivalence witnesses
- candidate novelty/non-isomorphism screening

**Warnings**

- Equivalence is purpose-indexed. Geometry reuse, SAT reuse, proof-admission
  reuse, suppression, and tile/contact equivalence do not share one global
  comparator.
- A near-duplicate heuristic may prioritize or suppress only when backed by the
  declared suppression/equivalence proof required for that action.

**Test requirements**

- Relabeled, reordered, or contact-equivalent candidates suppress duplicate
  checker work when the declared equivalence scope matches the planned action.
- A previously dead candidate cannot be replanned unless a typed reactivation
  condition references qualifying new evidence.

**Engineering decisions**

- Define equivalence classes for motif terminal behavior, exact conflict graph,
  periodic quotient constraints, tile contact graph, metric threshold class,
  generated closure, and proof-admission gaps.
- Counters must expose candidate breadth, equivalence hits, suppression hits,
  reactivation hits, and hidden-broad-scan refusals.

**Open questions**

- Which equivalence scopes become compile-boundary protected public types in
  the first implementation batch.

### Phase 7: Iteration Packets And Research Cockpit Integration

Turn retained tiling evidence into replayable next-action packets for a
human/agent loop.

**Relevant subsystems**

- `research_cockpit`
- `agent_advisory`
- `explanations`
- `research_graph_invariants`

**Relevant APIs**

- `ResearchCockpitSession`
- `ResearchCockpitActionPacket`
- `AgentExplorationBatch`
- `CandidateScreeningEvaluation`
- Query contribution-composed orchestration for advisory/support/explanation

**Warnings**

- The cockpit recommends and packages next actions; it does not execute
  experiments automatically.
- Agent proposals must remain proposal/advisory artifacts until screening,
  suppression, invariant legality, and checker/proof authority admit the next
  state.

**Test requirements**

- Replaying the same candidate corpus produces the same action packet,
  counters, blocked-action rows, and certification bundle.
- Stale frontier state, missing Query readiness, or suppressed/dead-end
  equivalence blocks automatic checker/proof action and permits advisory
  preview only.

**Engineering decisions**

- Add iteration packets for lower-bound obstruction generation and upper-bound
  periodic quotient generation.
- Each packet names seed family, evidence basis, expected information gain,
  required checker lanes, suppression basis, and reactivation obligations.

**Open questions**

- None.

### Phase 8: Milestone 2 Certification Bundle

Certify that tiling iteration is ready to begin without authority leakage.

**Relevant subsystems**

- all Milestone 2 tiling candidate subsystems
- `research_cockpit`
- Query declaration/contribution/recovery surfaces

**Relevant APIs**

- Hadwiger certification bundle
- Query declaration readiness/inventory
- Query recovery briefs
- candidate screening evaluation reports
- research graph invariant denial and blocked registration plans

**Warnings**

- The certification bundle is a report/proof artifact, not a runtime registry
  and not a search executor.
- Passing Milestone 2 means ready to iterate; it does not mean any generated
  candidate is mathematically successful.

**Test requirements**

- Certification includes golden lower-bound motif-to-core and upper-bound
  periodic-cell-to-failure scenarios with retained Query declaration digests.
- Certification proves advisory text, floating geometry, unchecked generated
  rules, and duplicate dead ends cannot become checker/proof/invariant
  authority.

**Engineering decisions**

- Extend the certification digest inventory with tiling declaration, motif,
  terminal relation, tile geometry, boundary ownership, periodic quotient,
  generated closure, conflict fact, coloring proof/core, parameter failure,
  equivalence, iteration packet, and cockpit action digests.
- Include explicit scenarios for the three seed families: minimized
  5-chromatic cores, Moser-basin-avoidance constructions, and periodic
  six-color near-solution families.

**Open questions**

- Which seed fixtures are small enough to live in the crate without making the
  ordinary test suite expensive.

## Must Ship

- Query declaration families for tiling candidates, motif seeds, terminal
  forcing, periodic quotient cells, generated closures, conflict extraction,
  and core extraction.
- Motif artifacts with typed terminals, source family, novelty signature,
  geometry/proof status, and terminal relation evidence.
- Exact tiling geometry artifacts for the first supported special cases,
  including boundary ownership.
- Periodic quotient and generated-pattern replay artifacts.
- Exact conflict graph extraction from certified unit-distance possibility.
- Core/minimal obstruction extraction records with ancestry.
- Purpose-indexed equivalence and suppression records for tiling candidates.
- Research cockpit iteration packets for lower-bound and upper-bound workflows.
- A Milestone 2 certification bundle proving tiling iteration is replayable,
  suppressible, and non-authoritative until checked.

## Must Preserve

- Query remains the public entry point.
- Hadwiger owns tiling and motif domain meaning.
- Milestone 1 checker/proof/screening lanes remain the only mathematical
  authority lanes.
- Agent and human suggestions remain advisory/proposal input.
- Tiling equivalence and suppression remain planning evidence, not proof.
- Failed attempts remain structured graph memory and reusable evidence.
- Runtime invariant registration remains blocked unless reached through the
  proper lower-authority Query/relational path.

## Acceptance Evidence

- `cargo fmt -p hadwiger-research --check`
- `cargo check -p hadwiger-research`
- `cargo test -p hadwiger-research --jobs 1`
- Milestone 2 candidate/tiling certification tests pass.
- Compile-boundary tests prove no deep imports, no unchecked tiling execution,
  no floating-only authority, no advisory-to-proof promotion, and no raw
  candidate bypass of Query declaration entry.
- Line-cap and directory-cap audits remain clean.
- The certification bundle includes digest inventory rows for every required
  Milestone 2 artifact and counter snapshot.

## Sequencing Notes

Milestone 2 belongs immediately after Milestone 1 because the evidence,
screening, explanation, discovery, invariant, advisory, and cockpit substrate
now exists. Building tiling iteration before Milestone 1 would have encouraged
unchecked generator loops and lost failures. Building it now lets every
candidate enter through Query, every checker result retain authority, every
failure become graph memory, and every repeated dead end become mechanically
suppressed.

After Milestone 2, the project should be ready for human/agent iterative
exploration over real tiling and motif candidates.

## Self-Check

- Does the milestone solve a real structural problem? Yes: it turns tiling
  exploration from prose/generation into a typed candidate language with
  replayable evidence.
- Is the adversarial constraint precise and load-bearing? Yes: every phase
  prevents visual/advisory/generated candidates from leaking into authority.
- Does the roadmap justify this milestone now? Yes: Milestone 1 built the
  authority substrate; Milestone 2 gives that substrate concrete tiling inputs.
- Does the spec preserve crate authority boundaries? Yes: Query owns entry and
  contribution posture, Hadwiger owns tiling meaning, checkers own checked math
  authority.
- Are the phases carrying most of the real design information? Yes.
- Is each phase centered on one conceptual detail or boundary? Yes.
- Does each phase contain at least two adversarial tests? Yes.
- Could a competent engineer map this spec into honest types, modules, and
  tests? Yes.
- Does the milestone belong in this roadmap sequence? Yes.
