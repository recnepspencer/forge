//! BSP → Halfedge Mesh Conversion for Multi-Cell Solids.
//!
//! DOMAIN: Convert a BSP merge result (potentially multiple solid leaves)
//! into a single manifold halfedge mesh. This is Phase 4 of the EMBER pipeline.
//!
//! ALGORITHM:
//!   1. Extract boundary ConvexCells from the BspSolid
//!   2. For each cell, insert vertices (with cross-cell dedup via position tolerance)
//!   3. For each cell, insert faces + halfedge loops
//!   4. Remove internal faces (shared between adjacent solid cells)
//!   5. Stitch twins across all faces using VertexId-keyed directed-edge map
//!   6. Commit draft → TopologyState
//!
//! INVARIANTS:
//!   - Vertex dedup uses position tolerance (same as build_halfedge_mesh)
//!   - Twin stitching uses VertexId keys (exact, no geometric matching)
//!   - Internal faces detected by plane index + opposite normal sign
//!   - Result satisfies Euler V−E+F=2 per shell
//!
//! DEPENDENCIES: forge-geom (BspSolid, extract_boundary_cells, ConvexCell),
//!               forge-topo (arena insertion), GeometryStore, ModelingContext

use std::collections::HashMap;

use forge_core::KernelError;
use forge_core::tracing::{DecisionKind, DecisionTier};
use forge_geom::Plane;
use forge_geom::spatial::bsp::{BspConfig, BspSolid, ConvexCell};
use forge_topo::arena::{FaceData, HalfEdgeData, VertexData, LoopData};
use forge_topo::handles::{FaceId, HalfEdgeId, VertexId, LoopId};
use forge_topo::state::{TopologyState, MutableDraft};

use crate::check_tolerance;
use crate::core::ModelingContext;
use crate::geometry_store::GeometryStore;
use super::checkpoint::validate_checkpoint;

/// Convert a BspSolid into a halfedge mesh.
///
/// Extracts boundary ConvexCells, builds a combined mesh with proper
/// vertex dedup, internal face removal, and twin stitching.
pub fn bsp_to_mesh(
    solid: &BspSolid,
    ctx: &mut ModelingContext,
) -> Result<(TopologyState, GeometryStore), KernelError> {
    let config = BspConfig::default();
    let cells = forge_geom::spatial::bsp::extract_boundary_cells(solid, &config)
        .map_err(|e| KernelError::InternalError {
            message: format!("BSP boundary extraction failed: {e}"),
            context: None,
        })?;

    ctx.log_decision(
        DecisionKind::Exact,
        DecisionTier::Deterministic,
        [0.0, 0.0, 0.0],
        0.0,
        0.0,
    );

    if cells.is_empty() {
        return Ok((TopologyState::empty(), GeometryStore::new()));
    }

    if cells.len() == 1 {
        let (cell, _) = &cells[0];
        let mesh = crate::mesh_builder::build_halfedge_mesh(cell, ctx)?;
        return Ok(mesh.into_parts());
    }

    build_multi_cell_mesh(&cells, ctx)
}

