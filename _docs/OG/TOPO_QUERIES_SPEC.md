# Forge Kernel: Topological Queries Engineering Spec

**Status:** Proposed
**Domain:** `forge-topo::queries`
**Goal:** Fill the critical read-only query gaps in the topology crate to support aerospace-grade boolean and filleting operations without duplicating traversal logic in higher layers.

---

## 1. Loop and Hole Iterators

Currently, `FaceEdgeIterator` only walks the outer loop. We need explicit iterators for holes.

### 1.1 `LoopEdgeIterator`

- **Purpose**: Walk the half-edges of a specific `LoopId`.
- **Implementation**: Identical to the current `FaceEdgeIterator`, but takes a `LoopId` instead of a `FaceId`. Follows `he.next()`.

### 1.2 `FaceAllEdgesIterator`

- **Purpose**: Iterate over _all_ half-edges of a face (outer loop + all inner loops).
- **Implementation**:
  - Iterates the outer loop first.
  - Then iterates each inner loop sequentially.
  - Yields `Result<HalfEdgeId, KernelError>` to maintain the corruption-safe `MAX_ITER` guarantees.

### 1.3 `FaceLoopsIterator` / `face_loops(arena, face) -> Vec<LoopId>`

- **Purpose**: Return the `outer_loop` followed by all `inner_loops`.
- **Implementation**: Direct read from `FaceData::outer_loop` and `FaceData::inner_loops`.

---

## 2. Hierarchy and Shell Queries

The current boolean code hand-rolls BFS (Breadth-First Search) traversals to discover shells. This must be pushed down to `forge-topo`.

### 2.1 `shell_faces(arena, shell: ShellId) -> Result<Vec<FaceId>>`

- **Purpose**: Return all faces belonging to a given shell.
- **Implementation**:
  - Since `FaceData` stores a parent `shell: ShellId`, this requires an arena scan: `arena.iter_faces().filter(|f| f.1.shell() == shell)`.
  - _Optimization Note_: If this O(N) scan is too slow, we may need to add a `faces: Vec<FaceId>` to `ShellData` in the future, but for now an arena filter is geometrically safe and straightforward.

### 2.2 `region_shells(arena, region: RegionId) -> Vec<ShellId>`

- **Purpose**: Return the outer shell and all inner shells (voids) of a region.
- **Implementation**: Direct read from `RegionData::outer_shell` and `RegionData::inner_shells`.

---

## 3. Adjacency and Neighborhood Queries

Boolean operations frequently need to look at face-to-face adjacencies and vertex umbrellas.

### 3.1 `face_adjacent_faces(arena, face: FaceId) -> Result<Vec<FaceId>>`

- **Purpose**: Find all faces that share an edge with the given face.
- **Implementation**:
  - Use `FaceAllEdgesIterator`.
  - For each half-edge, look at its `radial_next()`. If the adjacent face is different from the source face, add it to a deduplicated set.

### 3.2 `vertex_faces(arena, vertex: VertexId) -> Result<Vec<FaceId>>`

- **Purpose**: Find all faces touching a specific vertex.
- **Implementation**:
  - Use our existing `VertexRingIterator`.
  - For each outgoing half-edge, collect its `face()`.
  - Deduplicate and return.

---

## 4. Manifold and Topology Type Classifications

Aerospace kernels must explicitly handle non-manifold states.

### 4.1 Edge Classification

- **`is_boundary_edge(arena, he)`**: (Already exists) Valence == 1.
- **`is_manifold_edge(arena, he) -> Result<bool>`**: True if radial valence == 2.
- **`is_non_manifold_edge(arena, he) -> Result<bool>`**: True if radial valence > 2.
- **`is_laminar_edge(arena, he) -> Result<bool>`**: Another terminology for boundary edge (valence == 1), useful for Sheet bodies.

### 4.2 Vertex Manifoldness (`vertex_is_manifold(arena, vertex) -> Result<bool>`)

- **Purpose**: Detect "bowtie" vertices where multiple surface fans touch at a single mathematical point without sharing edges.
- **Algorithm**: _This is the only algorithm requiring care._
  - We must determine if the incident faces form a single connected topological disk (or half-disk for boundaries).
  - 1. Find all outgoing half-edges from the vertex using `VertexRingIterator`. Let this set be `O`.
  - 2. Pick the first edge in `O`. Walk its "umbrella" by taking the `prev()` half-edge, then taking its `radial_next()` (the twin). That lands you on another outgoing half-edge.
  - 3. Repeat this walk until you loop back or hit a boundary. Mark all visited edges in `O`.
  - 4. If there are any unvisited edges remaining in `O` after the umbrella walk finishes, the vertex has multiple disconnected umbrellas — it is a **bowtie (non-manifold)** vertex.

