# Milestone 3 Engineering Spec: Topology Editing Core

> **Status:** Active
>
> **Roadmap parent:** [worth_roadmap.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/worth/worth_roadmap.md)
>
> **Predecessors:**
> - [milestone-1.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/worth/milestone-1.md)
> - [milestone-2.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/worth/milestone-2.md)
>
> **Predecessor closeouts:**
> - [milestone-1-closeout.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/worth/milestone-1-closeout.md)
> - [milestone-2-closeout.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/worth/milestone-2-closeout.md)
>
> **Vision parent:** [VISION.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/worth/VISION.md)
>
> **Test requirements:**
> - [test-requirements.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/worth/test-requirements.md)
> - [topo-test-requirements.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/worth/topo-test-requirements.md)
>
> **Primary architectural driver:** freeze a topology-only edit substrate that
> survives hostile topology certification without leaking geometry semantics into
> `worth-topo` or forcing later milestones to refound the topology model

## Goal

Establish the first honest topology-editing substrate for Worth:

- topology-only
- naming-aware
- replay-safe
- branch-safe
- hostile-test-oriented from the start

## Why This Milestone Exists

Milestone 3 is not "add a few Euler operators."

It is the milestone that decides whether Worth topology editing becomes:

- a typed, authoritative, hostility-ready mutation layer over the Milestone 1
  and Milestone 2 substrate, or
- a loose pile of local rewires that happen to work on friendly fixtures and
  later collapse under primitive construction, branch pressure, or bowtie-class
  stress

Milestone 3 must stay brutally scoped:

- it is `worth-topo` work
- it must remain geometry-free
- it must not absorb primitive construction policy from `worth-kernel`
- it must not absorb binding or classification semantics from `worth-spatial`

The job here is to make topology edits so structurally honest that the hostile
topology certification bar becomes easier to satisfy because the runtime has
fewer ways to cheat.

## Governing Summaries

- `MENTALITY.md`
  The most important thing it protects is adversarial-design-first thinking.
  Milestone 3 must therefore be designed around the topology edit failure modes
  that would break a naive edit substrate at production scale, not around the
  first operator demo we can get passing.
- `arch_laws.md`
  The most important thing it protects here is proof-bearing phase boundaries.
  Edits must move through explicit, typed stages and must preserve authority /
  derivation separation rather than letting read helpers or diagnostics become
  shadow mutation logic.
- `forge-signal`
  The most important thing it contributes here is selective derived fallout.
  Milestone 3 should use aspect-aware and, where localizable, region-aware
  recompute so local topology edits do not force whole-topology derived work by
  default.
- `perf_laws.md`
  The most important thing it protects is semantic-delta-bounded work. Milestone
  3 must keep edit-local breadth, validator breadth, naming-resolution breadth,
  and replay breadth explicit and testable instead of burying broad scans inside
  innocent-looking edit APIs.
- `domain_laws.md`
  The most important thing it protects is responsibility-shaped structure.
  Milestone 3 must keep topology editing in `worth-topo`, leave geometry-facing
  semantics to `worth-spatial`, and leave primitive construction programs to
  `worth-kernel`.
- `VISION.md`
  The most important thing it protects is that the spec graph and authoritative
  truth stay central. Milestone 3 must mutate authoritative topology truth
  honestly rather than inventing a convenience edit arena.
- `worth_roadmap.md`
  The most important thing it protects is dependency order. Milestone 3 belongs
  before primitive construction, spatial binding, planar contracts, and
  booleans because all of those depend on an honest topology-only edit layer.
- `worth/test-requirements.md`
  The most important thing it protects is workflow-class closure. Milestone 3
  must close over generic edit workflows on admitted topology families, not a
  few friendly local rewires.
- `worth/topo-test-requirements.md`
  The most important thing it protects is the topology brutality bar.
  Milestone 3 must directly inherit the relevant topology torture categories:
  mutation pipeline integrity, operator brutality, query/traversal brutality,
  non-manifold hostility, determinism assault, and corruption-localization.
- `milestone-1-closeout.md`
  The most important thing it protects is the frozen authority and naming
  boundary. Milestone 3 must not reopen topology truth, naming truth, or
  same-commit mutation semantics.
- `milestone-2-closeout.md`
  The most important thing it protects is the frozen derived read pipeline.
  Milestone 3 must consume materialized/interpreted/validated read products and
  their diagnostics honestly rather than smuggling edit meaning into caches or
  helper layers.

