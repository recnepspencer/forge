use forge_store_physical_backend::{
    AdmittedBackendCapabilityWitness, BackendCapabilityAdmissionRequest,
    BackendCapabilityEvidenceBasis, BackendCapabilityKind, BackendCapabilitySupportPosture,
    BackendCapabilitySupportSet, BackendMediaAssumptionSet, BackendRebindTriggers,
    BackendTargetProfile, CapabilityEvidenceClass, PhysicalBackendCapabilityAdmissionAuthority,
};

use crate::pressure_harness::fixtures::replay_bundle_for;
use crate::{
    BackendQualificationMatrixDenial, IoPressureHarnessEvidence, IoPressureHarnessScenario,
    PhysicalFaultEvidenceClass, PhysicalSimulationProfile, QualificationHarnessProofClaim,
    QualificationHarnessProofStrength, QualificationMatrixPublisher, QualificationResidualDebt,
    QualificationResidualDebtReason,
};

#[test]
fn publisher_materializes_all_capability_rows_from_executed_evidence() {
    let evidence = io_pressure_evidence(BackendTargetProfile::PosixFileFsyncDirSync);
    let witness = admitted_backend(
        BackendTargetProfile::PosixFileFsyncDirSync,
        BackendCapabilityEvidenceBasis::certified_backend_profile(),
        BackendCapabilitySupportSet::all_supported(),
    );

    let matrix = QualificationMatrixPublisher::from_executed_store_evidence()
        .with_executed_buffered_file_row(&witness, &evidence)
        .unwrap()
        .with_executed_direct_io_row(&witness, &evidence)
        .unwrap()
        .with_executed_mmap_row(&witness, &evidence)
        .unwrap()
        .with_executed_async_io_row(&witness, &evidence)
        .unwrap()
        .with_executed_flush_durability_row(&witness, &evidence)
        .unwrap()
        .with_executed_directory_sync_row(&witness, &evidence)
        .unwrap()
        .with_executed_durable_rename_row(&witness, &evidence)
        .unwrap()
        .with_executed_secure_frame_io_row(&witness, &evidence)
        .unwrap()
        .publish()
        .unwrap();

    for (capability, proof_claim) in backend_qualification_capabilities() {
        let row = matrix
            .rows_for_claim(BackendTargetProfile::PosixFileFsyncDirSync, capability)
            .next()
            .unwrap();

        assert_eq!(row.capability(), capability);
        assert_eq!(row.harness_proof().claim(), proof_claim);
        assert_eq!(
            row.harness_proof().strength(),
            QualificationHarnessProofStrength::ExplicitBackendQualification
        );
        assert_eq!(
            row.require_certified_backend_support()
                .unwrap()
                .capability(),
            capability
        );
    }
}

#[test]
fn publisher_materializes_non_flush_residual_debt_without_erasing_capability() {
    let evidence = io_pressure_evidence(BackendTargetProfile::PosixFileFsyncDirSync);
    let support = BackendCapabilitySupportSet::all_supported().with_posture(
        BackendCapabilityKind::Mmap,
        BackendCapabilitySupportPosture::Unsupported,
    );
    let witness = admitted_backend(
        BackendTargetProfile::PosixFileFsyncDirSync,
        BackendCapabilityEvidenceBasis::certified_backend_profile(),
        support,
    );
    let debt = QualificationResidualDebt::missing_evidence(
        BackendCapabilityKind::Mmap,
        CapabilityEvidenceClass::CertifiedBackendProfile,
        witness.rebind_triggers(),
    );

    let matrix = QualificationMatrixPublisher::from_executed_store_evidence()
        .with_executed_mmap_row_and_residual_debt(&witness, &evidence, debt)
        .unwrap()
        .publish()
        .unwrap();
    let row = matrix
        .rows_for_claim(
            BackendTargetProfile::PosixFileFsyncDirSync,
            BackendCapabilityKind::Mmap,
        )
        .next()
        .unwrap();

    assert_eq!(row.capability(), BackendCapabilityKind::Mmap);
    assert_eq!(
        row.residual_debt().reason(),
        QualificationResidualDebtReason::MissingEvidence
    );
    assert_eq!(
        row.require_certified_backend_support().unwrap_err(),
        BackendQualificationMatrixDenial::UnsupportedCapability {
            capability: BackendCapabilityKind::Mmap,
            posture: BackendCapabilitySupportPosture::Unsupported,
        }
    );
}

#[test]
fn publisher_rejects_non_flush_unsupported_rows_without_debt() {
    let evidence = io_pressure_evidence(BackendTargetProfile::PosixFileFsyncDirSync);
    let support = BackendCapabilitySupportSet::all_supported().with_posture(
        BackendCapabilityKind::DirectIo,
        BackendCapabilitySupportPosture::Unsupported,
    );
    let witness = admitted_backend(
        BackendTargetProfile::PosixFileFsyncDirSync,
        BackendCapabilityEvidenceBasis::certified_backend_profile(),
        support,
    );

    assert_eq!(
        QualificationMatrixPublisher::from_executed_store_evidence()
            .with_executed_direct_io_row_and_residual_debt(
                &witness,
                &evidence,
                QualificationResidualDebt::none(
                    BackendCapabilityKind::DirectIo,
                    witness.rebind_triggers(),
                ),
            )
            .unwrap_err(),
        BackendQualificationMatrixDenial::MissingResidualDebt {
            capability: BackendCapabilityKind::DirectIo,
            posture: BackendCapabilitySupportPosture::Unsupported,
        }
    );
}

fn backend_qualification_capabilities() -> [(BackendCapabilityKind, QualificationHarnessProofClaim); 8] {
    [
        (
            BackendCapabilityKind::BufferedFile,
            QualificationHarnessProofClaim::BufferedFile,
        ),
        (
            BackendCapabilityKind::DirectIo,
            QualificationHarnessProofClaim::DirectIo,
        ),
        (
            BackendCapabilityKind::Mmap,
            QualificationHarnessProofClaim::Mmap,
        ),
        (
            BackendCapabilityKind::AsyncIo,
            QualificationHarnessProofClaim::AsyncIo,
        ),
        (
            BackendCapabilityKind::Fsync,
            QualificationHarnessProofClaim::FlushDurability,
        ),
        (
            BackendCapabilityKind::DirectorySync,
            QualificationHarnessProofClaim::DirectorySync,
        ),
        (
            BackendCapabilityKind::DurableRename,
            QualificationHarnessProofClaim::DurableRename,
        ),
        (
            BackendCapabilityKind::SecureFrameIo,
            QualificationHarnessProofClaim::SecureFrameIo,
        ),
    ]
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
            all_capability_media_assumptions(),
            BackendRebindTriggers::kernel_filesystem_mount_firmware_and_backend()
                .with_cloud_volume()
                .with_sector_alignment()
                .with_security_posture(),
        ))
        .unwrap()
}

fn all_capability_media_assumptions() -> BackendMediaAssumptionSet {
    BackendMediaAssumptionSet::platform_file_defaults()
        .with_direct_io_alignment()
        .with_sector_atomicity()
        .with_page_cache_policy()
        .with_mmap_coherence()
        .with_async_ordering()
        .with_secure_frame_io()
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
