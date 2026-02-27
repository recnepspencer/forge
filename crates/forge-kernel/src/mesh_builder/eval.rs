//! ConvexCell → halfedge mesh conversion algorithm.
//!
//! Builds a complete halfedge mesh from a BSP ConvexCell by:
//! 1. Creating all vertices via direct arena insertion (with spatial dedup)
//! 2. Building face loops with properly wired halfedges
//! 3. Stitching twins between adjacent faces via shared-edge matching
//! 4. Registering face planes and vertex positions in GeometryState

use forge_core::{DecisionKind, KernelError};
use crate::geom_facade::ConvexCell;
use forge_topo::arena::{
    BodyData, EdgeData, FaceData, HalfEdgeData, LoopData, LumpData, RegionData, ShellData,
    ShellKind, ShellOrientation, VertexData,
};
use forge_topo::handles::{EdgeId, HalfEdgeId, LoopId, ShellId, VertexId};
use forge_topo::state::{MutableDraft, TopologyState};

use crate::brep::state::BrepState;
use crate::check_tolerance;
use crate::core::ModelingContext;
use crate::geometry_state::GeometryState;

/// Result of building a halfedge mesh from a ConvexCell.
pub struct MeshBuildResult {
    /// The committed topology state.
    topology: TopologyState,
    /// The associated geometry store.
    geometry: GeometryState,
    /// The associated B-Rep data.
    brep: BrepState,
}

impl MeshBuildResult {
    /// The committed topology state.
    pub fn topology(&self) -> &TopologyState {
        &self.topology
    }

    /// The associated geometry store.
    pub fn geometry(&self) -> &GeometryState {
        &self.geometry
    }

    /// The associated B-Rep data.
    pub fn brep(&self) -> &BrepState {
        &self.brep
    }

