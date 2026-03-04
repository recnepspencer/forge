# B-Rep Structural Hardening

Four milestones that permanently stabilize the B-Rep:

| Milestone | Scope                                                                          |
| :-------- | :----------------------------------------------------------------------------- |
| **M0**    | Invariant Contract System — compile-time enforcement, traced validation        |
| **M1**    | Entity Struct Stabilization — metadata → side-car maps, structs frozen         |
| **M2**    | NMT Disk Entries — vertex connectivity for non-manifold pinch points           |
| **M3**    | NMT Queries + Validator Hardening — computed disk queries, per-disk validators |

---

## Milestone 0: Invariant Contract System

### The Registry

Every structural invariant lives in a single enum. Two exhaustive `match` statements
enforce completeness at compile time:

1. Every `TopoOperator`'s contract closure
2. The `validator_for()` dispatch function

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InvariantId {
    // ── Pointer coherence ───────────────────────────────────────
    RadialReciprocity,         // radial_next∘radial_next == id
    NextPrevReciprocity,       // next∘prev == id, prev∘next == id
    NoDanglingRefs,            // no HE refs to deleted entities
    GenerationalFreshness,     // no refs to recycled slots

    // ── Loop structure ──────────────────────────────────────────
    FaceHasLoop,               // every face has ≥1 loop
    LoopMinCardinality,        // every loop has ≥3 HEs
    NoDuplicateCoedges,        // no duplicate HEs in a loop
    FaceLoopMembership,        // loop HEs ref owning face
    VertexContinuity,          // adjacent HEs share a vertex
    EdgeEndpointsMatch,        // edge endpoints match loop vertices

    // ── Ownership ───────────────────────────────────────────────
    SingleLoopOwner,           // each loop → exactly one face
    NoOrphanHalfEdges,         // every HE belongs to a loop
    AcyclicContainment,        // containment hierarchy is a DAG
    InnerOuterConsistency,     // inner/outer loop nesting correct

    // ── Radial edge ─────────────────────────────────────────────
    RadialCycleUniqueness,     // radial ring has no duplicates
    RadialNeighborConsistency, // radial neighbors share same edge
    NoBrokenRadialSplices,     // radial ring continuity

    // ── Shell closure ───────────────────────────────────────────
    FaceAdjacencyConsistency,  // face adjacency is symmetric
    NoBrokenFaceBoundary,      // all loops close
    BoundaryEdgesLaminar,      // boundary edges not in solid shells

    // ── Vertex disk ─────────────────────────────────────────────
    DiskEntriesAlive,          // every disk_entry → alive HE
    DiskPartitionCorrect,      // disk_entries.len() == actual disk count
    DiskClosure,               // each disk cycle closes
    NoCrossDiskCoedges,        // no cross-disk co-edges

    // ── Euler formula ───────────────────────────────────────────
    PerComponentEuler,         // V - E + F = 2(S - G) per component

    // ── Side-car coherence ──────────────────────────────────────
    SideCarCoherence,          // side-car maps don't ref deleted entities
    IndexCoherence,            // cache indexes match entity data
}

impl InvariantId {
    pub const ALL: &[InvariantId] = &[
        Self::RadialReciprocity,
        Self::NextPrevReciprocity,
        // ... every variant listed exhaustively
    ];
}
```

---

### Invariant Relations (Pre/Post Semantics)

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InvariantRelation {
    /// Doesn't read or write state relevant to this invariant.
    Unrelated,

    /// Precondition: assumes the invariant holds on entry.
    Requires,

    /// Postcondition: guarantees the invariant holds on exit.
    Ensures,

    /// Temporarily violates during execution but restores before
    /// returning. Implies both Requires and Ensures.
    TemporarilyViolatesButEnsures,

    /// May leave this invariant violated after execution.
    /// The validator MUST check it post-op.
    MayBreak,
}
```

**Blame assignment:** If an invariant fails, the trace shows which operator
declared `MayBreak` vs. which subsequent operator declared `Requires` —
pinpointing the root cause.

---

### Compile-Time Enforcement: Exhaustive Match Function

```rust
pub struct InvariantContract {
    pub relation: fn(InvariantId) -> InvariantRelation,
}

impl InvariantContract {
    pub fn may_break(&self) -> impl Iterator<Item = InvariantId> + '_ {
        InvariantId::ALL.iter().copied()
            .filter(|id| matches!((self.relation)(*id), InvariantRelation::MayBreak))
    }

    pub fn requires(&self) -> impl Iterator<Item = InvariantId> + '_ {
        InvariantId::ALL.iter().copied()
            .filter(|id| matches!(
                (self.relation)(*id),
                InvariantRelation::Requires
                    | InvariantRelation::TemporarilyViolatesButEnsures
            ))
    }
}
```

