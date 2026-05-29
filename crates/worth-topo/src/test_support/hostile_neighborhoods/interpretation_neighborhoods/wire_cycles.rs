use crate::brep::topology_graph::{TopologyView, TopologyWire};

use super::primitives::*;
pub(crate) fn closed_wire_cycle_view() -> TopologyView {
    closed_wire_cycle_of_size(3)
}

pub(crate) fn open_wire_chain_view(length: usize) -> TopologyView {
    assert!(
        length >= 2,
        "open wire chain requires at least two half-edges"
    );

    let wire_id = entity(60);
    let mut half_edges = Vec::new();
    let mut edges = Vec::new();
    let mut vertices = Vec::new();
    let mut half_edge_ids = Vec::new();

    for index in 0..=length {
        vertices.push(vertex(&format!("v{index}"), entity(61 + index as u64)));
    }

    for index in 0..length {
        let edge_id = entity(80 + index as u64);
        let half_edge_id = entity(100 + index as u64);
        let next_half_edge_id = if index + 1 < length {
            Some(entity(100 + (index + 1) as u64))
        } else {
            None
        };
        let prev_half_edge_id = if index == 0 {
            None
        } else {
            Some(entity(100 + (index - 1) as u64))
        };

        edges.push(edge(&format!("e{index}"), edge_id));
        half_edges.push(half_edge_full(
            half_edge_id,
            None,
            Some(wire_id),
            next_half_edge_id,
            prev_half_edge_id,
            Some(half_edge_id),
            Some(edge_id),
            Some(entity(61 + index as u64)),
            Some(entity(61 + index as u64 + 1)),
            None,
        ));
        half_edge_ids.push(half_edge_id);
    }

    TopologyView {
        wires: vec![TopologyWire {
            entity_id: wire_id,
            label: "open-chain".into(),
            half_edge_ids,
        }],
        half_edges,
        edges,
        vertices,
        ..TopologyView::default()
    }
}

pub(crate) fn connected_wire_branch_view(branch_count: usize) -> TopologyView {
    assert!(
        branch_count >= 3,
        "connected wire branch requires at least three arms"
    );

    let wire_id = entity(500);
    let center_vertex = entity(501);
    let mut vertices = vec![vertex("center", center_vertex)];
    let mut edges = Vec::new();
    let mut half_edges = Vec::new();
    let mut half_edge_ids = Vec::new();

    for index in 0..branch_count {
        let outer_vertex = entity(510 + index as u64);
        let edge_id = entity(530 + index as u64);
        let half_edge_id = entity(550 + index as u64);
        vertices.push(vertex(&format!("leaf{index}"), outer_vertex));
        edges.push(edge(&format!("branch{index}"), edge_id));
        half_edges.push(half_edge_full(
            half_edge_id,
            None,
            Some(wire_id),
            Some(half_edge_id),
            Some(half_edge_id),
            Some(half_edge_id),
            Some(edge_id),
            Some(center_vertex),
            Some(outer_vertex),
            None,
        ));
        half_edge_ids.push(half_edge_id);
    }

    TopologyView {
        wires: vec![TopologyWire {
            entity_id: wire_id,
            label: "branch".into(),
            half_edge_ids,
        }],
        half_edges,
        edges,
        vertices,
        ..TopologyView::default()
    }
}

pub(crate) fn closed_wire_cycle_of_size(length: usize) -> TopologyView {
    assert!(
        length >= 3,
        "closed wire cycle requires at least three half-edges"
    );

    let wire_id = entity(1);
    let mut half_edges = Vec::new();
    let mut edges = Vec::new();
    let mut vertices = Vec::new();
    let mut half_edge_ids = Vec::new();

    for index in 0..length {
        vertices.push(vertex(&format!("v{index}"), entity(2 + index as u64)));
    }

    for index in 0..length {
        let edge_id = entity(20 + index as u64);
        let half_edge_id = entity(40 + index as u64);
        let next_half_edge_id = entity(40 + ((index + 1) % length) as u64);
        let prev_half_edge_id = entity(40 + ((index + length - 1) % length) as u64);
        let origin_vertex_id = entity(2 + index as u64);

        edges.push(edge(&format!("e{index}"), edge_id));
        half_edges.push(half_edge_full(
            half_edge_id,
            None,
            Some(wire_id),
            Some(next_half_edge_id),
            Some(prev_half_edge_id),
            Some(half_edge_id),
            Some(edge_id),
            Some(origin_vertex_id),
            Some(entity(2 + ((index + 1) % length) as u64)),
            None,
        ));
        half_edge_ids.push(half_edge_id);
    }

    TopologyView {
        wires: vec![TopologyWire {
            entity_id: wire_id,
            label: "cycle".into(),
            half_edge_ids,
        }],
        half_edges,
        edges,
        vertices,
        ..TopologyView::default()
    }
}
