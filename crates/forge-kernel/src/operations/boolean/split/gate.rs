//! Gate logic: should a face be cut by a plane?
//!
//! DOMAIN: Determines IF a face needs cutting, separately from applying the cut.
//! DEPENDENCIES: forge_geom (intersection_line, clip_line), GeometryStore.
//! INVARIANTS: `compute_face_chord` is the SOLE gate for face cutting.
//!   No cut occurs without passing this gate first.

use forge_core::KernelError;
use forge_geom::primitives::plane::{Plane, classify_point_exact};
use forge_math::arithmetic::Rational;
use forge_math::sign::TriSign;
use forge_topo::arena::TopologyArena;
use forge_topo::handles::{FaceId, VertexId};
use forge_topo::traverse::FaceEdgeIterator;
use forge_geom::primitives::implicit_vertex::{Vertex, PlaneRef};
use forge_geom::primitives::implicit_vertex::orient3d_symbolic;

use crate::geometry_store::GeometryStore;
use crate::core::ToleranceConfig;

/// Gate: does the cut_plane produce an interior chord segment in this face?
///
/// Literature-correct gate (Thibault-Naylor/CGAL/Cherchi): compute the
/// intersection LINE of face_plane and cut_plane, then clip it to the face
/// polygon via Cyrus-Beck. Falls back to vertex sign-walk for degenerate
/// polygons from prior splits.
pub fn compute_face_chord(
    arena: &TopologyArena,
    geometry: &GeometryStore,
    face: FaceId,
    face_plane: &Plane,
    cut_plane: &Plane,
    config: &ToleranceConfig,
) -> Result<Option<([f64; 3], [f64; 3])>, KernelError> {
    if forge_geom::primitives::plane::are_parallel_exact(face_plane, cut_plane) {
        return Ok(None);
    }

    let chord = try_cyrus_beck_clip(arena, geometry, face, face_plane, cut_plane, config)?;
    if chord.is_some() {
        return Ok(chord);
    }

    try_sign_walk_fallback(arena, geometry, face, cut_plane, config)
}

/// Primary gate: Cyrus-Beck line clipping against the face polygon.
///
/// Computes the intersection line of the two planes, then clips it to the
/// face polygon. Tries both winding orientations since pre-split fragments
/// may have CW winding relative to the stored plane normal.
fn try_cyrus_beck_clip(
    arena: &TopologyArena,
    geometry: &GeometryStore,
    face: FaceId,
    face_plane: &Plane,
    cut_plane: &Plane,
    config: &ToleranceConfig,
) -> Result<Option<([f64; 3], [f64; 3])>, KernelError> {
    let fn_a = face_plane.normal();
    let fo_a = face_plane.offset();
    let fn_b = cut_plane.normal();
    let fo_b = cut_plane.offset();
    let min_chord = config.get_min_edge_length();

    let (line_pt, line_dir) = match forge_geom::compute_intersection_line(fn_a, fo_a, fn_b, fo_b, config.get_degeneracy()) {
        None => return Ok(None),
        Some(l) => l,
    };

    let verts = collect_face_positions(arena, geometry, face)?;
    if verts.len() < 3 {
        return Ok(None);
    }

    let chord = forge_geom::clip_line_to_face_polygon(line_pt, line_dir, &verts, fn_a, min_chord);
    if chord.is_some() {
        return Ok(chord);
    }

    let fn_a_neg = [-fn_a[0], -fn_a[1], -fn_a[2]];
    Ok(forge_geom::clip_line_to_face_polygon(line_pt, line_dir, &verts, fn_a_neg, min_chord))
}

/// Fallback gate: vertex sign-walk for degenerate post-split polygons.
///
/// When the Cyrus-Beck polygon is numerically degenerate (very thin strip
/// from a prior split), fall back to checking if the sign walk finds a
/// Pos↔Neg crossing. Returns a synthetic chord from crossing midpoints.
fn try_sign_walk_fallback(
    arena: &TopologyArena,
    geometry: &GeometryStore,
    face: FaceId,
    cut_plane: &Plane,
    config: &ToleranceConfig,
) -> Result<Option<([f64; 3], [f64; 3])>, KernelError> {
    let edges: Vec<_> = FaceEdgeIterator::new(arena, face)?
        .collect::<Result<Vec<_>, _>>()?;
    let mut crossings: Vec<[f64; 3]> = Vec::new();

    for he in &edges {
        let he_data = arena.get_half_edge(*he)?;
        let origin = he_data.origin();
        let next_data = arena.get_half_edge(he_data.next())?;
        let dest = next_data.origin();

        if let (Some(p_o), Some(p_d)) = (geometry.get_vertex_position(origin), geometry.get_vertex_position(dest)) {
            // Cut plane index is not known in try_sign_walk_fallback currently,
            // so we pass `std::usize::MAX` to bypass the symbolic check in this pure fallback context.
            let s_o = exact_sign_for_vertex(geometry, origin, p_o, cut_plane, std::usize::MAX);
            let s_d = exact_sign_for_vertex(geometry, dest, p_d, cut_plane, std::usize::MAX);

            let is_crossing = (s_o == TriSign::Pos && s_d == TriSign::Neg)
                           || (s_o == TriSign::Neg && s_d == TriSign::Pos);
            if is_crossing {
                let mid = forge_geom::primitives::plane::intersect_edge_plane(
                    cut_plane, p_o, p_d, config.get_edge_split_degeneracy(),
                );
                crossings.push(mid);
            } else if s_o == TriSign::Zero {
                crossings.push(*p_o);
            }
        }

        if crossings.len() >= 2 {
            return Ok(Some((crossings[0], crossings[1])));
        }
    }

    Ok(None)
}

/// Compute the exact sign of a vertex relative to a plane.
///
/// If the vertex has an exact rational position, uses it.
/// If the vertex is symbolic, evaluates the 4x4 determinant.
/// Fallback: promotes f64 to Rational.
pub fn exact_sign_for_vertex(
    geometry: &GeometryStore,
    vertex: VertexId,
    f64_pos: &[f64; 3],
    plane: &Plane,
    plane_idx: usize, // Required for symbolic 4x4 determinant against the plane table
) -> TriSign {
    // 1. Try exact explicit rational coordinates
    if let Some(exact) = geometry.get_vertex_position_exact(vertex) {
        return classify_point_exact(plane, exact);
    }

    // 2. Try symbolic (4x4 determinant)
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

    // 3. Fallback: Promote f64 to Rational
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

/// Collect f64 vertex positions for a face in winding order.
fn collect_face_positions(
    arena: &TopologyArena,
    geometry: &GeometryStore,
    face: FaceId,
) -> Result<Vec<[f64; 3]>, KernelError> {
    let edges: Vec<_> = FaceEdgeIterator::new(arena, face)?
        .collect::<Result<Vec<_>, _>>()?;
    let mut verts = Vec::with_capacity(edges.len());
    for he in &edges {
        let v = arena.get_half_edge(*he)?.origin();
        if let Some(p) = geometry.get_vertex_position(v) {
            verts.push(*p);
        }
    }
    Ok(verts)
}
