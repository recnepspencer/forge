//! Exact vertex-vs-plane sign classification for split operations.
//!
//! DOMAIN: Centralizes the explicit/symbolic/f64-promoted sign pipeline used by
//! split gating and cut-point discovery.

use forge_geom::primitives::implicit_vertex::{orient3d_symbolic, PlaneRef, Vertex};
use forge_geom::primitives::plane::{classify_point_exact, Plane};
use forge_math::arithmetic::Rational;
use forge_math::sign::TriSign;
use forge_topo::handles::VertexId;

use crate::geometry_store::GeometryStore;

/// Compute the exact sign of a vertex relative to a plane.
///
/// If the vertex has exact rational coordinates, uses them.
/// If the vertex is symbolic, evaluates the 4x4 determinant.
/// Fallback: promotes f64 to `Rational`.
pub fn exact_sign_for_vertex(
    geometry: &GeometryStore,
    vertex: VertexId,
    f64_pos: &[f64; 3],
    plane: &Plane,
    plane_idx: usize,
) -> TriSign {
    if let Some(exact) = geometry.get_vertex_position_exact(vertex) {
        return classify_point_exact(plane, exact);
    }

    if let Some(sym_refs) = geometry.get_vertex_symbolic_planes(vertex) {
        let sym_v = Vertex::try_new_symbolic([
            PlaneRef::new(sym_refs[0]),
            PlaneRef::new(sym_refs[1]),
            PlaneRef::new(sym_refs[2]),
        ]);
        if let Ok(sign) = orient3d_symbolic(&sym_v, PlaneRef::new(plane_idx), geometry) {
            return sign;
        }
    }

    if !f64_pos[0].is_finite() || !f64_pos[1].is_finite() || !f64_pos[2].is_finite() {
        return TriSign::Zero;
    }

    let promoted = [
        Rational::try_from_f64(f64_pos[0]).unwrap_or_else(|_| Rational::zero()),
        Rational::try_from_f64(f64_pos[1]).unwrap_or_else(|_| Rational::zero()),
        Rational::try_from_f64(f64_pos[2]).unwrap_or_else(|_| Rational::zero()),
    ];
    classify_point_exact(plane, &promoted)
}
