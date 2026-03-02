# Pre-Boolean Feature Roadmap

This document provides the **strictly ordered** sequence of work required to build the Forge geometric kernel up to the point where Boolean operations can be safely introduced.

It weaves together the Proof System (`P-` milestones) and the Kernel Foundation (`K-` milestones) into a single dependency-respecting path. Booleans are mathematically violent; if we attempt them before this foundation is solid, we will be debugging Boolean logic and structural corruption simultaneously.

**Estimated total: ~47 PRs across 6 phases.**

---

## 🏗️ Phase 1: Observability & Causality Foundation
*Before we can trust geometry, we must be able to observe, trace, and replay decisions.*

### 1. Tracing Pipeline — DecisionSink Collection [K-1]
Replace the `NullSink` dummy implementations. Thread `&mut dyn DecisionSink` into every predicate and topology operator so that every choice is recorded with its scalar margin.
- **Difficulty:** 🟡 Medium | **Size:** ~2-3 PRs
- **Test:** Run `make_block`, assert `DecisionLog` contains entries with `DecisionKind::NearBoundary` and `margin() > 0.0`.

### 2. Lineage DAG Wiring [P3.3]
Every primitive operation must generate a Merkle causal chain linking the resulting topology back to the original `feature_id`.
- **Difficulty:** 🟡 Medium | **Size:** ~3-4 PRs
- **Test:** Run `make_block`, query `LineageStore` for a face, assert its `Lineage::Root` contains the exact `feature_id` from `OperationScope`.

### 3. Pipeline Infrastructure — Multi-Step State Threading [K-6]
Deterministic parametric chains (`MakeBox → Fillet → Cut → Result`) where `TopologyState` output of step N forks into step N+1, accumulating unified `DecisionLog` and `LineageStore`.
- **Difficulty:** 🟡 Medium | **Size:** ~2 PRs
- **Test:** Evaluate a 3-step pipeline twice, assert identical topology hashes and trace span counts.

### 4. Replay Determinism & Serialization Round-Trip [P3.5-lite]
Serialize a `TopologyState` to bytes, deserialize, and assert bit-identical result. Also serialize a `ReplayLog`, replay in a fresh context, assert identical `TopologyState`.
- **Difficulty:** ✅ Easy | **Size:** ~1 PR
- **Test:** Generate a cube, serialize, deserialize, assert `topology_hash` is identical.

---

## 🧮 Phase 2: Geometric & Mathematical Truth
*Before we mutate topology, ensure our math stops floating-point lies and our geometry is real.*

### 5. Volume Oracle [P4.3]
Divergence theorem volume computation for polyhedra (`V = (1/6) Σ |det([v1, v2, v3])|`). Ground-truth fuzzing oracle.
- **Difficulty:** ✅ Easy | **Size:** ~1-2 PRs
- **Test:** Generate a 10×10×10 cube, assert volume is exactly `1000.0`. Generate a tetrahedron, assert analytical match.

### 6. Face Normal Computation
Every face needs a computable, consistently outward-pointing normal. Required by tessellation, winding number classifier, orientation checks, and UI rendering.
- **Difficulty:** ✅ Easy | **Size:** ~1 PR
- **Test:** Generate a cube, assert all 6 face normals point away from the centroid.

### 7. Geometry Completeness — Surfaces & Tolerances [K-4]
Every topological Face must have a `SurfaceId` (a `Plane` for now). Every Vertex must have a 3D coordinate + local tolerance bubble radius. Geometric validation asserts vertices lie on their assigned plane.
- **Difficulty:** ✅ Easy | **Size:** ~2-3 PRs
- **Test:** Move a vertex 10.0 units off its face plane, run geometric validation, assert `TopologyError::GeometricDeviation`.

### 8. Edge-Curve Association
Even planar solids have edges that are lines (plane-plane intersections). Add an `EdgeCurve` trait with `Line3D` as the only implementor for now. When Booleans split faces, new edges need curves — this cannot be retrofitted.
- **Difficulty:** 🟡 Medium | **Size:** ~2 PRs
- **Test:** Generate a cube, assert every edge has an associated `Line3D` whose endpoints match vertex positions within tolerance.

### 9. Geometric Invariants [P0.1]
Block degenerate primitives. Zero-area face detection, zero-length edge detection, inverted shell detection (inside-out cubes).
- **Difficulty:** 🟡 Medium | **Size:** ~2-3 PRs
- **Test:** Collapse two vertices to create a zero-length edge, commit, assert `TopologyError::DegenerateGeometry`.

### 10. Precision Escalation Pipeline [P2.2]
Wire `forge-math` predicates into a dynamic Float → Interval → Rational escalation pipeline. Only pay exact-math cost when float margins are violated.
- **Difficulty:** 🟡 Medium | **Size:** ~3-4 PRs
- **Test:** Query `point_vs_plane` with point `1e-15` off plane, assert `PrecisionEscalation { resolved_at: Interval, float_agreed: false }`.

---

## 🕸️ Phase 3: Topological Complexity & Naming
*Building the complete boundary representation mutation toolkit.*

### 11. Full Euler Operator Suite & NMT Support [K-2]
Complete set of Euler-Poincaré operators (`make_vertex_face_ring`, `kill_edge_make_ring`, etc.) to support face holes, open sheets, and Non-Manifold Transitional states.
- **Difficulty:** 🔴 Hard | **Size:** ~5-8 PRs
- **Test:** Create a non-manifold T-junction. Assert `NmtIntermediate` validation passes and `ManifoldStrict` fails.

