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
pub mod measurement;
pub mod polygon;
pub mod sorting;
pub mod triangulation;
