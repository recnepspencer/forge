use forge_core::KernelError;

use crate::projection::data::ProjectedTopology;

use super::super::shared::vf;

pub fn validate_projected_face_has_at_least_one_loop(
    topology: &ProjectedTopology,
) -> Result<(), KernelError> {
    for (face_index, face) in topology.faces().iter().enumerate() {
        let outer = face.outer_loop;
        if outer.index() >= topology.loop_count() {
            return Err(vf(
                "projected_face_has_at_least_one_loop",
                format!("Face {} outer_loop {} is invalid", face_index, outer.raw()),
            ));
        }
    }
    Ok(())
}
