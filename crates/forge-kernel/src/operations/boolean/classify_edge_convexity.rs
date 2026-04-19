//! Classify edge convexity for fillet and chamfer operations.
//!
//! DOMAIN: Determines whether an edge boundary is convex (ridge), concave
//! (valley), or smooth (coplanar adjacent faces) relative to the solid
//! interior. Geometry is received via a `plane_fn` callback — GeometryState
//! is never imported here (architecture §2).
//!
//! Half-edge traversal uses `forge-topo` directly (structural, no geometry).
//! The face plane lookup goes through the callback, which the caller wires
//! to `GeometryState::get_face_plane` without surfacing the store type.
//!
//! POLICY REQUIREMENTS: NearTangency (declared in step contract).
//!
//! DEPENDENCIES: forge-topo (handles, arena), worth-geom (Plane)

use forge_core::KernelError;
use forge_topo::b_rep::TopologyArena;
use forge_topo::handles::{FaceId, HalfEdgeId};
use worth_geom::facade::Plane;

/// Convexity classification for a boundary edge.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EdgeConvexity {
    /// Adjacent face normals form a convex dihedral (exterior angle < 180°).
    /// Fillets here add material.
    Convex,
    /// Adjacent face normals form a concave dihedral (exterior angle > 180°).
    /// Fillets here remove material.
    Concave,
    /// Adjacent faces are coplanar within tolerance. Not a valid fillet edge.
    Smooth,
}

/// Classify the convexity of an edge by comparing its two adjacent face planes.
///
/// `half_edge` is one of the two halfedges of the edge to classify.
/// Its radial partner (the "twin") is obtained via `.radial_next()`.
///
/// Geometry is accessed through `plane_fn` — no `GeometryState` import.
/// The caller wires this as: `|f| geom.get_face_plane(f).cloned()`.
///
/// # Parameters
/// - `half_edge` — one face-use of the edge
/// - `arena` — topology arena (read-only, forge-topo)
/// - `plane_fn` — resolve `FaceId` → `Plane` (caller provides via closure)
/// - `tangency_tol` — sine-of-dihedral threshold for "smooth" judgment
pub fn classify_edge_convexity(
    half_edge: HalfEdgeId,
    arena: &TopologyArena,
    plane_fn: &dyn Fn(FaceId) -> Option<Plane>,
    tangency_tol: f64,
) -> Result<EdgeConvexity, KernelError> {
    let he_data = arena
        .get_half_edge(half_edge)
        .map_err(|_| KernelError::InvalidInput {
            message: format!(
                "classify_edge_convexity: halfedge {} not alive",
                half_edge.index()
            ),
            context: None,
        })?;

    let face_a = he_data.face();

    // The "twin" in a radial-edge half-edge mesh is the radial partner.
    // For a manifold edge, radial_next() gives the other face-use; for a
    // boundary edge, radial_next() == self (self-radial sentinel).
    let twin_id = he_data.radial_next();
    if twin_id == half_edge {
        return Err(KernelError::InvalidInput {
            message: format!(
                "classify_edge_convexity: halfedge {} is a boundary edge (self-radial)",
                half_edge.index()
            ),
            context: None,
        });
    }

    let twin_data = arena
        .get_half_edge(twin_id)
        .map_err(|_| KernelError::InternalError {
            message: format!(
                "classify_edge_convexity: radial partner {} not alive",
                twin_id.index()
            ),
            context: None,
        })?;

    let face_b = twin_data.face();

    let plane_a = plane_fn(face_a).ok_or_else(|| KernelError::InternalError {
        message: format!(
            "classify_edge_convexity: face {} has no plane binding",
            face_a.index()
        ),
        context: None,
    })?;

    let plane_b = plane_fn(face_b).ok_or_else(|| KernelError::InternalError {
        message: format!(
            "classify_edge_convexity: face {} has no plane binding",
            face_b.index()
        ),
        context: None,
    })?;

    classify_dihedral(&plane_a, &plane_b, tangency_tol)
}

/// Compute convexity from two face planes using their cached f64 normals.
fn classify_dihedral(
    plane_a: &Plane,
    plane_b: &Plane,
    tangency_tol: f64,
) -> Result<EdgeConvexity, KernelError> {
    let na = plane_a.normal();
    let nb = plane_b.normal();
    let (sin_angle, dot) = worth_geom::facade::dihedral_sine(&na, &nb);

    if sin_angle < tangency_tol {
        return Ok(EdgeConvexity::Smooth);
    }

    // dot < 0 → normals point away from each other → convex ridge.
    // dot > 0 → normals curve toward each other → concave valley.
    if dot < 0.0 {
        Ok(EdgeConvexity::Convex)
    } else {
        Ok(EdgeConvexity::Concave)
    }
}
