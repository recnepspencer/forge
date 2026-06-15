use super::catalog_contract_support::stage_row;
use worth_kernel::workload_composition::{
    WorkloadCatalog, WorkloadCatalogError, WorkloadCatalogSupportPosture,
};
use worth_spatial::facade::workload_vocabulary::WorkloadEvidenceStage;

#[test]
fn boolean_catalog_recipes_build_real_workload_operand_pairs() {
    run_with_large_stack(|| {
        let clean = WorkloadCatalog::planar_boolean_clean_planar_body_pair()
            .build()
            .expect("clean planar pair should build");
        assert_real_operand_pair_ledgers(&clean.left().workload().evidence_ledger());
        assert_real_operand_pair_ledgers(&clean.right().workload().evidence_ledger());
        assert!(!clean.operand_pair_identity().trim().is_empty());
        assert_ne!(
            clean.left().declaration().query_declaration_digest(),
            clean.right().declaration().query_declaration_digest()
        );

        let coplanar = WorkloadCatalog::planar_boolean_coplanar_overlap_pair()
            .build()
            .expect("coplanar pair should build");
        assert_eq!(
            topology_face_count(coplanar.left().workload().evidence_ledger()),
            64
        );
        assert_eq!(
            topology_face_count(coplanar.right().workload().evidence_ledger()),
            64
        );

        let thin_feature = WorkloadCatalog::planar_boolean_thin_feature_pair()
            .build()
            .expect("thin-feature pair should build");
        assert!(
            topology_entity_count(thin_feature.left().workload().evidence_ledger())
                > topology_entity_count(thin_feature.right().workload().evidence_ledger())
        );
        assert!(
            binding_target_count(thin_feature.left().workload().evidence_ledger())
                > binding_target_count(thin_feature.right().workload().evidence_ledger())
        );
        assert!(
            projected_entity_count(thin_feature.left().workload().evidence_ledger())
                > projected_entity_count(thin_feature.right().workload().evidence_ledger())
        );

        let high_valence = WorkloadCatalog::planar_boolean_high_valence_contact_pair()
            .build()
            .expect("high-valence pair should build");
        assert_eq!(
            high_valence
                .left()
                .topology_neighborhood()
                .expect("high-valence left operand should expose neighborhood breadth")
                .valence(),
            5
        );
        assert!(high_valence.right().topology_neighborhood().is_none());
    });
}

#[test]
fn boolean_catalog_relevant_operand_pairs_preserve_retained_replay_receipts() {
    run_with_large_stack(|| {
        let coplanar = WorkloadCatalog::planar_boolean_coplanar_overlap_pair()
            .build()
            .expect("coplanar pair should build");
        assert!(coplanar.left().replay_receipts().is_some());
        assert!(coplanar.right().replay_receipts().is_some());

        let high_valence = WorkloadCatalog::planar_boolean_high_valence_contact_pair()
            .build()
            .expect("high-valence pair should build");
        assert!(high_valence.left().replay_receipts().is_some());
        assert!(high_valence.right().replay_receipts().is_none());

        let open = WorkloadCatalog::planar_boolean_open_unbounded_denial_pair()
            .build_denial()
            .expect("open denial pair should build");
        assert!(open.left_operand().replay_receipts().is_some());
        assert!(open.right_operand().replay_receipts().is_none());
    });
}

#[test]
fn boolean_clean_fail_catalog_recipes_deny_without_fabricating_workloads() {
    run_with_large_stack(|| {
        let dirty_pair = WorkloadCatalog::planar_boolean_dirty_clean_fail_pair();
        let dirty_support = dirty_pair
            .inspect_support()
            .expect("dirty pair support should be inspectable");
        assert_eq!(
            dirty_support.posture(),
            WorkloadCatalogSupportPosture::Admitted
        );
        assert!(dirty_support.human_reason().contains("clean-fail lane"));

        let dirty = dirty_pair
            .clone()
            .build_clean_fail()
            .expect("dirty pair should build clean-fail product");
        assert_eq!(
            dirty
                .left_clean_fail()
                .topology_clean_fail()
                .kind()
                .as_str(),
            "self-intersecting-loop"
        );
        assert_eq!(
            dirty
                .right_operand()
                .workload()
                .evidence_ledger()
                .counters()
                .rows(),
            8
        );

        match dirty_pair
            .build()
            .expect_err("dirty pair must not fabricate admitted workload pairs")
        {
            WorkloadCatalogError::UnsupportedRecipe { reason, .. } => {
                assert!(reason.contains("clean-fail lane"));
            }
            other => panic!("expected unsupported dirty pair build, got {other:?}"),
        }

        let open_pair = WorkloadCatalog::planar_boolean_open_unbounded_denial_pair();
        let open_support = open_pair
            .inspect_support()
            .expect("open pair support should be inspectable");
        assert_eq!(
            open_support.posture(),
            WorkloadCatalogSupportPosture::Admitted
        );
        assert!(open_support.human_reason().contains("denial lane"));

        let open = open_pair
            .clone()
            .build_denial()
            .expect("open pair should build denial product");
        assert_eq!(
            open.left_operand()
                .workload()
                .evidence_ledger()
                .counters()
                .rows(),
            8
        );
        assert_eq!(
            open.right_operand()
                .workload()
                .evidence_ledger()
                .counters()
                .rows(),
            8
        );
        assert!(open.denial_reason().contains("open or unbounded"));

        match open_pair
            .build()
            .expect_err("open pair must not fabricate admitted workload pairs")
        {
            WorkloadCatalogError::UnsupportedRecipe { reason, .. } => {
                assert!(reason.contains("denial lane"));
            }
            other => panic!("expected unsupported open pair build, got {other:?}"),
        }
    });
}

