# Milestone 1 Engineering Spec: NMT Topology Truth, Persistent Naming, And Validation Authority

> **Status:** Planned
>
> **Roadmap parent:** [worth_roadmap.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/worth/worth_roadmap.md)
>
> **Vision parent:** [VISION.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/worth/VISION.md)
>
> **Test requirements:** [test-requirements.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/worth/test-requirements.md)
>
> **Primary architectural driver:** lock one authoritative topology and naming truth model before topology editing, geometry binding, regeneration, or specialized features are allowed to widen the system
>
> **Companion docs:**
> - [MENTALITY.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/coding_guidelines/MENTALITY.md)
> - [arch_laws.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/coding_guidelines/arch_laws.md)
> - [perf_laws.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/coding_guidelines/perf_laws.md)
> - [domain_laws.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/coding_guidelines/domain_laws.md)
> - [worth_roadmap.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/worth/worth_roadmap.md)
> - [test-requirements.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/worth/test-requirements.md)
> - [forge_relational_roadmap.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-relational/forge_relational_roadmap.md)
> - [forge_runtime_bridge_roadmap.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-runtime-bridge/forge_runtime_bridge_roadmap.md)

## Goal

Define one authoritative Worth topology and persistent-naming truth model and
make it the only semantically authoritative substrate for shells, wires, open
boundaries, admitted non-manifold radial structure, and branch-local topology
history.

## Why This Milestone Exists

Milestone 1 is not "seed a few topological records."

It is the milestone that decides whether Worth becomes:

- a real topology and naming authority layer built honestly on the Forge
  runtimes, or
- another transitional kernel stack where topology, validation, naming, and
  diagnostics quietly spread across overlapping pseudo-authority layers

Everything later depends on this boundary being honest:

- topology edits must mutate this truth rather than some topo-owned arena of
  convenience
- geometry bindings must attach to this topology authority rather than to
  derived topology views
- regeneration must consume this truth rather than inventing a second topology
  model
- persistent naming continuity must be judged from this truth rather than from
  UI labels, host caches, or post-hoc heuristics

If Milestone 1 is vague, every later milestone will renegotiate what "the real
Worth topology" actually is. This spec exists to stop that failure before it
starts.

## Governing Summaries

- `MENTALITY.md`
  The most important thing it protects is structural honesty under adversarial
  pressure. This milestone therefore starts from the hostile topology failure
  mode and treats replay, diagnostics, and exact boundary ownership as product
  requirements rather than cleanup work.
- `arch_laws.md`
  The most important thing it protects here is authority-versus-derivation
  separation. Worth topology truth, derived topology interpretation, and
  certification audits must be different objects with different lifecycle and
  proof boundaries.
- `perf_laws.md`
  The most important thing it protects is bounded breadth and visible cost.
  Milestone 1 must keep commit-boundary checks local, derived interpretation
  incremental, and every fallback breadth explicit in counters.
- `domain_laws.md`
  The most important thing it protects is responsibility-shaped structure.
  Topology truth, naming truth, validation families, and diagnostics must be
  split by real semantic reasons, not collapsed into giant `topology` or
  `validation` buckets.
- `VISION.md`
  The most important thing it protects is that the spec graph is the product
  and that topology, naming, and traced reasoning are first-class product
  surfaces. Milestone 1 must therefore make naming and topology truth
  authoritative from day one.
- `worth_roadmap.md`
  The most important thing it protects is sequencing. Milestone 1 belongs first
  because every later geometry, feature, branch, merge, and interaction claim
  depends on a frozen topology and naming authority model.
- `worth/test-requirements.md`
  The most important thing it protects is milestone-specific proof instead of
  toy examples. Milestone 1 is not closed until the canonical primitive corpus
  families for wires, sheets, shell patches, and admitted NMT fans pass with
  machine-checkable outputs.
- `forge_relational_roadmap.md`
  The most important thing it protects is that authoritative truth lives in the
  relational runtime. Milestone 1 must consume that truth substrate rather than
  rebuilding a custom topology store.
- `forge_runtime_bridge_roadmap.md`
  The most important thing it protects is causal separation between truth and
  derived work. Milestone 1 must reserve bridge-facing aspect vocabulary and
  refuse to hide invalidation semantics inside topo-owned caches.

## Adversarial Constraint

Milestone 1 must survive this hostile condition:

> Worth must be able to author, validate, replay, and inspect shell, wire,
> loop, halfedge, radial, and persistent-name truth across arbitrary admitted
> topology cardinalities and admitted NMT valence, while branch-local histories
> and derived topology reads continue to converge on the same meaning without
> any topo-owned cache, host object graph, or post-hoc validator redefining
> authority.

Concretely, the design fails if any admitted path:

- stores topology legality only in a derived validator instead of blocking
  impossible truth at the commit boundary
- stores persistent names as advisory metadata or display labels instead of
  authoritative truth
- allows derived topology views to become the only place where shell, wire, or
  radial meaning exists
- allows naming continuity conclusions to depend on iteration order, read path,
  or branch context for the same authoritative history
- proves topology claims with a tetrahedron, cube, or a few hand-authored
  examples while the admitted primitive family remains incomplete

The hostile question for this milestone is:

`if every derived topology view disappeared tomorrow, what exact authoritative records would still let us prove shell, wire, boundary, radial, and persistent-name truth?`

## Product Decision Lock

The following decisions are locked in this milestone:

