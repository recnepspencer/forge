# Spec-Graph-Native Topology Migration Engineering Spec

## Summary

Replace `forge-topo`'s current arena-centered truth model with a **new spec-graph truth runtime** owned by a dedicated crate, `forge-spec`.

In the new architecture:

- The **spec graph is the source of truth**
- `forge-topo` becomes a **B-Rep projection + topology query/validation crate**
- `forge-kernel` operates on **`SpecState` + derived projections**, not on `TopologyArena`
- `forge-signal` remains the **evaluation/control plane**, not the truth container
- Undo becomes **snapshot restore**
- Merge becomes **graph merge**
- Persistent naming, replay, lineage, and certification all become **graph-native**

This is a **parallel-core migration**, not an in-place arena mutation. We will build the new truth runtime beside the existing arena system, establish full projection parity, port operators and kernel access incrementally, then delete `TopologyArena`.

## Why The Truth Runtime Must Be A New Crate

This should live in a new crate, `forge-spec`, not inside `forge-topo` and not inside `forge-core`.

### `forge-topo` is the wrong long-term owner
`forge-topo` is only one consumer of the future truth model. The truth graph will also be used by:

- feature graph execution
- UI state / selections / inspector queries
- persistent naming
- replay / audit / certification
- physics / engineering assumptions
- AI planning / agent edits
- geometry / boolean / fillet orchestration

That makes topology a **projection domain**, not the owner of truth.

### `forge-core` is the wrong long-term owner
`forge-core` should remain the home for:

- shared error types
- shared tracing/audit types
- shared policy / invariant contracts
- crate-neutral common abstractions

It should not own a heavy graph runtime, snapshot engine, merge engine, typed schema, or domain storage.

### `forge-spec` is the correct owner
`forge-spec` becomes the product-level source of truth:

- stable graph identities
- typed schema
- snapshots / drafts / merge
- graph-native provenance
- graph-native naming anchors
- serialization of the model itself

That aligns directly with the vision: the specification graph is the product.

---

## Goals

1. Replace `TopologyArena` as the topology source of truth.
2. Make all topology entities first-class graph nodes and relations.
3. Make B-Rep a deterministic, disposable projection of graph truth.
4. Make undo/redo graph-snapshot operations, not arena rollbacks.
5. Make merge a three-way graph merge, not a topology blob collision.
6. Make persistent naming, replay, lineage, and audit graph-native.
7. Preserve high-performance local topology algorithms through dense projected views.
8. Keep `forge-signal` domain-free and use it only for scheduling derived state.

## Non-Goals

1. Do not turn `forge-signal` into the truth graph.
2. Do not make half-edge structural cycles part of the evaluation DAG.
3. Do not keep `TopologyArena` as a hidden long-term truth layer.
4. Do not preserve backward compatibility wrappers once the cutover phase is reached.
5. Do not model NURBS control points or numerical solver internals as individual signal DAG nodes in this phase.

---

## Locked Decisions

1. **Migration shape:** parallel-core migration.
2. **Truth runtime home:** new crate `forge-spec`.
3. **Graph schema style:** typed relation graph, not a generic property graph.
4. **Public topo handles:** become projection-only handles during migration, then replace old arena truth handles.
5. **Truth/evaluation separation:** structural truth graph and reactive evaluation DAG remain separate systems.
6. **Transaction semantics for truth graph:** immutable `SpecState` + mutable `SpecDraft`, commit produces a new snapshot, rollback is drop / snapshot restore.
7. **Undo model:** snapshot restore only.
8. **Merge model:** deterministic three-way graph merge against common base.
9. **B-Rep performance model:** dense projected topology remains mandatory for traversal-heavy operations.
10. **Backwards compatibility:** no permanent dual-truth model and no permanent adapter shims.

---

# 1. Target Architecture

## 1.1 Three-layer model

### Layer A: Truth graph (`forge-spec`)
Owns:
- model state
- topology state
- feature/constraint/parameter state
- graph-native identity
- graph transactions
- graph snapshots
- graph diff/merge
- graph-native naming anchors
- graph-native lineage/replay

