# Forge Topology Crate Handoff Documentation

This document is a source-driven handoff for `crates/forge-topo`.
It is intended to let a new engineer understand architecture, mutation lifecycle, invariants, major APIs, and extension points without reverse engineering from scratch.

## Scope and Current Size

- Crate: `forge-topo`
- Rust source files: `232`
- Approximate Rust LOC: `33,323`
- Root module: `crates/forge-topo/src/lib.rs`

## Important Repo Note

- `AGENTS.md` says "read llms.txt" but no `llms.txt` exists in this workspace.
- I proceeded from source of `forge-topo` directly.

## 1. What This Crate Owns

`forge-topo` owns topological state and topological mutation.

It does **not** own geometric numerics in this layer. Geometry references are opaque handles (`CurveRef`, `SurfaceRef`, `CoedgeRef`) stored as sidecars.

Primary responsibilities:

- Half-edge B-Rep connectivity storage (`TopologyArena`)
- Containment hierarchy (Body -> Lump -> Region -> Shell -> Face/loops/halfedges/vertices/edges)
- Topological operators (Euler primitives + composite algorithms)
- Transactional mutation (`TopologyState` -> `MutableDraft` -> `commit`)
- Validation and invariant policy
- Topology change diffing and structural hashing
- Lineage/provenance, replay logging, persistent naming substrate

## 2. Top-Level Module Map

`lib.rs` exports:

- Core subsystems:
  - `b_rep`
  - `semantic_attributes`
  - `provenance`
  - `persistent_naming`
  - `change_detection`
  - `transactions`
- Infrastructure:
  - `handles`
  - `prelude`
- Operational surface:
  - `operations`
  - `queries`
  - `validators`

Also re-exports typed IDs and convenience modules (`validate`, `algorithms`, `operator`, etc.).

## 3. Core Data Model

### 3.1 Typed Generational IDs

Defined in `handles.rs` via macro-generated types:

- Mesh/container IDs: `FaceId`, `HalfEdgeId`, `VertexId`, `LoopId`, `EdgeId`, `ShellId`, `RegionId`, `LumpId`, `BodyId`
- Geometry refs: `CurveRef`, `SurfaceRef`, `CoedgeRef`

Properties:

- `(index, generation)` stale-handle protection
- `DANGLING` sentinel for staged wiring
- Serialization as `"index:generation"`

### 3.2 Mesh Entities

Defined under `b_rep/data/mesh`:

- `HalfEdgeData`: `radial_next`, `next`, `prev`, `face`, `origin`, `edge`
- `EdgeData`: representative halfedge
- `VertexData`: `primary_disk` representative
- `LoopData`: representative halfedge + owning face
- `FaceData`: outer loop, inner loops, shell, optional `SurfaceRef`
- `CoedgeInfo`: optional halfedge sidecar with UV coedge + direction sense

### 3.3 Containment Entities

Defined under `b_rep/data/containment`:

- `BodyData`: owns lumps
- `LumpData`: owns regions + parent body
- `RegionData`: outer shell + inner shells + parent lump
- `ShellData`: representative face + kind + parent region

`ShellKind`:

- `Solid(ShellOrientation)`
- `Sheet`
- `Wire`

This `ShellKind` metadata is used by runtime validation policy and debug consistency checks.

## 4. Storage Architecture (`TopologyArena`)

`TopologyArena` (in `b_rep/data/storage/arena.rs`) is the central storage object.

### 4.1 Slot + Free-list Design

- One slot vector per entity type (`Vec<Slot<T>>`)
- One free-list head per type
- Active counters tracked in O(1)
- Slot generation bump on removal

### 4.2 Sidecar Design

Connectivity structs stay lean. Metadata is in slot-parallel sidecars:

- Halfedge sidecars: `bridge_flags`, `coedge_data`
- Edge sidecars: `edge_curves`, `edge_shells`
- Vertex sidecars: `vertex_provenance`, `nmt_extra_disks`, `vertex_is_nmt`
- Shell sidecar: `shell_entry_edges`

Debug builds enforce sidecar parity (`assert_sidecar_parity`).

### 4.3 Reverse Indexes

Derived O(1) indexes (serde-skipped, rebuilt on deserialize):

- `shell_faces: ShellId -> [FaceId]`
- `face_halfedges: FaceId -> [HalfEdgeId]`
- `vertex_halfedges: VertexId -> [HalfEdgeId]`

Maintained by insert/remove/reassign hooks.

## 5. Mutation Lifecycle

### 5.1 Snapshot -> Draft -> Commit

- Immutable state: `TopologyState` (`Arc<TopologyArena>`)
- Start transaction: `TopologyState::into_mutation[_with]()` -> `MutableDraft`
- Mutate only through draft
- Finalize via `draft.commit()`

