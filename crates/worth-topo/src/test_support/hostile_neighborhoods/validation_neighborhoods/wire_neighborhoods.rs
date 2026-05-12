use crate::brep::topology_graph::TopologyView;

use super::primitives::*;
pub(crate) fn open_wire_chain_view(length: usize) -> TopologyView {
    assert!(
        length >= 2,
        "open wire chain requires at least two half-edges"
    );

    let wire_id = entity(300);
    let mut vertices = Vec::new();
    let mut edges = Vec::new();
    let mut half_edges = Vec::new();
    let mut half_edge_ids = Vec::new();

    for index in 0..=length {
        vertices.push(vertex(&format!("v{index}"), entity(302 + index as u64)));
    }

    for index in 0..length {
        let edge_id = entity(320 + index as u64);
        let half_edge_id = entity(340 + index as u64);

        edges.push(edge(&format!("e{index}"), edge_id));
        half_edges.push(half_edge_with_links(
            half_edge_id,
            &format!("he{index}"),
            None,
            Some(wire_id),
            Some(entity(340 + ((index + 1) % length) as u64)),
            Some(entity(340 + ((index + length - 1) % length) as u64)),
            Some(half_edge_id),
            Some(edge_id),
            Some(entity(302 + index as u64)),
            Some(entity(303 + index as u64)),
            None,
        ));
        half_edge_ids.push(half_edge_id);
    }

    TopologyView {
        wires: vec![crate::brep::topology_graph::TopologyWire {
            entity_id: wire_id,
            label: "chain".into(),
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

    let wire_id = entity(360);
    let center_vertex = entity(361);
    let mut vertices = vec![vertex("center", center_vertex)];
    let mut edges = Vec::new();
    let mut half_edges = Vec::new();
    let mut half_edge_ids = Vec::new();

    for index in 0..branch_count {
        let outer_vertex = entity(370 + index as u64);
        let edge_id = entity(390 + index as u64);
        let half_edge_id = entity(410 + index as u64);
        vertices.push(vertex(&format!("leaf{index}"), outer_vertex));
        edges.push(edge(&format!("branch{index}"), edge_id));
        half_edges.push(half_edge_with_links(
            half_edge_id,
            &format!("branch-he{index}"),
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
        wires: vec![crate::brep::topology_graph::TopologyWire {
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
