use forge_store_certification::certify_s6_later_readiness_handoffs;
use forge_store_io_scheduler::{
    admit_security_scope_for_scheduler,
    admit_secure_frame_backend_capability_for_scheduler_claim, admit_secure_io_scope_for_scheduler,
    admit_store_published_isolation_capability,
    background_pacing_outcome_for_later_readiness_certification_test,
    publish_s10_backup_export_io_readiness_handoff, publish_s10_compaction_io_readiness_handoff,
    publish_s10_repair_scan_io_readiness_handoff, publish_s11_operator_io_readiness_handoff,
    publish_s7_placement_io_readiness_handoff, BackgroundIoPressureClass,
    IoSchedulerBackendCapabilityAdmission, IoSchedulerIsolationAdmission,
    IoSchedulerSecurityScopeAdmission, S10BackupExportPacingEvidence,
    S10CompactionPacingEvidence, S10RepairScanPacingEvidence, SchedulerSecurityScopeEvidence,
    SecureIoOperation, SecureIoPosture, SecureIoPostureRequirement, SecureIoPreservationReceipt,
    SecureIoPreservationRequest,
};
use forge_store_physical_backend::{
    BackendCapabilityAdmissionRequest, BackendCapabilityEvidenceBasis, BackendCapabilitySupportSet,
    BackendMediaAssumptionSet, BackendRebindTriggers, BackendTargetProfile,
    PhysicalBackendCapabilityAdmissionAuthority,
};
use forge_store_physical_isolation::publish_scheduler_isolation_capability_for_certification_test;
use forge_store_readiness::{
    accept_s5_1_admitted_security_scope_readiness, S10BackupExportReadinessNonClaim,
    S10CompactionReadinessNonClaim, S10RepairScanReadinessNonClaim, S11OperatorReadinessNonClaim,
    S51SecurityScopeReadinessReservation, S6LaterMilestoneDestination,
    S7PlacementReadinessNonClaim,
};
use forge_store_security::admitted_store_internal_security_scope_for_s6_test;

#[test]
fn certification_preserves_distinct_later_readiness_handoff_evidence() {
    let readiness = scheduler_readiness();
    let security = scheduler_security_scope();
    let backend = secure_frame_backend_admission(&security);
    let placement = publish_s7_placement_io_readiness_handoff(&readiness);
    let compaction = publish_s10_compaction_io_readiness_handoff(
        &readiness,
        S10CompactionPacingEvidence::from_background_pacing(
            background_pacing_outcome_for_later_readiness_certification_test(
                BackgroundIoPressureClass::CompactionRewrite,
            ),
        )
        .unwrap(),
    );
    let backup = publish_s10_backup_export_io_readiness_handoff(
        &readiness,
        S10BackupExportPacingEvidence::from_background_pacing(
            background_pacing_outcome_for_later_readiness_certification_test(
                BackgroundIoPressureClass::BackupPrepRead,
            ),
        )
        .unwrap(),
    );
    let repair = publish_s10_repair_scan_io_readiness_handoff(
        &readiness,
        S10RepairScanPacingEvidence::from_background_pacing(
            background_pacing_outcome_for_later_readiness_certification_test(
                BackgroundIoPressureClass::RepairScan,
            ),
        )
        .unwrap(),
    );
    let operator = publish_s11_operator_io_readiness_handoff(
        &readiness,
        &security,
        secure_io_receipt_with_posture(
            &security,
            &backend,
            SecureIoOperation::VerificationPressure,
            SecureIoPostureRequirement::SecureFrameCompatible,
        ),
    )
    .unwrap();

    let certification =
        certify_s6_later_readiness_handoffs(&placement, &compaction, &backup, &repair, &operator);

    assert_eq!(
        certification.placement().destination(),
        S6LaterMilestoneDestination::S7Placement
    );
    assert_eq!(
        certification.placement().non_claims(),
        &[
            S7PlacementReadinessNonClaim::BlobLifecycleCorrectness,
            S7PlacementReadinessNonClaim::ChunkDedupeCorrectness,
            S7PlacementReadinessNonClaim::PlacementPolicyCorrectness,
        ]
    );
    assert_eq!(
        certification.compaction().destination(),
        S6LaterMilestoneDestination::S10Compaction
    );
    assert_eq!(
        certification.compaction().non_claims(),
        &[
            S10CompactionReadinessNonClaim::CompactionProductCorrectness,
            S10CompactionReadinessNonClaim::ForensicCorrectness,
            S10CompactionReadinessNonClaim::PlacementCorrectness,
        ]
    );
    assert_eq!(certification.compaction().compaction_pressure_units(), 1);
    assert_eq!(
        certification.backup_export().destination(),
        S6LaterMilestoneDestination::S10BackupExport
    );
    assert_eq!(
        certification.backup_export().non_claims(),
        &[
            S10BackupExportReadinessNonClaim::BackupRestoreCorrectness,
            S10BackupExportReadinessNonClaim::ExportFormatCorrectness,
            S10BackupExportReadinessNonClaim::PointInTimeRecoveryCorrectness,
        ]
    );
    assert_eq!(certification.backup_export().backup_pressure_units(), 1);
    assert_eq!(
        certification.repair_scan().destination(),
        S6LaterMilestoneDestination::S10RepairScan
    );
    assert_eq!(
        certification.repair_scan().non_claims(),
        &[
            S10RepairScanReadinessNonClaim::RepairOperatorAuthorization,
            S10RepairScanReadinessNonClaim::RepairPlanCorrectness,
            S10RepairScanReadinessNonClaim::ForensicCorrectness,
        ]
    );
    assert_eq!(certification.repair_scan().repair_pressure_units(), 1);
    assert_eq!(
        certification.operator().destination(),
        S6LaterMilestoneDestination::S11OperatorReadiness
    );
    assert_eq!(
        certification.operator().non_claims(),
        &[
            S11OperatorReadinessNonClaim::EncryptionAlgorithm,
            S11OperatorReadinessNonClaim::KeyRotation,
            S11OperatorReadinessNonClaim::AuditCorrectness,
            S11OperatorReadinessNonClaim::OperatorAuthorization,
        ]
    );
    assert_eq!(
        certification.operator().backend_requirement(),
        backend.requirement()
    );
    assert_eq!(
        certification.operator().backend_profile(),
        backend.profile()
    );
    assert_eq!(
        certification.operator().backend_evidence_class(),
        backend.evidence_class()
    );
    assert_eq!(
        certification.operator().secure_io_posture(),
        SecureIoPosture::SecureFrameCompatible
    );
}

