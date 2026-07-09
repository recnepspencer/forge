use worth_store_readiness::{
    S10BackupExportReadinessNonClaim, S10CompactionReadinessNonClaim,
    S10RepairScanReadinessNonClaim, S11OperatorReadinessNonClaim, S6LaterMilestoneDestination,
    S7PlacementReadinessNonClaim,
};

use crate::{
    admit_secure_io_scope_for_scheduler, BackgroundIoPressureClass,
    IoSchedulerBackendCapabilityRequirement, S10BackupExportPacingEvidence,
    S10CompactionPacingEvidence, S10RepairScanPacingEvidence, SecureIoOperation, SecureIoPosture,
    SecureIoPostureRequirement, SecureIoPreservationDenial, SecureIoPreservationRequest,
};

mod test_support;
use test_support::*;

use super::{
    admit_s11_operator_io_readiness_seed, publish_s10_backup_export_io_readiness_handoff,
    publish_s10_compaction_io_readiness_handoff, publish_s10_repair_scan_io_readiness_handoff,
    publish_s11_operator_io_readiness_handoff, publish_s7_placement_io_readiness_handoff,
    readmit_s10_backup_export_io_readiness_after_publication,
    readmit_s11_operator_io_readiness_after_publication,
    readmit_s7_placement_io_readiness_after_publication,
    reject_certification_only_evidence_as_later_readiness_handoff,
    reject_raw_s6_counters_as_later_readiness_handoff, S6LaterReadinessHandoffDenial,
};

#[test]
fn placement_handoff_exposes_io_posture_and_non_claims_only() {
    let readiness = scheduler_readiness();
    let handoff = publish_s7_placement_io_readiness_handoff(&readiness);

    assert_eq!(
        handoff.destination(),
        S6LaterMilestoneDestination::S7Placement
    );
    assert_eq!(
        handoff.non_claims(),
        &[
            S7PlacementReadinessNonClaim::BlobLifecycleCorrectness,
            S7PlacementReadinessNonClaim::ChunkDedupeCorrectness,
            S7PlacementReadinessNonClaim::PlacementPolicyCorrectness,
        ]
    );
    assert_eq!(handoff.counters(), readiness.counters());
    assert_eq!(
        handoff.foreground_interference(),
        readiness.foreground_interference()
    );
}

#[test]
fn compaction_handoff_requires_executed_background_pacing_outcome() {
    let readiness = scheduler_readiness();
    let pacing = S10CompactionPacingEvidence::from_background_pacing(background_pacing_outcome(
        BackgroundIoPressureClass::CompactionRewrite,
    ))
    .unwrap();
    let handoff = publish_s10_compaction_io_readiness_handoff(&readiness, pacing);

    assert_eq!(
        handoff.destination(),
        S6LaterMilestoneDestination::S10Compaction
    );
    assert_eq!(
        handoff.non_claims(),
        &[
            S10CompactionReadinessNonClaim::CompactionProductCorrectness,
            S10CompactionReadinessNonClaim::ForensicCorrectness,
            S10CompactionReadinessNonClaim::PlacementCorrectness,
        ]
    );
    assert_eq!(handoff.background_pacing_counters().violation_events(), 1);
    assert!(!handoff
        .background_pacing_counters()
        .compaction_debt()
        .is_empty());
}

#[test]
fn backup_and_repair_handoffs_are_distinct_s10_readiness_lanes() {
    let readiness = scheduler_readiness();
    let backup_pacing = S10BackupExportPacingEvidence::from_background_pacing(
        background_pacing_outcome(BackgroundIoPressureClass::BackupPrepRead),
    )
    .unwrap();
    let repair_pacing = S10RepairScanPacingEvidence::from_background_pacing(
        background_pacing_outcome(BackgroundIoPressureClass::RepairScan),
    )
    .unwrap();

    let backup = publish_s10_backup_export_io_readiness_handoff(&readiness, backup_pacing);
    let repair = publish_s10_repair_scan_io_readiness_handoff(&readiness, repair_pacing);

    assert_eq!(
        backup.destination(),
        S6LaterMilestoneDestination::S10BackupExport
    );
    assert_eq!(
        backup.non_claims(),
        &[
            S10BackupExportReadinessNonClaim::BackupRestoreCorrectness,
            S10BackupExportReadinessNonClaim::ExportFormatCorrectness,
            S10BackupExportReadinessNonClaim::PointInTimeRecoveryCorrectness,
        ]
    );
    assert!(!backup
        .background_pacing_counters()
        .backup_pressure()
        .is_empty());
    assert_eq!(
        repair.destination(),
        S6LaterMilestoneDestination::S10RepairScan
    );
    assert_eq!(
        repair.non_claims(),
        &[
            S10RepairScanReadinessNonClaim::RepairOperatorAuthorization,
            S10RepairScanReadinessNonClaim::RepairPlanCorrectness,
            S10RepairScanReadinessNonClaim::ForensicCorrectness,
        ]
    );
    assert!(!repair
        .background_pacing_counters()
        .repair_pressure()
        .is_empty());
}

