use forge_core::KernelError;
use forge_spec::facade::SpecShellKind;

use crate::projection::data::{ProjectedShellId, ProjectedTopology};
use crate::projection::logic::ProjectedTopologyQueries;

use super::super::shared::vf;

pub fn validate_projected_laminar_edges(
    topology: &ProjectedTopology,
) -> Result<(), KernelError> {
    for shell_index in 0..topology.shell_count() {
        let shell = ProjectedShellId::new(shell_index as u32);
        if !matches!(topology.shell(shell).kind, SpecShellKind::Sheet) {
            continue;
        }

        for face in topology.shell_faces(shell) {
            for edge in topology
                .face_edges(face)
                .map_err(|err| vf("projected_laminar_edges", err.to_string()))?
            {
                let valence = topology.radial_valence(edge);
                if valence > 2 {
                    return Err(vf(
                        "projected_laminar_edges",
                        format!(
                            "Sheet shell {} contains edge {} on face {} with radial valence {}",
                            shell.raw(),
                            edge.raw(),
                            face.raw(),
                            valence
                        ),
                    ));
                }
            }
        }
    }

    Ok(())
}
