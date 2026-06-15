# Worth Future Roadmap

## Purpose

This document defines the future work for Worth.

It is a future-only roadmap. It exists to sequence the work required to build
Worth as a spec-native, AI-native, manufacturing-grade geometry system on top
of the Forge runtimes without reintroducing the old failure mode of duplicated
authority across topology, geometry, validation, naming, orchestration,
interaction, and diagnostics.

This roadmap is intended to be the roadmap that can actually get Worth to a
Parasolid-class and eventually aerospace-trustworthy state, not a simplified
placeholder skeleton.

The operating rule is:

`commit spec, topology, naming, and binding truth canonically once, derive everything else honestly`

## Current Roadmap State

- `Milestone 1`: Complete
- `Milestone 2`: Complete
- `Forge Query Runtime Rewrite Gate`: Proposed, blocks further Milestone 3
  implementation and rewrites Worth runtime mutation onto Forge Query's
  aspect-native public mutation surface
- `Worth Read Composition Side Quest`: Closed for the Milestone 3 return gate;
  promoted bounded read composition into a first-class Forge Query product so
  Worth neighborhoods stop rebuilding caller-owned row joins
- `Worth Topology Domain Structure Gate`: Closed; `worth-topo` is rehomed into
  a topology-domain skeleton that cleanly separates B-rep truth, derived
  topology, validation, topology operators, projection, certification, and test
  support
- `Milestone 3`: Complete
- `Milestone 4`: Implemented in the current Worth kernel/spatial direction;
  final roadmap bookkeeping may still lag closeout evidence
- `Milestone 5`: Binding/rebinding authority and Query-native geometry
  hard-break work are the current predecessor for Milestone 6; do not treat
  Milestone 6 as starting from the older pre-Query-native geometry shape
- `Milestone 6.5`: Planned as the operational workload platform between exact
  planar contracts and booleans; hostile tests must consume real topology,
  binding, projection, retained replay, transform, response, and evidence rails
  rather than private synthetic fixtures
- Milestones `6+`: Planned

Milestone 1 closeout record:
- [milestone-1-closeout.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/worth/milestone-1-closeout.md)
Milestone 2 closeout record:
- [milestone-2-closeout.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/worth/milestone-2-closeout.md)
Milestone 3 closeout record:
- [milestone-3-closeout.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/worth/milestone-3-closeout.md)
Forge Query runtime rewrite gate:
- [forge-query-runtime-rewrite-plan.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/worth/forge-query-runtime-rewrite-plan.md)
Worth read composition side quest:
- [worth-query-domain-substrate.md](/Users/Esther/Documents/Programming/forge_workspace/worktree_2/_docs/worth/worth-query-domain-substrate.md)
Worth topology domain structure gate:
- [worth-topo-domain-structure-gate.md](/Users/Esther/Documents/Programming/forge_workspace/worktree_2/_docs/worth/worth-topo-domain-structure-gate.md)

## Global Adversarial Constraint

Worth must survive this hostile condition:

> A long-lived design system with deep feature history, persistent naming,
> branch-local edits, merge pressure, non-manifold intermediate topology,
> exact planar decisions, bounded curved approximation, topology-to-geometry
> rebinding, fillet and junction complexity, AI-authored graph edits,
> intent-sensitive interactive operations, replay, certification, and
> manufacturing audit requirements must preserve the same authoritative model
> truth, the same naming continuity conclusions, the same derived topology and
> geometry meaning, the same user-intent decisions, and the same causal
> explanations regardless of whether the system is reading live committed
> truth, branch-local truth, replayed truth, rebuilt derived state, or
> agent-authored command streams.

If Worth:

- lets derived topology or geometry masquerade as authoritative truth
- stores persistent naming as advisory metadata instead of truth
- allows topology legality to depend on derived validation alone
- hides ambiguous modeling intent behind heuristics instead of explicit policy
- spreads topology, geometry, naming, or trace meaning across multiple
  overlapping pseudo-authority layers
- allows bridge routing, replay, scheduling, or branch context to change
  derived conclusions for the same authoritative history
- fuses topology identity, naming identity, lineage identity, feature identity,
  and geometry anchoring into one ambiguous state model
- lets approximation, snapping, coalescence, fillet collapse, or policy
  fallback happen silently rather than explicitly, traced, and auditable
- crashes, hangs, or emits corrupted output where it should have either
  produced a correct result or an exact structured failure

then Worth has failed.

## Roadmap Rules

- Each milestone must describe a real Worth capability boundary, not a bag of
  chores.
- `forge-relational` owns authoritative truth semantics; Worth must not build a
  second truth runtime.
- `forge-signal` owns derived computation; Worth must not hide derived state as
  authority.
- `forge-runtime-bridge` owns truth-to-derived causality; Worth must not embed
  manual invalidation or manual continuity as substitute boundaries.
- Every Worth state family must be classified as authoritative, derived,
  certification-only, or interaction-only.
- Worth runtime/query work must enter through Forge Query. If Worth exposes a
  missing generic query/runtime capability, `forge-query` must be hardened
  rather than Worth inventing a local workaround.
- Persistent naming is first-class from day one rather than a later add-on.
- Non-manifold topology support must be explicit about what is admitted,
  denied, interpreted, or only certified.
- User intent for ambiguous operations must be explicit and inspectable rather
  than guessed silently by heuristics.
- Sequence numbers express dependency order, not staffing order.
- Every milestone must declare its own adversarial constraint.
- Every hot-path milestone must declare named complexity contracts and exact
  counter proof obligations.
- Any knowingly incomplete first ship must be marked as explicit debt rather
  than implied completeness.
- [test-requirements.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/worth/test-requirements.md)
  and
  [test-requirements_pt2.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/worth/test-requirements_pt2.md)
  are the authoritative milestone-closeout test-document set; roadmap
  milestones are not closed until their required named suites pass.

## Definition Of Done Rules

Every milestone in this roadmap must be read with the same definition-of-done
discipline.

A milestone is not done because:

- a demo works
- one happy-path operator succeeds
- one canonical shape succeeds
- a tetrahedron, cube, or other toy body can be created
- a single workflow path passes while general workflow classes remain missing

A milestone is only done when all of the following are explicit:

- `Admitted Surface`: exactly what capability classes are supported
- `Excluded Surface`: exactly what is still fail-closed or out of scope
- `Workflow Surface`: what workflow classes must work generically, not just for
  toy examples
- `Operator Closure`: which operator families are fully supported
- `Validator Closure`: which validator families must hold
- `Replay Closure`: what must replay identically
- `Diagnostics Closure`: what must be localizable and explainable
- `Determinism Closure`: what orderings, tie-breakers, and digests must be
  stable
- `Complexity / Proof Closure`: which counters and proofs must exist
- `Allowed Debt`: what incompleteness is still explicitly permitted

## Workflow Surface Rule

Worth milestones must close over workflow classes, not only over showcase
shapes.

Examples of insufficient closure:

- "primitive topology is done because tetrahedrons work"
- "loop handling is done because triangles and quads work"
- "face editing is done because boxes and prisms work"
- "fillets are done because one radius on one cube edge works"

Examples of required closure language:

- what happens for arbitrary `n`-edge loops
- what happens for arbitrary `n`-face shells
- what happens for arbitrary radial valence within the admitted class
- what happens for arbitrary branch-local edit histories within the admitted
  workflow class
- what happens when the workflow exits the admitted class and must fail cleanly

The roadmap must always prefer:

- "supports simple closed loops of arbitrary admitted cardinality"

over:

- "supports triangles, quads, and a few sample polygons"

and:

- "supports admitted shell-building workflows over arbitrary face counts"

over:

- "supports cubes, prisms, and a few hand-built solids"

## Milestone Closure Template

Every milestone should be read and eventually specified using the same closure
structure.

At minimum, each implementation-facing milestone spec should make all of these
surfaces explicit:

- `Admitted Surface`
- `Excluded Surface`
- `Workflow Surface`
- `Operator Closure`
- `Validator Closure`
- `Replay Closure`
- `Diagnostics Closure`
- `Determinism Closure`
- `Complexity / Proof Closure`
- `Allowed Debt`
- `Milestone Done When`

The most important of these for Worth is `Workflow Surface`.

This exists to prevent false closure on toy examples.

Examples:

- a primitive-topology milestone is not done because one tetrahedron and one
  cube can be built
- a loop milestone is not done because triangles and quads work
- a shell milestone is not done because a handful of hand-authored bodies work
- a blend milestone is not done because one cube edge accepts one radius

Instead, `Workflow Surface` must say what general class is admitted.

Examples of honest workflow-surface language:

- simple closed loops of arbitrary admitted cardinality
- shell-building workflows over arbitrary admitted face counts
- radial edit workflows over arbitrary admitted valence within the milestone's
  class
- branch-local edit histories over arbitrary admitted history length within the
  milestone's replay contract

If a workflow class is not admitted generally, the roadmap or milestone must
say so explicitly and require clean failure outside the admitted class.

## Runtime Ownership Boundaries

- `forge-relational` owns:
  - spec truth
  - topology truth
  - naming truth
  - geometry-binding truth
  - feature-intent truth
  - lineage-bearing authoritative history
  - commit-time invariant authority
  - same-commit graph creation semantics for entity and relation publication
- `forge-signal` owns:
  - topology materializations
  - geometry materializations
  - diagnostics-oriented derived views
  - selective recompute over Worth read models
  - derived analysis nodes such as manufacturability, tolerance stack-up, and
    future engineering analysis
- `forge-runtime-bridge` owns:
  - patch-to-invalidation routing
  - snapshot-backed truth evaluation for derived work
  - continuity and branch-aware bridge semantics
  - replayable causality across truth and derived work
- Worth owns:
  - schema meaning
  - invariant meaning
  - topology semantics
  - geometry semantics
  - feature semantics
  - interaction semantics
  - certification interpretation specific to the geometry domain

## Worth Crate Boundary Rule

The Worth crate split must stay explicit as the roadmap widens:

- `worth-topo` owns topology truth semantics, topology editing, topology
  certification, and topology hostility proof