### Layer B: Evaluation/control plane (`forge-signal`)
Owns:
- invalidation
- projection refresh scheduling
- validator scheduling
- analysis scheduling
- UI/physics/reactive downstream computation scheduling

It does **not** own topology truth.

### Layer C: Derived projections (`forge-topo`, later others)
Owns:
- B-Rep projection materialization
- dense traversal structures
- topology queries
- topology validators
- topology-local derived indexes

It does **not** own truth.

---

## 1.2 Crate boundaries

### `forge-spec` (new)
Owns:
- truth graph
- schema
- graph node/relation storage
- snapshots/drafts
- diff/merge
- graph-native provenance
- graph-native naming anchors
- graph serialization

### `forge-topo` (re-scoped)
Owns:
- B-Rep projection builder
- projected topology data structures
- topology queries
- topology validators
- topology-local projection caches
- graph-to-topology projection diagnostics

### `forge-kernel`
Owns:
- feature execution
- geometry generation
- conditioning / pipeline orchestration
- boolean/fillet/etc orchestration against `SpecDraft`
- output envelopes built from `SpecState` + derived projections

### `forge-core`
Owns:
- neutral errors
- neutral tracing / decision / audit types
- policy/invariant common contracts
- no truth graph runtime

### `forge-signal`
Owns:
- reactive scheduling only
- no truth ownership
- no structural cycles

---

# 2. Truth Graph Model (`forge-spec`)

## 2.1 Identity model

### New public types
- `SpecNodeId`
- `SpecRelationId` if relation identity is needed for replay/conflict attribution
- `SpecState`
- `SpecDraft`

### `SpecNodeId`
`SpecNodeId` is the true identity for all persisted graph nodes.

#### Requirements
- globally unique within a model
- stable across snapshots
- serialized as a human-readable deterministic string
- not tied to slot index
- survives graph reallocation and compaction
- suitable for merge and persistent naming

#### Decision
Use a **stable opaque 128-bit identifier** serialized as a string, minted by a deterministic identity allocator:

- allocator namespace = model root id + operation identity + semantic creation role
- node id generation is deterministic within a replayed operation sequence
- the allocator does **not** depend on slot index or insertion position

#### Rationale
This gives:
- stable persisted ids
- merge-safe identity
- deterministic replay
- no arena-slot coupling

### Projection handles
`forge-topo` introduces:
- `ProjectedBodyId`
- `ProjectedLumpId`
- `ProjectedRegionId`
- `ProjectedShellId`
- `ProjectedFaceId`
- `ProjectedLoopId`
- `ProjectedHalfEdgeId`
- `ProjectedEdgeId`
- `ProjectedVertexId`

These are:
- dense
- ephemeral
- projection-scoped
- invalid outside a specific `ProjectedTopology`

They replace current truth-style arena handles.

---

## 2.2 Node domains and node kinds

The graph is typed by domain. Topology is only one domain.

### Domain A: Intent domain
- `Model`
- `Feature`
- `Constraint`
- `Parameter`
- `DesignDecision`

### Domain B: Topology domain
- `Body`
- `Lump`
- `Region`
- `Shell`
- `Face`
- `Loop`
- `HalfEdge`
- `Edge`
- `Vertex`

### Domain C: Geometry binding domain
- `SurfaceBinding`
- `CurveBinding`
- `CoedgeBinding`
- `VertexGeometryBinding`

These bind topological entities to geometry payloads without making numerical data itself part of graph adjacency.

### Domain D: Provenance/naming domain
- `NamingAnchor`
- `ReplayRecord`
- `LineageAnchor`

### Important rule
Heavy numeric payloads are **not** exploded into graph adjacency. A NURBS surface remains a structured payload block referenced by a binding node. The graph stores dependency and ownership semantics, not low-level control net traversal.

---

## 2.3 Relation kinds

Relations are strongly typed and schema-enforced.

### Intent relations
- `ModelOwnsFeature`
- `ModelOwnsConstraint`
- `ModelOwnsParameter`
- `FeatureConsumesParameter`
- `FeatureConsumesConstraint`
- `FeatureProducesTopology`
- `FeatureDependsOnFeature`
- `DecisionAffectsNode`