## Adversarial Constraint

Milestone 3 must survive this hostile condition:

> Arbitrary admitted topology edit histories, including high-cardinality shell
> edits, non-manifold radial rewires, bowtie-adjacent local changes,
> cancellation chains, branch-local divergence pressure, and ambiguous
> name-preservation outcomes, must either commit to the same deterministic
> authoritative topology truth with the same replay-safe certification artifacts
> or fail with exact, localized, structured diagnostics, all while `worth-topo`
> remains geometry-free.

Concretely, the design fails if Milestone 3:

- lets an edit outcome depend on a derived read helper instead of authoritative
  truth and explicit edit contracts
- allows geometry semantics to leak into topology editing
- allows admitted edits to replay differently from the same authoritative input
  history
- preserves counts while silently changing structural meaning
- handles bowtie, branch, or radial-boundary pressure with undefined behavior,
  silent corruption, or hidden widening of admitted semantics
- claims closure because a few Euler operators work on a cube-like or
  tetrahedron-like body while generic edit workflows remain unproven

The hostile question for this milestone is:

`if we attack every admitted topology edit workflow with the topology brutality bar, do the edit contracts make the wrong thing impossible, or only unlikely?`

## Product Decision Lock

- Milestone 3 is strictly `worth-topo` work
- Milestone 3 must remain geometry-free
- topology editing semantics belong in `worth-topo`
- topology/geometry interaction semantics do not belong in `worth-topo`
- primitive construction programs do not belong in `worth-topo`
- edits must mutate authoritative topology truth directly
- edits must be typed, staged, and proof-bearing
- `forge-relational` owns authoritative edit truth
- `forge-runtime-bridge` owns edit-causality lowering from touched Worth
  aspects and changed topology scope into derived invalidation
- `forge-signal` owns derived edit fallout:
  - selective recompute
  - reuse legality
  - diagnostics and history
  - parity harnesses over derived edit artifacts
- `forge-signal` may not define edit meaning or authoritative legality
- hostile topology tests are a design target, not a cleanup phase

## Admitted Surface

Milestone 3 admits topology-only edit workflows over the admitted topology
families inherited from Milestone 1 and Milestone 2, but only for the edit
classes explicitly named here.

The admitted surface includes:

- primitive lifecycle edits over arbitrary admitted entity counts
- loop and boundary rewiring over arbitrary admitted loop cardinalities
- shell and wire membership edits
- admitted radial splice or reseating workflows within the milestone's admitted
  NMT class
- branch-local and replayed histories over the admitted edit surface
- explicit naming-preservation or naming-ambiguity conclusions for the admitted
  edit surface

The canonical primitive families that must remain visible in Milestone 3 proof
are:

- `WireOpen(n)`
- `WireClosed(n)`
- `SheetDisk(n)`
- `SheetPatch(f)`
- admitted `SolidShell(f)`

These are workflow families, not showcase examples.

## Excluded Surface

Milestone 3 explicitly excludes:

- primitive and body construction programs
- geometry binding
- spatial classification
- planar exactness
- boolean operators
- curve, surface, or trim semantics
- edit classes that require `worth-spatial` or `worth-kernel` meaning to even
  define the operation honestly
- silent "best effort" repair of out-of-class or ambiguous edit requests

Excluded workflows must clean-fail with explicit rejection classes rather than
being smuggled in as convenience helpers.

## Workflow Surface

Milestone 3 is not done because:

- one edge split works
- one loop rewire works
- one shell membership mutation works
- one cube or tetrahedron survives a hand-authored edit

It is only done when the admitted edit workflows operate generically over:

- arbitrary admitted primitive lifecycle counts
- arbitrary admitted loop cardinalities
- arbitrary admitted shell and wire membership scopes
- arbitrary admitted radial valence within the milestone's NMT class
- arbitrary admitted branch-local history length within the replay contract

This milestone must close workflow classes, not operator demos.

## Initial Admitted Operator Set

Milestone 3 should not leave the first admitted operator surface implicit.

The initial admitted operator families are:

- `CreateTopologyEntity`
  - create one admitted topology entity within the current admitted family set
- `RetireTopologyEntity`
  - retire one admitted topology entity when doing so does not violate the
    remaining topology truth contracts
- `AttachBoundaryMembership`
  - attach admitted boundary ownership or membership relations
- `DetachBoundaryMembership`
  - remove admitted boundary ownership or membership relations
