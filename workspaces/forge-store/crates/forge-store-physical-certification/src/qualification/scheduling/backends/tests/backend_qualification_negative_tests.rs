use forge_store_physical_backend::{
    AdmittedBackendCapabilityWitness, BackendCapabilityAdmissionRequest,
    BackendCapabilityEvidenceBasis, BackendCapabilityKind, BackendCapabilitySupportPosture,
    BackendCapabilitySupportSet, BackendMediaAssumptionSet, BackendRebindTriggers,
    BackendTargetProfile, CapabilityEvidenceClass, PhysicalBackendCapabilityAdmissionAuthority,
};

use crate::pressure_harness::fixtures::replay_bundle_for;
use crate::{
    BackendQualificationMatrixDenial, BackendQualificationRow, IoPressureHarnessEvidence,
    IoPressureHarnessScenario, PhysicalFaultEvidenceClass, PhysicalSimulationProfile,
    QualificationCapabilityProofAuthority, QualificationHarnessProof,
    QualificationHarnessProofStrength, QualificationResidualDebt,
};

#[test]
fn non_supported_rows_require_matching_machine_readable_debt() {
    let evidence = io_pressure_evidence(BackendTargetProfile::PosixFileFsyncDirSync);
    let unsupported = BackendCapabilitySupportSet::all_supported().with_posture(
        BackendCapabilityKind::Fsync,
        BackendCapabilitySupportPosture::Unsupported,
    );
    let witness = admitted_backend(
        BackendTargetProfile::PosixFileFsyncDirSync,
        BackendCapabilityEvidenceBasis::certified_backend_profile(),
        unsupported,
    );

    assert_eq!(
        BackendQualificationRow::from_admitted_backend_witness_with_proof_and_residual_debt(
            &witness,
            BackendCapabilityKind::Fsync,
            &evidence,
            flush_durability_proof(&evidence),
            QualificationResidualDebt::none(
                BackendCapabilityKind::Fsync,
                witness.rebind_triggers()
            ),
        )
        .unwrap_err(),
        BackendQualificationMatrixDenial::MissingResidualDebt {
            capability: BackendCapabilityKind::Fsync,
            posture: BackendCapabilitySupportPosture::Unsupported,
        }
    );
    assert_eq!(
        BackendQualificationRow::from_admitted_backend_witness_with_proof_and_residual_debt(
            &witness,
            BackendCapabilityKind::Fsync,
            &evidence,
            flush_durability_proof(&evidence),
            QualificationResidualDebt::backend_specific_denial(
                BackendCapabilityKind::DirectIo,
                CapabilityEvidenceClass::CertifiedBackendProfile,
                witness.rebind_triggers(),
            ),
        )
        .unwrap_err(),
        BackendQualificationMatrixDenial::ResidualDebtCapabilityMismatch {
            expected: BackendCapabilityKind::Fsync,
            actual: BackendCapabilityKind::DirectIo,
        }
    );
}

#[test]
fn ordinary_row_publication_rejects_generic_pressure_evidence() {
    let evidence = io_pressure_evidence(BackendTargetProfile::PosixFileFsyncDirSync);
    let witness = admitted_backend(
        BackendTargetProfile::PosixFileFsyncDirSync,
        BackendCapabilityEvidenceBasis::certified_backend_profile(),
        BackendCapabilitySupportSet::all_supported(),
    );

    assert_eq!(
        BackendQualificationRow::from_admitted_backend_witness(
            &witness,
            BackendCapabilityKind::Fsync,
            &evidence,
        )
        .unwrap_err(),
        BackendQualificationMatrixDenial::MissingHarnessProof
    );
    assert_eq!(
        BackendQualificationRow::from_admitted_backend_witness_with_residual_debt(
            &witness,
            BackendCapabilityKind::Fsync,
            &evidence,
            QualificationResidualDebt::none(
                BackendCapabilityKind::Fsync,
                witness.rebind_triggers()
            ),
        )
        .unwrap_err(),
        BackendQualificationMatrixDenial::MissingHarnessProof
    );
}

