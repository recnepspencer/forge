//! Destination rows for recovery-integrity surfaces moved out of recovery physics.

const PHYSICAL_READMISSION: &str = "worth-store-physical-integrity/recovery-readmission";
const PHYSICAL_HANDOFF: &str = "worth-store-physical-integrity/recovery-handoff";
const PHYSICAL_DAMAGE: &str = "worth-store-physical-integrity/damage-classification";
const LAYOUT_READMISSION: &str = "worth-store-layout-indexes/integrity-readmission";
const PHYSICAL_RECORDS: &str = "worth-store-physical-integrity/recovery-records";
const PHYSICAL_BLOCKING: &str = "worth-store-physical-integrity/recovery-blocking";
const PHASE_EIGHT: &str = "phase-8";

const MOVED_INTEGRITY_OWNERS: [&str; 6] = [
    PHYSICAL_READMISSION,
    PHYSICAL_HANDOFF,
    PHYSICAL_DAMAGE,
    LAYOUT_READMISSION,
    PHYSICAL_RECORDS,
    PHYSICAL_BLOCKING,
];

macro_rules! moved_integrity_destination_rows {
    ($(($surface:literal, $owner_key:expr)),* $(,)?) => {
        &[
            $(($surface, MOVED_INTEGRITY_OWNERS[$owner_key], PHASE_EIGHT)),*
        ]
    };
}

