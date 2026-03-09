mod broken_boundary;
mod face_adjacency;
mod laminar_edges;
mod manifold_edges;
mod orientation_consistency;
mod shell_consistency;

use forge_core::KernelError;

use crate::projection::data::ProjectedTopology;

pub use broken_boundary::validate_projected_broken_boundary;
pub use face_adjacency::validate_projected_face_adjacency;
pub use laminar_edges::validate_projected_laminar_edges;
pub use manifold_edges::validate_projected_manifold_edges;
pub use orientation_consistency::validate_projected_orientation_consistency;
pub use shell_consistency::validate_projected_shell_consistency;

pub fn validate_projected_shell_closure(topology: &ProjectedTopology) -> Result<(), KernelError> {
    validate_projected_shell_consistency(topology)?;
    validate_projected_laminar_edges(topology)?;
    validate_projected_manifold_edges(topology)?;
    validate_projected_orientation_consistency(topology)?;
    validate_projected_broken_boundary(topology)?;
    validate_projected_face_adjacency(topology)?;
    Ok(())
}
