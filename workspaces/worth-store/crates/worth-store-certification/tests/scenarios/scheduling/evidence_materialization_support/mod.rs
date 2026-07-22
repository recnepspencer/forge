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
pub use source_variants::{
    sources_with_access_policy_backend_mismatch, sources_with_backend_evidence_class_mismatch,
    sources_with_backend_profile_mismatch, sources_with_empty_qualification_matrix,
};
use worth_foundational::{
    performance, FoundationalPerformanceAccessPatternPosture, FoundationalPerformanceBoundary,
    FoundationalPerformanceBreadthLocalityPosture, FoundationalPerformanceBudgetKind,
    FoundationalPerformanceEvidenceStrength, FoundationalPerformanceExecutionTemperature,
    FoundationalPerformanceFallbackDebtPosture, FoundationalPerformanceFreshnessRetentionPosture,
    FoundationalPerformanceWorkClass,
};

use worth_store_certification::{
    certify_io_pressure_backend_qualification_matrix, IoPressureHarnessCloseoutEvidence,
    S6AccessPolicyEvidenceRow, S6FlushDurabilityEvidenceRow,
    StoreOwnedS6CertificationMaterializationSources,
};
use worth_store_contracts::QueueProducerResourceShape;
use worth_store_io_scheduler::foreground_reservation::admitted_point_read_reservation_for_certification_test;
use worth_store_io_scheduler::{
    admit_backend_capability_for_scheduler_claim, admit_background_capacity,
    admit_background_pacing, admit_queue_execution_plan,
    admit_secure_frame_backend_capability_for_scheduler_claim, admit_secure_io_scope_for_scheduler,
    admit_security_scope_for_scheduler, admit_store_published_isolation_capability,
    execute_ready_queue_plan, lower_buffer_pool_queue_declaration,
    BackgroundCapacityAdmissionRequest, BackgroundIdleCapacityLeaseRequest,
    BackgroundIoPressureShape, BackgroundPacingOutcome, BackgroundResourceBudget, BandwidthToken,
    CacheResidencyHint, IoSchedulerBackendCapabilityAdmission, QueueExecutionAdmissionRequest,
    QueueExecutionOutcome, QueueSlot, ReadAheadWindow, SecureIoOperation,
    SecureIoPostureRequirement, SecureIoPreservationReceipt, SecureIoPreservationRequest,
    WorkerPermit,
};
use worth_store_physical_backend::{
    AdmittedBackendCapabilityWitness, BackendCapabilityAdmissionRequest,
    BackendCapabilityEvidenceBasis, BackendCapabilitySupportSet, BackendMediaAssumptionSet,
    BackendQueueExecutionAdaptation, BackendQueueExecutionCompletion, BackendQueueExecutionPosture,
    BackendRebindTriggers, BackendTargetProfile, PhysicalBackendCapabilityAdmissionAuthority,
};
use worth_store_physical_certification::{
    io_pressure_test_replay_bundle_for, IoPressureHarnessEvidence, IoPressureHarnessScenario,
    PhysicalFaultEvidenceClass, PhysicalSimulationProfile,
};
use worth_store_physical_isolation::publish_scheduler_isolation_capability_for_certification_test;
use worth_store_security::admitted_store_internal_security_scope_for_io_qos_test;

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
    let background = background_pacing_outcome();
    let queue_outcome = queue_outcome();
    let security = security_scope();
    let secure_io_preservation = secure_io_preservation(&security);
    let harness = harness_evidence();
    let closeout = IoPressureHarnessCloseoutEvidence::from_harness_evidence(harness.clone());
    let qualification = certify_io_pressure_backend_qualification_matrix(
        qualification_residual_debt::matrix_with_required_residual_debt(&witness, &harness),
    )
    .unwrap();
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
        None,
    )
    .unwrap()
}

