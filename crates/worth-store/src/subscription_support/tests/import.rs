use super::super::{
    new, pipeline, SubscriptionSupportActionOrigin, SubscriptionSupportArtifactId,
    SubscriptionSupportClassificationViolation, SubscriptionSupportOperationalVerdict,
    SubscriptionSupportPortabilityDecision, SubscriptionSupportPortabilityDecisionKind,
    SubscriptionSupportPortabilityOutcome, SubscriptionSupportPublicationPipeline,
    SupportActionBreadthBudget, SupportActionId, SupportAllocationScope, SupportPathClass,
    SupportPortabilityManifestBudget, SupportProgramDensityClass,
};
use super::portability_basis;
use super::StoreErrorKind;

#[test]
fn phase_4_target_import_requires_admission_before_semantic_access() {
    let mut pipeline = SubscriptionSupportPublicationPipeline::first_ship();
    let plan = pipeline
        .admit_support_portability_batch(
            SupportActionId::new("support-portability:import").unwrap(),
            vec![portability_basis(
                SubscriptionSupportActionOrigin::ReplicationImport,
                "import-a",
            )],
            1,
            0,
            SupportPortabilityManifestBudget::new(4, 1024).unwrap(),
            SubscriptionSupportPortabilityDecision::target_import_admitted(
                "target-import-admission",
                "source-identity-preservation:import-a",
                "imported-support-semantic",
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
    let SubscriptionSupportPortabilityOutcome::Imported(access) = report.outcome() else {
        panic!("target import admission must produce semantic access");
    };

    assert_eq!(
        access.import_admission().manifest_digest(),
        report.manifest().manifest_digest()
    );
    assert_eq!(
        access
            .import_admission()
            .source_identity_preservation_digest(),
        Some("source-identity-preservation:import-a")
    );
    assert_eq!(
        access.imported_semantic_digest(),
        "imported-support-semantic"
    );
    assert_eq!(
        report.completed_action().envelope().action_origin(),
        SubscriptionSupportActionOrigin::ReplicationImport
    );
    assert_eq!(pipeline.counters().support_import_admission_count(), 1);
}

#[test]
fn phase_4_capsule_import_missing_basis_reports_not_resumable_without_semantic_access() {
    let mut pipeline = SubscriptionSupportPublicationPipeline::first_ship();
    let present_basis =
        SubscriptionSupportArtifactId("artifact:portability:missing-basis-a".into());
    let plan = pipeline
        .admit_support_portability_batch(
            SupportActionId::new("support-portability:missing-basis-import").unwrap(),
            vec![
                portability_basis(
                    SubscriptionSupportActionOrigin::ReplicationImport,
                    "missing-basis-a",
                ),
                portability_basis(
                    SubscriptionSupportActionOrigin::ReplicationImport,
                    "missing-basis-b",
                ),
            ],
            2,
            0,
            SupportPortabilityManifestBudget::new(4, 1024).unwrap(),
            SubscriptionSupportPortabilityDecision::target_import_missing_basis_not_resumable(
                "target-import-admission:missing-basis",
                vec![present_basis],
                "capsule omitted required basis evidence for one imported support artifact",
            )
            .unwrap(),
            SupportPathClass::ImportAdmission,
            SupportProgramDensityClass::PortabilityScopeBatch,
            SupportAllocationScope::PortabilityManifest,
            SupportActionBreadthBudget::new(4, 1024).unwrap(),
            128,
        )
        .unwrap();

    assert_eq!(plan.manifest().manifest_entry_count(), 2);
    assert_eq!(plan.manifest().required_basis_count(), 1);

    let report = pipeline
        .publish_support_portability_consequence(plan)
        .unwrap();
    let SubscriptionSupportPortabilityOutcome::ImportedNotResumable(denial) = report.outcome()
    else {
        panic!("missing import basis must publish a typed not-resumable import report");
    };

    assert_eq!(denial.missing_basis_count(), 1);
    assert_eq!(
        denial.import_admission().manifest_digest(),
        report.manifest().manifest_digest()
    );
    assert_eq!(
        denial
            .import_admission()
            .source_identity_preservation_digest(),
        None
    );
    assert_eq!(
        report.completed_action().envelope().verdict(),
        SubscriptionSupportOperationalVerdict::NotResumable
    );
    assert_eq!(
        report.participation_record().decision_kind(),
        SubscriptionSupportPortabilityDecisionKind::TargetImportMissingBasisNotResumable
    );
    assert_eq!(
        pipeline
            .counters()
            .support_portability_required_basis_count(),
        1
    );
    assert_eq!(pipeline.counters().support_import_admission_count(), 1);
}

#[test]
fn phase_4_capsule_import_missing_basis_rejects_WORTHd_basis_membership() {
    let mut pipeline = SubscriptionSupportPublicationPipeline::first_ship();
    let foreign_basis = pipeline
        .admit_support_portability_batch(
            SupportActionId::new("support-portability:missing-basis-foreign").unwrap(),
            vec![
                portability_basis(
                    SubscriptionSupportActionOrigin::ReplicationImport,
                    "basis-foreign-a",
                ),
                portability_basis(
                    SubscriptionSupportActionOrigin::ReplicationImport,
                    "basis-foreign-b",
                ),
            ],
            2,
            0,
            SupportPortabilityManifestBudget::new(4, 1024).unwrap(),
            SubscriptionSupportPortabilityDecision::target_import_missing_basis_not_resumable(
                "target-import-admission:foreign-basis",
                vec![SubscriptionSupportArtifactId(
                    "artifact:portability:not-in-import-scope".into(),
                )],
                "WORTHd basis evidence",
            )
            .unwrap(),
            SupportPathClass::ImportAdmission,
            SupportProgramDensityClass::PortabilityScopeBatch,
            SupportAllocationScope::PortabilityManifest,
            SupportActionBreadthBudget::new(4, 1024).unwrap(),
            128,
        )
        .expect_err("import basis evidence must be scope-local");
    assert_eq!(
        foreign_basis.kind(),
        &StoreErrorKind::SubscriptionSupportClassificationViolation
    );

    let duplicate_basis_id =
        SubscriptionSupportArtifactId("artifact:portability:basis-dup-a".into());
    let duplicate_basis = pipeline
        .admit_support_portability_batch(
            SupportActionId::new("support-portability:missing-basis-duplicate").unwrap(),
            vec![
                portability_basis(
                    SubscriptionSupportActionOrigin::ReplicationImport,
                    "basis-dup-a",
                ),
                portability_basis(
                    SubscriptionSupportActionOrigin::ReplicationImport,
                    "basis-dup-b",
                ),
            ],
            2,
            0,
            SupportPortabilityManifestBudget::new(4, 1024).unwrap(),
            SubscriptionSupportPortabilityDecision::target_import_missing_basis_not_resumable(
                "target-import-admission:duplicate-basis",
                vec![duplicate_basis_id.clone(), duplicate_basis_id],
                "duplicate basis evidence",
            )
            .unwrap(),
            SupportPathClass::ImportAdmission,
            SupportProgramDensityClass::PortabilityScopeBatch,
            SupportAllocationScope::PortabilityManifest,
            SupportActionBreadthBudget::new(4, 1024).unwrap(),
            128,
        )
        .expect_err("import basis evidence must not duplicate artifact ids");
    assert_eq!(
        duplicate_basis.kind(),
        &StoreErrorKind::SubscriptionSupportClassificationViolation
    );
}