### 4.3 Loop Classification

- **`is_outer_loop(arena, face, loop) -> Result<bool>`**:
  - **Implementation**: Trivial O(1) check. `arena.get_face(face)?.outer_loop() == loop`.
- **`is_inner_loop(arena, face, loop) -> Result<bool>`**:
  - **Implementation**: Return `!is_outer_loop(arena, face, loop)`.

---

## 5. Geometric and Bounding Box Queries

We need centralized AABB caching/computation to stop duplicating it.

### 5.1 Hierarchical Bounds

- **`face_bounds(arena, geom, face) -> Result<Option<Aabb>>`**: Refactors `compute_face_aabb` into `queries/bounds.rs`. Iterates `FaceAllEdgesIterator` and computes AABB.
- **`shell_bounds(arena, geom, shell) -> Result<Option<Aabb>>`**: Accumulates `face_bounds` for all faces in the shell.
- **`solid_bounds(arena, geom, body) -> Result<Option<Aabb>>`**: Accumulates bounds of all regions/shells inside the solid body.

---

## 6. Continuity Queries (The G1/G2 missing links)

While exact curvature (G2) requires parametric surfaces, tangent continuity (G1) is critical for polyhedral meshes representing smooth objects.

### 6.1 `edge_dihedral_angle(arena, geom, edge) -> Result<Option<f64>>`

- **Purpose**: Compute the signed angle between the two faces sharing a manifold edge.
- **Implementation**: Needs normal computation for the two adjacent faces. Returns `None` if the edge is non-manifold or a boundary.

### 6.2 `is_edge_g1_continuous(arena, geom, edge, angle_threshold) -> Result<bool>`

- **Purpose**: Boolean check used heavily by feature recognition and filleting.
- **Implementation**: Wraps `edge_dihedral_angle` against the provided tolerance.

---

## 7. Geometry Layer Updates (`worth-geom`)

To support the rigorous, O(1) mathematical lookups required by the advanced Topo queries (specifically bounding and continuity), the geometry layer needs three minor additions.

### 7.1 Unified AABB Aggregation

- **Method**: `Aabb::union(&self, other: &Aabb) -> Aabb`
- **Purpose**: Required by `shell_bounds` and `solid_bounds` in `queries/bounds.rs`. Allows O(F) topological accumulation of face bounds instead of an O(V) full vertex rescan.

### 7.2 Point-to-AABB Distance Checks

- **Method**: `Aabb::distance_to_point_sq(&self, point: &[f64; 3]) -> f64`
- **Purpose**: Required to quickly prune branches during ray-casting or point-in-solid classification when searching the BVH for the nearest face.

### 7.3 Parametric Surface Evaluation (Future-Proofing)

- **Trait**: `pub trait EvaluateNormal { fn normal_at(&self, point: &[f64; 3]) -> Result<[f64; 3], KernelError>; }`
- **Purpose**: Required by `edge_dihedral_angle`. The Topology layer must stop assuming all faces are flat planar polygons that can have a Newell normal computed from their boundary vertices.

---

## 8. Hidden Algorithms to Extract from Boolean Engine

The Boolean layer currently hoards several advanced algorithms that belong in lower crates. These must be extracted to their proper architectural homes so other operations (like sweeping, lofting, and filleting) can use them.

### 8.1 Computational Geometry (`worth-geom`)

These are pure mathematical routines that operate on arrays of `[f64; 3]`. They must not depend on `FaceId` or `TopologyState`.

- **Cyrus-Beck Line Clipping**:
  - **From**: `boolean/split/gate.rs`
  - **To**: `worth_geom::algorithms::clipping::clip_line_to_polygon`
  - **Purpose**: Clips an infinite intersection line against a face's boundary polygon. Classic 2D graphics algorithm adapted for 3D boolean gating.
