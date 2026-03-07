mod face_loop_existence;
mod inner_outer_consistency;
mod orphan_half_edges;
mod single_owner;

use forge_core::KernelError;

use crate::projection::data::ProjectedTopology;

pub use face_loop_existence::validate_projected_face_has_at_least_one_loop;
pub use inner_outer_consistency::validate_projected_inner_outer_loop_consistency;
pub use orphan_half_edges::validate_projected_no_orphan_half_edges;
pub use single_owner::validate_projected_single_owner_per_loop;

pub fn validate_projected_reference_integrity(
    topology: &ProjectedTopology,
) -> Result<(), KernelError> {
    validate_projected_face_has_at_least_one_loop(topology)?;
    validate_projected_single_owner_per_loop(topology)?;
    validate_projected_no_orphan_half_edges(topology)?;
    validate_projected_inner_outer_loop_consistency(topology)?;
    Ok(())
}
