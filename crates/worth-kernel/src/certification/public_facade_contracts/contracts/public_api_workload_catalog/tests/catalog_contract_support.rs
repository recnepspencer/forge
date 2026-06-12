use worth_kernel::workload_composition::{
    GrazingBasketStackSpec, WorkloadCatalog, WorkloadCatalogRecipe,
};
use worth_spatial::facade::workload_vocabulary::{
    CompleteWorkloadEvidenceLedger, WorkloadEvidenceRow, WorkloadEvidenceStage,
};

pub(crate) fn admitted_catalog_recipes() -> Vec<WorkloadCatalogRecipe> {
    vec![
        WorkloadCatalog::cube(),
        WorkloadCatalog::tetrahedron(),
        WorkloadCatalog::single_face_loop(),
        WorkloadCatalog::coplanar_overlap_storm(),
        WorkloadCatalog::thin_feature_wall(),
        WorkloadCatalog::high_valence_vertex(),
        WorkloadCatalog::mixed_surface_kill_box(),
        WorkloadCatalog::grazing_open_shell_basket_stack(
            GrazingBasketStackSpec::new().layers(6).strips_per_layer(12),
        ),
        WorkloadCatalog::open_wire(),
        WorkloadCatalog::open_sheet(),
        WorkloadCatalog::open_shell_nmt_edge_fan(4),
        WorkloadCatalog::transform_cycle(),
        WorkloadCatalog::retained_cancellation_chain(),
    ]
}

pub(crate) fn assert_authority_stage(
    ledger: &CompleteWorkloadEvidenceLedger,
    stage: WorkloadEvidenceStage,
) {
    let row = ledger
        .row_for_stage(stage)
        .expect("catalog ledger must include authority stage");

    assert!(row.is_receipt_backed());
    assert!(row.is_admitted());
    assert!(!row.evidence_identity().trim().is_empty());
}

pub(crate) fn assert_real_geometry_breadth(ledger: &CompleteWorkloadEvidenceLedger) {
    let topology = stage_row(ledger, WorkloadEvidenceStage::Topology);
    assert!(topology.counters().topology_entity_count() > 0);
    assert!(topology.counters().topology_relation_count() > 0);

    let binding = stage_row(ledger, WorkloadEvidenceStage::GeometryBinding);
    assert!(binding.counters().binding_target_count() > 0);

    let projection = stage_row(ledger, WorkloadEvidenceStage::Projection);
    assert!(projection.counters().projected_entity_count() > 0);
    assert!(projection.counters().local_basis_part_count() > 0);

    let transform = stage_row(ledger, WorkloadEvidenceStage::Transform);
    assert!(transform.counters().transform_step_count() > 0);
}

pub(crate) fn assert_catalog_query_receipt_is_digest(digest: &str) {
    assert!(!digest.trim().is_empty());
    assert!(!digest.contains("catalog"));
    assert!(!digest.contains("workload"));
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct CatalogBreadth {
    pub topology_faces: usize,
    pub topology_entities: usize,
    pub topology_relations: usize,
    pub binding_targets: usize,
    pub projected_entities: usize,
    pub retained_artifacts: usize,
    pub replay_checkpoints: usize,
}

pub(crate) fn catalog_breadth(recipe: WorkloadCatalogRecipe) -> CatalogBreadth {
    let built = recipe.build().expect("catalog recipe should build");
    let ledger = built.workload().evidence_ledger();
    let topology = stage_row(ledger, WorkloadEvidenceStage::Topology).counters();
    let binding = stage_row(ledger, WorkloadEvidenceStage::GeometryBinding).counters();
    let projection = stage_row(ledger, WorkloadEvidenceStage::Projection).counters();
    let replay = stage_row(ledger, WorkloadEvidenceStage::RetainedReplay).counters();

    CatalogBreadth {
        topology_faces: topology.topology_face_count(),
        topology_entities: topology.topology_entity_count(),
        topology_relations: topology.topology_relation_count(),
        binding_targets: binding.binding_target_count(),
        projected_entities: projection.projected_entity_count(),
        retained_artifacts: replay.retained_artifact_count(),
        replay_checkpoints: replay.replay_checkpoint_count(),
    }
}

pub(crate) fn stage_row(
    ledger: &CompleteWorkloadEvidenceLedger,
    stage: WorkloadEvidenceStage,
) -> &WorkloadEvidenceRow {
    ledger.row_for_stage(stage).expect("catalog stage row")
}
