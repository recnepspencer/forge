use forge_store_certification::{
    certify_s6_backend_qualification_matrix, S6CertificationMaterializationDenial,
    S6IoPressureHarnessCloseoutEvidence, StoreOwnedS6CertificationMaterializationSources,
};
use forge_store_io_scheduler::foreground_reservation::admitted_point_read_reservation_for_certification_test;
use forge_store_physical_backend::{
    AdmittedBackendCapabilityWitness, BackendCapabilityAdmissionRequest,
    BackendCapabilityEvidenceBasis, BackendCapabilitySupportSet, BackendMediaAssumptionSet,
    BackendRebindTriggers, BackendTargetProfile, PhysicalBackendCapabilityAdmissionAuthority,
};
use forge_store_physical_certification::QualificationMatrixPublisher;

pub fn sources_with_backend_profile_mismatch(
) -> Result<StoreOwnedS6CertificationMaterializationSources, S6CertificationMaterializationDenial> {
    from_parts(mismatched_backend_witness(
        BackendTargetProfile::WindowsFlushFileBuffers,
        BackendCapabilityEvidenceBasis::certified_backend_profile(),
    ))
}

pub fn sources_with_backend_evidence_class_mismatch(
) -> Result<StoreOwnedS6CertificationMaterializationSources, S6CertificationMaterializationDenial> {
    from_parts(mismatched_backend_witness(
        BackendTargetProfile::PosixFileFsyncDirSync,
        BackendCapabilityEvidenceBasis::externally_guaranteed(1),
    ))
}

pub fn sources_with_access_policy_backend_mismatch(
) -> Result<StoreOwnedS6CertificationMaterializationSources, S6CertificationMaterializationDenial> {
    let access_backend = mismatched_backend_witness(
        BackendTargetProfile::WindowsFlushFileBuffers,
        BackendCapabilityEvidenceBasis::certified_backend_profile(),
    );
    from_parts_with_access_policy(
        super::backend_witness(),
        super::access_policy_evidence::access_policy_rows_for_backend(&access_backend),
    )
}

pub fn sources_with_empty_qualification_matrix(
) -> Result<StoreOwnedS6CertificationMaterializationSources, S6CertificationMaterializationDenial> {
    let empty = QualificationMatrixPublisher::from_executed_store_evidence()
        .publish()
        .unwrap();
    from_parts_with_qualification(
        super::backend_witness(),
        certify_s6_backend_qualification_matrix(empty).unwrap(),
    )
}

#[allow(dead_code)]
pub fn sources_without_required_residual_debt(
) -> Result<StoreOwnedS6CertificationMaterializationSources, S6CertificationMaterializationDenial> {
    let harness = super::harness_evidence();
    from_parts_with_qualification(
        super::backend_witness(),
        qualification_for_harness(&harness),
    )
}

#[allow(dead_code)]
pub fn sources_with_amplified_required_residual_debt(
) -> Result<StoreOwnedS6CertificationMaterializationSources, S6CertificationMaterializationDenial> {
    let witness = super::backend_witness();
    let harness = super::harness_evidence();
    from_parts_with_qualification(
        witness.clone(),
        certify_s6_backend_qualification_matrix(
            super::qualification_residual_debt::matrix_with_amplified_required_residual_debt(
                &witness, &harness,
            ),
        )
        .unwrap(),
    )
}

fn from_parts(
    witness: AdmittedBackendCapabilityWitness,
) -> Result<StoreOwnedS6CertificationMaterializationSources, S6CertificationMaterializationDenial> {
    from_parts_with_access_policy(witness, super::access_policy_evidence::access_policy_rows())
}

fn from_parts_with_access_policy(
    witness: AdmittedBackendCapabilityWitness,
    access_rows: Vec<forge_store_certification::S6AccessPolicyEvidenceRow>,
) -> Result<StoreOwnedS6CertificationMaterializationSources, S6CertificationMaterializationDenial> {
    let harness = super::harness_evidence();
    let qualification = qualification_for_harness(&harness);
    from_parts_full(witness, access_rows, qualification)
}

fn from_parts_with_qualification(
    witness: AdmittedBackendCapabilityWitness,
    qualification: forge_store_certification::S6BackendQualificationMatrixCertification,
) -> Result<StoreOwnedS6CertificationMaterializationSources, S6CertificationMaterializationDenial> {
    from_parts_full(
        witness,
        super::access_policy_evidence::access_policy_rows(),
        qualification,
    )
}

fn qualification_for_harness(
    harness: &forge_store_physical_certification::S6IoPressureHarnessEvidence,
) -> forge_store_certification::S6BackendQualificationMatrixCertification {
    certify_s6_backend_qualification_matrix(
        QualificationMatrixPublisher::from_executed_store_evidence()
            .with_executed_buffered_file_row(&super::backend_witness(), harness)
            .unwrap()
            .publish()
            .unwrap(),
    )
    .unwrap()
}

fn from_parts_full(
    witness: AdmittedBackendCapabilityWitness,
    access_rows: Vec<forge_store_certification::S6AccessPolicyEvidenceRow>,
    qualification: forge_store_certification::S6BackendQualificationMatrixCertification,
) -> Result<StoreOwnedS6CertificationMaterializationSources, S6CertificationMaterializationDenial> {
    let security = super::security_scope();
    let harness = super::harness_evidence();
    StoreOwnedS6CertificationMaterializationSources::from_bound_store_execution(
        witness,
        admitted_point_read_reservation_for_certification_test(),
        super::background_pacing_outcome(),
        super::queue_outcome(),
        super::secure_io_preservation(&security),
        access_rows,
        vec![super::durability_evidence::flush_row()],
        S6IoPressureHarnessCloseoutEvidence::from_harness_evidence(harness),
        qualification,
        None,
    )
}

fn mismatched_backend_witness(
    profile: BackendTargetProfile,
    basis: BackendCapabilityEvidenceBasis,
) -> AdmittedBackendCapabilityWitness {
    PhysicalBackendCapabilityAdmissionAuthority::store_owned()
        .admit_backend_capability(BackendCapabilityAdmissionRequest::new(
            profile,
            basis,
            BackendCapabilitySupportSet::all_supported(),
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