fn queue_outcome() -> QueueExecutionOutcome {
    let witness = queue_backend_witness();
    let reservation = admitted_point_read_reservation_for_certification_test();
    let budget = point_read_budget();
    let producer = worth_store_test_support::read_ahead_declaration_for_real_pool(
        reservation.security_scope_identity(),
        7,
        QueueProducerResourceShape::new()
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
    security: &worth_store_io_scheduler::IoSchedulerSecurityScopeAdmission,
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

fn background_pacing_outcome() -> BackgroundPacingOutcome {
    let requested = BackgroundResourceBudget::new().with_queue_slots(QueueSlot::new(2).unwrap());
    let admitted = BackgroundResourceBudget::new().with_queue_slots(QueueSlot::new(1).unwrap());
    let pressure = BackgroundIoPressureShape::repair_scan().requesting(requested);
    let foreground = admitted_point_read_reservation_for_certification_test();
    let readiness = scheduler_readiness();
    let security = security_scope();
    let backend = admit_backend_capability_for_scheduler_claim(
        &backend_witness(),
        pressure.backend_requirement(),
    )
    .unwrap();
    let secure_io = admit_secure_io_scope_for_scheduler(
        SecureIoPreservationRequest::new(SecureIoOperation::RepairScan, &security, &backend)
            .require_posture(SecureIoPostureRequirement::ScopePreserving),
    )
    .unwrap();
    let capacity = admit_background_capacity(
        BackgroundCapacityAdmissionRequest::new(
            pressure,
            &foreground,
            &backend,
            &readiness,
            policy_receipt(requested),
        )
        .with_idle_available(admitted)
        .with_policy_admitted(requested)
        .with_debt_limit(admitted)
        .with_secure_io_scope(secure_io),
    )
    .unwrap();
    admit_background_pacing(
        BackgroundIdleCapacityLeaseRequest::new(capacity)
            .with_foreground_pressure_events(1)
            .with_late_yield(),
    )
}

fn harness_evidence() -> IoPressureHarnessEvidence {
    let scenario = IoPressureHarnessScenario::deterministic_read_under_repair_pressure()
        .with_fault_evidence_class(PhysicalFaultEvidenceClass::CertifiedBackend);
    let replay = io_pressure_test_replay_bundle_for(
        scenario.clone(),
        PhysicalSimulationProfile::HardwareQualification,
    );
    IoPressureHarnessEvidence::from_replay_bundle(scenario, &replay).unwrap()
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

fn scheduler_readiness() -> worth_store_io_scheduler::IoSchedulerIsolationAdmission {
    let readiness = publish_scheduler_isolation_capability_for_certification_test(2, 1).unwrap();
    admit_store_published_isolation_capability(&readiness).unwrap()
}

fn security_scope() -> worth_store_io_scheduler::IoSchedulerSecurityScopeAdmission {
    admit_security_scope_for_scheduler(&admitted_store_internal_security_scope_for_io_qos_test())
        .unwrap()
}

fn secure_frame_backend(
    security: &worth_store_io_scheduler::IoSchedulerSecurityScopeAdmission,
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
) -> worth_foundational::FoundationalPolicyAdmissionReceipt {
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
    let receipt = performance()
        .policy_admission_receipt(claim)
        .budget_decision(
            FoundationalPerformanceBudgetKind::Breadth,
            (budget.queue_slots() + budget.worker_permits()) as u32,
            (budget.queue_slots() + budget.worker_permits()) as u32,
        );
    let density = (budget.bandwidth_tokens() + budget.cache_residency_hints()) as u32;
    let receipt = if density == 0 {
        receipt
    } else {
        receipt.budget_decision(FoundationalPerformanceBudgetKind::Density, density, density)
    };
    let locality = budget.read_ahead_window() as u32;
    let receipt = if locality == 0 {
        receipt
    } else {
        receipt.budget_decision(
            FoundationalPerformanceBudgetKind::Locality,
            locality,
            locality,
        )
    };
    receipt.finish().unwrap()
}
