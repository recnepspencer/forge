use std::collections::BTreeMap;

use forge_core::KernelError;

use crate::projection::data::{ProjectedLoopId, ProjectedTopology};

use super::super::shared::vf;

pub fn validate_projected_single_owner_per_loop(
    topology: &ProjectedTopology,
) -> Result<(), KernelError> {
    let mut owner_count: BTreeMap<u32, usize> = BTreeMap::new();

    for face in topology.faces() {
        *owner_count.entry(face.outer_loop.raw()).or_default() += 1;
        for inner in &face.inner_loops {
            *owner_count.entry(inner.raw()).or_default() += 1;
        }
    }

    for (loop_index, _) in topology.loops().iter().enumerate() {
        let loop_id = ProjectedLoopId::new(loop_index as u32);
        match owner_count.get(&loop_id.raw()).copied() {
            None => {
                return Err(vf(
                    "projected_single_owner_per_loop",
                    format!("Loop {} is orphaned", loop_id.raw()),
                ));
            }
            Some(count) if count > 1 => {
                return Err(vf(
                    "projected_single_owner_per_loop",
                    format!("Loop {} is claimed by {} faces", loop_id.raw(), count),
                ));
            }
            _ => {}
        }
    }
    Ok(())
}
