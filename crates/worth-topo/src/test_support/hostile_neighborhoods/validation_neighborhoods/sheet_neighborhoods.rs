use crate::brep::topology_graph::{
    TopologyBody, TopologyFace, TopologyLoop, TopologyLump, TopologyModel, TopologyRegion,
    TopologyShell, TopologyView,
};

use super::primitives::*;

pub(crate) fn single_face_sheet_disk_view(edge_count: usize) -> TopologyView {
    assert!(
        edge_count >= 3,
        "sheet disk requires at least three boundary edges"
    );

    let model_id = entity(700);
    let body_id = entity(701);
    let lump_id = entity(702);
    let region_id = entity(703);
    let shell_id = entity(704);
    let face_id = entity(705);
    let loop_id = entity(706);

    let mut vertices = Vec::new();
    let mut edges = Vec::new();
    let mut half_edges = Vec::new();
    let mut half_edge_ids = Vec::new();

    for index in 0..edge_count {
        vertices.push(vertex(
            &format!("disk-v{index}"),
            entity(710 + index as u64),
        ));
    }

    for index in 0..edge_count {
        let edge_id = entity(730 + index as u64);
        let half_edge_id = entity(750 + index as u64);
        let origin = entity(710 + index as u64);
        let target = entity(710 + ((index + 1) % edge_count) as u64);
        let next = entity(750 + ((index + 1) % edge_count) as u64);
        let prev = entity(750 + ((index + edge_count - 1) % edge_count) as u64);

        edges.push(edge(&format!("disk-e{index}"), edge_id));
        half_edges.push(half_edge_with_links(
            half_edge_id,
            "disk-he",
            Some(loop_id),
            None,
            Some(next),
            Some(prev),
            Some(half_edge_id),
            Some(edge_id),
            Some(origin),
            Some(target),
            Some(face_id),
        ));
        half_edge_ids.push(half_edge_id);
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
            label: "sheet-disk".into(),
            region_id: Some(region_id),
            face_ids: vec![face_id],
        }],
        faces: vec![TopologyFace {
            entity_id: face_id,
            label: "disk-face".into(),
            shell_id: Some(shell_id),
            outer_loop_id: Some(loop_id),
            inner_loop_ids: vec![],
            boundary_half_edge_ids: half_edge_ids.clone(),
        }],
        loops: vec![TopologyLoop {
            entity_id: loop_id,
            label: "disk-loop".into(),
            face_ids: vec![face_id],
            half_edge_ids,
        }],
        half_edges,
        edges,
        vertices,
        ..TopologyView::default()
    }
}

