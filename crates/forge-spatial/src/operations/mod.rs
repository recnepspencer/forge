//! Spatial operations — queries that bridge topology and geometry.
//!
//! DOMAIN: Position-dependent spatial queries (bounds, classification,
//!         proximity) that require both topology traversal and geometric types.
//!
//! Vertical slices:
//! - `bounds`: AABB computation, distance, proximity (coincident vertex detection)
//! - `classify`: Point-in-solid, point-on-face classification

pub mod facade;
pub mod bounds;
pub mod classify;
