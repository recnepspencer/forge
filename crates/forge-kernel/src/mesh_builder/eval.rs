//! ConvexCell → halfedge mesh conversion algorithm.
//!
//! Builds a complete halfedge mesh from a BSP ConvexCell by:
//! 1. Creating all vertices via direct arena insertion (with spatial dedup)
//! 2. Building face loops with properly wired halfedges
//! 3. Stitching twins between adjacent faces via shared-edge matching
//! 4. Registering face planes and vertex positions in GeometryState
//! 5. Validating geometry bindings post-commit

use forge_core::{DecisionKind, KernelError};
use crate::geom_facade::ConvexCell;
use forge_topo::b_rep::{
    BodyData, EdgeData, FaceData, HalfEdgeData, LoopData, LumpData, RegionData, ShellData,
    ShellKind, ShellOrientation, VertexData,
};
use forge_topo::handles::{EdgeId, HalfEdgeId, LoopId, ShellId, VertexId};
use forge_topo::lineage::OpSignature;
use forge_topo::transactions::{MutableDraft, TopologyState};

use crate::brep::state::BrepState;
use crate::check_tolerance;
use crate::core::config::resolve::ResolvedConfig;
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
/// Spatially-coincident vertices (within `config.config().tolerance.spatial_tolerance`)
/// are deduplicated and logged as `DecisionKind::NearBoundary` (D2).
///
/// # Algorithm
///
/// 1. Insert all vertices from the ConvexCell (with spatial dedup)
/// 2. For each face, create a loop of halfedges connecting its vertices
/// 3. Stitch twin pointers by matching shared directed edges
/// 4. Register geometry (positions, planes)
/// 5. Validate geometry bindings (every face has a plane, every vertex a position)
pub fn build_halfedge_mesh(
    cell: &ConvexCell,
    config: &ResolvedConfig,
) -> Result<MeshBuildResult, KernelError> {
    validate_cell(cell)?;

    let tolerance = config.scaled_vertex_tolerance();
    let mut ctx = ModelingContext::new();
    let sig = OpSignature::new("build_halfedge_mesh");
    let mut ordinal: u64 = 0;

    let state = TopologyState::empty();
    let mut draft = state.into_mutation();
    let mut geometry = GeometryState::new();

    let vertex_ids = insert_vertices(
        &mut draft, &mut geometry, cell, tolerance, &mut ctx, &sig, &mut ordinal,
    )?;

    let body = draft.insert_body(BodyData::new());
    ordinal += 1;
    let lump = draft.insert_lump(LumpData::new(body));
    ordinal += 1;
    let region = draft.insert_region(RegionData::new(lump));
    ordinal += 1;
    draft.arena_mut().get_body_mut(body)?.add_lump(lump);
    draft.arena_mut().get_lump_mut(lump)?.add_region(region);
    let shell = draft.insert_shell(ShellData::new(
        forge_topo::handles::FaceId::new(u32::MAX, 0),
        ShellKind::Solid(ShellOrientation::Outer),
        region,
    ));
    ordinal += 1;
    draft.arena_mut().get_region_mut(region)?.add_shell(shell);

    let edge_map = insert_faces_and_loops(
        &mut draft, &mut geometry, cell, &vertex_ids, shell, &sig, &mut ordinal,
    )?;

    stitch_twins(&mut draft, &edge_map, &sig, &mut ordinal)?;

    let first_face = draft.arena().iter_faces().next().map(|(fid, _)| fid);
    if let Some(fid) = first_face {
        draft
            .arena_mut()
            .get_shell_mut(shell)?
            .set_representative_face(fid);
    }

    let topology = draft.commit()?;

    geometry.validate_geometry_bindings(topology.arena())?;
    geometry.validate_geometry_completeness(topology.arena())?;

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
    sig: &OpSignature,
    ordinal: &mut u64,
) -> Result<Vec<VertexId>, KernelError> {
    let placeholder_he = HalfEdgeId::new(u32::MAX, 0);
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
            *ordinal += 1;

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
/// Uses the geometry facade's `is_same_point_within` for coincidence
/// detection (L∞ per-axis check). Computes L2 distance only in the
/// merge path for decision logging.
fn find_coincident_vertex(
    inserted: &[(VertexId, [f64; 3])],
    pos: &[f64; 3],
    tolerance: f64,
) -> Option<(VertexId, f64)> {
    for (vid, existing_pos) in inserted {
        if crate::geom_facade::is_same_point_within(pos, existing_pos, tolerance) {
            let dx = pos[0] - existing_pos[0];
            let dy = pos[1] - existing_pos[1];
            let dz = pos[2] - existing_pos[2];
            let dist = (dx * dx + dy * dy + dz * dz).sqrt();
            return Some((*vid, dist));
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
    sig: &OpSignature,
    ordinal: &mut u64,
) -> Result<EdgeMap, KernelError> {
    let placeholder_he = HalfEdgeId::new(u32::MAX, 0);
    let placeholder_loop = LoopId::new(u32::MAX, 0);
    let placeholder_edge = EdgeId::new(u32::MAX, 0);
    let cell_planes = cell.planes();

    let vertex_count = vertex_ids.len();
    let mut edge_map = EdgeMap::new(vertex_count);

    for cell_face in cell.faces() {
        let face_verts = cell_face.vertices();
        if face_verts.len() < 3 {
            continue;
        }

        let face_id = draft.insert_face(FaceData::new(
            placeholder_loop, shell,
        ));
        *ordinal += 1;

        let loop_id = draft.insert_loop(LoopData::new(placeholder_he, face_id));

        let plane_idx = cell_face.plane_idx();
        if plane_idx < cell_planes.len() {
            geometry.set_face_plane(face_id, cell_planes[plane_idx].clone());
        }

        let vert_count = face_verts.len();
        let mut he_ids = Vec::with_capacity(vert_count);

        for &cell_vert_idx in face_verts {
            let origin = vertex_ids[cell_vert_idx];
            let he_id = draft.insert_half_edge(HalfEdgeData::new(placeholder_he, placeholder_he, placeholder_he, face_id, origin, placeholder_edge));
            *ordinal += 1;
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
fn stitch_twins(
    draft: &mut MutableDraft,
    edge_map: &EdgeMap,
    sig: &OpSignature,
    ordinal: &mut u64,
) -> Result<(), KernelError> {
    for (a, b, he_id) in edge_map.iter_ascending() {
        if a < b {
            if let Some(twin_id) = edge_map.get(b, a) {
                let edge = draft.insert_edge(EdgeData::new(
                    he_id,
                ));
                *ordinal += 1;
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

/// Safety margin: dimensions smaller than this multiple of the spatial
/// tolerance will be rejected as too small to produce reliable topology.
const DIMENSION_TOLERANCE_SAFETY_FACTOR: f64 = 10.0;

/// Validate a primitive dimension is usable.
///
/// - **Hard reject**: NaN, Inf, or ≤ 0.
/// - **Policy reject**: Finite positive but smaller than
///   `vertex_tolerance × DIMENSION_TOLERANCE_SAFETY_FACTOR`. The BSP
///   would merge all vertices and produce degenerate topology.
fn validate_dimension(
    value: f64,
    name: &str,
    config: &ResolvedConfig,
) -> Result<(), KernelError> {
    if value.is_nan() || value.is_infinite() {
        return Err(KernelError::InvalidInput {
            message: format!("{name} must be finite, got {value}"),
            context: None,
        });
    }
    if value <= 0.0 {
        return Err(KernelError::InvalidInput {
            message: format!("{name} must be > 0, got {value}"),
            context: None,
        });
    }

    let min_usable = config.scaled_vertex_tolerance() * DIMENSION_TOLERANCE_SAFETY_FACTOR;
    if value < min_usable {
        return Err(KernelError::InvalidInput {
            message: format!(
                "{name} = {value:.2e} is smaller than the minimum usable dimension \
                 ({min_usable:.2e} = {DIMENSION_TOLERANCE_SAFETY_FACTOR}× vertex tolerance). \
                 BSP would produce degenerate topology."
            ),
            context: None,
        });
    }
    Ok(())
}

/// Validate that a coordinate is finite (not NaN or ±Inf).
fn validate_coordinate(value: f64, name: &str) -> Result<(), KernelError> {
    if value.is_nan() || value.is_infinite() {
        return Err(KernelError::InvalidInput {
            message: format!("{name} must be finite, got {value}"),
            context: None,
        });
    }
    Ok(())
}

/// Validate center coordinates and a single size dimension.
fn validate_center_and_size(
    center: [f64; 3],
    size: f64,
    config: &ResolvedConfig,
) -> Result<(), KernelError> {
    validate_coordinate(center[0], "center[0]")?;
    validate_coordinate(center[1], "center[1]")?;
    validate_coordinate(center[2], "center[2]")?;
    validate_dimension(size, "size", config)
}

/// Build a convex solid from arbitrary planes.
///
/// General-purpose constructor: planes → BSP → halfedge mesh.
pub fn make_convex_solid(
    planes: Vec<crate::geom_facade::Plane>,
    config: &ResolvedConfig,
) -> Result<MeshBuildResult, KernelError> {
    let cell = crate::geom_facade::build_convex_polyhedron(
        &planes,
        &crate::geom_facade::BspConfig::default(),
    )?;
    build_halfedge_mesh(&cell, config)
}

/// Create a cube centered at `center` with side length `size`.
pub fn make_cube(
    center: [f64; 3],
    size: f64,
    config: &ResolvedConfig,
) -> Result<MeshBuildResult, KernelError> {
    validate_center_and_size(center, size, config)?;
    let planes = crate::geom_facade::shapes::cube(center, size / 2.0)?;
    make_convex_solid(planes, config)
}

/// Create a regular tetrahedron centered at `center` with the given `scale`.
pub fn make_tetrahedron(
    center: [f64; 3],
    scale: f64,
    config: &ResolvedConfig,
) -> Result<MeshBuildResult, KernelError> {
    validate_center_and_size(center, scale, config)?;
    let planes = crate::geom_facade::shapes::tetrahedron(center, scale)?;
    make_convex_solid(planes, config)
}

/// Create a regular dodecahedron centered at `center` with the given `scale`.
pub fn make_dodecahedron(
    center: [f64; 3],
    scale: f64,
    config: &ResolvedConfig,
) -> Result<MeshBuildResult, KernelError> {
    validate_center_and_size(center, scale, config)?;
    let planes = crate::geom_facade::shapes::dodecahedron(center, scale)?;
    make_convex_solid(planes, config)
}

/// Create an axis-aligned block with independent half-extents.
pub fn make_block(
    center: [f64; 3],
    half_extents: [f64; 3],
    config: &ResolvedConfig,
) -> Result<MeshBuildResult, KernelError> {
    validate_coordinate(center[0], "center[0]")?;
    validate_coordinate(center[1], "center[1]")?;
    validate_coordinate(center[2], "center[2]")?;
    validate_dimension(half_extents[0], "half_extents[0]", config)?;
    validate_dimension(half_extents[1], "half_extents[1]", config)?;
    validate_dimension(half_extents[2], "half_extents[2]", config)?;;
    let planes = crate::geom_facade::shapes::block(center, half_extents)?;
    make_convex_solid(planes, config)
}

/// Create a regular prism (n-gon extrusion) centered at `center`.
pub fn make_prism(
    center: [f64; 3],
    sides: u32,
    radius: f64,
    height: f64,
    config: &ResolvedConfig,
) -> Result<MeshBuildResult, KernelError> {
    validate_coordinate(center[0], "center[0]")?;
    validate_coordinate(center[1], "center[1]")?;
    validate_coordinate(center[2], "center[2]")?;
    validate_dimension(radius, "radius", config)?;
    validate_dimension(height, "height", config)?;;
    if sides < 3 {
        return Err(KernelError::InvalidInput {
            message: format!("prism needs at least 3 sides, got {sides}"),
            context: None,
        });
    }
    let planes = crate::geom_facade::shapes::prism(center, sides, radius, height)?;
    make_convex_solid(planes, config)
}

/// Create a regular pyramid (n-gon base with apex) centered at `center`.
pub fn make_pyramid(
    center: [f64; 3],
    sides: u32,
    radius: f64,
    height: f64,
    config: &ResolvedConfig,
) -> Result<MeshBuildResult, KernelError> {
    validate_coordinate(center[0], "center[0]")?;
    validate_coordinate(center[1], "center[1]")?;
    validate_coordinate(center[2], "center[2]")?;
    validate_dimension(radius, "radius", config)?;
    validate_dimension(height, "height", config)?;;
    if sides < 3 {
        return Err(KernelError::InvalidInput {
            message: format!("pyramid needs at least 3 sides, got {sides}"),
            context: None,
        });
    }
    let planes = crate::geom_facade::shapes::pyramid(center, sides, radius, height)?;
    make_convex_solid(planes, config)
}

/// Create a wedge (triangular cross-section extrusion) centered at `center`.
pub fn make_wedge(
    center: [f64; 3],
    dimensions: [f64; 3],
    config: &ResolvedConfig,
) -> Result<MeshBuildResult, KernelError> {
    validate_coordinate(center[0], "center[0]")?;
    validate_coordinate(center[1], "center[1]")?;
    validate_coordinate(center[2], "center[2]")?;
    validate_dimension(dimensions[0], "width", config)?;
    validate_dimension(dimensions[1], "depth", config)?;
    validate_dimension(dimensions[2], "height", config)?;
    let planes = crate::geom_facade::shapes::wedge(center, dimensions)?;
    make_convex_solid(planes, config)
}