pub(crate) fn open_sheet_patch_view(face_count: usize) -> TopologyView {
    assert!(face_count >= 2, "sheet patch requires at least two faces");
    assert!(
        face_count <= 3,
        "test helper currently supports up to three faces"
    );

    let model_id = entity(800);
    let body_id = entity(801);
    let lump_id = entity(802);
    let region_id = entity(803);
    let shell_id = entity(804);

    let vertex_ids = [
        entity(810),
        entity(811),
        entity(812),
        entity(813),
        entity(814),
    ];
    let edge_ids = [
        entity(820),
        entity(821),
        entity(822),
        entity(823),
        entity(824),
        entity(825),
        entity(826),
    ];
    let face_ids = [entity(830), entity(831), entity(832)];
    let loop_ids = [entity(840), entity(841), entity(842)];
    let half_edge_ids = [
        entity(850),
        entity(851),
        entity(852),
        entity(853),
        entity(854),
        entity(855),
        entity(856),
        entity(857),
        entity(858),
    ];

    let all_faces = vec![
        TopologyFace {
            entity_id: face_ids[0],
            label: "patch-f0".into(),
            shell_id: Some(shell_id),
            outer_loop_id: Some(loop_ids[0]),
            inner_loop_ids: vec![],
            boundary_half_edge_ids: vec![half_edge_ids[0], half_edge_ids[1], half_edge_ids[2]],
        },
        TopologyFace {
            entity_id: face_ids[1],
            label: "patch-f1".into(),
            shell_id: Some(shell_id),
            outer_loop_id: Some(loop_ids[1]),
            inner_loop_ids: vec![],
            boundary_half_edge_ids: vec![half_edge_ids[3], half_edge_ids[4], half_edge_ids[5]],
        },
        TopologyFace {
            entity_id: face_ids[2],
            label: "patch-f2".into(),
            shell_id: Some(shell_id),
            outer_loop_id: Some(loop_ids[2]),
            inner_loop_ids: vec![],
            boundary_half_edge_ids: vec![half_edge_ids[6], half_edge_ids[7], half_edge_ids[8]],
        },
    ];
    let all_loops = vec![
        TopologyLoop {
            entity_id: loop_ids[0],
            label: "patch-l0".into(),
            face_ids: vec![face_ids[0]],
            half_edge_ids: vec![half_edge_ids[0], half_edge_ids[1], half_edge_ids[2]],
        },
        TopologyLoop {
            entity_id: loop_ids[1],
            label: "patch-l1".into(),
            face_ids: vec![face_ids[1]],
            half_edge_ids: vec![half_edge_ids[3], half_edge_ids[4], half_edge_ids[5]],
        },
        TopologyLoop {
            entity_id: loop_ids[2],
            label: "patch-l2".into(),
            face_ids: vec![face_ids[2]],
            half_edge_ids: vec![half_edge_ids[6], half_edge_ids[7], half_edge_ids[8]],
        },
    ];
    let all_half_edges = vec![
        half_edge_with_links(
            half_edge_ids[0],
            "patch-he0",
            Some(loop_ids[0]),
            None,
            Some(half_edge_ids[1]),
            Some(half_edge_ids[2]),
            Some(half_edge_ids[0]),
            Some(edge_ids[0]),
            Some(vertex_ids[0]),
            Some(vertex_ids[1]),
            Some(face_ids[0]),
        ),
        half_edge_with_links(
            half_edge_ids[1],
            "patch-he1",
            Some(loop_ids[0]),
            None,
            Some(half_edge_ids[2]),
            Some(half_edge_ids[0]),
            Some(half_edge_ids[5]),
            Some(edge_ids[1]),
            Some(vertex_ids[1]),
            Some(vertex_ids[4]),
            Some(face_ids[0]),
        ),
        half_edge_with_links(
            half_edge_ids[2],
            "patch-he2",
            Some(loop_ids[0]),
            None,
            Some(half_edge_ids[0]),
            Some(half_edge_ids[1]),
            Some(half_edge_ids[2]),
            Some(edge_ids[2]),
            Some(vertex_ids[4]),
            Some(vertex_ids[0]),
            Some(face_ids[0]),
        ),
        half_edge_with_links(
            half_edge_ids[3],
            "patch-he3",
            Some(loop_ids[1]),
            None,
            Some(half_edge_ids[4]),
            Some(half_edge_ids[5]),
            Some(half_edge_ids[3]),
            Some(edge_ids[3]),
            Some(vertex_ids[1]),
            Some(vertex_ids[2]),
            Some(face_ids[1]),
        ),
        half_edge_with_links(
            half_edge_ids[4],
            "patch-he4",
            Some(loop_ids[1]),
            None,
            Some(half_edge_ids[5]),
            Some(half_edge_ids[3]),
            Some(half_edge_ids[8]),
            Some(edge_ids[4]),
            Some(vertex_ids[2]),
            Some(vertex_ids[4]),
            Some(face_ids[1]),
        ),
        half_edge_with_links(
            half_edge_ids[5],
            "patch-he5",
            Some(loop_ids[1]),
            None,
            Some(half_edge_ids[3]),
            Some(half_edge_ids[4]),
            Some(half_edge_ids[1]),
            Some(edge_ids[1]),
            Some(vertex_ids[4]),
            Some(vertex_ids[1]),
            Some(face_ids[1]),
        ),
        half_edge_with_links(
            half_edge_ids[6],
            "patch-he6",
            Some(loop_ids[2]),
            None,
            Some(half_edge_ids[7]),
            Some(half_edge_ids[8]),
            Some(half_edge_ids[6]),
            Some(edge_ids[5]),
            Some(vertex_ids[2]),
            Some(vertex_ids[3]),
            Some(face_ids[2]),
        ),
        half_edge_with_links(
            half_edge_ids[7],
            "patch-he7",
            Some(loop_ids[2]),
            None,
            Some(half_edge_ids[8]),
            Some(half_edge_ids[6]),
            Some(half_edge_ids[7]),
            Some(edge_ids[6]),
            Some(vertex_ids[3]),
            Some(vertex_ids[4]),
            Some(face_ids[2]),
        ),
        half_edge_with_links(
            half_edge_ids[8],
            "patch-he8",
            Some(loop_ids[2]),
            None,
            Some(half_edge_ids[6]),
            Some(half_edge_ids[7]),
            Some(half_edge_ids[4]),
            Some(edge_ids[4]),
            Some(vertex_ids[4]),
            Some(vertex_ids[2]),
            Some(face_ids[2]),
        ),
    ];
    let all_edges = vec![
        edge("patch-e0", edge_ids[0]),
        edge("patch-e1", edge_ids[1]),
        edge("patch-e2", edge_ids[2]),
        edge("patch-e3", edge_ids[3]),
        edge("patch-e4", edge_ids[4]),
        edge("patch-e5", edge_ids[5]),
        edge("patch-e6", edge_ids[6]),
    ];
    let vertices = vertex_ids
        .iter()
        .enumerate()
        .map(|(index, entity_id)| vertex(&format!("patch-v{index}"), *entity_id))
        .collect::<Vec<_>>();

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
            label: "sheet-patch".into(),
            region_id: Some(region_id),
            face_ids: face_ids[..face_count].to_vec(),
        }],
        faces: all_faces[..face_count].to_vec(),
        loops: all_loops[..face_count].to_vec(),
        half_edges: all_half_edges[..(face_count * 3)].to_vec(),
        edges: all_edges,
        vertices,
        ..TopologyView::default()
    }
}