### Topology containment relations
- `BodyOwnsLump`
- `LumpOwnsRegion`
- `RegionOwnsShell`
- `ShellOwnsFace`

### Face/loop relations
- `FaceOuterLoop`
- `FaceInnerLoop`
- `LoopEntryHalfEdge`

### Half-edge structural relations
- `HalfEdgeNext`
- `HalfEdgeRadialNext`
- `HalfEdgeUsesEdge`
- `HalfEdgeOriginVertex`
- `HalfEdgeBoundsFace`

### Geometry binding relations
- `FaceUsesSurfaceBinding`
- `EdgeUsesCurveBinding`
- `HalfEdgeUsesCoedgeBinding`
- `VertexUsesGeometryBinding`

### Naming/provenance relations
- `NamingAnchorTargetsNode`
- `LineageAnchorDerivedFrom`
- `ReplayRecordAppliesToFeature`
- `ReplayRecordTouchesNode`

---

## 2.4 Cardinality and invariants

These are schema invariants, not optional conventions.

### Containment
- `Body -> Lump`: one-to-many
- `Lump -> Region`: one-to-many
- `Region -> Shell`: one-to-many
- `Shell -> Face`: zero-to-many

### Face/loop
- each `Face` has exactly one outgoing `FaceOuterLoop`
- each `Face` has zero or more outgoing `FaceInnerLoop`
- each `Loop` has exactly one outgoing `LoopEntryHalfEdge`

### Half-edge
- each `HalfEdge` has exactly one outgoing `HalfEdgeNext`
- each `HalfEdge` has exactly one outgoing `HalfEdgeRadialNext`
- each `HalfEdge` has exactly one outgoing `HalfEdgeUsesEdge`
- each `HalfEdge` has exactly one outgoing `HalfEdgeOriginVertex`
- each `HalfEdge` has exactly one outgoing `HalfEdgeBoundsFace`

### Derived, not stored
The following are no longer truth fields:
- `prev`
- primary disk pointer
- extra NMT disk sidecars
- radial valence cache
- shell face reverse indexes
- face halfedge reverse indexes
- vertex halfedge reverse indexes
- loop membership vectors
- edge shell sidecars
- bridge flags as topology truth

These become:
- derived projection structure
- derived indexes
- or explicit non-topological metadata in separate domains

---

## 2.5 Deterministic ordering rules

All observable ordering must be deterministic.

### Rules
- node serialization order: ascending `SpecNodeId`
- relation serialization order: `(relation_kind, source_id, target_id, ordinal)`
- `FaceInnerLoop` enumeration order: explicit deterministic ordinal
- projection traversal entry ordering:
  - shells by `SpecNodeId`
  - faces by `SpecNodeId`
  - loops: outer first, then inners by ordinal
  - radial ring traversal starts from canonical smallest `SpecNodeId` in ring if no entry relation exists

### Important decision
Ordering is never inferred from hash iteration or insertion order.

---

# 3. Storage Architecture (`forge-spec`)

## 3.1 Data-oriented storage shape

The truth graph is not stored as heap-scattered generic node objects.

### Required internal structure
- dense node arena by node slot
- stable `SpecNodeId -> NodeIndex` lookup table
- typed relation tables, one per relation kind
- forward and reverse adjacency indexes by typed relation
- payload stores separate from adjacency stores

### Internal structures
- `NodeArena`
- `NodeMetaStore`
- `RelationStore<R>`
- `PayloadStore<P>`
- `IdIndex`

### Lookup policy
- hot point lookups may use fixed-hasher maps internally if iteration is not observable
- observable iteration uses sorted stable sequences only

### Why this matters
The spec graph will host massive models. Generic `Rc<RefCell<Node>>` style graph storage is disallowed.

---

## 3.2 Payload strategy

Heavy payloads live in typed stores, not embedded in every node record.

Examples:
- parameter payloads
- feature DTO payloads
- surface specs
- curve specs
- coedge specs
- vertex geometry payloads

### Rules
- payload id is referenced from the owning node
- payload storage is immutable within a committed `SpecState`
- payload changes in a draft produce new payload records or payload patches, not in-place mutation of committed state

---

# 4. Transaction Model (`forge-spec`)

