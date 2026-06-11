use topology::facade::{TopologySeed, TopologySeedKind, TopologySeedTopologyPosture};

#[test]
fn topology_workload_seeds_build_real_topology_truth() {
    let cases = [
        (TopologySeed::cube(), TopologySeedKind::Cube, 6, 12, 8),
        (
            TopologySeed::tetrahedron(),
            TopologySeedKind::Tetrahedron,
            4,
            6,
            4,
        ),
        (
            TopologySeed::single_face_loop(5),
            TopologySeedKind::SingleFaceLoop,
            1,
            5,
            5,
        ),
        (
            TopologySeed::multi_face_shell(7),
            TopologySeedKind::MultiFaceShell,
            7,
            15,
            10,
        ),
        (
            TopologySeed::open_sheet(),
            TopologySeedKind::OpenSheet,
            1,
            4,
            4,
        ),
        (
            TopologySeed::open_wire(),
            TopologySeedKind::OpenWire,
            0,
            4,
            5,
        ),
    ];

    for (recipe, expected_kind, expected_faces, expected_edges, expected_vertices) in cases {
        let receipt = recipe.build().unwrap();
        let counters = receipt.counters();

        assert_eq!(receipt.kind(), expected_kind);
        assert_eq!(receipt.topology_posture(), expected_kind.topology_posture());
        assert_eq!(counters.face_count(), expected_faces);
        assert_eq!(counters.edge_count(), expected_edges);
        assert_eq!(counters.vertex_count(), expected_vertices);
        assert!(counters.total_topology_entities() > 0);
        assert!(receipt.entity_identities().model_ids().len() == counters.model_count());
        assert_eq!(
            receipt.entity_identities().edge_ids().len(),
            counters.edge_count()
        );
        assert_eq!(
            receipt.entity_identities().vertex_ids().len(),
            counters.vertex_count()
        );
        assert!(receipt.validation().row_count() >= 5);
        assert_eq!(
            receipt
                .query_receipts()
                .declaration_receipt()
                .identity()
                .name(),
            expected_kind.default_declaration_for_test()
        );
        assert_eq!(
            receipt.query_receipts().query_surface(),
            format!(".topology.seed.{}", expected_kind.as_str())
        );
        assert!(receipt.neighborhood().is_none());
        assert!(receipt.can_enter_spatial_binding());
    }
}

#[test]
fn topology_workload_shell_seeds_admit_the_full_small_face_range() {
    for face_count in 4..=64 {
        let receipt = TopologySeed::multi_face_shell(face_count).build().unwrap();
        assert_eq!(receipt.kind(), TopologySeedKind::MultiFaceShell);
        assert_eq!(
            receipt.topology_posture(),
            TopologySeedTopologyPosture::ClosedValid
        );
        assert_eq!(receipt.counters().face_count(), face_count);
        assert_eq!(receipt.entity_identities().face_ids().len(), face_count);
        assert_eq!(receipt.entity_identities().loop_ids().len(), face_count);
        assert!(receipt.validation().row_count() >= 5);
    }
}

#[test]
fn topology_workload_high_valence_seed_builds_neighborhood_receipt() {
    let receipt = TopologySeed::high_valence_vertex().build().unwrap();
    let neighborhood = receipt.neighborhood().unwrap();

    assert_eq!(receipt.kind(), TopologySeedKind::HighValenceVertex);
    assert_eq!(
        receipt.topology_posture(),
        TopologySeedTopologyPosture::OpenValid
    );
    assert_eq!(neighborhood.valence(), 5);
    assert_eq!(neighborhood.incident_half_edge_ids().len(), 5);
    assert_eq!(receipt.counters().face_count(), 5);
    assert_eq!(receipt.counters().edge_count(), 10);
    assert_eq!(receipt.counters().vertex_count(), 6);
    assert_eq!(receipt.entity_identities().face_ids().len(), 5);
    assert_eq!(receipt.entity_identities().loop_ids().len(), 5);
    assert!(receipt
        .entity_identities()
        .vertex_ids()
        .contains(&neighborhood.center_vertex_id()));
}

trait SeedKindTestName {
    fn default_declaration_for_test(self) -> String;
}

impl SeedKindTestName for TopologySeedKind {
    fn default_declaration_for_test(self) -> String {
        format!("topology seed {}", self.as_str())
    }
}
