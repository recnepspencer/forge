//! Point classification query sub-module.
//!
//! DOMAIN: Point-in-solid and point-on-face classification using SoS
//!         (Simulation of Simplicity) ray-casting and tolerance proximity.
//!
//! DEPENDENCIES: forge-topo (arena, handles, traversal), forge-geom (Aabb,
//!               BvhNode, polygon helpers), forge-math (orient2d, orient3d).
//! INVARIANTS:
//! - No topology mutation.
//! - Deterministic for identical floating-point inputs.

pub mod face_sampling;
pub mod normal_orientation;
pub mod point_in_solid;
pub mod point_on_face;
pub mod schema;
pub mod sos;

#[cfg(test)]
mod point_in_solid_tests;
