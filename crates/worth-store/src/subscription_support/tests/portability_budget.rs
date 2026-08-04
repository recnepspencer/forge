use super::super::{
    new, pipeline, SubscriptionSupportActionOrigin, SubscriptionSupportClassificationViolation,
    SubscriptionSupportOperationalVerdict, SubscriptionSupportPortabilityDecision,
    SubscriptionSupportPortabilityDecisionKind, SubscriptionSupportPortabilityOutcome,
    SubscriptionSupportPublicationPipeline, SupportActionBreadthBudget, SupportActionId,
    SupportAllocationScope, SupportPathClass, SupportPortabilityManifestBudget,
    SupportProgramDensityClass,
};
use super::StoreErrorKind;
use super::{portability_basis, retention_basis};

#[test]
fn phase_4_portability_rejects_oversized_manifest_before_materialization() {
    let mut pipeline = SubscriptionSupportPublicationPipeline::first_ship();
    let error = pipeline
        .admit_support_portability_batch(
            SupportActionId::new("support-portability:oversized").unwrap(),
            vec![
                portability_basis(SubscriptionSupportActionOrigin::ReplicationExport, "big-a"),
                portability_basis(SubscriptionSupportActionOrigin::ReplicationExport, "big-b"),
            ],
            2,
            0,
            SupportPortabilityManifestBudget::new(1, 64).unwrap(),
            SubscriptionSupportPortabilityDecision::full_scope_replication(
                "support-identity:oversized",
                "support-identity:oversized",
            )
            .unwrap(),
            SupportPathClass::ReplicationExport,
            SupportProgramDensityClass::PortabilityScopeBatch,
            SupportAllocationScope::PortabilityManifest,
            SupportActionBreadthBudget::new(4, 1024).unwrap(),
            128,
        )
        .expect_err("oversized capsule support manifests must reject before payload decode");

    assert_eq!(
        error.kind(),
        &StoreErrorKind::SubscriptionSupportClassificationViolation
    );
    assert_eq!(pipeline.counters().support_portability_plan_count(), 0);
    assert_eq!(
        pipeline
            .counters()
            .support_capsule_manifest_budget_denial_count(),
        1
    );
}

#[test]
fn phase_4_unsupported_family_portability_rejects_typed() {
    let mut pipeline = SubscriptionSupportPublicationPipeline::first_ship();
    let plan = pipeline
        .admit_support_portability_batch(
            SupportActionId::new("support-portability:unsupported-family").unwrap(),
            vec![portability_basis(
                SubscriptionSupportActionOrigin::ReplicationImport,
                "unsupported-family",
            )],
            0,
            1,
            SupportPortabilityManifestBudget::new(4, 1024).unwrap(),
            SubscriptionSupportPortabilityDecision::unsupported_family_rejected(
                "target store does not admit this support family",
            )
            .unwrap(),
            SupportPathClass::ImportAdmission,
            SupportProgramDensityClass::PortabilityScopeBatch,
            SupportAllocationScope::PortabilityManifest,
            SupportActionBreadthBudget::new(4, 1024).unwrap(),
            128,
        )
        .unwrap();
    let report = pipeline
        .publish_support_portability_consequence(plan)
        .unwrap();
    let SubscriptionSupportPortabilityOutcome::Rejected(rejection) = report.outcome() else {
        panic!("unsupported support portability must publish a typed rejection");
    };

    assert_eq!(
        rejection.rejection_kind(),
        SubscriptionSupportPortabilityDecisionKind::UnsupportedFamilyRejected
    );
    assert_eq!(
        report.completed_action().envelope().verdict(),
        SubscriptionSupportOperationalVerdict::RejectedByPolicy
    );
    assert_eq!(pipeline.counters().support_import_rejection_count(), 1);
}

