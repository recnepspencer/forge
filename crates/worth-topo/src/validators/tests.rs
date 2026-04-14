#[cfg(test)]
mod validator_tests {
    use forge_relational::facade::identity::{EntityId, PartitionId};
    use forge_relational::facade::payloads::RecordPayload;
    use forge_relational::facade::runtime::RelationalRuntimeApi;
    use forge_relational::facade::symbols::InternedString;
    use forge_relational::facade::transactions::{
        CreateIntent, EntitySpec, MutationIntent, TransactionOptions, WorkerIntentBatch,
    };
    use worth_schema::facade::{seed_minimal_topology, worth_bootstrap_schema_registry};

    use crate::data::topology_view::{
        WorthTopologyBody, WorthTopologyEdge, WorthTopologyFace, WorthTopologyHalfEdge,
        WorthTopologyLoop, WorthTopologyLump, WorthTopologyModel, WorthTopologyRegion,
        WorthTopologyShell, WorthTopologyVertex, WorthTopologyView,
    };
    use crate::facade::{
        validate_named_topology_truth, validate_topology_view, WorthTopologyMaterializer,
    };

    #[test]
    fn seeded_topology_view_passes_milestone_one_validators() {
        let mut runtime = RelationalRuntimeApi::builder()
            .schema_registry(
                worth_bootstrap_schema_registry().expect("worth bootstrap schema registry"),
            )
            .build();

        let seeded = seed_minimal_topology(&mut runtime, "validator").expect("seed worth topology");
        let read_view = runtime
            .read_truth()
            .read_snapshot(&seeded.snapshot)
            .expect("worth snapshot read");

        let topology = WorthTopologyMaterializer::materialize_from_truth(&read_view)
            .expect("worth topology materialization");

        validate_topology_view(&topology).expect("seeded topology should validate");
        validate_named_topology_truth(&read_view).expect("seeded topology should be fully named");
    }

    #[test]
    fn validator_rejects_topology_truth_missing_persistent_name() {
        let mut runtime = RelationalRuntimeApi::builder()
            .schema_registry(
                worth_bootstrap_schema_registry().expect("worth bootstrap schema registry"),
            )
            .build();

        let seeded = seed_minimal_topology(&mut runtime, "missing-name").expect("seed worth topology");

        let mut tx = runtime.begin_transaction(TransactionOptions::default());
        tx.push_batch(
            WorkerIntentBatch::new("worth-missing-name-vertex").push(MutationIntent::Create(
                CreateIntent::Entity(EntitySpec {
                    partition_id: PartitionId::main(),
                    kind_id: worth_schema::facade::WorthEntityKind::Topology(
                        worth_schema::facade::WorthTopologyEntityKind::Vertex,
                    )
                    .kind_id(),
                    client_key: InternedString::Raw("missing-name.vertex".to_string()),
                    payload: RecordPayload::StructuredJson(serde_json::json!({
                        "label": "missing-name.vertex"
                    })),
                }),
            )),
        );
        let commit = tx.commit().expect("commit unnamed topology entity");
        let read_view = runtime
            .read_truth()
            .read_snapshot(&commit.snapshot)
            .expect("read updated snapshot");

        let error = validate_named_topology_truth(&read_view)
            .expect_err("unnamed topology entity should fail naming validation");
        assert_eq!(error.validator(), "naming.persistent_name_coverage");

        let original_read = runtime
            .read_truth()
            .read_snapshot(&seeded.snapshot)
            .expect("read seeded snapshot");
        validate_named_topology_truth(&original_read).expect("original seeded topology remains fully named");
    }

    #[test]
    fn validator_rejects_missing_prev_link() {
        let mut runtime = RelationalRuntimeApi::builder()
            .schema_registry(
                worth_bootstrap_schema_registry().expect("worth bootstrap schema registry"),
            )
            .build();

        let seeded = seed_minimal_topology(&mut runtime, "validator").expect("seed worth topology");
        let read_view = runtime
            .read_truth()
            .read_snapshot(&seeded.snapshot)
            .expect("worth snapshot read");

        let mut topology = WorthTopologyMaterializer::materialize_from_truth(&read_view)
            .expect("worth topology materialization");
        topology.half_edges[0].prev_half_edge_id = None;

        let error = validate_topology_view(&topology).expect_err("validator should reject missing prev");
        assert_eq!(error.validator(), "loop_wiring.prev_next_symmetry");
    }

