mod loop_wiring;
mod radial_edge;
mod reference_integrity;
mod shell_closure;
mod shared;

use forge_core::KernelError;

use crate::projection::data::ProjectedTopology;

pub use loop_wiring::validate_projected_loop_wiring;
pub use radial_edge::validate_projected_radial_edge;
pub use shell_closure::{
    validate_projected_broken_boundary, validate_projected_face_adjacency,
    validate_projected_manifold_edges, validate_projected_shell_closure,
};

pub fn validate_projected_topology_baseline(
    topology: &ProjectedTopology,
) -> Result<(), KernelError> {
    validate_projected_loop_wiring(topology)?;
    validate_projected_radial_edge(topology)?;
    reference_integrity::validate_projected_reference_integrity(topology)?;
    shell_closure::validate_projected_shell_closure(topology)?;
    Ok(())
}