#[test]
fn phase_4_portability_rejects_wrong_origin_hot_path_and_non_portability_density() {
    let mut pipeline = SubscriptionSupportPublicationPipeline::first_ship();
    let wrong_origin = pipeline
        .admit_support_portability_batch(
            SupportActionId::new("support-portability:wrong-origin").unwrap(),
            vec![retention_basis("wrong-origin")],
            1,
            0,
            SupportPortabilityManifestBudget::new(4, 1024).unwrap(),
            SubscriptionSupportPortabilityDecision::full_scope_replication(
                "support-identity:wrong-origin",
                "support-identity:wrong-origin",
            )
            .unwrap(),
            SupportPathClass::ReplicationExport,
            SupportProgramDensityClass::PortabilityScopeBatch,
            SupportAllocationScope::PortabilityManifest,
            SupportActionBreadthBudget::new(4, 1024).unwrap(),
            128,
        )
        .expect_err("portability affected sets cannot consume retention-origin bases");
    assert_eq!(
        wrong_origin.kind(),
        &StoreErrorKind::SubscriptionSupportClassificationViolation
    );

    let hot_path = pipeline
        .admit_support_portability_batch(
            SupportActionId::new("support-portability:hot-path").unwrap(),
            vec![portability_basis(
                SubscriptionSupportActionOrigin::ReplicationExport,
                "hot-path",
            )],
            1,
            0,
            SupportPortabilityManifestBudget::new(4, 1024).unwrap(),
            SubscriptionSupportPortabilityDecision::full_scope_replication(
                "support-identity:hot-path",
                "support-identity:hot-path",
            )
            .unwrap(),
            SupportPathClass::ForegroundRead,
            SupportProgramDensityClass::PortabilityScopeBatch,
            SupportAllocationScope::PortabilityManifest,
            SupportActionBreadthBudget::new(4, 1024).unwrap(),
            128,
        )
        .expect_err("foreground reads cannot plan support portability");
    assert_eq!(
        hot_path.kind(),
        &StoreErrorKind::SubscriptionSupportClassificationViolation
    );
    assert_eq!(pipeline.counters().support_hot_path_rejections(), 1);

    let import_decision_on_export_origin = pipeline
        .admit_support_portability_batch(
            SupportActionId::new("support-portability:import-on-export").unwrap(),
            vec![portability_basis(
                SubscriptionSupportActionOrigin::ReplicationExport,
                "import-on-export",
            )],
            1,
            0,
            SupportPortabilityManifestBudget::new(4, 1024).unwrap(),
            SubscriptionSupportPortabilityDecision::target_import_admitted(
                "target-import-admission",
                "source-identity-preservation:import-on-export",
                "imported-semantic",
            )
            .unwrap(),
            SupportPathClass::ImportAdmission,
            SupportProgramDensityClass::PortabilityScopeBatch,
            SupportAllocationScope::PortabilityManifest,
            SupportActionBreadthBudget::new(4, 1024).unwrap(),
            128,
        )
        .expect_err("import decisions cannot ride export-origin affected sets");
    assert_eq!(
        import_decision_on_export_origin.kind(),
        &StoreErrorKind::SubscriptionSupportClassificationViolation
    );

    let export_decision_on_import_origin = pipeline
        .admit_support_portability_batch(
            SupportActionId::new("support-portability:export-on-import").unwrap(),
            vec![portability_basis(
                SubscriptionSupportActionOrigin::ReplicationImport,
                "export-on-import",
            )],
            1,
            0,
            SupportPortabilityManifestBudget::new(4, 1024).unwrap(),
            SubscriptionSupportPortabilityDecision::full_scope_replication(
                "support-identity:export-on-import",
                "support-identity:export-on-import",
            )
            .unwrap(),
            SupportPathClass::ReplicationExport,
            SupportProgramDensityClass::PortabilityScopeBatch,
            SupportAllocationScope::PortabilityManifest,
            SupportActionBreadthBudget::new(4, 1024).unwrap(),
            128,
        )
        .expect_err("export decisions cannot ride import-origin affected sets");
    assert_eq!(
        export_decision_on_import_origin.kind(),
        &StoreErrorKind::SubscriptionSupportClassificationViolation
    );

    let wrong_density = pipeline
        .admit_support_portability_batch(
            SupportActionId::new("support-portability:wrong-density").unwrap(),
            vec![portability_basis(
                SubscriptionSupportActionOrigin::ReplicationExport,
                "wrong-density",
            )],
            1,
            0,
            SupportPortabilityManifestBudget::new(4, 1024).unwrap(),
            SubscriptionSupportPortabilityDecision::full_scope_replication(
                "support-identity:wrong-density",
                "support-identity:wrong-density",
            )
            .unwrap(),
            SupportPathClass::ReplicationExport,
            SupportProgramDensityClass::FamilyLocalBatch,
            SupportAllocationScope::PortabilityManifest,
            SupportActionBreadthBudget::new(4, 1024).unwrap(),
            128,
        )
        .expect_err("portability plans require portability-scope density");
    assert_eq!(
        wrong_density.kind(),
        &StoreErrorKind::SubscriptionSupportClassificationViolation
    );
}