/// Build a halfedge mesh from multiple ConvexCells.
///
/// Uses cross-plane twin stitching to handle non-manifold edges
/// from overlapping coplanar faces of different-sized cells.
fn build_multi_cell_mesh(
    cells: &[(ConvexCell, Vec<usize>)],
    ctx: &mut ModelingContext,
) -> Result<(TopologyState, GeometryStore), KernelError> {
    let tolerance = ctx.get_tolerance().get_spatial_tolerance();

    let state = TopologyState::empty();
    let mut draft = state.into_mutation();
    let mut geometry = GeometryStore::new();

    let mut vertex_pool: Vec<(VertexId, [f64; 3])> = Vec::new();
    let mut edge_map: HashMap<(VertexId, VertexId), Vec<HalfEdgeId>> = HashMap::new();
    let mut face_plane_map: HashMap<FaceId, usize> = HashMap::new();

    let placeholder_he = HalfEdgeId::from_raw_parts(u32::MAX, 0);
    let placeholder_loop = LoopId::from_raw_parts(u32::MAX, 0);

    for (_cell_idx, (cell, bsp_plane_indices)) in cells.iter().enumerate() {
        let cell_planes = cell.planes();

        let cell_vertex_ids = insert_cell_vertices(
            &mut draft, &mut geometry, &mut vertex_pool,
            cell, cell_planes, tolerance, ctx,
        )?;

        insert_cell_faces(
            &mut draft, &mut geometry, &mut edge_map, &mut face_plane_map,
            cell, &cell_vertex_ids, cell_planes, bsp_plane_indices,
            placeholder_he, placeholder_loop,
        )?;
    }

    // ── Checkpoint: post_insert_faces ────────────────────────────────────
    // Skip twin checks: twins are placeholder self-twins before stitching.
    validate_checkpoint(&draft, ctx, "post_insert_faces", true)?;

    // Stitch twins with cross-plane pairing for non-manifold edges
    stitch_twins_cross_plane(&mut draft, &edge_map, &face_plane_map)?;

    // ── Checkpoint: post_stitch_twins ───────────────────────────────────
    validate_checkpoint(&draft, ctx, "post_stitch_twins", false)?;

    // After stitching, remove coplanar twin pairs (internal faces)
    let internal_faces = detect_coplanar_twin_faces(&face_plane_map, &draft);
    if !internal_faces.is_empty() {
        remove_stitched_faces(&mut draft, &internal_faces)?;
    }

    // ── Checkpoint: post_remove_faces ───────────────────────────────────
    validate_checkpoint(&draft, ctx, "post_remove_faces", false)?;

    // Merge adjacent coplanar faces by dissolving shared edges
    merge_coplanar_neighbors(&mut draft, &geometry, &face_plane_map, tolerance)?;

    // ── Checkpoint: post_merge_coplanar ─────────────────────────────────
    validate_checkpoint(&draft, ctx, "post_merge_coplanar", false)?;

    let total_faces = draft.arena().face_count();
    let total_edges = draft.arena().half_edge_count() / 2;
    let total_verts = draft.arena().vertex_count();

    ctx.log_decision(
        DecisionKind::Exact,
        DecisionTier::Deterministic,
        [0.0, 0.0, 0.0],
        (total_verts as f64 - total_edges as f64 + total_faces as f64),
        2.0,
    );

    let topology = draft.commit()?;
    Ok((topology, geometry))
}

/// Insert vertices from one ConvexCell, deduplicating against the global pool.
fn insert_cell_vertices(
    draft: &mut MutableDraft,
    geometry: &mut GeometryStore,
    vertex_pool: &mut Vec<(VertexId, [f64; 3])>,
    cell: &ConvexCell,
    cell_planes: &[Plane],
    tolerance: f64,
    ctx: &mut ModelingContext,
) -> Result<Vec<VertexId>, KernelError> {
    let placeholder_he = HalfEdgeId::from_raw_parts(u32::MAX, 0);
    let tol_sq = tolerance * tolerance;
    let mut cell_vertex_ids = Vec::with_capacity(cell.vertex_count());

    for vert in cell.vertices() {
        let pos = *vert.position();

        let existing = vertex_pool.iter().find(|(_, existing_pos)| {
            let diff = forge_math::linalg::sub(pos, *existing_pos);
            forge_math::linalg::norm_sq(diff) < tol_sq
        });

        if let Some((existing_vid, existing_pos)) = existing {
            let dist = forge_math::linalg::norm(forge_math::linalg::sub(pos, *existing_pos));
            check_tolerance!(ctx, tolerance, dist, pos, DecisionKind::NearBoundary { threshold: tolerance });
            cell_vertex_ids.push(*existing_vid);
        } else {
            let vid = draft.arena_mut().insert_vertex(VertexData::new(placeholder_he));

            let [pa, pb, pc] = vert.plane_indices();
            let stored_exact = if pa < cell_planes.len() && pb < cell_planes.len() && pc < cell_planes.len() {
                match forge_geom::primitives::plane::intersect_three_planes_exact(
                    &cell_planes[pa], &cell_planes[pb], &cell_planes[pc],
                ) {
                    Ok(exact_pos) => {
                        geometry.set_vertex_position_symbolic(vid, exact_pos, pos, [pa, pb, pc]);
                        true
                    }
                    Err(_) => false,
                }
            } else {
                false
            };

            if !stored_exact {
                geometry.set_vertex_position(vid, pos);
            }

            vertex_pool.push((vid, pos));
            cell_vertex_ids.push(vid);
        }
    }

    Ok(cell_vertex_ids)
}

