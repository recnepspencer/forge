use super::super::{
    new, pipeline, SubscriptionSupportAllocationScope, SubscriptionSupportClassificationViolation,
    SubscriptionSupportMaintenanceDecision, SubscriptionSupportMaintenanceDecisionKind,
    SubscriptionSupportOperationalVerdict, SubscriptionSupportPublicationPipeline,
    SupportActionBreadthBudget, SupportActionId, SupportAllocationScope,
    SupportMaintenanceWorkKind, SupportPathClass, SupportProgramDensityClass,
};
use super::maintenance_basis;
use super::StoreErrorKind;

#[test]
fn phase_5_maintenance_rebuild_descriptor_is_admitted_and_coalesced_by_key() {
    let mut pipeline = SubscriptionSupportPublicationPipeline::first_ship();
    let basis = maintenance_basis("rebuild");
    let retained_basis_digest = basis.basis_digest().to_string();
    let plan = pipeline
        .admit_support_maintenance_batch(
            SupportActionId::new("support-maintenance:rebuild").unwrap(),
            vec![basis.clone(), basis],
            SubscriptionSupportMaintenanceDecision::rebuild_descriptor_admitted(
                retained_basis_digest,
            )
            .unwrap(),
            SupportPathClass::MaintenanceExecution,
            SupportProgramDensityClass::MaintenanceKeyBatch,
            SupportAllocationScope::FamilyLocalBatch,
            SupportActionBreadthBudget::new(4, 1024).unwrap(),
            128,
        )
        .unwrap();

    assert_eq!(plan.affected_set().affected_count(), 2);
    assert_eq!(plan.descriptors().len(), 1);
    assert_eq!(plan.coalesced_duplicate_count(), 1);
    assert_eq!(
        plan.maintenance_receipt().batch_summary().batch_class(),
        crate::MaintenanceBatchClass::SubscriptionSupport
    );
    assert_eq!(
        plan.descriptors()[0].work_kind(),
        SupportMaintenanceWorkKind::Rebuild
    );

    let report = pipeline
        .publish_support_maintenance_consequence(plan)
        .unwrap();

    assert_eq!(report.admissions().len(), 1);
    assert_eq!(report.descriptor_records().len(), 1);
    assert_eq!(
        report.completed_action().envelope().verdict(),
        SubscriptionSupportOperationalVerdict::RebuildRequired
    );
    assert_eq!(
        report.descriptor_records()[0].declaration_id(),
        report.admissions()[0].declaration_id()
    );
    assert_eq!(
        report.participation_record().decision_kind(),
        SubscriptionSupportMaintenanceDecisionKind::RebuildDescriptorAdmitted
    );
    assert_eq!(report.participation_record().coalesced_duplicate_count(), 1);
    assert_eq!(
        pipeline.counters().support_maintenance_descriptor_count(),
        1
    );
    assert_eq!(
        pipeline
            .counters()
            .support_maintenance_coalesced_duplicate_count(),
        1
    );
    assert_eq!(
        pipeline.counters().support_maintenance_rebuild_debt_count(),
        1
    );
}

