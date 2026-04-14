//! Proximity matching and coincidence detection.
//!
//! DOMAIN: Edge matching (fuzzy/exact), epsilon welding,
//! and coincidence graph construction for near-coincident geometry.
//!
//! DEPENDENCIES: `primitives`, `worth-math` (predicates)

pub mod coincidence;
pub mod edge_match;
pub mod epsilon_weld;
