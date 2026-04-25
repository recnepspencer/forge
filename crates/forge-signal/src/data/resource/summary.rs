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
    CompletionStaging,
    CompletionDenialStaging,
    CompletionCommit,
    CompletionRollback,
    SummaryRead,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResourceCostPosture {
    Verified,
    Debt,
    DeniedFallback,
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
    cost_contract: ResourceCostContractId,
    cost_posture: ResourceCostPosture,
}

impl ResourceBoundaryPerformanceEnvelope {
    pub(crate) fn declaration_lowering(input_width: u32) -> Self {
        Self {
            boundary: ResourceBoundaryKind::DeclarationLowering,
            input_width,
            lifecycle_transition_count: input_width,
            admitted_count: input_width,
            denied_count: 0,
            broad_scan_denial_count: 0,
            temporal_wake_footprint: 0,
            cost_contract: ResourceCostContractId::new(0),
            cost_posture: ResourceCostPosture::Verified,
        }
    }

    pub(crate) fn request_admission(
        admitted_count: u32,
        denied_count: u32,
        lifecycle_transition_count: u32,
    ) -> Self {
        Self {
            boundary: ResourceBoundaryKind::RequestAdmission,
            input_width: admitted_count.saturating_add(denied_count),
            lifecycle_transition_count,
            admitted_count,
            denied_count,
            broad_scan_denial_count: 0,
            temporal_wake_footprint: 0,
            cost_contract: ResourceCostContractId::new(1),
            cost_posture: ResourceCostPosture::Verified,
        }
    }

    pub(crate) fn completion_admission(
        admitted_count: u32,
        denied_count: u32,
        lifecycle_transition_count: u32,
    ) -> Self {
        Self {
            boundary: ResourceBoundaryKind::CompletionAdmission,
            input_width: admitted_count.saturating_add(denied_count),
            lifecycle_transition_count,
            admitted_count,
            denied_count,
            broad_scan_denial_count: 0,
            temporal_wake_footprint: 0,
            cost_contract: ResourceCostContractId::new(2),
            cost_posture: ResourceCostPosture::Verified,
        }
    }

    pub(crate) fn completion_commit(lifecycle_transition_count: u32) -> Self {
        Self {
            boundary: ResourceBoundaryKind::CompletionCommit,
            input_width: 1,
            lifecycle_transition_count,
            admitted_count: 1,
            denied_count: 0,
            broad_scan_denial_count: 0,
            temporal_wake_footprint: 0,
            cost_contract: ResourceCostContractId::new(8),
            cost_posture: ResourceCostPosture::Verified,
        }
    }

    pub(crate) fn completion_staging() -> Self {
        Self {
            boundary: ResourceBoundaryKind::CompletionStaging,
            input_width: 1,
            lifecycle_transition_count: 0,
            admitted_count: 1,
            denied_count: 0,
            broad_scan_denial_count: 0,
            temporal_wake_footprint: 0,
            cost_contract: ResourceCostContractId::new(9),
            cost_posture: ResourceCostPosture::Verified,
        }
    }

    pub(crate) fn completion_denial_staging() -> Self {
        Self {
            boundary: ResourceBoundaryKind::CompletionDenialStaging,
            input_width: 1,
            lifecycle_transition_count: 0,
            admitted_count: 0,
            denied_count: 1,
            broad_scan_denial_count: 0,
            temporal_wake_footprint: 0,
            cost_contract: ResourceCostContractId::new(10),
            cost_posture: ResourceCostPosture::Verified,
        }
    }

    pub(crate) fn completion_rollback(admitted_count: u32, denied_count: u32) -> Self {
        Self {
            boundary: ResourceBoundaryKind::CompletionRollback,
            input_width: admitted_count.saturating_add(denied_count),
            lifecycle_transition_count: 0,
            admitted_count,
            denied_count,
            broad_scan_denial_count: 0,
            temporal_wake_footprint: 0,
            cost_contract: ResourceCostContractId::new(11),
            cost_posture: ResourceCostPosture::Verified,
        }
    }

