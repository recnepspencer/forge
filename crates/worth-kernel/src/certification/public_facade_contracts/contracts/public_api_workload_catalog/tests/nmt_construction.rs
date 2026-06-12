use super::catalog_contract_support::{assert_authority_stage, stage_row};
use topology::facade::{
    NmtTopologyConstruction, NmtTopologyPosture, OpenLayerPattern, OpenLayerStackSpec,
    OpenSheetPatchSpec,
};
use worth_kernel::workload_composition::{
    GrazingBasketStackSpec, WorkloadCatalog, WorkloadCatalogError, WorkloadCatalogSupportPosture,
};
use worth_spatial::facade::workload_vocabulary::WorkloadEvidenceStage;

#[test]
fn workload_catalog_consumes_nmt_construction_receipts_without_owning_rows() {
    let topology = NmtTopologyConstruction::open_layer_stack(
        OpenLayerStackSpec::new()
            .layers(4)
            .layer_pattern(OpenLayerPattern::sheet_patch(
                OpenSheetPatchSpec::new().strips(6),
            ))
            .with_layer_identity()
            .with_open_boundary_receipts()
            .with_radial_adjacency_receipts(),
    )
    .declared("catalog-owned NMT construction source")
    .construct()
    .expect("NMT construction should build before catalog consumption");

    let expected_identity = topology.pattern_identity().identity_digest().to_string();
    let expected_faces = topology.counters().face_count();

    let built = WorkloadCatalog::from_topology_construction(topology)
        .declared("catalog consumes NMT construction receipt")
        .build()
        .expect("catalog should consume construction receipt");

    let construction = built
        .topology_construction()
        .expect("built catalog workload exposes consumed NMT construction receipt");
    assert_eq!(
        construction.pattern_identity().identity_digest(),
        expected_identity
    );
    assert_eq!(construction.counters().face_count(), expected_faces);
    assert_eq!(
        stage_row(
            built.workload().evidence_ledger(),
            WorkloadEvidenceStage::Topology
        )
        .counters()
        .topology_face_count(),
        expected_faces
    );
    assert_eq!(
        built.support().posture(),
        WorkloadCatalogSupportPosture::Admitted
    );
}

#[test]
fn workload_catalog_open_class_recipes_compile_generic_nmt_construction() {
    let cases = [
        (
            WorkloadCatalog::open_wire(),
            NmtTopologyPosture::OpenWire,
            "open-wire-chain",
        ),
        (
            WorkloadCatalog::open_sheet(),
            NmtTopologyPosture::OpenSheet,
            "open-sheet-patch",
        ),
        (
            WorkloadCatalog::open_shell_nmt_edge_fan(4),
            NmtTopologyPosture::OpenNonManifold,
            "open-radial-fan",
        ),
    ];

    for (recipe, expected_posture, expected_pattern) in cases {
        let built = recipe
            .with_retained_replay_artifacts()
            .build()
            .expect("open-class catalog recipe should build through NMT construction");
        let construction = built
            .topology_construction()
            .expect("open-class workload exposes consumed NMT construction");

        assert_eq!(construction.topology_posture().posture(), expected_posture);
        assert_eq!(
            construction.pattern_identity().pattern_name(),
            expected_pattern
        );
        assert_eq!(
            built.support().posture(),
            WorkloadCatalogSupportPosture::Admitted
        );
        assert_authority_stage(
            built.workload().evidence_ledger(),
            WorkloadEvidenceStage::Topology,
        );
        assert_authority_stage(
            built.workload().evidence_ledger(),
            WorkloadEvidenceStage::RetainedReplay,
        );
    }
}

#[test]
fn workload_catalog_open_layer_stack_consumes_generic_layer_construction() {
    let built = WorkloadCatalog::open_layer_stack(
        OpenLayerStackSpec::new()
            .layers(3)
            .layer_pattern(OpenLayerPattern::sheet_patch(
                OpenSheetPatchSpec::new().strips(5),
            ))
            .with_layer_identity()
            .with_open_boundary_receipts()
            .with_radial_adjacency_receipts(),
    )
    .declared("catalog open layer stack workload")
    .build()
    .expect("layer stack catalog recipe should build");

    let construction = built
        .topology_construction()
        .expect("layer stack exposes construction receipt");

    assert_eq!(
        construction.topology_posture().posture(),
        NmtTopologyPosture::LayeredOpen
    );
    assert_eq!(construction.counters().layer_count(), 3);
    assert_eq!(construction.counters().face_count(), 15);
    assert_eq!(
        stage_row(
            built.workload().evidence_ledger(),
            WorkloadEvidenceStage::Topology
        )
        .counters()
        .topology_face_count(),
        15
    );
}

