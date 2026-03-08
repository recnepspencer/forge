use std::collections::BTreeMap;

use forge_core::KernelError;

use crate::projection::data::{ProjectedTopology, ProjectedVertexId};
use crate::projection::logic::ProjectedTopologyQueries;

use super::super::shared::vf;

pub fn validate_projected_no_cross_disk_coedges(
    topology: &ProjectedTopology,
) -> Result<(), KernelError> {
    for (vertex_index, _) in topology.vertices().iter().enumerate() {
        let vertex_id = ProjectedVertexId::new(vertex_index as u32);
        let components = topology
            .vertex_disk_components(vertex_id)
            .map_err(|err| vf("projected_no_cross_disk_coedges", err.to_string()))?;

        if components.len() <= 1 {
            continue;
        }

        let mut disk_by_half_edge = BTreeMap::new();
        for (disk_index, component) in components.iter().enumerate() {
            for half_edge in component {
                disk_by_half_edge.insert(half_edge.raw(), disk_index);
            }
        }

        for half_edge in topology.vertex_outgoing_half_edges(vertex_id) {
            let incoming = topology.half_edge(half_edge).prev;
            let candidate = topology.half_edge(incoming).radial_next;
            if topology.half_edge(candidate).origin != vertex_id {
                continue;
            }

            let outgoing_disk = disk_by_half_edge.get(&half_edge.raw()).copied();
            let incoming_disk = disk_by_half_edge.get(&candidate.raw()).copied();
            if let (Some(a), Some(b)) = (outgoing_disk, incoming_disk) {
                if a != b {
                    return Err(vf(
                        "projected_no_cross_disk_coedges",
                        format!(
                            "Vertex {} has cross-disk coedge between halfedges {} and {}",
                            vertex_id.raw(),
                            half_edge.raw(),
                            candidate.raw()
                        ),
                    ));
                }
            }
        }
    }

    Ok(())
}
