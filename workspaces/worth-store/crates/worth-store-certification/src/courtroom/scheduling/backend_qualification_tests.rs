use worth_store_physical_backend::{
    AdmittedBackendCapabilityWitness, BackendCapabilityAdmissionRequest,
    BackendCapabilityEvidenceBasis, BackendCapabilityKind, BackendCapabilitySupportPosture,
    BackendCapabilitySupportSet, BackendMediaAssumptionSet, BackendRebindTriggers,
    BackendTargetProfile, PhysicalBackendCapabilityAdmissionAuthority,
};
use worth_store_physical_certification::{
    BackendQualificationMatrixDenial, IoPressureHarnessEvidence, IoPressureHarnessScenario,
    PhysicalFaultEvidenceClass, PhysicalSimulationProfile, PublishedQualificationPosture,
    QualificationMatrixPublisher, QualificationResidualDebt,
};

use super::{
    certify_io_pressure_backend_qualification_matrix, S6BackendQualificationMatrixCertification,
};

#[test]
fn certification_outcomes_keep_denied_and_degraded_rows_visible() {
    let evidence = io_pressure_evidence();
    let supported = admitted_backend(BackendCapabilitySupportSet::all_supported());
    let unsupported = admitted_backend(BackendCapabilitySupportSet::all_supported().with_posture(
        BackendCapabilityKind::DirectIo,
        BackendCapabilitySupportPosture::Unsupported,
    ));
    let degraded_debt = QualificationResidualDebt::missing_evidence(
        BackendCapabilityKind::Mmap,
        worth_store_physical_backend::CapabilityEvidenceClass::CertifiedBackendProfile,
        supported.rebind_triggers(),
    );

    let matrix = QualificationMatrixPublisher::from_executed_store_evidence()
        .with_executed_flush_durability_row(&supported, &evidence)
        .unwrap()
        .with_executed_direct_io_row(&unsupported, &evidence)
        .unwrap()
        .with_executed_mmap_row_and_residual_debt(&supported, &evidence, degraded_debt)
        .unwrap()
        .publish()
        .unwrap();
    let certification = certify_io_pressure_backend_qualification_matrix(matrix).unwrap();

    assert_eq!(certification.row_count(), 3);
    assert_eq!(certification.certified_support_rows().len(), 1);
    assert_row_outcomes_preserve_denials(certification);
}

fn assert_row_outcomes_preserve_denials(certification: S6BackendQualificationMatrixCertification) {
    let outcomes = certification.row_outcomes();

    assert_eq!(outcomes.len(), 3);
    assert!(outcomes.iter().any(|outcome| {
        outcome.row().capability() == BackendCapabilityKind::Fsync
            && outcome.certified_support().is_ok()
    }));
    assert!(outcomes.iter().any(|outcome| {
        outcome.row().capability() == BackendCapabilityKind::DirectIo
            && outcome.row().published_posture() == PublishedQualificationPosture::Unsupported
            && outcome.certified_support()
                == Err(BackendQualificationMatrixDenial::UnsupportedCapability {
                    capability: BackendCapabilityKind::DirectIo,
                    posture: BackendCapabilitySupportPosture::Unsupported,
                })
    }));
    assert!(outcomes.iter().any(|outcome| {
        outcome.row().capability() == BackendCapabilityKind::Mmap
            && outcome.row().published_posture() == PublishedQualificationPosture::Degraded
            && outcome.certified_support()
                == Err(BackendQualificationMatrixDenial::ResidualDebtPresent {
                    capability: BackendCapabilityKind::Mmap,
                })
    }));
}

fn admitted_backend(support: BackendCapabilitySupportSet) -> AdmittedBackendCapabilityWitness {
    PhysicalBackendCapabilityAdmissionAuthority::store_owned()
        .admit_backend_capability(BackendCapabilityAdmissionRequest::new(
            BackendTargetProfile::PosixFileFsyncDirSync,
            BackendCapabilityEvidenceBasis::certified_backend_profile(),
            support,
            BackendMediaAssumptionSet::platform_file_defaults()
                .with_direct_io_alignment()
                .with_sector_atomicity()
                .with_page_cache_policy()
                .with_mmap_coherence()
                .with_async_ordering()
                .with_secure_frame_io(),
            BackendRebindTriggers::kernel_filesystem_mount_firmware_and_backend()
                .with_cloud_volume()
                .with_sector_alignment()
                .with_security_posture(),
        ))
        .unwrap()
}

fn io_pressure_evidence() -> IoPressureHarnessEvidence {
    let scenario = IoPressureHarnessScenario::deterministic_read_under_repair_pressure()
        .with_backend_profile(BackendTargetProfile::PosixFileFsyncDirSync)
        .with_fault_evidence_class(PhysicalFaultEvidenceClass::CertifiedBackend);
    let replay = worth_store_physical_certification::io_pressure_test_replay_bundle_for(
        scenario.clone(),
        PhysicalSimulationProfile::HardwareQualification,
    );
    IoPressureHarnessEvidence::from_replay_bundle(scenario, &replay).unwrap()
}