- authoritative Worth topology truth lives in `forge-relational`
- persistent naming is authoritative truth, not host-side metadata
- shells, wires, open boundaries, and admitted non-manifold radial states are
  part of the first topology truth model, not later extensions
- commit-boundary structural legality and derived topology interpretation are
  separate systems with separate responsibilities
- topology truth and persistent naming truth must admit parameterized primitive
  families, not only showcase parts
- the first bridge-facing topology and naming aspect vocabulary is frozen here
- authoritative topology creation must publish one coherent same-commit graph
  mutation, including relations that target entities created inside that same
  commit, rather than forcing an orphan-entity intermediate publish

Normative consequence:

- any implementation that keeps the "real" topology in a topo-owned mutable
  store and mirrors it into relational records is out of spec
- any implementation that treats persistent names as generated labels, debug
  tags, or UI handles is out of spec
- any implementation that proves shells, solids, or NMT only by fixed toy
  shapes is out of spec

## Scope

### In Scope

- authoritative truth for:
  - body
  - lump
  - region
  - shell
  - face
  - loop
  - halfedge
  - edge
  - vertex
  - wire
  - persistent name
- topology relations for:
  - ownership
  - loop entry
  - next / prev or equivalent local ring coherence
  - radial adjacency
  - shell and wire membership
  - persistent-name targeting
- commit-boundary invariant groups for local structural legality
- derived topology interpretation boundaries for shell, wire, boundary, and
  admitted non-manifold structure
- full milestone-1 admission of:
  - `WireBranch(k)` within the declared branch-valence class
  - `SolidShell(f)` within the declared closed-shell face-count class
- seed and local reseating workflows over the admitted topology truth surface
- branch-local history over the admitted truth surface
- bridge-facing aspect vocabulary for topology and naming changes

### Admitted Class Lock

Milestone 1 is a full-robustness milestone for its admitted class.

That means:

- every workflow inside the admitted class must be implemented honestly,
  validated honestly, replayed honestly, and diagnosed honestly
- no admitted family may ship as "foundation only," "first cases," "partial,"
  "mostly works," or "good enough for the demo"
- anything not fully supported must move to the excluded surface and fail
  closed

Milestone-1 admitted class boundaries are:

- `WireOpen(n)` for arbitrary `n >= 1`
- `WireClosed(n)` for arbitrary `n >= 3`
- `WireBranch(k)` for arbitrary admitted branch valence `k >= 3` at one or
  more branch vertices, with deterministic branch partitioning and no silent
  fallback to generic NMT ambiguity
- `SheetDisk(n)` for arbitrary `n >= 3`
- `SheetPatch(f)` for arbitrary admitted face count `f >= 2`
- `SolidShell(f)` for arbitrary admitted face count `f >= 4` within the
  milestone's closed, orientable, genus-0 shell class
- `NmtEdgeFan(k)` for arbitrary admitted radial valence `k >= 3` within the
  milestone's edge-fan class

Milestone-1 explicitly excludes:

- `SolidWithVoid(...)`
- `MultiLumpBody(...)`
- `NmtVertexPinch(d)`
- non-orientable shells
- higher-genus closed shells
- shell self-intersection semantics
- unsupported mixed wire / shell / pinch topologies outside the admitted class

### Explicitly Out Of Scope

- broad topology edit-operator catalog
- geometry binding or topology-to-geometry continuity
- boolean execution
- blend and feature execution
- broad healing and import-recovery workflows
- unsupported NMT classes beyond the declared admitted radial and wire-branch
  surface
- vertex-pinch and multi-disk NMT semantics
- non-orientable or higher-genus solid-shell semantics
- void-shell and multi-lump solid semantics

Milestone 1 may reserve semantic surface for later milestones. It may not
pretend those later capabilities are already shipped.

## Authoritative Truth Model

Milestone 1 must define one explicit Worth topology truth vocabulary.
This vocabulary is not a placeholder enum list. It is the first real semantic
surface the rest of Worth must build on.

### Required truth families

Milestone 1 authorizes exactly these first-class truth families:

- topology entity kinds
- topology relation kinds
- persistent-name entity and relation kinds
- topology and naming aspect kinds
- topology invariant groups
- topology mutation records
- topology diagnostic records

All six families must have explicit runtime-facing identifiers and stable names.

Milestone 1 must also classify each admitted primitive family by one explicit
topology class record so robust admitted workflows are machine-visible rather
than implicit:

```rust
pub enum WorthTopologyClass {
    WireOpen,
    WireClosed,
    WireBranch,
    SheetDisk,
    SheetPatch,
    SolidShellGenus0,
    NmtEdgeFan,
}
```

No admitted primitive family may be recognized only by ad hoc derived
inspection logic.

### Required topology entity kinds

Representative surface:

```rust
pub enum WorthTopologyEntityKind {
    Body,
    Lump,
    Region,
    Shell,
    Face,
    Loop,
    Halfedge,
    Edge,
    Vertex,
    Wire,
}

pub enum WorthNamingEntityKind {
    PersistentName,
}
```

Rules:

- `Wire` is not optional side metadata; it is a first-class topology entity
- `Body`, `Lump`, and `Region` are in scope now because shell, void, and later
  solid claims cannot be honest without hierarchy semantics from the beginning
- `PersistentName` is an entity family, not a string field stapled onto other
  records

### Required topology relation kinds

Representative surface:

