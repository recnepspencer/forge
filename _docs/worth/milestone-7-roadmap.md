# Worth Milestone 7 Roadmap: B-Rep Planar Boolean Program

> **Status:** Draft
>
> **Purpose:** break the B-rep planar boolean build into roadmap-sized
> sub-milestones without polluting the main Worth roadmap with the full split
> lattice.

## Goal

Define the real implementation sequence for `Milestone 7` as a band of smaller
milestones that closes planar B-rep booleans honestly before `Milestone 8`
introduces EMBER.

This document is intentionally narrower than the main roadmap. It exists
because "planar boolean splitting" is not one milestone-sized problem. It is a
stack of separately hard closures:

- workload truth
- certified common-plane reduction
- event extraction
- edge splitting
- loop reconstruction
- coplanar overlap region extraction
- fragment classification
- face assembly
- degeneracy cleanup
- topology legality
- replay / determinism
- planar metaboss closure

## Product Decision Lock

- `Milestone 7` is the B-rep planar boolean band only.
- `Milestone 8` still owns EMBER.
- The main Worth roadmap should keep the short `Milestone 7` and
  `Milestone 8` entries; this document holds the internal `7.x` split.
- Query owns the ordinary declaration, admission, readiness, route, receipt,
  envelope, and outcome boundary for planar booleans.
- `worth-kernel` consumes that lowered path.
- `worth-spatial` owns planar classification, projection, transforms, replay,
  diagnostics, and workload evidence semantics.
- `worth-topo` owns topology truth, topology legality, and topology-side
  workload construction.
- No `7.x` milestone may count synthetic fixtures, hand-built evidence rows,
  kernel summaries, or re-extraction replay as closeout proof.

## Why This Needs Its Own Roadmap

The main Worth roadmap is supposed to stay readable at the milestone level.
But planar B-rep boolean work is too large to treat as one implementation blob.

The hard part is not merely "run split / classify / assemble." The hard part is
that each of those words hides a large closure surface with its own edge cases,
determinism risks, and test-theatre failure modes. A single-plan milestone would
be too large for implementation planning and too easy to close dishonestly.

## Dependency Chain

The intended chain is:

`Milestone 6` -> `Milestone 6.5` -> `Milestone 7.0` -> `Milestone 7.1` ->
`Milestone 7.2` -> `Milestone 7.3` -> `Milestone 7.4` ->
`Worth Query Graph Authority Hardening Gate` -> `Milestone 7.5` ->
`Milestone 7.6` -> `Milestone 7.7` -> `Milestone 7.8` -> `Milestone 7.9`
-> `Milestone 7.10` -> `Milestone 7.11` -> `Milestone 8`

Each sub-milestone should leave behind a coherent, reviewable, replayable
artifact boundary for the next one.

## Milestone 7.0: Boolean Entry And Anti-Theatre Boundary

Freeze the public entry and proof boundary before any boolean execution work is
allowed to claim progress.

Closes:
- Query-backed planar boolean declaration family
- admission / denial / policy vocabulary
- workload-catalog boolean operand recipes
- workload evidence-ledger stage additions for boolean execution
- boolean user-outcome vocabulary
- compile-fail and contract fences against synthetic boolean proof

Done when:
- boolean workloads can only enter through real declaration and workload rails
- no synthetic path can masquerade as a real boolean closeout row

## Milestone 7.1: Certified Common-Plane Reduction

Freeze the honest reduction of admitted planar boolean operands into one shared
certified planar frame.

Closes:
- common-plane admission
- distinct-plane and inconsistent-plane denial
- certified operand projection into one canonical 2D basis
- deterministic basis identity and replay stability

Done when:
- every admitted planar boolean workload proves one certified common plane
- later split work does not invent its own local basis or coordinate folklore

## Milestone 7.2: Point / Segment / Interval Event Extraction

Freeze the event vocabulary that planar boolean splitting actually depends on.

Closes:
- proper segment-segment intersection extraction
- endpoint contact extraction
- shared-endpoint contact extraction
- collinear disjoint classification
- collinear touching classification
- partial-overlap interval extraction
- containment-overlap interval extraction
- identical and anti-parallel coincidence extraction
- typed denial or policy posture for degenerate micro-events

Done when:
- planar boolean event extraction is complete and deterministic for the admitted
  planar class
- later milestones consume typed event products instead of recomputing raw
  segment relations

## Milestone 7.3: Edge Splitting And Edge-Chain Normalization

Freeze edge-level topology rewriting from extracted events.

Closes:
- inserting intersection vertices into edges
- splitting one edge at many points
- deterministic split-point ordering
- overlap interval subdivision into canonical edge chains
- micro-interval merge and redundancy normalization
- edge identity and provenance preservation across splits

Done when:
- planar boolean edge splitting is receipt-backed, deterministic, and replayable
- later loop work receives canonical split edge chains rather than raw events

## Milestone 7.4: Loop Splitting And Loop Reconstruction

Freeze loop-level rebuilding on top of canonical split edges.

