use crate::{
    courtroom::harness::test_support::physical_container_integrity_test_support::{
        inspect_page_report, page_payload_with_record,
    },
    courtroom::harness::test_support::physical_scope_admission_test_support::{
        page_cell, page_slot_admission, root_admission, root_with_slot, scope_membership,
        validation, with_checked_frame, with_checked_page,
    },
    courtroom::harness::test_support::s3_integrity_readiness_test_support::s3_integrity_readiness,
    courtroom::layout::derived_index_damage_tests::inspect_with_damaged_authority,
};
use forge_store_physical_format::{
    CheckpointAdjacencyPosture, PhysicalReferenceScope, RootManifestIntegrityPosture,
};
use forge_store_physical_integrity::{
    ExecutedQuarantineFinding, ManifestExpectedReference, ManifestIntegrityAuthority,
    ManifestIntegrityInspectionRequest, PhysicalIntegrityEvidenceAuthority,
    PhysicalIntegrityEvidenceProfile, PhysicalQuarantineAuthority, PhysicalScopeAdmission,
    PhysicalScopeAdmissionRequest, QuarantineRecord, QuarantineSealRequest,
    ScopedPhysicalValidatorInput, StoreExecutedIntegrityEvidence, WalFrameIntegrityAuthority,
    WalFrameIntegrityInspectionRequest,
};
use forge_store_recovery_physics::{
    AdmittedRecoveryIntegrityInput, BoundedInspectionEnvelopeEvidence, IntegrityHandoffAdmission,
    IntegrityHandoffDenialKind, IntegrityHandoffPayload, IntegrityVettedCheckpointRecord,
    IntegrityVettedPageFrameRecord, IntegrityVettedRootManifestRecord,
    IntegrityVettedSegmentManifestRecord, IntegrityVettedWalFrame, QuarantineSummary,
    RecoveryBlockedByIntegrityDamage, RecoveryIntegrityHandoffReceipt,
};

pub(crate) fn intact_readiness(label: &str) -> AdmittedRecoveryIntegrityInput {
    admit_s4_handoff_payload(intact_handoff_payload(label))
}

pub(crate) fn admit_s4_handoff_payload(
    payload: IntegrityHandoffPayload,
) -> AdmittedRecoveryIntegrityInput {
    IntegrityHandoffAdmission::admit(s3_integrity_readiness().payload(), payload)
        .expect("complete S.3 readiness admits S.4 integrity handoff")
}

pub(crate) fn manifest_receipt_swap_denial_kind(
    first_generation: u64,
    second_generation: u64,
) -> IntegrityHandoffDenialKind {
    let first_manifest = inspect_manifest_for_generation(first_generation);
    let second_manifest = inspect_manifest_for_generation(second_generation);
    let first_receipt = receipt(StoreExecutedIntegrityEvidence::authoritative_manifest(
        &first_manifest,
    ));
    IntegrityVettedRootManifestRecord::from_manifest_report(&second_manifest, first_receipt)
        .unwrap_err()
        .kind()
}

pub(crate) fn forged_inspection_envelope_counter_denial_kind(
    label: &'static [u8],
) -> IntegrityHandoffDenialKind {
    let payload = page_payload_with_record(label);
    let mut denial = None;
    with_checked_page(&payload, page_cell(1, 2, 7), |checked| {
        let checked_bytes = checked.counters().checked_byte_count();
        denial = Some(
            BoundedInspectionEnvelopeEvidence::from_checked_page(
                &checked,
                checked_bytes - 1,
                checked_bytes,
                checked_bytes,
            )
            .unwrap_err()
            .kind(),
        );
    });
    denial.unwrap()
}

fn intact_handoff_payload(label: &str) -> IntegrityHandoffPayload {
    let page_payload = page_payload_with_record(label.as_bytes());
    let page = inspect_page_report(&page_payload);
    let wal = inspect_wal_frame(CheckpointAdjacencyPosture::NotCheckpointAdjacent);
    let checkpoint = inspect_checkpoint_record();
    let manifest = inspect_manifest_for_generation(7);
    let (quarantine_record, quarantine_receipt, quarantine_damage) =
        recovery_blocking_quarantine_binding();
    let page_record = IntegrityVettedPageFrameRecord::from_page_report(
        &page,
        receipt(StoreExecutedIntegrityEvidence::authoritative_page(&page)),
    )
    .unwrap();
    let wal_record = IntegrityVettedWalFrame::from_integrity_report(
        &wal,
        receipt(StoreExecutedIntegrityEvidence::authoritative_wal_frame(
            &wal,
        )),
    )
    .unwrap();
    let checkpoint_record = IntegrityVettedCheckpointRecord::from_integrity_report(
        &checkpoint,
        receipt(StoreExecutedIntegrityEvidence::authoritative_checkpoint_record(&checkpoint)),
    )
    .unwrap();
    let manifest_receipt = receipt(StoreExecutedIntegrityEvidence::authoritative_manifest(
        &manifest,
    ));
    let root = IntegrityVettedRootManifestRecord::from_manifest_report(
        &manifest,
        manifest_receipt.clone(),
    )
    .unwrap();
    let segment =
        IntegrityVettedSegmentManifestRecord::from_manifest_report(&manifest, manifest_receipt)
            .unwrap();
    IntegrityHandoffPayload::declare()
        .root_manifest(root)
        .segment_manifest(segment)
        .page_frame(page_record)
        .wal_frame(wal_record)
        .checkpoint_record(checkpoint_record)
        .damage_map(
            forge_store_recovery_physics::IntegrityDamageMap::new()
                .with_recovery_blocking_quarantine(
                    &quarantine_record,
                    quarantine_receipt,
                    &quarantine_damage,
                )
                .unwrap(),
        )
        .inspection_envelope(inspection_envelope_for_payload(&page_payload))
        .seal()
        .unwrap()
}

