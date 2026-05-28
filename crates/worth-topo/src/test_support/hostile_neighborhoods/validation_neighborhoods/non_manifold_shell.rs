use crate::brep::topology_graph::{
    TopologyBody, TopologyFace, TopologyLoop, TopologyLump, TopologyModel, TopologyRegion,
    TopologyShell, TopologyView,
};

use super::primitives::*;

pub(crate) fn open_shell_nmt_fan_view(fan_size: usize) -> TopologyView {
    assert!(
        fan_size >= 3,
        "nmt fan requires at least three incident faces"
    );

    let model_id = entity(500);
    let body_id = entity(501);
    let lump_id = entity(502);
    let region_id = entity(503);
    let shell_id = entity(504);
    let edge_shared = entity(505);
    let v1 = entity(506);
    let v2 = entity(507);

    let mut faces = Vec::new();
    let mut loops = Vec::new();
    let mut half_edges = Vec::new();
    let mut edges = vec![edge("shared", edge_shared)];
    let mut vertices = vec![vertex("v1", v1), vertex("v2", v2)];
    let mut face_ids = Vec::new();
    let mut shared_half_edge_ids = Vec::new();

    for index in 0..fan_size {
        let face_id = entity(520 + index as u64);
        let loop_id = entity(540 + index as u64);
        let third_vertex = entity(560 + index as u64);
        let shared_he = entity(580 + index as u64 * 3);
        let side_a = entity(581 + index as u64 * 3);
        let side_b = entity(582 + index as u64 * 3);
        let edge_a = entity(620 + index as u64 * 2);
        let edge_b = entity(621 + index as u64 * 2);

        let (shared_origin, shared_target) = if index == 0 { (v1, v2) } else { (v2, v1) };
        let (side_a_origin, side_a_target) = if index == 0 {
            (v2, third_vertex)
        } else {
            (v1, third_vertex)
        };
        let (side_b_origin, side_b_target) = if index == 0 {
            (third_vertex, v1)
        } else {
            (third_vertex, v2)
        };

        face_ids.push(face_id);
        shared_half_edge_ids.push(shared_he);
        vertices.push(vertex(&format!("fanv{index}"), third_vertex));
        edges.push(edge(&format!("fane{index}a"), edge_a));
        edges.push(edge(&format!("fane{index}b"), edge_b));

        faces.push(TopologyFace {
            entity_id: face_id,
            label: format!("fanf{index}"),
            shell_id: Some(shell_id),
            outer_loop_id: Some(loop_id),
            inner_loop_ids: vec![],
            boundary_half_edge_ids: vec![shared_he, side_a, side_b],
        });
        loops.push(TopologyLoop {
            entity_id: loop_id,
            label: format!("fanl{index}"),
            face_ids: vec![face_id],
            half_edge_ids: vec![shared_he, side_a, side_b],
        });
        half_edges.push(half_edge_with_links(
            shared_he,
            &format!("shared-he{index}"),
            Some(loop_id),
            None,
            Some(side_a),
            Some(side_b),
            None,
            Some(edge_shared),
            Some(shared_origin),
            Some(shared_target),
            Some(face_id),
        ));
        half_edges.push(half_edge_with_links(
            side_a,
            &format!("side-a{index}"),
            Some(loop_id),
            None,
            Some(side_b),
            Some(shared_he),
            Some(side_a),
            Some(edge_a),
            Some(side_a_origin),
            Some(side_a_target),
            Some(face_id),
        ));
        half_edges.push(half_edge_with_links(
            side_b,
            &format!("side-b{index}"),
            Some(loop_id),
            None,
            Some(shared_he),
            Some(side_a),
            Some(side_b),
            Some(edge_b),
            Some(side_b_origin),
            Some(side_b_target),
            Some(face_id),
        ));
    }

    for index in 0..fan_size {
        let current = shared_half_edge_ids[index];
        let next = shared_half_edge_ids[(index + 1) % fan_size];
        if let Some(record) = half_edges
            .iter_mut()
            .find(|record| record.entity_id == current)
        {
            record.radial_next_half_edge_id = Some(next);
        }
    }

    TopologyView {
        models: vec![TopologyModel {
            entity_id: model_id,
            label: "model".into(),
            body_ids: vec![body_id],
        }],
        bodies: vec![TopologyBody {
            entity_id: body_id,
            label: "body".into(),
            model_id: Some(model_id),
            lump_ids: vec![lump_id],
        }],
        lumps: vec![TopologyLump {
            entity_id: lump_id,
            label: "lump".into(),
            body_id: Some(body_id),
            region_ids: vec![region_id],
        }],
        regions: vec![TopologyRegion {
            entity_id: region_id,
            label: "region".into(),
            lump_id: Some(lump_id),
            shell_ids: vec![shell_id],
        }],
        shells: vec![TopologyShell {
            entity_id: shell_id,
            label: "sheet".into(),
            region_id: Some(region_id),
            face_ids,
        }],
        faces,
        loops,
        half_edges,
        edges,
        vertices,
        ..TopologyView::default()
    }
}




