//! Empty Boolean result handling.
//!
//! DOMAIN: Create a trivial result when no faces are selected from either solid.

use forge_core::KernelError;
use forge_topo::state::TopologyState;
use forge_topo::replay::ReplayLog;

use crate::geometry_state::GeometryState;
use crate::operations::boolean::result::{BooleanResult, BooleanIntrospection};

/// Build an empty result when no faces are selected on either side.
pub fn build_empty_result(
    mut introspection: BooleanIntrospection,
    _replay: ReplayLog,
    start_time: std::time::Instant,
) -> Result<BooleanResult, KernelError> {
    introspection.duration_micros = start_time.elapsed().as_micros() as u64;
    Ok(BooleanResult::new(
        TopologyState::empty(), GeometryState::new(), crate::brep::state::BrepState::new(),
        0, 0, introspection,
    ))
}
