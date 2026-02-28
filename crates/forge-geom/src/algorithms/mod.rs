//! Algorithms module — composed geometry algorithms.
//!
//! Subdirectories:
//! - `triangulation`: CDT and polygon decomposition
//! - `intersection`: Line-polygon clipping, overlap detection
//! - `polygon`: Polygon operations and segment utilities
//! - `sorting`: Radial angular sorting
//! - `boundary_cert`: Boundary certification

pub mod boundary_cert;
pub mod intersection;
pub mod polygon;
pub mod sorting;
pub mod triangulation;

pub use sorting::angular_sort;
pub use sorting::angular_sort::sort_edges_radially;

pub use triangulation::cdt;
pub use triangulation::cdt::{triangulate_face_with_cut, triangulate_polygon_2d, CdtResult};

pub use intersection::chord;
pub use intersection::chord::{clip_line_to_face_polygon, compute_intersection_line};
pub use intersection::clipping;
pub use intersection::clipping::clip_line_to_polygon;
pub use intersection::overlap as intersection_overlap;
pub use intersection::overlap::polygons_overlap_3d;
pub use intersection::polygon_overlap;
pub use intersection::polygon_overlap::{
    dominant_projection_axes, point_strictly_inside_polygon, polygons_overlap_2d,
    segments_properly_cross,
};

pub use polygon::polygon::{bridge_polygon_hole, bridge_polygon_holes};
pub use polygon::segment::point_on_segment;