### 12a. Persistent Naming — Semantic Tagging [P3.3+]
Assign semantic differentiators (`"Top"`, `"Front"`) to known faces of known primitive shapes during generation.
- **Difficulty:** 🟡 Medium | **Size:** ~2 PRs
- **Test:** Generate a box, assert each face has a unique semantic tag (`"Top"`, `"Bottom"`, `"Front"`, `"Back"`, `"Left"`, `"Right"`).

### 12b. Persistent Naming — Resolution Across Mutation [P3.3+]
Given a persistent name (feature ID + differentiator), resolve it to the correct surviving `FaceId` after topology mutations (e.g., `split_edge` cutting the `"Top"` face).
- **Difficulty:** � Hard | **Size:** ~3 PRs
- **Test:** Tag a face `"Top"`, split an edge on its boundary, query `"Top"` by name, assert it resolves to the correct surviving `FaceId`.

---

## 🎨 Phase 4: Rendering, SDF & Spatial Queries
*Making the kernel visible and queryable.*

### 13. Primitive SDF (Analytical)
Exact signed distance field for box, sphere, cylinder, torus. Gives distance-to-boundary for `NearBoundary` proof decisions, enables ray-marched UI previews, and supports clearance queries.
- **Difficulty:** ✅ Easy | **Size:** ~2 PRs
- **Test:** Query SDF of a unit cube at `(0,0,0)` → assert `-0.5`. At `(2,0,0)` → assert `+1.5`. At `(0.5,0,0)` → assert `0.0` (on boundary).

### 14. Tessellation — Planar Face Triangulation
Triangulate B-Rep faces into `Vec<Triangle>` for GPU rendering. Ear-clipping or monotone decomposition for planar polygons, including faces with holes (inner loops).
- **Difficulty:** � Medium | **Size:** ~2-3 PRs
- **Test:** Tessellate a cube, assert 12 triangles, all normals outward, total triangle area equals sum of face areas.

### 15. Spatial Indexing — Certified Face-Pair Broad Phase [K-5]
R-Tree / BVH over faces of two solids to prune O(N²) intersection checks. Bounding boxes inflated by vertex tolerances.
- **Difficulty:** 🟡 Medium | **Size:** ~2-3 PRs
- **Test:** Broad-phase intersect two disjoint 10,000-face spheres. Assert `< 5ms`, zero candidate pairs.

### 16. Winding Number Classifier [P1.2]
Independent solid-angle-based point-in-solid classifier. Catches ray-casting bugs through algorithmic independence.
- **Difficulty:** 🔴 Hard | **Size:** ~3-4 PRs
- **Test:** Query point on a dodecahedron face boundary, assert `winding_number_classify` matches ray-cast classifier.

---

## ⏮️ Phase 5: Undo/Redo & Transaction Integrity
*Proving that our transactional foundation is truly robust.*

### 17. Undo/Redo System
Snapshot-based undo/redo using `TopologyState` epochs. Each committed state is an immutable snapshot; undo pops to the previous epoch, redo replays forward. Validates that serialization, lineage, and replay all survive round-trips through the undo stack.
- **Difficulty:** 🟡 Medium | **Size:** ~3-4 PRs
- **Test:**
  1. Create box (epoch 1).
  2. Split an edge (epoch 2).
  3. Undo → assert `topology_hash` matches epoch 1 exactly.
  4. Redo → assert `topology_hash` matches epoch 2 exactly.
  5. Assert lineage and replay logs are correctly restored at each epoch.

---

## 🔒 Phase 6: Scale Hardening
*Final preparation for extreme-scale operations.*

### 18. Scale-Invariant Local Coordinates [P2.4]
Mandatory local coordinate transforms before geometric operations to prevent f64 quantization errors at extreme (1e12) coordinates.
- **Difficulty:** 🟡 Medium | **Size:** ~2-3 PRs
- **Test:** Generate a 1×1×1 cube at `(1e12, 1e12, 1e12)`. Assert internal coordinate magnitudes are `~1.0`. Assert global-space query round-trips correctly.

---

> **Deferred: `forge-signal` Execution Upgrades.** Fixing strict single-threading, allocation overhead, and stop-the-world GC (🔴 Hard, 4-6 PRs). Defer until active performance walls are hit.

---

## Summary Table

| # | Item | Diff | PRs | Phase |
|---|------|------|-----|-------|
| 1 | DecisionSink threading | � | 2-3 | 1 |
| 2 | Lineage DAG wiring | 🟡 | 3-4 | 1 |
| 3 | Pipeline state threading | 🟡 | 2 | 1 |
| 4 | Replay determinism + serialization | ✅ | 1 | 1 |
| 5 | Volume Oracle | ✅ | 1-2 | 2 |
| 6 | Face Normal Computation | ✅ | 1 | 2 |
| 7 | Geometry Completeness | ✅ | 2-3 | 2 |
| 8 | Edge-Curve Association | 🟡 | 2 | 2 |
| 9 | Geometric Invariants | 🟡 | 2-3 | 2 |
| 10 | Precision Escalation | 🟡 | 3-4 | 2 |
| 11 | Euler Operators + NMT | 🔴 | 5-8 | 3 |
| 12a | Persistent Naming (tagging) | 🟡 | 2 | 3 |
| 12b | Persistent Naming (resolution) | 🔴 | 3 | 3 |
| 13 | Primitive SDF | ✅ | 2 | 4 |
| 14 | Tessellation | 🟡 | 2-3 | 4 |
| 15 | Spatial Indexing (BVH) | 🟡 | 2-3 | 4 |
| 16 | Winding Number Classifier | 🔴 | 3-4 | 4 |
| 17 | Undo/Redo | 🟡 | 3-4 | 5 |
| 18 | Scale-Invariant Coordinates | 🟡 | 2-3 | 6 |
| | **Total** | | **~44-50** | |