## 4.1 Public types

### `SpecState`
Immutable committed snapshot.

Owns:
- committed graph
- committed payload stores
- committed lineage log
- committed naming anchor table
- committed replay records
- monotonic state epoch
- structural/spec hash

### `SpecDraft`
Transactional mutable view.

Owns:
- base snapshot reference
- copy-on-write overlays for nodes/relations/payloads
- mutation journal
- replay staging
- lineage staging
- naming-anchor staging
- merge metadata if draft is merge-derived

## 4.2 Transaction semantics

### Begin
`SpecState::into_draft()` produces a `SpecDraft`.

### Execute
Mutation programs operate against `SpecDraft`.

### Commit
`SpecDraft::commit()`:
- validates schema
- validates required projection invariants
- finalizes replay/lineage/naming staging
- produces a new immutable `SpecState`

### Rollback
Dropping or explicit rollback discards the draft overlay.

### Undo
Undo is restoring a previous `SpecState`.

No arena hard-rewind model is used here. Truth uses immutable snapshots.

---

## 4.3 Mutation journal

Replace topology mutation journal with graph-native journal:

### New staged records
- `NodeCreated`
- `NodeDeleted`
- `NodePayloadChanged`
- `RelationAdded`
- `RelationRemoved`
- `AnchorCreated`
- `AnchorRetargeted`

### Journal purpose
- replay/audit
- merge diagnostics
- conflict explanations
- projection invalidation routing
- certification trace

---

# 5. Replay, Lineage, and Naming

## 5.1 Replay

Current replay is topo-op/arena-hash centric. Replace it with graph-native mutation records.

### New type
- `SpecReplayRecord`

Fields:
- operation identity
- mutation program type
- input parameters
- pre-state hash
- post-state hash
- touched node ids
- touched relation kinds
- deterministic mutation trace
- projection refresh trace
- decision delta

### Important rule
Replay records reference `SpecNodeId`, never projection handles.

---

## 5.2 Lineage

Lineage becomes graph-native.

### New rule
Lineage attaches to truth nodes, not arena handles.

### New structure
Each truth node has:
- creation operation
- producing feature
- parent lineage anchors or parent node refs
- deterministic ancestry hash
- optional semantic derivation role

### Why keep ancestry hash
Because it still helps:
- audit
- deterministic identity checks
- naming fallback
- equivalence reasoning across merges

But it is no longer the only naming mechanism.

---

## 5.3 Persistent naming

Replace topology-local ancestry-only naming with graph-native naming anchors.

### New truth node kind
`NamingAnchor`

### Naming anchor fields
- stable anchor id
- target node id
- node kind
- semantic role
- optional ordinal/disambiguator
- origin feature id
- origin operation id
- retargeting lineage

### Public naming object
`PersistentName` becomes:
- anchor id
- expected target kind
- optional semantic subselector

### Resolution rule
Persistent names resolve through naming anchors first, lineage fallback second.

### Why
This is required for:
- rebuild survival
- merge survival
- branch conflict reasoning
- explicit naming semantics across feature edits

---

# 6. Merge Model

## 6.1 Merge input
Three-way merge:
- base `SpecState`
- left `SpecState`
- right `SpecState`

## 6.2 Merge output
Either:
- merged `SpecState`
- or typed `GraphMergeConflictSet`

## 6.3 Conflict types

### Structural conflicts
- same node deleted on one side, modified on the other
- same single-cardinality relation retargeted differently
- same ordered relation slot changed differently
- loop/radial relation edits produce incompatible cycle structure
- shell/face ownership conflicts

### Payload conflicts
- same feature payload field changed differently
- same parameter value changed differently
- same geometry binding payload changed differently

### Naming/provenance conflicts
- same naming anchor retargeted differently
- same lineage anchor rewritten incompatibly

### Projection-invalid conflicts
- merged graph passes raw schema merge but cannot produce a valid projection
- result is conflict, not silent invalid state

## 6.4 Auto-merge rules

Auto-merge only when:
- touched subgraphs are disjoint
- relation cardinalities remain valid
- ordering semantics are not in conflict
- merged result passes projection prevalidation

