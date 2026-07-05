use forge_store_certification::{
    certify_s6_backend_qualification_matrix, certify_s6_later_readiness_handoffs,
    S6CertificationMaterializationDenial, S6IoPressureHarnessCloseoutEvidence,
    StoreOwnedS6CertificationMaterializationSources,
};
use forge_store_io_scheduler::foreground_reservation::admitted_point_read_reservation_for_certification_test;
use forge_store_io_scheduler::{
    admit_secure_frame_backend_capability_for_scheduler_claim, admit_secure_io_scope_for_scheduler,
    background_pacing_outcome_for_later_readiness_certification_test,
    publish_s10_backup_export_io_readiness_handoff, publish_s10_compaction_io_readiness_handoff,
    publish_s10_repair_scan_io_readiness_handoff, publish_s11_operator_io_readiness_handoff,
    publish_s7_placement_io_readiness_handoff, BackgroundIoPressureClass,
    S10BackupExportPacingEvidence, S10CompactionPacingEvidence, S10RepairScanPacingEvidence,
    SecureIoOperation, SecureIoPostureRequirement, SecureIoPreservationRequest,
};
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

pub fn sources_with_later_handoff_backend_mismatch(
) -> Result<StoreOwnedS6CertificationMaterializationSources, S6CertificationMaterializationDenial> {
    from_parts_with_later_handoffs(
        super::backend_witness(),
        later_handoffs_for_operator_backend(&mismatched_backend_witness(
            BackendTargetProfile::WindowsFlushFileBuffers,
            BackendCapabilityEvidenceBasis::certified_backend_profile(),
        )),
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
    from_parts_with_later_handoffs_and_access(witness, super::later_handoffs(), access_rows)
}

fn from_parts_with_later_handoffs(
    witness: AdmittedBackendCapabilityWitness,
    later: forge_store_certification::S6LaterReadinessHandoffCertification,
) -> Result<StoreOwnedS6CertificationMaterializationSources, S6CertificationMaterializationDenial> {
    from_parts_with_later_handoffs_and_access(
        witness,
        later,
        super::access_policy_evidence::access_policy_rows(),
    )
}

fn from_parts_with_qualification(
    witness: AdmittedBackendCapabilityWitness,
    qualification: forge_store_certification::S6BackendQualificationMatrixCertification,
) -> Result<StoreOwnedS6CertificationMaterializationSources, S6CertificationMaterializationDenial> {
    from_parts_full(
        witness,
        super::later_handoffs(),
        super::access_policy_evidence::access_policy_rows(),
        qualification,
    )
}

fn from_parts_with_later_handoffs_and_access(
    witness: AdmittedBackendCapabilityWitness,
    later: forge_store_certification::S6LaterReadinessHandoffCertification,
    access_rows: Vec<forge_store_certification::S6AccessPolicyEvidenceRow>,
) -> Result<StoreOwnedS6CertificationMaterializationSources, S6CertificationMaterializationDenial> {
    let harness = super::harness_evidence();
    let qualification = qualification_for_harness(&harness);
    from_parts_full(witness, later, access_rows, qualification)
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
    later: forge_store_certification::S6LaterReadinessHandoffCertification,
    access_rows: Vec<forge_store_certification::S6AccessPolicyEvidenceRow>,
    qualification: forge_store_certification::S6BackendQualificationMatrixCertification,
) -> Result<StoreOwnedS6CertificationMaterializationSources, S6CertificationMaterializationDenial> {
    let security = super::security_scope();
    let harness = super::harness_evidence();
    StoreOwnedS6CertificationMaterializationSources::from_bound_store_execution(
        witness,
        admitted_point_read_reservation_for_certification_test(),
        background_pacing_outcome_for_later_readiness_certification_test(
            BackgroundIoPressureClass::RepairScan,
        ),
        super::queue_outcome(),
        super::secure_io_preservation(&security),
        access_rows,
        vec![super::durability_evidence::flush_row()],
        S6IoPressureHarnessCloseoutEvidence::from_harness_evidence(harness),
        qualification,
        later,
        None,
    )
}

fn later_handoffs_for_operator_backend(
    witness: &AdmittedBackendCapabilityWitness,
) -> forge_store_certification::S6LaterReadinessHandoffCertification {
    let readiness = super::scheduler_readiness();
    let security = super::security_scope();
    let backend =
        admit_secure_frame_backend_capability_for_scheduler_claim(witness, &security).unwrap();
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
        admit_secure_io_scope_for_scheduler(
            SecureIoPreservationRequest::new(
                SecureIoOperation::VerificationPressure,
                &security,
                &backend,
            )
            .require_posture(SecureIoPostureRequirement::SecureFrameCompatible),
        )
        .unwrap(),
    )
    .unwrap();
    certify_s6_later_readiness_handoffs(&placement, &compaction, &backup, &repair, &operator)
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