```rust
pub enum WorthTopologyRelationKind {
    BodyOwnsLump,
    LumpOwnsRegion,
    RegionOwnsShell,
    ShellOwnsFace,
    FaceOwnsLoop,
    LoopOwnsHalfedge,
    WireOwnsHalfedge,
    HalfedgeStartsAtVertex,
    HalfedgeUsesEdge,
    HalfedgeNext,
    HalfedgePrev,
    HalfedgeRadialNext,
    FaceOuterLoop,
    FaceInnerLoop,
}

pub enum WorthNamingRelationKind {
    PersistentNameTargetsEntity,
}
```

Rules:

- ownership, loop-entry, local ring, and radial relations must be explicit,
  never inferred solely from one opaque topology blob
- Milestone 1 locks the following as authoritative truth relations:
  - `BodyOwnsLump`
  - `LumpOwnsRegion`
  - `RegionOwnsShell`
  - `ShellOwnsFace`
  - `FaceOwnsLoop`
  - `LoopOwnsHalfedge`
  - `WireOwnsHalfedge`
  - `HalfedgeStartsAtVertex`
  - `HalfedgeUsesEdge`
  - `HalfedgeNext`
  - `HalfedgePrev`
  - `HalfedgeRadialNext`
  - `FaceOuterLoop`
  - `FaceInnerLoop`
- Milestone 1 locks `FaceOwnsHalfedge` as derived-only and forbids treating it
  as a second ownership authority; face/halfedge membership is reconstructed
  from `FaceOwnsLoop` plus `LoopOwnsHalfedge`
- any relation needed for validator closure must exist as truth or be
  mechanically derived from declared truth; hand-wavy "the topo layer knows"
  is out of spec

### Required topology aspect kinds

Representative surface:

```rust
pub enum WorthTopologyAspectKind {
    TopologyStructure,
    TopologyOwnership,
    TopologyBoundary,
    TopologyRadial,
    NamingPersistentName,
    NamingTargeting,
}
```

Rules:

- aspect kinds must be explicit enough that bridge routing can invalidate only
  the right derived topology work
- aspect names must describe semantic delta, not internal implementation
  mechanics

### Required topology mutation records

Milestone 1 must define the mutation families that authoritative topology truth
can accept.

Representative surface:

```rust
pub enum WorthTopologyMutation {
    UpsertEntity {
        entity_id: WorthEntityId,
        kind: WorthEntityKind,
    },
    UpsertRelation {
        relation_id: WorthRelationId,
        kind: WorthRelationKind,
        source: WorthEntityId,
        target: WorthEntityId,
    },
    RemoveEntity {
        entity_id: WorthEntityId,
    },
    RemoveRelation {
        relation_id: WorthRelationId,
    },
}

pub struct WorthTopologyMutationBatch {
    pub mutations: Vec<WorthTopologyMutation>,
    pub touched_aspects: WorthAspectMask,
    pub mutation_origin: WorthMutationOrigin,
}
```

Rules:

- mutation records must be declarative truth deltas, not topo-owned imperative
  graph surgery scripts
- the batch must already know its touched aspect mask before derived work is
  asked to rerun
- mutation origin must distinguish seed, local edit, replay, and branch-local
  application

### Required proof-bearing commit flow

Milestone 1 must make these proof-bearing boundaries real:

```rust
pub struct RawWorthTopologyIntent { ... }
pub struct CanonicalTopologyMutationBatch { ... }
pub struct VerifiedTopologyCommit { ... }
pub struct PersistedTopologyTruthBatch { ... }
pub struct DerivedTopologyReadBasis { ... }
pub struct CertifiedTopologyInterpretation { ... }
```

Normative consequence:

- only one module may mint `VerifiedTopologyCommit`
- only one authority-path gateway may mint `PersistedTopologyTruthBatch`
- only one derived-path gateway may mint `CertifiedTopologyInterpretation`

## Canonical Primitive And Workflow Basis

Milestone 1 adopts the primitive-family proof rule from
[test-requirements.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/worth/test-requirements.md).

The minimum primitive families this milestone must admit are:

- `WireOpen(n)`
- `WireClosed(n)`
- `WireBranch(k)`
- `SheetDisk(n)`
- `SheetPatch(f)`
- `SolidShell(f)`
- `NmtEdgeFan(k)`

For each admitted family, Milestone 1 must prove:

- the smallest non-degenerate admitted member
- at least one larger generic member that is not a showcase shape
- at least one hostile admitted member near the milestone boundary
- at least one out-of-class member that fails cleanly

Robustness rule:

- an admitted family is not shipped until every member inside the declared
  admitted class is handled by the same authoritative truth model, the same
  validator ladder, the same replay rules, and the same diagnostic surfaces
- there is no "works for some branch valences" state for `WireBranch(k)`
- there is no "works for boxes and prisms" state for `SolidShell(f)`
- if a subset cannot be made fully robust now, that subset must be excluded
  explicitly and typed as clean-failing

This is a hard anti-cheat rule.
`WireClosed(3)` does not prove `WireClosed(n)`.
One star wire does not prove `WireBranch(k)`.
One cube does not prove `SolidShell(f)`.
One 3-face radial fan does not prove NMT.

## Authority And Validation Model

Milestone 1 must freeze four distinct surfaces and refuse to blur them.

### Freeze Definition

In this milestone, `freeze` has an operational meaning.

Freezing a surface means:

- no semantic widening without explicit roadmap and milestone-spec amendment
- no semantic narrowing that invalidates existing proof obligations without
  explicit roadmap and milestone-spec amendment
- no reinterpretation of the same truth records into a different authority or
  derivation role without explicit roadmap and milestone-spec amendment