## 6.5 Deterministic merge
Merge order and conflict ordering are deterministic:
- conflicts sorted by `(domain, relation/node kind, target id)`

---

# 7. B-Rep Projection Layer (`forge-topo`)

## 7.1 New responsibility
`forge-topo` becomes a **projection and topology reasoning crate**.

It no longer owns truth mutation.

## 7.2 New public types
- `ProjectedTopology`
- `ProjectedBodyId`
- `ProjectedLumpId`
- `ProjectedRegionId`
- `ProjectedShellId`
- `ProjectedFaceId`
- `ProjectedLoopId`
- `ProjectedHalfEdgeId`
- `ProjectedEdgeId`
- `ProjectedVertexId`
- `ProjectionBuilder`
- `ProjectionCache`

## 7.3 Projection contract

### Input
- `&SpecState`
- optional subgraph root(s) for partial projection

### Output
A dense, immutable, traversal-efficient `ProjectedTopology` with:
- arrays for projected entities
- transient handles
- dense adjacency
- derived `prev`
- derived vertex disk structure
- derived radial classes
- derived reverse indexes

### Projection rules
- `HalfEdgeNext` relation becomes projected `next`
- `prev` is derived during projection
- `HalfEdgeRadialNext` becomes projected radial ring link
- `HalfEdgeOriginVertex` becomes projected origin
- `HalfEdgeUsesEdge` becomes projected edge binding
- `HalfEdgeBoundsFace` becomes projected face membership
- `LoopEntryHalfEdge` seeds deterministic loop traversal
- `FaceOuterLoop` / `FaceInnerLoop` become projected face loop sets
- containment relations become projected body/lump/region/shell structure

### Validation at projection time
Projection fails if:
- required cardinality is missing
- loop traversal is non-closing
- radial traversal is invalid
- ownership cycle is invalid
- projection emits duplicate halfedge membership in one loop
- projected graph violates required B-Rep projection invariants

Projection failure is a typed error, not UB.

---

## 7.4 Performance rule
Traversal-heavy algorithms must use `ProjectedTopology`, not direct truth-graph walks.

This is mandatory for:
- booleans
- fillets
- local adjacency traversals
- shell extraction
- loop/radial validation
- topological measurements

Truth graph is the source of truth. Projection is the algorithm substrate.

---

# 8. Operator Model

## 8.1 Replace `TopoOperator` with graph mutation programs

### New trait
`SpecMutation`

Proposed shape:
```rust
pub trait SpecMutation: Debug {
    type Output;

    const NAME: &'static str;
    const SCHEMA_VERSION: u32 = 1;
    const INVARIANT_CONTRACT: InvariantContract;

    fn execute(
        &self,
        draft: &mut SpecDraft,
        recorder: &mut SpecLineageRecorder,
    ) -> Result<MutationResult<Self::Output>, KernelError>;

    fn semantic_summary(&self) -> String;
}
```

### `MutationResult`
Contains:
- `value`
- `declared_effects`
- `touched_truth_domains`
- optional projection expectations if needed for audit

## 8.2 Euler operators
Euler operators stop being primitive truth semantics.

### Transitional role
During migration:
- old Euler op families are classification labels for porting
- each old Euler op is re-expressed as one graph mutation program

### Final role
`EulerOpKind` is removed from the truth model.
Kernel contracts move to:
- `TopologyTransformKind`
- `ProjectionBehaviorKind`
- `TruthMutationKind`

---

# 9. Signal Integration

## 9.1 `forge-signal` remains external
Do not put truth graph structure into `forge-signal`.

## 9.2 Signal domains around `SpecState`
`forge-signal` consumes mutation effects emitted by `SpecDraft` commit.

### Initial signal domains
- `ProjectionComponent`
- `ProjectionReverseIndex`
- `NamingResolution`
- `LineageIndex`
- `ReplayDigest`
- `ValidationSnapshot`
- `SpatialPreview`
- `UISelectionProjection`
- future `PhysicsAnalysis`
- future `CostAnalysis`

## 9.3 Checkpoint policy
At minimum:
- per-mutation: disallowed for heavy topology projection
- per-operation: projection/component refresh
- per-commit: global validation / merged digest refresh
- on-demand: expensive analysis nodes

