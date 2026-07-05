use forge_foundational::{
    performance, FoundationalPerformanceAccessPatternPosture, FoundationalPerformanceBoundary,
    FoundationalPerformanceBreadthLocalityPosture, FoundationalPerformanceBudgetKind,
    FoundationalPerformanceEvidenceStrength, FoundationalPerformanceExecutionTemperature,
    FoundationalPerformanceFallbackDebtPosture, FoundationalPerformanceFreshnessRetentionPosture,
    FoundationalPerformanceWorkClass,
};
use forge_store_physical_backend::{
    BackendCapabilityAdmissionRequest, BackendCapabilityEvidenceBasis, BackendCapabilitySupportSet,
    BackendMediaAssumptionSet, BackendQueueExecutionAdaptation, BackendQueueExecutionBackpressure,
    BackendQueueExecutionCompletion, BackendQueueExecutionPlanBinding,
    BackendQueueExecutionPosture, BackendQueueSpeculativeScope, BackendRebindTriggers,
    BackendTargetProfile, PhysicalBackendCapabilityAdmissionAuthority,
};

use crate::foreground_reservation::admitted_point_read_reservation_for_certification_test;
use crate::{
    admit_backend_capability_for_scheduler_claim, admit_queue_execution_plan,
    admit_s5_1_security_scope_for_s6_io_qos, admit_secure_io_scope_for_scheduler,
    BackgroundResourceBudget, BandwidthToken, CacheResidencyHint,
    IoSchedulerBackendCapabilityAdmission, QueueExecutionAdmissionRequest, QueueExecutionReadyPlan,
    QueueGroupingBasis, QueueRecoveryOrdering, QueueSlot, QueueWorkDeclaration,
    QueueWritebackPolicy, ReadAheadWindow, S6IoQosSecurityScopeHandoff, S6QueueDurabilityClass,
    SecureIoOperation, SecureIoPostureRequirement, SecureIoPreservationRequest, WorkerPermit,
};

pub(crate) fn admitted_plan() -> QueueExecutionReadyPlan {
    admitted_plan_for_backend_profile(BackendTargetProfile::PosixFileFsyncDirSync)
}

pub(crate) fn admitted_plan_for_backend_profile(
    profile: BackendTargetProfile,
) -> QueueExecutionReadyPlan {
    let reservation = admitted_point_read_reservation_for_certification_test();
    let budget = point_read_budget();
    let work = QueueWorkDeclaration::foreground(
        reservation.execution_ready(),
        S6QueueDurabilityClass::ReadOnly,
        budget,
    )
    .with_grouping_basis(grouping_for(reservation.security_scope_identity()));
    let backend = backend_for_profile(work, profile);
    let work = work.with_secure_io_scope(secure_io_for_work(work, &backend));
    admit_queue_execution_plan(QueueExecutionAdmissionRequest::new(
        work,
        &backend,
        policy_receipt(budget),
    ))
    .expect("test plan should admit")
}

pub(crate) fn secure_io_for_work(
    work: QueueWorkDeclaration,
    backend: &IoSchedulerBackendCapabilityAdmission,
) -> crate::SecureIoPreservationReceipt {
    let operation = secure_operation_for_test_work(work);
    admit_secure_io_scope_for_scheduler(
        SecureIoPreservationRequest::new(operation, &s6_security_scope_admission(), backend)
            .require_posture(SecureIoPostureRequirement::ScopePreserving),
    )
    .expect("test secure-I/O scope should admit")
}

fn secure_operation_for_test_work(work: QueueWorkDeclaration) -> SecureIoOperation {
    match work.class() {
        crate::QueueWorkClass::Background(crate::BackgroundIoPressureClass::RepairScan) => {
            SecureIoOperation::RepairScan
        }
        crate::QueueWorkClass::Background(
            crate::BackgroundIoPressureClass::VerificationPressure,
        ) => SecureIoOperation::VerificationPressure,
        crate::QueueWorkClass::Background(_) => SecureIoOperation::BackgroundLease,
        _ if work.requested_budget().read_ahead_window() > 0 => SecureIoOperation::ReadAhead,
        _ if work.requested_budget().write_back_window() > 0 => SecureIoOperation::WriteBack,
        crate::QueueWorkClass::Foreground(_) => SecureIoOperation::BatchedWrite,
    }
}

