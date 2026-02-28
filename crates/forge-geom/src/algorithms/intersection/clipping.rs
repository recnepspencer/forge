//! Clipping algorithms.
//!
//! DOMAIN: Stateless geometric clipping routines over point arrays.

/// Clip an infinite line to a planar polygon and return the interior segment.
///
/// This is a spec-aligned alias of [`crate::algorithms::chord::clip_line_to_face_polygon`]
/// implementing Cyrus-Beck clipping against polygon edge half-planes.
pub fn clip_line_to_polygon(
    line_pt: [f64; 3],
    line_dir: [f64; 3],
    polygon_verts: &[[f64; 3]],
    face_normal: [f64; 3],
    min_chord_len: f64,
) -> Option<([f64; 3], [f64; 3])> {
    crate::algorithms::chord::clip_line_to_face_polygon(
        line_pt,
        line_dir,
        polygon_verts,
        face_normal,
        min_chord_len,
    )
}
