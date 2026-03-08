mod broken_splices;
mod cycle_uniqueness;
mod edge_consistency;
mod neighbor_consistency;
mod ring_closure;

use std::collections::BTreeSet;

use forge_core::KernelError;

use crate::projection::data::{ProjectedHalfEdgeId, ProjectedTopology};

use super::shared::vf;

pub use broken_splices::validate_projected_no_broken_radial_splices;
pub use cycle_uniqueness::validate_projected_radial_cycle_uniqueness;
pub use edge_consistency::validate_projected_radial_edge_consistency;
pub use neighbor_consistency::validate_projected_radial_neighbor_consistency;
pub use ring_closure::validate_projected_radial_rings;

pub fn validate_projected_radial_edge(topology: &ProjectedTopology) -> Result<(), KernelError> {
    validate_projected_radial_rings(topology)?;
    validate_projected_radial_edge_consistency(topology)?;
    validate_projected_radial_cycle_uniqueness(topology)?;
    validate_projected_radial_neighbor_consistency(topology)?;
    validate_projected_no_broken_radial_splices(topology)?;
    Ok(())
}

fn walk_radial_ring(
    topology: &ProjectedTopology,
    start: ProjectedHalfEdgeId,
) -> Result<Vec<ProjectedHalfEdgeId>, String> {
    let max_steps = topology.half_edge_count().max(1);
    let mut result = Vec::new();
    let mut seen = BTreeSet::new();
    let mut current = start;

    for _ in 0..max_steps {
        if !seen.insert(current.raw()) {
            return Err(format!(
                "radial ring seeded at {} revisited {} before closing",
                start.raw(),
                current.raw()
            ));
        }
        result.push(current);
        let next = topology.half_edge(current).radial_next;
        if next == start {
            return Ok(result);
        }
        current = next;
    }

    Err(format!(
        "radial ring seeded at {} does not close within {} halfedges",
        start.raw(),
        max_steps
    ))
}