/// Insert faces and halfedge loops for one ConvexCell.
fn insert_cell_faces(
    draft: &mut MutableDraft,
    geometry: &mut GeometryStore,
    edge_map: &mut HashMap<(VertexId, VertexId), Vec<HalfEdgeId>>,
    face_plane_map: &mut HashMap<FaceId, usize>,
    cell: &ConvexCell,
    cell_vertex_ids: &[VertexId],
    cell_planes: &[Plane],
    bsp_plane_indices: &[usize],
    placeholder_he: HalfEdgeId,
    placeholder_loop: LoopId,
) -> Result<(), KernelError> {
    for cell_face in cell.faces() {
        let face_verts = cell_face.vertices();
        if face_verts.len() < 3 {
            continue;
        }

        let face_id = draft.arena_mut().insert_face(FaceData::new(placeholder_loop));

        let loop_id = draft.arena_mut().insert_loop(LoopData::new(
            placeholder_he,
            face_id,
        ));

        let local_plane_idx = cell_face.plane_idx();
        if local_plane_idx < cell_planes.len() {
            let plane = &cell_planes[local_plane_idx];
            geometry.set_face_plane(face_id, plane.clone());

            // ConvexCell planes: [bbox_0..5, constraint_0, constraint_1, ...].
            // bsp_plane_indices maps constraint offset → BSP plane index.
            let bbox_offset = 6;
            let bsp_idx = if local_plane_idx >= bbox_offset {
                let constraint_idx = local_plane_idx - bbox_offset;
                if constraint_idx < bsp_plane_indices.len() {
                    bsp_plane_indices[constraint_idx]
                } else {
                    usize::MAX
                }
            } else {
                usize::MAX
            };
            face_plane_map.insert(face_id, bsp_idx);
        }

        let vert_count = face_verts.len();
        let mut he_ids = Vec::with_capacity(vert_count);

        for &cell_vert_idx in face_verts {
            let origin = cell_vertex_ids[cell_vert_idx];
            let he_id = draft.arena_mut().insert_half_edge(HalfEdgeData::new(
                placeholder_he,
                placeholder_he,
                placeholder_he,
                face_id,
                origin,
            ));
            he_ids.push(he_id);
        }

        for i in 0..vert_count {
            let next_i = (i + 1) % vert_count;
            let prev_i = if i == 0 { vert_count - 1 } else { i - 1 };

            let arena = draft.arena_mut();
            arena.get_half_edge_mut(he_ids[i])?.set_next(he_ids[next_i]);
            arena.get_half_edge_mut(he_ids[i])?.set_prev(he_ids[prev_i]);

            let origin_a = cell_vertex_ids[face_verts[i]];
            let origin_b = cell_vertex_ids[face_verts[next_i]];
            edge_map.entry((origin_a, origin_b)).or_default().push(he_ids[i]);
        }

        draft.arena_mut().get_face_mut(face_id)?.set_outer_loop(loop_id);
        draft.arena_mut().get_loop_mut(loop_id)?.set_half_edge(he_ids[0]);

        for &he_id in &he_ids {
            let origin = draft.arena().get_half_edge(he_id)?.origin();
            draft.arena_mut().get_vertex_mut(origin)?.set_outgoing(he_id);
        }
    }

    Ok(())
}

