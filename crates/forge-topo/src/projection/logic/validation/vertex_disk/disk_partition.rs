use std::collections::BTreeSet;

use forge_core::KernelError;

use crate::projection::data::{ProjectedTopology, ProjectedVertexId};
use crate::projection::logic::ProjectedTopologyQueries;

use super::super::shared::vf;

pub fn validate_projected_vertex_disk_partition(
    topology: &ProjectedTopology,
) -> Result<(), KernelError> {
    for (vertex_index, _) in topology.vertices().iter().enumerate() {
        let vertex_id = ProjectedVertexId::new(vertex_index as u32);
        let outgoing = topology.vertex_outgoing_half_edges(vertex_id);
        let expected = outgoing
            .iter()
            .map(|half_edge| half_edge.raw())
            .collect::<BTreeSet<_>>();
        if expected.is_empty() {
            continue;
        }

        let components = topology
            .vertex_disk_components(vertex_id)
            .map_err(|err| vf("projected_vertex_disk_partition", err.to_string()))?;
        let mut covered = BTreeSet::new();

        for component in &components {
            for half_edge in component {
                if !covered.insert(half_edge.raw()) {
                    return Err(vf(
                        "projected_vertex_disk_partition",
                        format!(
                            "Vertex {} halfedge {} appears in multiple disk components",
                            vertex_id.raw(),
                            half_edge.raw()
                        ),
                    ));
                }
            }
        }

        if covered != expected {
            return Err(vf(
                "projected_vertex_disk_partition",
                format!(
                    "Vertex {} disk components do not cover all outgoing halfedges (covered={}, expected={})",
                    vertex_id.raw(),
                    covered.len(),
                    expected.len()
                ),
            ));
        }
    }

    Ok(())
}