#[test]
fn phase_5_maintenance_rejects_missing_basis_wrong_path_and_wrong_density() {
    let mut pipeline = SubscriptionSupportPublicationPipeline::first_ship();
    let missing_basis = pipeline
        .admit_support_maintenance_batch(
            SupportActionId::new("support-maintenance:missing-basis").unwrap(),
            vec![maintenance_basis("missing-basis")],
            SubscriptionSupportMaintenanceDecision::rebuild_descriptor_admitted(
                "basis:maintenance:other",
            )
            .unwrap(),
            SupportPathClass::MaintenanceExecution,
            SupportProgramDensityClass::MaintenanceKeyBatch,
            SupportAllocationScope::FamilyLocalBatch,
            SupportActionBreadthBudget::new(4, 1024).unwrap(),
            128,
        )
        .expect_err("rebuild descriptors must consume matching retained basis evidence");
    assert_eq!(
        missing_basis.kind(),
        &StoreErrorKind::SubscriptionSupportClassificationViolation
    );

    let hot_path = pipeline
        .admit_support_maintenance_batch(
            SupportActionId::new("support-maintenance:hot-path").unwrap(),
            vec![maintenance_basis("hot-path")],
            SubscriptionSupportMaintenanceDecision::refresh_descriptor_admitted(
                "stale support refresh",
            )
            .unwrap(),
            SupportPathClass::ForegroundRead,
            SupportProgramDensityClass::MaintenanceKeyBatch,
            SupportAllocationScope::FamilyLocalBatch,
            SupportActionBreadthBudget::new(4, 1024).unwrap(),
            128,
        )
        .expect_err("foreground reads cannot admit support maintenance work");
    assert_eq!(
        hot_path.kind(),
        &StoreErrorKind::SubscriptionSupportClassificationViolation
    );
    assert_eq!(pipeline.counters().support_hot_path_rejections(), 1);

    let wrong_density = pipeline
        .admit_support_maintenance_batch(
            SupportActionId::new("support-maintenance:wrong-density").unwrap(),
            vec![maintenance_basis("wrong-density")],
            SubscriptionSupportMaintenanceDecision::refresh_descriptor_admitted(
                "stale support refresh",
            )
            .unwrap(),
            SupportPathClass::MaintenanceExecution,
            SupportProgramDensityClass::FamilyLocalBatch,
            SupportAllocationScope::FamilyLocalBatch,
            SupportActionBreadthBudget::new(4, 1024).unwrap(),
            128,
        )
        .expect_err("support maintenance must use maintenance-key density");
    assert_eq!(
        wrong_density.kind(),
        &StoreErrorKind::SubscriptionSupportClassificationViolation
    );
}

#[test]
fn phase_5_maintenance_refresh_migration_degradation_and_restart_publish_typed_posture() {
    let mut pipeline = SubscriptionSupportPublicationPipeline::first_ship();

    let refresh = pipeline
        .admit_support_maintenance_batch(
            SupportActionId::new("support-maintenance:refresh").unwrap(),
            vec![maintenance_basis("refresh")],
            SubscriptionSupportMaintenanceDecision::refresh_descriptor_admitted(
                "support refresh keeps exact posture",
            )
            .unwrap(),
            SupportPathClass::MaintenanceExecution,
            SupportProgramDensityClass::MaintenanceKeyBatch,
            SupportAllocationScope::FamilyLocalBatch,
            SupportActionBreadthBudget::new(4, 1024).unwrap(),
            128,
        )
        .unwrap();
    let refresh = pipeline
        .publish_support_maintenance_consequence(refresh)
        .unwrap();
    assert_eq!(
        refresh.completed_action().envelope().verdict(),
        SubscriptionSupportOperationalVerdict::ExactResumePreserved
    );
    assert_eq!(pipeline.counters().support_maintenance_refresh_count(), 1);

    let migration = pipeline
        .admit_support_maintenance_batch(
            SupportActionId::new("support-maintenance:migration").unwrap(),
            vec![maintenance_basis("migration")],
            SubscriptionSupportMaintenanceDecision::compatibility_migration_descriptor_admitted(
                "compatibility-migration:exact",
            )
            .unwrap(),
            SupportPathClass::MaintenanceExecution,
            SupportProgramDensityClass::MaintenanceKeyBatch,
            SupportAllocationScope::FamilyLocalBatch,
            SupportActionBreadthBudget::new(4, 1024).unwrap(),
            128,
        )
        .unwrap();
    let migration = pipeline
        .publish_support_maintenance_consequence(migration)
        .unwrap();
    assert_eq!(
        migration.participation_record().decision_kind(),
        SubscriptionSupportMaintenanceDecisionKind::CompatibilityMigrationDescriptorAdmitted
    );
    assert_eq!(
        pipeline
            .counters()
            .support_maintenance_compatibility_migration_count(),
        1
    );

    let degradation = pipeline
        .admit_support_maintenance_batch(
            SupportActionId::new("support-maintenance:degradation").unwrap(),
            vec![maintenance_basis("degradation")],
            SubscriptionSupportMaintenanceDecision::degradation_recovery_descriptor_admitted(
                "degraded support recovery remains degraded",
            )
            .unwrap(),
            SupportPathClass::MaintenanceExecution,
            SupportProgramDensityClass::MaintenanceKeyBatch,
            SupportAllocationScope::FamilyLocalBatch,
            SupportActionBreadthBudget::new(4, 1024).unwrap(),
            128,
        )
        .unwrap();
    let degradation = pipeline
        .publish_support_maintenance_consequence(degradation)
        .unwrap();
    assert_eq!(
        degradation.completed_action().envelope().verdict(),
        SubscriptionSupportOperationalVerdict::DegradedResumePreserved
    );
    assert_eq!(
        pipeline
            .counters()
            .support_maintenance_degradation_recovery_count(),
        1
    );

    let recovered = pipeline
        .admit_support_maintenance_batch(
            SupportActionId::new("support-maintenance:restart").unwrap(),
            vec![maintenance_basis("restart")],
            SubscriptionSupportMaintenanceDecision::interrupted_restart_recovered(
                SupportMaintenanceWorkKind::Rebuild,
                "maintenance-restart:descriptor-recovered",
            )
            .unwrap(),
            SupportPathClass::MaintenanceExecution,
            SupportProgramDensityClass::MaintenanceKeyBatch,
            SupportAllocationScope::FamilyLocalBatch,
            SupportActionBreadthBudget::new(4, 1024).unwrap(),
            128,
        )
        .unwrap();
    assert!(recovered.descriptors()[0]
        .descriptor()
        .recovered_from_restart());
    let recovered = pipeline
        .publish_support_maintenance_consequence(recovered)
        .unwrap();
    assert_eq!(
        recovered.participation_record().decision_kind(),
        SubscriptionSupportMaintenanceDecisionKind::InterruptedRestartRecovered
    );
    assert_eq!(
        pipeline
            .counters()
            .support_maintenance_interrupted_restart_recovery_count(),
        1
    );
}

