use std::collections::BTreeMap;

use forge_relational::facade::identity::EntityId;

use crate::data::topology_view::WorthTopologyHalfEdge;
use crate::interpretation::types::WorthWireInterpretation;
use crate::interpretation::vertex_branching::interpret_wire_branching;
use crate::materialization::MaterializedTopologyView;

pub fn interpret_wires(view: &MaterializedTopologyView) -> Vec<WorthWireInterpretation> {
    let topology = view.topology();
    let half_edge_map: BTreeMap<EntityId, &WorthTopologyHalfEdge> = topology
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

            WorthWireInterpretation {
                wire_id: wire.entity_id,
                class: branching.class,
                connected_component_count: branching.connected_component_count,
                terminal_vertex_ids: branching.terminal_vertex_ids,
                branch_vertex_ids: branching.branch_vertex_ids,
            }
        })
        .collect()
}