- `RewireLoopSuccessor`
  - change admitted local loop-successor structure
- `RewireLoopEndpoint`
  - change admitted local loop endpoint structure while preserving admitted
    topology semantics
- `AttachShellOrWireMembership`
  - add admitted shell or wire membership
- `DetachShellOrWireMembership`
  - remove admitted shell or wire membership
- `SpliceRadialAdjacency`
  - add or alter admitted radial adjacency inside the admitted NMT class
- `DetachRadialAdjacency`
  - remove admitted radial adjacency inside the admitted NMT class

These names are contract-family names, not necessarily final Rust type names.

Milestone 3 does not admit one giant untyped "topology edit" surface.
Every implemented edit must declare which admitted operator family it belongs
to or else clean-fail as unsupported.

## Operator Closure

The admitted operator families for Milestone 3 are:

- primitive entity lifecycle operators
- loop and boundary rewiring operators
- shell and wire membership operators
- admitted radial splice or reseating operators

For every admitted operator family, Milestone 3 must certify:

- legal admitted cases
- hostile admitted cases
- explicit out-of-class exits
- replay parity
- exact rejection localization when blocked

## Validator Closure

Milestone 3 must preserve and explicitly exercise:

- Milestone 1 truth and naming validator closure
- Milestone 2 derived validation closure as inspection support, not mutation
  authority
- edit-local continuity validation
- exact invariant-boundary or continuity-boundary localization for rejected
  edits

No admitted edit may bypass the validator ladder simply because the local
mutation "looks obviously fine."

## Naming Continuity Decision Rule

Milestone 3 naming outcomes must not remain implicit.

Every admitted edit with naming consequences must classify continuity as one of:

- `Preserved`
  - one canonical predecessor-to-successor continuity mapping exists
  - the mapping stays inside naming truth rules
- `Ambiguous`
  - more than one legally plausible continuity mapping survives
  - the system cannot honestly claim one without extra intent
- `Rejected`
  - the requested edit exits the admitted naming or topology class
  - or the continuity request violates naming truth constraints

Implementation rule:

- the topology edit contract may propose expected continuity scope
- the continuity classifier decides `Preserved`, `Ambiguous`, or `Rejected`
- no helper may silently upgrade `Ambiguous` to `Preserved`
- no helper may silently discard naming consequences as "best effort"

## Replay Closure

Milestone 3 must prove:

- admitted accepted edit histories replay identically
- admitted rejected edit histories replay as the same typed rejection
- branch-local parity over the admitted edit surface
- stable edit and diagnostic digests for the same admitted topology history

## Diagnostics Closure

Milestone 3 must emit diagnostics that identify:

- exact blocking boundary
- exact rejection class
- exact affected topology scope
- exact continuity consequence where naming is in scope

If an edit fails and the system cannot say exactly why and where, the edit is
not certified.

## Changed Scope Vocabulary

Milestone 3 must normalize changed scope instead of letting each edit contract
invent its own local language.

At minimum, changed scope should be expressible in terms of:

- entity scope
- relation scope
- local neighborhood scope
- loop scope
- wire scope
- shell scope
- radial neighborhood scope
- naming scope

Implementation rule:

- edit contracts declare expected changed scope using this shared vocabulary
- diagnostics and certification surfaces reuse the same vocabulary
- bridge lowering and signal invalidation may refine from this vocabulary, but
  may not replace it with unrelated ad hoc labels

## Derived Region Vocabulary

Milestone 3 must also normalize the first derived invalidation partition
language.

Where locality is honestly knowable, derived fallout should be scoped in terms
of one or more of:

- loop region
- wire region
- shell region
- radial neighborhood region
- edit-local neighborhood region
- naming continuity region

This vocabulary exists so that:

- bridge lowering has one canonical region language
- signal invalidation can be aspect-aware and region-aware without each edit
  inventing custom partitions
- parity and fallback reporting can compare like with like

## Determinism Closure

Milestone 3 must make explicit:

- canonical edit ordering rules where ordering should not change meaning
- canonical tie-break rules where multiple legal local rewires exist
- stable rejection classification for the same illegal or ambiguous request
- stable changed-scope and continuity artifacts for the same admitted edit
  history

## Complexity / Proof Closure

Milestone 3 must name and prove:

- edit-local breadth contracts
- validation breadth contracts
- naming-resolution breadth contracts
- replay and branch-local parity breadth contracts
- aspect-aware invalidation breadth contracts
- region-aware invalidation breadth contracts where topology neighborhoods can
  be localized honestly
