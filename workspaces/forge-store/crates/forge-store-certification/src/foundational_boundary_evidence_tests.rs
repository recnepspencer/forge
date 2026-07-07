use crate::{
    courtroom::harness::test_support::record_view_evidence_test_support::{
        admit_payload_frame, allocation_admission, framed_record, resident_frame_table,
    },
    AllocationEnvelopePerformanceReceipt, BufferPoolProvenanceAttachment,
    CompletedResidencyBoundaryReceipt, CopyMaterializationPerformanceReceipt,
    FoundationalBoundaryAuthorityResult, FoundationalBoundaryEvidenceDenial,
    FoundationalEvidenceProfile, MaterializationProfileReport, ResidentMemoryPerformanceReceipt,
    ZeroCopyLayoutPostureReport,
};
use forge_store_buffer_pool::{
    AllocationCounterSnapshot, AllocationRequest, AllocationScope,
    BufferPoolExecutedEvidenceSource, RecordCopyCounterSnapshot, RecordViewMaterializationProfile,
    ResidentFrameCounterSnapshot,
};

#[test]
fn completed_boundary_receipt_uses_distinct_executed_store_reports() {
    let source = executed_source();
    let receipt = CompletedResidencyBoundaryReceipt::from_executed_store_counters(
        source,
        FoundationalEvidenceProfile::full(),
    )
    .unwrap();

    assert_eq!(
        receipt
            .resident_memory()
            .counters()
            .resident_bytes()
            .as_bytes(),
        source
            .counters()
            .resident_memory()
            .resident_bytes()
            .as_bytes()
    );
    assert_eq!(
        receipt
            .allocation()
            .counters()
            .scope(AllocationScope::Foreground)
            .allocated_bytes(),
        12
    );
    assert_eq!(receipt.copy_materialization().counters().copied_bytes(), 12);
    assert_eq!(receipt.layout().zero_copy_admissions(), 1);
    assert!(!receipt.layout().semantic_domain_object_claimed());
    assert_eq!(receipt.provenance().counters(), source.counters());
    assert_eq!(
        receipt.profile().authority_result(),
        FoundationalBoundaryAuthorityResult::CounterBackedStoreExecution
    );
}

#[test]
fn reduced_profile_removes_diagnostics_without_changing_authority_or_counters() {
    let source = executed_source();
    let full = CompletedResidencyBoundaryReceipt::from_executed_store_counters(
        source,
        FoundationalEvidenceProfile::full(),
    )
    .unwrap();
    let reduced = CompletedResidencyBoundaryReceipt::from_executed_store_counters(
        source,
        FoundationalEvidenceProfile::reduced(),
    )
    .unwrap();

    assert!(full.profile().optional_diagnostic_count() > 0);
    assert_eq!(reduced.profile().optional_diagnostic_count(), 0);
    assert_eq!(
        full.profile().authority_result(),
        reduced.profile().authority_result()
    );
    assert_eq!(
        full.resident_memory().counters(),
        reduced.resident_memory().counters()
    );
    assert_eq!(
        full.allocation().counters(),
        reduced.allocation().counters()
    );
    assert_eq!(full.profile().counters(), reduced.profile().counters());
    assert_eq!(
        full.copy_materialization().counters(),
        reduced.copy_materialization().counters()
    );
    assert_eq!(full.layout(), reduced.layout());
    assert_eq!(full.provenance(), reduced.provenance());
}

#[test]
fn independent_report_constructor_materializes_same_foundational_basis() {
    let source = executed_source();
    let counters = source.counters();
    let direct = CompletedResidencyBoundaryReceipt::from_executed_store_counters(
        source,
        FoundationalEvidenceProfile::full(),
    )
    .unwrap();
    let resident_memory =
        ResidentMemoryPerformanceReceipt::from_executed_counters(counters.resident_memory())
            .unwrap();
    let allocation =
        AllocationEnvelopePerformanceReceipt::from_executed_counters(counters.allocation())
            .unwrap();
    let copy_materialization = CopyMaterializationPerformanceReceipt::from_executed_counters(
        counters.copy_materialization(),
    )
    .unwrap();
    let layout =
        ZeroCopyLayoutPostureReport::from_executed_copy_counters(counters.copy_materialization())
            .unwrap();
    let profile = MaterializationProfileReport::from_executed_counters(
        FoundationalEvidenceProfile::full(),
        counters,
    );
    let provenance = BufferPoolProvenanceAttachment::from_executed_counters(counters);

    let independent = CompletedResidencyBoundaryReceipt::from_distinct_reports(
        resident_memory,
        allocation,
        copy_materialization,
        layout,
        profile,
        provenance,
    )
    .unwrap();

    assert_eq!(independent, direct);
}