- `worth-topo` must remain geometry-free apart from explicitly permitted opaque
  handles and topology-safe identifiers
- `worth-spatial` owns topology/geometry interaction semantics, including
  geometry binding, exact planar classification contracts, and later
  intersection-facing spatial contracts
- `worth-kernel` owns primitive construction, body construction, and higher
  engine programs that consume Query-lowered `worth-topo` and `worth-spatial`
  contracts through canonical declaration, receipt, and envelope lanes
- booleans require both `worth-spatial` and `worth-kernel`; they are not
  purely topo work

This boundary should be CI-enforced once the corresponding crates and contract
surfaces exist.

## Validator Family Progression

Worth should not treat "validation" as one bucket.
Validator families become mandatory in stages, and the roadmap should be read
with those closure gates in mind.

### Foundation Validators

These must be closed by `Milestone 1` and `Milestone 2`:

- reference integrity and ownership
- half-edge / loop wiring invariants
- radial-edge invariants for admitted NMT states
- persistent naming uniqueness and dangling-reference checks
- basic determinism guards for canonical ordering and replay stability

### Topology, Primitive, And Planar Contract Validators

These must be closed by `Milestone 3`, `Milestone 4`, `Milestone 5`, and
`Milestone 6`:

- shell / body closure and orientation
- region / cellular topology invariants
- Euler / genus / generalized characteristic checks
- degeneracy classification consistency
- primitive-construction topology closure validators
- topology-to-spatial contract completeness validators
- exact planar / predicate pipeline validators
- stronger determinism validators for chain-hostile replay and ordering

### Boolean And Curved Validators

These must be closed by `Milestone 7`, `Milestone 8`, and `Milestone 9`:

- boolean boundary certification
- split / classify / assemble consistency
- planar B-rep boolean result legality, replay, and diagnostics closure
- EMBER / B-rep parity or explicit divergence classification
- boolean corruption-localization and brutality-tier determinism
- trim loop closure in UV
- p-curve / coedge sense consistency
- UV domain and seam accounting checks
- shared-edge dual-trim compatibility
- xyz↔uv inversion residual checks
- tangent-event and coplanar-overlap encoding consistency

### Feature And Regeneration Validators

These must be closed by `Milestone 10` and `Milestone 11`:

- feature dependency determinism
- regeneration replay exactness
- selector and persistent-name resolution determinism
- cache / index staleness validators for regenerated read models

### Freeform Validators

These must be closed by `Milestone 12` and `Milestone 13`:

- freeform binding and trim integrity
- freeform escalation and unsupported-case classification consistency
- freeform chain-stability and degradation localization

### Specialized Feature Validators

These must be closed by `Milestone 14` through `Milestone 17`:

- edge-modification consistency validators
- blend collapse / overflow / continuity validators
- high-valence junction consistency validators
- blend hostility and localization validators

### History / Merge / Interaction Validators

These must be closed by `Milestone 18` through `Milestone 20`:

- historical lookup determinism
- name survival through split / merge
- merge conflict taxonomy determinism
- interaction intent preservation and replay validators
- UI / DSL parity validators

## Operator Family Progression

Worth should also treat topology and feature operators as explicit families
rather than a single vague mutation surface.

### Core Topology Operators

These should be admitted by `Milestone 3`:

- primitive entity lifecycle operators
- loop / boundary wiring operators
- radial splice and radial repair operators for admitted NMT states
- shell / wire membership edits

### Primitive Construction Operators

These should be widened across `Milestone 4`:

- primitive-body construction programs
- topology-certified construction chains
- shell / wire composition programs for admitted primitive families

### Spatial Contract And Planar Operators

These should be widened across `Milestone 5` and `Milestone 6`:

- topology-to-spatial binding / rebinding operators
- exact planar classification operators
- planar structural identity / fingerprint operators
- typed planar clean-fail and ambiguity exits

### Boolean Operators

These should be widened across `Milestone 7` and `Milestone 8`:

- B-rep boolean pipeline operators
- Query-lowered planar boolean declaration, readiness, route, receipt, and
  envelope operators
- planar boolean parity and divergence operators
- EMBER boolean pipeline operators
- split / classify / assemble / postprocess operators
- brutality-tier replay and corruption-localization operators

### Curved Operators

These should be widened across `Milestone 9`:

- anchored curve and trim operators
- curved split / classify / assemble operators
- tangent / drift repair or fail-closed operators

### Feature And Regeneration Operators

These should be widened across `Milestone 10` and `Milestone 11`:

- feature authoring operators
- parameter rewrite operators
- regeneration scheduling / replay operators
- selector and naming continuity operators

### Freeform Operators

These should be widened across `Milestone 12` and `Milestone 13`:

- freeform surface authoring operators
- freeform trim / intersection operators
- freeform repair or fail-closed operators

### Specialized Edge And Blend Operators

These should be widened across `Milestone 14` through `Milestone 17`:

- chamfer operators
- constant-radius fillet operators
- variable-radius fillet operators
- junction solving operators
- cascade repair or fail-closed operators

### History, Merge, And Intent Operators

These should be widened across `Milestone 18` through `Milestone 20`:

- history inspection operators
- merge and reconciliation operators
- intent-resolution operators
- UI / DSL authored command operators

## Critical Path And Parallel Tracks

Critical path:

- `Milestone 1` -> `Milestone 2` -> `Forge Query Runtime Rewrite Gate` ->
  `Worth Read Composition Side Quest` ->
  `Worth Topology Domain Structure Gate` -> `Milestone 3` ->
  `Milestone 4` -> `Milestone 5` -> `Milestone 6` -> `Milestone 6.5` ->
  `Milestone 7` ->
  `Milestone 8` -> `Milestone 9` -> `Milestone 10` -> `Milestone 11` ->
  `Milestone 12` -> `Milestone 13` -> `Milestone 14` -> `Milestone 15` ->
  `Milestone 16` -> `Milestone 17` -> `Milestone 18` -> `Milestone 19` ->
  `Milestone 20`

Parallel tracks:

- manufacturing and domain-analysis derived programs can deepen in parallel
  after `Milestone 9` once boolean and curved geometry certification
  boundaries are explicit
- collaborative interaction workflows in `Milestone 20` can overlap late
  `Milestone 19` once merge and identity semantics are frozen
- certification harness expansion can overlap late `Milestone 19` once hostile
  replay and merge semantics are stable

## Milestone 1: NMT Topology Truth And Naming Foundation

Engineering spec:
[milestone-1.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/worth/milestone-1.md)

Closeout:
[milestone-1-closeout.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/worth/milestone-1-closeout.md)

### Goal

Make non-manifold-capable topology truth, persistent naming, and the validation
ladder first-class before broader topology editing or geometry regeneration is
allowed to land.

### Adversarial Constraint

Shells, wires, open boundaries, radial structure, persistent names, and
branch-local history must coexist without requiring a second topology authority
outside relational truth.

### Admitted Surface

- authoritative Worth topology truth for:
  - body, lump, region, shell
  - face, loop, halfedge, edge, vertex
  - wire as a first-class topological concept
- authoritative persistent naming truth attached to admitted topology entities
- open boundaries and admitted non-manifold radial states as truth-bearing
  topology classes
- wire-branch topology within the admitted branch-topology class
- closed, orientable, genus-0 solid shells within the admitted shell-face-count
  class
- branch-local history over seeded and minimally edited topology truth

### Excluded Surface

- broad topology-edit operator families
- geometry binding and topology-to-geometry continuity
- full boolean, blend, or regeneration semantics
- unsupported non-manifold classes whose legality rules are not yet explicit
- vertex-pinch NMT classes
- void-shell, higher-genus, and non-orientable solid-shell classes

Excluded classes must fail cleanly rather than being silently represented
approximately or inferred by convention.

### Workflow Surface

Milestone 1 is not done because one tetrahedron, cube, or hand-authored toy
solid can be seeded.

It is only done when the admitted topology truth model can represent and
validate these workflow classes generically:

- simple closed loops of arbitrary admitted cardinality
- shells with arbitrary admitted face counts
- wires with arbitrary admitted segment counts
- wire branches with arbitrary admitted branch valence
- closed solid shells with arbitrary admitted face counts
- radial structures with arbitrary admitted valence within the milestone's NMT
  class
- branch-local seeded histories and local truth edits within the admitted
  topology surface

This milestone does not need to admit every topology workflow, but it must
state exactly which workflow classes are admitted generally and fail cleanly
outside that boundary.

### Operator Closure

- truth-authoring operators for all admitted topology entities and relations
- seed and local reseating workflows for admitted shell, wire, loop, and radial
  truth
- seed and local reseating workflows for admitted wire-branch and closed-shell
  truth
- persistent-name authoring and attachment workflows

Broad topology-edit families may remain out of scope, but no admitted
truth-authoring path may bypass the milestone's invariant ladder.

### Validator Closure

The following validator families must be closed for the admitted surface:

- reference integrity and ownership
- half-edge / loop wiring invariants
- radial-edge invariants for admitted NMT states
- wire-branch and admitted vertex-disk invariants
- shell closure, shell orientation, and admitted solid-boundary invariants
- persistent naming uniqueness and dangling-reference legality
- commit-boundary rejection of impossible local structural states
- derived shell, wire, boundary, and non-manifold interpretation

### Replay Closure

- seeded topology truth must replay identically
- local admitted reseating or truth-edit workflows must replay identically
- branch-local reads over the admitted surface must preserve the same topology
  and naming conclusions

### Diagnostics Closure

- every accepted and rejected truth-authoring path must emit replayable
  diagnostic evidence
- diagnostics must localize whether rejection occurred at commit-boundary
  validation or derived topology interpretation
- naming continuity conclusions must be inspectable even when later continuity
  logic is still incomplete

### Determinism Closure

- canonical ordering of admitted topology truth must be stable
- admitted replay histories must produce the same truth and derived-topology
  summaries
- naming attachment and uniqueness outcomes must not depend on iteration order

### Complexity / Proof Closure

