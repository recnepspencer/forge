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

    let model_id = entity(20);
    let body_id = entity(21);
    let lump_id = entity(22);
    let region_id = entity(23);
    let shell_id = entity(24);
    let edge_shared = entity(31);
    let v1 = entity(32);
    let v2 = entity(33);

    let mut faces = Vec::new();
    let mut loops = Vec::new();
    let mut half_edges = Vec::new();
    let mut edges = vec![edge("shared", edge_shared)];
    let mut vertices = vec![vertex("v1", v1), vertex("v2", v2)];
    let mut face_ids = Vec::new();
    let mut shared_half_edge_ids = Vec::new();

    for index in 0..fan_size {
        let face_id = entity(100 + index as u64);
        let loop_id = entity(120 + index as u64);
        let third_vertex = entity(140 + index as u64);
        let first_shared_he = entity(160 + index as u64 * 3);
        let second_he = entity(161 + index as u64 * 3);
        let third_he = entity(162 + index as u64 * 3);
        let edge_a = entity(200 + index as u64 * 2);
        let edge_b = entity(201 + index as u64 * 2);

        let (shared_origin, shared_target) = if index == 0 { (v1, v2) } else { (v2, v1) };
        let (second_origin, second_target) = if index == 0 {
            (v2, third_vertex)
        } else {
            (v1, third_vertex)
        };
        let (third_origin, third_target) = if index == 0 {
            (third_vertex, v1)
        } else {
            (third_vertex, v2)
        };

        face_ids.push(face_id);
        shared_half_edge_ids.push(first_shared_he);
        vertices.push(vertex(&format!("vf{index}"), third_vertex));
        edges.push(edge(&format!("ea{index}"), edge_a));
        edges.push(edge(&format!("eb{index}"), edge_b));

        faces.push(TopologyFace {
            entity_id: face_id,
            label: format!("f{index}"),
            shell_id: Some(shell_id),
            outer_loop_id: Some(loop_id),
            inner_loop_ids: vec![],
            boundary_half_edge_ids: vec![first_shared_he, second_he, third_he],
        });
        loops.push(TopologyLoop {
            entity_id: loop_id,
            label: format!("l{index}"),
            face_ids: vec![face_id],
            half_edge_ids: vec![first_shared_he, second_he, third_he],
        });
        half_edges.push(half_edge_full(
            first_shared_he,
            Some(loop_id),
            None,
            Some(second_he),
            Some(third_he),
            None,
            Some(edge_shared),
            Some(shared_origin),
            Some(shared_target),
            Some(face_id),
        ));
        half_edges.push(half_edge_full(
            second_he,
            Some(loop_id),
            None,
            Some(third_he),
            Some(first_shared_he),
            Some(second_he),
            Some(edge_a),
            Some(second_origin),
            Some(second_target),
            Some(face_id),
        ));
        half_edges.push(half_edge_full(
            third_he,
            Some(loop_id),
            None,
            Some(first_shared_he),
            Some(second_he),
            Some(third_he),
            Some(edge_b),
            Some(third_origin),
            Some(third_target),
            Some(face_id),
        ));
    }

    for index in 0..fan_size {
        let next_shared = shared_half_edge_ids[(index + 1) % fan_size];
        let current_shared = shared_half_edge_ids[index];
        if let Some(record) = half_edges
            .iter_mut()
            .find(|record| record.entity_id == current_shared)
        {
            record.radial_next_half_edge_id = Some(next_shared);
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