    /// Consume and return parts.
    pub fn into_parts(self) -> (TopologyState, GeometryState, BrepState) {
        (self.topology, self.geometry, self.brep)
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
pub fn build_halfedge_mesh(
    cell: &ConvexCell,
    ctx: &mut ModelingContext,
) -> Result<MeshBuildResult, KernelError> {
    validate_cell(cell)?;

    let tolerance = ctx.get_tolerance().get_spatial_tolerance();

    let state = TopologyState::empty();
    let mut draft = state.into_mutation();
    let mut geometry = GeometryState::new();

    let vertex_ids = insert_vertices(&mut draft, &mut geometry, cell, tolerance, ctx)?;

    let body = draft.insert_body(BodyData::new());
    let lump = draft.insert_lump(LumpData::new(body));
    let region = draft.insert_region(RegionData::new(lump));
    draft.arena_mut().get_body_mut(body)?.add_lump(lump);
    draft.arena_mut().get_lump_mut(lump)?.add_region(region);
    let shell = draft.insert_shell(ShellData::new(
        forge_topo::handles::FaceId::from_raw_parts(u32::MAX, 0),
        ShellKind::Solid(ShellOrientation::Outer),
        region,
    ));
    draft.arena_mut().get_region_mut(region)?.add_shell(shell);

    let edge_map = insert_faces_and_loops(&mut draft, &mut geometry, cell, &vertex_ids, shell)?;

    stitch_twins(&mut draft, &edge_map)?;

    let first_face = draft.arena().iter_faces().next().map(|(fid, _)| fid);
    if let Some(fid) = first_face {
        draft
            .arena_mut()
            .get_shell_mut(shell)?
            .set_representative_face(fid);
    }

    let topology = draft.commit()?;

    Ok(MeshBuildResult {
        topology,
        geometry,
        brep: BrepState::new(),
    })
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
    geometry: &mut GeometryState,
    cell: &ConvexCell,
    tolerance: f64,
    ctx: &mut ModelingContext,
) -> Result<Vec<VertexId>, KernelError> {
    let placeholder_he = HalfEdgeId::from_raw_parts(u32::MAX, 0);
    let mut vertex_ids = Vec::with_capacity(cell.vertex_count());
    let mut inserted: Vec<(VertexId, [f64; 3])> = Vec::with_capacity(cell.vertex_count());
    let cell_planes = cell.planes();

    for vert in cell.vertices() {
        let pos = *vert.position();

        let existing = find_coincident_vertex(&inserted, &pos, tolerance);

        if let Some((existing_vid, dist)) = existing {
            check_tolerance!(
                ctx,
                tolerance,
                dist,
                pos,
                DecisionKind::NearBoundary {
                    threshold: tolerance
                }
            );
            vertex_ids.push(existing_vid);
        } else {
            let vid = draft.insert_vertex(VertexData::new(placeholder_he));

            let [pa, pb, pc] = vert.plane_indices();
            let stored_exact = if pa < cell_planes.len()
                && pb < cell_planes.len()
                && pc < cell_planes.len()
            {
                match crate::geom_facade::intersect_three_planes_exact(
                    &cell_planes[pa],
                    &cell_planes[pb],
                    &cell_planes[pc],
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
        let diff = forge_math::linalg::sub(*pos, *existing_pos);
        let dist_sq = forge_math::linalg::norm_sq(diff);
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
    geometry: &mut GeometryState,
    cell: &ConvexCell,
    vertex_ids: &[VertexId],
    shell: ShellId,
) -> Result<EdgeMap, KernelError> {
    let placeholder_he = HalfEdgeId::from_raw_parts(u32::MAX, 0);
    let placeholder_loop = LoopId::from_raw_parts(u32::MAX, 0);
    let placeholder_edge = EdgeId::from_raw_parts(u32::MAX, 0);
    let cell_planes = cell.planes();

    let vertex_count = vertex_ids.len();
    let mut edge_map = EdgeMap::new(vertex_count);

    for cell_face in cell.faces() {
        let face_verts = cell_face.vertices();
        if face_verts.len() < 3 {
            continue;
        }

        let face_id = draft.insert_face(FaceData::new(placeholder_loop, shell));

        let loop_id = draft.insert_loop(LoopData::new(placeholder_he, face_id));

        let plane_idx = cell_face.plane_idx();
        if plane_idx < cell_planes.len() {
            geometry.set_face_plane(face_id, cell_planes[plane_idx].clone());
        }

        let vert_count = face_verts.len();
        let mut he_ids = Vec::with_capacity(vert_count);

        for &cell_vert_idx in face_verts {
            let origin = vertex_ids[cell_vert_idx];
            let he_id = draft.insert_half_edge(HalfEdgeData::new(
                placeholder_he,
                placeholder_he,
                placeholder_he,
                face_id,
                origin,
                placeholder_edge,
            ));
            he_ids.push(he_id);
        }

        for i in 0..vert_count {
            let next_i = (i + 1) % vert_count;
            let prev_i = if i == 0 { vert_count - 1 } else { i - 1 };

            let arena = draft.arena_mut();
            arena.get_half_edge_mut(he_ids[i])?.set_next(he_ids[next_i]);
            arena.get_half_edge_mut(he_ids[i])?.set_prev(he_ids[prev_i]);

            edge_map.insert(face_verts[i], face_verts[next_i], he_ids[i]);
        }

        draft
            .arena_mut()
            .get_face_mut(face_id)?
            .set_outer_loop(loop_id);
        draft
            .arena_mut()
            .get_loop_mut(loop_id)?
            .set_half_edge(he_ids[0]);

        for &he_id in &he_ids {
            let origin = draft.arena().get_half_edge(he_id)?.origin();
            draft
                .arena_mut()
                .get_vertex_mut(origin)?
                .set_outgoing(he_id);
        }
    }

    Ok(edge_map)
}

/// Dense bitmap mapping (vertex_a, vertex_b) → HalfEdgeId.
///
/// Flat Vec of size vertex_count², indexed by `a * n + b`.
/// O(1) insert/lookup, zero hash overhead, deterministic iteration order.
struct EdgeMap {
    data: Vec<Option<HalfEdgeId>>,
    vertex_count: usize,
}

impl EdgeMap {
    /// Create a new edge map for the given vertex count.
    fn new(vertex_count: usize) -> Self {
        Self {
            data: vec![None; vertex_count * vertex_count],
            vertex_count,
        }
    }

    /// Insert a directed edge.
    fn insert(&mut self, a: usize, b: usize, he: HalfEdgeId) {
        self.data[a * self.vertex_count + b] = Some(he);
    }

    /// Look up a directed edge.
    fn get(&self, a: usize, b: usize) -> Option<HalfEdgeId> {
        self.data[a * self.vertex_count + b]
    }

    /// Iterate all entries in deterministic ascending order by (a, b).
    fn iter_ascending(&self) -> impl Iterator<Item = (usize, usize, HalfEdgeId)> + '_ {
        self.data.iter().enumerate().filter_map(move |(idx, opt)| {
            opt.map(|he| {
                let a = idx / self.vertex_count;
                let b = idx % self.vertex_count;
                (a, b, he)
            })
        })
    }
}

/// Stitch twin pointers between halfedges on adjacent faces.
///
/// For each directed edge (a→b), find the matching (b→a) and set twins.
/// Iterates in deterministic ascending order by vertex-pair key.
fn stitch_twins(draft: &mut MutableDraft, edge_map: &EdgeMap) -> Result<(), KernelError> {
    for (a, b, he_id) in edge_map.iter_ascending() {
        if a < b {
            if let Some(twin_id) = edge_map.get(b, a) {
                let edge = draft.insert_edge(EdgeData::new(he_id));
                draft
                    .arena_mut()
                    .get_half_edge_mut(he_id)?
                    .set_radial_next(twin_id);
                draft
                    .arena_mut()
                    .get_half_edge_mut(twin_id)?
                    .set_radial_next(he_id);
                draft.arena_mut().get_half_edge_mut(he_id)?.set_edge(edge);
                draft.arena_mut().get_half_edge_mut(twin_id)?.set_edge(edge);
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
    }
    Ok(())
}
/// Build a convex solid from arbitrary planes.
///
/// General-purpose constructor: planes → BSP → halfedge mesh.
pub fn make_convex_solid(planes: Vec<crate::geom_facade::Plane>) -> Result<MeshBuildResult, KernelError> {
    let cell = crate::geom_facade::build_convex_polyhedron(
        &planes,
        &crate::geom_facade::BspConfig::default(),
    )?;
    let mut ctx = ModelingContext::new();
    build_halfedge_mesh(&cell, &mut ctx)
}

/// Create a cube centered at `center` with side length `size`.
pub fn make_cube(center: [f64; 3], size: f64) -> Result<MeshBuildResult, KernelError> {
    let planes = crate::geom_facade::shapes::cube(center, size / 2.0);
    make_convex_solid(planes)
}

/// Create a regular tetrahedron centered at `center` with the given `scale`.
pub fn make_tetrahedron(center: [f64; 3], scale: f64) -> Result<MeshBuildResult, KernelError> {
    let planes = crate::geom_facade::shapes::tetrahedron(center, scale);
    make_convex_solid(planes)
}

/// Create a regular dodecahedron centered at `center` with the given `scale`.
pub fn make_dodecahedron(center: [f64; 3], scale: f64) -> Result<MeshBuildResult, KernelError> {
    let planes = crate::geom_facade::shapes::dodecahedron(center, scale);
    make_convex_solid(planes)
}
