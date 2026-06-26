use crate::{
    derived_index_damage_tests::inspect_with_damaged_authority,
    physical_container_integrity_test_support::{inspect_page_report, page_payload_with_record},
    physical_scope_admission_test_support::{
        page_cell, page_slot_admission, root_admission, root_with_slot, scope_membership,
        validation, with_checked_frame, with_checked_page,
    },
    pre_decode_physical_admission_test_support::checksum_scope,
};
use forge_store_physical_format::{
    CheckpointAdjacencyPosture, PhysicalReferenceScope, RootManifestIntegrityPosture,
};
use forge_store_physical_integrity::{
    ChecksumAlgorithmId, ExecutedQuarantineFinding, ManifestExpectedReference,
    ManifestIntegrityAuthority, ManifestIntegrityInspectionRequest,
    PhysicalIntegrityEvidenceAuthority, PhysicalIntegrityEvidenceProfile,
    PhysicalQuarantineAuthority, PhysicalScopeAdmission, PhysicalScopeAdmissionRequest,
    QuarantineRecord, QuarantineSealRequest, ScopedPhysicalValidatorInput,
    StoreExecutedIntegrityEvidence, WalFrameDamageDenialKind, WalFrameIntegrityAuthority,
    WalFrameIntegrityInspectionRequest,
};
use forge_store_recovery_physics::{
    BoundedInspectionEnvelopeEvidence, IntegrityDamageMap, IntegrityVettedCheckpointRecord,
    IntegrityVettedPageFrameRecord, IntegrityVettedRootManifestRecord,
    IntegrityVettedSegmentManifestRecord, IntegrityVettedWalFrame, QuarantineSummary,
    RecoveryBlockedByIntegrityDamage, RecoveryBlockingIntegritySource,
    RecoveryIntegrityHandoffReceipt, S4IntegrityHandoffDenialKind, S4IntegrityHandoffPayload,
    S4RecoveryPhysicsIntegrityReadiness,
};

#[test]
fn intact_inputs_publish_stable_s4_handoff_identity_across_independent_runs() {
    let first = intact_readiness("stable-handoff");
    let second = intact_readiness("stable-handoff");

    assert_eq!(first.payload().identity(), second.payload().identity());
    assert_eq!(first.counters(), second.counters());
    assert_eq!(
        first.payload().root_manifest(),
        second.payload().root_manifest()
    );
    assert_eq!(
        first.payload().segment_manifest(),
        second.payload().segment_manifest()
    );
    assert_eq!(first.payload().wal_frames(), second.payload().wal_frames());
    assert_eq!(
        first.payload().checkpoint_records(),
        second.payload().checkpoint_records()
    );
    assert!(first.proves_no_raw_bytes_crossed());
    assert!(!first.claims_recovery());
}

#[test]
fn damaged_inputs_publish_typed_recovery_blockers_instead_of_replay_inputs() {
    let wal_damage = inspect_wal_denial(
        wal_payload("crc32c", 4, "checksum-fail", b"DATA"),
        CheckpointAdjacencyPosture::NotCheckpointAdjacent,
    );
    let checkpoint_damage = inspect_wal_denial(
        wal_payload("crc32c", 4, "checkpoint-damage", b"DATA"),
        CheckpointAdjacencyPosture::CheckpointAdjacent,
    );
    let manifest_damage = ManifestIntegrityAuthority::s3()
        .inspect_manifest(ManifestIntegrityInspectionRequest::damaged_root(
            root_with_slot(1, 2, 3, 7).root_publication().owner(),
        ))
        .unwrap_err();

    let damage_map = IntegrityDamageMap::new()
        .with_wal_damage(RecoveryBlockedByIntegrityDamage::damaged_wal_frame(
            &wal_damage,
        ))
        .unwrap()
        .with_checkpoint_damage(
            RecoveryBlockedByIntegrityDamage::checkpoint_adjacent_damage(&checkpoint_damage),
        )
        .unwrap()
        .with_manifest_root_damage(RecoveryBlockedByIntegrityDamage::damaged_manifest_root(
            &manifest_damage,
        ))
        .unwrap()
        .with_unresolved_authority_damage(
            RecoveryBlockedByIntegrityDamage::unresolved_authority_damage(
                &unresolved_authority_record(),
            )
            .unwrap(),
        )
        .unwrap();

    assert_eq!(wal_damage.kind(), WalFrameDamageDenialKind::ChecksumFailure);
    assert_eq!(
        checkpoint_damage.kind(),
        WalFrameDamageDenialKind::CheckpointAdjacentCorruption
    );
    assert_eq!(
        damage_map.wal_damage()[0].source(),
        RecoveryBlockingIntegritySource::WalFrame
    );
    assert_eq!(
        damage_map.checkpoint_damage()[0].source(),
        RecoveryBlockingIntegritySource::CheckpointAdjacentRecord
    );
    assert_eq!(
        damage_map.manifest_root_damage()[0].source(),
        RecoveryBlockingIntegritySource::ManifestRoot
    );
    assert_eq!(
        damage_map.unresolved_authority_damage()[0].source(),
        RecoveryBlockingIntegritySource::UnresolvedAuthorityDamage
    );
    assert_eq!(damage_map.recovery_blocking_findings().len(), 4);
}