- explicit fallback policy when locality is not yet available or not yet
  provable

Whole-view or whole-history fallback, when unavoidable in the first ship, must
be explicit in the proof surfaces rather than hidden behind innocent helper
APIs.

## Rejection Taxonomy

Milestone 3 should freeze the first rejection classes instead of treating
rejections as free-form strings.

At minimum, the implementation should classify rejected edits as:

- `OutOfClassEdit`
- `InvariantBlocked`
- `NamingContinuityAmbiguous`
- `NamingContinuityRejected`
- `ScopeLocalizationUnavailable`
- `DerivedFallbackExceeded`

The exact enum names may vary, but these semantic categories must exist as
direct machine-checkable outputs.

## Allowed Debt

- broader primitive-construction workflows may remain deferred to Milestone 4
- geometry-bound edits may remain deferred to `worth-spatial`
- broader boolean, curved, and kernel-driven edit semantics may remain deferred

What may not remain implicit debt:

- topology edit authority shape
- naming continuity outcomes
- replay safety
- branch safety
- hostile certification intent for the admitted edit class

## Phases

### Phase 1: Freeze The Edit Boundary

Define one honest topology-only edit entry surface for `worth-topo`.

This phase exists to stop topology editing from becoming "anything that can
mutate topology truth."

It must establish:

- one canonical edit boundary that consumes authoritative topology truth and
  produces authoritative topology truth
- explicit edit intent types for the first admitted topology edit families
- explicit exclusion of geometry-bearing, spatial, or primitive-construction
  semantics from the edit boundary
- explicit mutation ownership over:
  - primitive lifecycle edits
  - loop and boundary rewiring edits
  - shell and wire membership edits
  - admitted radial splice or reseating edits

This phase must make the following things impossible:

- geometry-aware operator inputs in `worth-topo`
- ad hoc local rewires outside the canonical edit boundary
- mutation helpers that bypass the same authoritative mutation and invariant
  surfaces frozen in Milestone 1
- edit semantics that depend on derived topology interpretation or diagnostics
  to decide what the authoritative mutation means

Expected implementation shape:

- `worth-topo` owns the edit contracts and edit runner
- authoritative mutation still goes through the same runtime truth boundary
- edit declarations are topology-shaped, not shape-program-shaped
- explicit out-of-class edit exits exist from the start

This phase is done when topology editing has one typed boundary and no second,
looser edit path remains.

### Phase 2: Freeze Edit Contracts And Proof-Carrying Mutation Types

Make admitted topology edits typed, staged, and proof-bearing.

This phase must introduce explicit edit contract families for the admitted
Milestone 3 workflow surface.

At minimum, the admitted edit surface must cover:

- primitive lifecycle edits over arbitrary admitted entity counts
- loop and boundary rewiring over arbitrary admitted loop cardinalities
- shell and wire membership edits
- admitted radial splice or reseating workflows within the admitted NMT class

These contracts must make explicit:

- authoritative preconditions
- edit intent
- touched Worth aspects
- expected changed topology scope
- expected changed naming scope
- expected invalidation scope for derived fallout when that scope is
  knowable honestly
- exact clean-fail exits for out-of-class or illegal edits

Edits must move through explicit stages such as:

- requested edit intent
- validated edit contract
- authoritative mutation batch
- commit outcome
- post-commit proof artifacts

The exact type names may evolve, but the phase structure may not collapse back
into one "apply edit" helper.

This phase must also define the first direct edit proof surfaces:

- `topology_edit_digest`
- changed-scope or changed-breadth evidence
- typed accepted versus typed rejected edit outcomes

The intended runtime shape is:

- relational truth decides what changed
- bridge lowers touched aspects and changed scope
- signal invalidates only the affected derived neighborhoods where possible
- diagnostics and history explain why those nodes reran

This phase is done when admitted edit semantics are explicit enough that later
certification is proving contracts, not reverse-engineering helper behavior.

### Phase 3: Freeze Edit-Local Validation And Naming Outcomes

Make edit legality and naming continuity explicit at the edit boundary.

This phase exists because topology editing is where naive systems silently lose
trust:

- a topology edit "works" but violates an invariant later
- a name appears to survive but continuity was ambiguous
- a rejection happens, but nobody can tell which boundary blocked it

Milestone 3 must not allow that.

