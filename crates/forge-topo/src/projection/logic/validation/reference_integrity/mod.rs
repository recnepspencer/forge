mod acyclic_containment;
mod bidirectional_links;
mod dangling_refs;
mod face_loop_existence;
mod hierarchy;
mod inner_outer_consistency;
mod orphan_half_edges;
mod single_owner;

use forge_core::KernelError;

use crate::projection::data::ProjectedTopology;

pub use face_loop_existence::validate_projected_face_has_at_least_one_loop;
pub use acyclic_containment::validate_projected_acyclic_containment;
pub use bidirectional_links::validate_projected_bidirectional_links;
pub use dangling_refs::validate_projected_no_dangling_refs;
pub use hierarchy::validate_projected_hierarchy;
pub use inner_outer_consistency::validate_projected_inner_outer_loop_consistency;
pub use orphan_half_edges::validate_projected_no_orphan_half_edges;
pub use single_owner::validate_projected_single_owner_per_loop;

pub fn validate_projected_reference_integrity(
    topology: &ProjectedTopology,
) -> Result<(), KernelError> {
    validate_projected_no_dangling_refs(topology)?;
    validate_projected_acyclic_containment(topology)?;
    validate_projected_hierarchy(topology)?;
    validate_projected_bidirectional_links(topology)?;
    validate_projected_face_has_at_least_one_loop(topology)?;
    validate_projected_single_owner_per_loop(topology)?;
    validate_projected_no_orphan_half_edges(topology)?;
    validate_projected_inner_outer_loop_consistency(topology)?;
    Ok(())
}