    #[test]
    fn validator_accepts_connected_wire_branch_family_member() {
        let mut topology = base_seeded_view("wire-branch");
        let shared_vertex = topology.vertices[0].entity_id;
        let wire_id = topology.wires[0].entity_id;
        let face_id = topology.faces[0].entity_id;

        let edge_a = entity(100);
        let edge_b = entity(101);
        topology.edges.push(edge("edge-100", edge_a));
        topology.edges.push(edge("edge-101", edge_b));

        let he_a = half_edge(entity(102), "he-102", Some(wire_id), Some(edge_a), Some(shared_vertex), Some(shared_vertex), Some(face_id));
        let he_b = half_edge(entity(103), "he-103", Some(wire_id), Some(edge_b), Some(shared_vertex), Some(shared_vertex), Some(face_id));
        topology.half_edges.push(he_a);
        topology.half_edges.push(he_b);
        topology.wires[0].half_edge_ids.push(entity(102));
        topology.wires[0].half_edge_ids.push(entity(103));

        validate_topology_view(&topology).expect("connected wire branch should validate");
    }

    #[test]
    fn validator_accepts_larger_connected_wire_branch_family_member() {
        let topology = connected_wire_branch_view(4);
        validate_topology_view(&topology).expect("larger connected wire branch should validate");
    }

    #[test]
    fn validator_rejects_disconnected_wire_branch_family_member() {
        let mut topology = base_seeded_view("wire-branch");
        let wire_id = topology.wires[0].entity_id;
        let face_id = topology.faces[0].entity_id;

        let disconnected_vertex = entity(110);
        topology.vertices.push(vertex("vertex-110", disconnected_vertex));

        let edge_a = entity(111);
        let edge_b = entity(112);
        topology.edges.push(edge("edge-111", edge_a));
        topology.edges.push(edge("edge-112", edge_b));

        topology.half_edges.push(half_edge(
            entity(113),
            "he-113",
            Some(wire_id),
            Some(edge_a),
            Some(disconnected_vertex),
            Some(disconnected_vertex),
            Some(face_id),
        ));
        topology.half_edges.push(half_edge(
            entity(114),
            "he-114",
            Some(wire_id),
            Some(edge_b),
            Some(disconnected_vertex),
            Some(disconnected_vertex),
            Some(face_id),
        ));
        topology.wires[0].half_edge_ids.push(entity(113));
        topology.wires[0].half_edge_ids.push(entity(114));

        let error =
            validate_topology_view(&topology).expect_err("disconnected wire branch should fail");
        assert_eq!(error.validator(), "vertex_branching.wire_connectivity");
    }

    #[test]
    fn validator_accepts_closed_solid_shell_family_member() {
        let topology = closed_shell_view();
        validate_topology_view(&topology).expect("closed shell family member should validate");
    }

    #[test]
    fn validator_accepts_larger_closed_solid_shell_family_member() {
        let topology = tetrahedral_closed_shell_view();
        validate_topology_view(&topology)
            .expect("larger closed shell family member should validate");
    }

    #[test]
    fn validator_accepts_larger_open_shell_nmt_edge_fan_family_member() {
        let topology = open_shell_nmt_fan_view(4);
        validate_topology_view(&topology)
            .expect("larger open-shell nmt edge fan should validate");
    }

    #[test]
    fn validator_accepts_single_face_sheet_disk_family_member() {
        let topology = single_face_sheet_disk_view(5);
        validate_topology_view(&topology).expect("single-face sheet disk should validate");
    }

    #[test]
    fn validator_accepts_multi_face_sheet_patch_family_member() {
        let topology = open_sheet_patch_view(3);
        validate_topology_view(&topology).expect("multi-face sheet patch should validate");
    }

    #[test]
    fn validator_accepts_longer_open_wire_chain_family_member() {
        let topology = open_wire_chain_view(4);
        validate_topology_view(&topology).expect("longer open wire chain should validate");
    }