This phase must prove:

- edit application preserves Milestone 1 and Milestone 2 validator closure
- naming preservation is explicit when continuity is unambiguous
- typed naming ambiguity is explicit when continuity cannot be claimed
- rejected edits localize to the exact invariant, continuity, or admitted-class
  boundary

Truth-side and derived-side ownership must stay explicit:

- topology truth legality stays authoritative
- naming truth and continuity conclusions stay explicit and separate
- derived read products may assist inspection, but they may not silently decide
  edit legality

Required direct outputs for this phase include:

- `naming_edit_continuity_matrix`
- `rejected_edit_scope_report`
- validator-family localization sufficient to identify the blocking boundary

This phase is done when edit acceptance and edit rejection are both
machine-explainable at the same precision.

### Phase 4: Freeze Replay, Branch, And Diagnostics Semantics

Make topology editing replay-safe, branch-safe, and diagnostics-complete.

This phase exists because topology operators that "work once" but replay
differently are not real operators.

Milestone 3 must prove:

- admitted edit histories replay identically
- rejected admitted edit histories replay as the same typed rejection
- branch-local edit application preserves the same semantics for the same local
  truth basis
- diagnostics survive replay and branch-local execution with stable meaning
- derived edit fallout remains parity-safe under the same touched-aspect and
  changed-scope basis

The direct proof outputs must include at minimum:

- `edit_replay_parity_report`
- stable edit digests
- exact accepted versus rejected outcome class
- exact changed-scope and rejection-locality evidence

And should expose direct supporting evidence for:

- derived invalidation breadth
- derived rebuild breadth
- explicit fallback when whole-view recompute still happens
- equivalence or reuse legality for any claimed recompute suppression

This phase must also state explicit determinism rules for:

- edit ordering where the contract says ordering is semantically irrelevant
- tie-breaking where the contract requires one canonical outcome
- diagnostic classification and rejection codes

This phase is done when admitted edit workflows can no longer drift quietly
under replay or branch-local execution.

Branch and replay proof for Milestone 3 must include at minimum:

- same-basis replay of accepted edit histories
- same-basis replay of rejected edit histories
- branch-local execution of admitted accepted edits
- branch-local execution of admitted rejected edits
- divergence from a shared base followed by parity checks on resulting
  diagnostics and outcome class

### Phase 5: Freeze Hostile Edit Certification

Turn topology editing into a hostile-certification target, not a demo feature.

Milestone 3 must directly inherit the relevant parts of
[topo-test-requirements.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/worth/topo-test-requirements.md).

At minimum, hostile Milestone 3 certification must exercise:

- mutation pipeline integrity
- primitive topology family closure for the Milestone 3 admitted families
- operator brutality for the admitted edit surface
- query and traversal brutality over edited topology states
- non-manifold and radial brutality for the admitted edit class
- degeneracy and corruption-localization
- determinism and order assault
- diagnostics and failure taxonomy
- some scale, depth, and sustained edit pressure

Named hostile scenario families for Milestone 3 should include, at minimum:

- bowtie or bowtie-adjacent survivability where the milestone admits those
  neighborhoods or must explicitly reject them
- commutative edit-order fuzzers where edit contracts claim order invariance
- ambiguous local rewire selection
- cancellation chains
- repeated split or collapse churn within the admitted edit class
- broken-loop and broken-radial corruption cases

The design intent is simple:

`the edit substrate should be shaped so the hostile tests look inevitable, not surprising`

This phase is done when the admitted Milestone 3 edit workflows are certified
as workflow classes instead of hand-built examples.

The minimum required hostile suites for Milestone 3 are:

- `BowtieAdjacentRewire`
  - admitted survival or explicit typed rejection
- `CancellationChainParity`
  - repeated local edits that preserve counts but threaten structural meaning
- `SplitCollapseChurn`
  - repeated local topology churn within the admitted edit class
- `AmbiguousLocalRewireContinuity`
  - topology succeeds but naming continuity must classify ambiguity honestly
- `BrokenRadialLocalization`
  - corrupted or illegal radial local states must localize exactly

These suite names are requirement names, not necessarily file names.

## Must Ship

- one topology-only edit boundary in `worth-topo`
- typed edit intent and validated edit-contract surfaces for the admitted
  Milestone 3 edit class
- authoritative mutation application over the existing relational authority
  runtime
- explicit naming continuity outcomes:
  - preserved
  - typed ambiguity
  - typed rejection
