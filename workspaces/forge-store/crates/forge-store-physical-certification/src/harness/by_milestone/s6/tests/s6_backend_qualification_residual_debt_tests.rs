use forge_store_physical_backend::{
    AdmittedBackendCapabilityWitness, BackendCapabilityAdmissionRequest,
    BackendCapabilityEvidenceBasis, BackendCapabilityKind, BackendCapabilitySupportPosture,
    BackendCapabilitySupportSet, BackendMediaAssumptionSet, BackendRebindTriggers,
    BackendTargetProfile, CapabilityEvidenceClass, PhysicalBackendCapabilityAdmissionAuthority,
};

use crate::s6_io_pressure_test_support::replay_bundle_for;
use crate::{
    BackendQualificationMatrixDenial, PhysicalFaultEvidenceClass, PhysicalSimulationProfile,
    PublishedQualificationPosture, QualificationMatrixPublisher, QualificationResidualDebt,
    QualificationResidualDebtReason, S6IoPressureHarnessEvidence, S6IoPressureHarnessScenario,
};

#[test]
fn residual_debt_is_machine_readable_for_all_non_supported_postures() {
    for case in residual_debt_cases() {
        let capability = BackendCapabilityKind::Mmap;
        let support = BackendCapabilitySupportSet::all_supported()
            .with_posture(capability, case.support_posture);
        let witness = admitted_backend(support);
        let evidence = io_pressure_evidence();
        let matrix = QualificationMatrixPublisher::from_executed_store_evidence()
            .with_executed_mmap_row(&witness, &evidence)
            .unwrap()
            .publish()
            .unwrap();
        let row = matrix
            .rows_for_claim(BackendTargetProfile::PosixFileFsyncDirSync, capability)
            .next()
            .unwrap();

        assert_eq!(row.published_posture(), case.published_posture);
        assert_eq!(row.residual_debt().reason(), case.reason);
        assert_eq!(row.residual_debt().affected_capability(), capability);
        assert_eq!(
            row.residual_debt().missing_evidence_class(),
            CapabilityEvidenceClass::CertifiedBackendProfile
        );
        assert_eq!(
            row.residual_debt().rebind_triggers(),
            witness.rebind_triggers()
        );
        assert_eq!(
            row.require_certified_backend_support().unwrap_err(),
            case.denial
        );
    }
}

#[test]
fn supported_rows_with_residual_debt_publish_as_degraded_with_full_cause() {
    for (reason, debt) in supported_debt_cases() {
        let capability = BackendCapabilityKind::DirectIo;
        let witness = admitted_backend(BackendCapabilitySupportSet::all_supported());
        let evidence = io_pressure_evidence();
        let matrix = QualificationMatrixPublisher::from_executed_store_evidence()
            .with_executed_direct_io_row_and_residual_debt(&witness, &evidence, debt)
            .unwrap()
            .publish()
            .unwrap();
        let row = matrix
            .rows_for_claim(BackendTargetProfile::PosixFileFsyncDirSync, capability)
            .next()
            .unwrap();

        assert_eq!(
            row.published_posture(),
            PublishedQualificationPosture::Degraded
        );
        assert_eq!(row.residual_debt().reason(), reason);
        assert_eq!(row.residual_debt().affected_capability(), capability);
        assert_eq!(
            row.residual_debt().missing_evidence_class(),
            CapabilityEvidenceClass::CertifiedBackendProfile
        );
        assert_eq!(
            row.residual_debt().rebind_triggers(),
            witness.rebind_triggers()
        );
        assert_eq!(
            row.require_certified_backend_support().unwrap_err(),
            BackendQualificationMatrixDenial::ResidualDebtPresent { capability }
        );
    }
}

#[derive(Clone, Copy)]
struct ResidualDebtCase {
    support_posture: BackendCapabilitySupportPosture,
    published_posture: PublishedQualificationPosture,
    reason: QualificationResidualDebtReason,
    denial: BackendQualificationMatrixDenial,
}

fn residual_debt_cases() -> [ResidualDebtCase; 5] {
    let capability = BackendCapabilityKind::Mmap;
    [
        ResidualDebtCase {
            support_posture: BackendCapabilitySupportPosture::Unsupported,
            published_posture: PublishedQualificationPosture::Unsupported,
            reason: QualificationResidualDebtReason::BackendSpecificDenial,
            denial: BackendQualificationMatrixDenial::UnsupportedCapability {
                capability,
                posture: BackendCapabilitySupportPosture::Unsupported,
            },
        },
        ResidualDebtCase {
            support_posture: BackendCapabilitySupportPosture::Unavailable,
            published_posture: PublishedQualificationPosture::Unavailable,
            reason: QualificationResidualDebtReason::BackendSpecificDenial,
            denial: BackendQualificationMatrixDenial::UnsupportedCapability {
                capability,
                posture: BackendCapabilitySupportPosture::Unavailable,
            },
        },
        ResidualDebtCase {
            support_posture: BackendCapabilitySupportPosture::Unknown,
            published_posture: PublishedQualificationPosture::Unknown,
            reason: QualificationResidualDebtReason::BackendSpecificDenial,
            denial: BackendQualificationMatrixDenial::UnsupportedCapability {
                capability,
                posture: BackendCapabilitySupportPosture::Unknown,
            },
        },
        ResidualDebtCase {
            support_posture: BackendCapabilitySupportPosture::Stale,
            published_posture: PublishedQualificationPosture::Stale,
            reason: QualificationResidualDebtReason::StaleEvidence,
            denial: BackendQualificationMatrixDenial::StaleRow { capability },
        },
        ResidualDebtCase {
            support_posture: BackendCapabilitySupportPosture::RebindRequired,
            published_posture: PublishedQualificationPosture::RebindRequired,
            reason: QualificationResidualDebtReason::StaleEvidence,
            denial: BackendQualificationMatrixDenial::RebindRequired {
                capability,
                triggers: BackendRebindTriggers::kernel_filesystem_mount_firmware_and_backend()
                    .with_cloud_volume()
                    .with_sector_alignment()
                    .with_security_posture(),
            },
        },
    ]
}

fn supported_debt_cases() -> [(QualificationResidualDebtReason, QualificationResidualDebt); 2] {
    let triggers = BackendRebindTriggers::kernel_filesystem_mount_firmware_and_backend()
        .with_cloud_volume()
        .with_sector_alignment()
        .with_security_posture();
    [
        (
            QualificationResidualDebtReason::MissingEvidence,
            QualificationResidualDebt::missing_evidence(
                BackendCapabilityKind::DirectIo,
                CapabilityEvidenceClass::CertifiedBackendProfile,
                triggers,
            ),
        ),
        (
            QualificationResidualDebtReason::DegradedOperation,
            QualificationResidualDebt::degraded_operation(
                BackendCapabilityKind::DirectIo,
                CapabilityEvidenceClass::CertifiedBackendProfile,
                triggers,
            ),
        ),
    ]
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

fn io_pressure_evidence() -> S6IoPressureHarnessEvidence {
    let scenario = S6IoPressureHarnessScenario::deterministic_read_under_repair_pressure()
        .with_backend_profile(BackendTargetProfile::PosixFileFsyncDirSync)
        .with_fault_evidence_class(PhysicalFaultEvidenceClass::CertifiedBackend);
    let replay = replay_bundle_for(
        scenario.clone(),
        PhysicalSimulationProfile::HardwareQualification,
    );
    S6IoPressureHarnessEvidence::from_replay_bundle(scenario, &replay).unwrap()
}
