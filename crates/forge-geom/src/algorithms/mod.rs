//! Algorithms module — composed geometry algorithms.

pub mod angular_sort;
pub mod cdt;
pub mod chord;
pub mod polygon_overlap;

pub use angular_sort::sort_edges_radially;
pub use cdt::{triangulate_polygon_2d, triangulate_face_with_cut, CdtResult};
pub use chord::{compute_intersection_line, clip_line_to_face_polygon};
pub use polygon_overlap::{dominant_projection_axes, polygons_overlap_2d,
                          point_strictly_inside_polygon, segments_properly_cross};