#[test]
fn s4_handoff_payload_exposes_required_integrity_surfaces_and_exact_counters() {
    let readiness = intact_readiness("payload-proof");
    let payload = readiness.payload();
    let counters = payload.counters();

    assert_eq!(payload.root_manifest().counters().root_manifest_reads(), 1);
    assert_eq!(
        payload
            .segment_manifest()
            .counters()
            .segment_manifest_reads(),
        1
    );
    assert_eq!(payload.page_frames().len(), 1);
    assert_eq!(payload.wal_frames().len(), 1);
    assert_eq!(payload.checkpoint_records().len(), 1);
    assert_eq!(payload.damage_map().quarantine_summaries().len(), 1);
    assert_eq!(
        payload.checksum_basis().algorithm(),
        ChecksumAlgorithmId::crc32c()
    );
    assert_eq!(payload.checksum_basis().scope(), &checksum_scope());
    assert_eq!(payload.inspection_envelope().resident_byte_limit(), 128);
    assert_eq!(payload.inspection_envelope().protected_read_limit(), 128);
    assert_eq!(payload.inspection_envelope().streaming_window_limit(), 128);
    assert_eq!(counters.vetted_record_count(), 5);
    assert_eq!(counters.quarantine_summary_count(), 1);
    assert_eq!(counters.recovery_blocking_count(), 0);
    assert_eq!(
        counters.checked_byte_count(),
        payload
            .inspection_envelope()
            .counters()
            .checked_byte_count()
    );
    assert_eq!(counters.checksum_execution_count(), 1);
    assert_eq!(counters.skipped_decode_count(), 0);
    assert!(payload.proves_no_raw_bytes_crossed());
    assert!(!payload.claims_recovery());
}

#[test]
fn handoff_rejects_manifest_receipt_swaps_and_forged_envelope_counters() {
    let first_manifest = inspect_manifest_for_generation(7);
    let second_manifest = inspect_manifest_for_generation(8);
    let first_receipt = receipt(StoreExecutedIntegrityEvidence::authoritative_manifest(
        &first_manifest,
    ));
    let denial =
        IntegrityVettedRootManifestRecord::from_manifest_report(&second_manifest, first_receipt)
            .unwrap_err();
    assert_eq!(
        denial.kind(),
        S4IntegrityHandoffDenialKind::ReceiptBasisMismatch
    );

    let payload = page_payload_with_record(b"forged-envelope");
    with_checked_page(&payload, page_cell(1, 2, 7), |checked| {
        let checked_bytes = checked.counters().checked_byte_count();
        let denial = BoundedInspectionEnvelopeEvidence::from_checked_page(
            &checked,
            checked_bytes - 1,
            checked_bytes,
            checked_bytes,
        )
        .unwrap_err();
        assert_eq!(
            denial.kind(),
            S4IntegrityHandoffDenialKind::InspectionEnvelopeExceeded
        );
    });
}

pub(crate) fn intact_readiness(label: &str) -> S4RecoveryPhysicsIntegrityReadiness {
    let page_payload = page_payload_with_record(label.as_bytes());
    let page = inspect_page_report(&page_payload);
    let wal = inspect_wal_frame(CheckpointAdjacencyPosture::NotCheckpointAdjacent);
    let checkpoint = inspect_checkpoint_record();
    let manifest = inspect_manifest_for_generation(7);
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
    let damage_map = IntegrityDamageMap::new().with_quarantine_summary(quarantine_summary(&page));
    let payload = S4IntegrityHandoffPayload::declare()
        .root_manifest(root)
        .segment_manifest(segment)
        .page_frame(page_record)
        .wal_frame(wal_record)
        .checkpoint_record(checkpoint_record)
        .damage_map(damage_map)
        .inspection_envelope(inspection_envelope_for_payload(&page_payload))
        .seal()
        .unwrap();
    S4RecoveryPhysicsIntegrityReadiness::from_s3_integrity_handoff(payload)
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

fn inspect_wal_denial(
    payload: Vec<u8>,
    adjacency: CheckpointAdjacencyPosture,
) -> forge_store_physical_integrity::WalFrameDamageDenial {
    let mut denial = None;
    with_wal_payload_input(payload, adjacency, |input| {
        let request = WalFrameIntegrityInspectionRequest::from_admitted_wal_frame(input).unwrap();
        denial = Some(
            WalFrameIntegrityAuthority::s3()
                .inspect(request)
                .unwrap_err(),
        );
    });
    denial.unwrap()
}

fn with_wal_frame_input(
    adjacency: CheckpointAdjacencyPosture,
    run: impl FnOnce(ScopedPhysicalValidatorInput<'_>),
) {
    with_wal_payload_input(wal_payload("crc32c", 4, "ok", b"DATA"), adjacency, run);
}

fn with_wal_payload_input(
    payload: Vec<u8>,
    adjacency: CheckpointAdjacencyPosture,
    run: impl FnOnce(ScopedPhysicalValidatorInput<'_>),
) {
    let validation = validation(1, 2, 3, 7);
    let scope = PhysicalReferenceScope::wal_frame(validation);
    let root = root_with_slot(1, 2, 3, 7);
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

fn quarantine_summary(
    page: &forge_store_physical_integrity::PageIntegrityReport,
) -> QuarantineSummary {
    let record = PhysicalQuarantineAuthority::seal(QuarantineSealRequest::from_executed_finding(
        ExecutedQuarantineFinding::intact_page(page),
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
    QuarantineSummary::from_quarantine_record(&record, receipt).unwrap()
}

fn unresolved_authority_record() -> QuarantineRecord {
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

fn wal_payload(algorithm: &str, declared_len: usize, status: &str, body: &[u8]) -> Vec<u8> {
    let mut payload = format!("WALF|{algorithm}|{declared_len}|{status}|").into_bytes();
    payload.extend_from_slice(body);
    payload
}
