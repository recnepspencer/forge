use std::collections::BTreeSet;

use forge_core::KernelError;

use crate::projection::data::{ProjectedHalfEdgeId, ProjectedTopology};

use super::{vf, walk_radial_ring};

pub fn validate_projected_radial_cycle_uniqueness(
    topology: &ProjectedTopology,
) -> Result<(), KernelError> {
    let mut globally_checked = BTreeSet::new();
    for (start_index, _) in topology.half_edges().iter().enumerate() {
        let start_id = ProjectedHalfEdgeId::new(start_index as u32);
        if globally_checked.contains(&start_id.raw()) {
            continue;
        }
        let ring = walk_radial_ring(topology, start_id)
            .map_err(|err| vf("projected_radial_cycle_uniqueness", err))?;
        let mut ring_seen = BTreeSet::new();
        for current in ring {
            globally_checked.insert(current.raw());
            if !ring_seen.insert(current.raw()) {
                return Err(vf(
                    "projected_radial_cycle_uniqueness",
                    format!(
                        "Radial ring seeded at HE {} contains duplicate HE {}",
                        start_id.raw(),
                        current.raw()
                    ),
                ));
            }
        }
    }
    Ok(())
}