    pub(crate) fn cancellation(admitted_count: u32, denied_count: u32) -> Self {
        Self {
            boundary: ResourceBoundaryKind::Cancellation,
            input_width: admitted_count.saturating_add(denied_count),
            lifecycle_transition_count: admitted_count,
            admitted_count,
            denied_count,
            broad_scan_denial_count: 0,
            temporal_wake_footprint: 0,
            cost_contract: ResourceCostContractId::new(3),
            cost_posture: ResourceCostPosture::Verified,
        }
    }

    pub(crate) fn timeout_admission(
        admitted_count: u32,
        denied_count: u32,
        temporal_wake_footprint: u32,
    ) -> Self {
        Self {
            boundary: ResourceBoundaryKind::TimeoutAdmission,
            input_width: admitted_count.saturating_add(denied_count),
            lifecycle_transition_count: admitted_count,
            admitted_count,
            denied_count,
            broad_scan_denial_count: 0,
            temporal_wake_footprint,
            cost_contract: ResourceCostContractId::new(4),
            cost_posture: ResourceCostPosture::Verified,
        }
    }

    pub(crate) fn retry_schedule(admitted_count: u32, denied_count: u32) -> Self {
        Self {
            boundary: ResourceBoundaryKind::RetrySchedule,
            input_width: admitted_count.saturating_add(denied_count),
            lifecycle_transition_count: 0,
            admitted_count,
            denied_count,
            broad_scan_denial_count: 0,
            temporal_wake_footprint: admitted_count,
            cost_contract: ResourceCostContractId::new(5),
            cost_posture: ResourceCostPosture::Verified,
        }
    }

    pub(crate) fn retry_admission(
        admitted_count: u32,
        denied_count: u32,
        lifecycle_transition_count: u32,
        temporal_wake_footprint: u32,
    ) -> Self {
        Self {
            boundary: ResourceBoundaryKind::RetryAdmission,
            input_width: admitted_count.saturating_add(denied_count),
            lifecycle_transition_count,
            admitted_count,
            denied_count,
            broad_scan_denial_count: 0,
            temporal_wake_footprint,
            cost_contract: ResourceCostContractId::new(6),
            cost_posture: ResourceCostPosture::Verified,
        }
    }

    pub(crate) fn revalidation_admission(
        admitted_count: u32,
        denied_count: u32,
        lifecycle_transition_count: u32,
        temporal_wake_footprint: u32,
    ) -> Self {
        Self {
            boundary: ResourceBoundaryKind::RevalidationAdmission,
            input_width: admitted_count.saturating_add(denied_count),
            lifecycle_transition_count,
            admitted_count,
            denied_count,
            broad_scan_denial_count: 0,
            temporal_wake_footprint,
            cost_contract: ResourceCostContractId::new(7),
            cost_posture: ResourceCostPosture::Verified,
        }
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
    denied_completion_count: u64,
    next_descriptor_id: ResourceDescriptorId,
}

impl ResourceRuntimeSummary {
    pub(crate) fn new(
        descriptor_count: usize,
        declared_resource_node_count: usize,
        in_flight_request_count: usize,
        active_in_flight_node_count: usize,
        denied_completion_count: usize,
        next_descriptor_id: ResourceDescriptorId,
    ) -> Self {
        Self {
            descriptor_count: descriptor_count as u64,
            declared_resource_node_count: declared_resource_node_count as u64,
            in_flight_request_count: in_flight_request_count as u64,
            active_in_flight_node_count: active_in_flight_node_count as u64,
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

    pub fn denied_completion_count(self) -> u64 {
        self.denied_completion_count
    }

    pub fn next_descriptor_id(self) -> ResourceDescriptorId {
        self.next_descriptor_id
    }
}