fn scheduler_readiness() -> IoSchedulerIsolationAdmission {
    let readiness = publish_scheduler_isolation_capability_for_certification_test(2, 1)
        .expect("S.5 closeout should publish S.6 readiness");
    admit_store_published_isolation_capability(&readiness)
        .expect("scheduler should admit Store-published S.6 readiness")
}

fn scheduler_security_scope() -> IoSchedulerSecurityScopeAdmission {
    let readiness = accept_s5_1_admitted_security_scope_readiness(
        S51SecurityScopeReadinessReservation::io_qos(),
        admitted_store_internal_security_scope_for_s6_test(),
    );
    let handoff = SchedulerSecurityScopeEvidence::from_s5_1_readiness(readiness)
        .expect("S.5.1 handoff should admit");
    admit_security_scope_for_scheduler(handoff)
}

fn secure_io_receipt_with_posture(
    security: &IoSchedulerSecurityScopeAdmission,
    backend: &IoSchedulerBackendCapabilityAdmission,
    operation: SecureIoOperation,
    posture: SecureIoPostureRequirement,
) -> SecureIoPreservationReceipt {
    admit_secure_io_scope_for_scheduler(
        SecureIoPreservationRequest::new(operation, security, backend).require_posture(posture),
    )
    .expect("secure I/O scope should admit")
}

fn secure_frame_backend_admission(
    security: &IoSchedulerSecurityScopeAdmission,
) -> IoSchedulerBackendCapabilityAdmission {
    let request = BackendCapabilityAdmissionRequest::new(
        BackendTargetProfile::PosixFileFsyncDirSync,
        BackendCapabilityEvidenceBasis::externally_guaranteed(1),
        BackendCapabilitySupportSet::all_supported(),
        BackendMediaAssumptionSet::platform_file_defaults()
            .with_direct_io_alignment()
            .with_sector_atomicity()
            .with_page_cache_policy()
            .with_mmap_coherence()
            .with_async_ordering()
            .with_secure_frame_io()
            .with_flush_ordering(),
        BackendRebindTriggers::kernel_filesystem_mount_firmware_and_backend(),
    );
    let witness = PhysicalBackendCapabilityAdmissionAuthority::store_owned()
        .admit_backend_capability(request)
        .expect("backend should admit");
    admit_secure_frame_backend_capability_for_scheduler_claim(&witness, security)
        .expect("scheduler backend should admit")
}