fn s6_security_scope_admission() -> crate::IoSchedulerS6SecurityScopeAdmission {
    let readiness = forge_store_readiness::accept_s5_1_admitted_security_scope_readiness(
        forge_store_readiness::S51SecurityScopeReadinessReservation::io_qos(),
        forge_store_security::admitted_store_internal_security_scope_for_s6_test(),
    );
    let handoff = S6IoQosSecurityScopeHandoff::from_s5_1_readiness(readiness)
        .expect("test S.5.1 readiness should hand off to S.6");
    admit_s5_1_security_scope_for_s6_io_qos(handoff)
}

pub(crate) fn grouping_for(
    security_scope_identity: forge_store_security::StoreSecurityScopeIdentity,
) -> QueueGroupingBasis {
    QueueGroupingBasis::new(
        security_scope_identity,
        security_scope_identity.tenant_scope(),
        security_scope_identity.key_scope(),
        security_scope_identity.authenticity_requirement(),
        S6QueueDurabilityClass::ReadOnly,
        7,
        crate::QueueWorkClass::Foreground(
            crate::foreground_reservation::ForegroundIoLaneKind::PointRead,
        ),
        QueueRecoveryOrdering::NotRecoveryCritical,
        QueueWritebackPolicy::None,
    )
}

pub(crate) trait GroupingTestMutation {
    fn with_different_durability_for_test(self) -> Self;
}

impl GroupingTestMutation for QueueGroupingBasis {
    fn with_different_durability_for_test(self) -> Self {
        QueueGroupingBasis::new(
            self.security_scope_identity(),
            self.tenant_scope(),
            self.key_scope(),
            self.authenticity_requirement(),
            S6QueueDurabilityClass::PlatformDurable,
            self.flush_epoch(),
            self.work_class(),
            self.recovery_ordering(),
            self.writeback_policy(),
        )
    }
}

pub(crate) fn point_read_budget() -> BackgroundResourceBudget {
    BackgroundResourceBudget::new()
        .with_queue_slots(QueueSlot::new(1).unwrap())
        .with_bandwidth(BandwidthToken::bytes(4096).unwrap())
        .with_read_ahead(ReadAheadWindow::pages(1).unwrap())
        .with_worker_permits(WorkerPermit::new(1).unwrap())
        .with_cache_residency(CacheResidencyHint::frames(1).unwrap())
}

pub(crate) fn backend_for(work: QueueWorkDeclaration) -> IoSchedulerBackendCapabilityAdmission {
    backend_for_profile(work, BackendTargetProfile::PosixFileFsyncDirSync)
}

pub(crate) fn backend_for_profile(
    work: QueueWorkDeclaration,
    profile: BackendTargetProfile,
) -> IoSchedulerBackendCapabilityAdmission {
    admit_backend_capability_for_scheduler_claim(
        &backend_witness_for_profile(profile),
        work.backend_requirement(),
    )
    .expect("backend should admit for test queue work")
}

pub(crate) fn speculative_scope(plan: &QueueExecutionReadyPlan) -> BackendQueueSpeculativeScope {
    BackendQueueSpeculativeScope::admitted(
        plan.grouping_basis().security_scope_identity(),
        plan.grouping_basis().tenant_scope(),
        plan.grouping_basis().key_scope(),
    )
}

pub(crate) struct TestBackendQueueCompletionBuilder {
    binding: BackendQueueExecutionPlanBinding,
    read_ahead_units: u64,
    read_ahead_scope: Option<BackendQueueSpeculativeScope>,
    write_back_units: u64,
    write_back_scope: Option<BackendQueueSpeculativeScope>,
    queue_depth_sample: u32,
    mechanical_retries: u64,
    partial_read_events: u64,
    short_write_events: u64,
    backpressure: Option<BackendQueueExecutionBackpressure>,
    foreground_wait_events: u64,
}

pub(crate) fn completion_for_plan(
    plan: &QueueExecutionReadyPlan,
    read_ahead_units: u64,
    read_ahead_scope: Option<BackendQueueSpeculativeScope>,
    write_back_units: u64,
    write_back_scope: Option<BackendQueueSpeculativeScope>,
) -> TestBackendQueueCompletionBuilder {
    completion_for_binding(
        plan.backend_completion_binding()
            .backend_execution_binding(),
        read_ahead_units,
        read_ahead_scope,
        write_back_units,
        write_back_scope,
    )
}