- **Extremal Vertex Raycasting / Hole Bridging**:
  - **From**: `boolean/postprocess/hole_splice.rs`
  - **To**: `worth_geom::algorithms::polygon::bridge_polygon_holes`
  - **Purpose**: Finds the +X extremal vertex on a hole and raycasts to the mutually visible outer boundary. Textbook triangulation subroutine.
- **Exact Symbolic Orientation (4x4 Determinants)**:
  - **From**: `boolean/split/gate.rs`
  - **To**: Already exists conceptually in `worth-math::predicates`, but usage needs to be centralized. Shewchuk-style robust predicates preventing roundoff shatters.
- **2D Dominant Axis Projection & Polygon Overlap**:
  - **From**: `boolean/classify/coplanar.rs`
  - **To**: `worth_geom::algorithms::intersection::polygons_overlap_3d`
  - **Purpose**: Projects 3D coplanar faces onto their dominant 2D axis to compute geometric intersection areas.

### 8.2 Topological Graph Theory (`forge-topo::operations::algorithms`)

These algorithms mutate or interrogate the half-edge graph using standard Euler ops or arena queries. They should know nothing about "tools" or "targets".

- **Maximal Region Extraction (Perimeter Twin-Hopping)**:
  - **From**: `boolean/postprocess/polygon_extract.rs`
  - **To**: `forge_topo::operations::algorithms::region_extraction`
  - **Purpose**: BFS coplanar discovery + $O(N)$ boundary walk via twin-hopping. Must be rewritten to use compound Euler ops (e.g., iterative `KillEdgeFace`) instead of raw arena graph nucleations.
- **Degenerate Graph Surgery**:
  - **From**: `boolean/assemble/cleanup.rs`
  - **To**: `forge_topo::operations::algorithms::simplify`
  - **Purpose**: Safely excising zero-length edges and zero-area faces. This is highly dangerous and belongs strictly in certified topo algorithms.
- **Collinear Degree-2 Vertex Consolidation**:
  - **From**: `boolean/postprocess/vertex.rs`
  - **To**: `forge_topo::operations::algorithms::simplify::consolidate_collinear_vertices`
  - **Purpose**: Umbrella walks to find valence-2 vertices and merge collinear edges.

### 8.3 Spatial Heuristics & Tolerance Resolvers

Algorithms bridging exact math and floating-point reality.

- **Multi-Sample Normal Perturbation**:
  - **From**: `boolean/classify/eval.rs`
  - **To**: `forge_topo::queries::classify::classify_point_with_perturbation`
  - **Purpose**: Pragmatic resolution of coplanar grazing contact via $\pm \epsilon$ sampling and majority voting.
- **Fuzzy Bipartite Edge Matching**:
  - **From**: `boolean/assemble/stitch/fallback.rs`
  - **To**: `worth_geom::spatial::edge_match::fuzzy_match_edges`
  - **Purpose**: Matching undirected topological edges via 3D Euclidean endpoint proximity. The geometric distance check belongs in `worth_geom`, while the twin-wiring stays in `forge_topo::euler::SewEdge`.

---

## Execution Plan

1. **Extract Geometry Algorithms**: Move Cyrus-Beck, Hole Bridging, and Point-in-Polygon logic into `worth-geom::algorithms`.
2. **Upgrade Geometry Primitives**: Add `union` and `distance_to_point_sq` to `Aabb`, and add the `EvaluateNormal` trait in `worth-geom`.
3. **Build Topo Query Modules**:
   - `crates/forge-topo/src/topology/queries/hierarchy.rs` (Shell/Region/Lump traversal)
   - `crates/forge-topo/src/topology/queries/bounds.rs` (AABB aggregations)
   - `crates/forge-topo/src/topology/queries/continuity.rs` (G1/G2 checks)
   - `crates/forge-topo/src/topology/queries/classification.rs` (Edge/Loop typing, Vertex Bowtie checks)
4. **Expand Traversals**: Add `LoopEdgeIterator` and `FaceAllEdgesIterator` to `traverse.rs`.
5. **Extract Topo Algorithms**: Move `polygon_extract.rs` (Region Extraction), `cleanup.rs` (Degenerate Surgery), and `vertex.rs` (Consolidation) into `forge-topo::operations::algorithms`.
6. **Refactor Boolean Engine**: Rip out all hand-rolled `he.next()` loops, raw memory insertions, and duplicated math from `forge-kernel::operations::boolean` and replace them with calls to the new queries and algorithms.