- no changes to the admitted or excluded surface without matching updates to
  [worth_roadmap.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/worth/worth_roadmap.md)
  and
  [test-requirements.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/worth/test-requirements.md)
- no authoritative truth-vocabulary changes without corresponding migration or
  compatibility design
- no failure-taxonomy or artifact-shape changes without explicit versioning or
  proof-update consequences

`Freeze` does not mean "never improve implementation."
It means the semantic contract for the named surface is no longer allowed to
drift casually during implementation.

### 1. Authoritative topology truth

This surface owns:

- topology entities and topology relations
- persistent-name entities and targeting relations
- branch-local topology truth records

This surface lives in `forge-relational`.

### 2. Commit-boundary topology legality

This surface owns the cheapest and strongest structural checks that must block
illegal truth publication.

At minimum, it must cover these validator families for the admitted surface:

- reference integrity and ownership
- half-edge / loop wiring invariants
- radial-edge invariants for admitted NMT states
- vertex branch and vertex-disk invariants for admitted wire and NMT states
- shell closure, shell orientation, and solid-boundary legality for admitted
  `SolidShell(f)` workflows
- persistent-name uniqueness and dangling-reference legality
- rejection of impossible local structural states

### 3. Derived topology interpretation

This surface owns richer structural meaning that should remain derived and
rebuildable:

- shell closure interpretation
- wire classification
- wire-branch classification
- open-boundary interpretation
- admitted non-manifold adjacency interpretation
- closed-shell and solid-boundary interpretation
- region and shell summaries
- early naming continuity interpretation surfaces needed for later milestones

This surface must not redefine authoritative truth.

### 4. Certification-grade audits

This surface owns the deeper proof programs:

- hostile topology corruption localization
- replay parity proof
- branch-local history proof
- primitive-family proof coverage

These are allowed to be more expensive, but they must still be defined now.

### Branch-Local History Semantics

Milestone 1 does not yet widen into full branch and merge semantics, but it
must still define the minimum branch-local meaning clearly.

Milestone-1 branch-local rules:

- branch-local topology truth is authoritative within its branch context
- persistent-name uniqueness is enforced within the active branch truth basis,
  not globally across hypothetical future merge states
- replay parity for milestone 1 means:
  - identical truth conclusions
  - identical derived interpretation conclusions
  - identical machine-checkable artifact digests for the same canonical truth
    basis and evaluation policy
- milestone 1 may consume lineage-bearing runtime identity from relational, but
  it does not yet widen into full Worth-level history semantics beyond the
  branch-local truth basis needed for replay and inspection

Milestone 1 must therefore be honest about branch-locality without pretending
to have already solved later merge semantics.

## Proof-Carrying Workflow Surface

Milestone 1 must encode the first honest Worth topology proof chain.

Representative progression:

```rust
pub struct RawWorthTopologyIntent { ... }
pub struct CanonicalTopologyTruthMutation { ... }
pub struct VerifiedTopologyCommit { ... }
pub struct PersistedTopologyTruth { ... }
pub struct DerivedTopologyReadBasis { ... }
pub struct CertifiedTopologyInterpretation { ... }
```

Rules:

- raw intent may describe candidate topology and naming changes but carries no
  proof
- canonical topology mutation proves the mutation is expressed in the
  authoritative truth vocabulary
- verified topology commit proves commit-boundary invariants passed
- persisted topology truth proves the mutation became authoritative truth
- derived read basis proves a snapshot-backed truth view suitable for derived
  topology interpretation
- certified topology interpretation proves derived validators ran over a stable
  truth basis

Milestone 1 does not need these exact type names, but it must obey this proof
structure:

- no later phase accepts weaker pre-proof data once a stronger proof-bearing
  form exists
- exactly one authority path may mint persisted topology truth
- exactly one derived gateway may mint certified topology interpretation

## Invariant Allocation Table

Milestone 1 must allocate invariants explicitly so later convenience refactors
cannot move authority checks around carelessly.

| Invariant family | Proving phase | Enforcing subsystem | Failure family | Required proof surface |
| --- | --- | --- | --- | --- |
| entity existence and ownership | commit verification | `validation/ownership/` | `DanglingEntityReference` | topology truth digest |
| loop wiring symmetry | commit verification | `validation/loop_wiring/` | `BrokenLoopWiring` | topology validation digest |
| radial cycle legality | commit verification | `validation/radial/` | `IllegalRadialCycle` | topology localization report |
| admitted wire-branch vertex legality | commit verification | `validation/vertex_branching/` | `IllegalWireBranchState` | topology localization report |
| admitted shell closure and orientation legality | commit verification | `validation/shell_closure/` | `IllegalSolidShellState` | topology validation digest |
| persistent-name uniqueness | commit verification | `validation/naming/` | `DuplicatePersistentName` | naming truth digest |
| persistent-name targeting legality | commit verification | `validation/naming/` | `IllegalNameTarget` | naming attachment report |
| shell / wire / branch interpretation | derived interpretation | `interpretation/shells/`, `interpretation/wires/`, and `interpretation/vertex_branching/` | `DerivedTopologyInterpretationMismatch` | derived topology digest |
| branch-local replay parity | certification | `certification/replay/` | `TopologyReplayMismatch` | replay parity report |
| primitive-family coverage | certification | `certification/primitive_corpus/` | `PrimitiveFamilyCoverageGap` | primitive family coverage matrix |

Later refinements may split these into more files and more exact error kinds,
but no implementation may leave ownership of these invariants ambiguous.

## Failure Taxonomy

Milestone 1 must ship an explicit typed error family matrix at minimum
covering:

