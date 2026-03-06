1. Algorithms & Simplification (operations/algorithms/)
simplify/cleanup.rs (CRITICAL)

Graph Annihilation: In remove_degenerate_faces, the operator physically deletes all half-edges belonging to a collapsed face. However, it fails to update the radial_next pointers of the twin half-edges in adjacent faces. Every adjacent face is left with radial pointers aiming at freed memory, instantly destroying the topological graph.

Analytic Geometry Destruction: find_zero_length_edges flags any edge where origin == dest, and remove_degenerate_faces deletes any face with < 3 vertices. In aerospace CAD, a cylindrical seam or sphere equator is a valid 1-vertex, 1-edge self-loop. A hemisphere has exactly 2 vertices. This cleanup pass ruthlessly deletes perfectly valid analytic geometry.

Mathematical Edge Leak: remove_zero_length_edges deletes the half-edges but completely forgets to call draft.remove_edge(he.edge()). The mathematical 1D Edge entities leak permanently into the arena.

extract_shell.rs

NMT Boundary Blindness: Uses is_face_group_boundary_half_edge, which checks if an edge borders an outside face via he.radial_next(). On a non-manifold edge (valence 3+), this only checks one adjacent twin. It will completely miss group boundaries and leave the extracted shell sewn to the main body.

Unsew Crash: It uses UnsewEdge to cut the shell loose. Because UnsewEdge panics on valence > 2, extracting any shell touching an NMT junction will hard-crash the algorithm.

Component Contiguity Violation: It topologically unsews the faces but leaves them inside the original Shell entity, violating the rule that a Shell must be a single connected manifold component.

triangulate.rs

Fragile Loop State Tracking: The loop tracks the remaining polygon with current_face = face. If the underlying kernel's MakeEdgeFace implementation assigns the sliced triangle to face and the remaining N-gon to new_face, the loop will attempt to slice vertices out of a 3-sided triangle on the next iteration and crash.

Pinch-Vertex Degeneracy: It uses a blind fan-triangulation from verts[0]. If the face has an NMT pinch (the same vertex appears twice in the loop), fan triangulation will attempt to connect a vertex to itself, generating a fatal 0-length edge.

region_extraction.rs

Blind to Holes: walk_face_group_boundary_perimeter finds the first boundary half-edge and loops until it closes. If the face group has internal holes or multiple outer boundaries (like an open cylinder), it returns one and silently drops the rest.

flip_edge.rs

Non-Convex Inversion: Performs a purely topological diagonal flip. If the quad formed by the two triangles is non-convex, the new diagonal will lie physically outside the face boundary, causing self-intersecting B-Reps.

2. Boundary Editing Operators (operations/boundary_editing/)
join_faces.rs & join_faces_nmt.rs

Outer Loop Invariant Overwrite: The operators unconditionally set the surviving face's outer loop to he_next. If the edge being removed was bridging a hole (an inner loop shared by two faces), this silently overwrites the face's actual outer boundary with the inner loop, destroying the face definition.

Cross-Shell Contamination: Does not check if face_survive and face_remove belong to the same Shell. Merging faces across different shells creates a topological graph spanning multiple hierarchical components.

Dangling Vertex Pointers (1-Gon Corruption): If face_remove is a 1-edge loop (droplet), twin_next == he_twin. The code calls set_primary_disk(twin_next) on the vertex, but then immediately deletes he_twin. The vertex is left pointing to deleted memory.

kill_edge_make_loop.rs (KEML)

Outer Loop Corruption: Like JoinFaces, if this operator removes a bridge edge between two inner loops, it incorrectly promotes one of the holes to be the face's outer boundary via get_loop_mut(outer_loop)?.set_half_edge(twin_next).

Dangling Vertex Pointers: It calls remove_half_edge on the target edges but totally fails to check if vertex_a or vertex_b had their primary_disk pointing to them. If they did, the vertices are left with dangling pointers.

make_face_from_vertices.rs & make_face_in_shell_from_vertices.rs