**Why this works:** Rust's exhaustive pattern matching on a non-`#[non_exhaustive]`
enum means adding a new `InvariantId` variant instantly breaks every `match` that
doesn't cover it. Every operator's contract uses an exhaustive match, so every
operator must acknowledge every new invariant before the crate compiles.

### On `TopoOperator`

```rust
pub trait TopoOperator: std::fmt::Debug {
    type Output;

    fn execute(
        &self,
        draft: &mut MutableDraft,
        recorder: &mut LineageRecorder,
    ) -> Result<ExecutionResult<Self::Output>, KernelError>;

    const NAME: &'static str;

    /// Compile-time invariant contract.
    const INVARIANT_CONTRACT: InvariantContract;

    fn semantic_summary(&self) -> String { format!("{:?}", self) }
}
```

---

### Example: SplitEdge Contract

```rust
impl TopoOperator for SplitEdge {
    const INVARIANT_CONTRACT: InvariantContract = InvariantContract {
        relation: |id| match id {
            // Pointer coherence — rewire next/prev/radial, restore before return
            InvariantId::RadialReciprocity         => InvariantRelation::TemporarilyViolatesButEnsures,
            InvariantId::NextPrevReciprocity        => InvariantRelation::TemporarilyViolatesButEnsures,
            InvariantId::NoDanglingRefs             => InvariantRelation::Ensures,
            InvariantId::GenerationalFreshness      => InvariantRelation::Ensures,

            // Loop structure — insert a new edge into an existing loop
            InvariantId::FaceHasLoop                => InvariantRelation::Unrelated,
            InvariantId::LoopMinCardinality          => InvariantRelation::Ensures,
            InvariantId::NoDuplicateCoedges          => InvariantRelation::Ensures,
            InvariantId::FaceLoopMembership          => InvariantRelation::Ensures,
            InvariantId::VertexContinuity            => InvariantRelation::Ensures,
            InvariantId::EdgeEndpointsMatch          => InvariantRelation::Ensures,

            // Ownership — unchanged
            InvariantId::SingleLoopOwner             => InvariantRelation::Unrelated,
            InvariantId::NoOrphanHalfEdges           => InvariantRelation::Ensures,
            InvariantId::AcyclicContainment          => InvariantRelation::Unrelated,
            InvariantId::InnerOuterConsistency       => InvariantRelation::Unrelated,

            // Radial — new edge gets self-radial
            InvariantId::RadialCycleUniqueness       => InvariantRelation::Ensures,
            InvariantId::RadialNeighborConsistency   => InvariantRelation::Ensures,
            InvariantId::NoBrokenRadialSplices       => InvariantRelation::Ensures,

            // Shell — unchanged
            InvariantId::FaceAdjacencyConsistency    => InvariantRelation::Unrelated,
            InvariantId::NoBrokenFaceBoundary        => InvariantRelation::Unrelated,
            InvariantId::BoundaryEdgesLaminar        => InvariantRelation::Unrelated,

            // Vertex disk — new vertex gets 1 disk entry
            InvariantId::DiskEntriesAlive            => InvariantRelation::Ensures,
            InvariantId::DiskPartitionCorrect        => InvariantRelation::MayBreak,
            InvariantId::DiskClosure                 => InvariantRelation::Ensures,
            InvariantId::NoCrossDiskCoedges          => InvariantRelation::Unrelated,

            // Global
            InvariantId::PerComponentEuler           => InvariantRelation::Ensures,
            InvariantId::SideCarCoherence            => InvariantRelation::Ensures,
            InvariantId::IndexCoherence              => InvariantRelation::MayBreak,
        },
    };
    // ...
}
```

---

### Validator Registry (Second Compile-Time Enforcement)

