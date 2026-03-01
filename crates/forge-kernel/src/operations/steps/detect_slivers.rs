//! Detect sliver faces below the area threshold.
//!
//! DOMAIN: Scans all faces and returns those whose 3D area falls below
//! `sliver_threshold`. Area is computed from vertex positions via the
//! `forge-spatial` callback pattern — GeometryState is never imported here.
//!
//! POLICY REQUIREMENTS: SliverFace (declared in step contract).
//!
//! DEPENDENCIES: forge-spatial (face_bounds), forge-topo (handles, state)

use forge_core::KernelError;
use crate::spatial::face_bounds;
use forge_topo::handles::{FaceId, VertexId};
use forge_topo::transactions::TopologyState;

/// A face whose area falls below the sliver threshold.
#[derive(Debug, Clone)]
pub struct SliverFace {
    /// The offending face handle.
    pub face: FaceId,
    /// Computed 3D area of the face (mm² or model units²).
    pub area: f64,
    /// Threshold below which the face is classified as a sliver.
    pub threshold: f64,
}

/// Scan all faces and return those with 3D area below `sliver_threshold`.
///
/// Area is computed from f64 vertex position approximations via
/// `position_fn` — never from `GeometryState` directly (architecture §2).
/// The threshold comparison uses the supplied value from `ToleranceConfig`
/// (no inline magic numbers — Architecture Rule §4.1).
///
/// # Parameters
/// - `topo` — topology snapshot (read-only)
/// - `position_fn` — maps `VertexId` → position in model space
/// - `sliver_threshold` — area threshold in model units²
pub fn detect_slivers(
    topo: &TopologyState,
    position_fn: &dyn Fn(VertexId) -> Option<[f64; 3]>,
    sliver_threshold: f64,
) -> Result<Vec<SliverFace>, KernelError> {
    let mut slivers = Vec::new();
    let arena = topo.arena();

    for (face_id, _) in arena.iter_faces() {
        // forge-spatial computes the AABB for this face via position_fn.
        // We derive area from the AABB diagonal as a conservative lower bound.
        // For planar convex quads this is exact; for general polygons it is
        // an overestimate — sliver detection is a warning, not a hard reject.
        let area = compute_face_area_via_bounds(arena, position_fn, face_id)?;
        if area < sliver_threshold {
            slivers.push(SliverFace {
                face: face_id,
                area,
                threshold: sliver_threshold,
            });
        }
    }

    Ok(slivers)
}

/// Compute a face area estimate using fan triangulation over vertex positions.
///
/// Uses `forge-topo`'s `FaceAllEdgesIterator` for loop traversal,
/// and `position_fn` for vertex positions — no direct geometry store access.
fn compute_face_area_via_bounds(
    arena: &forge_topo::b_rep::TopologyArena,
    position_fn: &dyn Fn(VertexId) -> Option<[f64; 3]>,
    face_id: FaceId,
) -> Result<f64, KernelError> {
    use forge_topo::queries::traverse::FaceAllEdgesIterator;

    let mut positions: Vec<[f64; 3]> = Vec::new();

    for he_res in FaceAllEdgesIterator::new(arena, face_id)? {
        let he_id = he_res?;
        let vertex_id = arena.get_half_edge(he_id)?.origin();
        if let Some(pos) = position_fn(vertex_id) {
            positions.push(pos);
        }
    }

    if positions.len() < 3 {
        return Ok(0.0);
    }

    // Fan triangulation from first vertex.
    let p0 = positions[0];
    let mut total = 0.0_f64;
    for i in 1..positions.len() - 1 {
        total += triangle_area(&p0, &positions[i], &positions[i + 1]);
    }
    Ok(total)
}

/// Area of a triangle via cross-product magnitude.
fn triangle_area(a: &[f64; 3], b: &[f64; 3], c: &[f64; 3]) -> f64 {
    let ab = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
    let ac = [c[0] - a[0], c[1] - a[1], c[2] - a[2]];
    let cross = [
        ab[1] * ac[2] - ab[2] * ac[1],
        ab[2] * ac[0] - ab[0] * ac[2],
        ab[0] * ac[1] - ab[1] * ac[0],
    ];
    0.5 * (cross[0] * cross[0] + cross[1] * cross[1] + cross[2] * cross[2]).sqrt()
}