---

# 10. Public API Changes

## 10.1 New crate
Create `crates/forge-spec`

### Required root structure
```text
forge-spec/
  data/
  logic/
  presentation/
  facade.rs
```

## 10.2 `forge-spec` public facade exports
- `SpecNodeId`
- `SpecState`
- `SpecDraft`
- `SpecMutation`
- `SpecReplayRecord`
- `GraphMergeConflict`
- `PersistentName`
- `NamingAnchorId`
- graph schema enums/types
- diff/merge APIs

## 10.3 `forge-topo` public API replacement
Remove arena-truth exports over time.

Add:
- `ProjectedTopology`
- `ProjectionBuilder`
- projection handles
- query functions over projections
- validators over projections

Remove from public ownership surface:
- `TopologyArena`
- `TopologyState`
- `MutableDraft`
- truth-side `FaceId` / `VertexId` / etc

## 10.4 `forge-kernel` changes
`SolidEnvelope` changes to hold:
- `SpecState`
- `GeometryStore`
- lazy `ProjectedTopology` or projection cache

### Accessor behavior
- `topology()` returns `&SpecState`
- `projection()` lazily materializes or reuses `ProjectedTopology`
- `faces()`, `vertices()`, etc return projection handles, not truth ids

## 10.5 `forge-core` changes
Keep neutral shared types only.

Possible additions:
- generic merge conflict reporting traits
- generic graph audit summary types if needed

Do **not** move truth graph storage or schema there.

---

# 11. Repository Restructure

## 11.1 New crate: `forge-spec`
Suggested modules:

```text
crates/forge-spec/src/
  data/
    graph/
    identity/
    node/
    relation/
    payload/
    snapshot/
    schema/
    naming/
    lineage/
    replay/
  logic/
    transaction/
    mutation/
    diff/
    merge/
    validation/
    projection_effects/
  presentation/
    serialization/
    diagnostics/
    contracts/
  facade.rs
```

## 11.2 Re-scope `forge-topo`
Suggested modules:

```text
crates/forge-topo/src/
  projection/
    data/
    logic/
    cache/
    handles/
  queries/
  validators/
  testing/
  facade.rs
```

### Delete or migrate
Current `b_rep/data/storage/*` truth ownership code is removed after parity:
- `arena.rs`
- `slot.rs`
- `cache_runtime.rs`
- truth-side sidecars

---

# 12. Migration Phases

## Phase 0 — Spec and freeze
Goal:
- freeze arena-shape growth
- no new truth-side sidecars or long-lived arena-only contracts

Deliverables:
- architecture doc
- schema doc
- projection contract doc
- migration inventory

## Phase 1 — Create `forge-spec`
Build:
- ids
- node/relation schema
- snapshot/draft system
- serialization
- mutation journal
- replay/lineage/naming anchor core

No topo projection yet.

## Phase 2 — Build B-Rep projection in `forge-topo`
Build:
- `ProjectionBuilder`
- `ProjectedTopology`
- projected handles
- reverse indexes
- projection validators

Success criteria:
- can materialize current small topology cases from spec graph
- deterministic projection ordering locked

## Phase 3 — Parity harness
Build a parity test harness:
- current `TopologyArena` result vs graph-derived `ProjectedTopology`
- compare structural signatures
- compare loop/radial traversal results
- compare validator outcomes

Use this to port operators safely.

## Phase 4 — Port entity lifecycle ops
Port:
- `make_vertex_face`
- `make_edge_vertex`
- `make_edge_face`
- `split_edge`
- kill lifecycle inverses

These become `SpecMutation`s.

## Phase 5 — Port boundary and NMT ops
Port:
- face/loop editing
- sew/unsew
- join faces
- non-manifold boundary edits

## Phase 6 — Kernel cutover to `SpecState`
Update:
- `SolidEnvelope`
- feature pipeline
- geometry coupling
- output fingerprints
- decision/audit/replay integration

## Phase 7 — Merge/naming hardening
Finalize:
- graph merge
- naming anchors
- lineage retargeting
- replay parity
- conflict diagnostics

