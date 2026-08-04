use super::super::{
    digest, new, pipeline, SubscriptionSupportActionOrigin,
    SubscriptionSupportClassificationViolation, SubscriptionSupportFamilyKind,
    SubscriptionSupportOperationalVerdict, SubscriptionSupportPublicationPipeline,
    SubscriptionSupportRetentionDecision, SubscriptionSupportRetentionDecisionKind,
    SubscriptionSupportRetentionMaterialization, SubscriptionSupportRole,
    SupportActionBreadthBudget, SupportActionId, SupportAllocationScope, SupportPathClass,
    SupportProgramDensityClass,
};
use super::StoreErrorKind;
use super::{retention_basis, retention_basis_for_family};

#[test]
fn phase_2_retention_batch_publishes_exact_survival_before_completion() {
    let mut pipeline = SubscriptionSupportPublicationPipeline::first_ship();
    let plan = pipeline
        .admit_support_retention_batch(
            SupportActionId::new("support-retention:exact").unwrap(),
            vec![retention_basis("exact-1"), retention_basis("exact-2")],
            SubscriptionSupportRetentionDecision::retain_exact(),
            SupportPathClass::OperationalPlanning,
            SupportProgramDensityClass::FamilyLocalBatch,
            SupportAllocationScope::FamilyLocalBatch,
            SupportActionBreadthBudget::new(4, 1024).unwrap(),
            256,
        )
        .unwrap();

    assert_eq!(plan.affected_set().affected_count(), 2);
    assert_eq!(pipeline.counters().support_retention_plan_count(), 1);
    assert_eq!(pipeline.counters().support_retention_affected_entries(), 2);

    let report = pipeline
        .publish_support_retention_consequence(plan)
        .expect("retention completion must publish a support consequence");

    assert_eq!(
        report.survival_witness().verdict(),
        SubscriptionSupportOperationalVerdict::ExactResumePreserved
    );
    assert_eq!(report.survival_witness().affected_count(), 2);
    assert_eq!(report.retention_record().affected_count(), 2);
    assert_eq!(
        report.retention_record().decision_kind(),
        SubscriptionSupportRetentionDecisionKind::RetainExact
    );
    assert_eq!(
        report.retention_record().affected_set_digest(),
        report.survival_witness().affected_set_digest()
    );
    assert!(matches!(
        report.materialization(),
        SubscriptionSupportRetentionMaterialization::Retained(_)
    ));
    assert_eq!(
        report.completed_action().envelope().action_origin(),
        SubscriptionSupportActionOrigin::Retention
    );
    assert_eq!(
        pipeline.counters().support_action_envelope_publications(),
        1
    );
}

#[test]
fn phase_2_retention_materializes_degraded_compacted_and_expired_lanes() {
    let mut pipeline = SubscriptionSupportPublicationPipeline::first_ship();
    let degraded_plan = pipeline
        .admit_support_retention_batch(
            SupportActionId::new("support-retention:degraded").unwrap(),
            vec![retention_basis("degraded")],
            SubscriptionSupportRetentionDecision::retain_degraded("cursor lineage was weakened")
                .unwrap(),
            SupportPathClass::OperationalPlanning,
            SupportProgramDensityClass::FamilyLocalBatch,
            SupportAllocationScope::FamilyLocalBatch,
            SupportActionBreadthBudget::new(4, 1024).unwrap(),
            128,
        )
        .unwrap();
    assert_eq!(pipeline.counters().support_retained_family_count(), 0);
    assert_eq!(pipeline.counters().support_compacted_basis_count(), 0);
    assert_eq!(pipeline.counters().support_expired_family_count(), 0);
    let degraded_report = pipeline
        .publish_support_retention_consequence(degraded_plan)
        .unwrap();
    assert_eq!(
        degraded_report.survival_witness().verdict(),
        SubscriptionSupportOperationalVerdict::DegradedResumePreserved
    );
    let SubscriptionSupportRetentionMaterialization::Retained(retained) =
        degraded_report.materialization()
    else {
        panic!("degraded retention must materialize retained support");
    };
    assert_eq!(
        retained.decision_kind(),
        SubscriptionSupportRetentionDecisionKind::RetainDegraded
    );
    assert_eq!(
        retained.weakened_condition(),
        Some("cursor lineage was weakened")
    );

    let compacted_plan = pipeline
        .admit_support_retention_batch(
            SupportActionId::new("support-retention:compacted").unwrap(),
            vec![
                retention_basis("compacted-1"),
                retention_basis("compacted-2"),
            ],
            SubscriptionSupportRetentionDecision::compact_exact("compacted-basis:digest").unwrap(),
            SupportPathClass::OperationalPlanning,
            SupportProgramDensityClass::FamilyLocalBatch,
            SupportAllocationScope::FamilyLocalBatch,
            SupportActionBreadthBudget::new(4, 1024).unwrap(),
            256,
        )
        .unwrap();
    let compacted_report = pipeline
        .publish_support_retention_consequence(compacted_plan)
        .unwrap();
    let SubscriptionSupportRetentionMaterialization::Compacted(compacted) =
        compacted_report.materialization()
    else {
        panic!("compacted decision must materialize compacted support basis");
    };
    assert_eq!(compacted.compacted_basis_digest(), "compacted-basis:digest");
    assert_eq!(
        compacted_report.retention_record().decision_kind(),
        SubscriptionSupportRetentionDecisionKind::CompactExact
    );

    let expired_plan = pipeline
        .admit_support_retention_batch(
            SupportActionId::new("support-retention:expired").unwrap(),
            vec![retention_basis("expired")],
            SubscriptionSupportRetentionDecision::expire_by_policy("policy window expired")
                .unwrap(),
            SupportPathClass::OperationalPlanning,
            SupportProgramDensityClass::FamilyLocalBatch,
            SupportAllocationScope::FamilyLocalBatch,
            SupportActionBreadthBudget::new(4, 1024).unwrap(),
            128,
        )
        .unwrap();
    let expired_report = pipeline
        .publish_support_retention_consequence(expired_plan)
        .unwrap();
    let SubscriptionSupportRetentionMaterialization::Expired(expired) =
        expired_report.materialization()
    else {
        panic!("policy expiration must materialize expired support set");
    };
    assert_eq!(expired.policy_reason(), "policy window expired");
    assert_eq!(
        expired_report.survival_witness().verdict(),
        SubscriptionSupportOperationalVerdict::RejectedByPolicy
    );

    assert_eq!(pipeline.counters().support_retained_family_count(), 1);
    assert_eq!(pipeline.counters().support_compacted_basis_count(), 1);
    assert_eq!(pipeline.counters().support_expired_family_count(), 1);
    assert_eq!(pipeline.counters().support_policy_expiration_count(), 1);
}