Closes:
- rebuilding loops from split edge chains
- preserving loop closure and orientation
- preserving outer / inner loop meaning
- birthing new loops from imprints
- splitting one loop into multiple islands
- typed denial or cleanup posture for collapsed or degenerate loops

Done when:
- the system can rebuild honest planar loops after split without manual fixture
  assembly

## Worth Query Graph Authority Hardening Gate

Refactor the `worth-topo`, `worth-spatial`, and `worth-kernel` production
surfaces that became transitional after Forge Query Milestone 9.9 so later
boolean phases consume Query graph touch obligation authority rather than local
ceremony.

Spec:
- [worth-query-graph-authority-hardening-gate.md](./worth-query-graph-authority-hardening-gate.md)

Closes:
- topology operator graph authority catalog reconciliation
- topology query-native receipt phase separation
- spatial evidence / Query authority separation
- evidence-stage index and raw-scan rejection
- primitive construction graph authority cleanup
- exact primitive birth selector contract or named Query API gap
- split-ledger and loop-ledger exclusivity preservation after Query 9.9

Done when:
- every 9.9-relevant Worth production surface is either covered by Query graph
  obligation authority, deleted, blocked by a named Query capability gap, or
  explicit certified residue

## Milestone 7.5: Coplanar Overlap Region Extraction

Freeze overlap-region extraction as its own closure surface.

Closes:
- overlap interval graph construction
- overlap island extraction
- shared-boundary versus shared-area distinction
- opposite-sense coincident boundary handling
- overlap winding normalization
- canonical overlap-region products suitable for classification

Done when:
- coplanar overlap is a first-class product of the boolean pipeline rather than
  a special-case side helper

## Milestone 7.6: Fragment Classification

Freeze result-fragment classification after split and overlap extraction.

Closes:
- inside / outside / boundary classification
- keep / discard labeling for union, intersect, and subtract
- coplanar-overlap classification rules
- typed ambiguity posture and policy-required exits
- deterministic classification tie-breaks

Done when:
- every admitted fragment has a typed classification result or a typed
  ambiguity / denial outcome

## Milestone 7.7: Planar Face Assembly

Freeze result construction from classified fragments.

Closes:
- assembling kept fragments into faces
- assembling holes correctly
- assembling multiple disjoint islands
- preserving orientation and containment
- distinguishing solid-boundary versus sheet-like planar outcomes
- preserving lineage / naming through assembly where admitted

Done when:
- B-rep planar boolean can produce honest result faces on the admitted class

## Milestone 7.8: Post-Split Degeneracy Cleanup

Freeze the cleanup layer that prevents split success from hiding poisoned
results.

Closes:
- zero-length edge cleanup
- zero-area and sliver-face cleanup
- needle-fragment cleanup
- micro-bridge removal
- duplicate intersection-vertex cleanup
- dangling topology cleanup
- redundant collinear subdivision cleanup
- deterministic cleanup ordering

Done when:
- admitted outputs either clean to a legal result or deny honestly with
  explicit cause

## Milestone 7.9: Topology Legality And Result Certification

Freeze topology-side certification of assembled and cleaned results.

Closes:
- loop legality after cleanup
- face legality after assembly
- shell / body legality for admitted planar outputs
- manifoldization or sheet demotion posture
- typed denial when cleanup cannot honestly certify a legal result

Done when:
- `worth-topo` gives the final legality answer for admitted planar boolean
  results

## Milestone 7.10: Replay / Checkpoint / Determinism Closure

Freeze replay-grade and checkpoint-grade stability across the entire B-rep
planar boolean lane.

Closes:
- replay parity
- checkpoint / non-checkpoint parity
- deterministic split ordering
- deterministic overlap normalization
- deterministic classification
- deterministic assembly and cleanup
- stable diagnostics and decision-log identity

Done when:
- the admitted B-rep lane is replay-safe and deterministically inspectable

## Milestone 7.11: Planar Metaboss Closure For B-Rep

Freeze the hostile closeout bar for planar B-rep booleans before EMBER starts.

Closes:
- coplanar apocalypse / overlap-storm pressure
- thin-feature labyrinth pressure
- scale-separation pressure
- cancellation-chain pressure
- high-valence and degeneracy pressure
- open / half-space / dirty denial honesty where applicable
- full replay-safe, localization-safe closeout across the admitted planar class

Done when:
- the B-rep planar boolean lane survives the real planar metaboss surface
- both admitted and denied paths are mechanically proven through the workload
  rails

## What This Defers To Milestone 8

This document intentionally does not widen into EMBER.

`Milestone 8` still owns:
- EMBER execution on the same public boolean boundary
- EMBER hostile closure
- dual-lane parity and divergence
- cross-lane replay and corruption-localization closure

## Recommended Use

Use this document as the planning map when selecting the next boolean build.

The normal loop should be:

1. choose the next `7.x` milestone
2. derive a phase-structured implementation plan inside that milestone
3. implement and close it before moving to the next one

This keeps the main roadmap readable while still giving the boolean program the
granularity it actually needs.