- `DanglingEntityReference`
- `IllegalOwnershipRelation`
- `BrokenLoopWiring`
- `IllegalLoopCardinality`
- `IllegalRadialCycle`
- `IllegalRadialOrdering`
- `IllegalWireBranchState`
- `IllegalSolidShellState`
- `OutOfScopeNmtState`
- `DuplicatePersistentName`
- `IllegalNameTarget`
- `TopologyMutationTouchesUnknownAspect`
- `OutOfClassPrimitiveMember`
- `TopologyReplayMismatch`
- `DerivedTopologyInterpretationMismatch`

Rules:

- commit, read, replay, and certification paths must map failures into these
  families or explicit refinements of them
- host or backend-specific errors must not leak as the public topology semantic
  error surface
- primitive-family out-of-class failures must be typed distinctly from internal
  bugs

## Diagnostic Artifact Families

Milestone 1 must emit machine-checkable diagnostic artifacts, not just strings.

Required artifact families:

- `TopologyTruthDigest`
  Canonical digest of persisted milestone-1 topology and naming truth.
- `TopologyValidationDigest`
  Digest and summary of commit-boundary invariant participation.
- `TopologyLocalizationReport`
  Exact offending entities, relations, invariant family, and boundary of
  rejection or warning.
- `NamingAttachmentReport`
  Persistent-name attachment and targeting summary.
- `PrimitiveFamilyCoverageMatrix`
  Coverage record showing smallest admitted, larger admitted, hostile admitted,
  and out-of-class members for every primitive family.
- `ReplayParityReport`
  Comparison record for accepted and rejected histories across live and replayed
  runs.

Rules:

- artifact families must be derivable from authoritative truth and deterministic
  evaluation, not hand-curated logs
- every artifact family must have a stable machine-readable shape
- diagnostic richness may widen later, but these families must already exist

## Complexity Contracts

Milestone 1 is not making the final performance claims for Worth, but it must
still declare and test its first structural cost boundaries.

Minimum contracts:

- commit-boundary topology verification cost is proportional to:
  - touched entity count
  - touched relation count
  - local adjacency required by the admitted validator family
- derived topology interpretation cost is proportional to:
  - invalidated aspect surface
  - affected shell / wire / radial neighborhood breadth
- replay parity cost is proportional to:
  - history length under replay
  - derived interpretation rerun breadth

Minimum counters:

- `topology_entity_upsert_count`
- `topology_relation_upsert_count`
- `topology_relation_remove_count`
- `commit_boundary_validator_count`
- `commit_boundary_rejection_count`
- `derived_topology_interpretation_count`
- `derived_topology_full_fallback_count`
- `naming_target_lookup_count`
- `primitive_family_member_count`
- `replay_history_length`
- `replay_interpretation_rerun_count`

Milestone 1 is not closed if these counters are missing or if the
specification cannot say what breadth they are meant to bound.

## Required Internal Subsystems

Milestone 1 must decompose by responsibility:

- `facade/`
  Public Worth topology and naming entrypoints.
- `authority/`
  Entity kinds, relation kinds, aspect kinds, mutation batches, and sealed
  proof-bearing commit types.
- `schema/`
  Runtime-facing schema registration for topology, naming, aspects, and
  invariant groups.
- `validation/ownership/`
  Reference integrity and ownership invariants.
- `validation/loop_wiring/`
  Loop, halfedge, and local ring invariants.
- `validation/radial/`
  Admitted radial and NMT invariants.
- `validation/vertex_branching/`
  Wire-branch legality and admitted vertex-disk / branch partition invariants.
- `validation/shell_closure/`
  Closed-shell, region-boundary, and admitted `SolidShell(f)` invariants.
- `validation/naming/`
  Persistent-name uniqueness and targeting invariants.
- `interpretation/shells/`
  Derived shell and region meaning.
- `interpretation/wires/`
  Derived wire and open-boundary meaning.
- `interpretation/vertex_branching/`
  Derived wire-branch and admitted vertex-disk interpretation.
- `interpretation/radial/`
  Derived admitted non-manifold interpretation.
- `diagnostics/`
  Diagnostic bundle types, counters, and reporting helpers.
- `certification/primitive_corpus/`
  Primitive-family certification harnesses and coverage matrices.
- `certification/replay/`
  Replay and branch-local parity certification.

This layout follows `domain_laws.md`: separate by what changes and fails for
different reasons, not by generic layering or giant validation buckets.

## Public Worth Surface

Milestone 1 must expose one public Worth façade with explicit authority
vocabulary.

Representative surface:

```rust
pub struct WorthSchemaBuilder { ... }
pub struct WorthTopologyAuthority { ... }
pub struct WorthTopologyReader { ... }

impl WorthSchemaBuilder {
    pub fn new() -> Self;
    pub fn with_topology_kinds(self) -> Self;
    pub fn with_naming_kinds(self) -> Self;
    pub fn build(self) -> Result<WorthSchemaRegistry, WorthSchemaBuildError>;
}

impl WorthTopologyAuthority {
    pub fn apply_topology_intent(
        &mut self,
        intent: RawWorthTopologyIntent,
    ) -> Result<PersistedTopologyTruthBatch, WorthTopologyCommitError>;
}

impl WorthTopologyReader {
    pub fn read_basis(
        &self,
    ) -> Result<DerivedTopologyReadBasis, WorthTopologyReadError>;

    pub fn interpret(
        &self,
        basis: &DerivedTopologyReadBasis,
    ) -> Result<CertifiedTopologyInterpretation, WorthTopologyInterpretationError>;
}
```

