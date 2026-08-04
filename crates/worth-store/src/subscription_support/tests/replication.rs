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
fn phase_4_full_scope_replication_preserves_support_identity() {
    let mut pipeline = SubscriptionSupportPublicationPipeline::first_ship();
    let plan = pipeline
        .admit_support_portability_batch(
            SupportActionId::new("support-portability:full-replication").unwrap(),
            vec![
                portability_basis(SubscriptionSupportActionOrigin::ReplicationExport, "full-a"),
                portability_basis(SubscriptionSupportActionOrigin::ReplicationExport, "full-b"),
            ],
            2,
            0,
            SupportPortabilityManifestBudget::new(4, 1024).unwrap(),
            SubscriptionSupportPortabilityDecision::full_scope_replication(
                "support-identity:full",
                "support-identity:full",
            )
            .unwrap(),
            SupportPathClass::ReplicationExport,
            SupportProgramDensityClass::PortabilityScopeBatch,
            SupportAllocationScope::PortabilityManifest,
            SupportActionBreadthBudget::new(4, 1024).unwrap(),
            256,
        )
        .unwrap();

    assert_eq!(plan.footprint().included_support_count(), 2);
    assert_eq!(plan.footprint().omitted_support_count(), 0);
    assert_eq!(plan.manifest().manifest_entry_count(), 2);

    let report = pipeline
        .publish_support_portability_consequence(plan)
        .unwrap();
    let SubscriptionSupportPortabilityOutcome::FullScopeReplicated(bundle) = report.outcome()
    else {
        panic!("full-scope portability must materialize a replicated support bundle");
    };

    assert_eq!(bundle.preserved_count(), 2);
    assert_eq!(
        report.completed_action().envelope().verdict(),
        SubscriptionSupportOperationalVerdict::ExactResumePreserved
    );
    assert_eq!(
        report.participation_record().decision_kind(),
        SubscriptionSupportPortabilityDecisionKind::FullScopeReplication
    );
    assert_eq!(pipeline.counters().support_portability_plan_count(), 1);
    assert_eq!(
        pipeline.counters().support_portability_manifest_entries(),
        2
    );
    assert_eq!(pipeline.counters().support_replication_inclusion_count(), 2);
}

#[test]
fn phase_4_partial_replication_omission_cannot_report_exact_support() {
    let mut pipeline = SubscriptionSupportPublicationPipeline::first_ship();
    let omitted = SubscriptionSupportArtifactId("artifact:portability:partial-b".into());
    let plan = pipeline
        .admit_support_portability_batch(
            SupportActionId::new("support-portability:partial").unwrap(),
            vec![
                portability_basis(
                    SubscriptionSupportActionOrigin::ReplicationExport,
                    "partial-a",
                ),
                portability_basis(
                    SubscriptionSupportActionOrigin::ReplicationExport,
                    "partial-b",
                ),
            ],
            1,
            1,
            SupportPortabilityManifestBudget::new(4, 1024).unwrap(),
            SubscriptionSupportPortabilityDecision::partial_scope_omission(
                vec![omitted.clone()],
                "target capsule omits cold support artifact",
            )
            .unwrap(),
            SupportPathClass::ReplicationExport,
            SupportProgramDensityClass::PortabilityScopeBatch,
            SupportAllocationScope::PortabilityManifest,
            SupportActionBreadthBudget::new(4, 1024).unwrap(),
            256,
        )
        .unwrap();

    let report = pipeline
        .publish_support_portability_consequence(plan)
        .unwrap();
    let SubscriptionSupportPortabilityOutcome::PartialScopeOmitted(omission) = report.outcome()
    else {
        panic!("partial portability must publish an omission report");
    };

    assert_eq!(omission.omitted_count(), 1);
    assert_eq!(
        report.completed_action().envelope().verdict(),
        SubscriptionSupportOperationalVerdict::DegradedResumePreserved
    );
    assert_ne!(
        report.participation_record().verdict(),
        SubscriptionSupportOperationalVerdict::ExactResumePreserved
    );
    assert_eq!(
        pipeline
            .counters()
            .support_portability_omitted_support_count(),
        1
    );
    assert_eq!(pipeline.counters().support_replication_omission_count(), 1);
}

#[test]
fn phase_4_portability_rejects_identity_drift_and_invalid_omission_ids() {
    let identity_drift = SubscriptionSupportPortabilityDecision::full_scope_replication(
        "source-support-identity",
        "target-support-identity",
    )
    .expect_err(
        "full-scope replication must prove identity preservation, not just name identities",
    );
    assert_eq!(
        identity_drift.kind(),
        &StoreErrorKind::SubscriptionSupportClassificationViolation
    );

    let mut pipeline = SubscriptionSupportPublicationPipeline::first_ship();
    let foreign_omission = pipeline
        .admit_support_portability_batch(
            SupportActionId::new("support-portability:foreign-omission").unwrap(),
            vec![
                portability_basis(
                    SubscriptionSupportActionOrigin::ReplicationExport,
                    "foreign-a",
                ),
                portability_basis(
                    SubscriptionSupportActionOrigin::ReplicationExport,
                    "foreign-b",
                ),
            ],
            1,
            1,
            SupportPortabilityManifestBudget::new(4, 1024).unwrap(),
            SubscriptionSupportPortabilityDecision::partial_scope_omission(
                vec![SubscriptionSupportArtifactId(
                    "artifact:portability:not-in-scope".into(),
                )],
                "invalid omission report",
            )
            .unwrap(),
            SupportPathClass::ReplicationExport,
            SupportProgramDensityClass::PortabilityScopeBatch,
            SupportAllocationScope::PortabilityManifest,
            SupportActionBreadthBudget::new(4, 1024).unwrap(),
            128,
        )
        .expect_err("partial omission reports must name artifacts from the admitted scope");
    assert_eq!(
        foreign_omission.kind(),
        &StoreErrorKind::SubscriptionSupportClassificationViolation
    );

    let duplicate_id = SubscriptionSupportArtifactId("artifact:portability:duplicate-a".into());
    let duplicate_omission = pipeline
        .admit_support_portability_batch(
            SupportActionId::new("support-portability:duplicate-omission").unwrap(),
            vec![
                portability_basis(
                    SubscriptionSupportActionOrigin::ReplicationExport,
                    "duplicate-a",
                ),
                portability_basis(
                    SubscriptionSupportActionOrigin::ReplicationExport,
                    "duplicate-b",
                ),
            ],
            0,
            2,
            SupportPortabilityManifestBudget::new(4, 1024).unwrap(),
            SubscriptionSupportPortabilityDecision::partial_scope_omission(
                vec![duplicate_id.clone(), duplicate_id],
                "duplicate omission report",
            )
            .unwrap(),
            SupportPathClass::ReplicationExport,
            SupportProgramDensityClass::PortabilityScopeBatch,
            SupportAllocationScope::PortabilityManifest,
            SupportActionBreadthBudget::new(4, 1024).unwrap(),
            128,
        )
        .expect_err("partial omission reports must not duplicate omitted artifacts");
    assert_eq!(
        duplicate_omission.kind(),
        &StoreErrorKind::SubscriptionSupportClassificationViolation
    );
}
