//! Spatial indexing, proximity matching, and coordinate utilities.
//!
//! DOMAIN: Spatial acceleration structures, geometric matching,
//! coordinate frame construction, and union-find for component tracking.
//!
//! Subdirectories:
//! - `acceleration`: BSP trees, BVH trees (spatial indices)
//! - `matching`: Edge matching, epsilon welding, coincidence detection
//! - `coordinate`: Local coordinate spaces, bounds

pub mod acceleration;
pub mod matching;
pub mod coordinate;
pub mod union_find;

pub use acceleration::bsp;
pub use acceleration::bvh;
pub use matching::coincidence;
pub use matching::edge_match;
pub use matching::epsilon_weld;
pub use coordinate::bounds;
pub use coordinate::local_space;