- name the first commit-boundary topology validation contracts
- expose exact counters for:
  - local structural checks
  - rejected relation applications
  - derived topology fallback breadth
  - naming-continuity lookup breadth for admitted cases

### Allowed Debt

- broad edit-operator coverage may remain `Debt`
- richer topology certification beyond the admitted surface may remain `Debt`
- shell / wire truth, wire-branch truth, solid-shell truth, naming truth, and
  the validator ladder may not
- partial support for any admitted primitive family may not

### Milestone Done When

Milestone 1 is done only when Worth can author, validate, replay, and inspect
admitted shell, solid-shell, wire, wire-branch, loop, radial, and
persistent-name truth generically across the admitted workflow surface, with
explicit validator ownership, deterministic outcomes, and clean failure
outside the admitted class.

### Sequencing Notes

This belongs first because every later milestone depends on this substrate
being honest.

## Milestone 2: Derived Topology Materialization And Bridge-Causal Validation

> **Status:** Complete

Engineering spec:
[milestone-2.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/worth/milestone-2.md)

Closeout:
[milestone-2-closeout.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/worth/milestone-2-closeout.md)

### Goal

Make topology interpretation and validation incrementally derived, branch-safe,
and causally explainable rather than hidden in topo-owned caches or giant
validators.

### Adversarial Constraint

Large topology edits must invalidate only the intended derived topology work,
and replay must reproduce the same topology materialization and diagnostics.

### Admitted Surface

- signal-backed topology materializations for the milestone-1 admitted topology
  surface
- bridge mappings from topology and naming truth aspects into derived topology
  invalidation scopes
- derived topology validators for shell closure, wire classification, boundary
  interpretation, and non-manifold adjacency interpretation

### Excluded Surface

- full regeneration and feature-graph recompute
- geometry materialization and geometry-aware derived audits
- broad optimization beyond explicit and tested fallback paths

### Workflow Surface

Milestone 2 is not done because one derived topology view can be recomputed
after one tiny edit.

It is only done when the admitted derived-topology workflows operate
generically over:

- arbitrary admitted seeded topology truth
- arbitrary admitted local topology edits from Milestone 1
- arbitrary admitted shell, wire, and radial counts within the admitted truth
  class
- branch-local and replayed reads over the same admitted truth history

### Operator Closure

- bridge routing from truth aspects to derived topology scopes
- topology materialization refresh for admitted scope changes
- derived-validator execution over materialized shell, wire, boundary, and
  radial views

### Validator Closure

- derived shell closure interpretation
- derived wire classification
- derived boundary interpretation
- derived non-manifold adjacency interpretation
- cache / staleness correctness for topology materializations

### Replay Closure

- identical truth histories must replay to identical topology materializations
- identical truth histories must replay to identical bridge-routing summaries
- historical and branch-local reads must preserve the same derived-topology
  meaning for the same truth basis

### Diagnostics Closure

- explicit diagnostics for truth changes, rerun derived work, and changed
  structural interpretation
- diagnostics must identify invalidation scope, rerun scope, and final changed
  interpretation scope

### Determinism Closure

- bridge routing must be deterministic for the same truth delta
- derived topology summaries must be deterministic for the same truth basis
- cache or scheduling differences must not change the observable result

### Complexity / Proof Closure

- name topology materialization and invalidation contracts
- expose exact counters for:
  - invalidated targets
  - recompute breadth
  - whole-view fallbacks
  - cache-hit versus rebuild paths

### Allowed Debt

- unsupported fast-path narrowing may remain `Debt` if whole-view fallbacks are
  explicit and tested

### Milestone Done When

Milestone 2 is done only when derived topology is an honest, deterministic,
bridge-causal, rebuildable layer over the admitted topology truth surface, with
generic workflow coverage across admitted shell, wire, and radial histories
rather than single-view demo recomputes.

### Sequencing Notes

This belongs before topology editing because edit workflows need a trustworthy
derived topology surface and diagnostic story.

## Milestone 3: Topology Editing Core

> **Status:** Complete

### Goal

Establish the first honest topology-editing substrate for Worth instead of
bolting edits directly onto read models or ad hoc geometry programs.

### Adversarial Constraint

Local topology edits must preserve truth legality, naming attachment legality,
and replayability without requiring later re-foundation of the topology model.

### Admitted Surface

- typed mutation surfaces over authoritative topology truth
- admitted early topology edit workflows over shells, wires, loops, halfedges,
  and radial structure
- persistent-name-preserving and explicit-name-ambiguity edit outcomes

### Excluded Surface

- full boolean programs
- geometry-aware edit semantics
- broad healing and import-recovery operator families

Milestone 3 is explicitly `worth-topo` work. It must not import geometry
semantics that belong in `worth-spatial` or construction programs that belong
in `worth-kernel`.

### Workflow Surface

Milestone 3 is not done because a couple of Euler-style edits work on one
small body.

It is only done when the admitted edit workflows operate generically over:

- arbitrary admitted loops of arbitrary admitted cardinality
- arbitrary admitted shells over arbitrary admitted face counts
- arbitrary admitted wire-edit workflows
- arbitrary admitted local radial splice or reseating workflows within the
  milestone's NMT class

### Operator Closure

- primitive entity lifecycle operators for the admitted topology surface
- loop / boundary wiring operators for the admitted topology surface
- radial splice and radial repair operators for admitted NMT states
- shell / wire membership edits

### Validator Closure

- all milestone-1 commit validators remain closed after edits
- all milestone-2 derived validators remain closed after edits
- edit-specific naming legality and ambiguity checks are closed for admitted
  edit classes

### Replay Closure

- admitted edits must replay identically
- edit rejection must replay identically
- branch-local admitted edit histories must preserve the same conclusions

### Diagnostics Closure

- topology-edit diagnostics must expose changed structural scope
- diagnostics must localize rejected edits to the exact invariant or ambiguity
  boundary

### Determinism Closure

- edit application order within a deterministic journal must produce stable
  truth
- admitted name-preservation outcomes must be deterministic for the same edit
  and truth basis

### Complexity / Proof Closure

- name edit-application and changed-scope contracts
- expose exact counters for:
  - touched entities
  - touched relations
  - rejected edit scope
  - naming-resolution breadth during edits

### Allowed Debt

- broad operator coverage may remain `Debt`
- edit honesty may not

### Milestone Done When

Milestone 3 is done only when admitted topology-edit workflows operate
generically across the declared workflow surface, preserve validator closure
and naming honesty, and fail cleanly outside the admitted edit class instead of
silently depending on toy topology.

### Sequencing Notes

This belongs before planar hostile proof because the hostile proof must target a
real edit-capable substrate, not only seeded fixtures.
The
[Worth Read Composition Side Quest](/Users/Esther/Documents/Programming/forge_workspace/worktree_2/_docs/worth/worth-query-domain-substrate.md)
belongs inside this milestone's remaining expansion window before more
query-heavy hostile lanes widen, so topology edit certification does not keep
rebuilding neighborhood logic as caller-owned row joins.

## Milestone 4: Topology-Certified Primitive Construction

Engineering spec:
- [milestone-4.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/worth/milestone-4.md)

### Goal

Prove that Worth can construct the primitive and body classes enabled by the
topology architecture through an explicit kernel-to-topology pipeline before
booleans are allowed to claim readiness.

### Adversarial Constraint

Primitive and body construction must not smuggle geometry semantics into
`worth-topo`, must not bypass topology authority, and must not claim closure
from a cube, tetrahedron, or a few curated solids while general construction
classes remain unproven.

### Admitted Surface

- topology-certified primitive construction workflows authored by `worth-kernel`
- generic admitted construction families such as:
  - simplex-like solids
  - orthotope / box-like solids
  - prisms
  - pyramids
  - holed shells
  - wire bodies
- direct topology certification of constructed results

This is the first milestone where `worth-kernel` becomes a required crate in
the dependency story, but `worth-topo` still remains geometry-free and
topology-authoritative.

### Excluded Surface

- full boolean programs
- exact geometric classification beyond topology-safe contracts
- curved or freeform primitive construction classes

### Workflow Surface

Milestone 4 is not done because one tetrahedron, cube, prism, or showcase body
can be built.

It is only done when primitive construction operates generically over admitted
workflow classes such as:

- arbitrary admitted shell-building workflows over arbitrary admitted face
  counts
- arbitrary admitted body-construction workflows over the admitted primitive
  family ladder
- arbitrary admitted wire-body and shell-with-hole construction workflows
- arbitrary admitted replayed and branch-local primitive construction histories

### Operator Closure

- primitive construction programs in `worth-kernel`
- topology-certified construction chains
- admitted shell / wire composition and assembly operators

### Validator Closure

- primitive construction results satisfy all topology validators
- primitive construction topology legality is proven independently from any
  geometry assumptions
- naming and replay closure remain intact across constructed bodies

### Replay Closure

- admitted primitive construction workflows replay identically
- admitted rejected primitive construction workflows replay identically

### Diagnostics Closure

- construction failures localize to exact topology or kernel-contract boundaries
- diagnostics show whether failure came from construction intent, topology
  legality, or out-of-class admission

### Determinism Closure

- construction workflows produce stable topology truth and stable certification
  artifacts for the same admitted input history

### Complexity / Proof Closure

- name primitive construction and certification contracts
- expose exact counters for construction breadth, assembly breadth, and
  certification breadth
- freeze one canonical proof substrate before the milestone documentation pass:
  - canonical proof truth per family
  - named projections of that truth
  - typed verification results and verified artifacts
  - one stable, versioned, domain-separated digest protocol for proof identity

### Allowed Debt

- curved and freeform primitive classes may remain `Debt`
- topology-certified primitive closure over the admitted family ladder may not

### Milestone Done When

Milestone 4 is done only when admitted primitive and body construction
workflows operate generically over the admitted family ladder, certify through
the topology substrate honestly, and fail cleanly outside the admitted class.

### Sequencing Notes

This belongs before spatial binding, planar boolean contracts, and all boolean
pipelines because booleans need a real construction substrate, not only seeded
fixtures.

