//! Harness oracles — independent geometric truth sources for test validation.
//!
//! DOMAIN: These oracles validate geometric properties (volume, centroid,
//! normal orientation) with explicit precondition checking. They wrap
//! production adapters with structural invariant checks and classify
//! all failure modes explicitly via `OracleError`.
//!
//! ARCHITECTURE: Oracles are thin precondition wrappers around the crate stack:
//! - Volume/centroid adapters: `forge_kernel::geometry::facade`
//! - Pure math: `forge_geom::algorithms::measurement`
//! - Normal classification: `forge_spatial::operations::classify::normal_orientation`
//! - Face normals: `forge_spatial::operations::continuity`
//!
//! Each oracle checks structural invariants first (closed manifold, complete
//! positions), then delegates computation to the proper layer.

use forge_core::KernelError;
use forge_topo::b_rep::TopologyArena;
use forge_topo::handles::{FaceId, VertexId};

use crate::engine::facade::SolidEnvelope;
use crate::geometry::facade::{
    GeometryView, GeometryToleranceProvider,
    solid_volume, solid_centroid, collect_face_positions,
};
use forge_spatial::operations::facade::classify_face_normal_orientation;
pub use forge_spatial::operations::facade::NormalClassification;

// ── Error types ──────────────────────────────────────────────────────────

/// Classified oracle error — every failure mode is explicit.
#[derive(Debug)]
pub enum OracleError {
    /// Shell is not closed — has boundary edges.
    OpenShell { boundary_edge_count: usize },
    /// Face normals are not consistently oriented.
    InconsistentOrientation { shell_index: usize },
    /// Face geometry is degenerate (e.g., collinear vertices, zero area).
    DegenerateGeometry { face_index: u32, reason: &'static str },
    /// Internal topology error during oracle computation.
    TopologyError(KernelError),
    /// Position data missing for a vertex.
    MissingPosition { vertex_index: u32 },
}

impl From<KernelError> for OracleError {
    fn from(e: KernelError) -> Self {
        OracleError::TopologyError(e)
    }
}

impl std::fmt::Display for OracleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OpenShell { boundary_edge_count } =>
                write!(f, "OpenShell: {boundary_edge_count} boundary edges"),
            Self::InconsistentOrientation { shell_index } =>
                write!(f, "InconsistentOrientation: shell {shell_index}"),
            Self::DegenerateGeometry { face_index, reason } =>
                write!(f, "DegenerateGeometry: face {face_index}: {reason}"),
            Self::TopologyError(e) =>
                write!(f, "TopologyError: {e}"),
            Self::MissingPosition { vertex_index } =>
                write!(f, "MissingPosition: vertex {vertex_index}"),
        }
    }
}

impl std::error::Error for OracleError {}

// ── Volume oracle ────────────────────────────────────────────────────────

/// Compute the volume of a closed, oriented solid.
///
/// **Preconditions** (actively checked):
/// - Closed manifold: `is_boundary_edge` returns false for all half-edges
/// - All vertices have positions
///
/// **Delegates to**: `solid_volume` → `forge_geom::polyhedron_volume`
pub fn volume_of(env: &SolidEnvelope) -> Result<f64, OracleError> {
    let arena = env.topology().arena();
    let geom = env.geometry();
    check_closed_manifold(arena)?;
    check_positions_complete(arena, geom)?;
    Ok(solid_volume(arena, geom))
}

/// Compute the volumetric centroid of a closed, oriented solid.
///
/// **Delegates to**: `solid_centroid` → `forge_geom::polyhedron_centroid`
pub fn centroid_of(env: &SolidEnvelope) -> Result<[f64; 3], OracleError> {
    let arena = env.topology().arena();
    let geom = env.geometry();
    check_closed_manifold(arena)?;
    check_positions_complete(arena, geom)?;
    solid_centroid(arena, geom).ok_or(OracleError::DegenerateGeometry {
        face_index: 0,
        reason: "total volume is near-zero, cannot compute centroid",
    })
}

