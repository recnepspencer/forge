//! Pure evaluation helpers for Boolean operations.
//!
//! Shared utilities used by split, classify, and assemble phases.
//! All functions are stateless and side-effect-free.

use forge_core::KernelError;
use forge_geom::plane::Plane;
use forge_math::linalg::{cross, norm_sq};
use forge_topo::arena::TopologyArena;
use forge_topo::handles::FaceId;
use forge_topo::traverse::face_edges;

use crate::geometry_store::GeometryStore;

/// Nanometer quantization scale for exact position-based deduplication.
///
/// Converts meters to integer nanometers (1e-9 m resolution).
/// Vertices computed from the same intersection formula quantize identically.
const QUANTIZE_SCALE: f64 = 1e9;

/// Squared cross-product threshold for parallel-plane detection.
///
/// Two planes are considered parallel when the squared magnitude of the
/// cross product of their normals falls below this value.
/// For unit normals, 1e-20 corresponds to an angle of ~1e-10 radians.
const PARALLEL_CROSS_SQ_THRESHOLD: f64 = 1e-20;

/// Quantize a 3D position to integer units for exact HashMap keys.
///
/// Adapts the scale factor to the coordinate magnitude to prevent i64
/// overflow. Uses nanometer resolution (1e9) for typical coordinates,
/// but coarsens when coordinates exceed ~9e9 (i64::MAX / 1e9).
pub fn quantize_position(pos: &[f64; 3]) -> [i64; 3] {
    let max_abs = pos[0].abs().max(pos[1].abs()).max(pos[2].abs());
    let safe_limit = (i64::MAX as f64) * 0.5;
    let scale = if max_abs * QUANTIZE_SCALE > safe_limit {
        safe_limit / max_abs
    } else {
        QUANTIZE_SCALE
    };
    [
        (pos[0] * scale).round() as i64,
        (pos[1] * scale).round() as i64,
        (pos[2] * scale).round() as i64,
    ]
}

/// Check if two planes are parallel (cross product of normals near zero).
///
/// Uses `forge_math::linalg::cross` and `norm_sq` rather than
/// inline arithmetic, and a named threshold constant.
pub fn planes_are_parallel(a: &Plane, b: &Plane) -> bool {
    let c = cross(a.normal(), b.normal());
    norm_sq(c) < PARALLEL_CROSS_SQ_THRESHOLD
}

/// Compute the centroid of a face by averaging its vertex positions.
///
/// Uses `forge_topo::traverse::face_edges` for loop traversal rather
/// than manual loop walking.
pub fn compute_face_centroid(
    arena: &TopologyArena,
    geometry: &GeometryStore,
    face: FaceId,
) -> Result<[f64; 3], KernelError> {
    let edges = face_edges(arena, face)?;

    let mut sum = [0.0_f64; 3];
    let count = edges.len() as f64;

    for he_id in &edges {
        let he_data = arena.get_half_edge(*he_id)?;
        let pos = geometry.get_vertex_position(he_data.origin).ok_or_else(|| {
            KernelError::InvalidInput {
                message: format!("No position for vertex {} during centroid computation", he_data.origin),
                context: None,
            }
        })?;

        sum[0] += pos[0];
        sum[1] += pos[1];
        sum[2] += pos[2];
    }

    let inv = 1.0 / count;
    Ok([sum[0] * inv, sum[1] * inv, sum[2] * inv])
}
