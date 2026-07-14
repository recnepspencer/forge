use crate::{
    AllocationAdmission, AllocationByteBudget, AllocationCounterSnapshot,
    AllocationEnvelopeDeclaration, AllocationRequest, AllocationScope, BufferPoolCounterSnapshot,
    BufferPoolEvidenceSourceDenial, BufferPoolExecutedEvidenceSource, FixedMetadataReservation,
    RecordCopyCounterSnapshot, ResidentFrameCounterSnapshot,
};

#[test]
fn executed_evidence_source_rejects_empty_counter_snapshots() {
    let denial = BufferPoolCounterSnapshot::from_executed_store_counters(
        ResidentFrameCounterSnapshot::empty(),
        AllocationCounterSnapshot::default(),
        RecordCopyCounterSnapshot::empty(),
    )
    .unwrap_err();

    assert_eq!(
        denial,
        BufferPoolEvidenceSourceDenial::NoExecutedStoreCounters
    );
}

#[test]
fn executed_evidence_source_rejects_denied_allocation_attempts() {
    let mut admission = allocation_admission(64);
    let grant = admission
        .admit(AllocationRequest::copied_payload(AllocationScope::Foreground, 32).unwrap())
        .unwrap();
    admission.record_allocation(grant).unwrap();

    admission
        .admit(AllocationRequest::copied_payload(AllocationScope::Foreground, 33).unwrap())
        .unwrap_err();

    let denial = BufferPoolExecutedEvidenceSource::from_allocation_execution(&admission)
        .expect_err("denied allocation attempts must not mint certifying evidence");

    assert_eq!(
        denial,
        BufferPoolEvidenceSourceDenial::ExecutionContainedDeniedAllocation
    );
}

fn allocation_admission(bytes: u64) -> AllocationAdmission {
    let budget = AllocationByteBudget::bytes(bytes).unwrap();
    let envelope = AllocationEnvelopeDeclaration::declare()
        .foreground(budget)
        .maintenance(budget)
        .recovery(budget)
        .scrub(budget)
        .import_export(budget)
        .streaming(budget)
        .fixed_metadata(FixedMetadataReservation::constant_bytes(8).unwrap())
        .seal()
        .unwrap();
    AllocationAdmission::from_declaration(envelope)
}
