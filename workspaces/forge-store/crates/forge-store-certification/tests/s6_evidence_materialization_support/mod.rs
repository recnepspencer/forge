mod access_policy_evidence;
mod assertions;
mod durability_evidence;
mod qualification_residual_debt;
pub mod source_variants;
pub use assertions::{
    assert_fixture_counter_strength_matrix, assert_performance_receipts_are_exact_for_fixture,
    assert_readiness_fixture_counter_strength_matrix, assert_readiness_residual_debt_matrix,
    assert_source_denial, assert_violation_row,
};
use forge_foundational::{
    performance, FoundationalPerformanceAccessPatternPosture, FoundationalPerformanceBoundary,
    FoundationalPerformanceBreadthLocalityPosture, FoundationalPerformanceBudgetKind,
    FoundationalPerformanceEvidenceStrength, FoundationalPerformanceExecutionTemperature,
    FoundationalPerformanceFallbackDebtPosture, FoundationalPerformanceFreshnessRetentionPosture,
    FoundationalPerformanceWorkClass,
};
use forge_store_buffer_pool::BufferPoolQueueExecutionDeclaration;
pub use source_variants::{
    sources_with_access_policy_backend_mismatch, sources_with_backend_evidence_class_mismatch,
    sources_with_backend_profile_mismatch, sources_with_empty_qualification_matrix,
    sources_with_later_handoff_backend_mismatch,
};

use forge_store_certification::{
    certify_s6_backend_qualification_matrix, certify_s6_later_readiness_handoffs,
    S6AccessPolicyEvidenceRow, S6FlushDurabilityEvidenceRow, S6IoPressureHarnessCloseoutEvidence,
    StoreOwnedS6CertificationMaterializationSources,
};
use forge_store_contracts::S6QueueProducerResourceShape;
use forge_store_io_scheduler::foreground_reservation::admitted_point_read_reservation_for_certification_test;
use forge_store_io_scheduler::{
    admit_backend_capability_for_scheduler_claim, admit_queue_execution_plan,
    admit_s5_1_security_scope_for_s6_io_qos,
    admit_secure_frame_backend_capability_for_scheduler_claim, admit_secure_io_scope_for_scheduler,
    admit_store_published_s6_io_qos_isolation_readiness,
    background_pacing_outcome_for_later_readiness_certification_test, execute_ready_queue_plan,
    lower_buffer_pool_queue_declaration, publish_s10_backup_export_io_readiness_handoff,
    publish_s10_compaction_io_readiness_handoff, publish_s10_repair_scan_io_readiness_handoff,
    publish_s11_operator_io_readiness_handoff, publish_s7_placement_io_readiness_handoff,
    BackgroundIoPressureClass, BackgroundResourceBudget, BandwidthToken, CacheResidencyHint,
    IoSchedulerBackendCapabilityAdmission, QueueExecutionAdmissionRequest, QueueExecutionOutcome,
    QueueSlot, ReadAheadWindow, S10BackupExportPacingEvidence, S10CompactionPacingEvidence,
    S10RepairScanPacingEvidence, S6IoQosSecurityScopeHandoff, SecureIoOperation,
    SecureIoPostureRequirement, SecureIoPreservationReceipt, SecureIoPreservationRequest,
    WorkerPermit,
};
use forge_store_physical_backend::{
    AdmittedBackendCapabilityWitness, BackendCapabilityAdmissionRequest,
    BackendCapabilityEvidenceBasis, BackendCapabilitySupportSet, BackendMediaAssumptionSet,
    BackendQueueExecutionAdaptation, BackendQueueExecutionCompletion, BackendQueueExecutionPosture,
    BackendRebindTriggers, BackendTargetProfile, PhysicalBackendCapabilityAdmissionAuthority,
};
use forge_store_physical_certification::{
    s6_io_pressure_test_replay_bundle_for, PhysicalFaultEvidenceClass, PhysicalSimulationProfile,
    S6IoPressureHarnessEvidence, S6IoPressureHarnessScenario,
};
use forge_store_physical_isolation::publish_s6_io_qos_isolation_readiness_for_foreground_reservation_test;
use forge_store_readiness::{
    accept_s5_1_admitted_security_scope_readiness, S51SecurityScopeReadinessReservation,
};
use forge_store_security::admitted_store_internal_security_scope_for_s6_test;

pub fn sources() -> StoreOwnedS6CertificationMaterializationSources {
    sources_with_options(
        vec![durability_evidence::flush_row()],
        access_policy_evidence::access_policy_rows(),
    )
}