Milestone 4 also needs a late hardening tail before its documentation phase
closes. That tail now has two distinct jobs:

- `Phase 5.7` freezes the kernel proof substrate so later milestones inherit
  canonical truth, verified-artifact boundaries, and stable digest identity
- `Phase 5.8` through `Phase 5.11` freeze the matching `worth-spatial`
  substrate so later binding, planar, boolean, and audit milestones do not
  inherit stringly identity, ad hoc digests, informal phase progression, or a
  development-wide facade

That hardening still belongs in Milestone 4 because later binding, planar,
boolean, and audit milestones will inherit both the kernel proof model and the
spatial authority model directly.

## Milestone 5: Spatial Binding And Topology-To-Geometry Contracts

### Goal

Freeze the honest topology/geometry interaction boundary in `worth-spatial`
before exact planar work and boolean pipelines depend on it.

### Adversarial Constraint

Topology replacement, local rebuild, rebinding pressure, and primitive-body
construction must not erase the distinction between topology authority,
spatial/binding truth, and later kernel orchestration.

### Admitted Surface

- authoritative geometry-binding truth
- topology-to-geometry anchoring rules
- explicit binding semantics for:
  - surface bindings
  - curve bindings
  - coedge bindings
  - vertex geometry bindings
- robust carrier-local parameter-space anchor truth for admitted curve and
  surface bindings, replacing the typed unsupported placeholder intentionally
  left by Milestone 4 Phase 5.5.6
- diagnostics for rebinding and continuity decisions

This milestone belongs primarily to `worth-spatial`. It exists specifically so
geometry-facing contracts do not leak into `worth-topo`.

### Excluded Surface

- full boolean programs
- broad curved certification and tangent-hostility programs
- freeform / NURBS bindings beyond explicitly admitted classes

### Workflow Surface

Milestone 5 is not done because one body can be rebound to one surface.

It is only done when admitted binding workflows operate generically over:

- arbitrary admitted shell and face counts
- arbitrary admitted edge / coedge / vertex binding counts
- local topology replacement and rebuild workflows within the admitted binding
  class
- admitted historical and branch-local binding inspection workflows

### Operator Closure

- binding authoring operators for admitted carriers
- rebinding operators for admitted topology replacement workflows
- continuity-inspection operators over binding history

### Validator Closure

- geometry-binding legality stays separate from topology legality
- topology-to-geometry identity separation is explicit
- binding completeness for admitted entity classes is closed
- naming continuity inspectability remains independent from binding state

### Replay Closure

- admitted rebinding histories must replay identically
- historical reads over admitted binding workflows must preserve the same
  continuity conclusions

### Diagnostics Closure

- rebinding and continuity decisions must be localizable
- diagnostics must show whether a failure arose from topology legality,
  geometry-binding legality, or continuity ambiguity

### Determinism Closure

- binding lookup and rebinding outcomes must be deterministic
- identity separation between topology, naming, and geometry binding must be
  stable under replay

### Complexity / Proof Closure

- name geometry binding lookup and rebinding evaluation contracts
- expose exact counters for binding traversals and rebinding breadth

### Allowed Debt

- richer curved certification may remain `Debt`
- the authority boundary between topology truth and geometry binding may not

### Milestone Done When

Milestone 5 is done only when topology-to-geometry binding is an honest truth
surface over the admitted workflow class, with generic rebinding coverage,
stable identity separation, and clean failure where continuity is unjustified.

### Sequencing Notes

This belongs before exact planar contracts and before booleans because both
EMBER and B-rep boolean pipelines need an honest topology-to-spatial boundary.

## Milestone 6: Exact Planar Contracts, Structural Identity, And Boolean-Readiness

### Goal

Make the exact planar, structural-identity, and planar clean-fail substrate
honest enough that both boolean pipelines inherit a trustworthy spatial contract
instead of a pile of heuristics.

Milestone 6 must consume the Milestone 5 binding/rebinding and Query-native
geometry substrate rather than restating it. Its job is to prove that exact
planar meaning, structural identity, retained history, topology-to-spatial
contract completeness, projection consumption, and clean-fail behavior cohere
through the same authority boundaries established by Milestone 5.

### Adversarial Constraint

Coplanar storms, high-valence degeneracy, thin-feature pressure, local planar
rebuilds, topology replacement, retained-history replay, and long planar
histories must not drift, shred topology, or silently corrupt binding, naming,
projection-consumed facts, clean-fail outcomes, or structural-identity
conclusions.

### Admitted Surface

- exact-planar decision surfaces for topology-critical classifications
- structural identity and fingerprint surfaces for planar topology/geometry
  meaning
- explicit clean-fail surfaces for impossible or policy-gated planar cases
- topology-to-spatial contract completeness for admitted planar workflows
- Query-native retained, projected, recovered, and replayed planar facts where
  those surfaces are relevant
- spatially exact topology/geometry decisions required by later boolean work

This milestone should be driven primarily by `worth-spatial`, with `worth-topo`
consuming the resulting contracts rather than reimplementing geometric logic,
and `worth-kernel` assembling boolean-readiness proof without becoming a second
planar predicate or identity runtime.

The certified predicate machinery itself already lives in `worth-math`
(`orient2d`, `orient3d`, `incircle`, `in_sphere`, `CertifiedTriSign`,
`PrecisionEscalation`, `PrecisionMode`, and exact-rational budget tracking).
Milestone 6 must route admitted planar predicate authority through that existing
math substrate and use Query-native spatial facts to retain the local basis,
tolerance policy, movement/rotation posture, and certification context.

### Excluded Surface

- full hostile boolean certification
- curved approximation and tangent-heavy geometry programs
- freeform surface classes
- new non-Query geometry workflow transports or local kernel substitutes for
  retained planar truth

### Workflow Surface

Milestone 6 is not done because a few planar cuts or coplanar cases work on
boxes or prisms.

It is only done when the admitted planar exactness and identity surfaces apply
generically over:

- arbitrary admitted planar loop cardinalities
- arbitrary admitted shell face counts
- arbitrary admitted planar edit histories and local planar rebuilds
- arbitrary admitted coplanar and thin-feature cases within the milestone's
  exact-planar class
- arbitrary admitted retained/replayed planar histories within the milestone's
  exact-planar class

### Operator Closure

- exact-planar classification operators for admitted cases
- `PlanarLocalFrameCertificate`
- `ProjectPointToCertifiedPlane2D`
- `CertifiedSegmentSegment2D`
- `CertifiedPolygonWinding2D`
- `CertifiedSignedArea2D`
- `CoplanarOverlapContractExtractor`
- planar structural-identity and fingerprint lookup operators
- typed clean-fail outcomes for impossible or policy-gated planar cases
- Query-native retained/projection/recovery operators for admitted planar facts
  where those surfaces are relevant
- `PlanarContractBundleValidator`
- `PredicateCertificateConsumptionValidator`
- boolean-readiness certification operators that prove M7 can consume the
  planar substrate without inventing identity or predicate shortcuts

### Validator Closure

- predicate-pipeline validators for admitted planar cases, including proof that
  they consume `worth-math` certified signs and precision metadata rather than a
  local kernel or spatial substitute
- local-frame, certified projection, certified segment-contact,
  winding/containment, signed-area/degeneracy, and coplanar-overlap contract
  validators for admitted planar cases
- degeneracy classification consistency for admitted planar cases
- topology-to-spatial contract completeness validators
- structural identity separation from naming, lineage, topology identity, and
  binding identity

### Replay Closure

- identical planar histories must replay to identical classification outcomes
- identical planar histories must replay to identical structural identity
  digests
- retained planar facts and projection-consumed planar facts must replay to the
  same truth as live admitted planar workflows

### Diagnostics Closure

- exact-planar decisions and clean-fail outcomes must be localizable
- structural identity conclusions must be inspectable independently from naming
  and lineage
- failures must distinguish predicate uncertainty, topology contract failure,
  binding/rebinding failure, unsupported planar class, and policy denial

### Determinism Closure

- planar classifications must be deterministic under replay
- structural fingerprint computation must be invariant to legal ordering noise
- retained, projected, recovered, and replayed planar facts must preserve the
  same deterministic outcome class for equivalent semantic inputs

### Complexity / Proof Closure

- name planar classification and structural identity contracts
- name retained planar fact, projection consumption, recovery, and
  boolean-readiness contracts where those surfaces are admitted
- expose exact counters for precision escalation, identity lookup breadth,
  retained basis breadth, projection-consumption breadth, and clean-fail
  localization breadth

### Allowed Debt

- some extreme hostile cases may remain typed `Debt`
- crash-free, hang-free, and clean-fail behavior may not

### Milestone Done When

Milestone 6 is done only when exact-planar classification, structural identity,
retained/projection-consumed planar facts, and boolean-readiness certification
hold generically over the admitted planar workflow surface, with clean-fail
behavior outside the admitted class and without collapsing identity into naming,
lineage, topology identity, binding identity, or kernel-local workflow summaries.

### Sequencing Notes

This belongs after spatial binding/rebinding and the Query-native geometry hard
break, and before boolean pipelines, because booleans need exact planar
predicate authority, structural identity, retained planar truth, and
projection-consumable planar facts before split/classify/assemble work can be
trusted.

## Milestone 6.5: Operational Workload Platform

Engineering spec:
[milestone-6.5.md](/Users/Esther/Documents/Programming/forge_workspace/worktree_2/_docs/worth-kernel/milestone-6.5.md)

### Goal

Build the reusable Worth workload platform that makes real topology-to-geometry
proof easier than synthetic fixture staging.

Milestone 6.5 consumes M6 exact planar contracts and produces the operational
rails that hostile tests and later operators must use: topology workload seeds,
geometry binding workloads, surface support classification, local frame and
projection workloads, transform workloads, retained replay workloads, user
response outcomes, evidence ledgers, operator harnesses, and canonical workload
catalog recipes.

### Adversarial Constraint

Hostile tests and future operator proof must not be able to claim end-to-end
Worth evidence through hand-built spatial fixtures, label-only transforms,
re-extraction replay, fixture-generator arithmetic, or hand-filled evidence
ledgers.