    #[test]
    fn validator_rejects_closed_shell_with_non_manifold_radial_fan() {
        let mut topology = closed_shell_view();
        let face_c = entity(22);
        let loop_c = entity(23);
        let he12_c = entity(24);

        topology.faces.push(WorthTopologyFace {
            entity_id: face_c,
            label: "face-c".into(),
            shell_id: Some(topology.shells[0].entity_id),
            outer_loop_id: Some(loop_c),
            inner_loop_ids: vec![],
            boundary_half_edge_ids: vec![he12_c],
        });
        topology.loops.push(WorthTopologyLoop {
            entity_id: loop_c,
            label: "loop-c".into(),
            face_ids: vec![face_c],
            half_edge_ids: vec![he12_c],
        });
        topology.shells[0].face_ids.push(face_c);
        topology.half_edges.push(half_edge_with_links(
            he12_c,
            "he12-c",
            Some(loop_c),
            None,
            Some(he12_c),
            Some(he12_c),
            Some(topology.half_edges[0].entity_id),
            topology.half_edges[0].edge_id,
            topology.half_edges[0].origin_vertex_id,
            topology.half_edges[0].target_vertex_id,
            Some(face_c),
        ));
        topology.half_edges[0].radial_next_half_edge_id = Some(topology.half_edges[3].entity_id);
        topology.half_edges[3].radial_next_half_edge_id = Some(he12_c);

        let error = validate_topology_view(&topology)
            .expect_err("closed shell with valence-3 fan should fail");
        assert_eq!(error.validator(), "shell_closure.closed_shell_manifold");
    }

    fn base_seeded_view(stem: &str) -> WorthTopologyView {
        let mut runtime = RelationalRuntimeApi::builder()
            .schema_registry(
                worth_bootstrap_schema_registry().expect("worth bootstrap schema registry"),
            )
            .build();

        let seeded = seed_minimal_topology(&mut runtime, stem).expect("seed worth topology");
        let read_view = runtime
            .read_truth()
            .read_snapshot(&seeded.snapshot)
            .expect("worth snapshot read");

        WorthTopologyMaterializer::materialize_from_truth(&read_view)
            .expect("worth topology materialization")
    }

    fn closed_shell_view() -> WorthTopologyView {
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

        WorthTopologyView {
            models: vec![WorthTopologyModel {
                entity_id: model_id,
                label: "model".into(),
                body_ids: vec![body_id],
            }],
            bodies: vec![WorthTopologyBody {
                entity_id: body_id,
                label: "body".into(),
                model_id: Some(model_id),
                lump_ids: vec![lump_id],
            }],
            lumps: vec![WorthTopologyLump {
                entity_id: lump_id,
                label: "lump".into(),
                body_id: Some(body_id),
                region_ids: vec![region_id],
            }],
            regions: vec![WorthTopologyRegion {
                entity_id: region_id,
                label: "region".into(),
                lump_id: Some(lump_id),
                shell_ids: vec![shell_id],
            }],
            shells: vec![WorthTopologyShell {
                entity_id: shell_id,
                label: "solid-shell".into(),
                region_id: Some(region_id),
                face_ids: vec![face_a, face_b],
            }],
            faces: vec![
                WorthTopologyFace {
                    entity_id: face_a,
                    label: "face-a".into(),
                    shell_id: Some(shell_id),
                    outer_loop_id: Some(loop_a),
                    inner_loop_ids: vec![],
                    boundary_half_edge_ids: vec![he12_a, he23_a, he31_a],
                },
                WorthTopologyFace {
                    entity_id: face_b,
                    label: "face-b".into(),
                    shell_id: Some(shell_id),
                    outer_loop_id: Some(loop_b),
                    inner_loop_ids: vec![],
                    boundary_half_edge_ids: vec![he21_b, he13_b, he32_b],
                },
            ],
            loops: vec![
                WorthTopologyLoop {
                    entity_id: loop_a,
                    label: "loop-a".into(),
                    face_ids: vec![face_a],
                    half_edge_ids: vec![he12_a, he23_a, he31_a],
                },
                WorthTopologyLoop {
                    entity_id: loop_b,
                    label: "loop-b".into(),
                    face_ids: vec![face_b],
                    half_edge_ids: vec![he21_b, he13_b, he32_b],
                },
            ],
            wires: vec![],
            half_edges: vec![
                half_edge_with_links(he12_a, "he12-a", Some(loop_a), None, Some(he23_a), Some(he31_a), Some(he21_b), Some(e12), Some(v1), Some(v2), Some(face_a)),
                half_edge_with_links(he23_a, "he23-a", Some(loop_a), None, Some(he31_a), Some(he12_a), Some(he32_b), Some(e23), Some(v2), Some(v3), Some(face_a)),
                half_edge_with_links(he31_a, "he31-a", Some(loop_a), None, Some(he12_a), Some(he23_a), Some(he13_b), Some(e31), Some(v3), Some(v1), Some(face_a)),
                half_edge_with_links(he21_b, "he21-b", Some(loop_b), None, Some(he13_b), Some(he32_b), Some(he12_a), Some(e12), Some(v2), Some(v1), Some(face_b)),
                half_edge_with_links(he13_b, "he13-b", Some(loop_b), None, Some(he32_b), Some(he21_b), Some(he31_a), Some(e31), Some(v1), Some(v3), Some(face_b)),
                half_edge_with_links(he32_b, "he32-b", Some(loop_b), None, Some(he21_b), Some(he13_b), Some(he23_a), Some(e23), Some(v3), Some(v2), Some(face_b)),
            ],
            edges: vec![edge("e12", e12), edge("e23", e23), edge("e31", e31)],
            vertices: vec![vertex("v1", v1), vertex("v2", v2), vertex("v3", v3)],
        }
    }

