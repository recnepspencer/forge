//! Public API for geometry state.
//!
//! This facade exposes the stable geometry store surface consumed by other
//! kernel components.

pub use super::coalescence::{snap_or_coalesce_vertex, CoalescenceResult};
pub use super::eval::build_position_lookup;
pub use super::patch::GeometryPatch;
pub use super::schema::{ExactPosition, GeometryState};
pub use super::split_propagation::propagate_curve_on_split;
pub use super::GeometryView;