pub(crate) fn completion_for_group(
    grouped: &crate::QueueGroupedReadyPlans,
    read_ahead_units: u64,
    read_ahead_scope: Option<BackendQueueSpeculativeScope>,
    write_back_units: u64,
    write_back_scope: Option<BackendQueueSpeculativeScope>,
) -> TestBackendQueueCompletionBuilder {
    completion_for_binding(
        grouped
            .backend_completion_binding()
            .backend_execution_binding(),
        read_ahead_units,
        read_ahead_scope,
        write_back_units,
        write_back_scope,
    )
}

pub(crate) fn completion_for_binding(
    binding: BackendQueueExecutionPlanBinding,
    read_ahead_units: u64,
    read_ahead_scope: Option<BackendQueueSpeculativeScope>,
    write_back_units: u64,
    write_back_scope: Option<BackendQueueSpeculativeScope>,
) -> TestBackendQueueCompletionBuilder {
    TestBackendQueueCompletionBuilder {
        binding,
        read_ahead_units,
        read_ahead_scope,
        write_back_units,
        write_back_scope,
        queue_depth_sample: 1,
        mechanical_retries: 0,
        partial_read_events: 0,
        short_write_events: 0,
        backpressure: None,
        foreground_wait_events: 0,
    }
}

impl TestBackendQueueCompletionBuilder {
    pub(crate) const fn observe_queue_depth(mut self, queue_depth_sample: u32) -> Self {
        self.queue_depth_sample = queue_depth_sample;
        self
    }

    pub(crate) const fn observe_mechanical_adaptation(
        mut self,
        retries: u64,
        partial_reads: u64,
        short_writes: u64,
    ) -> Self {
        self.mechanical_retries = retries;
        self.partial_read_events = partial_reads;
        self.short_write_events = short_writes;
        self
    }

    pub(crate) const fn observe_backpressure(
        mut self,
        backpressure: BackendQueueExecutionBackpressure,
    ) -> Self {
        self.backpressure = Some(backpressure);
        self
    }

    pub(crate) const fn observe_foreground_wait_events(
        mut self,
        foreground_wait_events: u64,
    ) -> Self {
        self.foreground_wait_events = foreground_wait_events;
        self
    }

    pub(crate) fn complete(self) -> BackendQueueExecutionCompletion {
        let witness = backend_witness_for_profile(self.binding.backend_profile());
        let posture = BackendQueueExecutionPosture::from_admitted_capability(
            &witness,
            BackendQueueExecutionAdaptation::None,
        )
        .expect("test backend posture should admit");
        let mut completion =
            BackendQueueExecutionCompletion::for_certification(self.binding, posture)
                .observe_queue_depth(self.queue_depth_sample)
                .observe_mechanical_adaptation(
                    self.mechanical_retries,
                    self.partial_read_events,
                    self.short_write_events,
                )
                .observe_foreground_wait_events(self.foreground_wait_events);
        if let Some(scope) = self.read_ahead_scope {
            completion = completion.observe_read_ahead(self.read_ahead_units, scope);
        }
        if let Some(scope) = self.write_back_scope {
            completion = completion.observe_write_back(self.write_back_units, scope);
        }
        if let Some(backpressure) = self.backpressure {
            completion = completion.observe_backpressure(backpressure);
        }
        completion
    }
}

pub(crate) fn policy_receipt(
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
        .expect("policy claim should build");
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
            (budget.read_ahead_window() + budget.write_back_window() + budget.reclaim_permits())
                as u32,
            (budget.read_ahead_window() + budget.write_back_window() + budget.reclaim_permits())
                as u32,
        )
        .finish()
        .expect("policy receipt should build")
}

fn backend_witness_for_profile(
    profile: BackendTargetProfile,
) -> forge_store_physical_backend::AdmittedBackendCapabilityWitness {
    let request = BackendCapabilityAdmissionRequest::new(
        profile,
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
    );
    PhysicalBackendCapabilityAdmissionAuthority::store_owned()
        .admit_backend_capability(request)
        .expect("backend witness should admit")
}
