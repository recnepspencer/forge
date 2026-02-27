//! Spatial query shared operations.
//!
//! DOMAIN: BVH construction, face coincidence prepass, and normal
//! alignment queries. Used by boolean classification, split, and
//! intersection phases.

pub mod bvh;
pub mod coincidence;
pub mod normal_alignment;