If draft dropped without commit, state rollback is implicit.

### 5.2 Execute Runner (Single Choke Point)

All operators must run through `MutableDraft::execute` (`transactions/logic/operation_runner.rs`).

Runner performs:

- operation signature/invocation assignment
- replay start logging
- mutation journal reset
- operator dispatch
- draft poisoning on failure
- topology version bump
- optional per-op structural hashing
- declared Euler delta vs actual delta verification
- debug journal-vs-arena count consistency asserts
- contract-driven per-op invariant execution
- metrics + lineage delta assembly

### 5.3 Mutation Journal

`MutationJournal` auto-records all `insert_*` and `remove_*` via draft proxy hooks.

Used for:

- deletion auto-stamping in lineage store
- gross count metrics
- cross-check against actual arena deltas

## 6. Operator System

`TopoOperator` trait (`operations/operator.rs`) requires:

- `type Output`
- `execute(&self, draft, recorder) -> ExecutionResult<Output>`
- `const NAME`
- `const INVARIANT_CONTRACT`
- optional `semantic_summary()` override

`ExecutionResult` carries both value and declared `EulerDelta`.

### 6.1 Contract Profiles

Centralized in `validators/contract_registry.rs`:

- `CONTAINER_LIFECYCLE`
- `FULL_TOPO_WIRING`
- `RADIAL_SPLICE`
- `ISOLATED_VERTEX`

These gate which invariant groups run per-op.

### 6.2 Implemented Operator Families

#### Entity Lifecycle (`operations/entity_lifecycle`)

Core Euler-style mesh edits:

- `make_vertex_face`, `kill_vertex_face`
- `make_edge_vertex`, `kill_edge_vertex`
- `split_edge`, `kill_vertex_edge`
- `make_edge_face`, `join_faces` (in boundary dir)
- `make_face_vertex`, `kill_face_vertex`
- `make_shell_face`, `kill_shell_face`
- `make_isolated_vertex`

#### Boundary Editing (`operations/boundary_editing`)

Loop/face boundary surgery:

- `make_edge_kill_loop`, `kill_edge_make_loop`
- `make_face_kill_ring_hole`, `kill_face_make_ring_hole`
- `join_faces`, `join_faces_nmt`
- `make_loop_in_face_from_vertices`
- `make_face_from_vertices`
- `make_face_in_shell_from_vertices`

#### Non-Manifold (`operations/non_manifold`)

Radial splicing:

- `sew_edge`
- `unsew_edge`

#### Container Lifecycle (`operations/lifecycle`)

Body/lump/shell hierarchy operations:

- solids: `make_solid`, `destroy_body`
- lumps: `make_lump_region`, `destroy_lump`, `rehome_lump`, `extract_lump`, `split_lump`, `merge_lumps`
- shells: `make_empty_shell`, `destroy_shell`, `rehome_shell`, `extract_shell`, `split_shell`, `merge_shells`, `promote_shell`, `demote_shell`
- bodies: `split_body`, `merge_bodies`, `clone_body`

### 6.3 Composite Algorithms (`operations/algorithms`)

Implemented:

- traversal utilities: `bfs`, `components`
- shell extraction: `extract_shell`
- region extraction helpers
- editing algorithms: `bridge_edge`, `flip_edge`, `triangulate_face`
- simplification: `cleanup_degenerate_topology`

Other operation categories exist as stubs/modules (boolean, construction, transform, etc.) with catalog comments but limited/no implementation here.

## 7. Query Surface

`queries/` contains read-only helpers and iterators.

Main modules:

- Traversal iterators (`traverse.rs`):
  - `LoopEdgeIterator`, `FaceEdgeIterator`, `FaceAllEdgesIterator`, `VertexRingIterator`, `RadialEdgeIterator`
  - helpers like `face_loops`, `edge_faces`, `edge_endpoint_ids`, `radial_valence`, `is_boundary_edge`
- Classification (`classification.rs`): manifold/non-manifold tests, adjacency, loop role checks
- Hierarchy (`hierarchy.rs`): region/shell/face relationships
- Polygon helpers (`polygon.rs`): loop vertices, adjacency pairs
- Radial usage (`radial.rs`): ring snapshots and grouping by face
- Edge map utilities (`edge_map.rs`)
- Vertex disk computation (`vertex_disks.rs`)

## 8. Validation System

### 8.1 Validation Levels

`ValidationLevel`:

- `None`
- `Minimal` (cheap invariants)
- `Intermediate` (up to medium)
- `Full` (all)

Commit-time validation runs via `validators::structural::validate_topology`.