#[test]
fn s10_lane_evidence_accepts_only_its_pressure_class_before_publication() {
    for class in [
        BackgroundIoPressureClass::CompactionRewrite,
        BackgroundIoPressureClass::BackupPrepRead,
        BackgroundIoPressureClass::RepairScan,
    ] {
        let compaction =
            S10CompactionPacingEvidence::from_background_pacing(background_pacing_outcome(class));
        let backup =
            S10BackupExportPacingEvidence::from_background_pacing(background_pacing_outcome(class));
        let repair =
            S10RepairScanPacingEvidence::from_background_pacing(background_pacing_outcome(class));

        match class {
            BackgroundIoPressureClass::CompactionRewrite => {
                assert!(compaction.is_ok());
                assert_missing_s10_pacing_evidence(
                    backup,
                    S6LaterMilestoneDestination::S10BackupExport,
                );
                assert_missing_s10_pacing_evidence(
                    repair,
                    S6LaterMilestoneDestination::S10RepairScan,
                );
            }
            BackgroundIoPressureClass::BackupPrepRead => {
                assert_missing_s10_pacing_evidence(
                    compaction,
                    S6LaterMilestoneDestination::S10Compaction,
                );
                assert!(backup.is_ok());
                assert_missing_s10_pacing_evidence(
                    repair,
                    S6LaterMilestoneDestination::S10RepairScan,
                );
            }
            BackgroundIoPressureClass::RepairScan => {
                assert_missing_s10_pacing_evidence(
                    compaction,
                    S6LaterMilestoneDestination::S10Compaction,
                );
                assert_missing_s10_pacing_evidence(
                    backup,
                    S6LaterMilestoneDestination::S10BackupExport,
                );
                assert!(repair.is_ok());
            }
            _ => unreachable!("Phase 12 only uses these S.10 pressure classes"),
        }
    }
}

#[test]
fn readmission_is_visible_for_every_sealed_handoff_lane() {
    let readiness = scheduler_readiness();
    let placement = publish_s7_placement_io_readiness_handoff(&readiness);
    let compaction_pacing = S10CompactionPacingEvidence::from_background_pacing(
        background_pacing_outcome(BackgroundIoPressureClass::CompactionRewrite),
    )
    .unwrap();
    let backup_pacing = S10BackupExportPacingEvidence::from_background_pacing(
        background_pacing_outcome(BackgroundIoPressureClass::BackupPrepRead),
    )
    .unwrap();
    let repair_pacing = S10RepairScanPacingEvidence::from_background_pacing(
        background_pacing_outcome(BackgroundIoPressureClass::RepairScan),
    )
    .unwrap();
    let compaction = publish_s10_compaction_io_readiness_handoff(&readiness, compaction_pacing);
    let backup = publish_s10_backup_export_io_readiness_handoff(&readiness, backup_pacing);
    let repair = publish_s10_repair_scan_io_readiness_handoff(&readiness, repair_pacing);

    assert_eq!(
        backup.readmission_state(),
        super::S6LaterReadinessReadmissionState::CurrentStoreAuthority
    );

    let compaction = super::readmit_s10_compaction_io_readiness_after_publication(compaction);
    assert_eq!(
        compaction.readmission_state(),
        super::S6LaterReadinessReadmissionState::ReadmittedAfterPublication
    );
    assert_eq!(
        compaction.destination(),
        S6LaterMilestoneDestination::S10Compaction
    );

    let readmitted = readmit_s10_backup_export_io_readiness_after_publication(backup);
    assert_eq!(
        readmitted.readmission_state(),
        super::S6LaterReadinessReadmissionState::ReadmittedAfterPublication
    );
    assert_eq!(
        readmitted.destination(),
        S6LaterMilestoneDestination::S10BackupExport
    );

    let repair = super::readmit_s10_repair_scan_io_readiness_after_publication(repair);
    assert_eq!(
        repair.readmission_state(),
        super::S6LaterReadinessReadmissionState::ReadmittedAfterPublication
    );
    assert_eq!(
        repair.destination(),
        S6LaterMilestoneDestination::S10RepairScan
    );

    let placement = readmit_s7_placement_io_readiness_after_publication(placement);
    assert_eq!(
        placement.readmission_state(),
        super::S6LaterReadinessReadmissionState::ReadmittedAfterPublication
    );

    let security = scheduler_security_scope();
    let backend = secure_frame_backend_admission(&security);
    let secure_io = secure_io_receipt(&security, &backend, SecureIoOperation::VerificationPressure);
    let operator =
        publish_s11_operator_io_readiness_handoff(&readiness, &security, secure_io).unwrap();
    let operator = readmit_s11_operator_io_readiness_after_publication(operator);
    assert_eq!(
        operator.readmission_state(),
        super::S6LaterReadinessReadmissionState::ReadmittedAfterPublication
    );
}

