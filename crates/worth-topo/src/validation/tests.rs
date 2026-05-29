#[cfg(test)]
mod validator_tests {
    use forge_relational::facade::identity::PartitionId;
    use forge_relational::facade::payloads::RecordPayload;
    use forge_relational::facade::runtime::RelationalRuntimeApi;
    use forge_relational::facade::symbols::InternedString;
    use forge_relational::facade::transactions::{
        CreateIntent, EntitySpec, MutationIntent, TransactionOptions, WorkerIntentBatch,
    };
    use schema::facade::bootstrap_schema_registry;
    use schema::facade::topology_authoring::seed_minimal_topology;

    use crate::brep::topology_graph::{TopologyFace, TopologyLoop};
    use crate::facade::{
        validate_interpreted_topology, validate_named_topology_truth, validate_topology_view,
        TopologyMaterializer,
    };
    use crate::test_support::hostile_neighborhoods::validation_neighborhoods::{
        base_seeded_view, closed_shell_view, connected_wire_branch_view, edge, entity, half_edge,
        half_edge_with_links, open_sheet_patch_view, open_shell_nmt_fan_view, open_wire_chain_view,
        single_face_sheet_disk_view, tetrahedral_closed_shell_view, vertex,
    };

    #[test]
    fn seeded_topology_view_passes_milestone_one_validators() {
        let mut runtime = RelationalRuntimeApi::builder()
            .schema_registry(bootstrap_schema_registry().expect(" bootstrap schema registry"))
            .build();

        let seeded = seed_minimal_topology(&mut runtime, "validator").expect("seed  topology");
        let read_view = runtime
            .read_truth()
            .read_snapshot(&seeded.snapshot)
            .expect(" snapshot read");

        let topology = TopologyMaterializer::materialize_from_truth(&read_view)
            .expect(" topology materialization");

        let interpreted = crate::facade::interpret_topology_view(&topology);
        let report = validate_interpreted_topology(&topology, &interpreted)
            .expect("seeded topology should validate");
        assert!(report.rows.iter().any(|row| {
            row.validator == "ownership"
                && matches!(
                    row.phase,
                    crate::facade::TopologyValidationPhase::DerivedMaterialization
                )
        }));
        assert!(report.rows.iter().any(|row| {
            row.validator == "shell_closure"
                && matches!(
                    row.phase,
                    crate::facade::TopologyValidationPhase::DerivedInterpretation
                )
        }));
        validate_named_topology_truth(&read_view).expect("seeded topology should be fully named");
    }

    #[test]
    fn validator_rejects_topology_truth_missing_persistent_name() {
        let mut runtime = RelationalRuntimeApi::builder()
            .schema_registry(bootstrap_schema_registry().expect(" bootstrap schema registry"))
            .build();

        let seeded = seed_minimal_topology(&mut runtime, "missing-name").expect("seed  topology");

        let mut tx = runtime.begin_transaction(TransactionOptions::default());
        tx.push_batch(
            WorkerIntentBatch::new("-missing-name-vertex").push(MutationIntent::Create(
                CreateIntent::Entity(EntitySpec {
                    partition_id: PartitionId::main(),
                    kind_id: schema::facade::platform::entities::EntityKind::Topology(
                        schema::facade::platform::entities::TopologyEntityKind::Vertex,
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
        validate_named_topology_truth(&original_read)
            .expect("original seeded topology remains fully named");
    }

    #[test]
    fn validator_rejects_missing_prev_link() {
        let mut runtime = RelationalRuntimeApi::builder()
            .schema_registry(bootstrap_schema_registry().expect(" bootstrap schema registry"))
            .build();

        let seeded = seed_minimal_topology(&mut runtime, "validator").expect("seed  topology");
        let read_view = runtime
            .read_truth()
            .read_snapshot(&seeded.snapshot)
            .expect(" snapshot read");

        let mut topology = TopologyMaterializer::materialize_from_truth(&read_view)
            .expect(" topology materialization");
        topology.topology_mut().half_edges[0].prev_half_edge_id = None;

        let error = validate_topology_view(topology.topology())
            .expect_err("validator should reject missing prev");
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

        let he_a = half_edge(
            entity(102),
            "he-102",
            Some(wire_id),
            Some(edge_a),
            Some(shared_vertex),
            Some(shared_vertex),
            Some(face_id),
        );
        let he_b = half_edge(
            entity(103),
            "he-103",
            Some(wire_id),
            Some(edge_b),
            Some(shared_vertex),
            Some(shared_vertex),
            Some(face_id),
        );
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
        topology
            .vertices
            .push(vertex("vertex-110", disconnected_vertex));

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
        assert_eq!(error.validator(), "vertex_disks.wire_connectivity");
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
        validate_topology_view(&topology).expect("larger open-shell nmt edge fan should validate");
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

        topology.faces.push(TopologyFace {
            entity_id: face_c,
            label: "face-c".into(),
            shell_id: Some(topology.shells[0].entity_id),
            outer_loop_id: Some(loop_c),
            inner_loop_ids: vec![],
            boundary_half_edge_ids: vec![he12_c],
        });
        topology.loops.push(TopologyLoop {
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
}