### Admitted Surface

- topology-backed workload seeds for common and hostile model classes
- receipt-backed geometry binding, certified plane support, projection,
  transform, retained replay, diagnostics, and user response workloads
- evidence ledgers and honesty guards proving which stages actually ran
- operator harness support with coplanar overlap as the first concrete consumer
- canonical workload catalog recipes for real reusable test and certification
  setup

### Excluded Surface

- boolean split/classify/assemble execution
- implementation of future surface or feature operator families
- synthetic end-to-end fixture paths
- unsupported surface families masquerading as admitted support

### Workflow Surface

Milestone 6.5 is not done because one cube, tetrahedron, or overlap storm can
be built.

It is done only when the workload platform can build admitted topology-backed
shape families, bind them to geometry, certify planar support, project them,
transform them, retain and replay them, run the first operator consumer, and
emit evidence that blocks synthetic end-to-end claims.

### Operator Closure

- workload operator harness
- coplanar overlap workload operator
- unsupported operator family posture

### Validator Closure

- evidence ledger honesty guards
- synthetic fixture registration blockers
- topology-to-binding-to-projection receipt validation
- transform and retained replay validation
- human-readable response validation

### Replay Closure

- retained replay workloads must consume retained artifacts
- replay must not be modeled as re-extraction with the same inputs

### Diagnostics Closure

- unsupported workload stages must produce typed reasons
- no-options and denied user outcomes must explain their cause in readable
  language
- legacy synthetic fixture paths must produce diagnosable rejection

### Determinism Closure

- catalog recipes must produce stable workload identities and evidence digests
- equivalent transform workloads must converge through evidence, not labels

### Complexity / Proof Closure

- expose counters for topology breadth, binding breadth, surface support,
  projection, transform, retained replay, operator execution, and evidence
  ledger stages
- block counter claims based only on fixture-generator arithmetic

### Allowed Debt

- future surface and feature operator families may remain typed unsupported
- synthetic end-to-end claims, fake replay, fake transforms, and hand-filled
  ledgers may not remain

### Milestone Done When

Milestone 6.5 is done only when MB-M6-1 runs on the workload platform and
MB-M6-2 through MB-M6-8 each has its own workload-platform phase, explicit
outcome matrix, missing production feature list, and registration gate that
prevents synthetic or incomplete evidence from counting as closeout proof.

### Sequencing Notes

This belongs after M6 exact planar contracts and before M7 booleans because
booleans and hostile proof need trustworthy operational workloads, not private
test worlds.

## Milestone 7: Planar Boolean Foundation Through The B-Rep Pipeline

### Goal

Establish the first honest planar boolean foundation by hardening the B-rep
pipeline against the real planar workload surface that comes out of
`Milestone 6` and `Milestone 6.5`.

This milestone must also freeze the Query-owned declaration and lowering
contract that keeps planar boolean work future-compatible with EMBER without
pretending EMBER is already a co-equal admitted execution lane.

### Adversarial Constraint

Boolean work must not begin from a fake substrate, and planar boolean closure
must not depend on a caller-owned router, local status taxonomy, or
kernel-invented pseudo-Query lane.

The B-rep lane must survive planar coplanar overlap, thin-feature,
scale-separated, cancellation-chain, high-valence, unbounded/open, and dirty
input pressure through the real workload platform, while Query-owned
declaration, admission, readiness, route-plan, receipt, envelope,
projection-consumption, and ordinary-outcome artifacts already preserve the
future ability to run the same admitted planar workload through EMBER later
without re-authoring the public boundary.

### Admitted Surface

- first admitted planar boolean workflow classes
- explicit planar B-rep boolean pipeline
- explicit Query-owned planar boolean declaration family with support posture,
  readiness, route-plan, receipt, and envelope surfaces that classify:
  - planar workloads admitted to B-rep now
  - planar workloads reserved for future EMBER execution
  - planar workloads denied or policy-gated before execution
- typed clean-fail outcomes for unsupported or policy-gated boolean cases

This milestone belongs to Query as the ordinary runtime-facing boundary, with
`worth-kernel` consuming the lowered boolean declaration path over
`worth-topo` plus `worth-spatial` contracts. Booleans are not a pure-topology
milestone.

### Excluded Surface

- EMBER execution as an admitted required path
- full dual-pipeline parity closure
- full hostile boolean certification
- broad curved boolean classes
- freeform boolean classes

### Workflow Surface

Milestone 7 is not done because one union, subtract, or intersect works on one
pair of boxes.

It is only done when admitted boolean workflows operate generically over:

- arbitrary admitted planar body pairs
- arbitrary admitted coplanar and near-coplanar planar interactions within the
  milestone's class
- arbitrary admitted boolean replay histories
- the real planar metaboss suite through the B-rep lane
- Query-owned declaration artifacts that make the same admitted planar
  workload legible to a future EMBER lane without changing the public boolean
  entry contract

### Operator Closure

- B-rep boolean split / classify / assemble / postprocess pipeline
- Query-lowered planar boolean declaration entry, readiness, route-plan,
  receipt, and envelope operators
- workload-platform-backed replay, checkpoint, and causal-localization
  operators for admitted planar boolean families

### Validator Closure

- boolean boundary certification for admitted cases
- split / classify / assemble consistency for admitted cases
- planar result topology legality, cleanup, and clean-fail honesty for admitted
  B-rep outcomes
- Query declaration/support/readiness/route posture remains machine-checkable
  and per-family-row explicit rather than inferred from visible exports
- topology legality of boolean results remains subordinate to topology truth

### Replay Closure

- admitted B-rep boolean histories replay identically
- checkpointed and non-checkpointed B-rep histories converge to the same
  outcome class
- Query declaration, route, receipt, envelope, and denial posture replay
  identically for the same admitted planar workload history

### Diagnostics Closure

- boolean failures localize to exact phase and affected scope
- diagnostics must distinguish topology failure, spatial/predicate failure,
  Query-lane denial, and B-rep execution failure

### Determinism Closure

- the admitted B-rep pipeline must be deterministic under replay
- declaration classification, readiness, route-plan, denial, and admitted
  execution classification must be deterministic

### Complexity / Proof Closure

- name planar B-rep boolean contracts and the Query-owned declaration/lowering
  contract that future EMBER execution must inherit
- expose exact counters for split breadth, classify breadth, assemble breadth,
  postprocess breadth, replay breadth, localization breadth, and route-lowering
  breadth

### Allowed Debt

- EMBER execution may remain `Debt` only as a typed, explicit future-admitted
  lane
- full brutality closure may remain `Debt`
- the real planar B-rep lane and future-proof Query declaration/lowering
  contract may not

### Milestone Done When

Milestone 7 is done only when the planar B-rep boolean pipeline operates
honestly over the admitted planar workflow surface, including the planar
metaboss closure, while the public Query declaration/lowering contract already
preserves the future EMBER lane explicitly instead of baking B-rep-only
assumptions into the boundary.

### Sequencing Notes

This belongs before EMBER parity work and before curved geometry because Worth
needs one hardened real planar boolean lane before it can honestly compare two
lanes or widen into curved cases.

## Milestone 8: EMBER Boolean Lane And Dual-Pipeline Hostile Proof

### Goal

Add the EMBER planar boolean lane on top of the same Query declaration and
lowering contract and then prove both boolean lanes under hostile workloads
before curved surfaces,
freeform surfaces, fillets, or other advanced geometry are allowed to widen the
system.

### Adversarial Constraint

If the old boolean brutality families can still punch holes in the system, or
if the EMBER lane and B-rep lane can disagree silently about the same admitted
planar workload after Query has lowered them through the same declaration
family, then Worth is not ready to move on to curved surfaces.

This milestone must therefore absorb the brutality bar represented by the
legacy suites under
`crates/forge-kernel/src/operations/boolean/_deprecated/brutality/`, including:

- manifold tiers
- numerical tiers
- adversarial tiers
- MB1 through MB8 scenario families

### Admitted Surface

- explicit EMBER planar boolean pipeline over the same admitted planar routing
  surface
- hostile boolean proof harnesses over the admitted planar boolean foundation
- replay-safe corruption-localization and checkpoint diagnostics
- parity or typed-divergence proof between EMBER and B-rep boolean pipelines

### Excluded Surface

- curved boolean classes
- freeform boolean classes
- blend hostility beyond boolean contributors already admitted

### Workflow Surface

Milestone 8 is not done because one deep boolean chain or one pathological
coplanar case passes.

It is only done when hostile proof covers workflow classes such as:

- coplanar apocalypse and overlap storms
- thin labyrinth and micro-feature avalanche cases
- cancellation chains and deep boolean histories
- scale-separated and halfspace-storm cases
- singularity-star and ultimate-degeneracy cases
- accepted and rejected hostile workloads across both EMBER and B-rep pipelines

### Operator Closure

- EMBER boolean split / classify / assemble / postprocess pipeline
- Query-lowered parity, divergence, and cross-lane replay comparison operators
- hostile boolean replay and checkpoint operators
- brutality-tier causal localization operators
- typed outcome classification into exact success, exact structured failure, or
  exact typed pipeline divergence

### Validator Closure

- corruption-localization validators for hostile boolean chains
- deterministic replay validators for hostile boolean histories
- Query declaration/receipt/envelope parity validators across both lanes
- parity or divergence validators between EMBER and B-rep boolean runs

### Replay Closure

- replay parity for admitted hostile boolean workflows
- replay parity for accepted and rejected hostile boolean cases
- checkpointed and non-checkpointed hostile runs must converge to the same
  outcome class

### Diagnostics Closure

- chain-safe checkpoint and replay diagnostics for hostile boolean histories
- brutality-tier trace bundles with exact trigger localization
- every hostile failure must identify exact trigger step, phase, and affected
  scope

### Determinism Closure

- hostile boolean suites must produce stable outcome classes
- localization bundles and parity/divergence bundles must be deterministic for
  the same hostile input history

### Complexity / Proof Closure

