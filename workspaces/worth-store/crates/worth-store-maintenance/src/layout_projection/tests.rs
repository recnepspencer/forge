use worth_store_buffer_pool::{
    AllocationAdmission, AllocationByteBudget, AllocationEnvelopeDeclaration,
    BackgroundEnvelopeAdmission, BackgroundEnvelopeRequest, BackgroundWorkBudgetSnapshot,
    FixedMetadataReservation,
};

use crate::{CompactionPlanningMemoryEnvelope, ImportExportMemoryEnvelope};

#[test]
fn maintenance_queue_layout_preserves_declared_budget_from_compaction_envelope() {
    let envelope = admitted_background_envelope(BackgroundEnvelopeRequest::compaction_planning());
    let report = CompactionPlanningMemoryEnvelope::from_admitted(envelope)
        .expect("compaction planning envelope should admit")
        .project_maintenance_queue_layout();

    assert_eq!(report.family_id().label(), "maintenance_queue_declaration");
    assert_eq!(report.declared_budget().resident_frames(), 2);
    assert_eq!(report.declared_budget().resident_bytes(), 256);
    assert_eq!(report.declared_budget().pinned_pages(), 1);
    assert_eq!(report.declared_budget().allocation_bytes(), 128);
    assert_eq!(report.exact_counters().allocation_bytes_admitted(), 128);
}

#[test]
fn maintenance_queue_layout_preserves_declared_budget_from_import_export_envelope() {
    let envelope = admitted_background_envelope(BackgroundEnvelopeRequest::import_export());
    let report = ImportExportMemoryEnvelope::from_admitted(envelope)
        .expect("import-export envelope should admit")
        .project_maintenance_queue_layout();

    assert_eq!(report.family_id().label(), "maintenance_queue_declaration");
    assert_eq!(report.declared_budget().resident_frames(), 2);
    assert_eq!(report.declared_budget().resident_bytes(), 256);
    assert_eq!(report.declared_budget().allocation_bytes(), 128);
    assert_eq!(report.exact_counters().allocation_bytes_admitted(), 128);
}

fn admitted_background_envelope(
    builder: worth_store_buffer_pool::BackgroundEnvelopeRequestBuilder,
) -> worth_store_buffer_pool::AdmittedBackgroundEnvelope {
    let request = builder
        .resident_frames(2)
        .resident_bytes(256)
        .pin_pages_for_bounded_step(1)
        .allocation_bytes(128)
        .finish();
    let mut admission = BackgroundEnvelopeAdmission::new();
    let mut allocation = AllocationAdmission::from_declaration(
        AllocationEnvelopeDeclaration::declare()
            .foreground(bytes(512))
            .maintenance(bytes(512))
            .recovery(bytes(512))
            .scrub(bytes(512))
            .import_export(bytes(512))
            .streaming(bytes(512))
            .fixed_metadata(FixedMetadataReservation::constant_bytes(64).unwrap())
            .seal()
            .unwrap(),
    );
    admission
        .admit(
            request,
            BackgroundWorkBudgetSnapshot::foreground_reserved(16, 4, 0, 16),
            &mut allocation,
        )
        .expect("background envelope should admit on the real production path")
}

fn bytes(bytes: u64) -> AllocationByteBudget {
    AllocationByteBudget::bytes(bytes).unwrap()
}
