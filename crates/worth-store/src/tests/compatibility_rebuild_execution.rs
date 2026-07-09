use crate::{
    CompatibilityDerivedRebuildRequest, CompatibilityFamilyKind, WORTHStoreBuilder,
    MaintenanceDeclarationId, MaintenanceExecutionStatus, Milestone12CertificationLaneKind,
    Milestone12CertificationLaneStatus, Milestone12CertificationRunner, StoreErrorKind,
};

#[test]
fn compatibility_triggered_rebuild_executes_through_milestone_11_maintenance() {
    let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();

    let outcome = store
        .execute_compatibility_derived_rebuild(CompatibilityDerivedRebuildRequest::new(
            CompatibilityFamilyKind::Milestone11MaintenanceRecord,
        ))
        .unwrap();

    assert!(outcome
        .maintenance_declaration_id()
        .starts_with("compatibility-derived-rebuild:"));
    assert!(outcome
        .maintenance_lane_id()
        .contains("DerivedFamilyRebuild"));
    assert_eq!(
        outcome.completed_phase(),
        "derived_family_rebuild_container:compatibility-retained-authority:milestone_11_maintenance_record:milestone_11_maintenance_record:1->2"
    );

    let status = store
        .maintenance_status(&MaintenanceDeclarationId::new(
            outcome.maintenance_declaration_id().to_string(),
        ))
        .unwrap();
    assert_eq!(
        status.execution_status(),
        MaintenanceExecutionStatus::Completed
    );
    assert_eq!(
        status.last_completed_phase(),
        Some(outcome.completed_phase())
    );
    assert_eq!(outcome.admission_report().accepted_count, 1);
    assert_eq!(
        outcome
            .admission_report()
            .maintenance_compatibility_rebuild_admission_count,
        1
    );
}

#[test]
fn compatibility_triggered_rebuild_rejects_family_without_rebuild_requirement() {
    let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();

    let error = store
        .execute_compatibility_derived_rebuild(CompatibilityDerivedRebuildRequest::new(
            CompatibilityFamilyKind::CommitEnvelope,
        ))
        .unwrap_err();

    assert_eq!(
        error.kind(),
        &StoreErrorKind::CompatibilityDerivedRebuildIncompatible
    );
}

#[test]
fn compatibility_triggered_rebuild_matches_certification_lane_evidence() {
    let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();

    let outcome = store
        .execute_compatibility_derived_rebuild(CompatibilityDerivedRebuildRequest::new(
            CompatibilityFamilyKind::Milestone11MaintenanceRecord,
        ))
        .unwrap();
    let certification = Milestone12CertificationRunner::first_ship().run().unwrap();
    let admitted_lane = certification
        .evidence_bundle()
        .lane_outcomes()
        .iter()
        .find(|lane| {
            lane.lane_kind() == Milestone12CertificationLaneKind::MaintenanceSummaryRebuildAdmitted
        })
        .unwrap();

    assert_eq!(
        admitted_lane.status(),
        Milestone12CertificationLaneStatus::Accepted
    );
    assert_eq!(
        admitted_lane.relation(),
        Some(crate::CompatibilityRelation::Native)
    );
    assert_eq!(
        outcome
            .admission_report()
            .maintenance_compatibility_rebuild_admission_count,
        admitted_lane
            .counters()
            .maintenance_compatibility_rebuild_admission_count
    );
    assert_eq!(
        outcome
            .admission_report()
            .derived_maintenance_summary_rebuild_count,
        admitted_lane
            .counters()
            .derived_maintenance_summary_rebuild_count
    );
}
