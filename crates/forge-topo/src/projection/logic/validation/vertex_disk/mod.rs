mod cross_disk_coedges;
mod disk_closure;
mod disk_partition;
mod vertex_outgoing;

use forge_core::KernelError;

use crate::projection::data::ProjectedTopology;

pub use cross_disk_coedges::validate_projected_no_cross_disk_coedges;
pub use disk_closure::validate_projected_disk_closure;
pub use disk_partition::validate_projected_vertex_disk_partition;
pub use vertex_outgoing::validate_projected_vertex_outgoing;

pub fn validate_projected_vertex_disk(topology: &ProjectedTopology) -> Result<(), KernelError> {
    validate_projected_vertex_outgoing(topology)?;
    validate_projected_disk_closure(topology)?;
    validate_projected_vertex_disk_partition(topology)?;
    validate_projected_no_cross_disk_coedges(topology)?;
    Ok(())
}