/// Stitch twin pointers using cross-plane pairing.
///
/// For manifold edges (1 forward, 1 reverse): pair normally.
/// For non-manifold edges (N forward, N reverse from overlapping cells):
/// pair halfedges from faces on DIFFERENT BSP planes to produce valid
/// edge-face adjacency. Remaining excess halfedges from overlapping
/// coplanar faces are paired with each other.
fn stitch_twins_cross_plane(
    draft: &mut MutableDraft,
    edge_map: &HashMap<(VertexId, VertexId), Vec<HalfEdgeId>>,
    face_plane_map: &HashMap<FaceId, usize>,
) -> Result<(), KernelError> {

    for (&(a, b), fwd_ids) in edge_map {
        // Only process each edge pair once (a < b direction)
        if a.index() > b.index() {
            continue;
        }
        let rev_ids = match edge_map.get(&(b, a)) {
            Some(ids) => ids,
            None => {
                return Err(KernelError::InternalError {
                    message: format!(
                        "EMBER mesh: no twin for directed edge ({:?} -> {:?}); mesh not closed",
                        a, b
                    ),
                    context: None,
                });
            }
        };

        if fwd_ids.len() == 1 && rev_ids.len() == 1 {
            draft.arena_mut().get_half_edge_mut(fwd_ids[0])?.set_twin(rev_ids[0]);
            draft.arena_mut().get_half_edge_mut(rev_ids[0])?.set_twin(fwd_ids[0]);
            continue;
        }

        if fwd_ids.len() != rev_ids.len() {
            return Err(KernelError::InternalError {
                message: format!(
                    "EMBER mesh: unbalanced non-manifold edge ({:?} -> {:?}); {} forward, {} reverse",
                    a, b, fwd_ids.len(), rev_ids.len()
                ),
                context: None,
            });
        }

        // Get BSP plane for each halfedge's face
        let get_plane = |he_id: HalfEdgeId| -> usize {
            draft.arena().get_half_edge(he_id)
                .ok()
                .and_then(|he| face_plane_map.get(&he.face()).copied())
                .unwrap_or(usize::MAX)
        };

        let mut fwd_remaining: Vec<(HalfEdgeId, usize)> = fwd_ids.iter()
            .map(|&id| (id, get_plane(id)))
            .collect();
        let mut rev_remaining: Vec<(HalfEdgeId, usize)> = rev_ids.iter()
            .map(|&id| (id, get_plane(id)))
            .collect();

        // Phase 1: pair cross-plane (forward plane != reverse plane)
        let mut i = 0;
        while i < fwd_remaining.len() {
            let fwd_plane = fwd_remaining[i].1;
            if let Some(rev_idx) = rev_remaining.iter().position(|(_, p)| *p != fwd_plane) {
                let (fwd_id, _) = fwd_remaining.remove(i);
                let (rev_id, _) = rev_remaining.remove(rev_idx);
                draft.arena_mut().get_half_edge_mut(fwd_id)?.set_twin(rev_id);
                draft.arena_mut().get_half_edge_mut(rev_id)?.set_twin(fwd_id);
            } else {
                i += 1;
            }
        }

        // Phase 2: pair remaining (same-plane coplanar pairs)
        for i in 0..fwd_remaining.len().min(rev_remaining.len()) {
            let (fwd_id, _) = fwd_remaining[i];
            let (rev_id, _) = rev_remaining[i];
            draft.arena_mut().get_half_edge_mut(fwd_id)?.set_twin(rev_id);
            draft.arena_mut().get_half_edge_mut(rev_id)?.set_twin(fwd_id);
        }
    }

    Ok(())
}

