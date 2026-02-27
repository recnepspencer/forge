//! Face normal alignment queries.
//!
//! DOMAIN: Compute whether two faces have aligned (same-direction) or
//! opposing normals. Used by:
//! - Boolean classification: OnBoundary vs OppositeBoundary disambiguation
//! - Fillet/Chamfer: tangency direction at shared face boundaries
//! - Shell: offset direction consistency checks
//!
//! INVARIANT: All comparisons delegate to `geom_facade::normals_aligned_exact`
//! which uses exact rational arithmetic (D3-compliant — no `f64` comparison).

use crate::geom_facade;
use crate::geometry_state::GeometryState;
use forge_topo::handles::FaceId;

/// True when two faces have normals pointing in the same direction.
///
/// Fetches the plane for each face from the supplied `GeometryState` instances
/// and delegates to `geom_facade::normals_aligned_exact` for exact rational
/// dot-product sign evaluation.
///
/// Returns `true` when either plane is missing (conservative default — avoids
/// spurious OppositeBoundary classifications on incomplete geometry).
pub fn faces_have_aligned_normals(
    geom_a: &GeometryState,
    face_a: FaceId,
    geom_b: &GeometryState,
    face_b: FaceId,
) -> bool {
    match (geom_a.get_face_plane(face_a), geom_b.get_face_plane(face_b)) {
        (Some(pa), Some(pb)) => geom_facade::normals_aligned_exact(pa, pb),
        _ => true,
    }
}