    fn tetrahedral_closed_shell_view() -> WorthTopologyView {
        let model_id = entity(200);
        let body_id = entity(201);
        let lump_id = entity(202);
        let region_id = entity(203);
        let shell_id = entity(204);

        let vertices = [entity(210), entity(211), entity(212), entity(213)];
        let edges_ids = [entity(220), entity(221), entity(222), entity(223), entity(224), entity(225)];
        let face_ids = [entity(230), entity(231), entity(232), entity(233)];
        let loop_ids = [entity(240), entity(241), entity(242), entity(243)];
        let half_edge_ids = [
            entity(250), entity(251), entity(252), entity(253), entity(254), entity(255),
            entity(256), entity(257), entity(258), entity(259), entity(260), entity(261),
        ];

        WorthTopologyView {
            models: vec![WorthTopologyModel { entity_id: model_id, label: "model".into(), body_ids: vec![body_id] }],
            bodies: vec![WorthTopologyBody { entity_id: body_id, label: "body".into(), model_id: Some(model_id), lump_ids: vec![lump_id] }],
            lumps: vec![WorthTopologyLump { entity_id: lump_id, label: "lump".into(), body_id: Some(body_id), region_ids: vec![region_id] }],
            regions: vec![WorthTopologyRegion { entity_id: region_id, label: "region".into(), lump_id: Some(lump_id), shell_ids: vec![shell_id] }],
            shells: vec![WorthTopologyShell { entity_id: shell_id, label: "tetra".into(), region_id: Some(region_id), face_ids: face_ids.to_vec() }],
            faces: vec![
                WorthTopologyFace { entity_id: face_ids[0], label: "f012".into(), shell_id: Some(shell_id), outer_loop_id: Some(loop_ids[0]), inner_loop_ids: vec![], boundary_half_edge_ids: vec![half_edge_ids[0], half_edge_ids[1], half_edge_ids[2]] },
                WorthTopologyFace { entity_id: face_ids[1], label: "f013".into(), shell_id: Some(shell_id), outer_loop_id: Some(loop_ids[1]), inner_loop_ids: vec![], boundary_half_edge_ids: vec![half_edge_ids[3], half_edge_ids[4], half_edge_ids[5]] },
                WorthTopologyFace { entity_id: face_ids[2], label: "f123".into(), shell_id: Some(shell_id), outer_loop_id: Some(loop_ids[2]), inner_loop_ids: vec![], boundary_half_edge_ids: vec![half_edge_ids[6], half_edge_ids[7], half_edge_ids[8]] },
                WorthTopologyFace { entity_id: face_ids[3], label: "f023".into(), shell_id: Some(shell_id), outer_loop_id: Some(loop_ids[3]), inner_loop_ids: vec![], boundary_half_edge_ids: vec![half_edge_ids[9], half_edge_ids[10], half_edge_ids[11]] },
            ],
            loops: vec![
                WorthTopologyLoop { entity_id: loop_ids[0], label: "l012".into(), face_ids: vec![face_ids[0]], half_edge_ids: vec![half_edge_ids[0], half_edge_ids[1], half_edge_ids[2]] },
                WorthTopologyLoop { entity_id: loop_ids[1], label: "l013".into(), face_ids: vec![face_ids[1]], half_edge_ids: vec![half_edge_ids[3], half_edge_ids[4], half_edge_ids[5]] },
                WorthTopologyLoop { entity_id: loop_ids[2], label: "l123".into(), face_ids: vec![face_ids[2]], half_edge_ids: vec![half_edge_ids[6], half_edge_ids[7], half_edge_ids[8]] },
                WorthTopologyLoop { entity_id: loop_ids[3], label: "l023".into(), face_ids: vec![face_ids[3]], half_edge_ids: vec![half_edge_ids[9], half_edge_ids[10], half_edge_ids[11]] },
            ],
            wires: vec![],
            half_edges: vec![
                half_edge_with_links(half_edge_ids[0], "he01", Some(loop_ids[0]), None, Some(half_edge_ids[1]), Some(half_edge_ids[2]), Some(half_edge_ids[5]), Some(edges_ids[0]), Some(vertices[0]), Some(vertices[1]), Some(face_ids[0])),
                half_edge_with_links(half_edge_ids[1], "he12", Some(loop_ids[0]), None, Some(half_edge_ids[2]), Some(half_edge_ids[0]), Some(half_edge_ids[8]), Some(edges_ids[1]), Some(vertices[1]), Some(vertices[2]), Some(face_ids[0])),
                half_edge_with_links(half_edge_ids[2], "he20", Some(loop_ids[0]), None, Some(half_edge_ids[0]), Some(half_edge_ids[1]), Some(half_edge_ids[10]), Some(edges_ids[2]), Some(vertices[2]), Some(vertices[0]), Some(face_ids[0])),
                half_edge_with_links(half_edge_ids[3], "he03", Some(loop_ids[1]), None, Some(half_edge_ids[4]), Some(half_edge_ids[5]), Some(half_edge_ids[11]), Some(edges_ids[3]), Some(vertices[0]), Some(vertices[3]), Some(face_ids[1])),
                half_edge_with_links(half_edge_ids[4], "he31", Some(loop_ids[1]), None, Some(half_edge_ids[5]), Some(half_edge_ids[3]), Some(half_edge_ids[7]), Some(edges_ids[4]), Some(vertices[3]), Some(vertices[1]), Some(face_ids[1])),
                half_edge_with_links(half_edge_ids[5], "he10", Some(loop_ids[1]), None, Some(half_edge_ids[3]), Some(half_edge_ids[4]), Some(half_edge_ids[0]), Some(edges_ids[0]), Some(vertices[1]), Some(vertices[0]), Some(face_ids[1])),
                half_edge_with_links(half_edge_ids[6], "he23", Some(loop_ids[2]), None, Some(half_edge_ids[7]), Some(half_edge_ids[8]), Some(half_edge_ids[9]), Some(edges_ids[5]), Some(vertices[2]), Some(vertices[3]), Some(face_ids[2])),
                half_edge_with_links(half_edge_ids[7], "he31b", Some(loop_ids[2]), None, Some(half_edge_ids[8]), Some(half_edge_ids[6]), Some(half_edge_ids[4]), Some(edges_ids[4]), Some(vertices[3]), Some(vertices[1]), Some(face_ids[2])),
                half_edge_with_links(half_edge_ids[8], "he12b", Some(loop_ids[2]), None, Some(half_edge_ids[6]), Some(half_edge_ids[7]), Some(half_edge_ids[1]), Some(edges_ids[1]), Some(vertices[1]), Some(vertices[2]), Some(face_ids[2])),
                half_edge_with_links(half_edge_ids[9], "he32", Some(loop_ids[3]), None, Some(half_edge_ids[10]), Some(half_edge_ids[11]), Some(half_edge_ids[6]), Some(edges_ids[5]), Some(vertices[3]), Some(vertices[2]), Some(face_ids[3])),
                half_edge_with_links(half_edge_ids[10], "he20b", Some(loop_ids[3]), None, Some(half_edge_ids[11]), Some(half_edge_ids[9]), Some(half_edge_ids[2]), Some(edges_ids[2]), Some(vertices[2]), Some(vertices[0]), Some(face_ids[3])),
                half_edge_with_links(half_edge_ids[11], "he02", Some(loop_ids[3]), None, Some(half_edge_ids[9]), Some(half_edge_ids[10]), Some(half_edge_ids[3]), Some(edges_ids[3]), Some(vertices[0]), Some(vertices[3]), Some(face_ids[3])),
            ],
            edges: vec![
                edge("e01", edges_ids[0]),
                edge("e12", edges_ids[1]),
                edge("e20", edges_ids[2]),
                edge("e03", edges_ids[3]),
                edge("e31", edges_ids[4]),
                edge("e23", edges_ids[5]),
            ],
            vertices: vec![
                vertex("v0", vertices[0]),
                vertex("v1", vertices[1]),
                vertex("v2", vertices[2]),
                vertex("v3", vertices[3]),
            ],
        }
    }