Surface rules:

- public methods expose topology and naming authority concepts only
- public methods do not expose internal relational row details or ad hoc topo
  caches
- commit and interpretation remain explicit boundary crossings
- any helper that bypasses proof-bearing transitions is out of spec

## Phases

The phases below are the linear implementation sequencing of the authority and
validation model above.

They are not an alternate semantic model.
They are the order in which the frozen surfaces above must be made real.

Implementation mapping rule:

- `worth-schema` owns authoritative truth vocabulary, schema registration,
  aspect vocabulary, invariant-group declarations, mutation records, proof-
  bearing authority-path types, and diagnostic bundle types that describe truth
  and commit-boundary legality
- `worth-topo` owns commit-boundary validator implementations, derived
  interpretation implementations, topology read-basis handling, primitive-family
  certification harnesses, replay parity harnesses, and topology-specific
  diagnostic materialization
- neither crate may invent a second topology authority
- if a build target does not clearly belong to one of those ownership surfaces,
  the spec is not yet resolved enough to implement honestly

### Phase 1: Freeze The Admitted Class

Phase 1 locks exactly what milestone 1 does and does not claim.

Required work:

- freeze the admitted primitive families:
  - `WireOpen(n)`
  - `WireClosed(n)`
  - `WireBranch(k)`
  - `SheetDisk(n)`
  - `SheetPatch(f)`
  - `SolidShell(f)`
  - `NmtEdgeFan(k)`
- freeze the admitted-class boundaries for:
  - branch-topology class
  - closed, orientable, genus-0 shell class
  - admitted radial fan class
- freeze the explicit excluded surface for:
  - `NmtVertexPinch(d)`
  - void-shell solids
  - higher-genus shells
  - non-orientable shells
- freeze the anti-cheat rule that no toy subset counts as closure

Exit condition:

- milestone 1 has a closed admitted surface
- milestone 1 has a closed excluded surface
- no admitted family is still described as partial, early, or provisional

Implementation targets:

- `worth-schema`
  - add explicit primitive-class enums and IDs for every admitted family
  - add admitted/excluded-class constants consumed by later schema and harness
    code
  - expose the milestone-1 admitted-class contract through the public facade
- `worth-topo`
  - add primitive-family scenario descriptors used by certification harnesses
  - add out-of-class scenario descriptors for explicit clean-fail testing
  - reject any helper that hardcodes toy closure as milestone completion

### Phase 2: Freeze Authoritative Truth Vocabulary

Phase 2 defines the exact truth surface that authoritative Worth topology uses.

Required work:

- define topology entity kinds
- define naming entity kinds
- define topology relation kinds
- define naming relation kinds
- define topology class records for admitted primitive families
- freeze which relations are authoritative and which are derived-only
- freeze the distinction between:
  - topology identity
  - naming identity
  - lineage identity
  - display labels

Exit condition:

- Worth can express the full admitted class directly in authoritative truth
- no topo-owned shadow structure is required to understand milestone-1 truth
- authoritative versus derived relation ownership is no longer ambiguous

Implementation targets:

- `worth-schema`
  - finalize topology entity-kind modules
  - finalize topology relation-kind modules
  - finalize naming entity/relation modules
  - finalize topology-class and aspect-kind modules
  - remove any leftover placeholder or bootstrap wording from the facade
- `worth-topo`
  - update all topology readers/materializers to consume only the frozen
    authoritative relation set
  - remove any dependence on convenience relation guesses that are now
    derived-only

### Phase 3: Freeze Commit-Boundary Validator Ownership

Phase 3 assigns cheap, blocking legality checks exactly once.

Required work:

- define ownership and reference-integrity validators
- define loop-wiring validators
- define radial validators
- define wire-branch validators
- define shell-closure and solid-boundary validators
- define naming uniqueness and targeting validators
- define commit-boundary rejection semantics for illegal local structure

Exit condition:

- impossible truth is rejected before publication
- validator-family ownership is explicit and non-overlapping
- admitted `WireBranch(k)` and `SolidShell(f)` legality is enforced at commit
  time, not deferred to derived interpretation

Implementation targets:

- `worth-schema`
  - register invariant-group identifiers and public invariant-family vocabulary
  - define typed validator-result and commit-rejection surfaces for milestone 1
- `worth-topo`
  - implement validator modules under:
    - `validation/ownership/`
    - `validation/loop_wiring/`
    - `validation/radial/`
    - `validation/vertex_branching/`
    - `validation/shell_closure/`
    - `validation/naming/`
  - ensure the commit path runs these families from proof-bearing truth input,
    not topo-owned mutable state

### Phase 4: Freeze Mutation And Proof-Carrying Authority Flow

Phase 4 defines how topology truth is authored and what proofs each stage owns.

Required work:

- define raw topology intent
- define canonical topology mutation batches
- define verified topology commit proofs
- define persisted topology truth proofs
- define derived read-basis proofs
- define certified topology interpretation proofs
- define same-commit graph mutation semantics so entity creation and relation
  creation may target symbolic created endpoints inside one authoritative
  commit boundary
- make `VerifiedTopologyCommit` mean commit-boundary validator ownership has
  already been satisfied by the frozen Phase 3 validator families
- freeze mutation-origin semantics for:
  - seed
  - local edit
  - replay
  - branch-local application

Exit condition:

- there is one authoritative write path
- there is one derived interpretation entry path
- skipped proof transitions are structurally out of spec
- the proof-bearing authority path is defined in terms of already-frozen
  commit-boundary legality
- admitted topology creation does not require a second publish step just to
  attach relations to entities created in the same workflow

Implementation targets:

- `worth-schema`
  - implement raw-intent, canonical-mutation, verified-commit,
    persisted-truth-batch, and read-basis type modules
  - seal constructors so only the proving modules can mint stronger proof types
  - encode mutation-origin and touched-aspect contracts in the authority path
  - encode symbolic created-endpoint references so one authoritative commit can
    carry entity and relation creation for the same topology graph
- `worth-topo`
  - consume proof-bearing types only; do not accept weaker raw collections once
    stronger forms exist
  - add compile-time-facing tests or API-shape checks that make bypass paths
    obvious and unacceptable

### Phase 5: Freeze Derived Interpretation Ownership

Phase 5 defines the rebuildable topology meaning layer and prevents it from
becoming authority.

Required work:

- define derived shell interpretation
- define derived wire interpretation
- define derived wire-branch interpretation
- define derived open-boundary interpretation
- define derived radial fan interpretation
- define derived solid-shell interpretation
- define first naming-continuity interpretation surfaces needed by later
  milestones

Exit condition:

- shell, wire, branch, boundary, and solid meaning are derivable from truth
- derived interpretation does not redefine legality already owned by commit
  validation
- destroying derived state would not destroy milestone-1 meaning

Implementation targets:

- `worth-schema`
  - define derived-read-basis and certified-interpretation public types
  - define interpretation artifact type vocabulary consumed by diagnostics and
    tests
- `worth-topo`
  - implement interpretation modules under:
    - `interpretation/shells/`
    - `interpretation/wires/`
    - `interpretation/vertex_branching/`
    - `interpretation/radial/`
  - make interpretation consume stable read bases and emit typed summaries
  - forbid interpretation modules from mutating or patching authoritative truth

### Phase 6: Freeze Failure And Diagnostic Surfaces

Phase 6 defines exactly how milestone 1 succeeds, rejects, and explains.

Required work:

- freeze the milestone-1 failure taxonomy
- freeze machine-checkable diagnostic bundle shapes
- define localization surfaces for:
  - commit-boundary rejection
  - derived-interpretation mismatch
  - out-of-class primitive attempts
  - replay mismatch
- freeze named counters and complexity contracts for the authority path and
  derived path

Exit condition:

- every rejection path is typed and localizable
- every admitted success path can emit machine-checkable evidence
- milestone-1 cost claims are explicit and measurable

Implementation targets:

- `worth-schema`
  - finalize failure-taxonomy enums and diagnostic bundle schemas
  - finalize counter names and complexity-contract labels exposed at the facade
- `worth-topo`
  - implement localization and digest builders for:
    - topology truth digest
    - topology validation digest
    - naming attachment report
    - topology localization report
    - replay parity report
  - wire named counters into commit-boundary and interpretation paths

### Phase 7: Implement Primitive-Family Certification Harnesses

Phase 7 builds the proof machinery that exercises the admitted class as
families rather than examples.

Required work:

- implement certification harnesses for:
  - `WireOpen(n)`
  - `WireClosed(n)`
  - `WireBranch(k)`
  - `SheetDisk(n)`
  - `SheetPatch(f)`
  - `SolidShell(f)`
  - `NmtEdgeFan(k)`
- require smallest, larger generic, hostile admitted, and out-of-class members
  for every admitted family
- implement branch-local and replay parity harnesses over the admitted class
- implement primitive-family coverage matrices and parity reports

Exit condition:

- every admitted family is exercised as a family
- no admitted family can close using one showcase shape
- branch-local and replay proof machinery exists for the admitted class

Implementation targets:

- `worth-schema`
  - provide reusable seed/truth-authoring helpers that can express every
    admitted family without special-case host mutation paths
- `worth-topo`
  - implement primitive-corpus harness modules for:
    - `WireOpen(n)`
    - `WireClosed(n)`
    - `WireBranch(k)`
    - `SheetDisk(n)`
    - `SheetPatch(f)`
    - `SolidShell(f)`
    - `NmtEdgeFan(k)`
  - implement smallest/generic/hostile/out-of-class scenario generation
  - implement branch-local and replay parity harnesses over the same corpus

### Phase 8: Earn End-To-End Milestone Closure

Phase 8 proves the full milestone through the runtime stack.

Required work:

- run truth authoring through authoritative relational storage
- route topology and naming aspect changes through the bridge-facing aspect
  vocabulary
- run derived topology interpretation over stable read bases
- run primitive-family proof suites
- run branch-local and replay parity suites
- emit all required machine-checkable outputs

Exit condition:

- Worth can author, validate, interpret, replay, and inspect the full
  milestone-1 admitted class end to end
- `WireBranch(k)` and `SolidShell(f)` are fully robust within the admitted
  class
- milestone 1 is closed by family-grade proof, not by demos

Implementation targets:

- `worth-schema`
  - expose the final milestone-1 public facade without bootstrap-only escape
    hatches
  - ensure the schema and authority surfaces are sufficient for end-to-end
    truth authoring and read-basis creation
- `worth-topo`
  - run all admitted-family certification paths through the real authority and
    interpretation stack
  - emit every required machine-checkable output named in the test
    requirements
  - prove there is no remaining shadow authority needed to make admitted cases
    work

## Must Ship

- one authoritative topology truth model for the admitted milestone-1 topology
  entities and relations
