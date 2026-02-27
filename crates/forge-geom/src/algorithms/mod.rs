//! Algorithms module — composed geometry algorithms.

pub mod angular_sort;
pub mod boundary_cert;
pub mod cdt;
pub mod chord;
pub mod clipping;
pub mod intersection;
pub mod polygon;
pub mod polygon_overlap;
pub mod segment;

pub use angular_sort::sort_edges_radially;
pub use cdt::{triangulate_face_with_cut, triangulate_polygon_2d, CdtResult};
pub use chord::{clip_line_to_face_polygon, compute_intersection_line};
pub use clipping::clip_line_to_polygon;
pub use intersection::polygons_overlap_3d;
pub use polygon::{bridge_polygon_hole, bridge_polygon_holes};
pub use segment::point_on_segment;
pub use polygon_overlap::{
    dominant_projection_axes, point_strictly_inside_polygon, polygons_overlap_2d,
    segments_properly_cross,
};