#[test]
fn phase_2_reclaim_distinguishes_rebuildable_and_non_resumable_loss() {
    let mut pipeline = SubscriptionSupportPublicationPipeline::first_ship();
    let rebuild_plan = pipeline
        .admit_support_retention_batch(
            SupportActionId::new("support-reclaim:rebuild").unwrap(),
            vec![retention_basis("rebuild")],
            SubscriptionSupportRetentionDecision::reclaim_with_rebuild(
                "retained-rebuild-basis:1",
                "maintenance-admission:1",
            )
            .unwrap(),
            SupportPathClass::OperationalPlanning,
            SupportProgramDensityClass::FamilyLocalBatch,
            SupportAllocationScope::FamilyLocalBatch,
            SupportActionBreadthBudget::new(4, 1024).unwrap(),
            128,
        )
        .unwrap();
    let rebuild_consequence = pipeline
        .publish_support_reclaim_consequence(rebuild_plan)
        .unwrap();

    assert_eq!(
        rebuild_consequence.survival_witness().verdict(),
        SubscriptionSupportOperationalVerdict::RebuildRequired
    );
    assert_eq!(
        rebuild_consequence.retention_record().decision_kind(),
        SubscriptionSupportRetentionDecisionKind::ReclaimWithRebuild
    );
    assert_eq!(
        rebuild_consequence
            .reclaimed_artifacts()
            .retained_rebuild_basis_digest(),
        Some("retained-rebuild-basis:1")
    );
    assert_eq!(
        rebuild_consequence
            .reclaimed_artifacts()
            .maintenance_admission_key(),
        Some("maintenance-admission:1")
    );
    assert!(matches!(
        rebuild_consequence
            .survival_witness()
            .affected_set_digest()
            .as_str(),
        digest if !digest.is_empty()
    ));

    let denied_plan = pipeline
        .admit_support_retention_batch(
            SupportActionId::new("support-reclaim:not-resumable").unwrap(),
            vec![retention_basis("not-resumable")],
            SubscriptionSupportRetentionDecision::reclaim_without_rebuild(
                "retained rebuild basis was reclaimed",
            )
            .unwrap(),
            SupportPathClass::OperationalPlanning,
            SupportProgramDensityClass::FamilyLocalBatch,
            SupportAllocationScope::FamilyLocalBatch,
            SupportActionBreadthBudget::new(4, 1024).unwrap(),
            128,
        )
        .unwrap();
    let denied_consequence = pipeline
        .publish_support_reclaim_consequence(denied_plan)
        .unwrap();

    assert_eq!(
        denied_consequence.survival_witness().verdict(),
        SubscriptionSupportOperationalVerdict::NotResumable
    );
    assert_eq!(
        denied_consequence.retention_record().decision_kind(),
        SubscriptionSupportRetentionDecisionKind::ReclaimWithoutRebuild
    );
    assert_eq!(
        denied_consequence
            .reclaimed_artifacts()
            .missing_rebuild_basis_reason(),
        Some("retained rebuild basis was reclaimed")
    );
    assert_eq!(pipeline.counters().support_reclaim_consequence_count(), 2);
    assert_eq!(pipeline.counters().support_reclaimed_family_count(), 2);
}