fn inspect_manifest_for_generation(
    generation: u64,
) -> forge_store_physical_integrity::ManifestIntegrityReport {
    let root = root_with_slot(1, 2, 3, generation);
    ManifestIntegrityAuthority::s3()
        .inspect_manifest(
            ManifestIntegrityInspectionRequest::from_root_publication(
                root.clone(),
                root_admission(&root),
            )
            .with_expected_reference(ManifestExpectedReference::page_slot(
                page_slot_admission(1, 2, 3, generation),
            )),
        )
        .unwrap()
}

fn inspection_envelope_for_payload(payload: &[u8]) -> BoundedInspectionEnvelopeEvidence {
    let mut envelope = None;
    with_checked_page(payload, page_cell(1, 2, 7), |checked| {
        envelope = Some(
            BoundedInspectionEnvelopeEvidence::from_checked_page(&checked, 128, 128, 128).unwrap(),
        );
    });
    envelope.unwrap()
}

fn inspect_checkpoint_record() -> forge_store_physical_integrity::CheckpointRecordIntegrityReport {
    let mut report = None;
    with_wal_frame_input(CheckpointAdjacencyPosture::CheckpointAdjacent, |input| {
        let request = WalFrameIntegrityInspectionRequest::from_admitted_wal_frame(input).unwrap();
        report = Some(
            WalFrameIntegrityAuthority::s3()
                .inspect_checkpoint_adjacent(request)
                .unwrap(),
        );
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
    with_wal_payload_input(b"WALF|crc32c|4|ok|DATA", adjacency, run);
}

fn with_wal_payload_input(
    payload: &[u8],
    adjacency: CheckpointAdjacencyPosture,
    run: impl FnOnce(ScopedPhysicalValidatorInput<'_>),
) {
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

fn quarantine_summary() -> QuarantineSummary {
    let (record, receipt, damage) = recovery_blocking_quarantine_binding();
    QuarantineSummary::from_recovery_blocking_damage(&record, receipt, &damage).unwrap()
}

pub(crate) fn recovery_blocking_quarantine_binding() -> (
    QuarantineRecord,
    RecoveryIntegrityHandoffReceipt,
    RecoveryBlockedByIntegrityDamage,
) {
    let wal_damage = inspect_wal_damage(CheckpointAdjacencyPosture::NotCheckpointAdjacent);
    let record = PhysicalQuarantineAuthority::seal(QuarantineSealRequest::from_executed_finding(
        ExecutedQuarantineFinding::from_wal_frame_denial(&wal_damage).unwrap(),
    ))
    .unwrap();
    let evidence = PhysicalIntegrityEvidenceAuthority::store_local()
        .materialize(
            StoreExecutedIntegrityEvidence::receipt_evidence(&record),
            PhysicalIntegrityEvidenceProfile::reduced(),
        )
        .unwrap();
    let receipt =
        RecoveryIntegrityHandoffReceipt::from_quarantine_receipt_evidence(&evidence).unwrap();
    let damage = RecoveryBlockedByIntegrityDamage::damaged_wal_frame(&wal_damage);
    (record, receipt, damage)
}

fn inspect_wal_damage(
    adjacency: CheckpointAdjacencyPosture,
) -> forge_store_physical_integrity::WalFrameDamageDenial {
    let mut denial = None;
    with_wal_payload_input(b"WALF|crc32c|4|checksum-fail|DATA", adjacency, |input| {
        let request = WalFrameIntegrityInspectionRequest::from_admitted_wal_frame(input).unwrap();
        denial = Some(
            WalFrameIntegrityAuthority::s3()
                .inspect(request)
                .unwrap_err(),
        );
    });
    denial.unwrap()
}

pub(crate) fn unresolved_authority_record() -> QuarantineRecord {
    let finding =
        ExecutedQuarantineFinding::from_index_page_denial(&inspect_with_damaged_authority())
            .unwrap();
    PhysicalQuarantineAuthority::seal(QuarantineSealRequest::from_executed_finding(finding))
        .unwrap()
}

fn receipt(source: StoreExecutedIntegrityEvidence<'_>) -> RecoveryIntegrityHandoffReceipt {
    let evidence = PhysicalIntegrityEvidenceAuthority::store_local()
        .materialize(source, PhysicalIntegrityEvidenceProfile::reduced())
        .unwrap();
    RecoveryIntegrityHandoffReceipt::from_executed_evidence(&evidence).unwrap()
}
