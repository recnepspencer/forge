//! Algorithms module — composed geometry algorithms.

pub mod chord;
pub mod polygon_overlap;

pub use chord::{compute_intersection_line, clip_line_to_face_polygon};
pub use polygon_overlap::{dominant_projection_axes, polygons_overlap_2d,
                          point_strictly_inside_polygon, segments_properly_cross};