- name boolean brutality, chain replay, and corruption-localization contracts
- expose exact counters for split breadth, classification breadth, checkpoint
  breadth, and localization scope
- satisfy the boolean brutality suites before curved surfaces are allowed to
  widen the roadmap

### Allowed Debt

- some extreme hostile cases may remain typed `Debt`
- the EMBER lane, parity/divergence surface, and brutality gate before curved
  surfaces may not

### Milestone Done When

Milestone 8 is done only when hostile boolean workloads across the admitted
workflow surface either succeed exactly or fail with exact localized proof,
with replay-safe outcome classification, deterministic parity or explicit typed
divergence between EMBER and B-rep, and no silent corruption, crash, or
hang across either EMBER or B-rep paths.

### Sequencing Notes

This belongs before curved geometry. Worth is not allowed to move on to curved
surfaces until the boolean brutality bar is satisfied.

## Sequencing Correction Note

The corrected dependency chain through the boolean gate is now:

- `Milestone 3`: topology editing
- `Milestone 4`: topology-certified primitive construction
- `Milestone 5`: spatial binding and topology-to-geometry contracts
- `Milestone 6`: spatially exact planar contracts and structural identity
- `Milestone 6.5`: operational workload platform and hostile-test rails
- `Milestone 7`: planar B-rep boolean foundation and planar metaboss closure
- `Milestone 8`: EMBER lane plus dual-pipeline hostile boolean proof

The milestones that follow in this roadmap were originally written against the
older sequence and should now be read as provisional placeholders until they
are renumbered and re-specified against this corrected dependency chain.

## Milestone 9: Feature Intent Core And Spec Truth

### Goal

Freeze the authoritative feature-intent model and spec-graph semantics before
full regeneration and specialized feature families build on top of them.

### Adversarial Constraint

Feature truth must remain stable across replay, branch-local history, and
future specialized feature families without redefining earlier topology or
geometry contracts.

### Admitted Surface

- authoritative feature-intent truth model
- explicit feature dependency semantics
- spec truth surfaces distinct from derived regeneration
- diagnostics tying feature intent to affected truth domains

### Excluded Surface

- full regeneration execution breadth across the complete feature graph
- broad feature catalog completeness
- specialized feature families that bypass the core intent model

### Workflow Surface

Milestone 9 is not done because one or two features can be authored into the
spec graph.

It is only done when admitted feature-truth workflows operate generically over:

- arbitrary admitted feature counts
- arbitrary admitted dependency fan-in and fan-out within the milestone's
  feature class
- branch-local feature-history inspection workflows
- parameter rewrite workflows over the admitted feature surface

### Operator Closure

- feature authoring operators
- parameter rewrite operators
- dependency authoring and inspection operators

### Validator Closure

- dependency determinism and legality
- feature-truth separation from derived regeneration
- feature-to-topology / geometry domain-affect declarations

### Replay Closure

- identical feature-truth histories must replay identically
- branch-local feature histories must preserve the same feature-truth
  conclusions

### Diagnostics Closure

- diagnostics must tie feature intent to affected truth domains and dependency
  surfaces

### Determinism Closure

- dependency ordering and feature-truth evaluation must be deterministic for
  the same admitted feature graph

### Complexity / Proof Closure

- name feature dependency and intent-evaluation contracts
- expose exact counters for affected feature breadth and dependency traversals

### Allowed Debt

- broad feature catalog coverage may remain `Debt`
- spec truth and dependency honesty may not

### Milestone Done When

Milestone 9 is done only when feature intent is an honest authoritative truth
surface across the admitted feature workflow class, with deterministic
dependencies, replay-safe histories, and no host-side convention standing in
for truth.

### Sequencing Notes

This belongs before regeneration and before fillets because fillets are feature
semantics, not just geometry semantics.

## Milestone 10: Regeneration, Dependency Execution, And Replay Parity

### Goal

Make authored intent regenerate topology and geometry through runtime-native
commit and derive flows with explicit replay parity.

### Adversarial Constraint

Feature edits, parameter changes, and graph rewrites must preserve the same
truth and derived conclusions whether executed live, replayed, or evaluated on
branch-local histories.

### Admitted Surface

- bridge-aware derived regeneration paths
- deterministic dependency execution over admitted feature truth
- diagnostics connecting feature intent to topology and geometry consequences
- replayable spec-graph-driven Worth workflows

### Excluded Surface

- broad feature catalog coverage beyond admitted families
- specialized-feature regeneration that has not yet been explicitly admitted
- interaction-language surfaces that bypass regeneration truth

### Workflow Surface

Milestone 10 is not done because one feature graph rebuilds successfully.

It is only done when admitted regeneration workflows operate generically over:

- arbitrary admitted feature-graph sizes
- arbitrary admitted dependency depths
- topology-only and geometry-only delta workflows
- live, replayed, and branch-local regeneration over the same admitted feature
  history

### Operator Closure

- regeneration scheduling operators
- dependency execution operators
- replayed-regeneration operators
- topology-only versus geometry-only recompute discrimination

### Validator Closure

- regeneration replay parity
- dependency-execution determinism
- derived-state disposability and rebuildability

### Replay Closure

- live and replayed regeneration must produce the same admitted conclusions
- branch-local regeneration must preserve branch-local truth semantics

### Diagnostics Closure

- diagnostics must connect feature intent to topology and geometry consequences
- diagnostics must identify topology-only versus geometry-only recompute paths

### Determinism Closure

- dependency execution order and final regenerated result must be deterministic
  for the same admitted feature history

### Complexity / Proof Closure

- name feature regeneration and dependency-execution contracts
- expose exact counters for topology-only versus geometry-only recompute and
  fallback full regeneration breadth

### Allowed Debt

- broad feature workflow coverage may remain `Debt`
- regeneration parity may not

### Milestone Done When

Milestone 10 is done only when admitted feature-driven regeneration workflows
are deterministic, replay-safe, bridge-causal, and generic across the admitted
feature-graph workflow surface, without regeneration becoming a second runtime.

### Sequencing Notes

This belongs before specialized features because they must be implemented as
honest feature/regeneration programs.

## Milestone 11: NURBS And General Freeform Surface Foundation

### Goal

Establish the first honest freeform surface substrate rather than stretching the
analytic-curved layer past what it can honestly support.

### Adversarial Constraint

Freeform surface representation must be introduced without silently weakening
the explicit approximation, anchoring, and replay boundaries already
established for the rest of Worth.

### Admitted Surface

- freeform surface truth surfaces
- explicit freeform evaluation and certification boundaries
- diagnostics for freeform-specific escalation and unsupported cases

### Excluded Surface

- hostile freeform proof beyond the admitted foundation
- broad freeform feature catalog coverage
- unsupported freeform classes disguised as admitted NURBS coverage

### Workflow Surface

Milestone 11 is not done because one NURBS patch can be represented and
sampled.

It is only done when admitted freeform workflows operate generically over:

- arbitrary admitted freeform patch counts
- arbitrary admitted trim counts
- arbitrary admitted chained evaluation and rebinding workflows within the
  milestone's freeform class

### Operator Closure

- freeform surface authoring operators
- freeform evaluation operators
- typed unsupported-case exits for unadmitted freeform classes

### Validator Closure

- freeform binding and trim integrity for admitted classes
- explicit unsupported-case classification
- preservation of exact / bounded / policy-gated outcome distinctions

### Replay Closure

- admitted freeform histories must replay identically
- unsupported-case classification must replay identically

### Diagnostics Closure

- freeform escalation and unsupported-case diagnostics must be localizable and
  machine-checkable

### Determinism Closure

- admitted freeform evaluation outcomes and unsupported-case boundaries must be
  deterministic

### Complexity / Proof Closure

- name freeform evaluation and certification contracts
- expose exact counters for freeform escalation and unsupported-case breadth

### Allowed Debt

- broad freeform coverage may remain `Debt`
- explicit unsupported-case handling may not

### Milestone Done When

Milestone 11 is done only when freeform support is an honest admitted workflow
surface with generic patch and trim coverage across the admitted class, clear
unsupported boundaries, and no silent weakening of earlier geometry contracts.

### Sequencing Notes

This belongs before general freeform hostile proof and before full freeform
feature programs.

## Milestone 12: NURBS / Freeform Hostile Proof Program

### Goal

Prove the freeform layer under hostile workloads before Worth claims broad
freeform-kernel legitimacy.

### Adversarial Constraint

Freeform intersection, trimming, and chained freeform histories must either
succeed within declared bounds or fail with exact structured diagnostics.

### Admitted Surface

- hostile freeform proof harnesses over the admitted freeform foundation
- chain-local degradation diagnostics for freeform histories
- replay parity for admitted freeform workflows

### Excluded Surface

- unsupported freeform classes beyond the admitted hostile-proof surface
- blend or interaction surfaces not yet admitted to the freeform program

### Workflow Surface

Milestone 12 is not done because one hard freeform case passes.

It is only done when hostile proof covers workflow classes such as:

- chained freeform histories of arbitrary admitted length
- trim-heavy workloads over arbitrary admitted trim counts
- freeform escalation and degradation workloads within the admitted class

### Operator Closure

- hostile freeform replay and checkpoint operators
- degradation-localization operators
- typed exact-success versus exact-structured-failure classification

### Validator Closure

- freeform chain-stability validators
- degradation-localization validators
- deterministic hostile replay validators

### Replay Closure

- accepted and rejected hostile freeform cases must replay identically

### Diagnostics Closure

- freeform failures must identify trigger step, degradation scope, and
  unsupported or exhausted boundary exactly

### Determinism Closure

- hostile freeform suites must produce stable outcome classes and stable
  localization artifacts

### Complexity / Proof Closure

- name freeform chain-stability and degradation-localization contracts
- expose exact counters for freeform chain-local degradation checks
- satisfy the first hostile freeform proof suites once added to
  [test-requirements.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/worth/test-requirements.md)

### Allowed Debt

- some extreme hostile freeform cases may remain typed `Debt`
- clean localization may not

### Milestone Done When