#[test]
fn receipt_denies_when_required_evidence_counters_are_not_executed() {
    let resident_denial = ResidentMemoryPerformanceReceipt::from_executed_counters(
        ResidentFrameCounterSnapshot::empty(),
    )
    .unwrap_err();
    let allocation_denial = AllocationEnvelopePerformanceReceipt::from_executed_counters(
        AllocationCounterSnapshot::default(),
    )
    .unwrap_err();
    let copy_denial = CopyMaterializationPerformanceReceipt::from_executed_counters(
        RecordCopyCounterSnapshot::empty(),
    )
    .unwrap_err();

    assert_eq!(
        resident_denial,
        FoundationalBoundaryEvidenceDenial::MissingResidentMemoryCounters
    );
    assert_eq!(
        copy_denial,
        FoundationalBoundaryEvidenceDenial::MissingCopyCounters
    );
    assert_eq!(
        allocation_denial,
        FoundationalBoundaryEvidenceDenial::MissingAllocationCounters
    );
}

#[test]
fn full_and_reduced_profiles_preserve_evidence_denials() {
    let source = executed_source();
    let counters = source.counters();
    let different_counters = executed_source_with_payload(b"different-counter-basis").counters();

    assert_same_profile_denial_for_report_basis(
        counters,
        different_counters,
        FoundationalBoundaryEvidenceDenial::ReportBasisMismatch,
    );
}

fn executed_source() -> BufferPoolExecutedEvidenceSource {
    executed_source_with_payload(b"certify-view")
}

fn executed_source_with_payload(payload: &'static [u8]) -> BufferPoolExecutedEvidenceSource {
    let mut table = resident_frame_table();
    let admission = admit_payload_frame(&mut table, 7, 2, payload);
    table
        .resident_frame(admission.resident_frame_token())
        .unwrap();
    let mut allocation = allocation_admission(32);

    let bounded = {
        let framed = framed_record(7, 2, payload);
        let lease = table.lease_page(admission.resident_frame_token()).unwrap();
        let mut pinned = lease.pin().unwrap();
        let zero_copy = pinned
            .zero_copy_record_view(framed, RecordViewMaterializationProfile::PhysicalBytesOnly)
            .unwrap();
        let request =
            AllocationRequest::copied_payload(AllocationScope::Foreground, payload.len() as u64)
                .unwrap();
        let grant = allocation.admit(request).unwrap();
        let receipt = allocation.record_allocation(grant).unwrap();
        zero_copy.bounded_copy(receipt).unwrap()
    };

    BufferPoolExecutedEvidenceSource::from_store_execution(&table, &allocation, &bounded).unwrap()
}

fn assert_same_profile_denial_for_report_basis(
    counters: forge_store_buffer_pool::BufferPoolCounterSnapshot,
    mismatched_counters: forge_store_buffer_pool::BufferPoolCounterSnapshot,
    expected: FoundationalBoundaryEvidenceDenial,
) {
    let full = completed_receipt_basis_mismatch_denial(
        counters,
        FoundationalEvidenceProfile::full(),
        mismatched_counters,
    );
    let reduced = completed_receipt_basis_mismatch_denial(
        counters,
        FoundationalEvidenceProfile::reduced(),
        mismatched_counters,
    );

    assert_eq!(full, expected);
    assert_eq!(reduced, expected);
}

fn completed_receipt_basis_mismatch_denial(
    counters: forge_store_buffer_pool::BufferPoolCounterSnapshot,
    profile: FoundationalEvidenceProfile,
    mismatched_counters: forge_store_buffer_pool::BufferPoolCounterSnapshot,
) -> FoundationalBoundaryEvidenceDenial {
    CompletedResidencyBoundaryReceipt::from_distinct_reports(
        ResidentMemoryPerformanceReceipt::from_executed_counters(counters.resident_memory())
            .unwrap(),
        AllocationEnvelopePerformanceReceipt::from_executed_counters(counters.allocation())
            .unwrap(),
        CopyMaterializationPerformanceReceipt::from_executed_counters(
            counters.copy_materialization(),
        )
        .unwrap(),
        ZeroCopyLayoutPostureReport::from_executed_copy_counters(counters.copy_materialization())
            .unwrap(),
        MaterializationProfileReport::from_executed_counters(profile, mismatched_counters),
        BufferPoolProvenanceAttachment::from_executed_counters(counters),
    )
    .unwrap_err()
}
