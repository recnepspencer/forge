//! ConvexCell → halfedge mesh conversion algorithm.
//!
//! Builds a complete halfedge mesh from a BSP ConvexCell by:
//! 1. Creating all vertices via direct arena insertion (with spatial dedup)
//! 2. Building face loops with properly wired halfedges
//! 3. Stitching twins between adjacent faces via shared-edge matching
//! 4. Registering face planes and vertex positions in GeometryStore


use std::collections::HashMap;

use forge_core::{KernelError, DecisionKind};
use forge_geom::spatial::bsp::ConvexCell;
use forge_topo::arena::{FaceData, HalfEdgeData, VertexData, LoopData};
use forge_topo::handles::{HalfEdgeId, VertexId, LoopId};
use forge_topo::state::{TopologyState, MutableDraft};

use crate::check_tolerance;
use crate::core::ModelingContext;
use crate::geometry_store::GeometryStore;

/// Result of building a halfedge mesh from a ConvexCell.
pub struct MeshBuildResult {
    /// The committed topology state.
    topology: TopologyState,
    /// The associated geometry store.
    geometry: GeometryStore,
}

impl MeshBuildResult {
    /// The committed topology state.
    pub fn topology(&self) -> &TopologyState {
        &self.topology
    }

    /// The associated geometry store.
    pub fn geometry(&self) -> &GeometryStore {
        &self.geometry
    }

    /// Consume and return owned parts.
    pub fn into_parts(self) -> (TopologyState, GeometryStore) {
        (self.topology, self.geometry)
    }
}

/// Build a halfedge mesh from a BSP ConvexCell.
///
/// Uses direct arena insertion rather than Euler operators to ensure
/// correct face loops and twin stitching for arbitrary convex polyhedra.
///
/// Spatially-coincident vertices (within `ModelingContext.tolerance.spatial_tolerance`)
/// are deduplicated and logged as `DecisionKind::NearBoundary` (D2).
///
/// # Algorithm
///
/// 1. Insert all vertices from the ConvexCell (with spatial dedup)
/// 2. For each face, create a loop of halfedges connecting its vertices
/// 3. Stitch twin pointers by matching shared directed edges
/// 4. Register geometry (positions, planes)
pub fn build_halfedge_mesh(cell: &ConvexCell, ctx: &mut ModelingContext) -> Result<MeshBuildResult, KernelError> {
    validate_cell(cell)?;

    let tolerance = ctx.get_tolerance().get_spatial_tolerance();

    let state = TopologyState::empty();
    let mut draft = state.into_mutation();
    let mut geometry = GeometryStore::new();

    let vertex_ids = insert_vertices(&mut draft, &mut geometry, cell, tolerance, ctx)?;

    let edge_map = insert_faces_and_loops(&mut draft, &mut geometry, cell, &vertex_ids)?;

    stitch_twins(&mut draft, &edge_map)?;

    let topology = draft.commit()?;

    Ok(MeshBuildResult { topology, geometry })
}

/// Validate that the ConvexCell has enough structure for a valid polyhedron.
fn validate_cell(cell: &ConvexCell) -> Result<(), KernelError> {
    if cell.face_count() < 4 {
        return Err(KernelError::InvalidInput {
            message: format!(
                "ConvexCell needs at least 4 faces for a polyhedron, got {}",
                cell.face_count()
            ),
            context: None,
        });
    }
    if cell.vertex_count() < 4 {
        return Err(KernelError::InvalidInput {
            message: format!(
                "ConvexCell needs at least 4 vertices for a polyhedron, got {}",
                cell.vertex_count()
            ),
            context: None,
        });
    }
    Ok(())
}

/// Insert all ConvexCell vertices into the arena and geometry store.
///
/// Performs position-based deduplication: if two BSP vertices resolve
/// to positions within `tolerance` of each other, the later one reuses
/// the earlier's `VertexId`. Each merge is logged via `check_tolerance!`.
///
/// Returns a mapping from ConvexCell vertex index to VertexId.
fn insert_vertices(
    draft: &mut MutableDraft,
    geometry: &mut GeometryStore,
    cell: &ConvexCell,
    tolerance: f64,
    ctx: &mut ModelingContext,
) -> Result<Vec<VertexId>, KernelError> {
    let placeholder_he = HalfEdgeId::from_raw_parts(u32::MAX, 0);
    let mut vertex_ids = Vec::with_capacity(cell.vertex_count());
    let mut inserted: Vec<(VertexId, [f64; 3])> = Vec::with_capacity(cell.vertex_count());

    for vert in cell.vertices() {
        let pos = *vert.position();

        let existing = find_coincident_vertex(&inserted, &pos, tolerance);

        if let Some((existing_vid, dist)) = existing {
            check_tolerance!(ctx, tolerance, dist, pos, DecisionKind::NearBoundary { threshold: tolerance });
            vertex_ids.push(existing_vid);
        } else {
            let vid = draft.arena_mut().insert_vertex(VertexData::new(
                placeholder_he,
            ));
            geometry.set_vertex_position(vid, pos);
            inserted.push((vid, pos));
            vertex_ids.push(vid);
        }
    }

    Ok(vertex_ids)
}