// ── Normal classification oracle ─────────────────────────────────────────

/// Classify whether a face normal points outward.
///
/// Checks preconditions, then delegates entirely to
/// `forge_spatial::classify_face_normal_orientation`. See that function
/// for the algorithm, assumptions, and epsilon constraints.
pub fn classify_normal_outward(
    env: &SolidEnvelope,
    face_id: FaceId,
    epsilon: f64,
) -> Result<NormalClassification, OracleError> {
    let arena = env.topology().arena();
    let geom = env.geometry();

    check_closed_manifold(arena)?;
    check_positions_complete(arena, geom)?;

    // Build position table for classify_point_in_solid (raw u32 slot index)
    let positions = build_position_table(arena, geom);
    let position_fn = |idx: u32| -> Result<[f64; 3], KernelError> {
        positions
            .get(idx as usize)
            .copied()
            .flatten()
            .ok_or_else(|| KernelError::InvalidInput {
                message: format!("missing position for vertex slot {idx}"),
                context: None,
            })
    };

    // Typed position lookup for Newell normal (VertexId handles)
    let face_position_fn = |v: VertexId| geom.get_vertex_position(v).copied();

    // Face vertex positions for centroid
    let face_verts = collect_face_positions(arena, geom, face_id);

    // Geometry-derived tolerance
    let tol = GeometryToleranceProvider::new(geom);

    // Delegate to forge-spatial
    let result = classify_face_normal_orientation(
        arena, &position_fn, &face_position_fn, &face_verts, face_id, epsilon, &tol,
    )?;

    Ok(result)
}

// ── Precondition checks ──────────────────────────────────────────────────

/// Check that every edge is exactly 2-manifold (closed, no non-manifold pinches).
fn check_closed_manifold(arena: &TopologyArena) -> Result<(), OracleError> {
    let mut boundary_count = 0;
    let mut non_manifold_count = 0;
    
    // We iterate halfedges, but we only want to check the geometric edge once.
    // So we just check radial_valence for all halfedges and divide by 2 later, 
    // or just check valence and flag any deviation.
    for (he_id, _he) in arena.iter_half_edges() {
        let valence = forge_topo::queries::traverse::radial_valence(arena, he_id).unwrap_or(0);
        if valence == 1 {
            boundary_count += 1;
        } else if valence > 2 {
            non_manifold_count += 1;
        }
    }
    
    if boundary_count > 0 || non_manifold_count > 0 {
        return Err(OracleError::OpenShell { 
            // Halving because each edge contributes its valence number of half-edges,
            // but for errors just returning the raw half-edge count is fine for debugging.
            boundary_edge_count: boundary_count + non_manifold_count 
        });
    }
    Ok(())
}

/// Check all vertices have position data.
fn check_positions_complete(
    arena: &TopologyArena,
    geom: &impl GeometryView,
) -> Result<(), OracleError> {
    for (vid, _) in arena.iter_vertices() {
        if !geom.has_vertex_position(vid) {
            return Err(OracleError::MissingPosition { vertex_index: vid.index() });
        }
    }
    Ok(())
}

/// Build a position table indexed by raw vertex slot index.
///
/// `classify_point_in_solid` takes `Fn(u32) -> Result<[f64;3], KernelError>`.
/// The u32 is a raw slot index, not a generational handle. This table maps
/// each slot to its position (or None if empty/deleted).
fn build_position_table(
    arena: &TopologyArena,
    geom: &impl GeometryView,
) -> Vec<Option<[f64; 3]>> {
    let max_slot = arena
        .iter_vertices()
        .map(|(vid, _)| vid.index() as usize)
        .max()
        .unwrap_or(0);

    let mut table = vec![None; max_slot + 1];
    for (vid, _) in arena.iter_vertices() {
        if let Some(pos) = geom.get_vertex_position(vid) {
            table[vid.index() as usize] = Some(*pos);
        }
    }
    table
}
