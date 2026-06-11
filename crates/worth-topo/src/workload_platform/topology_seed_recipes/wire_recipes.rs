use forge_relational::facade::identity::EntityId;

use super::topology_record_constructors::{
    base_container, edge, entity, half_edge, vertex, wire, HalfEdgeRecordConstruction,
};
use crate::brep::topology_graph::TopologyView;
use crate::workload_platform::topology_seed::TopologySeedNeighborhoodReceipt;

pub(crate) fn open_wire_topology_view() -> TopologyView {
    wire_chain_view(60_000, "open wire", 4)
}

pub(crate) fn high_valence_vertex_topology_view() -> (TopologyView, TopologySeedNeighborhoodReceipt)
{
    let mut topology = base_container(70_000, "high valence wire");
    let center_id = entity(70_010);
    topology
        .vertices
        .push(vertex("high valence center", center_id));

    let mut incident_half_edges = Vec::new();
    let wire_id = entity(70_020);
    for index in 0..5 {
        let outer_id = entity(70_030 + index);
        let edge_id = entity(70_100 + index);
        let half_edge_id = entity(70_200 + index);
        topology
            .vertices
            .push(vertex(format!("high valence outer {index}"), outer_id));
        topology
            .edges
            .push(edge(format!("high valence edge {index}"), edge_id));
        topology
            .half_edges
            .push(half_edge(HalfEdgeRecordConstruction {
                label: format!("high valence half-edge {index}"),
                id: half_edge_id,
                loop_id: None,
                wire_id: Some(wire_id),
                next_id: Some(half_edge_id),
                prev_id: Some(half_edge_id),
                radial_next_id: Some(half_edge_id),
                edge_id,
                origin_id: center_id,
                target_id: outer_id,
                face_id: None,
            }));
        incident_half_edges.push(half_edge_id);
    }
    topology.wires.push(wire(
        "high valence wire",
        wire_id,
        incident_half_edges.clone(),
    ));
    (
        topology,
        TopologySeedNeighborhoodReceipt::new(center_id, incident_half_edges),
    )
}

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