```rust
fn validator_for(id: InvariantId) -> ValidatorEntry {
    match id {
        InvariantId::RadialReciprocity         => ValidatorEntry::cheap(radial_edge::validate_radial_rings),
        InvariantId::NextPrevReciprocity        => ValidatorEntry::cheap(loop_wiring::validate_prev_consistency),
        InvariantId::NoDanglingRefs             => ValidatorEntry::cheap(reference_integrity::validate_no_dangling_half_edge_refs),
        InvariantId::GenerationalFreshness      => ValidatorEntry::cheap(reference_integrity::validate_generational_id_freshness),
        InvariantId::FaceHasLoop                => ValidatorEntry::cheap(reference_integrity::validate_face_has_at_least_one_loop),
        InvariantId::LoopMinCardinality          => ValidatorEntry::cheap(loop_wiring::validate_loop_minimum_cardinality),
        InvariantId::NoDuplicateCoedges          => ValidatorEntry::cheap(loop_wiring::validate_no_duplicate_coedges_in_loop),
        InvariantId::FaceLoopMembership          => ValidatorEntry::medium(loop_wiring::validate_face_loop_membership_complete),
        InvariantId::VertexContinuity            => ValidatorEntry::cheap(loop_wiring::validate_vertex_continuity),
        InvariantId::EdgeEndpointsMatch          => ValidatorEntry::medium(loop_wiring::validate_edge_endpoints_match_loop_vertices),
        InvariantId::SingleLoopOwner             => ValidatorEntry::medium(reference_integrity::validate_single_owner_per_loop),
        InvariantId::NoOrphanHalfEdges           => ValidatorEntry::medium(reference_integrity::validate_no_orphan_half_edges),
        InvariantId::AcyclicContainment          => ValidatorEntry::medium(reference_integrity::validate_acyclic_containment),
        InvariantId::InnerOuterConsistency       => ValidatorEntry::medium(reference_integrity::validate_inner_outer_loop_consistency),
        InvariantId::RadialCycleUniqueness       => ValidatorEntry::cheap(radial_edge::validate_radial_cycle_uniqueness),
        InvariantId::RadialNeighborConsistency   => ValidatorEntry::expensive(radial_edge::validate_radial_neighbor_consistency),
        InvariantId::NoBrokenRadialSplices       => ValidatorEntry::expensive(radial_edge::validate_no_broken_radial_splices),
        InvariantId::FaceAdjacencyConsistency    => ValidatorEntry::expensive(shell_closure::validate_face_adjacency_consistency),
        InvariantId::NoBrokenFaceBoundary        => ValidatorEntry::expensive(shell_closure::validate_no_broken_face_boundary),
        InvariantId::BoundaryEdgesLaminar        => ValidatorEntry::expensive(shell_closure::validate_boundary_edges_laminar_only),
        InvariantId::DiskEntriesAlive            => ValidatorEntry::cheap(vertex_disk::validate_vertex_outgoing),
        InvariantId::DiskPartitionCorrect        => ValidatorEntry::expensive(vertex_disk::validate_vertex_disk_partition),
        InvariantId::DiskClosure                 => ValidatorEntry::expensive(vertex_disk::validate_disk_closure),
        InvariantId::NoCrossDiskCoedges          => ValidatorEntry::expensive(vertex_disk::validate_no_cross_disk_coedges),
        InvariantId::PerComponentEuler           => ValidatorEntry::expensive(euler_genus::validate_per_component_euler),
        InvariantId::SideCarCoherence            => ValidatorEntry::cheap(cache_index::validate_sidecar_coherence),
        InvariantId::IndexCoherence              => ValidatorEntry::cheap(cache_index::validate_index_coherence),
    }
}
```

Exhaustive match — adding an `InvariantId` without a validator = **compile error**.

---

### Invariant Groups (Feature-Level Composition)

Individual `InvariantId` variants are the atomic unit. `InvariantGroup` provides
named subsets for higher-level consumers — feature contracts, validation levels,
and human communication:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InvariantGroup {
    /// Radial/next/prev reciprocity, dangling refs, generational freshness.
    PointerCoherence,
    /// Face-has-loop, loop cardinality, duplicate coedges, vertex continuity, edge endpoints.
    LoopIntegrity,
    /// Single loop owner, no orphan HEs, acyclic containment, inner/outer consistency.
    Ownership,
    /// Radial cycle uniqueness, radial neighbor consistency, no broken splices.
    RadialEdge,
    /// Face adjacency symmetry, closed boundaries, laminar boundary edges.
    ShellClosure,
    /// Disk entries alive, partition correct, closure, no cross-disk coedges.
    VertexDisk,
    /// Per-component Euler formula.
    EulerFormula,
    /// Side-car and index coherence.
    CacheCoherence,
    // ── Future geometric groups (resolved at forge-kernel layer) ──
    /// G1 continuity at shared edges.
    G1Continuity,
    /// No self-intersection in the solid.
    NoSelfIntersection,
    /// No faces below the sliver area threshold.
    NoSliverFaces,
}