Milestone 12 is done only when hostile freeform workflows across the admitted
workflow surface either succeed within declared bounds or fail with exact,
replay-safe localization, without silent drift, crash, or hang.

### Sequencing Notes

This belongs before claiming broad NURBS-grade kernel coverage.

## Milestone 13: Chamfers And Edge-Modification Features

### Goal

Ship the first specialized edge-modification feature family on top of the now-
honest topology, geometry, and regeneration stack.

### Adversarial Constraint

Edge modification must not bypass feature semantics, naming continuity, or
topology legality.

### Admitted Surface

- chamfer feature truth and regeneration semantics
- diagnostics for chamfer-specific failure cases

### Excluded Surface

- fillet-specific or blend-junction semantics
- broad edge-modification families not explicitly admitted

### Workflow Surface

Milestone 13 is not done because one edge on one box can be chamfered.

It is only done when admitted chamfer workflows operate generically over:

- arbitrary admitted edge counts
- arbitrary admitted shell face counts touched by the chamfer workflow
- branch-local and replayed chamfer histories within the admitted feature class

### Operator Closure

- chamfer authoring operators
- chamfer regeneration operators
- typed chamfer failure-localization operators

### Validator Closure

- chamfer legality relative to topology truth
- chamfer naming continuity for admitted cases
- chamfer replay parity

### Replay Closure

- admitted chamfer histories must replay identically

### Diagnostics Closure

- chamfer-specific failure cases must be localized to exact edges and affected
  scope

### Determinism Closure

- admitted chamfer outcomes must be deterministic for the same feature history

### Complexity / Proof Closure

- name chamfer regeneration and failure-localization contracts
- expose exact counters for failed edge modifications and affected scope

### Allowed Debt

- broad chamfer families may remain `Debt`
- feature-truth honesty may not

### Milestone Done When

Milestone 13 is done only when chamfer workflows are honest feature programs
across the admitted edge-modification workflow surface, not just one-off edge
surgeries on showcase solids.

### Sequencing Notes

This belongs before fillets because it is a simpler specialized feature family
and should harden the edge-modification stack first.

## Milestone 14: Constant-Radius Fillet Foundation

### Goal

Ship the first honest fillet family as a feature/regeneration program rather
than as special-case geometry surgery.

### Adversarial Constraint

Constant-radius fillets must preserve topology legality, naming continuity, and
replay parity without silent sliver creation or hidden non-manifold side
effects.

### Admitted Surface

- constant-radius fillet feature truth and regeneration semantics
- diagnostics for fillet-specific failure and collapse cases

### Excluded Surface

- variable-radius families
- advanced junction solving
- hostile blend certification beyond the admitted constant-radius surface

### Workflow Surface

Milestone 14 is not done because one cube edge accepts one constant radius.

It is only done when admitted constant-radius workflows operate generically
over:

- arbitrary admitted edge-set selections
- arbitrary admitted local neighborhood sizes
- branch-local and replayed constant-radius histories within the admitted class

### Operator Closure

- constant-radius fillet authoring operators
- constant-radius regeneration operators
- collapse-localization operators for admitted failure modes

### Validator Closure

- fillet legality relative to topology and geometry truth
- no-silent-sliver and no-hidden-non-manifold checks for admitted cases
- constant-radius naming continuity for admitted cases

### Replay Closure

- admitted constant-radius histories must replay identically

### Diagnostics Closure

- fillet-specific failure and collapse cases must be localized exactly

### Determinism Closure

- admitted constant-radius outcomes must be deterministic for the same feature
  history

### Complexity / Proof Closure

- name fillet regeneration and collapse-localization contracts
- expose exact counters for collapse attempts and affected scope

### Allowed Debt

- variable-radius and advanced junctions may remain `Debt`
- constant-radius honesty may not

### Milestone Done When

Milestone 14 is done only when constant-radius fillets are honest feature and
regeneration workflows across the admitted edge-set workflow surface, with
explicit collapse behavior and no toy-shape loophole.

### Sequencing Notes

This belongs before variable-radius fillets and hostile blend proof.

## Milestone 15: Variable-Radius Fillets, Junctions, And Blend-Cascade Honesty

### Goal

Finish the blend stack to the point where variable-radius blends, junctions,
and cascade conditions have explicit semantics and explicit failure behavior.

### Adversarial Constraint

Variable-radius blends, high-valence junctions, thin-feature swallowing,
grazing tangencies, and long chained blend histories must not silently create
slivers, hidden non-manifold state, or undiagnosed continuity breakage.

### Admitted Surface

- variable-radius blend semantics
- junction and cascade diagnostics
- explicit failure surfaces for:
  - radius overflow
  - zero-radius collapse
  - thin-feature swallow
  - tangent ambiguity
  - continuity loss

### Excluded Surface

- advanced blend families not explicitly admitted
- hostile blend certification beyond this milestone

### Workflow Surface

Milestone 15 is not done because one variable-radius example and one junction
case work.

It is only done when admitted blend workflows operate generically over:

- arbitrary admitted variable-radius rail counts
- arbitrary admitted junction valence within the milestone's class
- arbitrary admitted cascade depths within the milestone's class
- branch-local and replayed blend histories across the admitted class

### Operator Closure

- variable-radius fillet operators
- junction solving operators
- cascade handling operators
- typed failure operators for the admitted blend-failure taxonomy

### Validator Closure

- continuity-loss validation
- no-silent-collapse and no-silent-sliver validation
- admitted junction-consistency validation

### Replay Closure

- admitted variable-radius and junction histories must replay identically
- typed failure outcomes must replay identically

### Diagnostics Closure

- junction, cascade, and continuity failures must be localized exactly and
  emitted as machine-checkable diagnostics

### Determinism Closure

- admitted junction solving and failure classification must be deterministic

### Complexity / Proof Closure

- name junction solving and continuity-check contracts
- expose exact counters for junction breadth and continuity fallback breadth

### Allowed Debt

- some advanced blend families may remain `Debt`
- explicit failure and no-silent-collapse behavior may not

### Milestone Done When

Milestone 15 is done only when the admitted variable-radius, junction, and
cascade workflows are honest across their admitted workflow classes, with
explicit failure taxonomy and no silent collapse or continuity drift.

### Sequencing Notes

This belongs before hostile blend proof and before branch/merge semantics widen
the blast radius of blend workflows.

## Milestone 16: Hostile Blend Proof Program

### Goal

Prove the blend layer under hostile workloads before Worth claims real
fillet/chamfer credibility.

### Adversarial Constraint

High-valence blend junctions, variable-radius collapse pressure, and chained
blend histories must either succeed honestly or fail with exact structural
causes.

### Admitted Surface

- hostile blend proof harnesses over the admitted blend foundation
- blend failure-localization and continuity-loss diagnostics

### Excluded Surface

- unsupported blend families beyond the admitted hostile-proof surface

### Workflow Surface

Milestone 16 is not done because one nasty fillet example clean-fails.

It is only done when hostile proof covers workflow classes such as:

- high-valence junction workloads across arbitrary admitted junction counts
- variable-radius collapse workloads across arbitrary admitted history length
- chained blend histories across the admitted blend class

### Operator Closure

- hostile blend replay and checkpoint operators
- continuity-loss and failure-localization operators
- typed exact-success versus exact-structured-failure classification

### Validator Closure

- hostile blend localization validators
- continuity-loss validators
- deterministic hostile replay validators

### Replay Closure

- accepted and rejected hostile blend cases must replay identically

### Diagnostics Closure

- every hostile blend failure must identify exact trigger step and affected
  structural scope

### Determinism Closure

- hostile blend suites must produce stable outcome classes and stable
  localization artifacts

### Complexity / Proof Closure

- name blend hostility localization contracts
- expose exact counters for hostile junction and collapse-localization breadth
- satisfy the first hostile fillet proof suites in
  [test-requirements.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/worth/test-requirements.md)

### Allowed Debt

- some extreme hostile blend cases may remain typed `Debt`
- clean localization may not

### Milestone Done When

Milestone 16 is done only when hostile blend workflows across the admitted
workflow surface either succeed honestly or fail with exact, replay-safe
structural causes, without silent slivers, silent non-manifold state, crash,
or hang.

### Sequencing Notes

This belongs before branch/merge semantics widen the blast radius of blend
workflows.

## Milestone 17: Branching, History, And Identity Evolution For Worth Models

### Goal

Freeze branch-local history, identity evolution, and historical inspection
semantics before full merge and collaborative interaction workflows depend on
them.

### Adversarial Constraint

Branch-local topology, naming, feature, geometry-binding, and specialized-
feature evolution must remain isolated and historically inspectable without
drifting into linear-history assumptions.

### Admitted Surface

- branch-aware Worth model semantics
- historical inspection surfaces for topology, naming, feature, and geometry-
  binding evolution
- identity-evolution handling built on relational lineage
- diagnostics for branch-local continuity and historical resolution

### Excluded Surface

- full merge semantics
- collaborative interaction semantics that depend on unfrozen history rules

### Workflow Surface

Milestone 17 is not done because one branch can be inspected manually.

It is only done when admitted history workflows operate generically over:

- arbitrary admitted branch counts
- arbitrary admitted history depths
- arbitrary admitted identity-evolution chains across topology, naming,
  feature, and geometry-binding truth

### Operator Closure

- history inspection operators
- branch comparison operators
- identity-evolution lookup operators

### Validator Closure

- branch isolation validation
- historical lookup determinism
- identity-evolution consistency over admitted truth families

### Replay Closure

- admitted branch-local histories must replay identically
- history inspection over the same branch and checkpoint basis must replay
  identically

### Diagnostics Closure

- branch-local continuity and historical resolution diagnostics must be
  localizable and queryable

### Determinism Closure

- historical lookup and branch comparison outcomes must be deterministic

### Complexity / Proof Closure

- name historical lookup and branch comparison contracts
- expose exact counters for historical traversals, continuity lookups, and
  branch-local comparisons

### Allowed Debt

- advanced history acceleration may remain `Debt`
- branch honesty and identity evolution semantics may not

### Milestone Done When

