//! Gate logic: should a face be cut by a plane?
//!
//! DOMAIN: Determines IF a face needs cutting, separately from applying the cut.
//! DEPENDENCIES: forge_geom (intersection_line, clip_line), GeometryState.
//! INVARIANTS: `compute_face_chord` is the SOLE gate for face cutting.
//!   No cut occurs without passing this gate first.

use forge_core::KernelError;
use forge_geom::primitives::plane::Plane;
use forge_math::sign::TriSign;
use forge_topo::arena::TopologyArena;
use forge_topo::handles::FaceId;

use crate::geometry_state::GeometryState;
use crate::core::ToleranceConfig;
use super::signs::exact_sign_for_vertex;

/// Gate: does the cut_plane produce an interior chord segment in this face?
///
/// Literature-correct gate (Thibault-Naylor/CGAL/Cherchi): compute the
/// intersection LINE of face_plane and cut_plane, then clip it to the face
/// polygon via Cyrus-Beck. Falls back to vertex sign-walk for degenerate
/// polygons from prior splits.
pub fn compute_face_chord(
    arena: &TopologyArena,
    geometry: &GeometryState,
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
    geometry: &GeometryState,
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

    let chord = forge_geom::algorithms::clipping::clip_line_to_polygon(
        line_pt,
        line_dir,
        &verts,
        fn_a,
        min_chord,
    );
    if chord.is_some() {
        return Ok(chord);
    }

    let fn_a_neg = [-fn_a[0], -fn_a[1], -fn_a[2]];
    Ok(forge_geom::algorithms::clipping::clip_line_to_polygon(
        line_pt,
        line_dir,
        &verts,
        fn_a_neg,
        min_chord,
    ))
}

/// Fallback gate: vertex sign-walk for degenerate post-split polygons.
///
/// When the Cyrus-Beck polygon is numerically degenerate (very thin strip
/// from a prior split), fall back to checking if the sign walk finds a
/// Pos↔Neg crossing. Returns a synthetic chord from crossing midpoints.
fn try_sign_walk_fallback(
    arena: &TopologyArena,
    geometry: &GeometryState,
    face: FaceId,
    cut_plane: &Plane,
    config: &ToleranceConfig,
) -> Result<Option<([f64; 3], [f64; 3])>, KernelError> {
    let loops = forge_topo::polygon::face_loop_vertices(arena, face)?;
    let outer_loop = match loops.first() {
        Some(loop_vertices) => loop_vertices,
        None => return Ok(None),
    };
    let mut crossings: Vec<[f64; 3]> = Vec::new();

    let n = outer_loop.len();
    for i in 0..n {
        let origin = outer_loop[i];
        let dest = outer_loop[(i + 1) % n];

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

/// Collect f64 vertex positions for a face in winding order.
fn collect_face_positions(
    arena: &TopologyArena,
    geometry: &GeometryState,
    face: FaceId,
) -> Result<Vec<[f64; 3]>, KernelError> {
    let loops = forge_topo::polygon::face_loop_vertices(arena, face)?;
    let mut verts = Vec::new();
    if let Some(outer_loop) = loops.first() {
        verts.reserve(outer_loop.len());
        for vertex in outer_loop {
            if let Some(p) = geometry.get_vertex_position(*vertex) {
                verts.push(*p);
            }
        }
    }
    Ok(verts)
}
