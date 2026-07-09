use crate::{
    courtroom::harness::test_support::record_view_evidence_test_support::{
        admit_payload_frame, record_view_table_without_conflicts, resident_frame_table,
    },
    RecordViewEvidenceDenial, RecordViewEvidenceReport, RecordViewEvidenceRow,
};

#[test]
fn real_dirty_conflict_certifies_view_mutation_compatibility() {
    let mut table = resident_frame_table();
    let admission = admit_payload_frame(&mut table, 7, 2, b"protected");
    let pinned = table
        .lease_page(admission.resident_frame_token())
        .unwrap()
        .pin()
        .unwrap();
    std::mem::WORTHt(pinned);

    let _ = table
        .mark_dirty(admission.resident_frame_token())
        .unwrap_err();
    let report = RecordViewEvidenceReport::from_table(
        RecordViewEvidenceRow::ViewMutationConflictDeniedBeforeDirtyMutation,
        &table,
    )
    .unwrap();

    assert_eq!(
        report.row(),
        RecordViewEvidenceRow::ViewMutationConflictDeniedBeforeDirtyMutation
    );
}

#[test]
fn real_publication_conflict_certifies_view_publication_compatibility() {
    let mut table = resident_frame_table();
    let admission = admit_payload_frame(&mut table, 7, 2, b"publication");
    let dirty = table.mark_dirty(admission.resident_frame_token()).unwrap();
    let plan = table.plan_dirty_publication(dirty.identity()).unwrap();
    let pinned = table
        .lease_page(admission.resident_frame_token())
        .unwrap()
        .pin()
        .unwrap();
    std::mem::WORTHt(pinned);

    let _ = table.record_dirty_write_scheduled(plan).unwrap_err();
    let report = RecordViewEvidenceReport::from_table(
        RecordViewEvidenceRow::ViewPublicationConflictDeniedBeforeScheduling,
        &table,
    )
    .unwrap();

    assert_eq!(
        report.row(),
        RecordViewEvidenceRow::ViewPublicationConflictDeniedBeforeScheduling
    );
}

#[test]
fn unrecorded_record_view_conflicts_do_not_certify_conflict_rows() {
    let table = record_view_table_without_conflicts();

    let mutation_denial = RecordViewEvidenceReport::from_table(
        RecordViewEvidenceRow::ViewMutationConflictDeniedBeforeDirtyMutation,
        &table,
    )
    .unwrap_err();
    let publication_denial = RecordViewEvidenceReport::from_table(
        RecordViewEvidenceRow::ViewPublicationConflictDeniedBeforeScheduling,
        &table,
    )
    .unwrap_err();

    assert_eq!(
        mutation_denial,
        RecordViewEvidenceDenial::UnprovenRecordViewRow
    );
    assert_eq!(
        publication_denial,
        RecordViewEvidenceDenial::UnprovenRecordViewRow
    );
}

#[test]
fn wrong_record_view_evidence_row_is_rejected_for_table_counters() {
    let table = record_view_table_without_conflicts();
    let denial = RecordViewEvidenceReport::from_table(
        RecordViewEvidenceRow::ZeroCopyLeaseScopedPhysicalBytes,
        &table,
    )
    .unwrap_err();

    assert_eq!(denial, RecordViewEvidenceDenial::WrongRow);
}
