# forge-topo QA Checklist

> Dense QA pass/fail reference for `forge-topo`. Per-module.
> **Stability key:** 🔒 = locked (done, won't change) · ⚙️ = will expand · ❄️ = irrelevant right now

---

## 1. Arena (`b_rep/data/storage/`) ⚙️

The generational-slot entity store. Expands when new entity fields appear (e.g. `CoedgeRef` direction was recent), but the _pattern_ is locked.

| #   | QA Gate                       | Pass Criteria                                                                                                                                                         |
| --- | ----------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 1   | **Generational slots**        | Every entity type uses `Slot<T>` with `occupy()`/`vacate()`. Generation bumps on reuse.                                                                               |
| 2   | **ABA safety**                | Stale handle access → immediate panic (thunderdome guarantee). No silent corruption.                                                                                  |
| 3   | **Schema completeness**       | All 9 entity types present: `FaceData`, `HalfEdgeData`, `EdgeData`, `VertexData`, `LoopData`, `ShellData`, `RegionData`, `LumpData`, `BodyData`.                      |
| 4   | **Encapsulation**             | All fields private. Access via `get_*()` / `set_*()`. No public struct fields.                                                                                        |
| 5   | **Lineage on all entities**   | `FaceData`, `HalfEdgeData`, `EdgeData`, `VertexData` carry `Option<Lineage>`. Constructors accept it.                                                                 |
| 6   | **Geometry-free (D0)**        | No `f64`, no positions, no planes in any `*Data` struct. Only opaque refs: `SurfaceRef`, `CurveRef`, `CoedgeRef`.                                                     |
| 7   | **Radial-edge structure**     | `HalfEdgeData` has `radial_next`, `next`, `prev`, `face`, `origin`, `edge`, `coedge`, `direction`.                                                                    |
| 8   | **Hierarchy chain**           | `FaceData.shell` → `ShellData.region` → `RegionData.lump` → `LumpData.body`. All bidirectional.                                                                       |
| 9   | **Serde round-trip**          | All `*Data` structs derive `Serialize + Deserialize`. Arena can be snapshot/restored.                                                                                 |
| 10  | **Inner loops**               | `FaceData.inner_loops: SmallVec<[LoopId; 2]>` with `add_inner_loop()` / `remove_inner_loop()`.                                                                        |
| 11  | **CRUD macro coverage**       | All 9 entity types registered via `define_entity_accessors!` + `define_plain_crud!` (or hooked equivalent) + `define_draft_proxies!`.                                 |
| 12  | **Hooked insert/remove**      | `Face` and `HalfEdge` use hand-written insert/remove that maintain reverse indexes (`shell_faces`, `face_halfedges`, `vertex_halfedges`) and grow side-car vectors.   |
| 13  | **`loop` keyword workaround** | `LoopId`/`LoopData` accessors and CRUD are spelled out explicitly (not macro-generated) due to the `loop` keyword conflict.                                           |
| 14  | **Side-car vectors**          | `bridge_flags`, `coedge_data` parallel to `half_edge_slots`; `edge_curves` parallel to `edge_slots`; `vertex_provenance`, `vertex_is_nmt` parallel to `vertex_slots`. |
| 15  | **Entity views**              | `view_half_edge()`, `view_edge()`, `view_vertex()` return read-only wrapper structs providing a unified access pattern over slots + side-cars.                        |

### What expands:

- New fields (e.g. future NMT metadata, smooth-crease tags for subdivision).
- `BodyData.lumps` → `SmallVec` if multi-lump bodies become common.
- Possible `ShellKind::Wire` additions for wire-body features.

---

## 2. Transactions (`transactions/`) 🔒

Transaction semantics. Done. Pattern is locked.

| #   | QA Gate                       | Pass Criteria                                                                                                                                                                |
| --- | ----------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 1   | **Immutability**              | `TopologyState` is `Clone` + read-only. No `&mut` methods.                                                                                                                   |
| 2   | **Draft entry**               | `into_mutation()` consumes `self`, returns `MutableDraft`. Zero-cost if unique `Arc` ref.                                                                                    |
| 3   | **Commit validates**          | `commit()` runs `assert_sidecar_parity()`, then `validate_topology()` → returns new `TopologyState` on success.                                                              |
| 4   | **Drop = rollback**           | Dropping `MutableDraft` without `commit()` silently discards changes.                                                                                                        |
| 5   | **Epoch versioning**          | `epoch`, `topology_version`, `geometry_version`, `topology_hash` all bump on commit.                                                                                         |
| 6   | **DraftConfig**               | `per_op_hashing`, `deterministic_seed`, `validation_level`, `per_op_validation`.                                                                                             |
| 7   | **commit_with_mode**          | Supports `TopologyMode::ManifoldStrict` (default) and `NmtIntermediate`.                                                                                                     |
| 8   | **Lineage events**            | `lineage_events()` returns chronological `Vec<LineageEvent>`.                                                                                                                |
| 9   | **ReidentificationLinkIndex** | Built on commit from lineage events. Queryable via `find_by_child_hash()` / `find_children_of()`.                                                                            |
| 10  | **Operation runner**          | `operation_runner.rs` handles `execute()` dispatch: invocation IDs, EulerDelta verification, invariant contract validation, lineage stamping, replay recording, and tracing. |

---

## 3. Provenance (`provenance/`) ⚙️

Lineage + replay infrastructure. Schemas are locked. _Queries_ will expand.

### 3a. Lineage (`provenance/lineage.rs`) 🔒

| #   | QA Gate              | Pass Criteria                                                                                                                                                                      |
| --- | -------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 1   | **Lineage struct**   | `ancestry_hash: u128`, `creation_op: OpSignature`, `origin_features: SmallVec<[u64; 2]>`, `parent_ancestry_hashes: SmallVec<[u128; 2]>`, `parent_linkage_mode: ParentLinkageMode`. |
| 2   | **Root creation**    | `Lineage::root(feature_id, op)` for entities created from scratch.                                                                                                                 |
| 3   | **Derivation**       | `Lineage::derive(parent, op)` — child hash incorporates parent's hash (Merkle-DAG).                                                                                                |
| 4   | **Merge**            | `Lineage::merge(a, b, op)` — for boolean-merged entities. Both parents recorded.                                                                                                   |
| 5   | **OpSignature**      | `(name: &'static str, invocation_id: u64)`. Heap-free cloning. Invocation ID assigned by `MutableDraft::execute()`.                                                                |
| 6   | **LineageEntityRef** | `(kind, index, generation)` — ABA-safe snapshot ref. `From<FaceId>` etc.                                                                                                           |
| 7   | **FNV-1a mixing**    | `fnv_mix_128(a, b)` for combining parent hashes. Deterministic.                                                                                                                    |

### 3b. Lineage Events, Journal, & LinkIndex (`provenance/`) ⚙️

| #   | QA Gate                        | Pass Criteria                                                                                                                                                                    |
| --- | ------------------------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 1   | **LineageEvent enum**          | Created / Deleted / Modified / Merged. All carry `LineageEntityRef`.                                                                                                             |
| 2   | **LineageStore**               | Draft-scoped `BTreeMap<EntityKey, Lineage>`. Merged into `TopologyState` on commit.                                                                                              |
| 3   | **ReidentificationLinkIndex**  | Built from events. V1 = one-hop only. `build()` sorts records deterministically.                                                                                                 |
| 4   | **ReidentificationLinkRecord** | child snapshot ref + ancestry hash + parent hashes + origin kind.                                                                                                                |
| 5   | **Query API**                  | `find_by_child_hash(hash, kind)`, `find_children_of(parent_hash, kind)`.                                                                                                         |
| 6   | **MutationJournal**            | Auto-tracks all created/destroyed entities per-operation. Evaluated by `MutableDraft::execute()` to automatically stamp deletions, removing the need for manual `bulk_stamping`. |
| 7   | **LineageRecorder**            | Enforces policy via `LineageMode` (`Root`, `Derived`, `Merged`). Generated monotonic ordinals guarantee unique `ancestry_hash` values for every stamp.                           |

### What expands:

- Multi-hop descendant queries (currently V1 = one-hop only).
- `ReidentificationMode::Ancestors` / `Hybrid` (currently `Descendants` only).
- `EntityOriginKind` gains more variants (currently `EulerOperator | GeometricIntersection | ConstraintSolver | Unknown`).
- `param_hash` on `OpSignature` for parametric rebuild (currently deferred).

### 3c. Replay (`provenance/replay.rs`) 🔒

| #   | QA Gate                | Pass Criteria                                                                       |
| --- | ---------------------- | ----------------------------------------------------------------------------------- |
| 1   | **ReplayEntry**        | `signature + parameters + seed + pre_hash + post_hash + decision_delta`.            |
| 2   | **ReplayLog**          | Append-only. `record()` during draft, `finalize_last()` sets post_hash.             |
| 3   | **Architecture check** | `verify_architecture(triple)` — catches FMA / debug-vs-release / platform mismatch. |
| 4   | **Determinism check**  | `verify_determinism(other)` — entry-by-entry sig + seed + hash comparison.          |
| 5   | **DecisionDelta**      | Optional per-entry — diff of decisions vs previous.                                 |

### What's irrelevant now:

- Replay _execution_ (re-running ops from log) — schema exists, executor is future.

---

## 4. Validators (`validators/`) ⚙️

Commit-time and per-op invariant validation. Uses the compile-enforced `InvariantId` → `validator_for()` dispatch system.

### 4a. Invariant System Architecture

| Module                    | Purpose                                                                                                                  |
| ------------------------- | ------------------------------------------------------------------------------------------------------------------------ |
| `invariant_id.rs`         | `InvariantId` enum (27 variants), `InvariantRelation`, `InvariantContract`, `ValidatorEntry`, `validator_for()` dispatch |
| `invariant_group.rs`      | `InvariantGroup` enum (8 categories). Every `InvariantId` maps to a group via `.group()`                                 |
| `contract_registry.rs`    | 4 named contract profiles: `FULL_TOPO_WIRING`, `CONTAINER_LIFECYCLE`, `RADIAL_SPLICE`, `ISOLATED_VERTEX`                 |
| `structural.rs`           | `validate_topology()` — loops over `InvariantId::ALL`, filters by `ValidatorCost` ≤ `ValidationLevel` ceiling            |
| `group_policy_runtime.rs` | `GroupPolicyRuntime` — resolves invariant group policies for per-op validation based on `TopologyContext`                |

### 4b. Adding a Validator — Compile-Enforced Pipeline

1. Add variant to `InvariantId` → compile error in `group()` until assigned to an `InvariantGroup`
2. Add `ValidatorEntry` in `validator_for()` → compile error until a check function and cost tier are provided
3. Update `InvariantId::ALL` → CI test `all_constant_covers_every_variant` catches omissions
4. `structural.rs::validate_topology()` **automatically** picks up the new validator — no manual wiring

### 4c. Cost-Tier Filtering (ValidationLevel → ValidatorCost)

| ValidationLevel | Max Cost    | What runs                                           |
| :-------------- | :---------- | :-------------------------------------------------- |
| `None`          | —           | Nothing                                             |
| `Minimal`       | `Cheap`     | Pointer coherence, basic loop/radial checks         |
| `Intermediate`  | `Medium`    | + ownership, membership, hierarchy                  |
| `Full`          | `Expensive` | Everything incl. Euler, shell closure, vertex disks |

### 4d. Per-Operator Contracts

Every `TopoOperator` declares `const INVARIANT_CONTRACT: InvariantContract`, typically using a named profile from `contract_registry.rs`. The profiles dispatch on `InvariantGroup`, not individual `InvariantId` — so adding a new invariant to an existing group is automatic for all operators.

### 4e. Side-Car Parity

`TopologyArena::assert_sidecar_parity()` (debug-only) runs at the top of `commit()`. Verifies 5 side-car vectors (`bridge_flags`, `coedge_data`, `edge_curves`, `vertex_provenance`, `vertex_is_nmt`) have grown in lockstep with their entity slot vectors.

### QA Gates

| #   | QA Gate                        | Pass Criteria                                                                                                             |
| --- | ------------------------------ | ------------------------------------------------------------------------------------------------------------------------- |
| 1   | **Exhaustive match**           | Adding an `InvariantId` variant without entries in `group()`, `validator_for()`, and `INVARIANT_CONTRACT` → compile error |
| 2   | **ALL covers every variant**   | `all_constant_covers_every_variant` CI test asserts `InvariantId::ALL.len() == 27`                                        |
| 3   | **EulerDelta check**           | `MutableDraft::execute()` compares declared `EulerDelta` vs actual arena count changes. Mismatch = error                  |
| 4   | **Per-op reciprocity**         | `validate_halfedge_reciprocity()` runs after every `execute()` when `per_op_validation` is on                             |
| 5   | **ShellKind structural match** | `verify_shell_kind_matches_structure()` runs as `debug_assert` at commit                                                  |
| 6   | **Side-car parity**            | `assert_sidecar_parity()` runs as `debug_assert` at commit                                                                |

### 4f. Other integrity modules

| Module                                | Status | Purpose                                                  |
| ------------------------------------- | ------ | -------------------------------------------------------- |
| `hashing.rs` (in `change_detection/`) | 🔒     | Structural hash (connectivity + lineage only, never f64) |

---

## 5. Naming (`persistent_naming/`) ⚙️

Persistent naming for parametric rebuild. Schemas locked. Selector variants will expand.

| #   | QA Gate                  | Pass Criteria                                                                     |
| --- | ------------------------ | --------------------------------------------------------------------------------- |
| 1   | **PersistentName**       | `(ancestry_hash: u128, kind: EntityKind, ordinal: u32)`. Ordinal = 0 for unsplit. |
| 2   | **assign_name**          | Reads `Lineage` from entity, captures ancestry_hash. Error if no lineage.         |
| 3   | **resolve_name**         | Returns `Vec<EntityKey>`: 0 = deleted, 1 = normal, 2+ = split.                    |
| 4   | **Selector algebra**     | `ByAncestry`, `ByFeature`, `ByOperation`, `And(Box, Box)`, `Or(Box, Box)`.        |
| 5   | **resolve_selector**     | Composable boolean evaluation against live arena.                                 |
| 6   | **Deterministic output** | `entity_key_sort_key()` ensures stable ordering. No HashMap-dependent iteration.  |

### What expands:

- More `Selector` variants (e.g. `ByGeometricProperty`, `ByProximity`).
- Ordinal assignment logic for complex splits (currently simple).
- Multi-generation resolution (fillet a face that was already filleted → re-resolve through history).

### What's irrelevant now:

- UI-layer selection (that's `forge-schema` / `forge-signal` territory, not topo).

---

## 6. Operations (`operations/`) ⚙️

Euler operators + composite algorithms. The operator _framework_ is locked. Individual operators expand.

### 6a. Operator Framework (`operations/operator.rs`) 🔒

| #   | QA Gate                              | Pass Criteria                                                                                                                                               |
| --- | ------------------------------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 1   | **TopoOperator trait**               | `execute()`, `const NAME`, `const INVARIANT_CONTRACT`, optional `semantic_summary()`.                                                                       |
| 2   | **INVARIANT_CONTRACT required**      | Every operator must declare a `const INVARIANT_CONTRACT: InvariantContract`. Use a named profile from `contract_registry.rs` or an exhaustive custom match. |
| 3   | **draft.execute() is the ONLY path** | Never call `op.execute()` directly. `MutableDraft::execute(op)` handles invocation IDs, delta verification, validation, lineage, and tracing.               |
| 4   | **EulerDelta verification**          | Declared V/E/F/L/S delta (from `ExecutionResult.declared_delta`) vs actual arena count change. Mismatch = `KernelError`.                                    |
| 5   | **Per-op validation**                | `DraftConfig.per_op_validation` → runs invariant validators filtered by the operator's contract after each op.                                              |
| 6   | **Lineage auto-stamp**               | `MutableDraft::execute()` assigns `OpSignature.invocation_id` monotonically per draft.                                                                      |
| 7   | **Replay auto-record**               | `MutableDraft::execute()` records `ReplayEntry` with pre/post hash.                                                                                         |

### 6b. Euler Operators ⚙️

Currently 44+ operators across `entity_lifecycle/`, `boundary_editing/`, `lifecycle/`, `non_manifold/`, `sheets_wires/`. Each must satisfy:

| #   | QA Gate                    | Per Operator                                                                                 |
| --- | -------------------------- | -------------------------------------------------------------------------------------------- |
| 1   | **Correct EulerDelta**     | Every code path declares its own `EulerDelta`. Self-loop paths may differ from normal paths. |
| 2   | **INVARIANT_CONTRACT set** | Must reference a `contract_registry` profile or define a custom exhaustive match.            |
| 3   | **Lineage stamped**        | All created entities get `Lineage::derive()` or `Lineage::root()` via `LineageRecorder`.     |
| 4   | **Radial ring correct**    | `SewEdge` / `UnsewEdge` correctly splice/unsplice radial rings.                              |
| 5   | **No raw f64**             | Zero floating-point comparisons. Pure connectivity.                                          |
| 6   | **Tests per op**           | Unit tests in `operations/tests/`.                                                           |

**Current operators:**
`MakeVertexFace`, `MakeEdgeFace`, `MakeEdgeVertex`, `MakeFaceFromVertices`, `MakeFaceInShellFromVertices`, `MakeLoopInFaceFromVertices`, `MakeShellFace`, `MakeEdgeKillLoop`, `SplitEdge`, `JoinFaces`, `JoinFacesNmt`, `KillEdgeVertex`, `KillEdgeMakeLoop`, `KillShellFace`, `KillVertexFace`, `SewEdge`, `UnsewEdge`, `BridgeEdge` (algorithm)

### What expands:

- **NMT Euler operators**: generalized `JoinFaces`/`KillEdge` accepting radial insertion parameters (when honeycombs / sheet-metal land).
- **Chamfer/fillet operators**: `InsertEdgeRing` or similar for systematic topology insertion.
- **Sweep/loft operators**: `Extrude` or `CreateLateralFaces` for systematic ruled-surface topology.

### 6c. Algorithms (`operations/algorithms/`) ⚙️

Higher-level composite operations using Euler ops:

| Algorithm              | Status | Expands?                                           |
| ---------------------- | ------ | -------------------------------------------------- |
| `bfs.rs`               | 🔒     | No                                                 |
| `bridge_edge.rs`       | 🔒     | No                                                 |
| `components.rs`        | 🔒     | No                                                 |
| `extract_shell.rs`     | ⚙️     | Multi-region extraction                            |
| `flip_edge.rs`         | 🔒     | No                                                 |
| `region_extraction.rs` | ⚙️     | Complex boolean result extraction                  |
| `simplify/`            | ⚙️     | Edge collapse, face merge for post-boolean cleanup |
| `triangulate.rs`       | ❄️     | Only for mesh export, not kernel-critical          |

---

## 7. Queries (`queries/`) ⚙️

Read-only traversal and classification. The _iterator pattern_ is locked. New query functions added per-feature.

| Module              | Key Functions                                                                                                                                                                    | Status                         |
| ------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------ |
| `traverse.rs`       | `LoopEdgeIterator`, `FaceEdgeIterator`, `FaceLoopsIterator`, `FaceAllEdgesIterator`, `VertexRingIterator`, `edge_endpoint_ids()`, `collect_face_vertices()`, `face_edge_count()` | ⚙️ — new convenience functions |
| `radial.rs`         | `radial_valence()`, `radial_ring()`, `is_boundary_edge()`, `is_manifold_edge()`, `twin()`, `radial_faces()`                                                                      | 🔒                             |
| `classification.rs` | `classify_edge()`, `classify_vertex()` — structural classification (boundary/manifold/non-manifold)                                                                              | ⚙️                             |
| `continuity.rs`     | G0/G1 continuity checking across edges                                                                                                                                           | ⚙️ — G2 for fillets            |
| `polygon.rs`        | `face_polygon_vertices()` — ordered vertex positions for a face loop                                                                                                             | 🔒                             |
| `ordering.rs`       | Spatial hash, deterministic entity ordering                                                                                                                                      | 🔒                             |
| `hierarchy.rs`      | `face_shell()`, `shell_region()`, etc.                                                                                                                                           | 🔒                             |
| `edge_map.rs`       | `EdgeVertexMap` for batch endpoint lookups                                                                                                                                       | 🔒                             |
| `filter.rs`         | Entity filtering predicates                                                                                                                                                      | ⚙️                             |
| `centroid.rs`       | Face centroid (topology-level, from vertex indices)                                                                                                                              | 🔒                             |

### QA Gates for all queries

| #   | QA Gate                        | Pass Criteria                                                                           |
| --- | ------------------------------ | --------------------------------------------------------------------------------------- |
| 1   | **Read-only**                  | No `&mut` on arena. No side effects.                                                    |
| 2   | **MAX_ITER guard**             | All iterators cap at 100K to detect corrupted loops.                                    |
| 3   | **Stale handle → Result::Err** | Never panics on stale handles in query paths. Uses `Result<>`.                          |
| 4   | **No f64**                     | Pure topology queries contain zero floating-point. (Spatial queries → `forge-spatial`.) |
| 5   | **Deterministic**              | Output order is deterministic given the same arena state.                               |

---

## Cross-Cutting: What's Completely Irrelevant Right Now

| Area                              | Why                                                                                                                                               |
| --------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------- |
| **NURBS/curve data in topo**      | Topo only holds opaque `CurveRef`/`SurfaceRef`. Actual curve math lives in `worth-geom` / `BrepState`. Topo doesn't care if it's planar or NURBS. |
| **Replay _execution_**            | Schema exists, executor is future.                                                                                                                |
| **Multi-hop re-identification**   | V1 is one-hop. Multi-generation descendant queries deferred.                                                                                      |
| **Wire body / antenna semantics** | Exempted from manifold checks, but no feature uses them deliberately yet.                                                                         |
| **Triangulation**                 | Only for mesh export / rendering. Not kernel-critical path.                                                                                       |
| **G2 / higher continuity**        | G0/G1 exist. G2 for fillets is future.                                                                                                            |

---

## Decision Boundary: forge-topo vs Everything Else

```
Question involves…              → Lives in…
────────────────────────────────────────────────────
Connectivity only?              → forge-topo
Float position / area / AABB?   → forge-spatial
Plane / curve math?             → worth-geom
Policy decisions?               → forge-kernel
Tolerance thresholds?           → forge-kernel (ToleranceConfig)
Surface/curve definitions?      → BrepState (forge-kernel)
Vertex positions?               → GeometryState (forge-kernel)
```
