use forge_core::KernelError;
use forge_spec::facade::SpecShellKind;

use crate::projection::data::{ProjectedShellId, ProjectedTopology};
use crate::projection::logic::ProjectedTopologyQueries;

use super::super::shared::vf;

pub fn validate_projected_shell_consistency(
    topology: &ProjectedTopology,
) -> Result<(), KernelError> {
    for shell_index in 0..topology.shell_count() {
        let shell = ProjectedShellId::new(shell_index as u32);
        if !matches!(topology.shell(shell).kind, SpecShellKind::Solid(_)) {
            continue;
        }

        for face in topology.shell_faces(shell) {
            for edge in topology
                .face_edges(face)
                .map_err(|err| vf("projected_shell_consistency", err.to_string()))?
            {
                if topology.is_boundary_edge(edge) {
                    return Err(vf(
                        "projected_shell_consistency",
                        format!(
                            "Solid shell {} contains boundary edge {} on face {}",
                            shell.raw(),
                            edge.raw(),
                            face.raw()
                        ),
                    ));
                }
            }
        }
    }

    Ok(())
}
