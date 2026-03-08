use std::collections::BTreeSet;

use forge_core::KernelError;

use crate::projection::data::{ProjectedFaceId, ProjectedTopology};
use crate::projection::logic::ProjectedTopologyQueries;

use super::vf;

pub fn validate_projected_face_loop_membership_complete(
    topology: &ProjectedTopology,
) -> Result<(), KernelError> {
    for (face_index, _) in topology.faces().iter().enumerate() {
        let face_id = ProjectedFaceId::new(face_index as u32);
        let reachable = topology
            .face_half_edges(face_id)
            .map_err(|err| vf("projected_face_loop_membership_complete", err.to_string()))?
            .into_iter()
            .map(|half_edge| half_edge.raw())
            .collect::<BTreeSet<_>>();
        for (he_index, half_edge) in topology.half_edges().iter().enumerate() {
            if half_edge.face == face_id && !reachable.contains(&(he_index as u32)) {
                return Err(vf(
                    "projected_face_loop_membership_complete",
                    format!(
                        "HE {} claims face {} but is unreachable from its loops",
                        he_index,
                        face_id.raw()
                    ),
                ));
            }
        }
    }
    Ok(())
}
