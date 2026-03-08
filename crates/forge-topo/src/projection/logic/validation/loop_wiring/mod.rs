mod duplicate_coedges;
mod edge_endpoints;
mod face_membership;
mod loop_cardinality;
mod loop_closure;
mod prev_consistency;
mod vertex_continuity;

use forge_core::KernelError;

use crate::projection::data::ProjectedTopology;

pub use duplicate_coedges::validate_projected_no_duplicate_coedges_in_loop;
pub use edge_endpoints::validate_projected_edge_endpoints_match_loop_vertices;
pub use face_membership::validate_projected_face_loop_membership_complete;
pub use loop_cardinality::validate_projected_loop_minimum_cardinality;
pub use loop_closure::validate_projected_loops;
pub use prev_consistency::validate_projected_prev_consistency;
pub use vertex_continuity::validate_projected_vertex_continuity;

use super::shared::vf;

pub fn validate_projected_loop_wiring(topology: &ProjectedTopology) -> Result<(), KernelError> {
    validate_projected_prev_consistency(topology)?;
    validate_projected_loops(topology)?;
    validate_projected_loop_minimum_cardinality(topology)?;
    validate_projected_no_duplicate_coedges_in_loop(topology)?;
    validate_projected_face_loop_membership_complete(topology)?;
    validate_projected_edge_endpoints_match_loop_vertices(topology)?;
    validate_projected_vertex_continuity(topology)?;
    Ok(())
}