pub(super) const MOVED_INTEGRITY_DESTINATION_SURFACES: &[(&str, &str, &str)] = moved_integrity_destination_rows![
    ("admit_recovery_corruption_readmission", 0),
    ("AdmittedRecoveryIntegrityInput", 1),
    ("AdmittedRecoveryIntegrityInput::claims_recovery", 1),
    ("AdmittedRecoveryIntegrityInput::counters", 1),
    ("AdmittedRecoveryIntegrityInput::payload", 1),
    (
        "AdmittedRecoveryIntegrityInput::proves_no_raw_bytes_crossed",
        1
    ),
    ("BoundedInspectionEnvelopeEvidence", 1),
    ("BoundedInspectionEnvelopeEvidence::checksum_basis", 1),
    ("BoundedInspectionEnvelopeEvidence::counters", 1),
    ("BoundedInspectionEnvelopeEvidence::from_checked_frame", 1),
    ("BoundedInspectionEnvelopeEvidence::from_checked_page", 1),
    ("BoundedInspectionEnvelopeEvidence::protected_read_limit", 1),
    ("BoundedInspectionEnvelopeEvidence::resident_byte_limit", 1),
    (
        "BoundedInspectionEnvelopeEvidence::streaming_window_limit",
        1
    ),
    ("ChecksumAlgorithmScopeBasis", 1),
    ("ChecksumAlgorithmScopeBasis::algorithm", 1),
    ("ChecksumAlgorithmScopeBasis::from_checksum_declaration", 1),
    ("ChecksumAlgorithmScopeBasis::scope", 1),
    ("classify_recovery_blocking_damage", 2),
    ("classify_recovery_repair_capability", 0),
    ("damage_map", 1),
    ("ImportLayoutReadmissionOutcome", 3),
    ("IntegrityDamageMap", 2),
    ("IntegrityDamageMap::admit_corruption_readmission", 2),
    ("IntegrityDamageMap::basis", 2),
    ("IntegrityDamageMap::checkpoint_damage", 2),
    ("IntegrityDamageMap::manifest_root_damage", 2),
    ("IntegrityDamageMap::new", 2),
    ("IntegrityDamageMap::quarantine_summaries", 2),
    ("IntegrityDamageMap::recovery_blocking_damage_cases", 2),
    ("IntegrityDamageMap::recovery_blocking_findings", 2),
    ("IntegrityDamageMap::unresolved_authority_damage", 2),
    ("IntegrityDamageMap::wal_damage", 2),
    ("IntegrityDamageMap::with_checkpoint_damage", 2),
    ("IntegrityDamageMap::with_manifest_root_damage", 2),
    ("IntegrityDamageMap::with_recovery_blocking_quarantine", 2),
    ("IntegrityDamageMap::with_unresolved_authority_damage", 2),
    ("IntegrityDamageMap::with_wal_damage", 2),
    ("IntegrityHandoffAdmission", 1),
    ("IntegrityHandoffAdmission::admit_model_payload", 1),
    ("IntegrityHandoffCounters", 1),
    ("IntegrityHandoffCounters::checked_byte_count", 1),
    ("IntegrityHandoffCounters::checksum_execution_count", 1),
    ("IntegrityHandoffCounters::quarantine_summary_count", 1),
    ("IntegrityHandoffCounters::recovery_blocking_count", 1),
    ("IntegrityHandoffCounters::skipped_decode_count", 1),
    ("IntegrityHandoffCounters::vetted_record_count", 1),
    ("IntegrityHandoffDeclaration", 1),
    ("IntegrityHandoffDeclaration::checkpoint_record", 1),
    ("IntegrityHandoffDeclaration::damage_map", 1),
    ("IntegrityHandoffDeclaration::inspection_envelope", 1),
    ("IntegrityHandoffDeclaration::page_frame", 1),
    ("IntegrityHandoffDeclaration::root_manifest", 1),
    ("IntegrityHandoffDeclaration::seal", 1),
    ("IntegrityHandoffDeclaration::segment_manifest", 1),
    ("IntegrityHandoffDeclaration::wal_frame", 1),
    ("IntegrityHandoffDenial", 1),
    ("IntegrityHandoffDenial::kind", 1),
    ("IntegrityHandoffDenial::new", 1),
    ("IntegrityHandoffDenialKind", 1),
    ("IntegrityHandoffPayload", 1),
    ("IntegrityHandoffPayload::checkpoint_records", 1),
    ("IntegrityHandoffPayload::checksum_basis", 1),
    ("IntegrityHandoffPayload::claims_recovery", 1),
    (
        "IntegrityHandoffPayload::corruption_readmission_handoffs",
        1
    ),
    ("IntegrityHandoffPayload::counters", 1),
    ("IntegrityHandoffPayload::damage_map", 1),
    ("IntegrityHandoffPayload::declare", 1),
    ("IntegrityHandoffPayload::identity", 1),
    ("IntegrityHandoffPayload::inspection_envelope", 1),
    ("IntegrityHandoffPayload::page_frames", 1),
    ("IntegrityHandoffPayload::proves_no_raw_bytes_crossed", 1),
    ("IntegrityHandoffPayload::root_manifest", 1),
    ("IntegrityHandoffPayload::segment_manifest", 1),
    ("IntegrityHandoffPayload::wal_frames", 1),
    ("IntegrityVettedCheckpointRecord", 4),
    ("IntegrityVettedCheckpointRecord::counters", 4),
    ("IntegrityVettedCheckpointRecord::from_integrity_report", 4),
    ("IntegrityVettedCheckpointRecord::input_identity", 4),
    ("IntegrityVettedCheckpointRecord::receipt", 4),
    ("IntegrityVettedCheckpointRecord::tail_posture", 4),
    ("IntegrityVettedPageFrameKind", 4),
    ("IntegrityVettedPageFrameRecord", 4),
    ("IntegrityVettedPageFrameRecord::boundary", 4),
    ("IntegrityVettedPageFrameRecord::counters", 4),
    ("IntegrityVettedPageFrameRecord::from_frame_report", 4),
    ("IntegrityVettedPageFrameRecord::from_page_report", 4),
    ("IntegrityVettedPageFrameRecord::kind", 4),
    ("IntegrityVettedPageFrameRecord::receipt", 4),
    ("IntegrityVettedPageFrameRecord::scope", 4),
    ("IntegrityVettedRootManifestRecord", 4),
    ("IntegrityVettedRootManifestRecord::counters", 4),
    ("IntegrityVettedRootManifestRecord::from_manifest_report", 4),
    ("IntegrityVettedRootManifestRecord::posture", 4),
    ("IntegrityVettedRootManifestRecord::receipt", 4),
    ("IntegrityVettedRootManifestRecord::root_owner", 4),
    ("IntegrityVettedSegmentManifestRecord", 4),
    ("IntegrityVettedSegmentManifestRecord::counters", 4),
    (
        "IntegrityVettedSegmentManifestRecord::from_manifest_report",
        4
    ),
    ("IntegrityVettedSegmentManifestRecord::receipt", 4),
    ("IntegrityVettedSegmentManifestRecord::segment", 4),
    ("IntegrityVettedWalFrame", 4),
    ("IntegrityVettedWalFrame::counters", 4),
    ("IntegrityVettedWalFrame::from_integrity_report", 4),
    ("IntegrityVettedWalFrame::input_identity", 4),
    ("IntegrityVettedWalFrame::receipt", 4),
    ("IntegrityVettedWalFrame::tail_posture", 4),
    ("layout_readmission", 3),
    ("LayoutReadmissionAuthority", 3),
    ("LayoutReadmissionAuthority::admit_import", 3),
    ("LayoutReadmissionAuthority::admit_quarantine", 3),
    ("QuarantineLayoutReadmissionOutcome", 3),
    ("QuarantineSummary", 2),
    ("QuarantineSummary::damage_case", 2),
    ("QuarantineSummary::from_recovery_blocking_damage", 2),
    ("QuarantineSummary::handoff_posture", 2),
    ("QuarantineSummary::locality", 2),
    ("QuarantineSummary::receipt", 2),
    ("RawBytesExcludedFromRecoveryHandoff", 1),
    ("RecoveryBlockedByIntegrityDamage", 5),
    ("RecoveryBlockedByIntegrityDamage::basis", 5),
    (
        "RecoveryBlockedByIntegrityDamage::checkpoint_adjacent_damage",
        5
    ),
    ("RecoveryBlockedByIntegrityDamage::damaged_manifest_root", 5),
    ("RecoveryBlockedByIntegrityDamage::damaged_wal_frame", 5),
    ("RecoveryBlockedByIntegrityDamage::locality", 5),
    ("RecoveryBlockedByIntegrityDamage::manifest_kind", 5),
    ("RecoveryBlockedByIntegrityDamage::root_posture", 5),
    ("RecoveryBlockedByIntegrityDamage::source", 5),
    ("RecoveryBlockedByIntegrityDamage::tail_posture", 5),
    (
        "RecoveryBlockedByIntegrityDamage::unresolved_authority_damage",
        5
    ),
    ("RecoveryBlockedByIntegrityDamage::wal_kind", 5),
    ("RecoveryBlockingIntegritySource", 5),
    ("RecoveryCorruptionReadmissionDenial", 0),
    ("RecoveryCorruptionReadmissionHandoff", 0),
    (
        "RecoveryCorruptionReadmissionHandoff::primary_damage_case",
        0
    ),
    ("RecoveryCorruptionReadmissionHandoff::repair_capability", 0),
    ("RecoveryCorruptionRepairCapability", 0),
    ("RecoveryIntegrityHandoffReceipt", 1),
    ("RecoveryIntegrityHandoffReceipt::basis", 1),
    ("RecoveryIntegrityHandoffReceipt::category", 1),
    ("RecoveryIntegrityHandoffReceipt::counters", 1),
    ("RecoveryIntegrityHandoffReceipt::from_executed_evidence", 1),
    (
        "RecoveryIntegrityHandoffReceipt::from_quarantine_receipt_evidence",
        1
    ),
    ("RecoveryIntegrityHandoffReceipt::locality", 1),
    ("RecoveryIntegrityHandoffReceipt::outcome", 1),
    (
        "RecoveryIntegrityHandoffReceipt::physical_authority_basis",
        1
    ),
    ("RecoveryIntegrityHandoffReceipt::role", 1),
    ("RecoveryLayoutReadmissionAdmissionDenial", 3),
    ("RecoveryLayoutReadmissionClass", 3),
    ("RecoveryLayoutReadmissionIdentity", 3),
    ("RecoveryLayoutReadmissionOutcomeView", 3),
    ("RecoveryLayoutReadmissionWitness", 3),
    ("RecoveryLayoutReadmissionWitness::class", 3),
    ("RecoveryLayoutReadmissionWitness::family_id", 3),
    ("RecoveryLayoutReadmissionWitness::identity", 3),
    (
        "RecoveryLayoutReadmissionWitness::source_security_scope_identity",
        3
    ),
    (
        "RecoveryLayoutReadmissionWitness::source_store_authority_identity",
        3
    ),
    ("RecoveryPhysicsIntegrityInput", 1),
    ("RecoveryPhysicsIntegrityInput::from_vetted_wal_frame", 1),
    ("RecoveryPhysicsIntegrityInput::tail_posture", 1),
    ("RecoveryPhysicsIntegrityInput::wal_identity", 1),
    ("verify_quarantine_handoff_for_readmission", 0),
    ("verify_store_authority_for_readmission", 0),
];