- one authoritative persistent-name truth model with explicit targeting and
  uniqueness semantics
- one first-class topology and naming aspect vocabulary suitable for bridge
  routing and diagnostics
- commit-boundary invariant groups for the admitted topology surface
- derived topology interpretation surfaces for shell, wire, boundary, and
  admitted non-manifold meaning
- full milestone-1 truth, validation, and interpretation support for:
  - `WireBranch(k)`
  - `SolidShell(f)`
- typed failure families for impossible structure, illegal naming, and
  out-of-class primitive attempts
- one end-to-end proof path using relational truth, bridge-facing aspects, and
  derived topology interpretation together
- canonical primitive-family proof coverage for:
  - `WireOpen(n)`
  - `WireClosed(n)`
  - `WireBranch(k)`
  - `SheetDisk(n)`
  - `SheetPatch(f)`
  - `SolidShell(f)`
- `NmtEdgeFan(k)`
- full robustness across the admitted class for every milestone-1 primitive
  family; no admitted family may ship as partial coverage

### Milestone-Lock Decisions That Must Exist By Closeout

The following milestone-lock decisions must exist explicitly by closeout:

- admitted surface frozen
- excluded surface frozen
- authoritative versus derived relation ownership frozen
- one authoritative topology write path frozen
- one derived interpretation entry path frozen
- branch-local replay semantics frozen for the milestone-1 truth basis
- failure taxonomy frozen
- diagnostic artifact bundle shapes frozen
- anti-cheat closure rules frozen

## Must Preserve

- `forge-relational` remains the sole authority for committed Worth truth
- `forge-signal` remains derived and disposable
- `forge-runtime-bridge` remains the causal boundary rather than topology
  authority
- persistent names remain distinct from storage identity, lineage identity, and
  display labels
- commit-boundary checks stay local enough to remain authority-path checks
- derived topology interpretation must not mutate truth or silently become
  authority
- primitive-family proof must stay parameterized; no fixed-shape success may be
  treated as general closure
- anything not fully robust inside milestone 1 must be removed from the
  admitted surface and fail closed instead of shipping half-implemented

## Acceptance Evidence

Milestone 1 is complete only when the named milestone-1 proof section in
[test-requirements.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/worth/test-requirements.md)
passes with machine-checkable evidence.

Required machine-checkable outputs:

- `topology_truth_digest`
- `topology_validation_digest`
- `topology_validation_report`
- `naming_truth_digest`
- `topology_localization_report`
- `naming_attachment_report`
- `milestone_1_counter_report`
- `primitive_family_coverage_matrix`
- `primitive_corpus_parity_report`
- `branch_local_topology_report`
- `milestone_1_replay_parity_report`
- `bridge_proof_report`

Milestone-specific proof obligations:

- every admitted primitive family includes:
  - smallest non-degenerate admitted member
  - larger generic member
  - hostile admitted member near the milestone boundary
  - explicit out-of-class clean-fail member
- `WireBranch(k)` must prove arbitrary admitted branch valence, not one
  showcase star node
- `SolidShell(f)` must prove arbitrary admitted closed-shell face counts, not
  one cube or prism
- every admitted primitive family must succeed or typed-fail uniformly across
  its declared admitted class; partial admitted subsets are forbidden
- commit-boundary invariants reject impossible topology before publication
- persistent-name uniqueness and targeting legality are enforced at the truth
  boundary
- derived topology interpretation reconstructs admitted shell, wire, boundary,
  and radial meaning deterministically from truth
- branch-local history and replay preserve the same conclusions for the same
  authoritative truth basis

Milestone 1 is not closed by seeding one cube, one tetrahedron, or one hand-made
NMT example.

## Non-Closure Conditions

Milestone 1 is not closed if any of the following remain true:

- one or more admitted primitive families are only proven on showcase members
- legality for an admitted workflow is still effectively deferred to derived
  interpretation
- diagnostic outputs are not machine-checkable
- proof-bearing transitions can be bypassed from public or normal production
  code paths
- a shadow topo authority is still required to interpret admitted truth
- the admitted class was widened without corresponding primitive-corpus and
  test-requirements updates
- `WireBranch(k)` is only robust for a subset of branch valences inside the
  admitted class
- `SolidShell(f)` is only robust for boxes, prisms, or other showcase shells
  rather than the admitted genus-0 shell class
- replay reproduces the same high-level meaning but not the same required
  machine-checkable artifact digests for the same canonical truth basis

## Architectural Notes

- persistent naming is not a convenience feature; it is part of the truth model
  because Worth's product thesis depends on referential continuity across edit,
  replay, branch, merge, and audit surfaces
- shell and wire semantics must exist in the first topology truth model because
  backfilling them later would force both schema and validator re-foundation
- the validator ladder replaces the old multi-layer "some validation happens in
  topo, some somewhere else" pattern with explicit runtime ownership
- primitive families are architecture, not just tests; if the truth model
  cannot express them honestly, the milestone has failed structurally

## Sequencing Notes

This milestone belongs first because every later topology, geometry, feature,
merge, and interaction milestone depends on it.

- `Milestone 2` can only materialize derived topology honestly once topology
  and naming authority are frozen
- `Milestone 3` can only ship topology edits honestly once the primitive
  families and validator ownership are fixed
- `Milestone 6` geometry binding must attach to stable topological authority,
  not to a semantically incomplete topology model

If Milestone 1 is weak, the rest of the roadmap turns into repeated arguments
about where topology and identity "really" live. This spec exists to prevent
that failure mode entirely.