#[test]
fn operator_handoff_preserves_secure_io_scope_without_security_claims() {
    let readiness = scheduler_readiness();
    let security = scheduler_security_scope();
    let backend = secure_frame_backend_admission(&security);
    let secure_io = secure_io_receipt(&security, &backend, SecureIoOperation::VerificationPressure);
    let handoff =
        publish_s11_operator_io_readiness_handoff(&readiness, &security, secure_io).unwrap();
    let seed = admit_s11_operator_io_readiness_seed(handoff);

    assert_eq!(
        seed.handoff().destination(),
        S6LaterMilestoneDestination::S11OperatorReadiness
    );
    assert_eq!(
        seed.non_claims(),
        &[
            S11OperatorReadinessNonClaim::EncryptionAlgorithm,
            S11OperatorReadinessNonClaim::KeyRotation,
            S11OperatorReadinessNonClaim::AuditCorrectness,
            S11OperatorReadinessNonClaim::OperatorAuthorization,
        ]
    );
    assert!(!seed.carries_encryption_algorithm_claim());
    assert!(!seed.carries_key_rotation_claim());
    assert!(!seed.carries_operator_authorization_claim());
    assert_eq!(
        seed.handoff().secure_io_posture(),
        SecureIoPosture::ScopePreserving
    );
    assert_eq!(seed.handoff().backend_requirement(), backend.requirement());
    assert_eq!(seed.handoff().backend_profile(), backend.profile());
    assert_eq!(
        seed.handoff().backend_evidence_class(),
        backend.evidence_class()
    );
}

#[test]
fn operator_handoff_exposes_secure_frame_posture_and_backend_evidence() {
    let readiness = scheduler_readiness();
    let security = scheduler_security_scope();
    let backend = secure_frame_backend_admission(&security);
    let secure_io = secure_io_receipt_with_posture(
        &security,
        &backend,
        SecureIoOperation::BackendExecution,
        SecureIoPostureRequirement::SecureFrameCompatible,
    );
    let handoff =
        publish_s11_operator_io_readiness_handoff(&readiness, &security, secure_io).unwrap();

    assert_eq!(
        handoff.secure_io_posture(),
        SecureIoPosture::SecureFrameCompatible
    );
    assert_eq!(handoff.backend_requirement(), backend.requirement());
    assert_eq!(handoff.backend_profile(), backend.profile());
    assert_eq!(handoff.backend_evidence_class(), backend.evidence_class());
}

#[test]
fn raw_counters_and_certification_evidence_are_typed_denials() {
    assert_eq!(
        reject_raw_s6_counters_as_later_readiness_handoff(
            S6LaterMilestoneDestination::S10Compaction
        ),
        Err(
            S6LaterReadinessHandoffDenial::RawCounterSourceCannotMintHandoff {
                destination: S6LaterMilestoneDestination::S10Compaction,
            }
        )
    );
    assert_eq!(
        reject_certification_only_evidence_as_later_readiness_handoff(
            S6LaterMilestoneDestination::S11OperatorReadiness
        ),
        Err(
            S6LaterReadinessHandoffDenial::CertificationOnlyEvidenceCannotMintHandoff {
                destination: S6LaterMilestoneDestination::S11OperatorReadiness,
            }
        )
    );
}

#[test]
fn operator_handoff_rejects_unrelated_secure_io_operation() {
    let readiness = scheduler_readiness();
    let security = scheduler_security_scope();
    let backend = secure_frame_backend_admission(&security);
    let secure_io = secure_io_receipt(&security, &backend, SecureIoOperation::BackgroundLease);

    assert_eq!(
        publish_s11_operator_io_readiness_handoff(&readiness, &security, secure_io),
        Err(
            S6LaterReadinessHandoffDenial::SecureIoOperationNotFoundation {
                destination: S6LaterMilestoneDestination::S11OperatorReadiness,
            }
        )
    );
}

#[test]
fn operator_handoff_cannot_be_minted_from_unsupported_secure_io_posture() {
    let security = scheduler_security_scope();
    let backend = non_secure_backend_admission();
    let denial = admit_secure_io_scope_for_scheduler(
        SecureIoPreservationRequest::new(
            SecureIoOperation::VerificationPressure,
            &security,
            &backend,
        )
        .require_posture(SecureIoPostureRequirement::SecureFrameCompatible),
    );

    assert_eq!(
        denial,
        Err(SecureIoPreservationDenial::UnsupportedSecureIoPosture {
            operation: SecureIoOperation::VerificationPressure,
            requirement: IoSchedulerBackendCapabilityRequirement::Fsync,
        })
    );
}

fn assert_missing_s10_pacing_evidence<T>(
    result: Result<T, S6LaterReadinessHandoffDenial>,
    destination: S6LaterMilestoneDestination,
) where
    T: core::fmt::Debug + PartialEq,
{
    assert_eq!(
        result,
        Err(S6LaterReadinessHandoffDenial::MissingBackgroundPacingEvidence { destination })
    );
}
