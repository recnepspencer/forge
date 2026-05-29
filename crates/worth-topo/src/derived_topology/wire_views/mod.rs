use std::collections::BTreeMap;

use forge_relational::facade::identity::EntityId;

use crate::brep::topology_graph::TopologyHalfEdge;
use crate::derived_topology::materialized_graph::MaterializedTopologyView;
use crate::derived_topology::traversal_views::types::WireInterpretation;
use crate::derived_topology::vertex_disks::interpret_wire_branching;

pub fn interpret_wires(view: &MaterializedTopologyView) -> Vec<WireInterpretation> {
    let topology = view.topology();
    let half_edge_map: BTreeMap<EntityId, &TopologyHalfEdge> = topology
        .half_edges
        .iter()
        .map(|record| (record.entity_id, record))
        .collect();

    topology
        .wires
        .iter()
        .map(|wire| {
            let branching = interpret_wire_branching(
                wire.half_edge_ids.iter().copied().collect(),
                &half_edge_map,
            );

            WireInterpretation {
                wire_id: wire.entity_id,
                class: branching.class,
                connected_component_count: branching.connected_component_count,
                terminal_vertex_ids: branching.terminal_vertex_ids,
                branch_vertex_ids: branching.branch_vertex_ids,
            }
        })
        .collect()
}




