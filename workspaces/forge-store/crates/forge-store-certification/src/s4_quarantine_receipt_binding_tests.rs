use crate::courtroom::harness::test_support::physical_container_integrity_test_support::{
    inspect_page_report, page_payload_with_record,
};
use crate::courtroom::harness::test_support::physical_scope_admission_test_support::{
    root_with_slot, scope_membership, validation, with_checked_frame,
};
use forge_store_physical_format::{
    CheckpointAdjacencyPosture, PhysicalReferenceScope, RootManifestIntegrityPosture,
};
use forge_store_physical_integrity::{
    ExecutedQuarantineFinding, PhysicalIntegrityEvidenceAuthority, PhysicalIntegrityEvidenceBundle,
    PhysicalIntegrityEvidenceProfile, PhysicalQuarantineAuthority, PhysicalScopeAdmission,
    PhysicalScopeAdmissionRequest, QuarantineRecord, QuarantineSealRequest,
    ScopedPhysicalValidatorInput, StoreExecutedIntegrityEvidence, WalFrameDamageDenial,
    WalFrameIntegrityAuthority, WalFrameIntegrityInspectionRequest,
};
use forge_store_recovery_physics::{
    IntegrityVettedWalFrame, QuarantineSummary, RecoveryBlockedByIntegrityDamage,
    RecoveryIntegrityHandoffReceipt, S4IntegrityHandoffDenialKind,
};

#[test]
fn handoff_constructors_reject_copied_or_mismatched_receipt_surfaces() {
    let page = inspect_page_report(&page_payload_with_record(b"receipt-gate"));
    let wal = inspect_wal_frame(CheckpointAdjacencyPosture::NotCheckpointAdjacent);
    let wal_damage = inspect_wal_damage(
        wal_payload(b"receipt-gate"),
        2,
        CheckpointAdjacencyPosture::NotCheckpointAdjacent,
    );
    let page_receipt = receipt(StoreExecutedIntegrityEvidence::authoritative_page(&page));
    let (record, damage) = sealed_quarantine_for_wal_damage(&wal_damage);
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

    let denial = QuarantineSummary::from_recovery_blocking_damage(&record, page_receipt, &damage)
        .unwrap_err();
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

    QuarantineSummary::from_recovery_blocking_damage(&record, quarantine_receipt, &damage).unwrap();
}

#[test]
fn quarantine_summary_rejects_receipt_evidence_from_different_sealed_record() {
    let first_wal = inspect_wal_damage(
        wal_payload(b"s4-quarantine-receipt-first"),
        2,
        CheckpointAdjacencyPosture::NotCheckpointAdjacent,
    );
    let second_wal = inspect_wal_damage(
        wal_payload(b"s4-quarantine-receipt-second"),
        3,
        CheckpointAdjacencyPosture::NotCheckpointAdjacent,
    );
    let (first_record, _) = sealed_quarantine_for_wal_damage(&first_wal);
    let (second_record, second_damage) = sealed_quarantine_for_wal_damage(&second_wal);
    let first_receipt = receipt_for(&first_record);

    let denial = QuarantineSummary::from_recovery_blocking_damage(
        &second_record,
        first_receipt,
        &second_damage,
    )
    .expect_err("quarantine summary must reject copied receipt evidence");

    assert_eq!(
        denial.kind(),
        S4IntegrityHandoffDenialKind::ReceiptBasisMismatch
    );
}

fn inspect_wal_frame(
    adjacency: CheckpointAdjacencyPosture,
) -> forge_store_physical_integrity::WalFrameIntegrityReport {
    let mut report = None;
    with_wal_input(b"WALF|crc32c|4|ok|DATA".to_vec(), 2, adjacency, |input| {
        let request = WalFrameIntegrityInspectionRequest::from_admitted_wal_frame(input).unwrap();
        report = Some(WalFrameIntegrityAuthority::s3().inspect(request).unwrap());
    });
    report.unwrap()
}

fn inspect_wal_damage(
    payload: Vec<u8>,
    page_id: u64,
    adjacency: CheckpointAdjacencyPosture,
) -> WalFrameDamageDenial {
    let mut denial = None;
    with_wal_input(payload, page_id, adjacency, |input| {
        let request = WalFrameIntegrityInspectionRequest::from_admitted_wal_frame(input).unwrap();
        denial = Some(
            WalFrameIntegrityAuthority::s3()
                .inspect(request)
                .unwrap_err(),
        );
    });
    denial.unwrap()
}

fn with_wal_input(
    payload: Vec<u8>,
    page_id: u64,
    adjacency: CheckpointAdjacencyPosture,
    run: impl FnOnce(ScopedPhysicalValidatorInput<'_>),
) {
    let validation = validation(1, page_id, 3, 7);
    let scope = PhysicalReferenceScope::wal_frame(validation);
    let root = root_with_slot(1, page_id, 3, 7);
    let membership = scope_membership(&root, scope);
    with_checked_frame(&payload, validation, |checked| {
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

fn wal_payload(label: &[u8]) -> Vec<u8> {
    let mut payload = b"WALF|crc32c|4|checksum-fail|".to_vec();
    payload.extend_from_slice(label);
    payload
}

fn sealed_quarantine_for_wal_damage(
    wal: &WalFrameDamageDenial,
) -> (QuarantineRecord, RecoveryBlockedByIntegrityDamage) {
    let record = PhysicalQuarantineAuthority::seal(QuarantineSealRequest::from_executed_finding(
        ExecutedQuarantineFinding::from_wal_frame_denial(wal).unwrap(),
    ))
    .unwrap();
    let damage = RecoveryBlockedByIntegrityDamage::damaged_wal_frame(wal);
    (record, damage)
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