#[test]
fn generic_pressure_proof_cannot_qualify_unrelated_backend_capability() {
    let evidence = io_pressure_evidence(BackendTargetProfile::PosixFileFsyncDirSync);
    let witness = admitted_backend(
        BackendTargetProfile::PosixFileFsyncDirSync,
        BackendCapabilityEvidenceBasis::certified_backend_profile(),
        BackendCapabilitySupportSet::all_supported(),
    );
    let generic_pressure_proof = QualificationHarnessProof::from_io_pressure_evidence(&evidence);

    assert_eq!(
        BackendQualificationRow::from_admitted_backend_witness_with_proof_and_residual_debt(
            &witness,
            BackendCapabilityKind::Fsync,
            &evidence,
            generic_pressure_proof,
            QualificationResidualDebt::none(
                BackendCapabilityKind::Fsync,
                witness.rebind_triggers()
            ),
        )
        .unwrap_err(),
        BackendQualificationMatrixDenial::HarnessProofCapabilityMismatch {
            capability: BackendCapabilityKind::Fsync,
        }
    );
}

#[test]
fn copied_capability_proof_cannot_qualify_different_executed_evidence() {
    let posix_evidence = io_pressure_evidence(BackendTargetProfile::PosixFileFsyncDirSync);
    let copied_posix_proof = flush_durability_proof(&posix_evidence);
    let windows_evidence = io_pressure_evidence(BackendTargetProfile::WindowsFlushFileBuffers);
    let windows_witness = admitted_backend(
        BackendTargetProfile::WindowsFlushFileBuffers,
        BackendCapabilityEvidenceBasis::certified_backend_profile(),
        BackendCapabilitySupportSet::all_supported(),
    );

    assert_eq!(
        BackendQualificationRow::from_admitted_backend_witness_with_proof_and_residual_debt(
            &windows_witness,
            BackendCapabilityKind::Fsync,
            &windows_evidence,
            copied_posix_proof,
            QualificationResidualDebt::none(
                BackendCapabilityKind::Fsync,
                windows_witness.rebind_triggers(),
            ),
        )
        .unwrap_err(),
        BackendQualificationMatrixDenial::HarnessProofEvidenceMismatch {
            capability: BackendCapabilityKind::Fsync,
        }
    );
}

#[test]
fn executed_capability_proof_constructors_bind_single_claims() {
    let evidence = io_pressure_evidence(BackendTargetProfile::PosixFileFsyncDirSync);

    for (proof, covered, rejected) in [
        (
            QualificationHarnessProof::from_executed_buffered_file_evidence(
                proof_authority(),
                &evidence,
            ),
            BackendCapabilityKind::BufferedFile,
            BackendCapabilityKind::DirectIo,
        ),
        (
            QualificationHarnessProof::from_executed_direct_io_evidence(
                proof_authority(),
                &evidence,
            ),
            BackendCapabilityKind::DirectIo,
            BackendCapabilityKind::Mmap,
        ),
        (
            QualificationHarnessProof::from_executed_mmap_evidence(proof_authority(), &evidence),
            BackendCapabilityKind::Mmap,
            BackendCapabilityKind::AsyncIo,
        ),
        (
            QualificationHarnessProof::from_executed_async_io_evidence(
                proof_authority(),
                &evidence,
            ),
            BackendCapabilityKind::AsyncIo,
            BackendCapabilityKind::Fsync,
        ),
        (
            QualificationHarnessProof::from_executed_flush_durability_evidence(
                proof_authority(),
                &evidence,
            ),
            BackendCapabilityKind::Fsync,
            BackendCapabilityKind::DirectorySync,
        ),
        (
            QualificationHarnessProof::from_executed_directory_sync_evidence(
                proof_authority(),
                &evidence,
            ),
            BackendCapabilityKind::DirectorySync,
            BackendCapabilityKind::DurableRename,
        ),
        (
            QualificationHarnessProof::from_executed_durable_rename_evidence(
                proof_authority(),
                &evidence,
            ),
            BackendCapabilityKind::DurableRename,
            BackendCapabilityKind::SecureFrameIo,
        ),
        (
            QualificationHarnessProof::from_executed_secure_frame_io_evidence(
                proof_authority(),
                &evidence,
            ),
            BackendCapabilityKind::SecureFrameIo,
            BackendCapabilityKind::BufferedFile,
        ),
    ] {
        assert!(proof.covers(covered));
        assert!(!proof.covers(rejected));
        assert_eq!(
            proof.strength(),
            QualificationHarnessProofStrength::ExplicitBackendQualification
        );
    }
}