pub fn sources_without_flush_rows() -> StoreOwnedS6CertificationMaterializationSources {
    sources_with_options(Vec::new(), access_policy_evidence::access_policy_rows())
}

pub fn sources_without_access_policy_rows() -> StoreOwnedS6CertificationMaterializationSources {
    sources_with_options(vec![durability_evidence::flush_row()], Vec::new())
}

pub fn sources_without_post_admission_violations() -> StoreOwnedS6CertificationMaterializationSources
{
    sources_with_options(
        vec![durability_evidence::flush_row()],
        access_policy_evidence::access_policy_rows_without_violations(),
    )
}

fn sources_with_options(
    flush_rows: Vec<S6FlushDurabilityEvidenceRow>,
    access_policy_rows: Vec<S6AccessPolicyEvidenceRow>,
) -> StoreOwnedS6CertificationMaterializationSources {
    let witness = backend_witness();
    let foreground = admitted_point_read_reservation_for_certification_test();
    let background = background_pacing_outcome_for_later_readiness_certification_test(
        BackgroundIoPressureClass::RepairScan,
    );
    let queue_outcome = queue_outcome();
    let security = security_scope();
    let secure_io_preservation = secure_io_preservation(&security);
    let harness = harness_evidence();
    let closeout = S6IoPressureHarnessCloseoutEvidence::from_harness_evidence(harness.clone());
    let qualification = certify_s6_backend_qualification_matrix(
        qualification_residual_debt::matrix_with_required_residual_debt(&witness, &harness),
    )
    .unwrap();
    let later = later_handoffs();
    StoreOwnedS6CertificationMaterializationSources::from_bound_store_execution(
        witness,
        foreground,
        background,
        queue_outcome,
        secure_io_preservation,
        access_policy_rows,
        flush_rows,
        closeout,
        qualification,
        later,
        None,
    )
    .unwrap()
}

fn queue_outcome() -> QueueExecutionOutcome {
    let witness = queue_backend_witness();
    let reservation = admitted_point_read_reservation_for_certification_test();
    let budget = point_read_budget();
    let producer = BufferPoolQueueExecutionDeclaration::read_ahead(
        7,
        S6QueueProducerResourceShape::new()
            .with_queue_slots(budget.queue_slots())
            .with_bandwidth_tokens(budget.bandwidth_tokens())
            .with_read_ahead_windows(budget.read_ahead_window())
            .with_worker_permits(budget.worker_permits())
            .with_cache_residency_hints(budget.cache_residency_hints()),
    );
    let mut work = lower_buffer_pool_queue_declaration(producer, reservation).unwrap();
    let backend =
        admit_backend_capability_for_scheduler_claim(&witness, work.backend_requirement()).unwrap();
    let secure_io = admit_secure_io_scope_for_scheduler(
        SecureIoPreservationRequest::new(SecureIoOperation::ReadAhead, &security_scope(), &backend)
            .require_posture(SecureIoPostureRequirement::ScopePreserving),
    )
    .unwrap();
    work = work.with_secure_io_scope(secure_io);
    let plan = admit_queue_execution_plan(QueueExecutionAdmissionRequest::new(
        work,
        &backend,
        policy_receipt(budget),
    ))
    .unwrap();
    let posture = BackendQueueExecutionPosture::from_admitted_capability(
        &witness,
        BackendQueueExecutionAdaptation::None,
    )
    .unwrap();
    let completion = BackendQueueExecutionCompletion::for_certification(
        plan.backend_completion_binding()
            .backend_execution_binding(),
        posture,
    )
    .observe_queue_depth(1);
    execute_ready_queue_plan(plan, completion)
}

fn secure_io_preservation(
    security: &forge_store_io_scheduler::IoSchedulerS6SecurityScopeAdmission,
) -> SecureIoPreservationReceipt {
    admit_secure_io_scope_for_scheduler(
        SecureIoPreservationRequest::new(
            SecureIoOperation::VerificationPressure,
            security,
            &secure_frame_backend(security),
        )
        .require_posture(SecureIoPostureRequirement::SecureFrameCompatible),
    )
    .unwrap()
}