impl InvariantGroup {
    /// Resolve this group to its constituent `InvariantId` variants.
    pub fn invariant_ids(&self) -> &[InvariantId] {
        match self {
            Self::PointerCoherence => &[
                InvariantId::RadialReciprocity,
                InvariantId::NextPrevReciprocity,
                InvariantId::NoDanglingRefs,
                InvariantId::GenerationalFreshness,
            ],
            Self::LoopIntegrity => &[
                InvariantId::FaceHasLoop,
                InvariantId::LoopMinCardinality,
                InvariantId::NoDuplicateCoedges,
                InvariantId::FaceLoopMembership,
                InvariantId::VertexContinuity,
                InvariantId::EdgeEndpointsMatch,
            ],
            // ... every group exhaustively listed
        }
    }
}
```

**Replaces `InvariantKind`:** The existing `InvariantKind` enum in `forge-kernel`
(`ManifoldEdges`, `G1Continuity`, `NoSelfIntersection`, `NoSliverFaces`) will
migrate to `InvariantGroup` after M0 lands. This unifies the feature-level
and operator-level invariant systems under one registry.

`FeatureContract::post_invariants()` will return `&[InvariantGroup]` instead
of `&[InvariantKind]`. The pipeline resolves groups → `InvariantId`s → validators.

---

### Validator Cost Tiers

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ValidatorCost {
    /// O(n) single pass — always safe to run per-op.
    Cheap,
    /// O(n log n) or requires secondary data structures.
    Medium,
    /// O(n²) or global analysis (Euler, shell closure).
    Expensive,
}

pub struct ValidatorEntry {
    pub cost: ValidatorCost,
    pub check: fn(&TopologyArena) -> Result<(), KernelError>,
}
```

**Validation is always on** — the question is _how much_ runs, controlled by cost:

| Context                                  | Cheap | Medium | Expensive |
| :--------------------------------------- | :---: | :----: | :-------: |
| Per-op (`MayBreak` only) — always on     |  ✅   |   ❌   |    ❌     |
| Commit-time                              |  ✅   |   ✅   |    ✅     |
| Debug override (all validators, all ops) |  ✅   |   ✅   |    ✅     |
| Macro-op batch (booleans)                |  ❌   |   ❌   |    ❌     |

`Cheap` validators are O(n) pointer walks — the cost is negligible compared to
the operator itself. Running them after every op means corruption is caught
_at the source_, not 6 ops later at commit time. This is the "every decision
is traced" philosophy: invariant checks are structural proof events.

