use forge_store_physical_format::{
    CheckpointAdjacencyPosture, ChecksumCoverageMap, ManifestMembershipProof, PageGenerationCell,
    PhysicalFrameKind, PhysicalPageKind, PhysicalReferenceAuthority, PhysicalReferenceScope,
    PhysicalReferenceValidationWitness, RootManifestIntegrityPosture,
};
use forge_store_physical_integrity::{
    ChecksumAlgorithmDeclaration, ChecksumAlgorithmId, ChecksumScopeDeclaration,
    DeclaredPhysicalChecksum, ExecutedQuarantineFinding, IntegrityEntryAdmission,
    IntegrityEntryRequest, ManifestExpectedReference, ManifestIntegrityAuthority,
    ManifestIntegrityInspectionRequest, PhysicalContainerIntegrity, PhysicalIntegrityAdmission,
    PhysicalIntegrityAdmissionRequest, PhysicalIntegrityAdmissionSeed,
    PhysicalIntegrityEvidenceAuthority, PhysicalIntegrityEvidenceProfile,
    PhysicalQuarantineAuthority, PhysicalScopeAdmission, PhysicalScopeAdmissionRequest,
    QuarantineSealRequest, ScopedPhysicalValidatorInput, StoreExecutedIntegrityEvidence,
    WalFrameIntegrityAuthority, WalFrameIntegrityInspectionRequest,
};
use forge_store_recovery_physics::{
    BoundedInspectionEnvelopeEvidence, QuarantineSummary, RecoveryBlockedByIntegrityDamage,
    RecoveryIntegrityHandoffReceipt,
};

use super::s4_recovery_physical_fixture::{
    frame_witness, page_cell, page_witness, root_with_slot, slot_cell, validation,
    with_protected_payload_view,
};
use super::s4_recovery_readiness_fixture::physical_integrity_readiness;

pub(super) fn inspect_page_report(
    payload: &[u8],
) -> forge_store_physical_integrity::PageIntegrityReport {
    let mut report = None;
    with_checked_page(payload, page_cell(1, 2, 7), |checked| {
        let scope = PhysicalReferenceScope::page(page_cell(1, 2, 7));
        let root = root_with_slot(1, 2, 3, 7);
        let membership = ManifestMembershipProof::from_root(&root, scope).unwrap();
        let request = PhysicalScopeAdmissionRequest::page(
            scope,
            membership,
            RootManifestIntegrityPosture::current_root_admitted(membership),
            checked.gate_evidence().coverage_basis().clone(),
        );
        let admission = PhysicalScopeAdmission::admit_page(checked, request).unwrap();
        report = Some(
            PhysicalContainerIntegrity::inspect_page(
                ScopedPhysicalValidatorInput::page(admission).unwrap(),
            )
            .unwrap(),
        );
    });
    report.unwrap()
}

pub(super) fn inspect_manifest() -> forge_store_physical_integrity::ManifestIntegrityReport {
    let root = root_with_slot(1, 2, 3, 7);
    ManifestIntegrityAuthority::new()
        .inspect_manifest(
            ManifestIntegrityInspectionRequest::from_root_publication(
                root.clone(),
                PhysicalReferenceAuthority::for_canonical_physical_format().admit_root_publication(root.root_publication()),
            )
            .with_expected_reference(ManifestExpectedReference::page_slot(
                PhysicalReferenceAuthority::for_canonical_physical_format().admit_page_slot(slot_cell(1, 2, 3, 7)),
            )),
        )
        .unwrap()
}

pub(super) fn inspect_checkpoint_record(
) -> forge_store_physical_integrity::CheckpointRecordIntegrityReport {
    let mut report = None;
    with_wal_input(CheckpointAdjacencyPosture::CheckpointAdjacent, |input| {
        let request = WalFrameIntegrityInspectionRequest::from_admitted_wal_frame(input).unwrap();
        report = Some(
            WalFrameIntegrityAuthority::new()
                .inspect_checkpoint_adjacent(request)
                .unwrap(),
        );
    });
    report.unwrap()
}

pub(super) fn inspect_wal_frame(
    adjacency: CheckpointAdjacencyPosture,
) -> forge_store_physical_integrity::WalFrameIntegrityReport {
    let mut report = None;
    with_wal_input(adjacency, |input| {
        let request = WalFrameIntegrityInspectionRequest::from_admitted_wal_frame(input).unwrap();
        report = Some(WalFrameIntegrityAuthority::new().inspect(request).unwrap());
    });
    report.unwrap()
}

pub(super) fn inspection_envelope(payload: &[u8]) -> BoundedInspectionEnvelopeEvidence {
    let mut envelope = None;
    with_checked_page(payload, page_cell(1, 2, 7), |checked| {
        envelope = Some(
            BoundedInspectionEnvelopeEvidence::from_checked_page(&checked, 4096, 4096, 4096)
                .unwrap(),
        );
    });
    envelope.unwrap()
}