/// Find an already-inserted vertex within `tolerance` of `pos`.
///
/// Returns the matching VertexId and the distance if found.
fn find_coincident_vertex(
    inserted: &[(VertexId, [f64; 3])],
    pos: &[f64; 3],
    tolerance: f64,
) -> Option<(VertexId, f64)> {
    let tol_sq = tolerance * tolerance;
    for (vid, existing_pos) in inserted {
        let dx = pos[0] - existing_pos[0];
        let dy = pos[1] - existing_pos[1];
        let dz = pos[2] - existing_pos[2];
        let dist_sq = dx * dx + dy * dy + dz * dz;
        if dist_sq < tol_sq {
            return Some((*vid, dist_sq.sqrt()));
        }
    }
    None
}

/// Create faces, loops, and halfedge chains for each ConvexCell face.
///
/// Each face's vertex list forms a closed loop of halfedges.
/// Returns a directed-edge map: (cell_vert_a, cell_vert_b) → HalfEdgeId,
/// used by [`stitch_twins`] to pair up twin halfedges.
fn insert_faces_and_loops(
    draft: &mut MutableDraft,
    geometry: &mut GeometryStore,
    cell: &ConvexCell,
    vertex_ids: &[VertexId],
) -> Result<HashMap<(usize, usize), HalfEdgeId>, KernelError> {
    let placeholder_he = HalfEdgeId::from_raw_parts(u32::MAX, 0);
    let placeholder_loop = LoopId::from_raw_parts(u32::MAX, 0);
    let cell_planes = cell.planes();

    let mut edge_map: HashMap<(usize, usize), HalfEdgeId> = HashMap::new();

    for cell_face in cell.faces() {
        let face_verts = cell_face.vertices();
        if face_verts.len() < 3 {
            continue;
        }

        let face_id = draft.arena_mut().insert_face(FaceData::new(
            placeholder_loop,
        ));

        let loop_id = draft.arena_mut().insert_loop(LoopData::new(
            placeholder_he,
            face_id,
        ));

        let plane_idx = cell_face.plane_idx();
        if plane_idx < cell_planes.len() {
            geometry.set_face_plane(face_id, cell_planes[plane_idx].clone());
        }

        let vert_count = face_verts.len();
        let mut he_ids = Vec::with_capacity(vert_count);

        for &cell_vert_idx in face_verts {
            let origin = vertex_ids[cell_vert_idx];
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

            edge_map.insert(
                (face_verts[i], face_verts[next_i]),
                he_ids[i],
            );
        }

        draft.arena_mut().get_face_mut(face_id)?.set_outer_loop(loop_id);
        draft.arena_mut().get_loop_mut(loop_id)?.set_half_edge(he_ids[0]);

        for &he_id in &he_ids {
            let origin = draft.arena().get_half_edge(he_id)?.origin();
            draft.arena_mut().get_vertex_mut(origin)?.set_outgoing(he_id);
        }
    }

    Ok(edge_map)
}

/// Stitch twin pointers between halfedges on adjacent faces.
///
/// For each directed edge (a→b), find the matching (b→a) and set twins.
fn stitch_twins(
    draft: &mut MutableDraft,
    edge_map: &HashMap<(usize, usize), HalfEdgeId>,
) -> Result<(), KernelError> {
    for (&(a, b), &he_id) in edge_map {
        if let Some(&twin_id) = edge_map.get(&(b, a)) {
            draft.arena_mut().get_half_edge_mut(he_id)?.set_twin(twin_id);
        } else {
            return Err(KernelError::InternalError {
                message: format!(
                    "No twin found for directed edge ({} -> {}); mesh is not closed",
                    a, b
                ),
                context: None,
            });
        }
    }
    Ok(())
}
/// Create a cube centered at `center` with side length `size`.
pub fn make_cube(
    center: [f64; 3],
    size: f64,
) -> Result<MeshBuildResult, KernelError> {
    let half_size = size / 2.0;

    let planes = vec![
        forge_geom::Plane::from_point_normal(
            [center[0] + half_size, center[1], center[2]],
            [1.0, 0.0, 0.0],
        )?,
        forge_geom::Plane::from_point_normal(
            [center[0] - half_size, center[1], center[2]],
            [-1.0, 0.0, 0.0],
        )?,
        forge_geom::Plane::from_point_normal(
            [center[0], center[1] + half_size, center[2]],
            [0.0, 1.0, 0.0],
        )?,
        forge_geom::Plane::from_point_normal(
            [center[0], center[1] - half_size, center[2]],
            [0.0, -1.0, 0.0],
        )?,
        forge_geom::Plane::from_point_normal(
            [center[0], center[1], center[2] + half_size],
            [0.0, 0.0, 1.0],
        )?,
        forge_geom::Plane::from_point_normal(
            [center[0], center[1], center[2] - half_size],
            [0.0, 0.0, -1.0],
        )?,
    ];

    let cell = forge_geom::spatial::bsp::build_convex_polyhedron(&planes, &forge_geom::spatial::bsp::BspConfig::default())?;
    let mut ctx = ModelingContext::new();

    build_halfedge_mesh(&cell, &mut ctx)
}