#[test]
fn developer_smoke_pressure_evidence_cannot_publish_certified_backend_row() {
    let evidence =
        developer_smoke_io_pressure_evidence(BackendTargetProfile::PosixFileFsyncDirSync);
    let witness = admitted_backend(
        BackendTargetProfile::PosixFileFsyncDirSync,
        BackendCapabilityEvidenceBasis::certified_backend_profile(),
        BackendCapabilitySupportSet::all_supported(),
    );

    assert_eq!(
        BackendQualificationRow::from_admitted_backend_witness_with_proof(
            &witness,
            BackendCapabilityKind::Fsync,
            &evidence,
            flush_durability_proof(&evidence),
        )
        .unwrap_err(),
        BackendQualificationMatrixDenial::HarnessProofStrengthTooWeak {
            required: QualificationHarnessProofStrength::ExplicitBackendQualification,
            actual: QualificationHarnessProofStrength::SimulationOnly,
        }
    );
}

fn admitted_backend(
    profile: BackendTargetProfile,
    basis: BackendCapabilityEvidenceBasis,
    support: BackendCapabilitySupportSet,
) -> AdmittedBackendCapabilityWitness {
    PhysicalBackendCapabilityAdmissionAuthority::store_owned()
        .admit_backend_capability(BackendCapabilityAdmissionRequest::new(
            profile,
            basis,
            support,
            BackendMediaAssumptionSet::platform_file_defaults(),
            BackendRebindTriggers::kernel_filesystem_mount_firmware_and_backend()
                .with_cloud_volume()
                .with_sector_alignment()
                .with_security_posture(),
        ))
        .unwrap()
}

fn io_pressure_evidence(profile: BackendTargetProfile) -> IoPressureHarnessEvidence {
    let scenario = IoPressureHarnessScenario::deterministic_read_under_repair_pressure()
        .with_backend_profile(profile)
        .with_fault_evidence_class(PhysicalFaultEvidenceClass::CertifiedBackend);
    let replay = replay_bundle_for(
        scenario.clone(),
        PhysicalSimulationProfile::HardwareQualification,
    );
    IoPressureHarnessEvidence::from_replay_bundle(scenario, &replay).unwrap()
}

fn developer_smoke_io_pressure_evidence(
    profile: BackendTargetProfile,
) -> IoPressureHarnessEvidence {
    let scenario = IoPressureHarnessScenario::deterministic_read_under_repair_pressure()
        .with_backend_profile(profile);
    let replay = replay_bundle_for(scenario.clone(), PhysicalSimulationProfile::DeveloperSmoke);
    IoPressureHarnessEvidence::from_replay_bundle(scenario, &replay).unwrap()
}

fn flush_durability_proof(evidence: &IoPressureHarnessEvidence) -> QualificationHarnessProof {
    QualificationHarnessProof::from_executed_flush_durability_evidence(proof_authority(), evidence)
}

fn proof_authority() -> QualificationCapabilityProofAuthority {
    QualificationCapabilityProofAuthority::from_executed_store_evidence()
}