#[test]
fn phase_2_retention_rejects_mixed_family_origin_and_non_reclaim_completion() {
    let mut pipeline = SubscriptionSupportPublicationPipeline::first_ship();
    let mixed_family_error = pipeline
        .admit_support_retention_batch(
            SupportActionId::new("support-retention:mixed-family").unwrap(),
            vec![
                retention_basis("family-a"),
                retention_basis_for_family(
                    "degraded-continuation-support",
                    SubscriptionSupportFamilyKind::DegradedContinuationSupport,
                    SubscriptionSupportRole::DegradedContinuation,
                    "family-b",
                    SubscriptionSupportActionOrigin::Retention,
                ),
            ],
            SubscriptionSupportRetentionDecision::retain_exact(),
            SupportPathClass::OperationalPlanning,
            SupportProgramDensityClass::FamilyLocalBatch,
            SupportAllocationScope::FamilyLocalBatch,
            SupportActionBreadthBudget::new(4, 1024).unwrap(),
            128,
        )
        .expect_err("retention affected sets must be family-local");
    assert_eq!(
        mixed_family_error.kind(),
        &StoreErrorKind::SubscriptionSupportClassificationViolation
    );

    let mixed_origin_error = pipeline
        .admit_support_retention_batch(
            SupportActionId::new("support-retention:mixed-origin").unwrap(),
            vec![
                retention_basis("origin-a"),
                retention_basis_for_family(
                    "basis-bound-continuation-support",
                    SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
                    SubscriptionSupportRole::ExactContinuation,
                    "origin-b",
                    SubscriptionSupportActionOrigin::Compatibility,
                ),
            ],
            SubscriptionSupportRetentionDecision::retain_exact(),
            SupportPathClass::OperationalPlanning,
            SupportProgramDensityClass::FamilyLocalBatch,
            SupportAllocationScope::FamilyLocalBatch,
            SupportActionBreadthBudget::new(4, 1024).unwrap(),
            128,
        )
        .expect_err("retention affected sets must reject non-retention-origin bases");
    assert_eq!(
        mixed_origin_error.kind(),
        &StoreErrorKind::SubscriptionSupportClassificationViolation
    );

    let retain_plan = pipeline
        .admit_support_retention_batch(
            SupportActionId::new("support-retention:not-reclaim").unwrap(),
            vec![retention_basis("not-reclaim")],
            SubscriptionSupportRetentionDecision::retain_exact(),
            SupportPathClass::OperationalPlanning,
            SupportProgramDensityClass::FamilyLocalBatch,
            SupportAllocationScope::FamilyLocalBatch,
            SupportActionBreadthBudget::new(4, 1024).unwrap(),
            128,
        )
        .unwrap();
    let non_reclaim_error = pipeline
        .publish_support_reclaim_consequence(retain_plan)
        .expect_err("retain decisions cannot complete through reclaim consequence API");
    assert_eq!(
        non_reclaim_error.kind(),
        &StoreErrorKind::SubscriptionSupportClassificationViolation
    );
}

#[test]
fn phase_2_retention_rejects_hot_path_and_store_global_sweeps() {
    let mut pipeline = SubscriptionSupportPublicationPipeline::first_ship();
    let budget = SupportActionBreadthBudget::new(4, 1024).unwrap();

    let hot_path_error = pipeline
        .admit_support_retention_batch(
            SupportActionId::new("support-retention:hot-path").unwrap(),
            vec![retention_basis("hot-path")],
            SubscriptionSupportRetentionDecision::retain_exact(),
            SupportPathClass::ForegroundResume,
            SupportProgramDensityClass::FamilyLocalBatch,
            SupportAllocationScope::FamilyLocalBatch,
            budget,
            128,
        )
        .expect_err("foreground resume cannot run retention support planning");

    assert_eq!(
        hot_path_error.kind(),
        &StoreErrorKind::SubscriptionSupportClassificationViolation
    );
    assert_eq!(pipeline.counters().support_hot_path_rejections(), 1);

    let store_global_error = pipeline
        .admit_support_retention_batch(
            SupportActionId::new("support-retention:global").unwrap(),
            vec![retention_basis("global")],
            SubscriptionSupportRetentionDecision::retain_exact(),
            SupportPathClass::OperationalPlanning,
            SupportProgramDensityClass::StoreGlobalDebt,
            SupportAllocationScope::FamilyLocalBatch,
            budget,
            128,
        )
        .expect_err("store-global support retention sweeps are explicit debt");

    assert_eq!(
        store_global_error.kind(),
        &StoreErrorKind::SubscriptionSupportClassificationViolation
    );
    assert_eq!(
        pipeline.counters().support_store_global_debt_rejections(),
        1
    );
}
