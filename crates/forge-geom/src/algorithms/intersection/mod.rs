//! Intersection and overlap algorithms.
//!
//! DOMAIN: Line-polygon clipping, chord/intersection line computation,
//! polygon-polygon overlap detection, and general intersection queries.
//!
//! DEPENDENCIES: `primitives`

pub mod chord;
pub mod clipping;
pub mod overlap;
pub mod polygon_overlap;