fn later_handoffs() -> forge_store_certification::S6LaterReadinessHandoffCertification {
    let readiness = scheduler_readiness();
    let security = security_scope();
    let backend = secure_frame_backend(&security);
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

fn harness_evidence() -> S6IoPressureHarnessEvidence {
    let scenario = S6IoPressureHarnessScenario::deterministic_read_under_repair_pressure()
        .with_fault_evidence_class(PhysicalFaultEvidenceClass::CertifiedBackend);
    let replay = s6_io_pressure_test_replay_bundle_for(
        scenario.clone(),
        PhysicalSimulationProfile::HardwareQualification,
    );
    S6IoPressureHarnessEvidence::from_replay_bundle(scenario, &replay).unwrap()
}

fn backend_witness() -> AdmittedBackendCapabilityWitness {
    PhysicalBackendCapabilityAdmissionAuthority::store_owned()
        .admit_backend_capability(BackendCapabilityAdmissionRequest::new(
            BackendTargetProfile::PosixFileFsyncDirSync,
            BackendCapabilityEvidenceBasis::certified_backend_profile(),
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

fn queue_backend_witness() -> AdmittedBackendCapabilityWitness {
    PhysicalBackendCapabilityAdmissionAuthority::store_owned()
        .admit_backend_capability(BackendCapabilityAdmissionRequest::new(
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
                .with_flush_ordering()
                .with_fdatasync_durability(),
            BackendRebindTriggers::kernel_filesystem_mount_firmware_and_backend(),
        ))
        .unwrap()
}

fn scheduler_readiness() -> forge_store_io_scheduler::IoSchedulerS6ReadinessAdmission {
    let readiness =
        publish_s6_io_qos_isolation_readiness_for_foreground_reservation_test(2, 1).unwrap();
    admit_store_published_s6_io_qos_isolation_readiness(&readiness).unwrap()
}

fn security_scope() -> forge_store_io_scheduler::IoSchedulerS6SecurityScopeAdmission {
    let readiness = accept_s5_1_admitted_security_scope_readiness(
        S51SecurityScopeReadinessReservation::io_qos(),
        admitted_store_internal_security_scope_for_s6_test(),
    );
    admit_s5_1_security_scope_for_s6_io_qos(
        S6IoQosSecurityScopeHandoff::from_s5_1_readiness(readiness).unwrap(),
    )
}

fn secure_frame_backend(
    security: &forge_store_io_scheduler::IoSchedulerS6SecurityScopeAdmission,
) -> IoSchedulerBackendCapabilityAdmission {
    admit_secure_frame_backend_capability_for_scheduler_claim(&backend_witness(), security).unwrap()
}

fn point_read_budget() -> BackgroundResourceBudget {
    BackgroundResourceBudget::new()
        .with_queue_slots(QueueSlot::new(1).unwrap())
        .with_bandwidth(BandwidthToken::bytes(4096).unwrap())
        .with_read_ahead(ReadAheadWindow::pages(1).unwrap())
        .with_worker_permits(WorkerPermit::new(1).unwrap())
        .with_cache_residency(CacheResidencyHint::frames(1).unwrap())
}

fn policy_receipt(
    budget: BackgroundResourceBudget,
) -> forge_foundational::FoundationalPolicyAdmissionReceipt {
    let claim = performance()
        .claim()
        .policy_admission()
        .boundary(FoundationalPerformanceBoundary::AuthoritativeExecution)
        .evidence_strength(FoundationalPerformanceEvidenceStrength::RuntimePolicyAdmission)
        .breadth_locality(FoundationalPerformanceBreadthLocalityPosture::DeltaBound)
        .access_pattern(FoundationalPerformanceAccessPatternPosture::PointLookup)
        .execution_temperature(FoundationalPerformanceExecutionTemperature::HotPath)
        .freshness_retention(FoundationalPerformanceFreshnessRetentionPosture::ExactBasisCurrent)
        .fallback_debt(FoundationalPerformanceFallbackDebtPosture::Verified)
        .include_work(FoundationalPerformanceWorkClass::ValidationPlanning)
        .exclude_work(FoundationalPerformanceWorkClass::SupportReportAssembly)
        .finish()
        .unwrap();
    performance()
        .policy_admission_receipt(claim)
        .budget_decision(
            FoundationalPerformanceBudgetKind::Breadth,
            (budget.queue_slots() + budget.worker_permits()) as u32,
            (budget.queue_slots() + budget.worker_permits()) as u32,
        )
        .budget_decision(
            FoundationalPerformanceBudgetKind::Density,
            (budget.bandwidth_tokens() + budget.cache_residency_hints()) as u32,
            (budget.bandwidth_tokens() + budget.cache_residency_hints()) as u32,
        )
        .budget_decision(
            FoundationalPerformanceBudgetKind::Locality,
            budget.read_ahead_window() as u32,
            budget.read_ahead_window() as u32,
        )
        .finish()
        .unwrap()
}
