//! Pure geometric measurement algorithms.
//!
//! DOMAIN: Computes scalar properties from raw vertex data —
//! polyhedron volume, centroid, distance. Polygon area already exists in
//! `primitives::polygon::compute_polygon_area`.

pub mod area;
pub mod centroid;
pub mod volume;
pub mod distance;
