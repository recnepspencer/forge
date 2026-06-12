use forge_relational::facade::identity::EntityId;

use super::topology_record_constructors::{
    base_container, edge, entity, half_edge, vertex, wire, HalfEdgeRecordConstruction,
};
use crate::brep::topology_graph::TopologyView;

pub(crate) fn wire_chain_view(base: u64, label: &str, edge_count: usize) -> TopologyView {
    let mut topology = base_container(base, label);
    let wire_id = entity(base + 10);
    let vertex_ids = (0..=edge_count)
        .map(|index| entity(base + 20 + index as u64))
        .collect::<Vec<_>>();
    let edge_ids = (0..edge_count)
        .map(|index| entity(base + 100 + index as u64))
        .collect::<Vec<_>>();
    let half_edge_ids = (0..edge_count)
        .map(|index| entity(base + 200 + index as u64))
        .collect::<Vec<_>>();

    for (index, id) in vertex_ids.iter().enumerate() {
        topology
            .vertices
            .push(vertex(format!("{label} vertex {index}"), *id));
    }

    for index in 0..edge_count {
        topology
            .edges
            .push(edge(format!("{label} edge {index}"), edge_ids[index]));
        topology.half_edges.push(wire_half_edge(
            label,
            index,
            wire_id,
            &half_edge_ids,
            edge_ids[index],
            vertex_ids[index],
            vertex_ids[index + 1],
        ));
    }
    topology
        .wires
        .push(wire(format!("{label} wire"), wire_id, half_edge_ids));
    topology
}

fn wire_half_edge(
    label: &str,
    index: usize,
    wire_id: EntityId,
    half_edge_ids: &[EntityId],
    edge_id: EntityId,
    origin_id: EntityId,
    target_id: EntityId,
) -> crate::brep::topology_graph::TopologyHalfEdge {
    half_edge(HalfEdgeRecordConstruction {
        label: format!("{label} half-edge {index}"),
        id: half_edge_ids[index],
        loop_id: None,
        wire_id: Some(wire_id),
        next_id: Some(half_edge_ids[(index + 1) % half_edge_ids.len()]),
        prev_id: Some(half_edge_ids[(index + half_edge_ids.len() - 1) % half_edge_ids.len()]),
        radial_next_id: Some(half_edge_ids[index]),
        edge_id,
        origin_id,
        target_id,
        face_id: None,
    })
}