## Phase 8 — Delete old truth model
Remove:
- `TopologyArena`
- `TopologyState`
- `MutableDraft`
- topo event bus lifecycle runtime
- old topo replay/naming plumbing tied to arena handles

---

# 13. Tests And Acceptance Criteria

## 13.1 Truth graph correctness
1. create/update/delete nodes and relations in `SpecDraft`
2. commit produces immutable `SpecState`
3. rollback discards changes
4. snapshot restore returns exact prior state
5. stable `SpecNodeId` survives snapshots
6. deterministic serialization bytes for identical state

## 13.2 Schema invariants
1. missing required cardinality is rejected
2. duplicate single-cardinality relation is rejected
3. invalid loop/radial truth relations are rejected
4. invalid ownership chain is rejected
5. ordering ordinals are deterministic and validated

## 13.3 Projection correctness
1. simple manifold body projects correctly
2. boundary sheet projects correctly
3. non-manifold edge ring projects correctly
4. face loops preserve outer/inner semantics
5. projected `prev` matches inverse of `next`
6. projected vertex disks match expected rings
7. projection rejects invalid truth graph deterministically

## 13.4 Operator parity
For each ported operator family:
1. run legacy op on arena path
2. run graph mutation + projection path
3. compare projected structural signature
4. compare validators
5. compare deterministic replay summary
6. compare persistent naming outcomes where applicable

## 13.5 Merge tests
1. disjoint feature additions auto-merge
2. same scalar field edits conflict
3. delete-vs-modify conflicts
4. conflicting loop/radial rewires conflict
5. merge output ordering deterministic
6. merge that breaks projection returns typed conflict, not invalid state

## 13.6 Naming tests
1. anchor resolves after non-destructive rebuild
2. split child disambiguation deterministic
3. merge preserves non-conflicting anchors
4. retargeted anchor lineage is auditable
5. projection handles are never persisted as names

## 13.7 Performance/scale tests
1. large spec graph snapshot commit remains bounded
2. local projection refresh only touches affected components
3. booleans/fillets use dense `ProjectedTopology`, not raw graph walks
4. graph merge on large disjoint branches remains deterministic and bounded
5. serialization/deserialization of large specs remains deterministic

## 13.8 Kernel integration tests
1. `SolidEnvelope` lazily projects from `SpecState`
2. face/edge/vertex enumeration remains deterministic
3. topology fingerprint and full fingerprint remain deterministic
4. feature pipeline works without `TopologyArena`
5. replay/audit lineage outputs still populate expected envelopes

---

# 14. Explicit Assumptions And Defaults

1. `forge-spec` is the new truth-runtime crate.
2. `forge-topo` becomes a projection/query/validator crate.
3. `forge-signal` remains a scheduling runtime, never a truth graph.
4. `SpecNodeId` is a stable opaque id, not a slot index.
5. Topological structural relations are stored as typed graph relations.
6. `prev`, disk sidecars, radial caches, and reverse indexes are derived, not truth.
7. Projection handles are ephemeral and scoped to a specific `ProjectedTopology`.
8. No permanent dual-truth architecture is allowed.
9. No long-term backward compatibility wrappers will be retained after cutover.
10. Heavy numeric structures remain payload-backed, not graph-adjacency-expanded.
11. Merge is deterministic three-way merge with typed conflict output.
12. Undo is snapshot restore, not imperative inverse-op replay.
13. Existing replay/lineage/naming mechanisms are replaced where they are truth-model-specific.
14. Existing topology algorithms continue to rely on dense projected topology for performance.

---

# 15. Success Criteria

This migration is complete when all of the following are true:

1. `TopologyArena` is no longer the topology source of truth.
2. `SpecState` is the only committed topology truth snapshot.
3. `forge-topo` can build a deterministic `ProjectedTopology` from `SpecState`.
4. Topology algorithms run against projected dense topology, not truth graph adjacency.
5. `forge-kernel` consumes `SpecState` and projection APIs instead of arena APIs.
6. Replay, lineage, and naming are graph-native.
7. Undo is snapshot restore.
8. Merge is graph merge with typed conflicts.
9. Old arena/event-bus truth plumbing is deleted.
10. The spec graph, not the B-Rep, is the canonical serialized product.

