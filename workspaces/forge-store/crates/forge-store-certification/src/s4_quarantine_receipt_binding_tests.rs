use crate::physical_container_integrity_test_support::{
    inspect_page_report, page_payload_with_record,
};
use crate::physical_scope_admission_test_support::{
    page_cell, page_request, root_with_slot, scope_membership, validation, with_checked_frame,
    with_checked_page,
};
use forge_store_physical_format::{
    CheckpointAdjacencyPosture, PhysicalReferenceScope, RootManifestIntegrityPosture,
};
use forge_store_physical_integrity::{
    ExecutedQuarantineFinding, PhysicalContainerIntegrity, PhysicalIntegrityEvidenceAuthority,
    PhysicalIntegrityEvidenceBundle, PhysicalIntegrityEvidenceProfile, PhysicalQuarantineAuthority,
    PhysicalScopeAdmission, PhysicalScopeAdmissionRequest, QuarantineRecord, QuarantineSealRequest,
    ScopedPhysicalValidatorInput, StoreExecutedIntegrityEvidence, WalFrameIntegrityAuthority,
    WalFrameIntegrityInspectionRequest,
};
use forge_store_recovery_physics::{
    IntegrityVettedWalFrame, QuarantineSummary, RecoveryBlockedByIntegrityDamage,
    RecoveryIntegrityHandoffReceipt, S4IntegrityHandoffDenialKind,
};

#[test]
fn handoff_constructors_reject_copied_or_mismatched_receipt_surfaces() {
    let page = inspect_page_report(&page_payload_with_record(b"receipt-gate"));
    let wal = inspect_wal_frame(CheckpointAdjacencyPosture::NotCheckpointAdjacent);
    let page_receipt = receipt(StoreExecutedIntegrityEvidence::authoritative_page(&page));
    let record = PhysicalQuarantineAuthority::seal(QuarantineSealRequest::from_executed_finding(
        ExecutedQuarantineFinding::intact_page(&page),
    ))
    .unwrap();
    let quarantine_evidence = quarantine_evidence_for(&record);
    let quarantine_receipt =
        RecoveryIntegrityHandoffReceipt::from_quarantine_receipt_evidence(&quarantine_evidence)
            .unwrap();

    let denial =
        IntegrityVettedWalFrame::from_integrity_report(&wal, page_receipt.clone()).unwrap_err();
    assert_eq!(
        denial.kind(),
        S4IntegrityHandoffDenialKind::ReceiptScopeMismatch
    );

    let denial =
        RecoveryIntegrityHandoffReceipt::from_executed_evidence(&quarantine_evidence).unwrap_err();
    assert_eq!(
        denial.kind(),
        S4IntegrityHandoffDenialKind::EvidenceIsNotAuthoritativeCurrent
    );

    let denial = QuarantineSummary::from_quarantine_record(&record, page_receipt).unwrap_err();
    assert_eq!(
        denial.kind(),
        S4IntegrityHandoffDenialKind::EvidenceIsNotReceiptEvidence
    );
    let denial =
        RecoveryBlockedByIntegrityDamage::unresolved_authority_damage(&record).unwrap_err();
    assert_eq!(
        denial.kind(),
        S4IntegrityHandoffDenialKind::UnresolvedAuthorityDamageRequiresAuthorityClassification
    );
    QuarantineSummary::from_quarantine_record(&record, quarantine_receipt).unwrap();
}

#[test]
fn quarantine_summary_rejects_receipt_evidence_from_different_sealed_record() {
    let first_record = scoped_sealed_record(b"s4-quarantine-receipt-first", 2);
    let second_record = scoped_sealed_record(b"s4-quarantine-receipt-second", 3);
    let first_receipt = receipt_for(&first_record);

    let denial = QuarantineSummary::from_quarantine_record(&second_record, first_receipt)
        .expect_err("quarantine summary must reject copied receipt evidence");

    assert_eq!(
        denial.kind(),
        S4IntegrityHandoffDenialKind::ReceiptBasisMismatch
    );
}