#[test]
fn workload_catalog_grazing_basket_stack_builds_open_layer_stack_storm_carrier() {
    let built = WorkloadCatalog::grazing_open_shell_basket_stack(
        GrazingBasketStackSpec::new().layers(6).strips_per_layer(12),
    )
    .declared("MB-M6-NMT-4 catalog grazing basket stack")
    .build()
    .expect("grazing basket stack catalog recipe should build");

    let construction = built
        .topology_construction()
        .expect("grazing basket stack exposes construction receipt");
    let topology = stage_row(
        built.workload().evidence_ledger(),
        WorkloadEvidenceStage::Topology,
    );
    let replay = stage_row(
        built.workload().evidence_ledger(),
        WorkloadEvidenceStage::RetainedReplay,
    );

    assert_eq!(
        construction.topology_posture().posture(),
        NmtTopologyPosture::LayeredOpen
    );
    assert_eq!(construction.counters().layer_count(), 6);
    assert_eq!(construction.counters().face_count(), 72);
    assert!(construction.open_boundary().boundary_half_edge_count() >= 6);
    assert_eq!(topology.counters().topology_face_count(), 72);
    assert!(replay.counters().retained_artifact_count() > 0);
    assert!(replay.counters().replay_checkpoint_count() > 0);
}

#[test]
fn workload_catalog_open_fan_invalid_cardinality_denies_through_construction() {
    let support = WorkloadCatalog::open_shell_nmt_edge_fan(2)
        .inspect_support()
        .expect("support inspection should produce a typed unsupported receipt");
    assert_eq!(
        support.posture(),
        WorkloadCatalogSupportPosture::Unsupported
    );
    assert!(support.human_reason().contains("open radial fan topology"));
    assert!(support.human_reason().contains("3 through 128"));
    assert!(support.human_reason().contains("requested 2"));

    let error = WorkloadCatalog::open_shell_nmt_edge_fan(2)
        .build()
        .expect_err("invalid NMT fan cardinality must deny before workload binding");

    match error {
        WorkloadCatalogError::UnsupportedRecipe { reason, .. } => {
            assert!(reason.contains("open radial fan topology"));
            assert!(reason.contains("3 through 128"));
            assert!(reason.contains("requested 2"));
        }
        other => panic!("expected unsupported NMT construction support denial, got {other:?}"),
    }
}

#[test]
fn workload_catalog_open_layer_stack_missing_evidence_denies_before_binding() {
    let missing_layer_identity = OpenLayerStackSpec::new().layers(3);
    assert_layer_stack_support_denial(
        missing_layer_identity,
        "layer identity receipts",
        "projection and replay",
    );

    let missing_open_boundary = OpenLayerStackSpec::new().layers(3).with_layer_identity();
    assert_layer_stack_support_denial(
        missing_open_boundary,
        "open-boundary receipts",
        "no-options outcomes",
    );

    let missing_radial_adjacency = OpenLayerStackSpec::new()
        .layers(3)
        .with_layer_identity()
        .with_open_boundary_receipts();
    assert_layer_stack_support_denial(
        missing_radial_adjacency,
        "radial-adjacency receipts",
        "ordinary sheet topology",
    );
}

#[test]
fn workload_catalog_open_layer_stack_invalid_cardinality_denies_before_binding() {
    let support = WorkloadCatalog::open_layer_stack(
        OpenLayerStackSpec::new()
            .layers(1)
            .with_layer_identity()
            .with_open_boundary_receipts()
            .with_radial_adjacency_receipts(),
    )
    .inspect_support()
    .expect("support inspection should expose invalid layer count");
    assert_eq!(
        support.posture(),
        WorkloadCatalogSupportPosture::Unsupported
    );
    assert!(support.human_reason().contains("open layer stack topology"));
    assert!(support.human_reason().contains("2 through 16"));
    assert!(support.human_reason().contains("requested 1"));
}

fn assert_layer_stack_support_denial(
    spec: OpenLayerStackSpec,
    required_reason: &str,
    required_context: &str,
) {
    let support = WorkloadCatalog::open_layer_stack(spec.clone())
        .inspect_support()
        .expect("support inspection should expose missing construction evidence");
    assert_eq!(
        support.posture(),
        WorkloadCatalogSupportPosture::Unsupported
    );
    assert!(support.human_reason().contains(required_reason));
    assert!(support.human_reason().contains(required_context));

    let error = WorkloadCatalog::open_layer_stack(spec)
        .build()
        .expect_err("missing layer evidence must deny before workload binding");
    match error {
        WorkloadCatalogError::UnsupportedRecipe { reason, .. } => {
            assert!(reason.contains(required_reason));
        }
        other => {
            panic!("expected unsupported layer-stack construction support denial, got {other:?}")
        }
    }
}
