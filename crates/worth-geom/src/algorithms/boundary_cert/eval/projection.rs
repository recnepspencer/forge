//! Projection of authored 3D boundaries into deterministic 2D frames.
//!
//! DOMAIN: Establishes the 2D coordinate frame used by boundary certification.

use super::super::schema::{ProjectedBoundary2D, ProjectionFrame2D, Segment2D};

/// Build a deterministic projection frame from a 3D plane normal.
///
/// Drops the axis with the largest absolute normal component.
/// Tie-break: X > Y > Z (spec §4.6).
/// The orientation sign preserves winding direction after projection.
pub fn build_projection_frame(normal: [f64; 3]) -> ProjectionFrame2D {
    let abs_n = [normal[0].abs(), normal[1].abs(), normal[2].abs()];

    let (drop_axis, u_axis, v_axis) = if abs_n[0] >= abs_n[1] && abs_n[0] >= abs_n[2] {
        (0, 1, 2)
    } else if abs_n[1] >= abs_n[2] {
        (1, 0, 2)
    } else {
        (2, 0, 1)
    };

    let orientation_sign = if normal[drop_axis] >= 0.0 { 1.0 } else { -1.0 };

    ProjectionFrame2D::new(drop_axis, u_axis, v_axis, orientation_sign)
}

/// Project a 3D point onto 2D using the given frame.
pub fn project_point(point: [f64; 3], frame: &ProjectionFrame2D) -> [f64; 2] {
    let u = point[frame.get_u_axis()];
    let v = point[frame.get_v_axis()];
    if frame.get_orientation_sign() < 0.0 {
        [v, u]
    } else {
        [u, v]
    }
}

/// Project 3D boundary segments to 2D using the given plane normal.
///
/// Builds a `ProjectionFrame2D` from the normal, then projects each segment.
pub fn project_boundary_to_2d(
    segments_3d: &[([f64; 3], [f64; 3], u64)],
    normal: [f64; 3],
) -> ProjectedBoundary2D {
    let frame = build_projection_frame(normal);
    let segments: Vec<Segment2D> = segments_3d
        .iter()
        .map(|(start, end, prov)| {
            let s2d = project_point(*start, &frame);
            let e2d = project_point(*end, &frame);
            Segment2D::new(s2d, e2d, *prov)
        })
        .collect();
    ProjectedBoundary2D::new(segments, frame)
}
