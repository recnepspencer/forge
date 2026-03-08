use forge_core::KernelError;

use crate::projection::data::{ProjectedHalfEdgeId, ProjectedTopology};

use super::{vf, walk_radial_ring};

pub fn validate_projected_radial_rings(topology: &ProjectedTopology) -> Result<(), KernelError> {
    for (start_index, _) in topology.half_edges().iter().enumerate() {
        walk_radial_ring(topology, ProjectedHalfEdgeId::new(start_index as u32))
            .map_err(|err| vf("projected_radial_ring_closure", err))?;
    }
    Ok(())
}