    fn open_wire_chain_view(length: usize) -> WorthTopologyView {
        assert!(length >= 2, "open wire chain requires at least two half-edges");

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

        WorthTopologyView {
            wires: vec![crate::data::topology_view::WorthTopologyWire {
                entity_id: wire_id,
                label: "chain".into(),
                half_edge_ids,
            }],
            half_edges,
            edges,
            vertices,
            ..WorthTopologyView::default()
        }
    }

    fn connected_wire_branch_view(branch_count: usize) -> WorthTopologyView {
        assert!(branch_count >= 3, "connected wire branch requires at least three arms");

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

        WorthTopologyView {
            wires: vec![crate::data::topology_view::WorthTopologyWire {
                entity_id: wire_id,
                label: "branch".into(),
                half_edge_ids,
            }],
            half_edges,
            edges,
            vertices,
            ..WorthTopologyView::default()
        }
    }

    fn open_shell_nmt_fan_view(fan_size: usize) -> WorthTopologyView {
        assert!(fan_size >= 3, "nmt fan requires at least three incident faces");

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

            faces.push(WorthTopologyFace {
                entity_id: face_id,
                label: format!("fanf{index}"),
                shell_id: Some(shell_id),
                outer_loop_id: Some(loop_id),
                inner_loop_ids: vec![],
                boundary_half_edge_ids: vec![shared_he, side_a, side_b],
            });
            loops.push(WorthTopologyLoop {
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
            if let Some(record) = half_edges.iter_mut().find(|record| record.entity_id == current) {
                record.radial_next_half_edge_id = Some(next);
            }
        }

        WorthTopologyView {
            models: vec![WorthTopologyModel { entity_id: model_id, label: "model".into(), body_ids: vec![body_id] }],
            bodies: vec![WorthTopologyBody { entity_id: body_id, label: "body".into(), model_id: Some(model_id), lump_ids: vec![lump_id] }],
            lumps: vec![WorthTopologyLump { entity_id: lump_id, label: "lump".into(), body_id: Some(body_id), region_ids: vec![region_id] }],
            regions: vec![WorthTopologyRegion { entity_id: region_id, label: "region".into(), lump_id: Some(lump_id), shell_ids: vec![shell_id] }],
            shells: vec![WorthTopologyShell { entity_id: shell_id, label: "sheet".into(), region_id: Some(region_id), face_ids }],
            faces,
            loops,
            half_edges,
            edges,
            vertices,
            ..WorthTopologyView::default()
        }
    }

    fn single_face_sheet_disk_view(edge_count: usize) -> WorthTopologyView {
        assert!(edge_count >= 3, "sheet disk requires at least three boundary edges");

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
            vertices.push(vertex(&format!("disk-v{index}"), entity(710 + index as u64)));
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

        WorthTopologyView {
            models: vec![WorthTopologyModel { entity_id: model_id, label: "model".into(), body_ids: vec![body_id] }],
            bodies: vec![WorthTopologyBody { entity_id: body_id, label: "body".into(), model_id: Some(model_id), lump_ids: vec![lump_id] }],
            lumps: vec![WorthTopologyLump { entity_id: lump_id, label: "lump".into(), body_id: Some(body_id), region_ids: vec![region_id] }],
            regions: vec![WorthTopologyRegion { entity_id: region_id, label: "region".into(), lump_id: Some(lump_id), shell_ids: vec![shell_id] }],
            shells: vec![WorthTopologyShell { entity_id: shell_id, label: "sheet-disk".into(), region_id: Some(region_id), face_ids: vec![face_id] }],
            faces: vec![WorthTopologyFace {
                entity_id: face_id,
                label: "disk-face".into(),
                shell_id: Some(shell_id),
                outer_loop_id: Some(loop_id),
                inner_loop_ids: vec![],
                boundary_half_edge_ids: half_edge_ids.clone(),
            }],
            loops: vec![WorthTopologyLoop {
                entity_id: loop_id,
                label: "disk-loop".into(),
                face_ids: vec![face_id],
                half_edge_ids,
            }],
            half_edges,
            edges,
            vertices,
            ..WorthTopologyView::default()
        }
    }

    fn open_sheet_patch_view(face_count: usize) -> WorthTopologyView {
        assert!(face_count >= 2, "sheet patch requires at least two faces");
        assert!(face_count <= 3, "test helper currently supports up to three faces");

        let model_id = entity(800);
        let body_id = entity(801);
        let lump_id = entity(802);
        let region_id = entity(803);
        let shell_id = entity(804);

        let vertex_ids = [entity(810), entity(811), entity(812), entity(813), entity(814)];
        let edge_ids = [entity(820), entity(821), entity(822), entity(823), entity(824), entity(825), entity(826)];
        let face_ids = [entity(830), entity(831), entity(832)];
        let loop_ids = [entity(840), entity(841), entity(842)];
        let half_edge_ids = [
            entity(850), entity(851), entity(852),
            entity(853), entity(854), entity(855),
            entity(856), entity(857), entity(858),
        ];

        let all_faces = vec![
            WorthTopologyFace { entity_id: face_ids[0], label: "patch-f0".into(), shell_id: Some(shell_id), outer_loop_id: Some(loop_ids[0]), inner_loop_ids: vec![], boundary_half_edge_ids: vec![half_edge_ids[0], half_edge_ids[1], half_edge_ids[2]] },
            WorthTopologyFace { entity_id: face_ids[1], label: "patch-f1".into(), shell_id: Some(shell_id), outer_loop_id: Some(loop_ids[1]), inner_loop_ids: vec![], boundary_half_edge_ids: vec![half_edge_ids[3], half_edge_ids[4], half_edge_ids[5]] },
            WorthTopologyFace { entity_id: face_ids[2], label: "patch-f2".into(), shell_id: Some(shell_id), outer_loop_id: Some(loop_ids[2]), inner_loop_ids: vec![], boundary_half_edge_ids: vec![half_edge_ids[6], half_edge_ids[7], half_edge_ids[8]] },
        ];
        let all_loops = vec![
            WorthTopologyLoop { entity_id: loop_ids[0], label: "patch-l0".into(), face_ids: vec![face_ids[0]], half_edge_ids: vec![half_edge_ids[0], half_edge_ids[1], half_edge_ids[2]] },
            WorthTopologyLoop { entity_id: loop_ids[1], label: "patch-l1".into(), face_ids: vec![face_ids[1]], half_edge_ids: vec![half_edge_ids[3], half_edge_ids[4], half_edge_ids[5]] },
            WorthTopologyLoop { entity_id: loop_ids[2], label: "patch-l2".into(), face_ids: vec![face_ids[2]], half_edge_ids: vec![half_edge_ids[6], half_edge_ids[7], half_edge_ids[8]] },
        ];
        let all_half_edges = vec![
            half_edge_with_links(half_edge_ids[0], "patch-he0", Some(loop_ids[0]), None, Some(half_edge_ids[1]), Some(half_edge_ids[2]), Some(half_edge_ids[0]), Some(edge_ids[0]), Some(vertex_ids[0]), Some(vertex_ids[1]), Some(face_ids[0])),
            half_edge_with_links(half_edge_ids[1], "patch-he1", Some(loop_ids[0]), None, Some(half_edge_ids[2]), Some(half_edge_ids[0]), Some(half_edge_ids[5]), Some(edge_ids[1]), Some(vertex_ids[1]), Some(vertex_ids[4]), Some(face_ids[0])),
            half_edge_with_links(half_edge_ids[2], "patch-he2", Some(loop_ids[0]), None, Some(half_edge_ids[0]), Some(half_edge_ids[1]), Some(half_edge_ids[2]), Some(edge_ids[2]), Some(vertex_ids[4]), Some(vertex_ids[0]), Some(face_ids[0])),
            half_edge_with_links(half_edge_ids[3], "patch-he3", Some(loop_ids[1]), None, Some(half_edge_ids[4]), Some(half_edge_ids[5]), Some(half_edge_ids[3]), Some(edge_ids[3]), Some(vertex_ids[1]), Some(vertex_ids[2]), Some(face_ids[1])),
            half_edge_with_links(half_edge_ids[4], "patch-he4", Some(loop_ids[1]), None, Some(half_edge_ids[5]), Some(half_edge_ids[3]), Some(half_edge_ids[8]), Some(edge_ids[4]), Some(vertex_ids[2]), Some(vertex_ids[4]), Some(face_ids[1])),
            half_edge_with_links(half_edge_ids[5], "patch-he5", Some(loop_ids[1]), None, Some(half_edge_ids[3]), Some(half_edge_ids[4]), Some(half_edge_ids[1]), Some(edge_ids[1]), Some(vertex_ids[4]), Some(vertex_ids[1]), Some(face_ids[1])),
            half_edge_with_links(half_edge_ids[6], "patch-he6", Some(loop_ids[2]), None, Some(half_edge_ids[7]), Some(half_edge_ids[8]), Some(half_edge_ids[6]), Some(edge_ids[5]), Some(vertex_ids[2]), Some(vertex_ids[3]), Some(face_ids[2])),
            half_edge_with_links(half_edge_ids[7], "patch-he7", Some(loop_ids[2]), None, Some(half_edge_ids[8]), Some(half_edge_ids[6]), Some(half_edge_ids[7]), Some(edge_ids[6]), Some(vertex_ids[3]), Some(vertex_ids[4]), Some(face_ids[2])),
            half_edge_with_links(half_edge_ids[8], "patch-he8", Some(loop_ids[2]), None, Some(half_edge_ids[6]), Some(half_edge_ids[7]), Some(half_edge_ids[4]), Some(edge_ids[4]), Some(vertex_ids[4]), Some(vertex_ids[2]), Some(face_ids[2])),
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

        WorthTopologyView {
            models: vec![WorthTopologyModel { entity_id: model_id, label: "model".into(), body_ids: vec![body_id] }],
            bodies: vec![WorthTopologyBody { entity_id: body_id, label: "body".into(), model_id: Some(model_id), lump_ids: vec![lump_id] }],
            lumps: vec![WorthTopologyLump { entity_id: lump_id, label: "lump".into(), body_id: Some(body_id), region_ids: vec![region_id] }],
            regions: vec![WorthTopologyRegion { entity_id: region_id, label: "region".into(), lump_id: Some(lump_id), shell_ids: vec![shell_id] }],
            shells: vec![WorthTopologyShell { entity_id: shell_id, label: "sheet-patch".into(), region_id: Some(region_id), face_ids: face_ids[..face_count].to_vec() }],
            faces: all_faces[..face_count].to_vec(),
            loops: all_loops[..face_count].to_vec(),
            half_edges: all_half_edges[..(face_count * 3)].to_vec(),
            edges: all_edges,
            vertices,
            ..WorthTopologyView::default()
        }
    }

    fn entity(slot: u64) -> EntityId {
        EntityId::new(PartitionId::main(), slot, 1)
    }

    fn edge(label: &str, entity_id: EntityId) -> WorthTopologyEdge {
        WorthTopologyEdge {
            entity_id,
            label: label.into(),
        }
    }

    fn vertex(label: &str, entity_id: EntityId) -> WorthTopologyVertex {
        WorthTopologyVertex {
            entity_id,
            label: label.into(),
        }
    }

    fn half_edge(
        entity_id: EntityId,
        label: &str,
        wire_id: Option<EntityId>,
        edge_id: Option<EntityId>,
        origin_vertex_id: Option<EntityId>,
        target_vertex_id: Option<EntityId>,
        face_id: Option<EntityId>,
    ) -> WorthTopologyHalfEdge {
        half_edge_with_links(
            entity_id,
            label,
            None,
            wire_id,
            Some(entity_id),
            Some(entity_id),
            Some(entity_id),
            edge_id,
            origin_vertex_id,
            target_vertex_id,
            face_id,
        )
    }

    fn half_edge_with_links(
        entity_id: EntityId,
        label: &str,
        loop_id: Option<EntityId>,
        wire_id: Option<EntityId>,
        next_half_edge_id: Option<EntityId>,
        prev_half_edge_id: Option<EntityId>,
        radial_next_half_edge_id: Option<EntityId>,
        edge_id: Option<EntityId>,
        origin_vertex_id: Option<EntityId>,
        target_vertex_id: Option<EntityId>,
        face_id: Option<EntityId>,
    ) -> WorthTopologyHalfEdge {
        WorthTopologyHalfEdge {
            entity_id,
            label: label.into(),
            loop_id,
            wire_id,
            next_half_edge_id,
            prev_half_edge_id,
            radial_next_half_edge_id,
            edge_id,
            origin_vertex_id,
            target_vertex_id,
            face_id,
        }
    }
}