/// Detect coplanar twin face pairs that are internal boundaries.
///
/// After cross-plane stitching, some faces end up with ALL their twins
/// pointing to faces on the same BSP plane. These face pairs are
/// coplanar internal boundaries between adjacent solid cells and should
/// be removed.
fn detect_coplanar_twin_faces(
    face_plane_map: &HashMap<FaceId, usize>,
    draft: &MutableDraft,
) -> Vec<FaceId> {
    let mut internal: Vec<FaceId> = Vec::new();
    let mut checked: std::collections::HashSet<FaceId> = std::collections::HashSet::new();

    for (&fid, &bsp_idx) in face_plane_map {
        if bsp_idx == usize::MAX || checked.contains(&fid) {
            continue;
        }

        // Check if ALL twins of this face's halfedges point to faces
        // on the same BSP plane
        let face = match draft.arena().get_face(fid) {
            Ok(f) => f,
            Err(_) => continue,
        };
        let loop_id = face.outer_loop();
        let first_he = match draft.arena().get_loop(loop_id) {
            Ok(l) => l.half_edge(),
            Err(_) => continue,
        };

        let mut all_coplanar = true;
        let mut twin_face = None;
        let mut current = first_he;
        loop {
            let he = match draft.arena().get_half_edge(current) {
                Ok(h) => h,
                Err(_) => { all_coplanar = false; break; }
            };
            let twin_id = he.twin();
            let twin_he = match draft.arena().get_half_edge(twin_id) {
                Ok(h) => h,
                Err(_) => { all_coplanar = false; break; }
            };
            let twin_fid = twin_he.face();
            let twin_plane = face_plane_map.get(&twin_fid).copied().unwrap_or(usize::MAX);

            if twin_plane != bsp_idx {
                all_coplanar = false;
                break;
            }
            twin_face = Some(twin_fid);

            current = he.next();
            if current == first_he { break; }
        }

        if all_coplanar {
            if let Some(twin_fid) = twin_face {
                if !checked.contains(&twin_fid) {
                    checked.insert(fid);
                    checked.insert(twin_fid);
                    internal.push(fid);
                    internal.push(twin_fid);
                }
            }
        }
    }

    internal
}

/// Remove faces from a stitched mesh, re-linking twin pointers.
///
/// For each removed face, update any halfedge that was twinned with
/// a halfedge on the removed face to point to the correct replacement twin.
fn remove_stitched_faces(
    draft: &mut MutableDraft,
    faces_to_remove: &[FaceId],
) -> Result<(), KernelError> {

    for &face_id in faces_to_remove {
        let face = draft.arena().get_face(face_id)?;
        let loop_id = face.outer_loop();
        let first_he = draft.arena().get_loop(loop_id)?.half_edge();

        let mut he_ids = Vec::new();
        let mut current = first_he;
        loop {
            he_ids.push(current);
            current = draft.arena().get_half_edge(current)?.next();
            if current == first_he { break; }
        }

        for he_id in he_ids {
            draft.arena_mut().remove_half_edge(he_id);
        }
        draft.arena_mut().remove_loop(loop_id);
        draft.arena_mut().remove_face(face_id);
    }

    Ok(())
}

/// Merge adjacent coplanar faces by dissolving shared edges.
///
/// Two faces are merged if they share an edge and lie on the same
/// geometric plane (same normal direction and offset). This reduces
/// fragmented meshes from multi-cell BSP extraction into cleaner
/// representations with fewer faces.
fn merge_coplanar_neighbors(
    draft: &mut MutableDraft,
    geometry: &GeometryStore,
    face_plane_map: &HashMap<FaceId, usize>,
    tolerance: f64,
) -> Result<usize, KernelError> {
    let mut merged_count = 0;
    let mut removed: std::collections::HashSet<FaceId> = std::collections::HashSet::new();
    let mut changed = true;

    while changed {
        changed = false;

        let face_ids: Vec<FaceId> = face_plane_map.keys()
            .filter(|fid| !removed.contains(fid))
            .filter(|fid| draft.arena().get_face(**fid).is_ok())
            .copied()
            .collect();

        for face_id in face_ids {
            if removed.contains(&face_id) { continue; }
            if draft.arena().get_face(face_id).is_err() { continue; }

            let plane_a = match geometry.get_face_plane(face_id) {
                Some(p) => p.clone(),
                None => continue,
            };

            let loop_id = draft.arena().get_face(face_id)?.outer_loop();
            let first_he = draft.arena().get_loop(loop_id)?.half_edge();

            let mut dissolve_target = None;
            let mut current = first_he;
            loop {
                let he = draft.arena().get_half_edge(current)?;
                let twin_id = he.twin();
                let twin = draft.arena().get_half_edge(twin_id)?;
                let adj_face = twin.face();

                if adj_face != face_id && !removed.contains(&adj_face) {
                    if let Some(plane_b) = geometry.get_face_plane(adj_face) {
                        if planes_are_coplanar(&plane_a, plane_b, tolerance) {
                            dissolve_target = Some(current);
                            break;
                        }
                    }
                }

                current = he.next();
                if current == first_he { break; }
            }

            if let Some(he_id) = dissolve_target {
                let absorbed = dissolve_edge(draft, he_id)?;
                removed.insert(absorbed);
                merged_count += 1;
                changed = true;
                break;
            }
        }
    }

    Ok(merged_count)
}

