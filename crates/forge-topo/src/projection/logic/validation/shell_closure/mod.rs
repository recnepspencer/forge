mod broken_boundary;
mod face_adjacency;
mod manifold_edges;

use forge_core::KernelError;

use crate::projection::data::ProjectedTopology;

pub use broken_boundary::validate_projected_broken_boundary;
pub use face_adjacency::validate_projected_face_adjacency;
pub use manifold_edges::validate_projected_manifold_edges;

pub fn validate_projected_shell_closure(
    topology: &ProjectedTopology,
) -> Result<(), KernelError> {
    validate_projected_manifold_edges(topology)?;
    validate_projected_broken_boundary(topology)?;
    validate_projected_face_adjacency(topology)?;
    Ok(())
}
