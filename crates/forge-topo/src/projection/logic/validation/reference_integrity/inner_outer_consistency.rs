use std::collections::BTreeSet;

use forge_core::KernelError;

use crate::projection::data::{ProjectedFaceId, ProjectedLoopId, ProjectedTopology};

use super::super::shared::vf;

pub fn validate_projected_inner_outer_loop_consistency(
    topology: &ProjectedTopology,
) -> Result<(), KernelError> {
    let mut outer_loops = BTreeSet::new();
    for face in topology.faces() {
        outer_loops.insert(face.outer_loop.raw());
    }

    for (face_index, face) in topology.faces().iter().enumerate() {
        let face_id = ProjectedFaceId::new(face_index as u32);
        validate_projected_loop_face(topology, face_id, face.outer_loop, "outer_loop")?;

        for inner_loop in &face.inner_loops {
            if outer_loops.contains(&inner_loop.raw()) {
                return Err(vf(
                    "projected_inner_outer_loop_consistency",
                    format!(
                        "Face {} has inner loop {} which is also an outer_loop of another face",
                        face_id.raw(),
                        inner_loop.raw()
                    ),
                ));
            }
            validate_projected_loop_face(topology, face_id, *inner_loop, "inner_loop")?;
        }
    }

    Ok(())
}

fn validate_projected_loop_face(
    topology: &ProjectedTopology,
    face: ProjectedFaceId,
    loop_id: ProjectedLoopId,
    role: &str,
) -> Result<(), KernelError> {
    let loop_data = topology.loop_data(loop_id);
    if loop_data.face != face {
        return Err(vf(
            "projected_inner_outer_loop_consistency",
            format!(
                "Face {} {} {} has loop.face() = {}",
                face.raw(),
                role,
                loop_id.raw(),
                loop_data.face.raw()
            ),
        ));
    }
    Ok(())
}