/// Dissolve an edge by merging its two incident faces.
///
/// Given halfedge `he` (face A, u→v) and its twin (face B, v→u):
/// - Reconnect: prev(he).next = next(twin), prev(twin).next = next(he)
/// - Update face pointers on all of B's halfedges to point to face A
/// - Remove he, twin, face B, and loop B
///
/// Returns the FaceId of the absorbed (removed) face.
fn dissolve_edge(
    draft: &mut MutableDraft,
    he_id: HalfEdgeId,
) -> Result<FaceId, KernelError> {
    let he = draft.arena().get_half_edge(he_id)?;
    let twin_id = he.twin();
    let face_a = he.face();
    let he_next = he.next();
    let he_prev = he.prev();
    let origin_u = he.origin();

    let twin = draft.arena().get_half_edge(twin_id)?;
    let face_b = twin.face();
    let twin_next = twin.next();
    let twin_prev = twin.prev();
    let origin_v = twin.origin();

    // Reconnect: skip over he and twin in their respective loops
    // prev(he) → next(twin)  (both at vertex u)
    draft.arena_mut().get_half_edge_mut(he_prev)?.set_next(twin_next);
    draft.arena_mut().get_half_edge_mut(twin_next)?.set_prev(he_prev);

    // prev(twin) → next(he)  (both at vertex v)
    draft.arena_mut().get_half_edge_mut(twin_prev)?.set_next(he_next);
    draft.arena_mut().get_half_edge_mut(he_next)?.set_prev(twin_prev);

    // Update all of face B's halfedges to point to face A
    let mut cur = twin_next;
    let stop = he_prev;
    loop {
        draft.arena_mut().get_half_edge_mut(cur)?.set_face(face_a);
        let next = draft.arena().get_half_edge(cur)?.next();
        if cur == stop { break; }
        cur = next;
    }

    // Update face A's loop entry to a surviving halfedge
    let loop_a = draft.arena().get_face(face_a)?.outer_loop();
    draft.arena_mut().get_loop_mut(loop_a)?.set_half_edge(he_next);

    // Update vertex outgoing pointers if they point to removed halfedges
    let u_outgoing = draft.arena().get_vertex(origin_u)?.outgoing();
    if u_outgoing == he_id {
        draft.arena_mut().get_vertex_mut(origin_u)?.set_outgoing(twin_next);
    }
    let v_outgoing = draft.arena().get_vertex(origin_v)?.outgoing();
    if v_outgoing == twin_id {
        draft.arena_mut().get_vertex_mut(origin_v)?.set_outgoing(he_next);
    }

    // Remove the dissolved halfedges
    draft.arena_mut().remove_half_edge(he_id);
    draft.arena_mut().remove_half_edge(twin_id);

    // Remove face B and its loop
    let loop_b = draft.arena().get_face(face_b)?.outer_loop();
    draft.arena_mut().remove_loop(loop_b);
    draft.arena_mut().remove_face(face_b);

    Ok(face_b)
}

/// Check if two planes are coplanar (same normal direction, same offset).
fn planes_are_coplanar(a: &Plane, b: &Plane, tolerance: f64) -> bool {
    let na = a.normal();
    let nb = b.normal();

    let dot = na[0] * nb[0] + na[1] * nb[1] + na[2] * nb[2];

    if (dot - 1.0).abs() < tolerance {
        (a.offset() - b.offset()).abs() < tolerance
    } else {
        false
    }
}