- explicit edit-local rejection and localization artifacts
- replay-safe and branch-safe edit proof surfaces
- a hostile Milestone 3 certification surface that references the relevant
  topology brutality categories directly
- direct closeout artifacts for Milestone 3 rather than nested helper-only
  reports

## Must Preserve

- Milestone 1 authority semantics
- same-commit graph mutation integrity
- Milestone 1 topology and naming validator closure
- Milestone 2 derived read pipeline boundaries
- `worth-topo` geometry purity
- explicit separation between:
  - authoritative truth
  - naming truth
  - derived interpretation
  - diagnostics
  - certification

Milestone 3 may add topology editing, but it may not reopen or blur these
boundaries.

## Acceptance Evidence

Milestone 3 is not done because a few Euler-style operators work on a cube,
tetrahedron, or other showcase body.

Milestone 3 is done only when it emits direct machine-checkable proof surfaces
over the admitted workflow class.

At minimum, Milestone 3 closeout must include:

- `topology_edit_digest`
- `naming_edit_continuity_matrix`
- `rejected_edit_scope_report`
- `edit_replay_parity_report`

Milestone 3 closeout should also expose direct aggregate surfaces for:

- validator-family coverage over admitted edit families
- edit-family coverage over the canonical admitted primitive families
- rejection-class distribution for edit failures
- failure locality by edit family and role
- branch-local parity over admitted edit histories
- derived invalidation and rebuild breadth over admitted edit histories
- explicit fallback reporting for edit-driven derived recompute
- equivalence or reuse-legality reporting where recompute suppression is
  claimed
- counter and breadth reports for edit validation, continuity, and replay work

And should keep direct machine-checkable rows for:

- admitted operator family coverage
- rejection taxonomy distribution
- naming continuity outcome distribution
- changed-scope vocabulary coverage
- derived region vocabulary coverage where locality is claimed

Required workload surface:

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

Must verify:

- edit application preserves Milestone 1 and Milestone 2 validator closure
- naming preservation or typed ambiguity is explicit for admitted edit classes
- rejected edits localize to the exact invariant or continuity boundary
- admitted edit histories replay identically
- edit-driven derived fallout respects touched aspects and changed scope rather
  than widening silently to whole-topology recompute without explicit fallback
- hostile admitted edit workloads either succeed exactly or fail with exact
  structured localization

Milestone 3 closes only when admitted topology edits operate generically across
the admitted workflow surface, replay and branch pressure do not change their
meaning, and the hostile topology certification subset for editing is real
rather than implied.

## Architectural Notes

- `worth-topo` owns topology edit semantics and topology edit certification
- `worth-spatial` does not participate in Milestone 3 implementation
- `worth-kernel` does not define Milestone 3 edit semantics
- primitive construction programs are not part of Milestone 3 and should not
  leak into the edit API
- geometry-bound or spatially classified edits are explicitly deferred
- `forge-runtime-bridge` should be used to lower touched aspects and changed
  topology scope into canonical derived invalidation inputs
- `forge-signal` should be used aggressively for:
  - aspect-aware derived recompute
  - region-aware derived recompute where the changed topology neighborhood is
    explicit
  - diagnostics and history for edit fallout
  - parity harnesses over edit-driven derived artifacts
- `forge-signal` should not be used as a second mutation authority or as the
  place where edit legality is first decided
- topology editing should prefer explicit contract families over one giant
  operator enum if that keeps subdomain ownership cleaner
- over-300-line files should be reviewed aggressively during implementation,
  especially in edit runner, contract, and certification code

Recommended subdomain split for implementation:

- `edit_intents/`
- `edit_contracts/`
- `edit_application/`
- `edit_naming/`
- `edit_diagnostics/`
- `edit_certification/`

The exact folder names may vary, but the responsibility split should remain
visible and enforceable.

## Sequencing Notes

- Milestone 3 is the last strictly topology-only milestone before primitive
  construction becomes first-class
- Milestone 4 consumes Milestone 3 but does not replace it
- Milestone 3 should therefore close generic topology edit workflows, not
  body-construction workflows
- passing Milestone 3 does not mean:
  - primitive construction is closed
  - geometry binding is ready
  - booleans are ready
- passing Milestone 3 does mean:
  - topology editing can be trusted as a substrate for primitive construction
    and later boolean-facing work
  - the admitted topology edit class is strong enough to survive serious
    hostile certification rather than only hand-built examples
