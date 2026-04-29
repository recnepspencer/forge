use serde::{Deserialize, Serialize};

use super::cancellation::{CancelledResourceRequest, DeniedResourceCancellation};
use super::completion::{
    AdmittedResourceCompletion, CommittedResourceCompletionArtifact, DeniedResourceCompletion,
    RolledBackResourceCompletionArtifact, StagedDeniedResourceCompletionEffect,
    StagedResourceCompletionEffect,
};
use super::descriptor::ResourceDescriptorId;
use super::lifecycle::{
    ResourceLifecycleClass, ResourceLifecycleOrdinal, ResourceLifecycleTransition,
    ResourceOutputContinuity,
};
use super::proof::AdmittedResourceRequest;
use super::request::{ResourceNodeId, ResourceRequestHandle};
use super::retry::{AdmittedResourceRetry, DeniedResourceRetry, ScheduledResourceRetry};
use super::revalidation::{AdmittedResourceRevalidation, DeniedResourceRevalidation};
use super::supersession::ResourceSupersessionRecord;
use super::timeout::{DeniedResourceTimeout, TimedOutResourceRequest};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResourceBoundaryKind {
    DeclarationLowering,
    RequestAdmission,
    Cancellation,
    TimeoutAdmission,
    RetrySchedule,
    RetryAdmission,
    RevalidationAdmission,
    CompletionAdmission,
    CompletionBatchAdmission,
    CompletionStaging,
    CompletionDenialStaging,
    CompletionCommit,
    CompletionRollback,
    BranchRestore,
    ReplayReconstruction,
    SummaryRead,
    DiagnosticsExpansion,
    LifecycleRetentionCompaction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResourceCostPosture {
    Verified,
    Debt,
    DeniedFallback,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResourceDensityStrategy {
    NotApplicable,
    SparseIndexedLookup,
    BurstySortedDeduplicated,
    DenseSortedDeduplicated,
}

impl ResourceDensityStrategy {
    pub(crate) fn request_pressure(in_flight_width: u32) -> Self {
        if in_flight_width <= 1 {
            Self::SparseIndexedLookup
        } else if in_flight_width <= 8 {
            Self::BurstySortedDeduplicated
        } else {
            Self::DenseSortedDeduplicated
        }
    }

    pub(crate) fn scalar_completion() -> Self {
        Self::SparseIndexedLookup
    }

    pub(crate) fn completion_batch(input_width: u32, in_flight_width: u32) -> Self {
        if input_width <= 1 {
            Self::SparseIndexedLookup
        } else if input_width >= 4 && input_width >= in_flight_width.max(1) {
            Self::DenseSortedDeduplicated
        } else {
            Self::BurstySortedDeduplicated
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ResourceCostContractId(u64);

impl ResourceCostContractId {
    pub fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn get(self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceBoundaryPerformanceEnvelope {
    boundary: ResourceBoundaryKind,
    input_width: u32,
    lifecycle_transition_count: u32,
    admitted_count: u32,
    denied_count: u32,
    broad_scan_denial_count: u32,
    temporal_wake_footprint: u32,
    operational_allocation_count: u32,
    retained_history_allocation_count: u32,
    diagnostics_allocation_count: u32,
    facade_report_allocation_count: u32,
    density_strategy: ResourceDensityStrategy,
    cost_contract: ResourceCostContractId,
    cost_posture: ResourceCostPosture,
}

impl ResourceBoundaryPerformanceEnvelope {
    fn new(
        boundary: ResourceBoundaryKind,
        input_width: u32,
        lifecycle_transition_count: u32,
        admitted_count: u32,
        denied_count: u32,
        broad_scan_denial_count: u32,
        temporal_wake_footprint: u32,
        operational_allocation_count: u32,
        retained_history_allocation_count: u32,
        diagnostics_allocation_count: u32,
        facade_report_allocation_count: u32,
        density_strategy: ResourceDensityStrategy,
        cost_contract: ResourceCostContractId,
        cost_posture: ResourceCostPosture,
    ) -> Self {
        Self {
            boundary,
            input_width,
            lifecycle_transition_count,
            admitted_count,
            denied_count,
            broad_scan_denial_count,
            temporal_wake_footprint,
            operational_allocation_count,
            retained_history_allocation_count,
            diagnostics_allocation_count,
            facade_report_allocation_count,
            density_strategy,
            cost_contract,
            cost_posture,
        }
    }

    pub(crate) fn with_density_strategy(
        mut self,
        density_strategy: ResourceDensityStrategy,
    ) -> Self {
        self.density_strategy = density_strategy;
        self
    }

    pub(crate) fn declaration_lowering(input_width: u32) -> Self {
        Self::new(
            ResourceBoundaryKind::DeclarationLowering,
            input_width,
            input_width,
            input_width,
            0,
            0,
            0,
            input_width,
            0,
            0,
            1,
            ResourceDensityStrategy::NotApplicable,
            ResourceCostContractId::new(0),
            ResourceCostPosture::Verified,
        )
    }

    pub(crate) fn request_admission(
        admitted_count: u32,
        denied_count: u32,
        lifecycle_transition_count: u32,
    ) -> Self {
        Self::new(
            ResourceBoundaryKind::RequestAdmission,
            admitted_count.saturating_add(denied_count),
            lifecycle_transition_count,
            admitted_count,
            denied_count,
            0,
            0,
            admitted_count,
            lifecycle_transition_count,
            0,
            1,
            ResourceDensityStrategy::NotApplicable,
            ResourceCostContractId::new(1),
            ResourceCostPosture::Verified,
        )
    }

    pub(crate) fn completion_admission(
        admitted_count: u32,
        denied_count: u32,
        lifecycle_transition_count: u32,
    ) -> Self {
        Self::new(
            ResourceBoundaryKind::CompletionAdmission,
            admitted_count.saturating_add(denied_count),
            lifecycle_transition_count,
            admitted_count,
            denied_count,
            0,
            0,
            0,
            denied_count,
            0,
            1,
            ResourceDensityStrategy::NotApplicable,
            ResourceCostContractId::new(2),
            ResourceCostPosture::Verified,
        )
    }

    pub(crate) fn completion_batch_admission(
        input_width: u32,
        admitted_count: u32,
        denied_count: u32,
    ) -> Self {
        Self::new(
            ResourceBoundaryKind::CompletionBatchAdmission,
            input_width,
            admitted_count,
            admitted_count,
            denied_count,
            0,
            0,
            0,
            denied_count,
            0,
            admitted_count.saturating_add(denied_count),
            ResourceDensityStrategy::NotApplicable,
            ResourceCostContractId::new(12),
            ResourceCostPosture::Verified,
        )
    }

    pub(crate) fn completion_commit(lifecycle_transition_count: u32) -> Self {
        Self::new(
            ResourceBoundaryKind::CompletionCommit,
            1,
            lifecycle_transition_count,
            1,
            0,
            0,
            0,
            0,
            lifecycle_transition_count,
            0,
            1,
            ResourceDensityStrategy::NotApplicable,
            ResourceCostContractId::new(8),
            ResourceCostPosture::Verified,
        )
    }

    pub(crate) fn completion_staging() -> Self {
        Self::new(
            ResourceBoundaryKind::CompletionStaging,
            1,
            0,
            1,
            0,
            0,
            0,
            1,
            0,
            0,
            1,
            ResourceDensityStrategy::NotApplicable,
            ResourceCostContractId::new(9),
            ResourceCostPosture::Verified,
        )
    }

    pub(crate) fn replay_reconstruction(
        descriptor_width: u32,
        lifecycle_summary_width: u32,
        denied_completion_width: u32,
        in_flight_width: u32,
        retained_history_unavailable_count: u32,
    ) -> Self {
        Self::new(
            ResourceBoundaryKind::ReplayReconstruction,
            descriptor_width
                .saturating_add(lifecycle_summary_width)
                .saturating_add(denied_completion_width)
                .saturating_add(in_flight_width),
            lifecycle_summary_width,
            in_flight_width,
            denied_completion_width,
            retained_history_unavailable_count,
            0,
            0,
            0,
            descriptor_width
                .saturating_add(lifecycle_summary_width)
                .saturating_add(denied_completion_width)
                .saturating_add(in_flight_width),
            1,
            ResourceDensityStrategy::NotApplicable,
            ResourceCostContractId::new(14),
            ResourceCostPosture::Debt,
        )
    }

    pub(crate) fn summary_read() -> Self {
        Self::new(
            ResourceBoundaryKind::SummaryRead,
            1,
            0,
            1,
            0,
            0,
            0,
            0,
            0,
            0,
            1,
            ResourceDensityStrategy::NotApplicable,
            ResourceCostContractId::new(15),
            ResourceCostPosture::Verified,
        )
    }

    pub(crate) fn diagnostics_expansion(
        runtime_summary_width: u32,
        replay_reconstruction_width: u32,
        branch_restore_width: u32,
    ) -> Self {
        Self::new(
            ResourceBoundaryKind::DiagnosticsExpansion,
            runtime_summary_width
                .saturating_add(replay_reconstruction_width)
                .saturating_add(branch_restore_width),
            0,
            runtime_summary_width,
            0,
            replay_reconstruction_width,
            0,
            0,
            0,
            replay_reconstruction_width,
            1,
            ResourceDensityStrategy::NotApplicable,
            ResourceCostContractId::new(16),
            ResourceCostPosture::Debt,
        )
    }

    pub(crate) fn diagnostics_expansion_denied(
        runtime_summary_width: u32,
        replay_reconstruction_width: u32,
        branch_restore_width: u32,
    ) -> Self {
        Self::new(
            ResourceBoundaryKind::DiagnosticsExpansion,
            runtime_summary_width
                .saturating_add(replay_reconstruction_width)
                .saturating_add(branch_restore_width),
            0,
            0,
            1,
            replay_reconstruction_width,
            0,
            0,
            0,
            0,
            1,
            ResourceDensityStrategy::NotApplicable,
            ResourceCostContractId::new(16),
            ResourceCostPosture::DeniedFallback,
        )
    }

    pub(crate) fn lifecycle_retention_compaction(
        selected_terminal_count: u32,
        reclaimed_in_flight_count: u32,
        retained_history_write_count: u32,
    ) -> Self {
        Self::new(
            ResourceBoundaryKind::LifecycleRetentionCompaction,
            selected_terminal_count,
            0,
            reclaimed_in_flight_count,
            0,
            0,
            0,
            0,
            retained_history_write_count,
            0,
            1,
            ResourceDensityStrategy::NotApplicable,
            ResourceCostContractId::new(17),
            ResourceCostPosture::Verified,
        )
    }

    pub(crate) fn completion_denial_staging() -> Self {
        Self::new(
            ResourceBoundaryKind::CompletionDenialStaging,
            1,
            0,
            0,
            1,
            0,
            0,
            1,
            0,
            0,
            1,
            ResourceDensityStrategy::NotApplicable,
            ResourceCostContractId::new(10),
            ResourceCostPosture::Verified,
        )
    }

    pub(crate) fn completion_rollback(admitted_count: u32, denied_count: u32) -> Self {
        Self::new(
            ResourceBoundaryKind::CompletionRollback,
            admitted_count.saturating_add(denied_count),
            0,
            admitted_count,
            denied_count,
            0,
            0,
            0,
            0,
            0,
            1,
            ResourceDensityStrategy::NotApplicable,
            ResourceCostContractId::new(11),
            ResourceCostPosture::Verified,
        )
    }

    pub(crate) fn branch_restore(
        restored_in_flight_width: u32,
        retained_summary_width: u32,
        broad_rebuild_denial_count: u32,
    ) -> Self {
        Self::new(
            ResourceBoundaryKind::BranchRestore,
            restored_in_flight_width.saturating_add(retained_summary_width),
            restored_in_flight_width,
            restored_in_flight_width,
            0,
            broad_rebuild_denial_count,
            0,
            restored_in_flight_width,
            retained_summary_width,
            0,
            1,
            ResourceDensityStrategy::NotApplicable,
            ResourceCostContractId::new(13),
            ResourceCostPosture::Verified,
        )
    }

    pub(crate) fn cancellation(admitted_count: u32, denied_count: u32) -> Self {
        Self::new(
            ResourceBoundaryKind::Cancellation,
            admitted_count.saturating_add(denied_count),
            admitted_count,
            admitted_count,
            denied_count,
            0,
            0,
            0,
            admitted_count,
            0,
            1,
            ResourceDensityStrategy::NotApplicable,
            ResourceCostContractId::new(3),
            ResourceCostPosture::Verified,
        )
    }

    pub(crate) fn timeout_admission(
        admitted_count: u32,
        denied_count: u32,
        temporal_wake_footprint: u32,
    ) -> Self {
        Self::new(
            ResourceBoundaryKind::TimeoutAdmission,
            admitted_count.saturating_add(denied_count),
            admitted_count,
            admitted_count,
            denied_count,
            0,
            temporal_wake_footprint,
            0,
            admitted_count,
            0,
            1,
            ResourceDensityStrategy::NotApplicable,
            ResourceCostContractId::new(4),
            ResourceCostPosture::Verified,
        )
    }

    pub(crate) fn retry_schedule(admitted_count: u32, denied_count: u32) -> Self {
        Self::new(
            ResourceBoundaryKind::RetrySchedule,
            admitted_count.saturating_add(denied_count),
            0,
            admitted_count,
            denied_count,
            0,
            admitted_count,
            admitted_count,
            0,
            0,
            1,
            ResourceDensityStrategy::NotApplicable,
            ResourceCostContractId::new(5),
            ResourceCostPosture::Verified,
        )
    }

    pub(crate) fn retry_admission(
        admitted_count: u32,
        denied_count: u32,
        lifecycle_transition_count: u32,
        temporal_wake_footprint: u32,
    ) -> Self {
        Self::new(
            ResourceBoundaryKind::RetryAdmission,
            admitted_count.saturating_add(denied_count),
            lifecycle_transition_count,
            admitted_count,
            denied_count,
            0,
            temporal_wake_footprint,
            admitted_count,
            lifecycle_transition_count,
            0,
            1,
            ResourceDensityStrategy::NotApplicable,
            ResourceCostContractId::new(6),
            ResourceCostPosture::Verified,
        )
    }

    pub(crate) fn revalidation_admission(
        admitted_count: u32,
        denied_count: u32,
        lifecycle_transition_count: u32,
        temporal_wake_footprint: u32,
    ) -> Self {
        Self::new(
            ResourceBoundaryKind::RevalidationAdmission,
            admitted_count.saturating_add(denied_count),
            lifecycle_transition_count,
            admitted_count,
            denied_count,
            0,
            temporal_wake_footprint,
            admitted_count,
            lifecycle_transition_count,
            0,
            1,
            ResourceDensityStrategy::NotApplicable,
            ResourceCostContractId::new(7),
            ResourceCostPosture::Verified,
        )
    }

    pub fn boundary(self) -> ResourceBoundaryKind {
        self.boundary
    }

    pub fn input_width(self) -> u32 {
        self.input_width
    }

    pub fn admitted_count(self) -> u32 {
        self.admitted_count
    }

    pub fn lifecycle_transition_count(self) -> u32 {
        self.lifecycle_transition_count
    }

    pub fn denied_count(self) -> u32 {
        self.denied_count
    }

    pub fn broad_scan_denial_count(self) -> u32 {
        self.broad_scan_denial_count
    }

    pub fn temporal_wake_footprint(self) -> u32 {
        self.temporal_wake_footprint
    }

    pub fn operational_allocation_count(self) -> u32 {
        self.operational_allocation_count
    }

    pub fn retained_history_allocation_count(self) -> u32 {
        self.retained_history_allocation_count
    }

    pub fn diagnostics_allocation_count(self) -> u32 {
        self.diagnostics_allocation_count
    }

    pub fn facade_report_allocation_count(self) -> u32 {
        self.facade_report_allocation_count
    }

    pub fn density_strategy(self) -> ResourceDensityStrategy {
        self.density_strategy
    }

    pub fn cost_contract(self) -> ResourceCostContractId {
        self.cost_contract
    }

    pub fn cost_posture(self) -> ResourceCostPosture {
        self.cost_posture
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceRetryScheduleReport {
    scheduled_retry: Option<ScheduledResourceRetry>,
    denied_retry: Option<DeniedResourceRetry>,
    performance: ResourceBoundaryPerformanceEnvelope,
}

impl ResourceRetryScheduleReport {
    pub(crate) fn admitted(
        scheduled_retry: ScheduledResourceRetry,
        performance: ResourceBoundaryPerformanceEnvelope,
    ) -> Self {
        Self {
            scheduled_retry: Some(scheduled_retry),
            denied_retry: None,
            performance,
        }
    }

    pub(crate) fn denied(
        denied_retry: DeniedResourceRetry,
        performance: ResourceBoundaryPerformanceEnvelope,
    ) -> Self {
        Self {
            scheduled_retry: None,
            denied_retry: Some(denied_retry),
            performance,
        }
    }

    pub fn scheduled_retry(self) -> Option<ScheduledResourceRetry> {
        self.scheduled_retry
    }

    pub fn denied_retry(self) -> Option<DeniedResourceRetry> {
        self.denied_retry
    }

    pub fn performance(&self) -> ResourceBoundaryPerformanceEnvelope {
        self.performance
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceRetryAdmissionReport {
    admitted_retry: Option<AdmittedResourceRetry>,
    denied_retry: Option<DeniedResourceRetry>,
    lifecycle: Option<ResourceLifecycleSummary>,
    transition: Option<ResourceLifecycleTransition>,
    performance: ResourceBoundaryPerformanceEnvelope,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceRevalidationReport {
    admitted_revalidation: Option<AdmittedResourceRevalidation>,
    denied_revalidation: Option<DeniedResourceRevalidation>,
    lifecycle: Option<ResourceLifecycleSummary>,
    transition: Option<ResourceLifecycleTransition>,
    performance: ResourceBoundaryPerformanceEnvelope,
}

impl ResourceRevalidationReport {
    pub(crate) fn admitted(
        admitted_revalidation: AdmittedResourceRevalidation,
        lifecycle: ResourceLifecycleSummary,
        transition: ResourceLifecycleTransition,
        performance: ResourceBoundaryPerformanceEnvelope,
    ) -> Self {
        Self {
            admitted_revalidation: Some(admitted_revalidation),
            denied_revalidation: None,
            lifecycle: Some(lifecycle),
            transition: Some(transition),
            performance,
        }
    }

    pub(crate) fn denied(
        denied_revalidation: DeniedResourceRevalidation,
        performance: ResourceBoundaryPerformanceEnvelope,
    ) -> Self {
        Self {
            admitted_revalidation: None,
            denied_revalidation: Some(denied_revalidation),
            lifecycle: None,
            transition: None,
            performance,
        }
    }

    pub fn admitted_revalidation(self) -> Option<AdmittedResourceRevalidation> {
        self.admitted_revalidation
    }

    pub fn denied_revalidation(self) -> Option<DeniedResourceRevalidation> {
        self.denied_revalidation
    }

    pub fn lifecycle(self) -> Option<ResourceLifecycleSummary> {
        self.lifecycle
    }

    pub fn transition(self) -> Option<ResourceLifecycleTransition> {
        self.transition
    }

    pub fn performance(&self) -> ResourceBoundaryPerformanceEnvelope {
        self.performance
    }
}

impl ResourceRetryAdmissionReport {
    pub(crate) fn admitted(
        admitted_retry: AdmittedResourceRetry,
        lifecycle: ResourceLifecycleSummary,
        transition: ResourceLifecycleTransition,
        performance: ResourceBoundaryPerformanceEnvelope,
    ) -> Self {
        Self {
            admitted_retry: Some(admitted_retry),
            denied_retry: None,
            lifecycle: Some(lifecycle),
            transition: Some(transition),
            performance,
        }
    }

    pub(crate) fn denied(
        denied_retry: DeniedResourceRetry,
        performance: ResourceBoundaryPerformanceEnvelope,
    ) -> Self {
        Self {
            admitted_retry: None,
            denied_retry: Some(denied_retry),
            lifecycle: None,
            transition: None,
            performance,
        }
    }

    pub fn admitted_retry(&self) -> Option<&AdmittedResourceRetry> {
        self.admitted_retry.as_ref()
    }

    pub fn denied_retry(self) -> Option<DeniedResourceRetry> {
        self.denied_retry
    }

    pub fn lifecycle(&self) -> Option<ResourceLifecycleSummary> {
        self.lifecycle
    }

    pub fn transition(&self) -> Option<ResourceLifecycleTransition> {
        self.transition
    }

    pub fn performance(&self) -> ResourceBoundaryPerformanceEnvelope {
        self.performance
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceCancellationReport {
    cancelled_request: Option<CancelledResourceRequest>,
    denied_cancellation: Option<DeniedResourceCancellation>,
    lifecycle: Option<ResourceLifecycleSummary>,
    transition: Option<ResourceLifecycleTransition>,
    performance: ResourceBoundaryPerformanceEnvelope,
}

impl ResourceCancellationReport {
    pub(crate) fn admitted(
        cancelled_request: CancelledResourceRequest,
        lifecycle: ResourceLifecycleSummary,
        transition: ResourceLifecycleTransition,
        performance: ResourceBoundaryPerformanceEnvelope,
    ) -> Self {
        Self {
            cancelled_request: Some(cancelled_request),
            denied_cancellation: None,
            lifecycle: Some(lifecycle),
            transition: Some(transition),
            performance,
        }
    }

    pub(crate) fn denied(
        denied_cancellation: DeniedResourceCancellation,
        performance: ResourceBoundaryPerformanceEnvelope,
    ) -> Self {
        Self {
            cancelled_request: None,
            denied_cancellation: Some(denied_cancellation),
            lifecycle: None,
            transition: None,
            performance,
        }
    }

    pub fn cancelled_request(self) -> Option<CancelledResourceRequest> {
        self.cancelled_request
    }

    pub fn denied_cancellation(self) -> Option<DeniedResourceCancellation> {
        self.denied_cancellation
    }

    pub fn lifecycle(self) -> Option<ResourceLifecycleSummary> {
        self.lifecycle
    }

    pub fn transition(self) -> Option<ResourceLifecycleTransition> {
        self.transition
    }

    pub fn performance(&self) -> ResourceBoundaryPerformanceEnvelope {
        self.performance
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceTimeoutReport {
    timed_out_request: Option<TimedOutResourceRequest>,
    denied_timeout: Option<DeniedResourceTimeout>,
    lifecycle: Option<ResourceLifecycleSummary>,
    transition: Option<ResourceLifecycleTransition>,
    performance: ResourceBoundaryPerformanceEnvelope,
}

impl ResourceTimeoutReport {
    pub(crate) fn admitted(
        timed_out_request: TimedOutResourceRequest,
        lifecycle: ResourceLifecycleSummary,
        transition: ResourceLifecycleTransition,
        performance: ResourceBoundaryPerformanceEnvelope,
    ) -> Self {
        Self {
            timed_out_request: Some(timed_out_request),
            denied_timeout: None,
            lifecycle: Some(lifecycle),
            transition: Some(transition),
            performance,
        }
    }

    pub(crate) fn denied(
        denied_timeout: DeniedResourceTimeout,
        performance: ResourceBoundaryPerformanceEnvelope,
    ) -> Self {
        Self {
            timed_out_request: None,
            denied_timeout: Some(denied_timeout),
            lifecycle: None,
            transition: None,
            performance,
        }
    }

    pub fn timed_out_request(&self) -> Option<&TimedOutResourceRequest> {
        self.timed_out_request.as_ref()
    }

    pub fn denied_timeout(&self) -> Option<DeniedResourceTimeout> {
        self.denied_timeout
    }

    pub fn lifecycle(&self) -> Option<ResourceLifecycleSummary> {
        self.lifecycle
    }

    pub fn transition(&self) -> Option<ResourceLifecycleTransition> {
        self.transition
    }

    pub fn performance(&self) -> ResourceBoundaryPerformanceEnvelope {
        self.performance
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceCompletionAdmissionReport {
    admitted_completion: Option<AdmittedResourceCompletion>,
    denied_completion: Option<DeniedResourceCompletion>,
    performance: ResourceBoundaryPerformanceEnvelope,
}

impl ResourceCompletionAdmissionReport {
    pub(crate) fn admitted(
        admitted_completion: AdmittedResourceCompletion,
        performance: ResourceBoundaryPerformanceEnvelope,
    ) -> Self {
        Self {
            admitted_completion: Some(admitted_completion),
            denied_completion: None,
            performance,
        }
    }

    pub(crate) fn denied(
        denied_completion: DeniedResourceCompletion,
        performance: ResourceBoundaryPerformanceEnvelope,
    ) -> Self {
        Self {
            admitted_completion: None,
            denied_completion: Some(denied_completion),
            performance,
        }
    }

    pub fn admitted_completion(self) -> Option<AdmittedResourceCompletion> {
        self.admitted_completion
    }

    pub fn denied_completion(self) -> Option<DeniedResourceCompletion> {
        self.denied_completion
    }

    pub fn performance(&self) -> ResourceBoundaryPerformanceEnvelope {
        self.performance
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceCompletionBatchAdmissionReport {
    admitted_completions: Vec<AdmittedResourceCompletion>,
    denied_completions: Vec<DeniedResourceCompletion>,
    input_width: u32,
    deduplicated_width: u32,
    duplicate_width: u32,
    performance: ResourceBoundaryPerformanceEnvelope,
}

impl ResourceCompletionBatchAdmissionReport {
    pub(crate) fn new(
        admitted_completions: Vec<AdmittedResourceCompletion>,
        denied_completions: Vec<DeniedResourceCompletion>,
        input_width: u32,
        duplicate_width: u32,
        performance: ResourceBoundaryPerformanceEnvelope,
    ) -> Self {
        Self {
            admitted_completions,
            denied_completions,
            input_width,
            deduplicated_width: input_width.saturating_sub(duplicate_width),
            duplicate_width,
            performance,
        }
    }

    pub fn admitted_completions(&self) -> &[AdmittedResourceCompletion] {
        &self.admitted_completions
    }

    pub fn denied_completions(&self) -> &[DeniedResourceCompletion] {
        &self.denied_completions
    }

    pub fn into_parts(
        self,
    ) -> (
        Vec<AdmittedResourceCompletion>,
        Vec<DeniedResourceCompletion>,
    ) {
        (self.admitted_completions, self.denied_completions)
    }

    pub fn input_width(&self) -> u32 {
        self.input_width
    }

    pub fn deduplicated_width(&self) -> u32 {
        self.deduplicated_width
    }

    pub fn duplicate_width(&self) -> u32 {
        self.duplicate_width
    }

    pub fn performance(&self) -> ResourceBoundaryPerformanceEnvelope {
        self.performance
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceCompletionStagingReport {
    staged_effect: StagedResourceCompletionEffect,
    performance: ResourceBoundaryPerformanceEnvelope,
}

impl ResourceCompletionStagingReport {
    pub(crate) fn new(
        staged_effect: StagedResourceCompletionEffect,
        performance: ResourceBoundaryPerformanceEnvelope,
    ) -> Self {
        Self {
            staged_effect,
            performance,
        }
    }

    pub fn staged_effect(self) -> StagedResourceCompletionEffect {
        self.staged_effect
    }

    pub fn performance(&self) -> ResourceBoundaryPerformanceEnvelope {
        self.performance
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceCompletionDenialStagingReport {
    staged_denial_effect: StagedDeniedResourceCompletionEffect,
    performance: ResourceBoundaryPerformanceEnvelope,
}

impl ResourceCompletionDenialStagingReport {
    pub(crate) fn new(
        staged_denial_effect: StagedDeniedResourceCompletionEffect,
        performance: ResourceBoundaryPerformanceEnvelope,
    ) -> Self {
        Self {
            staged_denial_effect,
            performance,
        }
    }

    pub fn staged_denial_effect(self) -> StagedDeniedResourceCompletionEffect {
        self.staged_denial_effect
    }

    pub fn performance(&self) -> ResourceBoundaryPerformanceEnvelope {
        self.performance
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceCompletionCommitReport {
    committed_completion: CommittedResourceCompletionArtifact,
    lifecycle: ResourceLifecycleSummary,
    transition: ResourceLifecycleTransition,
    performance: ResourceBoundaryPerformanceEnvelope,
}

impl ResourceCompletionCommitReport {
    pub(crate) fn new(
        committed_completion: CommittedResourceCompletionArtifact,
        lifecycle: ResourceLifecycleSummary,
        transition: ResourceLifecycleTransition,
        performance: ResourceBoundaryPerformanceEnvelope,
    ) -> Self {
        Self {
            committed_completion,
            lifecycle,
            transition,
            performance,
        }
    }

    pub fn committed_completion(self) -> CommittedResourceCompletionArtifact {
        self.committed_completion
    }

    pub fn lifecycle(&self) -> ResourceLifecycleSummary {
        self.lifecycle
    }

    pub fn transition(&self) -> ResourceLifecycleTransition {
        self.transition
    }

    pub fn performance(&self) -> ResourceBoundaryPerformanceEnvelope {
        self.performance
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceCompletionRollbackReport {
    rolled_back_completion: RolledBackResourceCompletionArtifact,
    performance: ResourceBoundaryPerformanceEnvelope,
}

impl ResourceCompletionRollbackReport {
    pub(crate) fn new(
        rolled_back_completion: RolledBackResourceCompletionArtifact,
        performance: ResourceBoundaryPerformanceEnvelope,
    ) -> Self {
        Self {
            rolled_back_completion,
            performance,
        }
    }

    pub fn rolled_back_completion(self) -> RolledBackResourceCompletionArtifact {
        self.rolled_back_completion
    }

    pub fn performance(&self) -> ResourceBoundaryPerformanceEnvelope {
        self.performance
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceBranchRestoreReport {
    restored_in_flight_width: u32,
    retained_summary_width: u32,
    broad_rebuild_denial_count: u32,
    performance: ResourceBoundaryPerformanceEnvelope,
}

impl ResourceBranchRestoreReport {
    pub(crate) fn new(
        restored_in_flight_width: u32,
        retained_summary_width: u32,
        broad_rebuild_denial_count: u32,
        performance: ResourceBoundaryPerformanceEnvelope,
    ) -> Self {
        Self {
            restored_in_flight_width,
            retained_summary_width,
            broad_rebuild_denial_count,
            performance,
        }
    }

    pub fn restored_in_flight_width(self) -> u32 {
        self.restored_in_flight_width
    }

    pub fn retained_summary_width(self) -> u32 {
        self.retained_summary_width
    }

    pub fn broad_rebuild_denial_count(self) -> u32 {
        self.broad_rebuild_denial_count
    }

    pub fn performance(self) -> ResourceBoundaryPerformanceEnvelope {
        self.performance
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceReplayReconstructionReport {
    descriptor_width: u32,
    lifecycle_summary_width: u32,
    denied_completion_width: u32,
    in_flight_width: u32,
    retained_history_unavailable_count: u32,
    descriptor_digest: String,
    lifecycle_digest: String,
    denied_completion_digest: String,
    in_flight_digest: String,
    replay_digest: String,
    performance: ResourceBoundaryPerformanceEnvelope,
}

impl ResourceReplayReconstructionReport {
    pub(crate) fn new(
        descriptor_width: u32,
        lifecycle_summary_width: u32,
        denied_completion_width: u32,
        in_flight_width: u32,
        retained_history_unavailable_count: u32,
        descriptor_digest: String,
        lifecycle_digest: String,
        denied_completion_digest: String,
        in_flight_digest: String,
        replay_digest: String,
        performance: ResourceBoundaryPerformanceEnvelope,
    ) -> Self {
        Self {
            descriptor_width,
            lifecycle_summary_width,
            denied_completion_width,
            in_flight_width,
            retained_history_unavailable_count,
            descriptor_digest,
            lifecycle_digest,
            denied_completion_digest,
            in_flight_digest,
            replay_digest,
            performance,
        }
    }

    pub fn descriptor_width(&self) -> u32 {
        self.descriptor_width
    }

    pub fn lifecycle_summary_width(&self) -> u32 {
        self.lifecycle_summary_width
    }

    pub fn denied_completion_width(&self) -> u32 {
        self.denied_completion_width
    }

    pub fn in_flight_width(&self) -> u32 {
        self.in_flight_width
    }

    pub fn retained_history_unavailable_count(&self) -> u32 {
        self.retained_history_unavailable_count
    }

    pub fn descriptor_digest(&self) -> &str {
        &self.descriptor_digest
    }

    pub fn lifecycle_digest(&self) -> &str {
        &self.lifecycle_digest
    }

    pub fn denied_completion_digest(&self) -> &str {
        &self.denied_completion_digest
    }

    pub fn in_flight_digest(&self) -> &str {
        &self.in_flight_digest
    }

    pub fn replay_digest(&self) -> &str {
        &self.replay_digest
    }

    pub fn performance(&self) -> ResourceBoundaryPerformanceEnvelope {
        self.performance
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceLifecycleSummary {
    node: ResourceNodeId,
    lifecycle: ResourceLifecycleClass,
    output_continuity: ResourceOutputContinuity,
    lifecycle_ordinal: ResourceLifecycleOrdinal,
}

impl ResourceLifecycleSummary {
    pub(crate) fn new(
        node: ResourceNodeId,
        lifecycle: ResourceLifecycleClass,
        output_continuity: ResourceOutputContinuity,
        lifecycle_ordinal: ResourceLifecycleOrdinal,
    ) -> Self {
        Self {
            node,
            lifecycle,
            output_continuity,
            lifecycle_ordinal,
        }
    }

    pub fn node(self) -> ResourceNodeId {
        self.node
    }

    pub fn lifecycle(self) -> ResourceLifecycleClass {
        self.lifecycle
    }

    pub fn output_continuity(self) -> ResourceOutputContinuity {
        self.output_continuity
    }

    pub fn lifecycle_ordinal(self) -> ResourceLifecycleOrdinal {
        self.lifecycle_ordinal
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceDeclarationReport {
    descriptor_id: ResourceDescriptorId,
    lifecycle: ResourceLifecycleSummary,
    transition: ResourceLifecycleTransition,
    performance: ResourceBoundaryPerformanceEnvelope,
}

impl ResourceDeclarationReport {
    pub(crate) fn new(
        descriptor_id: ResourceDescriptorId,
        lifecycle: ResourceLifecycleSummary,
        transition: ResourceLifecycleTransition,
        performance: ResourceBoundaryPerformanceEnvelope,
    ) -> Self {
        Self {
            descriptor_id,
            lifecycle,
            transition,
            performance,
        }
    }

    pub fn descriptor_id(self) -> ResourceDescriptorId {
        self.descriptor_id
    }

    pub fn lifecycle(self) -> ResourceLifecycleSummary {
        self.lifecycle
    }

    pub fn transition(self) -> ResourceLifecycleTransition {
        self.transition
    }

    pub fn performance(self) -> ResourceBoundaryPerformanceEnvelope {
        self.performance
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceRequestAdmissionReport {
    admitted_request: AdmittedResourceRequest,
    lifecycle: ResourceLifecycleSummary,
    transition: ResourceLifecycleTransition,
    supersession_record: Option<ResourceSupersessionRecord>,
    superseded_request: Option<ResourceRequestHandle>,
    superseded_transition: Option<ResourceLifecycleTransition>,
    performance: ResourceBoundaryPerformanceEnvelope,
}

impl ResourceRequestAdmissionReport {
    pub(crate) fn new(
        admitted_request: AdmittedResourceRequest,
        lifecycle: ResourceLifecycleSummary,
        transition: ResourceLifecycleTransition,
        supersession_record: Option<ResourceSupersessionRecord>,
        performance: ResourceBoundaryPerformanceEnvelope,
    ) -> Self {
        Self {
            admitted_request,
            lifecycle,
            transition,
            supersession_record,
            superseded_request: supersession_record.map(ResourceSupersessionRecord::previous),
            superseded_transition: supersession_record
                .map(ResourceSupersessionRecord::lifecycle_transition),
            performance,
        }
    }

    pub fn admitted_request(self) -> AdmittedResourceRequest {
        self.admitted_request
    }

    pub fn lifecycle(self) -> ResourceLifecycleSummary {
        self.lifecycle
    }

    pub fn transition(self) -> ResourceLifecycleTransition {
        self.transition
    }

    pub fn supersession_record(self) -> Option<ResourceSupersessionRecord> {
        self.supersession_record
    }

    pub fn superseded_request(self) -> Option<ResourceRequestHandle> {
        self.superseded_request
    }

    pub fn superseded_transition(self) -> Option<ResourceLifecycleTransition> {
        self.superseded_transition
    }

    pub fn performance(self) -> ResourceBoundaryPerformanceEnvelope {
        self.performance
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ResourceRuntimeSummary {
    descriptor_count: u64,
    declared_resource_node_count: u64,
    in_flight_request_count: u64,
    active_in_flight_node_count: u64,
    retained_lifecycle_history_count: u64,
    denied_completion_count: u64,
    next_descriptor_id: ResourceDescriptorId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceRuntimeSummaryReadReport {
    summary: ResourceRuntimeSummary,
    performance: ResourceBoundaryPerformanceEnvelope,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceLifecycleRetentionCompactionReport {
    selected_terminal_count: u32,
    reclaimed_in_flight_count: u32,
    retained_history_write_count: u32,
    retained_history_pruned_count: u32,
    retained_history_width: u32,
    hot_in_flight_width: u32,
    performance: ResourceBoundaryPerformanceEnvelope,
}

impl ResourceLifecycleRetentionCompactionReport {
    pub(crate) fn new(
        selected_terminal_count: u32,
        reclaimed_in_flight_count: u32,
        retained_history_write_count: u32,
        retained_history_pruned_count: u32,
        retained_history_width: u32,
        hot_in_flight_width: u32,
        performance: ResourceBoundaryPerformanceEnvelope,
    ) -> Self {
        Self {
            selected_terminal_count,
            reclaimed_in_flight_count,
            retained_history_write_count,
            retained_history_pruned_count,
            retained_history_width,
            hot_in_flight_width,
            performance,
        }
    }

    pub fn selected_terminal_count(self) -> u32 {
        self.selected_terminal_count
    }

    pub fn reclaimed_in_flight_count(self) -> u32 {
        self.reclaimed_in_flight_count
    }

    pub fn retained_history_write_count(self) -> u32 {
        self.retained_history_write_count
    }

    pub fn retained_history_pruned_count(self) -> u32 {
        self.retained_history_pruned_count
    }

    pub fn retained_history_width(self) -> u32 {
        self.retained_history_width
    }

    pub fn hot_in_flight_width(self) -> u32 {
        self.hot_in_flight_width
    }

    pub fn performance(self) -> ResourceBoundaryPerformanceEnvelope {
        self.performance
    }
}

impl ResourceRuntimeSummaryReadReport {
    pub(crate) fn new(
        summary: ResourceRuntimeSummary,
        performance: ResourceBoundaryPerformanceEnvelope,
    ) -> Self {
        Self {
            summary,
            performance,
        }
    }

    pub fn summary(self) -> ResourceRuntimeSummary {
        self.summary
    }

    pub fn performance(self) -> ResourceBoundaryPerformanceEnvelope {
        self.performance
    }
}

impl ResourceRuntimeSummary {
    pub(crate) fn new(
        descriptor_count: usize,
        declared_resource_node_count: usize,
        in_flight_request_count: usize,
        active_in_flight_node_count: usize,
        retained_lifecycle_history_count: usize,
        denied_completion_count: usize,
        next_descriptor_id: ResourceDescriptorId,
    ) -> Self {
        Self {
            descriptor_count: descriptor_count as u64,
            declared_resource_node_count: declared_resource_node_count as u64,
            in_flight_request_count: in_flight_request_count as u64,
            active_in_flight_node_count: active_in_flight_node_count as u64,
            retained_lifecycle_history_count: retained_lifecycle_history_count as u64,
            denied_completion_count: denied_completion_count as u64,
            next_descriptor_id,
        }
    }

    pub fn descriptor_count(self) -> u64 {
        self.descriptor_count
    }

    pub fn declared_resource_node_count(self) -> u64 {
        self.declared_resource_node_count
    }

    pub fn in_flight_request_count(self) -> u64 {
        self.in_flight_request_count
    }

    pub fn active_in_flight_node_count(self) -> u64 {
        self.active_in_flight_node_count
    }

    pub fn retained_lifecycle_history_count(self) -> u64 {
        self.retained_lifecycle_history_count
    }

    pub fn denied_completion_count(self) -> u64 {
        self.denied_completion_count
    }

    pub fn next_descriptor_id(self) -> ResourceDescriptorId {
        self.next_descriptor_id
    }
}