#[test]
fn boolean_catalog_support_rows_match_real_recipe_posture() {
    let admitted = WorkloadCatalog::planar_boolean_coplanar_overlap_pair()
        .inspect_support()
        .expect("coplanar pair support should inspect");
    assert_eq!(admitted.posture(), WorkloadCatalogSupportPosture::Admitted);
    assert!(admitted.human_reason().contains("real workload-backed"));

    let dirty = WorkloadCatalog::planar_boolean_dirty_clean_fail_pair()
        .inspect_support()
        .expect("dirty pair support should inspect");
    assert_eq!(dirty.posture(), WorkloadCatalogSupportPosture::Admitted);
    assert!(dirty.human_reason().contains("clean-fail lane"));

    let open = WorkloadCatalog::planar_boolean_open_unbounded_denial_pair()
        .inspect_support()
        .expect("open pair support should inspect");
    assert_eq!(open.posture(), WorkloadCatalogSupportPosture::Admitted);
    assert!(open.human_reason().contains("denial lane"));
}

#[test]
fn boolean_operand_pair_rejects_blank_declaration_before_lane_errors() {
    assert_eq!(
        WorkloadCatalog::planar_boolean_dirty_clean_fail_pair()
            .declared("   ")
            .build()
            .expect_err("blank declaration must fail before lane denial"),
        WorkloadCatalogError::MissingDeclaration
    );
    assert_eq!(
        WorkloadCatalog::planar_boolean_open_unbounded_denial_pair()
            .declared("   ")
            .build()
            .expect_err("blank declaration must fail before lane denial"),
        WorkloadCatalogError::MissingDeclaration
    );
    assert_eq!(
        WorkloadCatalog::planar_boolean_clean_planar_body_pair()
            .declared("   ")
            .build_clean_fail()
            .expect_err("blank declaration must fail before wrong clean-fail lane denial"),
        WorkloadCatalogError::MissingDeclaration
    );
    assert_eq!(
        WorkloadCatalog::planar_boolean_clean_planar_body_pair()
            .declared("   ")
            .build_denial()
            .expect_err("blank declaration must fail before wrong denial lane error"),
        WorkloadCatalogError::MissingDeclaration
    );
}

fn run_with_large_stack(body: impl FnOnce() + Send + 'static) {
    std::thread::Builder::new()
        .name("boolean-operand-pair-contract".to_string())
        .stack_size(16 * 1024 * 1024)
        .spawn(body)
        .expect("boolean operand-pair contract thread should spawn")
        .join()
        .expect("boolean operand-pair contract thread should finish");
}

fn assert_real_operand_pair_ledgers(
    ledger: &worth_spatial::facade::workload_vocabulary::CompleteWorkloadEvidenceLedger,
) {
    assert_eq!(ledger.counters().rows(), 8);
    assert!(topology_entity_count(ledger) > 0);
    assert!(binding_target_count(ledger) > 0);
    assert!(projected_entity_count(ledger) > 0);
}

fn topology_face_count(
    ledger: &worth_spatial::facade::workload_vocabulary::CompleteWorkloadEvidenceLedger,
) -> usize {
    stage_row(ledger, WorkloadEvidenceStage::Topology)
        .counters()
        .topology_face_count()
}

fn topology_entity_count(
    ledger: &worth_spatial::facade::workload_vocabulary::CompleteWorkloadEvidenceLedger,
) -> usize {
    stage_row(ledger, WorkloadEvidenceStage::Topology)
        .counters()
        .topology_entity_count()
}

fn binding_target_count(
    ledger: &worth_spatial::facade::workload_vocabulary::CompleteWorkloadEvidenceLedger,
) -> usize {
    stage_row(ledger, WorkloadEvidenceStage::GeometryBinding)
        .counters()
        .binding_target_count()
}

fn projected_entity_count(
    ledger: &worth_spatial::facade::workload_vocabulary::CompleteWorkloadEvidenceLedger,
) -> usize {
    stage_row(ledger, WorkloadEvidenceStage::Projection)
        .counters()
        .projected_entity_count()
}