### 8.2 Invariant Registry

`validator_for(InvariantId)` maps invariants to check fns + cost (`Cheap/Medium/Expensive`).

Invariant groups include:

- Pointer coherence
- Loop integrity
- Ownership
- Radial edge
- Vertex disk
- Shell closure
- Euler formula
- Cache coherence
- Geometry group placeholders (mostly no-op in this crate)

### 8.3 Runtime Policy

`GroupPolicyRuntime` resolves run masks based on topology context (`Solid/Sheet/Wire`, closure, certification stage) and per-checkpoint cost ceilings.

Per-op execution uses this policy plus operator contract to choose checks.

### 8.4 Implemented Structural Validators

Implemented directories with concrete checks:

- `reference_integrity/`
- `loop_wiring/`
- `radial_edge/`
- `vertex_disk/`
- `shell_closure/`
- `euler_genus/`
- `cache_index/`

Several other validator domains currently exist as documented placeholders (`degeneracy`, `parametric_binding`, `intersection_graph`, `numerical_predicate`, `determinism`, `persistent_naming`, `import_sanity`, `region_cellular`).

## 9. Provenance, Replay, and Naming

### 9.1 Lineage

Core types in `provenance/data/lineage`:

- `Lineage`
- `LineageEvent` (`EntityCreated`, `EntityDeleted`, `EntityModified`)
- `OpSignature`
- `LineageStore`

Supports root/derived/merged ancestry hashes and event log accumulation across epochs.

### 9.2 Replay

`ReplayLog` and `ReplayEntry` track operation signature, parameters blob, deterministic seed, pre/post hash, and semantic summary.

### 9.3 Re-identification Substrate

`reidentification_link.rs` defines V1 linkage record/index/query model for one-hop lineage-based re-identification.

### 9.4 Persistent Naming

`PersistentName` + `Selector` with resolver logic against `LineageStore`.

## 10. Change Detection and Hashing

- `change_detection::compute_diff(before, after, ...)` compares generations/versions by active index union
- `compute_arena_topology_hash` computes permutation-robust structural signature for topology-only change detection

## 11. Testing Strategy in Crate

Coverage appears strongest around operators and validator behavior.

Key suites:

- `operations/tests/*`: per-operator + integration + brutality + lineage
- `transactions/tests.rs`: commit lifecycle, replay, lineage persistence, re-identification index behavior
- `queries` and `validators` unit tests for targeted invariants
- top-level stress tests in `src/tests/*`

## 12. Known Technical Debt and Risks

1. `queries/vertex_disks.rs` contains direct `println!("DEBUG: ...")` output in hot logic.
2. Crate contains leftover artifact files in queries directory:
   - `vertex_disks.rs.orig`
   - `vertex_disks.rs.rej`
3. Many validator domains are spec-only stubs (documented but not yet implemented checks).
4. Some operations rely on specific topology assumptions called out in comments (examples: manifold assumptions in algorithms, edge flip geometric validity not enforced topologically).
5. Operator docs frequently mention invariants and exceptions; these are essential contracts and should be treated as API behavior.

## 13. How to Add or Modify Topology Behavior Safely

### 13.1 Add a New Operator

1. Add file under the right `operations/*` domain.
2. Implement `TopoOperator` with:
   - precise `NAME`
   - accurate `INVARIANT_CONTRACT`
   - accurate `declared_delta`
3. Use draft proxy methods (`insert_*`, `remove_*`) so mutation journaling stays correct.
4. Update/maintain reverse-index and sidecar consistency when mutating directly.
5. Add tests in `operations/tests/` covering normal + adversarial paths.

### 13.2 Modify Connectivity Rules

1. Update both operator logic and validator expectations.
2. Confirm affected invariant groups/costs in `invariant_id.rs` and contracts in `contract_registry.rs`.
3. Run and extend per-op + commit validation tests.

### 13.3 Add New Sidecar Metadata

1. Add vector to `TopologyArena`.
2. Add grow/clear hooks in sidecar accessor + insert/remove paths.
3. Include parity checks in `assert_sidecar_parity`.

### 13.4 Serialization Compatibility

- Mind `#[serde(default)]` for new persisted fields.
- Keep `#[serde(skip)]` derived indexes rebuildable (`rebuild_indexes`).

## 14. Practical Mental Model for New Engineers

Think of `forge-topo` as a strict transactional graph kernel:

- Entities are typed IDs into generational slots
- Mutations happen only in drafts
- Every operator declares intended topological delta
- Runner enforces declared-vs-actual counts and policy-driven invariants
- Commit is the global correctness gate
- Provenance and replay are first-class, not optional diagnostics

If you hold this model, most code organization decisions in this crate become predictable.