fn scoped_sealed_record(label: &'static [u8], page_id: u64) -> QuarantineRecord {
    let page_payload = page_payload_with_record(label);
    let page = inspect_page_report_for_scope(&page_payload, page_id);
    PhysicalQuarantineAuthority::seal(QuarantineSealRequest::from_executed_finding(
        ExecutedQuarantineFinding::intact_page(&page),
    ))
    .expect("scoped intact page quarantine record seals")
}

fn inspect_page_report_for_scope(
    page_payload: &[u8],
    page_id: u64,
) -> forge_store_physical_integrity::PageIntegrityReport {
    let mut report = None;
    let cell = page_cell(1, page_id, 7);
    with_checked_page(page_payload, cell, |checked| {
        let scope = PhysicalReferenceScope::page(cell);
        let root = root_with_slot(1, page_id, 3, 7);
        let membership = scope_membership(&root, scope);
        let request = page_request(&checked, scope, membership);
        let admission = PhysicalScopeAdmission::admit_page(checked, request).unwrap();
        let input = ScopedPhysicalValidatorInput::page(admission).unwrap();
        report = Some(PhysicalContainerIntegrity::inspect_page(input).unwrap());
    });
    report.unwrap()
}

fn inspect_wal_frame(
    adjacency: CheckpointAdjacencyPosture,
) -> forge_store_physical_integrity::WalFrameIntegrityReport {
    let mut report = None;
    with_wal_frame_input(adjacency, |input| {
        let request = WalFrameIntegrityInspectionRequest::from_admitted_wal_frame(input).unwrap();
        report = Some(WalFrameIntegrityAuthority::s3().inspect(request).unwrap());
    });
    report.unwrap()
}

fn with_wal_frame_input(
    adjacency: CheckpointAdjacencyPosture,
    run: impl FnOnce(ScopedPhysicalValidatorInput<'_>),
) {
    let payload = b"WALF|crc32c|4|ok|DATA";
    let validation = validation(1, 2, 3, 7);
    let scope = PhysicalReferenceScope::wal_frame(validation);
    let root = root_with_slot(1, 2, 3, 7);
    let membership = scope_membership(&root, scope);
    with_checked_frame(payload, validation, |checked| {
        let request = PhysicalScopeAdmissionRequest::frame(
            scope,
            membership,
            RootManifestIntegrityPosture::current_root_admitted(membership),
            adjacency,
            checked.gate_evidence().coverage_basis().clone(),
        );
        let admission = PhysicalScopeAdmission::admit_frame(checked, request).unwrap();
        run(ScopedPhysicalValidatorInput::wal_frame(admission).unwrap());
    });
}

fn quarantine_evidence_for(record: &QuarantineRecord) -> PhysicalIntegrityEvidenceBundle {
    PhysicalIntegrityEvidenceAuthority::store_local()
        .materialize(
            StoreExecutedIntegrityEvidence::receipt_evidence(record),
            PhysicalIntegrityEvidenceProfile::reduced(),
        )
        .expect("sealed quarantine record materializes receipt evidence")
}

fn receipt_for(record: &QuarantineRecord) -> RecoveryIntegrityHandoffReceipt {
    let evidence = quarantine_evidence_for(record);
    RecoveryIntegrityHandoffReceipt::from_quarantine_receipt_evidence(&evidence)
        .expect("receipt evidence admits to S.4 receipt wrapper")
}

fn receipt(source: StoreExecutedIntegrityEvidence<'_>) -> RecoveryIntegrityHandoffReceipt {
    let evidence = PhysicalIntegrityEvidenceAuthority::store_local()
        .materialize(source, PhysicalIntegrityEvidenceProfile::reduced())
        .unwrap();
    RecoveryIntegrityHandoffReceipt::from_executed_evidence(&evidence).unwrap()
}