NMT Vertex Disk Leak (Critical): When wiring vertices, it checks if orig_out == HalfEdgeId::DANGLING to set the primary disk. If the vertex is already used by another face, it correctly skips the overwrite but fails to call arena.add_disk_entry(v, he). The new face is disconnected from the vertex's secondary NMT disk list, making it invisible to radial graph traversals.

Dangling Shell Representative (in_shell variant): If the target shell was empty, its representative_face is DANGLING. Adding a new face does not update this pointer, leaving the shell structurally invalid.

make_edge_kill_loop.rs (MEKL)

Artificial NMT Restriction: Explicitly errors if he_a is not on an outer loop. In aerospace Boolean operations, bridging two inner loops (holes) together to form a single continuous cutout is standard. This arbitrary limit blocks valid topological operations.

kill_face_make_ring_hole.rs & make_face_kill_ring_hole.rs

Inverted Normals: Outer loops traverse counter-clockwise; inner loops traverse clockwise. Promoting a hole to an outer face (or vice versa) without explicitly reversing the half-edge cycle results in a mathematically "inside-out" face with inverted normals.

Self-Assignment Crash (kill_face): Lacks a face_to_kill != target_face check. If identical, it adds its own loop to itself as a hole, deletes itself, and corrupts the B-Rep tree.

Disjoint Shell Violation (make_face): Extracts a hole into an independent face but leaves it in the same Shell. Shells must be contiguous components.

3. Lifecycle Operations (operations/lifecycle/)
body_ops.rs (CloneBody)

Dangling Face Pointers on Loops: During the deep clone, new loops are created using LoopData::new(DANGLING, FaceId::DANGLING). Later, their half-edge pointers are wired up, but their face pointers are never updated. Every cloned loop points to DANGLING.

Loss of NMT Disks: When cloning vertices, it only maps the primary_disk. If the original vertex had multiple independent radial disks (an NMT pinch vertex), the secondary disks are entirely lost.

Wireframe Data Loss: The clone topology discovery loops over faces_of_shell. If a Body contains a Wire shell (1D edges but 0 faces), the clone operator completely ignores the geometry and causes data loss.

shell_ops.rs

Backwards Extraction Logic (ExtractShell): It blocks extraction of the outer shell if inner_count == 0. This logic is precisely backward. Extracting an outer shell from an otherwise empty region safely empties it. But extracting an outer shell when there are inner shells leaves those inner shells as boundless infinite voids—this is what should be blocked.

Ghost Shells (SplitShell): If all faces are moved to the new shell, the old shell's representative face is set to DANGLING, leaving a zombie shell in the region instead of garbage collecting it.

lump_ops.rs

Orphaned Empty Bodies (MergeLumps): If the source lump is the only lump belonging to source_body, the operator removes and deletes the lump but leaves behind a dangling, empty Body entity.

4. Non-Manifold Operators (operations/non_manifold/)
sew_edge.rs

Vertex Disk Desync: Sewing two boundaries correctly links their radial pointers, but the start and end vertices previously maintained separate topological disk lists. The operator fails to merge these lists in the VertexData, corrupting vertex-to-face iterators.

T-Junctions Blocked: Forces incoming edges to be boundaries (radial_next == self). A true NMT kernel must allow sewing a sheet boundary directly into the middle of an existing manifold edge to create a valence-3 T-junction.

Self-Sewing Corruption: Does not verify he_a != he_b. A zero-length boundary edge will pass the antiparallel check, allowing the operator to sew a half-edge to itself, permanently corrupting the radial iteration cycle.

unsew_edge.rs

Dangling Original Edge Pointer: It creates a new_edge and assigns it to he_b. However, original_edge.half_edge might have been pointing directly at he_b. The operator fails to reset original_edge's representative half-edge pointer back to he_a, causing downstream geometric queries to route into the newly severed mesh component.

Strict Manifold Limit: The valence == 2 check prevents unsewing/peeling a single face off an NMT radial ring containing 3 or more faces.

5. Files with Structural Integrity (But Inheriting Risk)
The read-only traversers (bfs.rs, components.rs), basic struct definitions (mod.rs files), and structural teardowns (solid.rs, lump.rs, shell.rs) are fundamentally sound B-Rep boilerplate, but they will inherently fail or infinite-loop if they encounter the graph corruption generated by the operators above.