Milestone 17 is done only when branch-local history and identity evolution are
honest across the admitted workflow surface, with deterministic inspection and
no drift into linear-history assumptions.

### Sequencing Notes

This belongs before merge and before collaborative intent workflows because
both depend on stable branch and identity semantics.

## Milestone 18: Merge, Conflict Taxonomy, And Multi-Branch Intent Semantics

### Goal

Consume the relational merge substrate honestly for Worth models so merge and
multi-branch intent evolution become explicit and replayable.

### Adversarial Constraint

Two branches with topology, naming, feature, geometry-binding, specialized-
feature, and interaction-authored intent evolution must merge or fail with
explicit semantics rather than heuristic conflict handling.

### Admitted Surface

- merge-aware Worth identity and continuity handling
- Worth-level conflict taxonomy for topology, naming, feature, geometry-
  binding, specialized-feature, and intent evolution
- diagnostics tying merge outcomes back to authoritative lineage and merge
  artifacts

### Excluded Surface

- advanced merge classes not explicitly admitted
- interaction workflows that assume unsupported merge semantics succeed

### Workflow Surface

Milestone 18 is not done because one simple branch pair merges.

It is only done when admitted merge workflows operate generically over:

- arbitrary admitted branch-pair histories
- arbitrary admitted continuity splits and name conflicts within the milestone
  merge class
- arbitrary admitted multi-domain conflicts across topology, naming, feature,
  geometry-binding, and intent truth

### Operator Closure

- merge comparison operators
- continuity-under-merge operators
- typed conflict-taxonomy and unsupported-merge operators

### Validator Closure

- merge conflict taxonomy determinism
- naming and continuity preservation or explicit failure under merge
- explicit unsupported-merge classification

### Replay Closure

- admitted merge outcomes and admitted merge failures must replay identically

### Diagnostics Closure

- merge diagnostics must tie outcomes back to lineage, conflict class, and
  continuity consequences exactly

### Determinism Closure

- merge comparison, conflict classification, and admitted merge outcomes must
  be deterministic

### Complexity / Proof Closure

- name merge comparison, continuity-under-merge, and merge-diagnostic contracts
- expose exact counters for continuity splits, unsupported merge classes, and
  intent conflict classes

### Allowed Debt

- some advanced merge classes may remain `Debt` if unsupported behavior is
  explicit and typed

### Milestone Done When

Milestone 18 is done only when admitted merge workflows across the admitted
multi-branch workflow surface either merge with explicit semantics or fail with
typed conflict and continuity diagnostics, without heuristic ambiguity hiding.

### Sequencing Notes

This belongs before collaborative interaction language because multi-branch
intent workflows depend on merge semantics being stable.

## Milestone 19: Interaction Language, AI Workflows, And Intent-Explicit UX

### Goal

Make Worth usable through AI-native and human-native interaction layers that
preserve intent explicitly instead of hiding ambiguity behind heuristics.

### Adversarial Constraint

Commands such as merging walls, wiring MEP runs, or reconciling adjacent
features must produce the same explicit intent semantics whether initiated by a
human through a cursor-IDE-like interface or by an AI agent through a DSL, and
ambiguous operations must ask for intent rather than guessing silently.

### Admitted Surface

- a structured DSL for AI-authored Worth operations
- a cursor-IDE-style interaction contract over Worth truth and diagnostics
- typed intent-resolution surfaces for ambiguous modeling operations
- diagnostics and history artifacts that record:
  - requested intent
  - alternatives presented
  - final chosen interpretation
  - downstream topology/geometry consequences

### Excluded Surface

- broad command-surface completeness
- hidden heuristic auto-resolution for unsupported ambiguous workflows

### Workflow Surface

Milestone 19 is not done because one DSL command maps to one UI action.

It is only done when admitted interaction workflows operate generically over:

- arbitrary admitted DSL-authored command sequences
- arbitrary admitted cursor-IDE interaction histories
- arbitrary admitted ambiguous-intent operations within the milestone's command
  class
- branch-local and replayed interaction histories over the admitted surface

### Operator Closure

- DSL lowering operators
- interactive intent-resolution operators
- UI / DSL parity operators
- typed ambiguity-prompt and typed unsupported-command exits

### Validator Closure

- interaction intent preservation
- UI / DSL parity for the same admitted intent
- no-silent-heuristic-resolution validation

### Replay Closure

- admitted interaction histories must replay identically
- admitted intent choices must reproduce the same downstream model conclusions

### Diagnostics Closure

- diagnostics and history artifacts must record requested intent,
  alternatives presented, chosen interpretation, and downstream consequences

### Determinism Closure

- DSL lowering and interaction-intent outcomes must be deterministic for the
  same admitted interaction history and intent inputs

### Complexity / Proof Closure

- name DSL lowering, interactive intent resolution, and replay-of-intent
  contracts
- expose exact counters for intent prompts, auto-admitted cases, rejected
  ambiguous cases, and replay parity checks

### Allowed Debt

- broad command-surface coverage may remain `Debt`
- intent explicitness and replayable interaction history may not

### Milestone Done When

Milestone 19 is done only when admitted UI and DSL workflows preserve explicit
intent across the admitted interaction workflow surface, with typed ambiguity
handling and replay-safe parity instead of heuristic guess-and-undo behavior.

### Sequencing Notes

This belongs after merge because collaborative user and agent workflows must
expose real branch and intent semantics rather than paper over them.

## Milestone 20: Worth Certification And Aircraft-Grade Auditability

### Goal

Prove that Worth is fit for the full product thesis: a spec-native, AI-native,
traceable geometry system whose authoritative truth, derived interpretation,
naming continuity, geometric decisions, interaction intent, and failure
localization remain auditable, replay-safe, and robust enough for
manufacturing-grade and ultimately aircraft-grade trust programs.

### Adversarial Constraint

Worth must emit the same trustworthy model conclusions under hostile replay,
branch, history, approximation, corruption-localization, interaction, and
MetaBoss-tier final-boss scenarios that it does in ordinary interactive use.

### Admitted Surface

- full-roadmap certification over all admitted Worth workflow classes
- machine-checkable artifact bundles across truth, derivation, replay,
  continuity, and interaction surfaces

### Excluded Surface

- any workflow class still explicitly marked `Debt` or unsupported in earlier
  milestones

### Acceptance Evidence

- persistent naming continuity and rebinding parity
- hostile non-manifold topology legality and localization parity
- hostile planar proof parity
- hostile curved proof parity
- hostile freeform proof parity where admitted
- hostile fillet proof parity
- branch-local topology, naming, and intent isolation parity
- topology-to-geometry anchoring and approximation diagnostics parity
- feature-regeneration replay parity
- interaction-intent preservation parity
- merge-bearing Worth history parity
- MetaBoss and clean-fail diagnostic sufficiency with machine-checkable
  artifacts

Each certification run must emit machine-checkable artifact bundles.

### Workflow Surface

Milestone 20 is not done because a few certification demos or benchmark parts
look convincing.

It is only done when certification closes over the full admitted Worth
workflow surface assembled by the earlier milestones, including:

- topology and naming histories
- geometry binding and approximation histories
- feature and regeneration histories
- specialized-feature and blend histories
- branch, merge, and interaction histories
- MetaBoss and final-boss compound workloads within the admitted class

### Operator Closure

- certification-bundle generation operators
- replay and audit extraction operators
- cross-surface final-boss verification operators

### Validator Closure

- every validator family declared earlier in the roadmap must be closed at its
  admitted boundary
- aircraft-grade audit sufficiency must be explicit for admitted artifact
  surfaces

### Replay Closure

- all admitted certification workloads must replay identically at the artifact
  level or fail with identical structured failure artifacts

### Diagnostics Closure

- every admitted certification workload must emit machine-checkable artifact
  bundles sufficient to explain success or exact structured failure

### Determinism Closure

- certification outcome class and artifact digests must be deterministic for
  the same admitted workload and truth basis

### Complexity / Proof Closure

- every named hot-path milestone contract must either be verified or still
  explicitly marked `Debt`
- certification must include proof that no hidden broad-scan or replay-breadth
  regressions invalidate earlier milestone claims

### Allowed Debt

- only debt already named explicitly in earlier milestones may remain
- hidden uncertified workflow classes may not

### Milestone Done When

Milestone 20 is done only when the entire admitted Worth workflow surface is
certified as exact, explicit, or clean-failing with machine-checkable evidence,
and any remaining incompleteness is explicit named debt rather than hidden
uncertainty.

## Completion Standard

Worth is roadmap-complete only when:

- spec, topology, naming, geometry binding, and feature intent are all
  expressed as authoritative truth
- derived topology and geometry work remain rebuildable and bridge-causal
- persistent naming remains first-class through edit, replay, branch, merge,
  and interaction flows
- topology legality and corruption localization remain explicit at the right
  validation boundaries
- exact planar, curved, and admitted freeform geometry all remain honest and
  auditable
- fillets and blend cascades either succeed correctly or fail with exact,
  structured causes
- UI and DSL layers preserve explicit user or agent intent rather than hiding
  ambiguity in heuristics
- all required named suites in
  [test-requirements.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/worth/test-requirements.md)
  pass with machine-checkable evidence
- Worth proves the vision without inventing a second runtime or a second
  authority model

## Companion Documents

- [VISION.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/worth/VISION.md)
- [worth_bootstrap_plan.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/worth/worth_bootstrap_plan.md)
- [milestone-1.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/worth/milestone-1.md)
- [worth-topo-domain-structure-gate.md](/Users/Esther/Documents/Programming/forge_workspace/worktree_2/_docs/worth/worth-topo-domain-structure-gate.md)
- [test-requirements.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/worth/test-requirements.md)
- [test-requirements_pt2.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/worth/test-requirements_pt2.md)
- [metaboss_tier4.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/worth/metaboss_tier4.md)
- [forge_relational_roadmap.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-relational/forge_relational_roadmap.md)
- [forge_runtime_bridge_roadmap.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-runtime-bridge/forge_runtime_bridge_roadmap.md)
- [forge_store_roadmap.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-store/forge_store_roadmap.md)
