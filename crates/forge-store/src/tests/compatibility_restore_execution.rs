use std::collections::BTreeMap;

use crate::{
    CompatibilityFamilyKind, ForgeStore, ForgeStoreBuilder, Milestone12CertificationLaneKind,
    Milestone12CertificationRunner, Milestone12CertificationLaneStatus,
    RestorePublicationConflictKind, RestorePublicationConflictSet, RestorePublicationConflictUnit,
    StoreErrorKind,
};

use super::harness::fixtures::runtime::{create_entity, latest_envelope, runtime_with_demo_schema};

#[test]
fn authoritative_export_restore_executes_receipts_for_visible_families() {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let envelope = latest_envelope(&runtime);

    let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    store.append_canonical_commit(envelope).unwrap();
    let export = store.export_authoritative_records();

    let (_restored, execution) =
        ForgeStore::restore_from_authoritative_export_with_compatibility(export.admit_restore())
            .expect("restore execution should admit visible authoritative families");

    assert_eq!(execution.receipt_count(), 3);
    assert_eq!(execution.visible_family_count(), 3);
    assert_eq!(execution.admission_report().restore_accept_count, 3);
    assert_eq!(execution.admission_report().restore_rejection_count, 0);
}

#[test]
fn authoritative_export_restore_rejects_publication_conflicts_before_visibility() {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let envelope = latest_envelope(&runtime);

    let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    store.append_canonical_commit(envelope).unwrap();
    let export = store.export_authoritative_records();

    let mut conflicts = BTreeMap::new();
    conflicts.insert(
        CompatibilityFamilyKind::BranchVersionDagRecord,
        RestorePublicationConflictSet::new(vec![RestorePublicationConflictUnit::new(
            CompatibilityFamilyKind::BranchVersionDagRecord.family_id(),
            RestorePublicationConflictKind::BranchHead,
        )]),
    );

    let error = ForgeStore::execute_restore_from_authoritative_export_with_conflicts_for_test(
        export.admit_restore(),
        conflicts,
    )
    .expect_err("conflicted restore publication must reject before store visibility");

    assert_eq!(error.kind(), &StoreErrorKind::CompatibilityRestoreRejected);
}

#[test]
fn authoritative_export_restore_matches_certification_lane_evidence() {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let envelope = latest_envelope(&runtime);

    let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    store.append_canonical_commit(envelope).unwrap();
    let export = store.export_authoritative_records();

    let (_restored, execution) =
        ForgeStore::restore_from_authoritative_export_with_compatibility(export.admit_restore())
            .expect("restore execution should admit visible authoritative families");
    let certification = Milestone12CertificationRunner::first_ship().run().unwrap();
    let admitted_lane = certification
        .evidence_bundle()
        .lane_outcomes()
        .iter()
        .find(|lane| {
            lane.lane_kind() == Milestone12CertificationLaneKind::RestoreScopedBackupAdmitted
        })
        .unwrap();
    let rejected_lane = certification
        .evidence_bundle()
        .lane_outcomes()
        .iter()
        .find(|lane| {
            lane.lane_kind() == Milestone12CertificationLaneKind::RestorePublicationConflictRejected
        })
        .unwrap();

    assert_eq!(
        admitted_lane.relation(),
        Some(crate::CompatibilityRelation::BackwardRead)
    );
    assert_eq!(
        admitted_lane.status(),
        Milestone12CertificationLaneStatus::Accepted
    );
    assert!(
        execution.admission_report().restore_accept_count
            >= admitted_lane.counters().restore_accept_count
    );
    assert!(
        execution.admission_report().manifest_index_lookup_count
            >= admitted_lane.counters().manifest_index_lookup_count
    );
    assert_eq!(execution.admission_report().restore_rejection_count, 0);
    assert_eq!(
        rejected_lane.rejection_kind(),
        Some(crate::CompatibilityRejectionKind::RestorePublicationConflictRejected)
    );
    assert_eq!(
        rejected_lane.counters().restore_publication_conflict_rejection_count,
        1
    );
}
