//! Algorithms module — composed geometry algorithms.

pub mod angular_sort;
pub mod cdt;
pub mod chord;
pub mod clipping;
pub mod intersection;
pub mod polygon;
pub mod polygon_overlap;
pub mod boundary_cert;

pub use angular_sort::sort_edges_radially;
pub use cdt::{triangulate_polygon_2d, triangulate_face_with_cut, CdtResult};
pub use chord::{compute_intersection_line, clip_line_to_face_polygon};
pub use clipping::clip_line_to_polygon;
pub use intersection::polygons_overlap_3d;
pub use polygon::{bridge_polygon_hole, bridge_polygon_holes};
pub use polygon_overlap::{dominant_projection_axes, polygons_overlap_2d,
                          point_strictly_inside_polygon, segments_properly_cross};
