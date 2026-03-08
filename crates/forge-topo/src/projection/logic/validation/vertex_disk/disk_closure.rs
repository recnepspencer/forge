use forge_core::KernelError;

use crate::projection::data::{ProjectedTopology, ProjectedVertexId};
use crate::projection::logic::ProjectedTopologyQueries;

pub fn validate_projected_disk_closure(topology: &ProjectedTopology) -> Result<(), KernelError> {
    for (vertex_index, _) in topology.vertices().iter().enumerate() {
        let vertex_id = ProjectedVertexId::new(vertex_index as u32);
        topology
            .vertex_disk_components(vertex_id)
            .map_err(|err| forge_core::KernelError::TopologyViolation {
                err: forge_core::TopologyError::BrokenLoop {
                    starting_halfedge: 0,
                    face_index: 0,
                },
                context: Some(forge_core::ErrorContext {
                    scope: forge_core::ErrorScope::Entity {
                        entity_kind: "ProjectedVertex".to_string(),
                        index: vertex_id.raw(),
                    },
                    suggested_fixes: Vec::new(),
                    detail: format!("projected_disk_closure: {err}"),
                }),
            })?;
    }
    Ok(())
}