#[test]
fn phase_5_maintenance_delay_reports_debt_without_mutating_truth() {
    let mut pipeline = SubscriptionSupportPublicationPipeline::first_ship();
    let basis = maintenance_basis("delayed");
    let retained_basis_digest = basis.basis_digest().to_string();
    let plan = pipeline
        .admit_support_maintenance_batch(
            SupportActionId::new("support-maintenance:delayed").unwrap(),
            vec![basis],
            SubscriptionSupportMaintenanceDecision::rebuild_descriptor_admitted(
                retained_basis_digest,
            )
            .unwrap(),
            SupportPathClass::MaintenanceExecution,
            SupportProgramDensityClass::MaintenanceKeyBatch,
            SupportAllocationScope::FamilyLocalBatch,
            SupportActionBreadthBudget::new(4, 1024).unwrap(),
            128,
        )
        .unwrap();

    let report = pipeline
        .report_delayed_support_maintenance(
            &plan,
            "maintenance lane deferred by batch pacing",
            SupportActionBreadthBudget::new(4, 1024).unwrap(),
            128,
        )
        .unwrap();

    assert_eq!(
        report.debt_summary().verdict(),
        SubscriptionSupportOperationalVerdict::RebuildRequired
    );
    assert_eq!(
        report.debt_summary().work_kind(),
        SupportMaintenanceWorkKind::Rebuild
    );
    assert_eq!(
        report.debt_summary().delay_reason(),
        "maintenance lane deferred by batch pacing"
    );
    assert_eq!(report.admissions().len(), 1);
    assert_eq!(
        report.cost_surface().allocation_scope(),
        crate::SubscriptionSupportAllocationScope::OperatorReport
    );
    assert_eq!(pipeline.counters().support_maintenance_delay_count(), 1);
    assert_eq!(
        pipeline.counters().support_action_envelope_publications(),
        0
    );
}