pub(super) fn quarantine_summary() -> QuarantineSummary {
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
    QuarantineSummary::from_recovery_blocking_damage(
        &record,
        receipt,
        &RecoveryBlockedByIntegrityDamage::damaged_wal_frame(&wal_damage),
    )
    .unwrap()
}

pub(super) fn inspect_wal_damage(
    adjacency: CheckpointAdjacencyPosture,
) -> forge_store_physical_integrity::WalFrameDamageDenial {
    let mut denial = None;
    with_wal_payload_input(b"WALF|crc32c|4|checksum-fail|DATA", adjacency, |input| {
        let request = WalFrameIntegrityInspectionRequest::from_admitted_wal_frame(input).unwrap();
        denial = Some(
            WalFrameIntegrityAuthority::new()
                .inspect(request)
                .unwrap_err(),
        );
    });
    denial.unwrap()
}

pub(super) fn receipt(
    source: StoreExecutedIntegrityEvidence<'_>,
) -> RecoveryIntegrityHandoffReceipt {
    let evidence = PhysicalIntegrityEvidenceAuthority::store_local()
        .materialize(source, PhysicalIntegrityEvidenceProfile::reduced())
        .unwrap();
    RecoveryIntegrityHandoffReceipt::from_executed_evidence(&evidence).unwrap()
}

pub(super) fn with_wal_input(
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
    with_checked_frame(payload, validation, |checked| {
        let scope = PhysicalReferenceScope::wal_frame(validation);
        let root = root_with_slot(1, 2, 3, 7);
        let membership = ManifestMembershipProof::from_root(&root, scope).unwrap();
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

fn with_checked_page(
    payload: &[u8],
    cell: PageGenerationCell,
    run: impl FnOnce(forge_store_physical_integrity::IntegrityCheckedPage<'_>),
) {
    with_entry_seed(payload, |seed| {
        let admission = seed
            .with_checksum_declaration(checksum_admission(seed))
            .unwrap();
        let checked = admission
            .admit_page(PhysicalIntegrityAdmissionRequest::page(
                cell,
                page_witness(payload, cell),
                PhysicalPageKind::DataPage,
                DeclaredPhysicalChecksum::new(crc32c(payload)),
            ))
            .unwrap();
        run(checked);
    });
}

fn with_checked_frame(
    payload: &[u8],
    validation: PhysicalReferenceValidationWitness,
    run: impl FnOnce(forge_store_physical_integrity::IntegrityCheckedFrame<'_>),
) {
    with_entry_seed(payload, |seed| {
        let admission = seed
            .with_checksum_declaration(checksum_admission(seed))
            .unwrap();
        let checked = admission
            .admit_frame(PhysicalIntegrityAdmissionRequest::frame(
                validation,
                frame_witness(payload, validation),
                PhysicalFrameKind::RecordFrame,
                DeclaredPhysicalChecksum::new(crc32c(payload)),
            ))
            .unwrap();
        run(checked);
    });
}

fn checksum_admission(
    seed: PhysicalIntegrityAdmissionSeed<'_>,
) -> forge_store_physical_integrity::S3ChecksumDeclarationAdmission {
    checksum_declaration().admit_for_physical_integrity_entry(seed.entry_witness())
}

fn with_entry_seed(payload: &[u8], run: impl FnOnce(PhysicalIntegrityAdmissionSeed<'_>)) {
    with_protected_payload_view(payload, |protected| {
        let entry = IntegrityEntryAdmission::from_physical_integrity_payload(physical_integrity_readiness().payload()).unwrap();
        let lease = entry.admit(IntegrityEntryRequest::new(protected)).unwrap();
        run(PhysicalIntegrityAdmission::from_entry(lease));
    });
}

fn checksum_declaration() -> ChecksumAlgorithmDeclaration {
    ChecksumAlgorithmId::crc32c()
        .declare_for_scope(checksum_scope())
        .unwrap()
}

fn checksum_scope() -> ChecksumScopeDeclaration {
    let format = forge_store_physical_format::PhysicalFormatDeclaration::physical_format_canonical().unwrap();
    ChecksumScopeDeclaration::for_physical_format(
        format.identity(),
        ChecksumCoverageMap::physical_format_page_and_frame_crc32c().unwrap(),
    )
    .unwrap()
}

fn crc32c(bytes: &[u8]) -> u64 {
    let mut crc = !0u32;
    for byte in bytes {
        crc ^= u32::from(*byte);
        for _ in 0..8 {
            let mask = 0u32.wrapping_sub(crc & 1);
            crc = (crc >> 1) ^ (0x82f6_3b78 & mask);
        }
    }
    u64::from(!crc)
}