> **Macro-op suppression (Edge Case #3):** During massive compound operations like
> booleans (thousands of raw Euler ops in milliseconds), even `Cheap` per-op
> validators add unacceptable overhead. `DraftConfig::suppress_per_op_validation: bool`
> defers ALL per-op checks to a single commit-time sweep. The boolean pipeline
> sets this flag, runs its batch, then relies on commit-time full validation.

---

### Traced Validation Events

```rust
// In MutableDraft::execute(), after op completes:
for id in op.INVARIANT_CONTRACT.may_break() {
    let entry = validator_for(id);
    if entry.cost <= max_cost_for_context {
        let result = (entry.check)(&self.arena);
        tracing::info!(
            invariant = ?id,
            operator = Op::NAME,
            invocation = invocation_id,
            passed = result.is_ok(),
            "invariant_check"
        );
        result?;
    }
}
```

---

### Drift Prevention

1. **Debug-mode full override:** `DraftConfig::validate_all_invariants_per_op: bool`
   — runs ALL validators after every op regardless of contract. Catches
   misclassified `Unrelated`/`Preserves` that should be `MayBreak`.

2. **Fuzz test:** Random operator sequences with periodic full validation. If full
   validation catches something the contract-driven validation missed, the
   operator's contract is wrong.

3. **CI gate:** `#[test] fn all_invariants_have_validators()` iterates
   `InvariantId::ALL` and calls `validator_for()` for each.

---

### Rollback Safety (Edge Case #4)

> **Problem:** If an operator declares `TemporarilyViolatesButEnsures` but panics
> or returns `Err` mid-execution, the draft is left in a corrupted state with the
> invariant still violated.
>
> **Guarantee:** `MutableDraft::execute()` already poisons the draft on any error
> (`self.poisoned = true`). A poisoned draft rejects all subsequent `execute()`
> and `commit()` calls, forcing the caller to drop it (implicit rollback). The
> corrupted arena is never committed to a `TopologyState`.
>
> For panics: `MutableDraft` does not implement `Clone`. On panic unwind, the
> draft is dropped and the arena is reclaimed. The original `TopologyState`
> (behind an `Arc`) is untouched.

---

### File Manifest

| File                                                | Change                                                                                           |
| :-------------------------------------------------- | :----------------------------------------------------------------------------------------------- |
| [NEW] `validators/invariant_id.rs`                  | `InvariantId`, `InvariantRelation`, `InvariantContract`, `ValidatorEntry`, `ValidatorCost`       |
| [NEW] `validators/invariant_group.rs`               | `InvariantGroup` enum with `invariant_ids()` resolution                                          |
| [MODIFY] `validators/structural.rs`                 | Replace hard-coded dispatch with `validator_for()` exhaustive match                              |
| [MODIFY] `operations/operator.rs`                   | Add `const INVARIANT_CONTRACT` to `TopoOperator`, delete `validate_halfedge_reciprocity()`       |
| [MODIFY] `transactions/logic/mutable_draft.rs`      | Wire contract-driven post-op validation + traced events into `execute()`                         |
| [MODIFY] `transactions/data/draft_configuration.rs` | Replace `per_op_validation` with `validate_all_invariants_per_op` + `suppress_per_op_validation` |
| [MODIFY] Every `TopoOperator` impl (~43)            | Add exhaustive-match `INVARIANT_CONTRACT` const                                                  |

---

## Milestone 1: Entity Struct Stabilization

### Goal

Strip metadata off entity structs. After this, structs contain only connectivity
pointers and are frozen forever.

### Storage Design

IDs are `(index: u32, generation: u32)` — index is a direct slot position.
Side-car data uses **slot-parallel vectors**, same length and indices as entity
slot vectors:

```rust
// In TopologyArena, parallel to half_edge_slots:
bridge_flags: BitVec,                       // 1 bit per HE slot
coedge_data: Vec<Option<CoedgeInfo>>,       // parallel to half_edge_slots

// Parallel to edge_slots:
edge_curves: Vec<Option<CurveRef>>,

// Parallel to vertex_slots:
vertex_provenance: Vec<Option<[usize; 3]>>,
```

**Growth rule:** `insert_slot` grows side-car vectors in lockstep with entity vectors.

> **Slot Recycling Safety (Edge Case #2):** When a slot is recycled
> (generation incremented), the side-car entry at that index MUST be zeroed
> before the new entity claims it. For `Option`-wrapped side-cars this is
> automatic (`None`). For any future side-car using raw `u8`/`f64` without
> `Option`, the insertion path must explicitly zero the slot. Enforce this
> with a `debug_assert!` in `claim_recycled_slot()` that checks the side-car
> is in its default state.

### New Type

```rust
/// Bundled coedge metadata (direction + geometry ref).
pub struct CoedgeInfo {
    pub coedge_ref: CoedgeRef,
    pub direction: bool,
}
```

### Stabilized Entity Structs (Final Form)

```rust
// 6 connectivity pointers. Nothing else. Ever.
pub struct HalfEdgeData {
    radial_next: HalfEdgeId,
    next: HalfEdgeId,
    prev: HalfEdgeId,
    face: FaceId,
    origin: VertexId,
    edge: EdgeId,
}

// 1 connectivity pointer + NMT side-car flag (Milestone 2).
pub struct VertexData {
    primary_disk: HalfEdgeId,
}

// 1 connectivity pointer. Nothing else. Ever.
pub struct EdgeData {
    half_edge: HalfEdgeId,
}
```

### Cleanup Ownership: `insert_remove.rs`

Single point of authority. Every entity removal cleans side-car entries:

```rust
// In remove_half_edge():
self.bridge_flags.set(index, false);
self.coedge_data[index] = None;

// In remove_edge():
self.edge_curves[index] = None;

// In remove_vertex():
self.vertex_provenance[index] = None;
self.nmt_extra_disks.remove(&vertex_id);  // NMT side-car (Milestone 2)
```

### Ergonomics: Entity View Wrappers

```rust
pub struct HalfEdgeView<'a> {
    id: HalfEdgeId,
    data: &'a HalfEdgeData,
    arena: &'a TopologyArena,
}

impl<'a> HalfEdgeView<'a> {
    // Connectivity (from HalfEdgeData)
    pub fn next(&self) -> HalfEdgeId { self.data.next() }
    pub fn prev(&self) -> HalfEdgeId { self.data.prev() }
    pub fn origin(&self) -> VertexId { self.data.origin() }

    // Side-car metadata (from arena maps)
    pub fn is_bridge(&self) -> bool { self.arena.is_bridge(self.id) }
    pub fn coedge_info(&self) -> Option<&CoedgeInfo> { self.arena.coedge_info(self.id) }
}
```

### Serialization Migration

1. Transitional deserialization struct with `#[serde(default)]` on old + new fields
2. `post_deserialize_migrate()` on `TopologyArena`: reads old inline fields →
   populates side-car vectors → clears old fields
3. Next serialize writes new format only
4. `is_bridge` is the only field with real data to preserve (others are
   `None`/default in Phase 1–2)

### File Manifest

| File                                              | Change                                                                                          |
| :------------------------------------------------ | :---------------------------------------------------------------------------------------------- |
| [MODIFY] `b_rep/data/mesh/half_edge.rs`           | Remove `is_bridge`, `coedge`, `direction`. Simplify `new()` to 6 args.                          |
| [MODIFY] `b_rep/data/mesh/edge.rs`                | Remove `curve` field + accessors.                                                               |
| [MODIFY] `b_rep/data/mesh/vertex.rs`              | Remove `provenance` field + accessors.                                                          |
| [MODIFY] `b_rep/data/storage/arena.rs`            | Add 4 side-car vectors. Grow in lockstep. Add typed accessors.                                  |
| [NEW] `b_rep/data/mesh/coedge_info.rs`            | `CoedgeInfo` struct.                                                                            |
| [MODIFY] `b_rep/logic/graph_ops/insert_remove.rs` | Side-car cleanup on removal, growth on insertion. Zeroing protocol for recycled slots.          |
| [MODIFY] `b_rep/facade.rs`                        | Export `CoedgeInfo`. Add `is_bridge()`, `coedge_info()`, `edge_curve()`, `vertex_provenance()`. |
| [NEW] `b_rep/logic/views.rs`                      | `HalfEdgeView`, `VertexView`, `EdgeView` wrappers.                                              |
| [MODIFY] `operations/algorithms/bridge_edge.rs`   | `he.set_bridge(true)` → `arena.set_bridge(he_id, true)`                                         |
| [MODIFY] `forge-spatial/operations/continuity.rs` | `he.is_bridge()` → `arena.is_bridge(he_id)`                                                     |
| [MODIFY] `operations/lifecycle/body_ops.rs`       | `vd.provenance()` → `arena.vertex_provenance(vid)`                                              |

---

## Milestone 2: NMT Disk Entries

### Goal

Support non-manifold pinch-point vertices where multiple disconnected disk cycles
meet at one geometric vertex.

### Design: Lean Struct + NMT Side-Car

> **Edge Case #1 — SmallVec Memory Trap:** A `SmallVec<[HalfEdgeId; 1]>` bloats
> `VertexData` from ~4 bytes to ~32 bytes (ptr + cap + len + tag). Since 99% of
> CAD vertices are manifold, this penalizes the common case for rare NMT geometry.
>
> **Solution:** Keep `VertexData` minimal with a single `primary_disk: HalfEdgeId`.
> NMT vertices store extra disk entries in a **sparse side-car map**, consistent
> with the Milestone 1 pattern.

```rust
// VertexData stays tiny (4 bytes + padding)
pub struct VertexData {
    primary_disk: HalfEdgeId,
}

// Side-car on TopologyArena (sparse — only NMT vertices appear)
nmt_extra_disks: HashMap<VertexId, SmallVec<[HalfEdgeId; 2]>>,
// Companion flag for O(1) "is this vertex NMT?" checks
vertex_is_nmt: BitVec,    // parallel to vertex_slots
```

### Vertex Disk API (on `TopologyArena`)

```rust
impl TopologyArena {
    /// Primary disk entry (always present).
    pub fn primary_disk_entry(&self, v: VertexId) -> Result<HalfEdgeId, KernelError> {
        Ok(self.get_vertex(v)?.primary_disk())
    }

    /// All disk entries: primary + any NMT extras.
    pub fn disk_entries(&self, v: VertexId) -> Result<SmallVec<[HalfEdgeId; 4]>, KernelError> {
        let primary = self.get_vertex(v)?.primary_disk();
        let mut entries = smallvec![primary];
        if let Some(extras) = self.nmt_extra_disks.get(&v) {
            entries.extend_from_slice(extras);
        }
        Ok(entries)
    }

    /// Number of disks (1 = manifold, 2+ = NMT).
    pub fn disk_count(&self, v: VertexId) -> usize {
        1 + self.nmt_extra_disks.get(&v).map_or(0, |e| e.len())
    }

    /// Is this a non-manifold pinch-point vertex?
    pub fn is_vertex_nmt(&self, v: VertexId) -> bool {
        self.vertex_is_nmt[v.index() as usize]
    }

    /// Add a disk entry (creates/extends NMT pinch point).
    pub fn add_disk_entry(&mut self, v: VertexId, he: HalfEdgeId) {
        self.nmt_extra_disks.entry(v).or_default().push(he);
        self.vertex_is_nmt.set(v.index() as usize, true);
    }

    /// Remove a disk entry. Returns false if not found.
    pub fn remove_disk_entry(&mut self, v: VertexId, he: HalfEdgeId) -> bool { ... }

    /// Replace a disk entry (swap deleted HE for survivor).
    pub fn replace_disk_entry(&mut self, v: VertexId, old: HalfEdgeId, new: HalfEdgeId) -> bool { ... }

    /// Set the primary disk entry.
    pub fn set_primary_disk_entry(&mut self, v: VertexId, he: HalfEdgeId) -> Result<(), KernelError> {
        self.get_vertex_mut(v)?.set_primary_disk(he);
        Ok(())
    }
}
```

### Strict Disk Entries Invariant

Each entry is the **representative halfedge** of one connected component
in the vertex's incident halfedge set, discovered via the disk walk
(`radial_next → prev` cycle).

- Length 1 = manifold vertex
- Length 2+ = NMT pinch point
- All entries alive, unique, one per connected disk

### Operator Renames

`.outgoing()` → `.primary_disk()`, `.set_outgoing(x)` → `.set_primary_disk(x)`.

| File                                                   | Writes | Reads |
| :----------------------------------------------------- | :----: | :---: |
| `entity_lifecycle/make_vertex_face.rs`                 |   1    |   0   |
| `entity_lifecycle/make_shell_face.rs`                  |   1    |   0   |
| `entity_lifecycle/make_face_vertex.rs`                 |   1    |   0   |
| `entity_lifecycle/make_edge_vertex.rs`                 |   1    |   0   |
| `entity_lifecycle/split_edge.rs`                       |   1    |   0   |
| `entity_lifecycle/kill_edge_vertex.rs`                 |   1    |   0   |
| `entity_lifecycle/kill_vertex_edge.rs`                 |   2    |   1   |
| `boundary_editing/join_faces.rs`                       |   2    |   2   |
| `boundary_editing/join_faces_nmt.rs`                   |   1    |   1   |
| `boundary_editing/make_face_from_vertices.rs`          |   1    |   1   |
| `boundary_editing/make_face_in_shell_from_vertices.rs` |   1    |   1   |
| `boundary_editing/make_loop_in_face_from_vertices.rs`  |   1    |   1   |
| `algorithms/simplify/cleanup.rs`                       |   2    |   2   |
| `lifecycle/body_ops.rs`                                |   1    |   1   |

### Validator Updates

| File                                                       | Change                          |
| :--------------------------------------------------------- | :------------------------------ |
| `validators/vertex_disk/vertex_outgoing.rs`                | `.primary_disk()`               |
| `validators/vertex_disk/disk_partition.rs`                 | Iterate `arena.disk_entries(v)` |
| `validators/euler_genus/per_component_euler.rs`            | `.primary_disk()`               |
| `validators/reference_integrity/bidirectional_links.rs`    | `.primary_disk()`               |
| `validators/reference_integrity/generational_freshness.rs` | Validate all disk entries       |
| `validators/validate.rs`                                   | Debug print + poison tests      |

### Test Updates (mechanical renames)

| File                                       | Sites |
| :----------------------------------------- | :---: |
| `testing.rs`                               |   6   |
| `tests/diagnostic.rs`                      |   1   |
| `tests/brutality.rs`                       |   1   |
| `tests/topology_stress.rs`                 |   2   |
| `operations/tests/join_faces_tests.rs`     |   3   |
| `operations/tests/join_faces_nmt_tests.rs` |  23   |
| `operations/tests/mekl_keml_tests.rs`      |   3   |
| `operations/tests/brutality_tests.rs`      |   1   |
| `operations/tests/invariant_checker.rs`    |   1   |

### Cross-Crate (mechanical renames)

| File                                      | Sites |
| :---------------------------------------- | :---: |
| `forge-kernel/.../ember/quantize.rs`      |   1   |
| `forge-kernel/.../ember/mesh.rs`          |   2   |
| `forge-kernel/.../brutality/integrity.rs` |   2   |
| `forge-kernel/.../pv_p0_5_tests.rs`       |   2   |
| `forge-spatial/.../simplify.rs`           |   1   |

---

## Milestone 3: NMT Queries + Validator Hardening

### New Query File: `queries/vertex_disks.rs`

```rust
/// Partition all outgoing half-edges at a vertex into connected disk cycles.
pub fn compute_vertex_disks(
    arena: &TopologyArena,
    vertex: VertexId,
) -> Result<Vec<Vec<HalfEdgeId>>, KernelError> { ... }

/// Is this vertex manifold (exactly 1 disk cycle)?
pub fn is_vertex_manifold(
    arena: &TopologyArena,
    vertex: VertexId,
) -> Result<bool, KernelError> { ... }

/// Slow-but-correct recomputation of disk entries from scratch.
/// Collects all HEs at vertex via vertex_halfedges index, partitions into
/// connected components via disk walks. Validators cross-check stored
/// disk entries against this canonical computation.
pub fn rebuild_disk_entries(
    arena: &TopologyArena,
    vertex: VertexId,
) -> Result<SmallVec<[HalfEdgeId; 1]>, KernelError> { ... }
```

### Operator NMT Logic

| File                                              | Change                                                                               |
| :------------------------------------------------ | :----------------------------------------------------------------------------------- |
| `operations/entity_lifecycle/kill_edge_vertex.rs` | After merging vertices, transfer `nmt_extra_disks` from killed vertex onto survivor. |

### Validator Hardening

| File                                                       | Change                                                              |
| :--------------------------------------------------------- | :------------------------------------------------------------------ |
| `validators/vertex_disk/disk_partition.rs`                 | Cross-check `arena.disk_count(v)` vs `rebuild_disk_entries()`       |
| `validators/vertex_disk/vertex_outgoing.rs`                | Per-disk-entry reachability: each entry reaches all HEs in its disk |
| `validators/reference_integrity/generational_freshness.rs` | Validate all disk entries are alive                                 |

---

## Execution Order

1. **Milestone 0** — `InvariantId`, `InvariantContract`, wire into `TopoOperator` +
   `execute()` runner. Add contracts to all operators.
2. **Milestone 1** — Strip metadata, add side-car maps, entity views.
3. **Milestone 2** — `outgoing` → `primary_disk` + NMT side-car, rename across ~38 files.
4. **Milestone 3** — NMT computed queries, validator hardening.

---

## Verification

```bash
cargo test --workspace
```

**Post-completion checklist:**

- [ ] Adding `InvariantId` variant breaks every operator (compile error)
- [ ] Adding `InvariantId` variant breaks `validator_for()` (compile error)
- [ ] `DraftConfig::validate_all_invariants_per_op` catches misclassified contracts
- [ ] `DraftConfig::suppress_per_op_validation` eliminates overhead for boolean batches
- [ ] Poisoned draft rejects `execute()`/`commit()` after `TemporarilyViolatesButEnsures` failure
- [ ] Entity structs contain only connectivity pointers
- [ ] Side-car data cleaned on entity removal in `insert_remove.rs`
- [ ] Recycled slots zeroed before reuse (`debug_assert!` in claim path)
- [ ] `rebuild_disk_entries()` matches stored values in validator
- [ ] Traced invariant check events emitted in `execute()`

---

## Deferred Refactors

Items deferred during M0–M1 that must be swept up after all milestones complete.

### From M0: Invariant Contract System

| #   | Item                                                                        | Context                                                                                                                                                                                                                                                                    |
| :-- | :-------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 1   | **Replace `conservative_contract!()` with explicit per-operator contracts** | All ~45 operators currently use `conservative_contract!()` which maps every `InvariantId` to `MayBreak`. Each operator should declare its actual relationship (Requires / Ensures / Preserves / MayBreak) to each invariant. This is a large but mechanical audit.         |
| 2   | **Refactor `structural.rs` to use `validator_for()` dispatch**              | `structural.rs` still calls compound validators directly (e.g. `validate_halfedge_reciprocity`). Once all compound validators are decomposed into individual `InvariantId` entries, refactor to use `validator_for()` with cost-tier filtering based on `ValidationLevel`. |
| 3   | **Decompose compound validators into individual `InvariantId` entries**     | Several existing validators (reciprocity, loop wiring, radial edge) bundle multiple checks. Each should map 1:1 to an `InvariantId` variant for granular contract analysis.                                                                                                |

### From M1: Entity Struct Stabilization

| #   | Item                                                      | Context                                                                                                                                                                                                                                           |
| :-- | :-------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| 4   | **Add `debug_assert!` for side-car / slot length parity** | Side-car vectors grow in lockstep with entity slot vectors via `insert_remove.rs` hooks, but there's no runtime assertion that `bridge_flags.len() == half_edge_slots.len()`. A `debug_assert!` at commit time would catch any future drift.      |
| 5   | **Migrate existing callers to use entity views**          | `bridge_edge.rs` and `continuity.rs` currently call `arena.is_bridge(id)` directly. Future callers should prefer `arena.view_half_edge(id)?.is_bridge()` to establish the view-first access pattern. Existing callers can be migrated as a sweep. |
