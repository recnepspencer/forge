use forge_relational::facade::runtime::RelationalRuntimeApi;
use schema::facade::bootstrap_schema_registry;
use schema::facade::topology_authoring::seed_minimal_topology;

use crate::brep::topology_graph::{
    TopologyBody, TopologyFace, TopologyLoop, TopologyLump, TopologyModel, TopologyRegion,
    TopologyShell, TopologyView,
};
use crate::facade::TopologyMaterializer;

use super::primitives::*;

pub(crate) fn base_seeded_view(stem: &str) -> TopologyView {
    let mut runtime = RelationalRuntimeApi::builder()
        .schema_registry(bootstrap_schema_registry().expect(" bootstrap schema registry"))
        .build();

    let seeded = seed_minimal_topology(&mut runtime, stem).expect("seed  topology");
    let read_view = runtime
        .read_truth()
        .read_snapshot(&seeded.snapshot)
        .expect(" snapshot read");

    TopologyMaterializer::materialize_from_truth(&read_view)
        .expect(" topology materialization")
        .topology()
        .clone()
}

pub(crate) fn closed_shell_view() -> TopologyView {
    let model_id = entity(1);
    let body_id = entity(2);
    let lump_id = entity(3);
    let region_id = entity(4);
    let shell_id = entity(5);
    let face_a = entity(6);
    let face_b = entity(7);
    let loop_a = entity(8);
    let loop_b = entity(9);
    let v1 = entity(10);
    let v2 = entity(11);
    let v3 = entity(12);
    let e12 = entity(13);
    let e23 = entity(14);
    let e31 = entity(15);
    let he12_a = entity(16);
    let he23_a = entity(17);
    let he31_a = entity(18);
    let he21_b = entity(19);
    let he13_b = entity(20);
    let he32_b = entity(21);

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
            label: "solid-shell".into(),
            region_id: Some(region_id),
            face_ids: vec![face_a, face_b],
        }],
        faces: vec![
            TopologyFace {
                entity_id: face_a,
                label: "face-a".into(),
                shell_id: Some(shell_id),
                outer_loop_id: Some(loop_a),
                inner_loop_ids: vec![],
                boundary_half_edge_ids: vec![he12_a, he23_a, he31_a],
            },
            TopologyFace {
                entity_id: face_b,
                label: "face-b".into(),
                shell_id: Some(shell_id),
                outer_loop_id: Some(loop_b),
                inner_loop_ids: vec![],
                boundary_half_edge_ids: vec![he21_b, he13_b, he32_b],
            },
        ],
        loops: vec![
            TopologyLoop {
                entity_id: loop_a,
                label: "loop-a".into(),
                face_ids: vec![face_a],
                half_edge_ids: vec![he12_a, he23_a, he31_a],
            },
            TopologyLoop {
                entity_id: loop_b,
                label: "loop-b".into(),
                face_ids: vec![face_b],
                half_edge_ids: vec![he21_b, he13_b, he32_b],
            },
        ],
        wires: vec![],
        half_edges: vec![
            half_edge_with_links(
                he12_a,
                "he12-a",
                Some(loop_a),
                None,
                Some(he23_a),
                Some(he31_a),
                Some(he21_b),
                Some(e12),
                Some(v1),
                Some(v2),
                Some(face_a),
            ),
            half_edge_with_links(
                he23_a,
                "he23-a",
                Some(loop_a),
                None,
                Some(he31_a),
                Some(he12_a),
                Some(he32_b),
                Some(e23),
                Some(v2),
                Some(v3),
                Some(face_a),
            ),
            half_edge_with_links(
                he31_a,
                "he31-a",
                Some(loop_a),
                None,
                Some(he12_a),
                Some(he23_a),
                Some(he13_b),
                Some(e31),
                Some(v3),
                Some(v1),
                Some(face_a),
            ),
            half_edge_with_links(
                he21_b,
                "he21-b",
                Some(loop_b),
                None,
                Some(he13_b),
                Some(he32_b),
                Some(he12_a),
                Some(e12),
                Some(v2),
                Some(v1),
                Some(face_b),
            ),
            half_edge_with_links(
                he13_b,
                "he13-b",
                Some(loop_b),
                None,
                Some(he32_b),
                Some(he21_b),
                Some(he31_a),
                Some(e31),
                Some(v1),
                Some(v3),
                Some(face_b),
            ),
            half_edge_with_links(
                he32_b,
                "he32-b",
                Some(loop_b),
                None,
                Some(he21_b),
                Some(he13_b),
                Some(he23_a),
                Some(e23),
                Some(v3),
                Some(v2),
                Some(face_b),
            ),
        ],
        edges: vec![edge("e12", e12), edge("e23", e23), edge("e31", e31)],
        vertices: vec![vertex("v1", v1), vertex("v2", v2), vertex("v3", v3)],
    }
}
