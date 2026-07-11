use forge_store_physical_backend::{
    AdmittedBackendCapabilityWitness, BackendCapabilityAdmissionRequest,
    BackendCapabilityEvidenceBasis, BackendCapabilityKind, BackendCapabilitySupportPosture,
    BackendCapabilitySupportSet, BackendMediaAssumptionSet, BackendRebindTriggers,
    BackendTargetProfile, CapabilityEvidenceClass, PhysicalBackendCapabilityAdmissionAuthority,
};
use forge_store_physical_certification::{
    BackendQualificationMatrix, IoPressureHarnessEvidence, QualificationMatrixPublisher,
    QualificationResidualDebt,
};

pub(super) fn matrix_with_required_residual_debt(
    witness: &AdmittedBackendCapabilityWitness,
    harness: &IoPressureHarnessEvidence,
) -> BackendQualificationMatrix {
    let rebind = BackendRebindTriggers::kernel_filesystem_mount_firmware_and_backend();
    QualificationMatrixPublisher::from_executed_store_evidence()
        .with_executed_buffered_file_row(witness, harness)
        .unwrap()
        .with_executed_direct_io_row_and_residual_debt(
            witness,
            harness,
            QualificationResidualDebt::missing_evidence(
                BackendCapabilityKind::DirectIo,
                CapabilityEvidenceClass::CertifiedBackendProfile,
                rebind,
            ),
        )
        .unwrap()
        .with_executed_mmap_row_and_residual_debt(
            witness,
            harness,
            QualificationResidualDebt::degraded_operation(
                BackendCapabilityKind::Mmap,
                CapabilityEvidenceClass::CertifiedBackendProfile,
                rebind,
            ),
        )
        .unwrap()
        .with_executed_async_io_row(
            &backend_witness_with_posture(
                BackendCapabilityKind::AsyncIo,
                BackendCapabilitySupportPosture::Unsupported,
            ),
            harness,
        )
        .unwrap()
        .with_executed_flush_durability_row(
            &backend_witness_with_posture(
                BackendCapabilityKind::Fsync,
                BackendCapabilitySupportPosture::Unavailable,
            ),
            harness,
        )
        .unwrap()
        .with_executed_directory_sync_row(
            &backend_witness_with_posture(
                BackendCapabilityKind::DirectorySync,
                BackendCapabilitySupportPosture::Stale,
            ),
            harness,
        )
        .unwrap()
        .with_executed_durable_rename_row(
            &backend_witness_with_posture(
                BackendCapabilityKind::DurableRename,
                BackendCapabilitySupportPosture::RebindRequired,
            ),
            harness,
        )
        .unwrap()
        .publish()
        .unwrap()
}

pub(super) fn matrix_with_amplified_required_residual_debt(
    witness: &AdmittedBackendCapabilityWitness,
    harness: &IoPressureHarnessEvidence,
) -> BackendQualificationMatrix {
    let rebind = BackendRebindTriggers::kernel_filesystem_mount_firmware_and_backend();
    QualificationMatrixPublisher::from_executed_store_evidence()
        .with_executed_buffered_file_row(witness, harness)
        .unwrap()
        .with_executed_direct_io_row_and_residual_debt(
            witness,
            harness,
            QualificationResidualDebt::missing_evidence(
                BackendCapabilityKind::DirectIo,
                CapabilityEvidenceClass::CertifiedBackendProfile,
                rebind,
            ),
        )
        .unwrap()
        .with_executed_mmap_row_and_residual_debt(
            witness,
            harness,
            QualificationResidualDebt::degraded_operation(
                BackendCapabilityKind::Mmap,
                CapabilityEvidenceClass::CertifiedBackendProfile,
                rebind,
            ),
        )
        .unwrap()
        .with_executed_async_io_row(
            &backend_witness_with_posture(
                BackendCapabilityKind::AsyncIo,
                BackendCapabilitySupportPosture::Unsupported,
            ),
            harness,
        )
        .unwrap()
        .with_executed_flush_durability_row(
            &backend_witness_with_posture(
                BackendCapabilityKind::Fsync,
                BackendCapabilitySupportPosture::Unavailable,
            ),
            harness,
        )
        .unwrap()
        .with_executed_directory_sync_row(
            &backend_witness_with_posture(
                BackendCapabilityKind::DirectorySync,
                BackendCapabilitySupportPosture::Stale,
            ),
            harness,
        )
        .unwrap()
        .with_executed_durable_rename_row(
            &backend_witness_with_posture(
                BackendCapabilityKind::DurableRename,
                BackendCapabilitySupportPosture::RebindRequired,
            ),
            harness,
        )
        .unwrap()
        .with_executed_secure_frame_io_row_and_residual_debt(
            witness,
            harness,
            QualificationResidualDebt::stale_evidence(
                BackendCapabilityKind::SecureFrameIo,
                CapabilityEvidenceClass::CertifiedBackendProfile,
                rebind,
            ),
        )
        .unwrap()
        .publish()
        .unwrap()
}

fn backend_witness_with_posture(
    capability: BackendCapabilityKind,
    posture: BackendCapabilitySupportPosture,
) -> AdmittedBackendCapabilityWitness {
    PhysicalBackendCapabilityAdmissionAuthority::store_owned()
        .admit_backend_capability(BackendCapabilityAdmissionRequest::new(
            BackendTargetProfile::PosixFileFsyncDirSync,
            BackendCapabilityEvidenceBasis::certified_backend_profile(),
            BackendCapabilitySupportSet::all_supported().with_posture(capability, posture),
            BackendMediaAssumptionSet::platform_file_defaults()
                .with_direct_io_alignment()
                .with_sector_atomicity()
                .with_page_cache_policy()
                .with_mmap_coherence()
                .with_async_ordering()
                .with_secure_frame_io()
                .with_flush_ordering()
                .with_fdatasync_durability(),
            BackendRebindTriggers::kernel_filesystem_mount_firmware_and_backend(),
        ))
        .unwrap()
}
