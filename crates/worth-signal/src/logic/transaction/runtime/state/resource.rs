use std::collections::{BTreeMap, BTreeSet};

use serde::Serialize;

use super::merge::canonical_digest;
use super::resource_retry_budget::{ResourceRetryBudgetCharge, ResourceRetryBudgetLedger};
use crate::data::node::NodeState;
use crate::data::resource::{
    ActiveResourceRevalidationProof, AdmittedResourceCompletion, AdmittedResourceRequest,
    AdmittedResourceRetry, AdmittedResourceRevalidation, AsyncDenialId, CancelledResourceRequest,
    CommittedResourceCompletionArtifact, CompletionDenialClass, DeniedResourceCancellation,
    DeniedResourceCompletion, DeniedResourcePolicyRestoreCompatibility, DeniedResourceRejection,
    DeniedResourceRetry, DeniedResourceRevalidation, DeniedResourceTimeout,
    DeniedResourceTimeoutHeartbeatExtension, DependencyChangeResourceRevalidationProof,
    ExtendedResourceTimeoutHeartbeat, FrozenResourcePolicyDescriptorSet,
    FrozenResourcePolicyRegistry, FulfilledLifecycleResourceRevalidationProof,
    InFlightResourceRequest, LoweredResourceDescriptor, LoweredResourcePolicyBundle,
    ObservedResourceNodeState, ObserverDemandResourceRevalidationProof, RawCompletionEnvelope,
    RejectedResourceRequest, ResourceAttemptId, ResourceBoundaryPerformanceEnvelope,
    ResourceBranchEpoch, ResourceBranchRestoreReport, ResourceCancellationDenialClass,
    ResourceCancellationGraceWindow, ResourceCancellationOrdinal, ResourceCancellationReason,
    ResourceCancellationReport, ResourceCompletionAdmissionReport,
    ResourceCompletionBatchAdmissionReport, ResourceCompletionCommitReport,
    ResourceCompletionDenialStagingReport, ResourceCompletionOrdinal,
    ResourceCompletionRollbackReport, ResourceCompletionStagingReport, ResourceDeclarationReport,
    ResourceDensityStrategy, ResourceDependentCancellationPropagation, ResourceDescriptorId,
    ResourceDescriptorVersion, ResourceDiagnosticsDecisionClass, ResourceGeneration,
    ResourceHostCancellationAdvisory, ResourceInFlightStatus, ResourceIntentEquivalenceCoalescing,
    ResourceLifecycleClass, ResourceLifecycleOrdinal, ResourceLifecycleRetentionCompactionReport,
    ResourceLifecycleSummary, ResourceLifecycleTransition, ResourceLifecycleTransitionKind,
    ResourceManagedQueueBinding, ResourceManagedQueueCounters, ResourceManagedQueueDenial,
    ResourceManagedQueueDenialClass, ResourceManagedQueueMutationKind,
    ResourceManagedQueueMutationReport, ResourceManagedQueueState, ResourceNodeDeclaration,
    ResourceNodeId, ResourceOldHostWorkCancellationAdvisory, ResourceOutputContinuity,
    ResourceOverlappingGenerationAdmission, ResourcePolicyCompatibilityClass,
    ResourcePolicyCompatibilityReport, ResourcePolicyDigest, ResourcePolicyResolutionError,
    ResourcePolicyRestoreCompatibilityProof, ResourceRejectionDenialClass,
    ResourceRejectionOrdinal, ResourceRejectionReason, ResourceRejectionReport,
    ResourceReplayDecisionPlan, ResourceReplayReconstructionReport, ResourceRequestAdmissionReport,
    ResourceRequestHandle, ResourceRequestId, ResourceRequestIntent,
    ResourceRetainedDeniedCompletionAvailability,
    ResourceRetainedDeniedCompletionAvailabilityClass, ResourceRetainedHistoryAvailability,
    ResourceRetainedHistoryAvailabilityClass, ResourceRetainedRetryLineageAvailability,
    ResourceRetainedRetryLineageAvailabilityClass, ResourceRetentionCompactionBudget,
    ResourceRetryAdmissionReport, ResourceRetryDenialClass, ResourceRetryOrdinal,
    ResourceRetryReason, ResourceRetryScheduleReport, ResourceRevalidationCoalescing,
    ResourceRevalidationDenialClass, ResourceRevalidationEvidence,
    ResourceRevalidationFreshnessClass, ResourceRevalidationFreshnessDecision,
    ResourceRevalidationIntent, ResourceRevalidationReport, ResourceRuntimeSummary,
    ResourceRuntimeSummaryReadReport, ResourceSafePointObservationCounters,
    ResourceSafePointObservationDenial, ResourceSafePointObservationEvidence,
    ResourceSafePointObservationOrdinal, ResourceSafePointObservationReport,
    ResourceSupersessionOrdinal, ResourceSupersessionRecord, ResourceTimeoutDeadlineAuthority,
    ResourceTimeoutDenialClass, ResourceTimeoutHeartbeatExtensionDenialClass,
    ResourceTimeoutHeartbeatExtensionReport, ResourceTimeoutOrdinal, ResourceTimeoutOutcomeClass,
    ResourceTimeoutReport, RetainedResourceRetryLineage, RolledBackResourceCompletionArtifact,
    ScheduledResourceRetry, StagedDeniedResourceCompletionEffect, StagedResourceCompletionEffect,
    TerminalStateResourceRevalidationProof, TimedOutResourceRequest, ValidatedCompletionEnvelope,
    ValidatedResourcePolicyDeclaration,
};
use crate::data::telemetry::ResourceTelemetry;
use crate::data::temporal::{ReadyTemporalWake, ScheduledTemporalWake, TemporalWakeId};
use crate::state::SignalBranchId;

const RESOURCE_REPLAY_RECONSTRUCTION_SCHEMA_VERSION: &str =
    "worth.resource.replay-reconstruction.v2";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ResourceRevalidationAdmissionPreview {
    Proceed {
        descriptor_id: ResourceDescriptorId,
    },
    Coalesce {
        descriptor_id: ResourceDescriptorId,
        active_request_id: ResourceRequestId,
    },
    Deny(ResourceRevalidationDenialClass),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PreparedResourceRevalidationDisposition {
    Proceed {
        descriptor_id: ResourceDescriptorId,
    },
    Coalesce {
        descriptor_id: ResourceDescriptorId,
        active_request_id: ResourceRequestId,
    },
}

#[derive(Debug)]
pub(super) struct PreparedResourceRevalidation {
    intent: ResourceRevalidationIntent,
    revalidation_decision_digest: ResourcePolicyDigest,
    freshness_decision: ResourceRevalidationFreshnessDecision,
    evidence: ResourceRevalidationEvidence,
    disposition: PreparedResourceRevalidationDisposition,
}

#[derive(Debug, Clone)]
pub(super) struct ResolvedResourceTimeoutPlan {
    timeout_duration: crate::data::temporal::TemporalDuration,
    due_tick: crate::data::temporal::ClockTick,
    outcome_class: ResourceTimeoutOutcomeClass,
    deadline_authority: ResourceTimeoutDeadlineAuthority,
    decision_digest: ResourcePolicyDigest,
}

impl ResolvedResourceTimeoutPlan {
    pub(super) fn new(
        timeout_duration: crate::data::temporal::TemporalDuration,
        due_tick: crate::data::temporal::ClockTick,
        outcome_class: ResourceTimeoutOutcomeClass,
        deadline_authority: ResourceTimeoutDeadlineAuthority,
        decision_digest: ResourcePolicyDigest,
    ) -> Self {
        Self {
            timeout_duration,
            due_tick,
            outcome_class,
            deadline_authority,
            decision_digest,
        }
    }

    pub(super) const fn timeout_duration(&self) -> crate::data::temporal::TemporalDuration {
        self.timeout_duration
    }

    pub(super) const fn due_tick(&self) -> crate::data::temporal::ClockTick {
        self.due_tick
    }

    pub(super) fn bind_scheduled_wake(
        self,
        wake_id: TemporalWakeId,
    ) -> ScheduledResourceTimeoutAdmission {
        ScheduledResourceTimeoutAdmission {
            timeout_duration: self.timeout_duration,
            due_tick: self.due_tick,
            outcome_class: self.outcome_class,
            deadline_authority: self.deadline_authority,
            decision_digest: self.decision_digest,
            wake_id,
        }
    }
}

#[derive(Debug)]
pub(super) struct ScheduledResourceTimeoutAdmission {
    timeout_duration: crate::data::temporal::TemporalDuration,
    due_tick: crate::data::temporal::ClockTick,
    outcome_class: ResourceTimeoutOutcomeClass,
    deadline_authority: ResourceTimeoutDeadlineAuthority,
    decision_digest: ResourcePolicyDigest,
    wake_id: TemporalWakeId,
}

#[derive(Debug)]
pub(super) struct PreparedScheduledResourceRetry {
    scheduled: ScheduledResourceRetry,
    previous: InFlightResourceRequest,
}

impl PreparedScheduledResourceRetry {
    pub(super) fn previous(&self) -> &InFlightResourceRequest {
        &self.previous
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ResourceTerminalVisibilityCause {
    Rejection,
    Timeout,
    Cancellation,
    Supersession,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::logic::transaction::runtime) struct EffectiveResourceDiagnosticsPolicy {
    class: ResourceDiagnosticsDecisionClass,
    max_replay_reconstruction_width: Option<u32>,
    max_forensic_reconstruction_width: Option<u32>,
    decision_digest: ResourcePolicyDigest,
    descriptor_width: u32,
}

impl EffectiveResourceDiagnosticsPolicy {
    pub(in crate::logic::transaction::runtime) const fn class(
        &self,
    ) -> ResourceDiagnosticsDecisionClass {
        self.class
    }

    pub(in crate::logic::transaction::runtime) const fn max_replay_reconstruction_width(
        &self,
    ) -> Option<u32> {
        self.max_replay_reconstruction_width
    }

    pub(in crate::logic::transaction::runtime) const fn max_forensic_reconstruction_width(
        &self,
    ) -> Option<u32> {
        self.max_forensic_reconstruction_width
    }

    pub(in crate::logic::transaction::runtime) fn decision_digest(&self) -> &ResourcePolicyDigest {
        &self.decision_digest
    }

    pub(in crate::logic::transaction::runtime) const fn descriptor_width(&self) -> u32 {
        self.descriptor_width
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::logic::transaction::runtime) struct ResourceRuntimeState {
    next_descriptor_id: ResourceDescriptorId,
    next_request_id: ResourceRequestId,
    next_generation: ResourceGeneration,
    next_lifecycle_ordinal: ResourceLifecycleOrdinal,
    next_denial_id: AsyncDenialId,
    next_completion_ordinal: ResourceCompletionOrdinal,
    next_cancellation_ordinal: ResourceCancellationOrdinal,
    next_timeout_ordinal: ResourceTimeoutOrdinal,
    next_rejection_ordinal: ResourceRejectionOrdinal,
    next_supersession_ordinal: ResourceSupersessionOrdinal,
    next_retry_ordinal: ResourceRetryOrdinal,
    next_safe_point_observation_ordinal: ResourceSafePointObservationOrdinal,
    restore_epoch: u64,
    policy_registry: FrozenResourcePolicyRegistry,
    descriptors: BTreeMap<ResourceDescriptorId, LoweredResourceDescriptor>,
    descriptors_by_node: BTreeMap<ResourceNodeId, ResourceDescriptorId>,
    lifecycle_by_node: BTreeMap<ResourceNodeId, ResourceLifecycleSummary>,
    in_flight_by_request: BTreeMap<ResourceRequestId, InFlightResourceRequest>,
    retained_in_flight_history_by_request: BTreeMap<ResourceRequestId, InFlightResourceRequest>,
    pruned_in_flight_history_by_request:
        BTreeMap<ResourceRequestId, ResourceRetainedHistoryAvailability>,
    terminal_in_flight_by_request: BTreeSet<ResourceRequestId>,
    active_request_by_node: BTreeMap<ResourceNodeId, ResourceRequestId>,
    stale_after_wake_by_node: BTreeMap<ResourceNodeId, TemporalWakeId>,
    pending_retry_by_request: BTreeMap<ResourceRequestId, ScheduledResourceRetry>,
    pending_retry_by_wake: BTreeMap<TemporalWakeId, ResourceRequestId>,
    pending_retry_by_node: BTreeMap<ResourceNodeId, ScheduledResourceRetry>,
    retained_retry_lineage_by_ordinal: BTreeMap<ResourceRetryOrdinal, RetainedResourceRetryLineage>,
    pruned_retry_lineage_by_ordinal:
        BTreeMap<ResourceRetryOrdinal, ResourceRetainedRetryLineageAvailability>,
    retry_budget_ledger: ResourceRetryBudgetLedger,
    denied_completions: BTreeMap<AsyncDenialId, DeniedResourceCompletion>,
    pruned_denied_completions_by_id:
        BTreeMap<AsyncDenialId, ResourceRetainedDeniedCompletionAvailability>,
    latest_denied_completion_by_node: BTreeMap<ResourceNodeId, DeniedResourceCompletion>,
    latest_branch_restore_report: Option<ResourceBranchRestoreReport>,
}

impl Default for ResourceRuntimeState {
    fn default() -> Self {
        Self {
            next_descriptor_id: ResourceDescriptorId::new(0),
            next_request_id: ResourceRequestId::new(0),
            next_generation: ResourceGeneration::new(0),
            next_lifecycle_ordinal: ResourceLifecycleOrdinal::ZERO,
            next_denial_id: AsyncDenialId::new(0),
            next_completion_ordinal: ResourceCompletionOrdinal::ZERO,
            next_cancellation_ordinal: ResourceCancellationOrdinal::ZERO,
            next_timeout_ordinal: ResourceTimeoutOrdinal::ZERO,
            next_rejection_ordinal: ResourceRejectionOrdinal::ZERO,
            next_supersession_ordinal: ResourceSupersessionOrdinal::ZERO,
            next_retry_ordinal: ResourceRetryOrdinal::ZERO,
            next_safe_point_observation_ordinal: ResourceSafePointObservationOrdinal::ZERO,
            restore_epoch: 0,
            policy_registry: FrozenResourcePolicyRegistry::built_in(),
            descriptors: BTreeMap::new(),
            descriptors_by_node: BTreeMap::new(),
            lifecycle_by_node: BTreeMap::new(),
            in_flight_by_request: BTreeMap::new(),
            retained_in_flight_history_by_request: BTreeMap::new(),
            pruned_in_flight_history_by_request: BTreeMap::new(),
            terminal_in_flight_by_request: BTreeSet::new(),
            active_request_by_node: BTreeMap::new(),
            stale_after_wake_by_node: BTreeMap::new(),
            pending_retry_by_request: BTreeMap::new(),
            pending_retry_by_wake: BTreeMap::new(),
            pending_retry_by_node: BTreeMap::new(),
            retained_retry_lineage_by_ordinal: BTreeMap::new(),
            pruned_retry_lineage_by_ordinal: BTreeMap::new(),
            retry_budget_ledger: ResourceRetryBudgetLedger::default(),
            denied_completions: BTreeMap::new(),
            pruned_denied_completions_by_id: BTreeMap::new(),
            latest_denied_completion_by_node: BTreeMap::new(),
            latest_branch_restore_report: None,
        }
    }
}

#[derive(Debug, Serialize)]
struct ResourceReplayLifecycleDigestBasis<'a> {
    schema_version: &'static str,
    lifecycle_entries: &'a [ResourceReplayLifecycleDigestEntry],
}

#[derive(Debug, Clone, Copy, Serialize)]
struct ResourceReplayLifecycleDigestEntry {
    node: ResourceNodeId,
    lifecycle: ResourceLifecycleClass,
    lifecycle_ordinal: ResourceLifecycleOrdinal,
}

#[derive(Debug, Serialize)]
struct ResourceReplayOutputContinuityDigestBasis<'a> {
    schema_version: &'static str,
    output_entries: &'a [ResourceReplayOutputContinuityDigestEntry],
}

#[derive(Debug, Clone, Copy, Serialize)]
struct ResourceReplayOutputContinuityDigestEntry {
    node: ResourceNodeId,
    output_continuity: ResourceOutputContinuity,
    lifecycle_ordinal: ResourceLifecycleOrdinal,
}

#[derive(Debug, Serialize)]
struct ResourceReplayDescriptorDigestBasis<'a> {
    schema_version: &'static str,
    descriptors: &'a [LoweredResourceDescriptor],
}

#[derive(Debug, Serialize)]
struct ResourceReplayDenialDigestBasis<'a> {
    schema_version: &'static str,
    denied_completions: &'a [ResourceReplayDeniedCompletionEntryDigestBasis],
    unavailable_denied_completions: &'a [ResourceReplayUnavailableDeniedCompletionDigestBasis],
}

#[derive(Debug, Serialize)]
struct ResourceReplayRetryLineageDigestBasis<'a> {
    schema_version: &'static str,
    retained_retry_lineages: &'a [RetainedResourceRetryLineage],
    unavailable_retry_lineages: &'a [ResourceRetainedRetryLineageAvailability],
}

#[derive(Debug, Serialize)]
struct ResourceReplayInFlightDigestBasis<'a> {
    schema_version: &'static str,
    in_flight_requests: &'a [ResourceReplayInFlightEntryDigestBasis<'a>],
    retained_history_availability: &'a [ResourceRetainedHistoryAvailability],
}

#[derive(Debug, Serialize)]
struct ResourceReplayHandleDigestBasis {
    request_id: ResourceRequestId,
    generation: ResourceGeneration,
}

#[derive(Debug, Serialize)]
struct ResourceReplayDeniedCompletionEntryDigestBasis {
    class: CompletionDenialClass,
    node: Option<ResourceNodeId>,
    request_id: ResourceRequestId,
    generation: ResourceGeneration,
    restore_epoch: u64,
    attempt: ResourceAttemptId,
    payload_byte_len: u64,
}

#[derive(Debug, Serialize)]
struct ResourceReplayUnavailableDeniedCompletionDigestBasis {
    request_id: ResourceRequestId,
    node: Option<ResourceNodeId>,
    denial_class: CompletionDenialClass,
    class: ResourceRetainedDeniedCompletionAvailabilityClass,
}

#[derive(Debug, Serialize)]
struct ResourceReplayInFlightEntryDigestBasis<'a> {
    handle: ResourceReplayHandleDigestBasis,
    node: ResourceNodeId,
    descriptor_id: ResourceDescriptorId,
    attempt: ResourceAttemptId,
    request_intent_digest: &'a str,
    generation_started_tick: crate::data::temporal::ClockTick,
    lifecycle: ResourceLifecycleClass,
    lifecycle_ordinal: ResourceLifecycleOrdinal,
    status: ResourceInFlightStatus,
    has_timeout_wake: bool,
    timeout_duration: Option<crate::data::temporal::TemporalDuration>,
    timeout_due_tick: Option<crate::data::temporal::ClockTick>,
    timeout_outcome_class: ResourceTimeoutOutcomeClass,
    timeout_deadline_authority: ResourceTimeoutDeadlineAuthority,
    timeout_decision_digest: &'a ResourcePolicyDigest,
    revalidation_freshness_class: Option<ResourceRevalidationFreshnessClass>,
    revalidation_freshness_digest: Option<String>,
    revalidation_policy_decision_digest: Option<ResourcePolicyDigest>,
    superseded_by: Option<ResourceReplayHandleDigestBasis>,
    managed_queue_depth: Option<u64>,
    managed_queue_capacity: Option<u64>,
}

#[derive(Debug, Serialize)]
struct ResourceReplayDigestBasis<'a> {
    schema_version: &'static str,
    descriptor_digest: &'a str,
    lifecycle_digest: &'a str,
    output_continuity_digest: &'a str,
    denied_completion_digest: &'a str,
    retry_lineage_digest: &'a str,
    in_flight_digest: &'a str,
    retained_history_unavailable_count: u32,
    denied_completion_unavailable_count: u32,
    retry_lineage_unavailable_count: u32,
}

#[derive(Debug, Serialize)]
struct ResourceRetentionCompactionPolicyProvenanceDigestBasis<'a> {
    schema_version: &'static str,
    retained_history_decision_digests: &'a [String],
    retry_lineage_decision_digests: &'a [String],
}

#[derive(Debug, Serialize)]
struct ObserverDemandResourceRevalidationDigestBasis<'a> {
    schema_version: &'static str,
    observer_id: u64,
    handle_id: u64,
    policy: &'a str,
    matched_nodes: Vec<String>,
    touched: bool,
    recomputed: bool,
    meaningful_change: bool,
    trigger_matched: bool,
    delivered: bool,
}

#[derive(Debug, Clone)]
struct AppliedResourceCancellation {
    cancelled: CancelledResourceRequest,
    lifecycle: ResourceLifecycleSummary,
    transition: ResourceLifecycleTransition,
    propagated_dependents: Vec<CancelledResourceRequest>,
}

impl ResourceRuntimeState {
    pub(in crate::logic::transaction::runtime) fn set_policy_registry(
        &mut self,
        policy_registry: FrozenResourcePolicyRegistry,
    ) {
        self.policy_registry = policy_registry;
    }

    pub(crate) fn observer_demand_observation_digest(
        observer_id: u64,
        handle_id: u64,
        policy: &str,
        matched_nodes: &[String],
        touched: bool,
        recomputed: bool,
        meaningful_change: bool,
        trigger_matched: bool,
        delivered: bool,
    ) -> String {
        canonical_digest(&ObserverDemandResourceRevalidationDigestBasis {
            schema_version: "worth.resource.observer-demand-revalidation.v1",
            observer_id,
            handle_id,
            policy,
            matched_nodes: matched_nodes.to_vec(),
            touched,
            recomputed,
            meaningful_change,
            trigger_matched,
            delivered,
        })
    }

    fn lifecycle_digest_entry(
        summary: ResourceLifecycleSummary,
    ) -> ResourceReplayLifecycleDigestEntry {
        ResourceReplayLifecycleDigestEntry {
            node: summary.node(),
            lifecycle: summary.lifecycle(),
            lifecycle_ordinal: summary.lifecycle_ordinal(),
        }
    }

    fn output_continuity_digest_entry(
        summary: ResourceLifecycleSummary,
    ) -> ResourceReplayOutputContinuityDigestEntry {
        ResourceReplayOutputContinuityDigestEntry {
            node: summary.node(),
            output_continuity: summary.output_continuity(),
            lifecycle_ordinal: summary.lifecycle_ordinal(),
        }
    }

    fn current_lifecycle_summary(&self, node: ResourceNodeId) -> Option<ResourceLifecycleSummary> {
        self.lifecycle_by_node.get(&node).copied()
    }

    pub(in crate::logic::transaction::runtime) fn observed_resource_node_state(
        &self,
        node: ResourceNodeId,
    ) -> Option<ObservedResourceNodeState> {
        let descriptor_id = *self.descriptors_by_node.get(&node)?;
        let descriptor = self.descriptors.get(&descriptor_id)?;
        let summary = self.current_lifecycle_summary(node)?;
        let output_continuity = descriptor
            .observation_decision_plan()
            .includes_output_continuity()
            .then_some(summary.output_continuity());
        let denied_completion = descriptor
            .observation_decision_plan()
            .includes_denied_completion()
            .then(|| self.latest_denied_completion_for_node(node))
            .flatten();
        let scheduled_retry = descriptor
            .observation_decision_plan()
            .includes_retry_schedule()
            .then(|| self.scheduled_retry_for_node(node))
            .flatten();
        Some(ObservedResourceNodeState::new(
            node,
            summary.lifecycle(),
            summary.lifecycle_ordinal(),
            output_continuity,
            denied_completion,
            scheduled_retry,
            descriptor
                .observation_decision_plan()
                .decision_digest()
                .clone(),
        ))
    }

    fn latest_denied_completion_for_node(
        &self,
        node: ResourceNodeId,
    ) -> Option<DeniedResourceCompletion> {
        self.latest_denied_completion_by_node.get(&node).copied()
    }

    fn clear_latest_denied_completion_for_node(&mut self, node: ResourceNodeId) {
        self.latest_denied_completion_by_node.remove(&node);
    }

    fn rebuild_latest_denied_completion_for_node(&mut self, node: ResourceNodeId) {
        let replacement = self
            .denied_completions
            .values()
            .filter(|denied| denied.node() == Some(node))
            .max_by_key(|denied| denied.denial_id().get())
            .copied();
        if let Some(denied) = replacement {
            self.latest_denied_completion_by_node.insert(node, denied);
        } else {
            self.latest_denied_completion_by_node.remove(&node);
        }
    }

    fn scheduled_retry_for_node(&self, node: ResourceNodeId) -> Option<ScheduledResourceRetry> {
        self.pending_retry_by_node.get(&node).cloned()
    }

    fn pending_output_continuity_for_node(
        &self,
        node: ResourceNodeId,
        descriptor_id: ResourceDescriptorId,
        telemetry: &mut ResourceTelemetry,
    ) -> ResourceOutputContinuity {
        let continuity = match (
            self.descriptors.get(&descriptor_id),
            self.current_lifecycle_summary(node),
        ) {
            (Some(descriptor), Some(current))
                if current.output_continuity() != ResourceOutputContinuity::NoPriorOutput =>
            {
                if descriptor
                    .output_continuity_decision_plan()
                    .preserves_previous_output_while_pending()
                {
                    ResourceOutputContinuity::PriorOutputPreserved
                } else {
                    ResourceOutputContinuity::OutputUnavailableByPolicy
                }
            }
            _ => ResourceOutputContinuity::NoPriorOutput,
        };
        self.record_output_continuity_decision(continuity, telemetry);
        continuity
    }

    fn classify_terminal_output_continuity_for_node(
        &self,
        node: ResourceNodeId,
        descriptor_id: ResourceDescriptorId,
        cause: ResourceTerminalVisibilityCause,
        telemetry: &mut ResourceTelemetry,
    ) -> (ResourceOutputContinuity, bool) {
        let prior_output_exists = self
            .current_lifecycle_summary(node)
            .map(|summary| summary.output_continuity() != ResourceOutputContinuity::NoPriorOutput)
            .unwrap_or(false);
        if !prior_output_exists {
            return (ResourceOutputContinuity::NoPriorOutput, false);
        }
        let descriptor = self
            .descriptors
            .get(&descriptor_id)
            .expect("output continuity classification requires a declared descriptor");
        let plan = descriptor.output_continuity_decision_plan();
        let preserves = match cause {
            ResourceTerminalVisibilityCause::Rejection => {
                plan.preserves_previous_output_after_rejection()
            }
            ResourceTerminalVisibilityCause::Timeout => {
                plan.preserves_previous_output_after_timeout()
            }
            ResourceTerminalVisibilityCause::Cancellation => {
                plan.preserves_previous_output_after_cancellation()
            }
            ResourceTerminalVisibilityCause::Supersession => {
                plan.preserves_previous_output_after_supersession()
            }
        };
        let continuity = if preserves {
            ResourceOutputContinuity::PriorOutputPreserved
        } else {
            ResourceOutputContinuity::OutputUnavailableByPolicy
        };
        self.record_output_continuity_decision(continuity, telemetry);
        (continuity, true)
    }

    fn record_output_continuity_decision(
        &self,
        continuity: ResourceOutputContinuity,
        telemetry: &mut ResourceTelemetry,
    ) {
        telemetry.resource_output_continuity_decision_count += 1;
        match continuity {
            ResourceOutputContinuity::PriorOutputPreserved => {
                telemetry.resource_previous_output_preserved_count += 1;
            }
            ResourceOutputContinuity::OutputUnavailableByPolicy => {
                telemetry.resource_previous_output_hidden_count += 1;
            }
            _ => {}
        }
    }

    fn record_boundary_performance(
        telemetry: &mut ResourceTelemetry,
        envelope: ResourceBoundaryPerformanceEnvelope,
    ) -> ResourceBoundaryPerformanceEnvelope {
        telemetry.record_boundary_performance_envelope(envelope);
        envelope
    }

    fn mark_terminal_in_flight(&mut self, request_id: ResourceRequestId) {
        if self.in_flight_by_request.contains_key(&request_id) {
            self.terminal_in_flight_by_request.insert(request_id);
        }
    }

    fn terminal_in_flight_record(
        &self,
        request_id: ResourceRequestId,
    ) -> Option<InFlightResourceRequest> {
        self.in_flight_by_request
            .get(&request_id)
            .cloned()
            .filter(|in_flight| in_flight.lifecycle().is_terminal())
    }

    pub fn summary(&self) -> ResourceRuntimeSummary {
        let retained_history_unavailable_count = self
            .lifecycle_by_node
            .values()
            .filter(|summary| {
                summary.lifecycle() == ResourceLifecycleClass::RetainedHistoryUnavailable
            })
            .count()
            .saturating_add(self.pruned_in_flight_history_by_request.len());
        ResourceRuntimeSummary::new(
            self.descriptors.len(),
            self.descriptors_by_node.len(),
            self.in_flight_by_request.len(),
            self.active_request_by_node.len(),
            self.retained_in_flight_history_by_request.len(),
            retained_history_unavailable_count,
            self.denied_completions.len(),
            self.retained_retry_lineage_by_ordinal.len(),
            self.denied_completions.len(),
            self.pruned_denied_completions_by_id.len(),
            self.pruned_retry_lineage_by_ordinal.len(),
            self.next_descriptor_id,
        )
    }

    pub fn summary_read_report(
        &self,
        telemetry: &mut ResourceTelemetry,
    ) -> ResourceRuntimeSummaryReadReport {
        telemetry.resource_retained_summary_read_count += 1;
        let performance = Self::record_boundary_performance(
            telemetry,
            ResourceBoundaryPerformanceEnvelope::summary_read(),
        );
        ResourceRuntimeSummaryReadReport::new(self.summary(), performance)
    }

    pub fn descriptor_for_node(&self, node: ResourceNodeId) -> Option<&LoweredResourceDescriptor> {
        self.descriptors_by_node
            .get(&node)
            .and_then(|descriptor_id| self.descriptors.get(descriptor_id))
    }

    pub fn latest_branch_restore_report(&self) -> Option<ResourceBranchRestoreReport> {
        self.latest_branch_restore_report
    }

    pub fn retained_history_availability_for_request(
        &self,
        request_id: ResourceRequestId,
    ) -> Option<&ResourceRetainedHistoryAvailability> {
        self.pruned_in_flight_history_by_request.get(&request_id)
    }

    pub fn retained_denied_completion_availability(
        &self,
        denial_id: AsyncDenialId,
    ) -> Option<&ResourceRetainedDeniedCompletionAvailability> {
        self.pruned_denied_completions_by_id.get(&denial_id)
    }

    pub fn retained_retry_lineage(
        &self,
        retry_ordinal: ResourceRetryOrdinal,
    ) -> Option<&RetainedResourceRetryLineage> {
        self.retained_retry_lineage_by_ordinal.get(&retry_ordinal)
    }

    pub fn retained_retry_lineage_availability(
        &self,
        retry_ordinal: ResourceRetryOrdinal,
    ) -> Option<&ResourceRetainedRetryLineageAvailability> {
        self.pruned_retry_lineage_by_ordinal.get(&retry_ordinal)
    }

    pub fn classify_policy_compatibility(
        &self,
        declaration: &ResourceNodeDeclaration,
        telemetry: &mut ResourceTelemetry,
    ) -> Result<ResourcePolicyCompatibilityReport, crate::data::error::SignalError> {
        let Some(historical_descriptor) = self.descriptor_for_node(declaration.node()) else {
            return Err(crate::data::error::SignalError::invalid_input(format!(
                "cannot classify resource policy compatibility for undeclared resource node {}",
                declaration.node().node()
            )));
        };
        let validated = self.validated_policy_declaration(declaration)?;
        let report = ResourcePolicyCompatibilityReport::classify_against_validated_declaration(
            historical_descriptor.descriptor_id(),
            historical_descriptor.node(),
            historical_descriptor.lowered_policy_bundle(),
            &validated,
            &self.policy_registry,
        )
        .map_err(resource_policy_resolution_signal_error)?;
        telemetry.resource_policy_compatibility_count += 1;
        telemetry.resource_policy_descriptor_comparison_count = telemetry
            .resource_policy_descriptor_comparison_count
            .saturating_add(report.compared_width() as u64);
        telemetry.resource_policy_descriptor_incompatibility_count = telemetry
            .resource_policy_descriptor_incompatibility_count
            .saturating_add(report.incompatible_width() as u64);
        telemetry.record_boundary_performance_envelope(report.performance());
        Ok(report)
    }

    pub fn admit_policy_restore_compatibility(
        &self,
        declaration: &ResourceNodeDeclaration,
        telemetry: &mut ResourceTelemetry,
    ) -> Result<
        Result<ResourcePolicyRestoreCompatibilityProof, DeniedResourcePolicyRestoreCompatibility>,
        crate::data::error::SignalError,
    > {
        let validated = self.validated_policy_declaration(declaration)?;
        let replay_decision_plan = self.replay_decision_plan_from_validated(&validated)?;
        let Some(historical_descriptor) = self.descriptor_for_node(declaration.node()) else {
            return Err(crate::data::error::SignalError::invalid_input(format!(
                "cannot classify resource policy compatibility for undeclared resource node {}",
                declaration.node().node()
            )));
        };
        let compatibility =
            ResourcePolicyCompatibilityReport::classify_against_validated_declaration(
                historical_descriptor.descriptor_id(),
                historical_descriptor.node(),
                historical_descriptor.lowered_policy_bundle(),
                &validated,
                &self.policy_registry,
            )
            .map_err(resource_policy_resolution_signal_error)?;
        telemetry.resource_policy_compatibility_count += 1;
        telemetry.resource_policy_descriptor_comparison_count = telemetry
            .resource_policy_descriptor_comparison_count
            .saturating_add(compatibility.compared_width() as u64);
        telemetry.resource_policy_descriptor_incompatibility_count = telemetry
            .resource_policy_descriptor_incompatibility_count
            .saturating_add(compatibility.incompatible_width() as u64);
        telemetry.record_boundary_performance_envelope(compatibility.performance());
        telemetry.resource_replay_compatibility_decision_count += 1;
        if compatibility.is_compatible() {
            if compatibility
                .families()
                .iter()
                .all(|family| replay_decision_plan.admits_compatible_class(family.class()))
            {
                telemetry.resource_replay_compatible_count += 1;
                Ok(Ok(
                    ResourcePolicyRestoreCompatibilityProof::from_compatibility(
                        compatibility,
                        &replay_decision_plan,
                    )
                    .expect("compatible report must admit restore compatibility proof"),
                ))
            } else {
                telemetry.resource_replay_incompatible_count += 1;
                let primary_incompatible_kind = compatibility
                    .families()
                    .iter()
                    .find(|family| !replay_decision_plan.admits_compatible_class(family.class()))
                    .map(|family| family.kind())
                    .expect("compatible report with replay gate denial must have a gated family");
                Ok(Err(
                    DeniedResourcePolicyRestoreCompatibility::from_replay_policy_gate(
                        compatibility,
                        &replay_decision_plan,
                        primary_incompatible_kind,
                    ),
                ))
            }
        } else {
            telemetry.resource_replay_incompatible_count += 1;
            if compatibility
                .families()
                .iter()
                .any(|family| family.class() == ResourcePolicyCompatibilityClass::MissingDescriptor)
            {
                telemetry.resource_replay_missing_policy_count += 1;
            }
            Ok(Err(
                DeniedResourcePolicyRestoreCompatibility::from_compatibility(
                    compatibility,
                    &replay_decision_plan,
                ),
            ))
        }
    }

    fn validated_policy_declaration(
        &self,
        declaration: &ResourceNodeDeclaration,
    ) -> Result<ValidatedResourcePolicyDeclaration, crate::data::error::SignalError> {
        ValidatedResourcePolicyDeclaration::from_declaration(declaration, &self.policy_registry)
            .map_err(resource_policy_resolution_signal_error)
    }

    pub fn validate_async_capability_declaration(
        &self,
        declaration: &ResourceNodeDeclaration,
        telemetry: &mut ResourceTelemetry,
    ) -> Result<ValidatedResourcePolicyDeclaration, crate::data::error::SignalError> {
        telemetry.async_node_capability_validation_count += 1;
        telemetry.resource_policy_resolution_count += 1;
        ValidatedResourcePolicyDeclaration::from_declaration(declaration, &self.policy_registry)
            .map_err(|err| {
                telemetry.resource_policy_resolution_denial_count += 1;
                resource_policy_resolution_signal_error(err)
            })
    }

    pub(in crate::logic::transaction::runtime) fn validate_resource_policy_declaration_without_async_accounting(
        &self,
        declaration: &ResourceNodeDeclaration,
        telemetry: &mut ResourceTelemetry,
    ) -> Result<ValidatedResourcePolicyDeclaration, crate::data::error::SignalError> {
        telemetry.resource_policy_resolution_count += 1;
        ValidatedResourcePolicyDeclaration::from_declaration(declaration, &self.policy_registry)
            .map_err(|err| {
                telemetry.resource_policy_resolution_denial_count += 1;
                resource_policy_resolution_signal_error(err)
            })
    }

    pub fn freeze_async_capability_declaration(
        &self,
        validated: &ValidatedResourcePolicyDeclaration,
        telemetry: &mut ResourceTelemetry,
    ) -> Result<FrozenResourcePolicyDescriptorSet, crate::data::error::SignalError> {
        telemetry.async_node_capability_freeze_count += 1;
        FrozenResourcePolicyDescriptorSet::from_validated_declaration(
            validated,
            &self.policy_registry,
        )
        .map_err(|err| {
            telemetry.resource_policy_resolution_denial_count += 1;
            resource_policy_resolution_signal_error(err)
        })
    }

    pub(in crate::logic::transaction::runtime) fn freeze_resource_policy_declaration_without_async_accounting(
        &self,
        validated: &ValidatedResourcePolicyDeclaration,
        telemetry: &mut ResourceTelemetry,
    ) -> Result<FrozenResourcePolicyDescriptorSet, crate::data::error::SignalError> {
        FrozenResourcePolicyDescriptorSet::from_validated_declaration(
            validated,
            &self.policy_registry,
        )
        .map_err(|err| {
            telemetry.resource_policy_resolution_denial_count += 1;
            resource_policy_resolution_signal_error(err)
        })
    }

    pub fn lower_async_capability_bundle(
        &self,
        frozen: &FrozenResourcePolicyDescriptorSet,
        telemetry: &mut ResourceTelemetry,
    ) -> LoweredResourcePolicyBundle {
        telemetry.async_node_capability_bundle_lowering_count += 1;
        LoweredResourcePolicyBundle::from_frozen_descriptors(frozen)
    }

    pub(in crate::logic::transaction::runtime) fn lower_resource_policy_bundle_without_async_accounting(
        &self,
        frozen: &FrozenResourcePolicyDescriptorSet,
    ) -> LoweredResourcePolicyBundle {
        LoweredResourcePolicyBundle::from_frozen_descriptors(frozen)
    }

    fn replay_decision_plan_from_validated(
        &self,
        validated: &ValidatedResourcePolicyDeclaration,
    ) -> Result<ResourceReplayDecisionPlan, crate::data::error::SignalError> {
        let frozen = FrozenResourcePolicyDescriptorSet::from_validated_declaration(
            validated,
            &self.policy_registry,
        )
        .map_err(resource_policy_resolution_signal_error)?;
        ResourceReplayDecisionPlan::lower(validated.declaration().replay_policy(), frozen.replay())
            .map_err(resource_policy_resolution_signal_error)
    }

    pub fn replay_reconstruction_width(&self) -> u32 {
        let width = self
            .descriptors
            .len()
            .saturating_add(self.lifecycle_by_node.len())
            .saturating_add(self.denied_completions.len())
            .saturating_add(self.pruned_denied_completions_by_id.len())
            .saturating_add(self.retained_retry_lineage_by_ordinal.len())
            .saturating_add(self.pruned_retry_lineage_by_ordinal.len())
            .saturating_add(self.in_flight_by_request.len())
            .saturating_add(self.retained_in_flight_history_by_request.len())
            .saturating_add(self.pruned_in_flight_history_by_request.len());
        width.min(u32::MAX as usize) as u32
    }

    pub fn effective_diagnostics_policy(&self) -> EffectiveResourceDiagnosticsPolicy {
        let mut class = ResourceDiagnosticsDecisionClass::BudgetedExpansion;
        let mut max_replay_reconstruction_width = Some(u32::MAX);
        let mut max_forensic_reconstruction_width = Some(u32::MAX);
        let mut decision_rows = Vec::new();

        for descriptor in self.descriptors.values() {
            let plan = descriptor.diagnostics_decision_plan();
            decision_rows.push(plan.decision_digest().as_str().to_owned());
            match plan.class() {
                ResourceDiagnosticsDecisionClass::DenyColdExpansion => {
                    class = ResourceDiagnosticsDecisionClass::DenyColdExpansion;
                    max_replay_reconstruction_width = None;
                    max_forensic_reconstruction_width = None;
                    break;
                }
                ResourceDiagnosticsDecisionClass::RetainedOnly => {
                    if class != ResourceDiagnosticsDecisionClass::DenyColdExpansion {
                        class = ResourceDiagnosticsDecisionClass::RetainedOnly;
                        max_replay_reconstruction_width = None;
                        max_forensic_reconstruction_width = None;
                    }
                }
                ResourceDiagnosticsDecisionClass::BudgetedExpansion => {
                    if matches!(class, ResourceDiagnosticsDecisionClass::BudgetedExpansion) {
                        max_replay_reconstruction_width = Some(
                            max_replay_reconstruction_width
                                .unwrap_or(u32::MAX)
                                .min(plan.max_replay_reconstruction_width().unwrap_or(u32::MAX)),
                        );
                        max_forensic_reconstruction_width = Some(
                            max_forensic_reconstruction_width
                                .unwrap_or(u32::MAX)
                                .min(plan.max_forensic_reconstruction_width().unwrap_or(u32::MAX)),
                        );
                    }
                }
                ResourceDiagnosticsDecisionClass::ForensicExpansionBudget => {
                    if matches!(
                        class,
                        ResourceDiagnosticsDecisionClass::BudgetedExpansion
                            | ResourceDiagnosticsDecisionClass::ForensicExpansionBudget
                    ) {
                        class = ResourceDiagnosticsDecisionClass::ForensicExpansionBudget;
                        max_replay_reconstruction_width = Some(
                            max_replay_reconstruction_width
                                .unwrap_or(u32::MAX)
                                .min(plan.max_replay_reconstruction_width().unwrap_or(u32::MAX)),
                        );
                        max_forensic_reconstruction_width = Some(
                            max_forensic_reconstruction_width
                                .unwrap_or(u32::MAX)
                                .min(plan.max_forensic_reconstruction_width().unwrap_or(u32::MAX)),
                        );
                    }
                }
            }
        }

        decision_rows.sort();
        let decision_digest = ResourcePolicyDigest::new(format!(
            "resource-diagnostics-effective-policy:{}:{}:{}:{}",
            class.as_str(),
            max_replay_reconstruction_width
                .map(|value| value.to_string())
                .unwrap_or_else(|| "none".to_owned()),
            max_forensic_reconstruction_width
                .map(|value| value.to_string())
                .unwrap_or_else(|| "none".to_owned()),
            decision_rows.join("|")
        ));

        EffectiveResourceDiagnosticsPolicy {
            class,
            max_replay_reconstruction_width,
            max_forensic_reconstruction_width,
            decision_digest,
            descriptor_width: self.descriptors.len().min(u32::MAX as usize) as u32,
        }
    }

    pub fn reconstruct_replay_summary(
        &self,
        telemetry: &mut ResourceTelemetry,
    ) -> ResourceReplayReconstructionReport {
        let descriptors = self.descriptors.values().cloned().collect::<Vec<_>>();
        let lifecycle_summaries = self.lifecycle_by_node.values().copied().collect::<Vec<_>>();
        let lifecycle_entries = lifecycle_summaries
            .iter()
            .copied()
            .map(Self::lifecycle_digest_entry)
            .collect::<Vec<_>>();
        let output_entries = lifecycle_summaries
            .iter()
            .copied()
            .map(Self::output_continuity_digest_entry)
            .collect::<Vec<_>>();
        let denied_completions = self
            .denied_completions
            .values()
            .copied()
            .collect::<Vec<_>>();
        let denied_completion_entries = denied_completions
            .iter()
            .map(|denied| ResourceReplayDeniedCompletionEntryDigestBasis {
                class: denied.class(),
                node: denied.node(),
                request_id: denied.request_id(),
                generation: denied.generation(),
                restore_epoch: denied.branch_epoch().restore_epoch(),
                attempt: denied.attempt(),
                payload_byte_len: denied.payload_byte_len(),
            })
            .collect::<Vec<_>>();
        let unavailable_denied_completions = self
            .pruned_denied_completions_by_id
            .values()
            .copied()
            .collect::<Vec<_>>();
        let unavailable_denied_completion_entries = unavailable_denied_completions
            .iter()
            .map(
                |availability| ResourceReplayUnavailableDeniedCompletionDigestBasis {
                    request_id: availability.request_id(),
                    node: availability.node(),
                    denial_class: availability.denial_class(),
                    class: availability.class(),
                },
            )
            .collect::<Vec<_>>();
        let retained_retry_lineages = self
            .retained_retry_lineage_by_ordinal
            .values()
            .cloned()
            .collect::<Vec<_>>();
        let unavailable_retry_lineages = self
            .pruned_retry_lineage_by_ordinal
            .values()
            .cloned()
            .collect::<Vec<_>>();
        let mut in_flight_requests = self
            .in_flight_by_request
            .values()
            .cloned()
            .collect::<Vec<_>>();
        in_flight_requests.extend(self.retained_in_flight_history_by_request.values().cloned());
        in_flight_requests.sort_by_key(|request| request.handle());
        let retained_history_availability = self
            .pruned_in_flight_history_by_request
            .values()
            .cloned()
            .collect::<Vec<_>>();
        let retained_history_unavailable_count = lifecycle_summaries
            .iter()
            .filter(|summary| {
                summary.lifecycle() == ResourceLifecycleClass::RetainedHistoryUnavailable
            })
            .count()
            .saturating_add(retained_history_availability.len())
            as u32;
        let lifecycle_summary_width = lifecycle_summaries.len() as u32;
        let descriptor_width = descriptors.len() as u32;
        let denied_completion_width = denied_completions.len() as u32;
        let retained_retry_lineage_width = retained_retry_lineages.len() as u32;
        let in_flight_width = in_flight_requests.len() as u32;
        let denied_completion_unavailable_count = unavailable_denied_completions.len() as u32;
        let retry_lineage_unavailable_count = unavailable_retry_lineages.len() as u32;
        let descriptor_digest = canonical_digest(&ResourceReplayDescriptorDigestBasis {
            schema_version: RESOURCE_REPLAY_RECONSTRUCTION_SCHEMA_VERSION,
            descriptors: &descriptors,
        });
        let lifecycle_digest = canonical_digest(&ResourceReplayLifecycleDigestBasis {
            schema_version: RESOURCE_REPLAY_RECONSTRUCTION_SCHEMA_VERSION,
            lifecycle_entries: &lifecycle_entries,
        });
        let output_continuity_digest =
            canonical_digest(&ResourceReplayOutputContinuityDigestBasis {
                schema_version: RESOURCE_REPLAY_RECONSTRUCTION_SCHEMA_VERSION,
                output_entries: &output_entries,
            });
        let denied_completion_digest = canonical_digest(&ResourceReplayDenialDigestBasis {
            schema_version: RESOURCE_REPLAY_RECONSTRUCTION_SCHEMA_VERSION,
            denied_completions: &denied_completion_entries,
            unavailable_denied_completions: &unavailable_denied_completion_entries,
        });
        let retry_lineage_digest = canonical_digest(&ResourceReplayRetryLineageDigestBasis {
            schema_version: RESOURCE_REPLAY_RECONSTRUCTION_SCHEMA_VERSION,
            retained_retry_lineages: &retained_retry_lineages,
            unavailable_retry_lineages: &unavailable_retry_lineages,
        });
        let in_flight_entries = in_flight_requests
            .iter()
            .map(|request| {
                let revalidation_freshness_decision = request.revalidation_freshness_decision();
                let managed_queue = request.managed_queue();
                ResourceReplayInFlightEntryDigestBasis {
                    handle: ResourceReplayHandleDigestBasis {
                        request_id: request.handle().request_id(),
                        generation: request.handle().generation(),
                    },
                    node: request.node(),
                    descriptor_id: request.descriptor_id(),
                    attempt: request.attempt(),
                    request_intent_digest: request.request_intent_digest().as_str(),
                    generation_started_tick: request.generation_started_tick(),
                    lifecycle: request.lifecycle(),
                    lifecycle_ordinal: request.lifecycle_ordinal(),
                    status: request.status(),
                    has_timeout_wake: request.timeout_wake_id().is_some(),
                    timeout_duration: request.timeout_duration(),
                    timeout_due_tick: request.timeout_due_tick(),
                    timeout_outcome_class: request.timeout_outcome_class(),
                    timeout_deadline_authority: request.timeout_deadline_authority(),
                    timeout_decision_digest: request.timeout_decision_digest(),
                    revalidation_freshness_class: revalidation_freshness_decision
                        .as_ref()
                        .map(|decision| decision.class()),
                    revalidation_freshness_digest: revalidation_freshness_decision
                        .as_ref()
                        .map(|decision| decision.freshness_digest().to_owned()),
                    revalidation_policy_decision_digest: revalidation_freshness_decision
                        .as_ref()
                        .map(|decision| decision.policy_decision_digest().clone()),
                    superseded_by: request.superseded_by().map(|handle| {
                        ResourceReplayHandleDigestBasis {
                            request_id: handle.request_id(),
                            generation: handle.generation(),
                        }
                    }),
                    managed_queue_depth: managed_queue.map(ResourceManagedQueueState::queue_depth),
                    managed_queue_capacity: managed_queue
                        .map(ResourceManagedQueueState::queue_capacity),
                }
            })
            .collect::<Vec<_>>();
        let in_flight_digest = canonical_digest(&ResourceReplayInFlightDigestBasis {
            schema_version: RESOURCE_REPLAY_RECONSTRUCTION_SCHEMA_VERSION,
            in_flight_requests: &in_flight_entries,
            retained_history_availability: &retained_history_availability,
        });
        let replay_digest = canonical_digest(&ResourceReplayDigestBasis {
            schema_version: RESOURCE_REPLAY_RECONSTRUCTION_SCHEMA_VERSION,
            descriptor_digest: &descriptor_digest,
            lifecycle_digest: &lifecycle_digest,
            output_continuity_digest: &output_continuity_digest,
            denied_completion_digest: &denied_completion_digest,
            retry_lineage_digest: &retry_lineage_digest,
            in_flight_digest: &in_flight_digest,
            retained_history_unavailable_count,
            denied_completion_unavailable_count,
            retry_lineage_unavailable_count,
        });

        telemetry.resource_replay_reconstruction_count += 1;
        telemetry.resource_replay_reconstruction_lifecycle_width = telemetry
            .resource_replay_reconstruction_lifecycle_width
            .max(lifecycle_summary_width as u64);
        telemetry.resource_replay_reconstruction_denial_width = telemetry
            .resource_replay_reconstruction_denial_width
            .max(denied_completion_width as u64);
        telemetry.resource_replay_reconstruction_in_flight_width = telemetry
            .resource_replay_reconstruction_in_flight_width
            .max(in_flight_width as u64);
        telemetry.resource_retained_history_unavailable_count = telemetry
            .resource_retained_history_unavailable_count
            .saturating_add(retained_history_unavailable_count as u64);
        let performance = ResourceBoundaryPerformanceEnvelope::replay_reconstruction(
            descriptor_width,
            lifecycle_summary_width,
            denied_completion_width,
            in_flight_width,
            retained_history_unavailable_count,
        );
        let performance = Self::record_boundary_performance(telemetry, performance);

        ResourceReplayReconstructionReport::new(
            descriptor_width,
            lifecycle_summary_width,
            denied_completion_width,
            retained_retry_lineage_width,
            in_flight_width,
            retained_history_unavailable_count,
            denied_completion_unavailable_count,
            retry_lineage_unavailable_count,
            descriptor_digest,
            lifecycle_digest,
            output_continuity_digest,
            denied_completion_digest,
            retry_lineage_digest,
            in_flight_digest,
            replay_digest,
            performance,
        )
    }

    pub fn bump_restore_epoch(
        &mut self,
        branch_id: SignalBranchId,
        telemetry: &mut ResourceTelemetry,
    ) -> ResourceBranchRestoreReport {
        self.restore_epoch = self.restore_epoch.saturating_add(1);
        let branch_epoch = ResourceBranchEpoch::new(branch_id, self.restore_epoch);
        for in_flight in self.in_flight_by_request.values_mut() {
            in_flight.refresh_branch_epoch(branch_epoch);
        }
        for retained in self.retained_in_flight_history_by_request.values_mut() {
            retained.refresh_branch_epoch(branch_epoch);
        }
        for pruned in self.pruned_in_flight_history_by_request.values_mut() {
            *pruned = pruned.clone().with_branch_epoch(branch_epoch);
        }
        for scheduled in self.pending_retry_by_request.values_mut() {
            *scheduled = scheduled
                .clone()
                .with_previous(scheduled.previous().with_branch_epoch(branch_epoch));
        }
        for retained in self.retained_retry_lineage_by_ordinal.values_mut() {
            *retained = retained.clone().with_branch_epoch(branch_epoch);
        }
        for pruned in self.pruned_retry_lineage_by_ordinal.values_mut() {
            *pruned = pruned.clone().with_branch_epoch(branch_epoch);
        }
        self.pending_retry_by_node = self
            .pending_retry_by_request
            .values()
            .cloned()
            .filter_map(|scheduled| {
                self.in_flight_by_request
                    .get(&scheduled.previous().request_id())
                    .map(|in_flight| (in_flight.node(), scheduled))
            })
            .collect();
        telemetry.resource_branch_restore_count += 1;
        telemetry.resource_branch_restore_in_flight_width = telemetry
            .resource_branch_restore_in_flight_width
            .max(self.in_flight_by_request.len() as u64);
        let restored_in_flight_width = self.in_flight_by_request.len() as u32;
        let retained_summary_width =
            self.lifecycle_by_node
                .len()
                .saturating_add(self.denied_completions.len())
                .saturating_add(self.pruned_denied_completions_by_id.len())
                .saturating_add(self.retained_retry_lineage_by_ordinal.len())
                .saturating_add(self.pruned_retry_lineage_by_ordinal.len())
                .saturating_add(self.retained_in_flight_history_by_request.len())
                .saturating_add(self.pruned_in_flight_history_by_request.len()) as u32;
        let broad_rebuild_denial_count = 1;
        telemetry.resource_branch_restore_retained_summary_width = telemetry
            .resource_branch_restore_retained_summary_width
            .max(retained_summary_width as u64);
        telemetry.resource_branch_restore_broad_rebuild_denial_count = telemetry
            .resource_branch_restore_broad_rebuild_denial_count
            .saturating_add(broad_rebuild_denial_count as u64);
        let performance = Self::record_boundary_performance(
            telemetry,
            ResourceBoundaryPerformanceEnvelope::branch_restore(
                restored_in_flight_width,
                retained_summary_width,
                broad_rebuild_denial_count,
            ),
        );
        let report = ResourceBranchRestoreReport::new(
            restored_in_flight_width,
            retained_summary_width,
            broad_rebuild_denial_count,
            performance,
        );
        self.latest_branch_restore_report = Some(report);
        report
    }

    pub fn in_flight_request(
        &self,
        handle: ResourceRequestHandle,
        telemetry: &mut ResourceTelemetry,
    ) -> Option<&InFlightResourceRequest> {
        telemetry.resource_hot_in_flight_lookup_count += 1;
        self.in_flight_by_request
            .get(&handle.request_id())
            .filter(|in_flight| in_flight.handle() == handle)
    }

    pub fn observe_safe_point(
        &mut self,
        binding: &ResourceManagedQueueBinding,
        telemetry: &mut ResourceTelemetry,
    ) -> Result<ResourceSafePointObservationReport, ResourceSafePointObservationDenial> {
        let counters = ResourceSafePointObservationCounters::exact_request_and_pressure();
        let (request, status, lifecycle_ordinal, pressure, timeout_wake_id) = {
            let request = self
                .in_flight_request(binding.request(), telemetry)
                .ok_or_else(|| {
                    ResourceSafePointObservationDenial::request_unavailable(
                        binding.request().request_id(),
                        counters,
                    )
                })?;
            let pressure = request
                .managed_queue()
                .filter(|queue| {
                    request.attempt() == binding.attempt()
                        && queue.queue_capacity() == binding.queue_capacity()
                })
                .map(ResourceManagedQueueState::pressure)
                .ok_or_else(|| {
                    ResourceSafePointObservationDenial::queue_unavailable(
                        binding.request().request_id(),
                        counters,
                    )
                })?;
            (
                request.handle(),
                request.status(),
                request.lifecycle_ordinal(),
                pressure,
                request.timeout_wake_id(),
            )
        };
        let ordinal = self.next_safe_point_observation_ordinal;
        self.next_safe_point_observation_ordinal = self.next_safe_point_observation_ordinal.next();
        Ok(ResourceSafePointObservationReport::new(
            ordinal,
            ResourceSafePointObservationEvidence {
                request,
                status,
                lifecycle_ordinal,
                pressure,
                timeout_wake_id,
            },
            counters,
        ))
    }

    pub fn bind_managed_queue(
        &mut self,
        admitted: AdmittedResourceRequest,
        queue_capacity: u64,
        telemetry: &mut ResourceTelemetry,
    ) -> Result<ResourceManagedQueueBinding, ResourceManagedQueueDenial> {
        let state = ResourceManagedQueueState::new(queue_capacity).map_err(|class| {
            ResourceManagedQueueDenial::new(
                admitted.handle().request_id(),
                class,
                ResourceManagedQueueCounters::none(),
            )
        })?;
        telemetry.resource_hot_in_flight_lookup_count += 1;
        let request_id = admitted.handle().request_id();
        let request = self
            .in_flight_by_request
            .get_mut(&request_id)
            .filter(|request| {
                request.handle() == admitted.handle() && request.attempt() == admitted.attempt()
            })
            .ok_or_else(|| {
                ResourceManagedQueueDenial::new(
                    request_id,
                    ResourceManagedQueueDenialClass::RequestUnavailable,
                    ResourceManagedQueueCounters::exact_lookup(0),
                )
            })?;
        if request.status() != ResourceInFlightStatus::Active
            || request.lifecycle() != ResourceLifecycleClass::Pending
        {
            return Err(ResourceManagedQueueDenial::new(
                request_id,
                ResourceManagedQueueDenialClass::RequestNotActive,
                ResourceManagedQueueCounters::exact_lookup(0),
            ));
        }
        if request.managed_queue().is_some() {
            return Err(ResourceManagedQueueDenial::new(
                request_id,
                ResourceManagedQueueDenialClass::QueueAlreadyBound,
                ResourceManagedQueueCounters::exact_lookup(0),
            ));
        }
        request.bind_managed_queue(state);
        Ok(ResourceManagedQueueBinding::new(
            admitted.handle(),
            admitted.attempt(),
            queue_capacity,
        ))
    }

    pub(super) fn bound_managed_queue_count(&self) -> u32 {
        u32::try_from(
            self.in_flight_by_request
                .values()
                .filter(|request| request.managed_queue().is_some())
                .count(),
        )
        .unwrap_or(u32::MAX)
    }

    pub fn enqueue_managed_queue(
        &mut self,
        binding: &ResourceManagedQueueBinding,
        width: u64,
        telemetry: &mut ResourceTelemetry,
    ) -> Result<ResourceManagedQueueMutationReport, ResourceManagedQueueDenial> {
        self.mutate_managed_queue(
            binding,
            width,
            ResourceManagedQueueMutationKind::Enqueued,
            telemetry,
        )
    }

    pub fn dequeue_managed_queue(
        &mut self,
        binding: &ResourceManagedQueueBinding,
        width: u64,
        telemetry: &mut ResourceTelemetry,
    ) -> Result<ResourceManagedQueueMutationReport, ResourceManagedQueueDenial> {
        self.mutate_managed_queue(
            binding,
            width,
            ResourceManagedQueueMutationKind::Dequeued,
            telemetry,
        )
    }

    fn mutate_managed_queue(
        &mut self,
        binding: &ResourceManagedQueueBinding,
        width: u64,
        kind: ResourceManagedQueueMutationKind,
        telemetry: &mut ResourceTelemetry,
    ) -> Result<ResourceManagedQueueMutationReport, ResourceManagedQueueDenial> {
        telemetry.resource_hot_in_flight_lookup_count += 1;
        let request_id = binding.request().request_id();
        let request = self
            .in_flight_by_request
            .get_mut(&request_id)
            .filter(|request| {
                request.handle() == binding.request() && request.attempt() == binding.attempt()
            })
            .ok_or_else(|| {
                ResourceManagedQueueDenial::new(
                    request_id,
                    ResourceManagedQueueDenialClass::RequestUnavailable,
                    ResourceManagedQueueCounters::exact_lookup(0),
                )
            })?;
        let request_is_active = request.status() == ResourceInFlightStatus::Active
            && request.lifecycle() == ResourceLifecycleClass::Pending;
        if kind == ResourceManagedQueueMutationKind::Enqueued && !request_is_active {
            return Err(ResourceManagedQueueDenial::new(
                request_id,
                ResourceManagedQueueDenialClass::RequestNotActive,
                ResourceManagedQueueCounters::exact_lookup(0),
            ));
        }
        let queue = request
            .managed_queue_mut()
            .filter(|queue| queue.queue_capacity() == binding.queue_capacity())
            .ok_or_else(|| {
                ResourceManagedQueueDenial::new(
                    request_id,
                    ResourceManagedQueueDenialClass::BindingMismatch,
                    ResourceManagedQueueCounters::exact_lookup(0),
                )
            })?;
        let mutation = match kind {
            ResourceManagedQueueMutationKind::Enqueued => queue.enqueue(width),
            ResourceManagedQueueMutationKind::Dequeued => queue.dequeue(width),
        };
        mutation.map_err(|class| {
            ResourceManagedQueueDenial::new(
                request_id,
                class,
                ResourceManagedQueueCounters::exact_lookup(0),
            )
        })?;
        Ok(ResourceManagedQueueMutationReport::new(
            binding.request(),
            kind,
            queue.pressure(),
            ResourceManagedQueueCounters::exact_lookup(1),
        ))
    }

    pub fn active_timeout_wake_for_handle(
        &self,
        handle: ResourceRequestHandle,
    ) -> Option<TemporalWakeId> {
        self.in_flight_by_request
            .get(&handle.request_id())
            .filter(|in_flight| in_flight.handle() == handle)
            .filter(|in_flight| in_flight.status() == ResourceInFlightStatus::Active)
            .filter(|in_flight| in_flight.lifecycle() == ResourceLifecycleClass::Pending)
            .and_then(|in_flight| in_flight.timeout_wake_id())
    }

    pub fn active_timeout_wakes_for_cancellation_footprint(
        &self,
        handle: ResourceRequestHandle,
    ) -> Vec<TemporalWakeId> {
        let mut visited_requests = BTreeSet::new();
        let mut collected_wakes = BTreeSet::new();
        self.collect_active_timeout_wakes_for_cancellation_footprint(
            handle.request_id(),
            handle,
            &mut visited_requests,
            &mut collected_wakes,
        );
        collected_wakes.into_iter().collect()
    }

    pub fn active_timeout_wake_for_node(&self, node: ResourceNodeId) -> Option<TemporalWakeId> {
        let request_id = self.active_request_by_node.get(&node)?;
        self.in_flight_by_request
            .get(request_id)
            .filter(|in_flight| in_flight.status() == ResourceInFlightStatus::Active)
            .filter(|in_flight| in_flight.lifecycle() == ResourceLifecycleClass::Pending)
            .and_then(|in_flight| in_flight.timeout_wake_id())
    }

    pub fn active_stale_after_wake_for_node(&self, node: ResourceNodeId) -> Option<TemporalWakeId> {
        self.stale_after_wake_by_node.get(&node).copied()
    }

    pub fn lifecycle_summary_for_node(
        &self,
        node: ResourceNodeId,
    ) -> Option<ResourceLifecycleSummary> {
        self.lifecycle_by_node.get(&node).copied()
    }

    pub fn active_request_handle_for_node(
        &self,
        node: ResourceNodeId,
    ) -> Option<ResourceRequestHandle> {
        let request_id = self.active_request_by_node.get(&node)?;
        self.in_flight_by_request
            .get(request_id)
            .filter(|in_flight| in_flight.status() == ResourceInFlightStatus::Active)
            .map(|in_flight| in_flight.handle())
    }

    pub fn attach_stale_after_wake(&mut self, node: ResourceNodeId, wake_id: TemporalWakeId) {
        self.stale_after_wake_by_node.insert(node, wake_id);
    }

    pub fn clear_stale_after_wake_for_node(
        &mut self,
        node: ResourceNodeId,
    ) -> Option<TemporalWakeId> {
        self.stale_after_wake_by_node.remove(&node)
    }

    pub fn extend_timeout_heartbeat(
        &mut self,
        handle: ResourceRequestHandle,
        previous_timeout_wake_id: TemporalWakeId,
        extended_timeout_wake: ScheduledTemporalWake,
        telemetry: &mut ResourceTelemetry,
    ) -> ResourceTimeoutHeartbeatExtensionReport {
        telemetry.resource_hot_in_flight_lookup_count += 1;
        let request_id = handle.request_id();
        let Some(in_flight) = self.in_flight_by_request.get_mut(&request_id) else {
            return self.deny_timeout_heartbeat_extension(
                request_id,
                ResourceTimeoutHeartbeatExtensionDenialClass::UnknownOrStaleRequest,
                telemetry,
            );
        };
        if in_flight.handle() != handle {
            return self.deny_timeout_heartbeat_extension(
                request_id,
                ResourceTimeoutHeartbeatExtensionDenialClass::UnknownOrStaleRequest,
                telemetry,
            );
        }
        if in_flight.status() != ResourceInFlightStatus::Active
            || in_flight.lifecycle() != ResourceLifecycleClass::Pending
        {
            return self.deny_timeout_heartbeat_extension(
                request_id,
                ResourceTimeoutHeartbeatExtensionDenialClass::NonActiveRequest,
                telemetry,
            );
        }
        let Some(active_timeout_wake) = in_flight.timeout_wake_id() else {
            return self.deny_timeout_heartbeat_extension(
                request_id,
                ResourceTimeoutHeartbeatExtensionDenialClass::MissingTimeoutWake,
                telemetry,
            );
        };
        if active_timeout_wake != previous_timeout_wake_id {
            return self.deny_timeout_heartbeat_extension(
                request_id,
                ResourceTimeoutHeartbeatExtensionDenialClass::MissingTimeoutWake,
                telemetry,
            );
        }
        let Some(descriptor) = self.descriptors.get(&in_flight.descriptor_id()) else {
            return self.deny_timeout_heartbeat_extension(
                request_id,
                ResourceTimeoutHeartbeatExtensionDenialClass::UnknownOrStaleRequest,
                telemetry,
            );
        };
        let timeout_plan = descriptor.timeout_decision_plan();
        let Some(extension_duration) = timeout_plan.heartbeat_extension() else {
            return self.deny_timeout_heartbeat_extension(
                request_id,
                ResourceTimeoutHeartbeatExtensionDenialClass::PolicyDoesNotAllowHeartbeatExtension,
                telemetry,
            );
        };

        in_flight.attach_timeout_wake(extended_timeout_wake.id());
        telemetry.resource_progress_heartbeat_extension_count += 1;
        telemetry.resource_timeout_temporal_wake_footprint = telemetry
            .resource_timeout_temporal_wake_footprint
            .saturating_add(1);
        let performance = Self::record_boundary_performance(
            telemetry,
            ResourceBoundaryPerformanceEnvelope::timeout_heartbeat_extension(1, 0, 1),
        );
        ResourceTimeoutHeartbeatExtensionReport::admitted(
            ExtendedResourceTimeoutHeartbeat::new(
                handle,
                previous_timeout_wake_id,
                extended_timeout_wake,
                extension_duration,
                timeout_plan.decision_digest().clone(),
            ),
            performance,
        )
    }

    pub fn timeout_heartbeat_extension_candidate(
        &self,
        handle: ResourceRequestHandle,
        telemetry: &mut ResourceTelemetry,
    ) -> Result<
        (
            ResourceNodeId,
            TemporalWakeId,
            crate::data::temporal::TemporalDuration,
        ),
        ResourceTimeoutHeartbeatExtensionDenialClass,
    > {
        telemetry.resource_hot_in_flight_lookup_count += 1;
        let request_id = handle.request_id();
        let Some(in_flight) = self.in_flight_by_request.get(&request_id) else {
            return Err(ResourceTimeoutHeartbeatExtensionDenialClass::UnknownOrStaleRequest);
        };
        if in_flight.handle() != handle {
            return Err(ResourceTimeoutHeartbeatExtensionDenialClass::UnknownOrStaleRequest);
        }
        if in_flight.status() != ResourceInFlightStatus::Active
            || in_flight.lifecycle() != ResourceLifecycleClass::Pending
        {
            return Err(ResourceTimeoutHeartbeatExtensionDenialClass::NonActiveRequest);
        }
        let Some(active_timeout_wake) = in_flight.timeout_wake_id() else {
            return Err(ResourceTimeoutHeartbeatExtensionDenialClass::MissingTimeoutWake);
        };
        let Some(descriptor) = self.descriptors.get(&in_flight.descriptor_id()) else {
            return Err(ResourceTimeoutHeartbeatExtensionDenialClass::UnknownOrStaleRequest);
        };
        let Some(extension_duration) = descriptor.timeout_decision_plan().heartbeat_extension()
        else {
            return Err(
                ResourceTimeoutHeartbeatExtensionDenialClass::PolicyDoesNotAllowHeartbeatExtension,
            );
        };
        Ok((in_flight.node(), active_timeout_wake, extension_duration))
    }

    pub fn declare_resource_node(
        &mut self,
        declaration: ResourceNodeDeclaration,
        telemetry: &mut ResourceTelemetry,
    ) -> Result<ResourceDeclarationReport, crate::data::error::SignalError> {
        let node = declaration.node();
        if self.descriptors_by_node.contains_key(&node) {
            telemetry.resource_duplicate_declaration_denial_count += 1;
            return Err(crate::data::error::SignalError::invalid_input(format!(
                "resource node {} already has a lowered resource descriptor",
                node.node()
            )));
        }

        telemetry.resource_policy_resolution_count += 1;
        let validated_policy_declaration =
            match ValidatedResourcePolicyDeclaration::from_declaration(
                &declaration,
                &self.policy_registry,
            ) {
                Ok(validated) => validated,
                Err(err) => {
                    telemetry.resource_policy_resolution_denial_count += 1;
                    return Err(resource_policy_resolution_signal_error(err));
                }
            };
        let frozen_policy_descriptors =
            match FrozenResourcePolicyDescriptorSet::from_validated_declaration(
                &validated_policy_declaration,
                &self.policy_registry,
            ) {
                Ok(frozen) => frozen,
                Err(err) => {
                    telemetry.resource_policy_resolution_denial_count += 1;
                    return Err(resource_policy_resolution_signal_error(err));
                }
            };
        let lowered_policy_bundle =
            LoweredResourcePolicyBundle::from_frozen_descriptors(&frozen_policy_descriptors);
        let descriptor_id = self.issue_descriptor_id();
        let descriptor = match LoweredResourceDescriptor::from_validated_policy_declaration(
            descriptor_id,
            ResourceDescriptorVersion::INITIAL,
            &validated_policy_declaration,
            lowered_policy_bundle,
        ) {
            Ok(descriptor) => descriptor,
            Err(err) => {
                telemetry.resource_policy_resolution_denial_count += 1;
                return Err(resource_policy_resolution_signal_error(err));
            }
        };
        self.descriptors_by_node.insert(node, descriptor_id);
        self.descriptors.insert(descriptor_id, descriptor);
        let ordinal = self.issue_lifecycle_ordinal();
        let lifecycle = ResourceLifecycleSummary::new(
            node,
            ResourceLifecycleClass::Unrequested,
            ResourceOutputContinuity::NoPriorOutput,
            ordinal,
        );
        let transition = ResourceLifecycleTransition::new(
            node,
            ResourceLifecycleClass::Unrequested,
            ResourceLifecycleClass::Unrequested,
            ResourceLifecycleTransitionKind::DeclarationInitialized,
            ordinal,
            ResourceOutputContinuity::NoPriorOutput,
        );
        self.lifecycle_by_node.insert(node, lifecycle);

        telemetry.resource_declaration_lowering_count += 1;
        telemetry.resource_descriptor_count = self.descriptors.len() as u64;
        let performance = Self::record_boundary_performance(
            telemetry,
            ResourceBoundaryPerformanceEnvelope::declaration_lowering(1),
        );

        Ok(ResourceDeclarationReport::new(
            descriptor_id,
            lifecycle,
            transition,
            performance,
        ))
    }

    pub(super) fn admit_resource_request(
        &mut self,
        intent: ResourceRequestIntent,
        branch_id: SignalBranchId,
        generation_started_tick: crate::data::temporal::ClockTick,
        allow_intent_equivalence_coalescing: bool,
        resolved_timeout: Option<ScheduledResourceTimeoutAdmission>,
        telemetry: &mut ResourceTelemetry,
    ) -> Result<ResourceRequestAdmissionReport, crate::data::error::SignalError> {
        let node = intent.node();
        let descriptor_id = self
            .descriptors_by_node
            .get(&node)
            .copied()
            .ok_or_else(|| {
                telemetry.resource_undeclared_owner_denial_count += 1;
                crate::data::error::SignalError::invalid_input(format!(
                    "cannot admit resource request for undeclared resource node {}",
                    node.node()
                ))
            })?;
        Ok(self.admit_resource_request_with_descriptor(
            intent,
            descriptor_id,
            branch_id,
            generation_started_tick,
            allow_intent_equivalence_coalescing,
            resolved_timeout,
            telemetry,
        ))
    }

    fn admit_resource_request_with_descriptor(
        &mut self,
        intent: ResourceRequestIntent,
        descriptor_id: ResourceDescriptorId,
        branch_id: SignalBranchId,
        generation_started_tick: crate::data::temporal::ClockTick,
        allow_intent_equivalence_coalescing: bool,
        resolved_timeout: Option<ScheduledResourceTimeoutAdmission>,
        telemetry: &mut ResourceTelemetry,
    ) -> ResourceRequestAdmissionReport {
        let node = intent.node();
        let request_intent_digest = intent.canonical_digest();
        if allow_intent_equivalence_coalescing {
            if let Some(coalesced) = self.try_coalesce_equivalent_request_intent(
                node,
                descriptor_id,
                &request_intent_digest,
                branch_id,
                generation_started_tick,
                telemetry,
            ) {
                return coalesced;
            }
        }

        let from = self
            .lifecycle_by_node
            .get(&node)
            .copied()
            .map(ResourceLifecycleSummary::lifecycle)
            .unwrap_or(ResourceLifecycleClass::Unrequested);
        let request_id = self.issue_request_id();
        let generation = self.issue_generation();
        let attempt = ResourceAttemptId::ZERO;
        let branch_epoch = ResourceBranchEpoch::new(branch_id, self.restore_epoch);
        let admitted = AdmittedResourceRequest::new(request_id, generation, branch_epoch, attempt);
        let handle = admitted.handle();
        let supersession =
            self.supersede_active_request_for_node(node, handle, descriptor_id, telemetry);
        let ordinal = self.issue_lifecycle_ordinal();
        let output_continuity =
            self.pending_output_continuity_for_node(node, descriptor_id, telemetry);
        let lifecycle = ResourceLifecycleSummary::new(
            node,
            ResourceLifecycleClass::Pending,
            output_continuity,
            ordinal,
        );
        let transition = ResourceLifecycleTransition::new(
            node,
            from,
            ResourceLifecycleClass::Pending,
            ResourceLifecycleTransitionKind::RequestAdmitted,
            ordinal,
            output_continuity,
        );
        let (
            timeout_duration,
            timeout_due_tick,
            timeout_outcome_class,
            timeout_deadline_authority,
            timeout_decision_digest,
            timeout_wake_id,
        ) = match resolved_timeout {
            Some(timeout) => (
                Some(timeout.timeout_duration),
                Some(timeout.due_tick),
                timeout.outcome_class,
                timeout.deadline_authority,
                timeout.decision_digest,
                Some(timeout.wake_id),
            ),
            None => (
                None,
                None,
                crate::data::resource::ResourceTimeoutOutcomeClass::Terminal,
                crate::data::resource::ResourceTimeoutDeadlineAuthority::Descriptor,
                crate::data::resource::ResourcePolicyDigest::new(
                    "resource-timeout:disabled-admission-default",
                ),
                None,
            ),
        };
        let mut in_flight = InFlightResourceRequest::new(
            handle,
            node,
            descriptor_id,
            generation,
            attempt,
            request_intent_digest,
            generation_started_tick,
            ordinal,
            timeout_duration,
            timeout_due_tick,
            timeout_outcome_class,
            timeout_deadline_authority,
            timeout_decision_digest,
        );
        if let Some(wake_id) = timeout_wake_id {
            in_flight.attach_timeout_wake(wake_id);
            telemetry.resource_timeout_temporal_wake_footprint = telemetry
                .resource_timeout_temporal_wake_footprint
                .saturating_add(1);
        }
        self.in_flight_by_request.insert(request_id, in_flight);
        self.active_request_by_node.insert(node, request_id);
        self.stale_after_wake_by_node.remove(&node);
        self.lifecycle_by_node.insert(node, lifecycle);
        self.clear_latest_denied_completion_for_node(node);

        telemetry.resource_request_admission_count += 1;
        telemetry.resource_in_flight_request_count = self.in_flight_by_request.len() as u64;
        telemetry.resource_in_flight_frontier_width = telemetry
            .resource_in_flight_frontier_width
            .max(self.active_request_by_node.len() as u64);

        let lifecycle_transition_count = if supersession.is_some() { 2 } else { 1 };
        let density_strategy =
            ResourceDensityStrategy::request_pressure(self.in_flight_by_request.len() as u32);
        let supersession_visibility_width = supersession
            .as_ref()
            .map(|record| {
                u32::from(
                    record.lifecycle_transition().output_continuity()
                        != ResourceOutputContinuity::NoPriorOutput,
                )
            })
            .unwrap_or(0);
        let performance = Self::record_boundary_performance(
            telemetry,
            ResourceBoundaryPerformanceEnvelope::request_admission(
                1,
                0,
                lifecycle_transition_count,
            )
            .with_temporal_wake_footprint(u32::from(timeout_wake_id.is_some()))
            .with_density_strategy(density_strategy)
            .with_output_continuity_classification_width(1 + supersession_visibility_width),
        );

        ResourceRequestAdmissionReport::new(
            admitted,
            lifecycle,
            transition,
            supersession,
            None,
            performance,
        )
    }

    pub fn retry_backoff_delay_for_handle(
        &self,
        handle: ResourceRequestHandle,
        current_tick: crate::data::temporal::ClockTick,
        telemetry: &mut ResourceTelemetry,
    ) -> Result<
        (
            crate::data::temporal::TemporalDuration,
            ResourceAttemptId,
            crate::data::resource::ResourcePolicyDigest,
            Option<ResourceRetryBudgetCharge>,
        ),
        ResourceRetryDenialClass,
    > {
        telemetry.resource_retry_policy_decision_count += 1;
        let in_flight = self
            .in_flight_by_request
            .get(&handle.request_id())
            .cloned()
            .filter(|in_flight| in_flight.handle() == handle)
            .ok_or(ResourceRetryDenialClass::UnknownOrStaleRequest)?;

        if in_flight.status() != ResourceInFlightStatus::TimedOut
            || in_flight.lifecycle() != ResourceLifecycleClass::TimedOut
        {
            return Err(ResourceRetryDenialClass::NonRetryableRequest);
        }
        if self
            .pending_retry_by_request
            .contains_key(&handle.request_id())
        {
            return Err(ResourceRetryDenialClass::RetryAlreadyScheduled);
        }

        let descriptor = self
            .descriptors
            .get(&in_flight.descriptor_id())
            .ok_or(ResourceRetryDenialClass::UnknownOrStaleRequest)?;
        if descriptor
            .timeout_decision_plan()
            .retry_window_exhausted(current_tick, in_flight.generation_started_tick())
        {
            return Err(ResourceRetryDenialClass::RetryTimeoutWindowExhausted);
        }
        let retry_plan = descriptor.retry_decision_plan();
        let next_attempt = in_flight.attempt().next();
        if !retry_plan.admits_attempt(next_attempt) {
            return Err(ResourceRetryDenialClass::RetryAttemptLimitReached);
        }
        if retry_plan.max_jitter().is_some() {
            telemetry.resource_retry_jitter_decision_count += 1;
        }
        let retry_budget_charge = self.retry_budget_ledger.charge_for(
            &in_flight,
            retry_plan.retry_budget_scope(),
            retry_plan.retry_budget_limit(),
        );
        if retry_budget_charge.is_some_and(|charge| charge.spent_before() >= charge.limit()) {
            return Err(ResourceRetryDenialClass::RetryBudgetExhausted);
        }
        let scheduled_delay = retry_plan
            .delay_for_attempt(in_flight.handle(), next_attempt)
            .ok_or(ResourceRetryDenialClass::RetryPolicyDisabled)?;
        Ok((
            scheduled_delay,
            next_attempt,
            retry_plan.decision_digest().clone(),
            retry_budget_charge,
        ))
    }

    pub fn pending_retry_wake_for_handle(
        &self,
        handle: ResourceRequestHandle,
    ) -> Option<TemporalWakeId> {
        self.pending_retry_by_request
            .get(&handle.request_id())
            .filter(|scheduled| scheduled.previous() == handle)
            .map(|scheduled| scheduled.backoff_wake_id())
    }

    pub fn pending_retry_wake_for_node(&self, node: ResourceNodeId) -> Option<TemporalWakeId> {
        self.pending_retry_by_node
            .get(&node)
            .map(|scheduled| scheduled.backoff_wake_id())
    }

    fn retry_policy_decision_digest_for_request(
        &self,
        request_id: ResourceRequestId,
    ) -> ResourcePolicyDigest {
        if let Some(scheduled) = self.pending_retry_by_request.get(&request_id) {
            return scheduled.policy_decision_digest().clone();
        }
        if let Some(in_flight) = self.in_flight_by_request.get(&request_id) {
            if let Some(descriptor) = self.descriptors.get(&in_flight.descriptor_id()) {
                return descriptor.retry_decision_plan().decision_digest().clone();
            }
        }
        ResourcePolicyDigest::new("resource-retry-policy-unavailable")
    }

    pub fn clear_pending_retry_for_node(
        &mut self,
        node: ResourceNodeId,
    ) -> Option<ScheduledResourceRetry> {
        let scheduled = self.pending_retry_by_node.remove(&node)?;
        self.pending_retry_by_request
            .remove(&scheduled.previous().request_id());
        self.pending_retry_by_wake
            .remove(&scheduled.backoff_wake_id());
        self.retain_retry_lineage(node, scheduled.clone());
        Some(scheduled)
    }

    pub fn deny_resource_retry_schedule(
        &mut self,
        handle: ResourceRequestHandle,
        class: ResourceRetryDenialClass,
        telemetry: &mut ResourceTelemetry,
    ) -> ResourceRetryScheduleReport {
        let retry_budget_charge = if class == ResourceRetryDenialClass::RetryBudgetExhausted {
            self.in_flight_by_request
                .get(&handle.request_id())
                .cloned()
                .filter(|in_flight| in_flight.handle() == handle)
                .and_then(|in_flight| {
                    self.descriptors
                        .get(&in_flight.descriptor_id())
                        .and_then(|descriptor| {
                            self.retry_budget_ledger.charge_for(
                                &in_flight,
                                descriptor.retry_decision_plan().retry_budget_scope(),
                                descriptor.retry_decision_plan().retry_budget_limit(),
                            )
                        })
                })
        } else {
            None
        };
        let retry_decision_digest =
            self.retry_policy_decision_digest_for_request(handle.request_id());
        self.deny_retry_schedule(
            handle.request_id(),
            class,
            retry_decision_digest,
            retry_budget_charge,
            telemetry,
        )
    }

    pub fn deny_resource_retry_admission_for_report(
        &mut self,
        handle: ResourceRequestHandle,
        class: ResourceRetryDenialClass,
        telemetry: &mut ResourceTelemetry,
    ) -> ResourceRetryAdmissionReport {
        let retry_decision_digest =
            self.retry_policy_decision_digest_for_request(handle.request_id());
        self.deny_retry_admission(handle.request_id(), class, retry_decision_digest, telemetry)
    }

    pub fn deny_forced_revalidation_for_report(
        &mut self,
        node: ResourceNodeId,
        handle: ResourceRequestHandle,
        class: ResourceRevalidationDenialClass,
        telemetry: &mut ResourceTelemetry,
    ) -> ResourceRevalidationReport {
        self.deny_revalidation(
            ResourceRevalidationIntent::with_expected_active(node, handle),
            class,
            telemetry,
        )
    }

    pub fn deny_resource_revalidation_for_report(
        &mut self,
        node: ResourceNodeId,
        class: ResourceRevalidationDenialClass,
        telemetry: &mut ResourceTelemetry,
    ) -> ResourceRevalidationReport {
        self.deny_revalidation(ResourceRevalidationIntent::new(node), class, telemetry)
    }

    pub fn validate_resource_revalidation_intent(
        &self,
        intent: ResourceRevalidationIntent,
    ) -> Option<ResourceRevalidationDenialClass> {
        let node = intent.node();
        if !self.descriptors_by_node.contains_key(&node) {
            return Some(ResourceRevalidationDenialClass::UndeclaredResourceNode);
        }

        match (
            self.active_request_by_node.get(&node).copied(),
            intent.expected_active(),
        ) {
            (Some(_), None) => {
                Some(ResourceRevalidationDenialClass::ActiveRequestRequiresExpectedHandle)
            }
            (Some(active_request_id), Some(expected)) => self
                .in_flight_by_request
                .get(&active_request_id)
                .cloned()
                .filter(|in_flight| in_flight.handle() == expected)
                .filter(|in_flight| in_flight.status() == ResourceInFlightStatus::Active)
                .filter(|in_flight| in_flight.lifecycle() == ResourceLifecycleClass::Pending)
                .map(|_| None)
                .unwrap_or(Some(
                    ResourceRevalidationDenialClass::ExpectedActiveRequestMismatch,
                )),
            (None, Some(_)) => Some(ResourceRevalidationDenialClass::ExpectedActiveRequestMismatch),
            (None, None) => None,
        }
    }

    pub fn prove_active_resource_revalidation_handle(
        &self,
        handle: ResourceRequestHandle,
        telemetry: &mut ResourceTelemetry,
    ) -> Option<ActiveResourceRevalidationProof> {
        telemetry.resource_revalidation_active_handle_proof_check_count += 1;
        telemetry.resource_hot_in_flight_lookup_count += 1;
        let in_flight = self.in_flight_by_request.get(&handle.request_id())?;
        if in_flight.handle() != handle
            || in_flight.status() != ResourceInFlightStatus::Active
            || in_flight.lifecycle() != ResourceLifecycleClass::Pending
        {
            return None;
        }
        let node = in_flight.node();
        let active_request_id = self.active_request_by_node.get(&node).copied()?;
        if active_request_id != handle.request_id() {
            return None;
        }
        let descriptor = self.descriptor_for_node(node)?;
        Some(ActiveResourceRevalidationProof::new(
            node,
            handle,
            descriptor
                .revalidation_decision_plan()
                .decision_digest()
                .clone(),
        ))
    }

    pub fn validate_forced_resource_revalidation_proof(
        &self,
        proof: &ActiveResourceRevalidationProof,
    ) -> Option<ResourceRevalidationDenialClass> {
        let Some(descriptor) = self.descriptor_for_node(proof.node()) else {
            return Some(ResourceRevalidationDenialClass::UndeclaredResourceNode);
        };
        if !descriptor
            .revalidation_decision_plan()
            .permits_active_handle_forcing()
        {
            return Some(ResourceRevalidationDenialClass::ForcedRevalidationPolicyDisabled);
        }
        let Some(in_flight) = self.in_flight_by_request.get(&proof.handle().request_id()) else {
            return Some(ResourceRevalidationDenialClass::ActiveHandleProofMismatch);
        };
        if in_flight.handle() != proof.handle()
            || in_flight.node() != proof.node()
            || in_flight.status() != ResourceInFlightStatus::Active
            || in_flight.lifecycle() != ResourceLifecycleClass::Pending
        {
            return Some(ResourceRevalidationDenialClass::ActiveHandleProofMismatch);
        }
        let Some(active_request_id) = self.active_request_by_node.get(&proof.node()).copied()
        else {
            return Some(ResourceRevalidationDenialClass::ActiveHandleProofMismatch);
        };
        if active_request_id != proof.handle().request_id()
            || descriptor.revalidation_decision_plan().decision_digest() != proof.decision_digest()
        {
            return Some(ResourceRevalidationDenialClass::ActiveHandleProofMismatch);
        }
        None
    }

    pub fn prove_dependency_change_resource_revalidation(
        &self,
        node: ResourceNodeId,
        node_state: NodeState,
        telemetry: &mut ResourceTelemetry,
    ) -> Option<DependencyChangeResourceRevalidationProof> {
        telemetry.resource_revalidation_dependency_change_proof_check_count += 1;
        let descriptor = self.descriptor_for_node(node)?;
        if !descriptor
            .revalidation_decision_plan()
            .permits_dependency_change_revalidation()
        {
            return None;
        }
        match node_state {
            NodeState::Dirty | NodeState::MaybeStale => {
                Some(DependencyChangeResourceRevalidationProof::new(
                    node,
                    node_state,
                    descriptor
                        .revalidation_decision_plan()
                        .decision_digest()
                        .clone(),
                ))
            }
            NodeState::Clean => None,
        }
    }

    pub fn validate_dependency_change_resource_revalidation_proof(
        &self,
        proof: &DependencyChangeResourceRevalidationProof,
        current_node_state: NodeState,
    ) -> Option<ResourceRevalidationDenialClass> {
        let Some(descriptor) = self.descriptor_for_node(proof.node()) else {
            return Some(ResourceRevalidationDenialClass::UndeclaredResourceNode);
        };
        if !descriptor
            .revalidation_decision_plan()
            .permits_dependency_change_revalidation()
        {
            return Some(
                ResourceRevalidationDenialClass::DependencyChangeRevalidationPolicyDisabled,
            );
        }
        if !matches!(current_node_state, NodeState::Dirty | NodeState::MaybeStale)
            || current_node_state != proof.node_state()
            || descriptor.revalidation_decision_plan().decision_digest() != proof.decision_digest()
        {
            return Some(ResourceRevalidationDenialClass::DependencyChangeProofMismatch);
        }
        None
    }

    pub fn validate_observer_demand_resource_revalidation_proof(
        &self,
        proof: &ObserverDemandResourceRevalidationProof,
    ) -> Option<ResourceRevalidationDenialClass> {
        let Some(descriptor) = self.descriptor_for_node(proof.node()) else {
            return Some(ResourceRevalidationDenialClass::UndeclaredResourceNode);
        };
        if !descriptor
            .revalidation_decision_plan()
            .permits_observer_demand_revalidation()
        {
            return Some(ResourceRevalidationDenialClass::ObserverDemandRevalidationPolicyDisabled);
        }
        if descriptor.revalidation_decision_plan().decision_digest() != proof.decision_digest() {
            return Some(ResourceRevalidationDenialClass::ObserverDemandProofMismatch);
        }
        None
    }

    pub fn prove_terminal_state_resource_revalidation(
        &self,
        node: ResourceNodeId,
        telemetry: &mut ResourceTelemetry,
    ) -> Option<TerminalStateResourceRevalidationProof> {
        telemetry.resource_revalidation_terminal_state_proof_check_count += 1;
        let descriptor = self.descriptor_for_node(node)?;
        if !descriptor
            .revalidation_decision_plan()
            .permits_terminal_state_revalidation()
        {
            return None;
        }
        let lifecycle = self.lifecycle_by_node.get(&node)?.lifecycle();
        let lifecycle_ordinal = self.lifecycle_by_node.get(&node)?.lifecycle_ordinal();
        if !lifecycle.is_terminal() {
            return None;
        }
        Some(TerminalStateResourceRevalidationProof::new(
            node,
            lifecycle,
            lifecycle_ordinal,
            descriptor
                .revalidation_decision_plan()
                .decision_digest()
                .clone(),
        ))
    }

    pub fn validate_terminal_state_resource_revalidation_proof(
        &self,
        proof: &TerminalStateResourceRevalidationProof,
    ) -> Option<ResourceRevalidationDenialClass> {
        let Some(descriptor) = self.descriptor_for_node(proof.node()) else {
            return Some(ResourceRevalidationDenialClass::UndeclaredResourceNode);
        };
        if !descriptor
            .revalidation_decision_plan()
            .permits_terminal_state_revalidation()
        {
            return Some(ResourceRevalidationDenialClass::TerminalStateRevalidationPolicyDisabled);
        }
        let Some(lifecycle) = self.lifecycle_by_node.get(&proof.node()).copied() else {
            return Some(ResourceRevalidationDenialClass::TerminalStateProofMismatch);
        };
        if !lifecycle.lifecycle().is_terminal()
            || lifecycle.lifecycle() != proof.lifecycle()
            || lifecycle.lifecycle_ordinal() != proof.lifecycle_ordinal()
            || descriptor.revalidation_decision_plan().decision_digest() != proof.decision_digest()
        {
            return Some(ResourceRevalidationDenialClass::TerminalStateProofMismatch);
        }
        None
    }

    pub fn prove_fulfilled_lifecycle_resource_revalidation(
        &self,
        node: ResourceNodeId,
        telemetry: &mut ResourceTelemetry,
    ) -> Option<FulfilledLifecycleResourceRevalidationProof> {
        telemetry.resource_revalidation_fulfilled_lifecycle_proof_check_count += 1;
        let descriptor = self.descriptor_for_node(node)?;
        if !descriptor
            .revalidation_decision_plan()
            .permits_fulfilled_lifecycle_revalidation()
        {
            return None;
        }
        let lifecycle = self.lifecycle_by_node.get(&node)?.lifecycle();
        let lifecycle_ordinal = self.lifecycle_by_node.get(&node)?.lifecycle_ordinal();
        if lifecycle != ResourceLifecycleClass::Fulfilled {
            return None;
        }
        Some(FulfilledLifecycleResourceRevalidationProof::new(
            node,
            lifecycle_ordinal,
            descriptor
                .revalidation_decision_plan()
                .decision_digest()
                .clone(),
        ))
    }

    pub fn validate_fulfilled_lifecycle_resource_revalidation_proof(
        &self,
        proof: &FulfilledLifecycleResourceRevalidationProof,
    ) -> Option<ResourceRevalidationDenialClass> {
        let Some(descriptor) = self.descriptor_for_node(proof.node()) else {
            return Some(ResourceRevalidationDenialClass::UndeclaredResourceNode);
        };
        if !descriptor
            .revalidation_decision_plan()
            .permits_fulfilled_lifecycle_revalidation()
        {
            return Some(
                ResourceRevalidationDenialClass::FulfilledLifecycleRevalidationPolicyDisabled,
            );
        }
        let Some(lifecycle) = self.lifecycle_by_node.get(&proof.node()).copied() else {
            return Some(ResourceRevalidationDenialClass::FulfilledLifecycleProofMismatch);
        };
        if lifecycle.lifecycle() != ResourceLifecycleClass::Fulfilled
            || lifecycle.lifecycle_ordinal() != proof.lifecycle_ordinal()
            || descriptor.revalidation_decision_plan().decision_digest() != proof.decision_digest()
        {
            return Some(ResourceRevalidationDenialClass::FulfilledLifecycleProofMismatch);
        }
        None
    }

    pub fn validate_stale_after_resource_revalidation(
        &self,
        node: ResourceNodeId,
        ready_wake: &ReadyTemporalWake,
    ) -> Option<ResourceRevalidationDenialClass> {
        let Some(descriptor) = self.descriptor_for_node(node) else {
            return Some(ResourceRevalidationDenialClass::UndeclaredResourceNode);
        };
        let revalidation_plan = descriptor.revalidation_decision_plan();
        if !revalidation_plan.permits_stale_after_revalidation()
            || !descriptor.stale_after_decision_plan().is_enabled()
        {
            return Some(ResourceRevalidationDenialClass::StaleAfterRevalidationPolicyDisabled);
        }
        if revalidation_plan.stale_after_requires_fulfilled_lifecycle()
            && self
                .lifecycle_by_node
                .get(&node)
                .is_none_or(|lifecycle| lifecycle.lifecycle() != ResourceLifecycleClass::Fulfilled)
        {
            return Some(ResourceRevalidationDenialClass::StaleAfterRequiresFulfilledLifecycle);
        }
        match self.stale_after_wake_by_node.get(&node).copied() {
            Some(wake_id) if wake_id == ready_wake.id() => None,
            _ => Some(ResourceRevalidationDenialClass::StaleAfterWakeMismatch),
        }
    }

    fn prepare_resource_revalidation(
        &mut self,
        intent: ResourceRevalidationIntent,
        count_policy_decision: bool,
        revalidation_decision_digest: ResourcePolicyDigest,
        freshness_decision: ResourceRevalidationFreshnessDecision,
        evidence: ResourceRevalidationEvidence,
        telemetry: &mut ResourceTelemetry,
    ) -> Result<PreparedResourceRevalidation, ResourceRevalidationReport> {
        if count_policy_decision {
            telemetry.resource_revalidation_policy_decision_count += 1;
        }
        let disposition = match self.preview_revalidation_admission(intent, &freshness_decision) {
            ResourceRevalidationAdmissionPreview::Proceed { descriptor_id } => {
                PreparedResourceRevalidationDisposition::Proceed { descriptor_id }
            }
            ResourceRevalidationAdmissionPreview::Coalesce {
                descriptor_id,
                active_request_id,
            } => PreparedResourceRevalidationDisposition::Coalesce {
                descriptor_id,
                active_request_id,
            },
            ResourceRevalidationAdmissionPreview::Deny(class) => {
                return Err(self.deny_revalidation(intent, class, telemetry));
            }
        };
        Ok(PreparedResourceRevalidation {
            intent,
            revalidation_decision_digest,
            freshness_decision,
            evidence,
            disposition,
        })
    }

    pub(super) fn prepare_explicit_resource_revalidation(
        &mut self,
        intent: ResourceRevalidationIntent,
        revalidation_decision_digest: ResourcePolicyDigest,
        telemetry: &mut ResourceTelemetry,
    ) -> Result<PreparedResourceRevalidation, ResourceRevalidationReport> {
        self.prepare_resource_revalidation(
            intent,
            true,
            revalidation_decision_digest.clone(),
            ResourceRevalidationFreshnessDecision::explicit_intent(revalidation_decision_digest),
            ResourceRevalidationEvidence::ExplicitIntent {
                expected_active: intent.expected_active(),
            },
            telemetry,
        )
    }

    pub(super) fn prepare_forced_resource_revalidation(
        &mut self,
        proof: ActiveResourceRevalidationProof,
        telemetry: &mut ResourceTelemetry,
    ) -> Result<PreparedResourceRevalidation, ResourceRevalidationReport> {
        let intent = ResourceRevalidationIntent::with_expected_active(proof.node(), proof.handle());
        self.prepare_resource_revalidation(
            intent,
            false,
            proof.decision_digest().clone(),
            ResourceRevalidationFreshnessDecision::forced_active_handle(
                proof.handle(),
                proof.decision_digest().clone(),
            ),
            ResourceRevalidationEvidence::ForcedActive(proof),
            telemetry,
        )
    }

    pub(super) fn prepare_dependency_change_resource_revalidation(
        &mut self,
        proof: DependencyChangeResourceRevalidationProof,
        telemetry: &mut ResourceTelemetry,
    ) -> Result<PreparedResourceRevalidation, ResourceRevalidationReport> {
        let intent = ResourceRevalidationIntent::new(proof.node());
        self.prepare_resource_revalidation(
            intent,
            false,
            proof.decision_digest().clone(),
            ResourceRevalidationFreshnessDecision::dependency_change(&proof),
            ResourceRevalidationEvidence::DependencyChange(proof),
            telemetry,
        )
    }

    pub(super) fn prepare_observer_demand_resource_revalidation(
        &mut self,
        proof: ObserverDemandResourceRevalidationProof,
        telemetry: &mut ResourceTelemetry,
    ) -> Result<PreparedResourceRevalidation, ResourceRevalidationReport> {
        let intent = ResourceRevalidationIntent::new(proof.node());
        self.prepare_resource_revalidation(
            intent,
            false,
            proof.decision_digest().clone(),
            ResourceRevalidationFreshnessDecision::observer_demand(&proof),
            ResourceRevalidationEvidence::ObserverDemand(proof),
            telemetry,
        )
    }

    pub(super) fn prepare_terminal_state_resource_revalidation(
        &mut self,
        proof: TerminalStateResourceRevalidationProof,
        telemetry: &mut ResourceTelemetry,
    ) -> Result<PreparedResourceRevalidation, ResourceRevalidationReport> {
        let intent = ResourceRevalidationIntent::new(proof.node());
        self.prepare_resource_revalidation(
            intent,
            false,
            proof.decision_digest().clone(),
            ResourceRevalidationFreshnessDecision::terminal_state(&proof),
            ResourceRevalidationEvidence::TerminalState(proof),
            telemetry,
        )
    }

    pub(super) fn prepare_fulfilled_lifecycle_resource_revalidation(
        &mut self,
        proof: FulfilledLifecycleResourceRevalidationProof,
        telemetry: &mut ResourceTelemetry,
    ) -> Result<PreparedResourceRevalidation, ResourceRevalidationReport> {
        let intent = ResourceRevalidationIntent::new(proof.node());
        self.prepare_resource_revalidation(
            intent,
            false,
            proof.decision_digest().clone(),
            ResourceRevalidationFreshnessDecision::fulfilled_lifecycle(&proof),
            ResourceRevalidationEvidence::FulfilledLifecycle(proof),
            telemetry,
        )
    }

    pub(super) fn prepare_stale_after_resource_revalidation(
        &mut self,
        node: ResourceNodeId,
        ready_wake: ReadyTemporalWake,
        revalidation_decision_digest: ResourcePolicyDigest,
        telemetry: &mut ResourceTelemetry,
    ) -> Result<PreparedResourceRevalidation, ResourceRevalidationReport> {
        let intent = ResourceRevalidationIntent::new(node);
        self.prepare_resource_revalidation(
            intent,
            false,
            revalidation_decision_digest.clone(),
            ResourceRevalidationFreshnessDecision::stale_after(
                node,
                ready_wake.id(),
                revalidation_decision_digest,
            ),
            ResourceRevalidationEvidence::StaleAfter(ready_wake),
            telemetry,
        )
    }

    pub(super) fn admit_prepared_resource_revalidation(
        &mut self,
        prepared: PreparedResourceRevalidation,
        branch_id: SignalBranchId,
        generation_started_tick: crate::data::temporal::ClockTick,
        resolved_timeout: Option<ScheduledResourceTimeoutAdmission>,
        telemetry: &mut ResourceTelemetry,
    ) -> ResourceRevalidationReport {
        let PreparedResourceRevalidation {
            intent,
            revalidation_decision_digest,
            freshness_decision,
            evidence,
            disposition,
        } = prepared;
        if let PreparedResourceRevalidationDisposition::Coalesce {
            descriptor_id,
            active_request_id,
        } = disposition
        {
            return self.coalesce_revalidation(
                intent,
                descriptor_id,
                active_request_id,
                branch_id,
                generation_started_tick,
                freshness_decision,
                evidence,
                revalidation_decision_digest,
                resolved_timeout,
                telemetry,
            );
        }
        let PreparedResourceRevalidationDisposition::Proceed { descriptor_id } = disposition else {
            unreachable!("coalescing disposition returned before request admission")
        };
        let temporal_wake_footprint = u32::from(resolved_timeout.is_some());
        let request_report = self.admit_resource_request_with_descriptor(
            match intent.transaction_deadline() {
                Some(deadline) => {
                    ResourceRequestIntent::with_transaction_deadline(intent.node(), deadline)
                }
                None => ResourceRequestIntent::new(intent.node()),
            },
            descriptor_id,
            branch_id,
            generation_started_tick,
            false,
            resolved_timeout,
            telemetry,
        );
        let admitted_request = request_report.admitted_request();
        if let Some(in_flight) = self
            .in_flight_by_request
            .get_mut(&admitted_request.handle().request_id())
        {
            in_flight.attach_revalidation_freshness(&freshness_decision);
        }
        let supersession_record = request_report.supersession_record();
        let lifecycle = request_report.lifecycle();
        let transition = request_report.transition();
        let lifecycle_transition_count = request_report.performance().lifecycle_transition_count();

        telemetry.resource_revalidation_admission_count += 1;
        let performance = Self::record_boundary_performance(
            telemetry,
            ResourceBoundaryPerformanceEnvelope::revalidation_admission(
                1,
                0,
                lifecycle_transition_count,
                temporal_wake_footprint,
            )
            .with_output_continuity_classification_width(
                request_report
                    .performance()
                    .output_continuity_classification_width(),
            ),
        );

        ResourceRevalidationReport::admitted(
            AdmittedResourceRevalidation::new(
                admitted_request,
                freshness_decision,
                evidence,
                None,
                supersession_record,
                revalidation_decision_digest,
            ),
            lifecycle,
            transition,
            performance,
        )
    }

    fn preview_revalidation_admission(
        &self,
        intent: ResourceRevalidationIntent,
        freshness_decision: &ResourceRevalidationFreshnessDecision,
    ) -> ResourceRevalidationAdmissionPreview {
        let node = intent.node();
        let Some(descriptor_id) = self.descriptors_by_node.get(&node).copied() else {
            return ResourceRevalidationAdmissionPreview::Deny(
                ResourceRevalidationDenialClass::UndeclaredResourceNode,
            );
        };

        let request_intent = match intent.transaction_deadline() {
            Some(deadline) => ResourceRequestIntent::with_transaction_deadline(node, deadline),
            None => ResourceRequestIntent::new(node),
        };
        let request_intent_digest = request_intent.canonical_digest();
        if let Some(active_request_id) = self.active_request_by_node.get(&node).copied() {
            if let Some(active_in_flight) = self.in_flight_by_request.get(&active_request_id) {
                if active_in_flight.status() == ResourceInFlightStatus::Active
                    && active_in_flight.lifecycle() == ResourceLifecycleClass::Pending
                    && active_in_flight.request_intent_digest() == &request_intent_digest
                    && active_in_flight
                        .revalidation_freshness_decision()
                        .as_ref()
                        .is_some_and(|existing| {
                            existing.class() == freshness_decision.class()
                                && existing.freshness_digest()
                                    == freshness_decision.freshness_digest()
                        })
                {
                    return ResourceRevalidationAdmissionPreview::Coalesce {
                        descriptor_id,
                        active_request_id,
                    };
                }
            }
        }

        self.validate_resource_revalidation_intent(intent)
            .map(ResourceRevalidationAdmissionPreview::Deny)
            .unwrap_or(ResourceRevalidationAdmissionPreview::Proceed { descriptor_id })
    }

    fn coalesce_revalidation(
        &mut self,
        intent: ResourceRevalidationIntent,
        descriptor_id: ResourceDescriptorId,
        active_request_id: ResourceRequestId,
        branch_id: SignalBranchId,
        generation_started_tick: crate::data::temporal::ClockTick,
        freshness_decision: ResourceRevalidationFreshnessDecision,
        evidence: ResourceRevalidationEvidence,
        revalidation_decision_digest: crate::data::resource::ResourcePolicyDigest,
        resolved_timeout: Option<ScheduledResourceTimeoutAdmission>,
        telemetry: &mut ResourceTelemetry,
    ) -> ResourceRevalidationReport {
        let node = intent.node();
        let temporal_wake_footprint = u32::from(resolved_timeout.is_some());
        let active_in_flight = self.in_flight_by_request[&active_request_id].clone();
        let request_intent_digest = match intent.transaction_deadline() {
            Some(deadline) => ResourceRequestIntent::with_transaction_deadline(node, deadline),
            None => ResourceRequestIntent::new(node),
        }
        .canonical_digest();

        let request_id = self.issue_request_id();
        let generation = self.issue_generation();
        let attempt = ResourceAttemptId::ZERO;
        let branch_epoch = ResourceBranchEpoch::new(branch_id, self.restore_epoch);
        let coalesced_request =
            AdmittedResourceRequest::new(request_id, generation, branch_epoch, attempt);
        let lifecycle_ordinal = self.issue_lifecycle_ordinal();
        let (output_continuity, terminal_visibility_classified) = self
            .classify_terminal_output_continuity_for_node(
                node,
                descriptor_id,
                ResourceTerminalVisibilityCause::Supersession,
                telemetry,
            );
        let transition = ResourceLifecycleTransition::new(
            node,
            ResourceLifecycleClass::Pending,
            ResourceLifecycleClass::Superseded,
            ResourceLifecycleTransitionKind::RequestSuperseded,
            lifecycle_ordinal,
            output_continuity,
        );
        let mut coalesced_in_flight = InFlightResourceRequest::new(
            coalesced_request.handle(),
            node,
            descriptor_id,
            generation,
            attempt,
            request_intent_digest,
            generation_started_tick,
            lifecycle_ordinal,
            active_in_flight.timeout_duration(),
            active_in_flight.timeout_due_tick(),
            active_in_flight.timeout_outcome_class(),
            active_in_flight.timeout_deadline_authority(),
            active_in_flight.timeout_decision_digest().clone(),
        );
        coalesced_in_flight.attach_revalidation_freshness(&freshness_decision);
        coalesced_in_flight.supersede(lifecycle_ordinal, active_in_flight.handle());
        self.in_flight_by_request
            .insert(request_id, coalesced_in_flight);
        self.mark_terminal_in_flight(request_id);

        telemetry.resource_request_admission_count += 1;
        telemetry.resource_revalidation_admission_count += 1;
        telemetry.resource_revalidation_coalesced_count += 1;
        telemetry.resource_in_flight_request_count = self.in_flight_by_request.len() as u64;
        telemetry.resource_in_flight_frontier_width = telemetry
            .resource_in_flight_frontier_width
            .max(self.active_request_by_node.len() as u64);

        let performance = Self::record_boundary_performance(
            telemetry,
            ResourceBoundaryPerformanceEnvelope::revalidation_admission(
                1,
                0,
                1,
                temporal_wake_footprint,
            )
            .with_coalescing_width(1)
            .with_output_continuity_classification_width(u32::from(terminal_visibility_classified)),
        );
        let lifecycle = self
            .lifecycle_by_node
            .get(&node)
            .copied()
            .unwrap_or_else(|| {
                ResourceLifecycleSummary::new(
                    node,
                    ResourceLifecycleClass::Pending,
                    ResourceOutputContinuity::NoPriorOutput,
                    active_in_flight.lifecycle_ordinal(),
                )
            });
        let admitted_request = AdmittedResourceRequest::new(
            active_in_flight.handle().request_id(),
            active_in_flight.generation(),
            active_in_flight.handle().branch_epoch(),
            active_in_flight.attempt(),
        );
        if let Some(timeout) = resolved_timeout {
            self.in_flight_by_request
                .get_mut(&active_request_id)
                .expect("coalesced revalidation winner must remain active")
                .attach_timeout_wake(timeout.wake_id);
            telemetry.resource_timeout_temporal_wake_footprint = telemetry
                .resource_timeout_temporal_wake_footprint
                .saturating_add(1);
        }

        ResourceRevalidationReport::admitted(
            AdmittedResourceRevalidation::new(
                admitted_request,
                freshness_decision.clone(),
                evidence,
                Some(ResourceRevalidationCoalescing::new(
                    active_in_flight.handle(),
                    coalesced_request,
                    freshness_decision,
                    transition,
                )),
                None,
                revalidation_decision_digest,
            ),
            lifecycle,
            transition,
            performance,
        )
    }

    pub fn schedule_resource_retry(
        &mut self,
        handle: ResourceRequestHandle,
        reason: ResourceRetryReason,
        backoff_wake_id: TemporalWakeId,
        next_attempt: ResourceAttemptId,
        scheduled_delay: crate::data::temporal::TemporalDuration,
        retry_decision_digest: crate::data::resource::ResourcePolicyDigest,
        retry_budget_charge: Option<ResourceRetryBudgetCharge>,
        telemetry: &mut ResourceTelemetry,
    ) -> ResourceRetryScheduleReport {
        telemetry.resource_hot_in_flight_lookup_count += 1;
        let request_id = handle.request_id();
        let Some(in_flight) = self.in_flight_by_request.get(&request_id).cloned() else {
            return self.deny_retry_schedule(
                request_id,
                ResourceRetryDenialClass::UnknownOrStaleRequest,
                retry_decision_digest.clone(),
                None,
                telemetry,
            );
        };

        if in_flight.handle() != handle {
            return self.deny_retry_schedule(
                request_id,
                ResourceRetryDenialClass::UnknownOrStaleRequest,
                retry_decision_digest.clone(),
                None,
                telemetry,
            );
        }
        if in_flight.status() != ResourceInFlightStatus::TimedOut
            || in_flight.lifecycle() != ResourceLifecycleClass::TimedOut
        {
            return self.deny_retry_schedule(
                request_id,
                ResourceRetryDenialClass::NonRetryableRequest,
                retry_decision_digest.clone(),
                None,
                telemetry,
            );
        }
        if self.pending_retry_by_request.contains_key(&request_id) {
            return self.deny_retry_schedule(
                request_id,
                ResourceRetryDenialClass::RetryAlreadyScheduled,
                retry_decision_digest.clone(),
                retry_budget_charge,
                telemetry,
            );
        }
        if let Some(charge) = retry_budget_charge {
            if charge.spent_before() >= charge.limit() {
                return self.deny_retry_schedule(
                    request_id,
                    ResourceRetryDenialClass::RetryBudgetExhausted,
                    retry_decision_digest.clone(),
                    Some(charge),
                    telemetry,
                );
            }
            self.retry_budget_ledger.consume(&in_flight, charge);
        }
        let scheduled = ScheduledResourceRetry::new(
            handle,
            self.issue_retry_ordinal(),
            reason,
            next_attempt,
            backoff_wake_id,
            scheduled_delay,
            retry_decision_digest,
            retry_budget_charge.map(|charge| charge.scope()),
            retry_budget_charge.map(|charge| charge.limit()),
            retry_budget_charge.map(|charge| charge.spent_before().saturating_add(1)),
        );
        self.pending_retry_by_request
            .insert(request_id, scheduled.clone());
        self.pending_retry_by_wake
            .insert(backoff_wake_id, request_id);
        self.pending_retry_by_node
            .insert(in_flight.node(), scheduled.clone());
        telemetry.resource_retry_schedule_count += 1;
        telemetry.resource_retry_temporal_wake_footprint = telemetry
            .resource_retry_temporal_wake_footprint
            .saturating_add(1);
        let performance = Self::record_boundary_performance(
            telemetry,
            ResourceBoundaryPerformanceEnvelope::retry_schedule(
                1,
                0,
                u32::from(retry_budget_charge.is_some()),
            ),
        );

        ResourceRetryScheduleReport::admitted(scheduled, performance)
    }

    pub(super) fn prepare_scheduled_resource_retry(
        &mut self,
        handle: ResourceRequestHandle,
        ready_wake: &ReadyTemporalWake,
        telemetry: &mut ResourceTelemetry,
    ) -> Result<PreparedScheduledResourceRetry, ResourceRetryAdmissionReport> {
        telemetry.resource_hot_in_flight_lookup_count += 1;
        let request_id = handle.request_id();
        let Some(scheduled) = self.pending_retry_by_request.get(&request_id).cloned() else {
            return Err(self.deny_retry_admission(
                request_id,
                ResourceRetryDenialClass::MissingRetryBackoffWake,
                self.retry_policy_decision_digest_for_request(request_id),
                telemetry,
            ));
        };
        if scheduled.previous() != handle {
            return Err(self.deny_retry_admission(
                request_id,
                ResourceRetryDenialClass::UnknownOrStaleRequest,
                scheduled.policy_decision_digest().clone(),
                telemetry,
            ));
        }
        if scheduled.backoff_wake_id() != ready_wake.id() {
            return Err(self.deny_retry_admission(
                request_id,
                ResourceRetryDenialClass::WakeMismatch,
                scheduled.policy_decision_digest().clone(),
                telemetry,
            ));
        }

        let Some(previous) = self.in_flight_by_request.get(&request_id).cloned() else {
            return Err(self.deny_retry_admission(
                request_id,
                ResourceRetryDenialClass::UnknownOrStaleRequest,
                scheduled.policy_decision_digest().clone(),
                telemetry,
            ));
        };
        if previous.handle() != handle {
            return Err(self.deny_retry_admission(
                request_id,
                ResourceRetryDenialClass::UnknownOrStaleRequest,
                scheduled.policy_decision_digest().clone(),
                telemetry,
            ));
        }
        if self
            .active_request_by_node
            .get(&previous.node())
            .is_some_and(|active| *active != request_id)
        {
            return Err(self.deny_retry_admission(
                request_id,
                ResourceRetryDenialClass::SupersededByNewerRequest,
                scheduled.policy_decision_digest().clone(),
                telemetry,
            ));
        }
        Ok(PreparedScheduledResourceRetry {
            scheduled,
            previous,
        })
    }

    pub(super) fn admit_prepared_scheduled_resource_retry(
        &mut self,
        prepared: PreparedScheduledResourceRetry,
        ready_wake: ReadyTemporalWake,
        branch_id: SignalBranchId,
        generation_started_tick: crate::data::temporal::ClockTick,
        resolved_timeout: Option<ScheduledResourceTimeoutAdmission>,
        telemetry: &mut ResourceTelemetry,
    ) -> ResourceRetryAdmissionReport {
        let PreparedScheduledResourceRetry {
            scheduled,
            previous,
        } = prepared;
        let scheduled_timeout_wake_footprint = u32::from(resolved_timeout.is_some());
        let request_id = previous.handle().request_id();
        let retry_request_id = self.issue_request_id();
        let branch_epoch = ResourceBranchEpoch::new(branch_id, self.restore_epoch);
        let admitted = AdmittedResourceRequest::new(
            retry_request_id,
            previous.generation(),
            branch_epoch,
            scheduled.next_attempt(),
        );
        let handle = admitted.handle();
        let ordinal = self.issue_lifecycle_ordinal();
        let output_continuity = self.pending_output_continuity_for_node(
            previous.node(),
            previous.descriptor_id(),
            telemetry,
        );
        let lifecycle = ResourceLifecycleSummary::new(
            previous.node(),
            ResourceLifecycleClass::Pending,
            output_continuity,
            ordinal,
        );
        let transition = ResourceLifecycleTransition::new(
            previous.node(),
            ResourceLifecycleClass::TimedOut,
            ResourceLifecycleClass::Pending,
            ResourceLifecycleTransitionKind::RequestAdmitted,
            ordinal,
            output_continuity,
        );
        let (
            timeout_duration,
            timeout_due_tick,
            timeout_outcome_class,
            timeout_deadline_authority,
            timeout_decision_digest,
            timeout_wake_id,
        ) = match resolved_timeout {
            Some(timeout) => (
                Some(timeout.timeout_duration),
                Some(timeout.due_tick),
                timeout.outcome_class,
                timeout.deadline_authority,
                timeout.decision_digest,
                Some(timeout.wake_id),
            ),
            None => (
                previous.timeout_duration(),
                previous.timeout_due_tick(),
                previous.timeout_outcome_class(),
                previous.timeout_deadline_authority(),
                previous.timeout_decision_digest().clone(),
                previous.timeout_wake_id(),
            ),
        };
        let mut in_flight = InFlightResourceRequest::new(
            handle,
            previous.node(),
            previous.descriptor_id(),
            previous.generation(),
            scheduled.next_attempt(),
            previous.request_intent_digest().clone(),
            generation_started_tick,
            ordinal,
            timeout_duration,
            timeout_due_tick,
            timeout_outcome_class,
            timeout_deadline_authority,
            timeout_decision_digest,
        );
        if let Some(wake_id) = timeout_wake_id {
            in_flight.attach_timeout_wake(wake_id);
            telemetry.resource_timeout_temporal_wake_footprint = telemetry
                .resource_timeout_temporal_wake_footprint
                .saturating_add(1);
        }
        self.pending_retry_by_request.remove(&request_id);
        self.pending_retry_by_wake.remove(&ready_wake.id());
        self.pending_retry_by_node.remove(&previous.node());
        self.retain_retry_lineage(previous.node(), scheduled.clone());
        self.in_flight_by_request
            .insert(retry_request_id, in_flight);
        self.active_request_by_node
            .insert(previous.node(), retry_request_id);
        self.lifecycle_by_node.insert(previous.node(), lifecycle);
        self.clear_latest_denied_completion_for_node(previous.node());

        telemetry.resource_retry_admission_count += 1;
        telemetry.resource_in_flight_request_count = self.in_flight_by_request.len() as u64;
        telemetry.resource_in_flight_frontier_width = telemetry
            .resource_in_flight_frontier_width
            .max(self.active_request_by_node.len() as u64);
        telemetry.resource_retry_temporal_wake_footprint = telemetry
            .resource_retry_temporal_wake_footprint
            .saturating_add(1);
        let performance = Self::record_boundary_performance(
            telemetry,
            ResourceBoundaryPerformanceEnvelope::retry_admission(
                1,
                0,
                1,
                1 + scheduled_timeout_wake_footprint,
            )
            .with_output_continuity_classification_width(1),
        );

        ResourceRetryAdmissionReport::admitted(
            AdmittedResourceRetry::new(scheduled, admitted, ready_wake),
            lifecycle,
            transition,
            performance,
        )
    }

    pub fn admit_resource_completion(
        &mut self,
        raw: RawCompletionEnvelope,
        telemetry: &mut ResourceTelemetry,
    ) -> ResourceCompletionAdmissionReport {
        self.admit_resource_completion_with_boundary(raw, telemetry, true)
    }

    fn admit_resource_completion_with_boundary(
        &mut self,
        raw: RawCompletionEnvelope,
        telemetry: &mut ResourceTelemetry,
        count_scalar_boundary: bool,
    ) -> ResourceCompletionAdmissionReport {
        telemetry.resource_completion_validation_count += 1;
        telemetry.resource_hot_in_flight_lookup_count += 1;

        let Some(in_flight) = self.in_flight_by_request.get(&raw.request_id()).cloned() else {
            if let Some(retained) = self
                .retained_in_flight_history_by_request
                .get(&raw.request_id())
                .cloned()
            {
                let retained_node = retained.node();
                let class = self.retained_completion_denial_class(&raw, retained);
                return self.deny_completion(
                    &raw,
                    class,
                    Some(retained_node),
                    telemetry,
                    count_scalar_boundary,
                );
            }
            if let Some(pruned) = self
                .pruned_in_flight_history_by_request
                .get(&raw.request_id())
                .cloned()
            {
                let class = Self::pruned_completion_denial_class(&raw, pruned);
                return self.deny_completion(&raw, class, None, telemetry, count_scalar_boundary);
            }
            return self.deny_completion(
                &raw,
                CompletionDenialClass::UnknownRequest,
                None,
                telemetry,
                count_scalar_boundary,
            );
        };

        let handle = in_flight.handle();
        if handle.request_id() != raw.request_id()
            || handle.generation() != raw.generation()
            || handle.branch_epoch() != raw.branch_epoch()
            || in_flight.attempt() != raw.attempt()
        {
            return self.deny_completion(
                &raw,
                CompletionDenialClass::Stale,
                Some(in_flight.node()),
                telemetry,
                count_scalar_boundary,
            );
        }

        if in_flight.status() == ResourceInFlightStatus::Superseded {
            return self.deny_completion(
                &raw,
                CompletionDenialClass::Superseded,
                Some(in_flight.node()),
                telemetry,
                count_scalar_boundary,
            );
        }

        if in_flight.status() == ResourceInFlightStatus::Cancelled {
            return self.deny_completion(
                &raw,
                CompletionDenialClass::Cancelled,
                Some(in_flight.node()),
                telemetry,
                count_scalar_boundary,
            );
        }

        if in_flight.status() == ResourceInFlightStatus::Rejected {
            return self.deny_completion(
                &raw,
                CompletionDenialClass::Rejected,
                Some(in_flight.node()),
                telemetry,
                count_scalar_boundary,
            );
        }

        if in_flight.status() == ResourceInFlightStatus::TimedOut {
            return self.deny_completion(
                &raw,
                CompletionDenialClass::TimedOut,
                Some(in_flight.node()),
                telemetry,
                count_scalar_boundary,
            );
        }

        if in_flight.status() != ResourceInFlightStatus::Active
            || in_flight.lifecycle() != ResourceLifecycleClass::Pending
        {
            return self.deny_completion(
                &raw,
                CompletionDenialClass::Retired,
                Some(in_flight.node()),
                telemetry,
                count_scalar_boundary,
            );
        }

        let Some(descriptor) = self.descriptors.get(&in_flight.descriptor_id()) else {
            return self.deny_completion(
                &raw,
                CompletionDenialClass::Impossible,
                Some(in_flight.node()),
                telemetry,
                count_scalar_boundary,
            );
        };

        if descriptor.payload_contract_digest() != raw.payload_contract_digest() {
            return self.deny_completion(
                &raw,
                CompletionDenialClass::Malformed,
                Some(in_flight.node()),
                telemetry,
                count_scalar_boundary,
            );
        }

        if descriptor
            .max_payload_bytes()
            .is_some_and(|max| raw.payload_byte_len() > max)
        {
            return self.deny_completion(
                &raw,
                CompletionDenialClass::Partial,
                Some(in_flight.node()),
                telemetry,
                count_scalar_boundary,
            );
        }

        if in_flight.lifecycle() != ResourceLifecycleClass::Pending {
            return self.deny_completion(
                &raw,
                CompletionDenialClass::Impossible,
                Some(in_flight.node()),
                telemetry,
                count_scalar_boundary,
            );
        }

        let validated =
            ValidatedCompletionEnvelope::new(handle, raw.attempt(), raw.payload_byte_len());
        self.admit_validated_completion(validated, in_flight, telemetry, count_scalar_boundary)
    }

    pub fn admit_resource_completion_batch(
        &mut self,
        completions: impl IntoIterator<Item = RawCompletionEnvelope>,
        telemetry: &mut ResourceTelemetry,
    ) -> ResourceCompletionBatchAdmissionReport {
        let mut completions = completions.into_iter().collect::<Vec<_>>();
        let input_width = completions.len() as u32;
        let density_strategy = ResourceDensityStrategy::completion_batch(
            input_width,
            self.in_flight_by_request.len() as u32,
        );
        completions.sort();

        let mut admitted_completions = Vec::new();
        let mut denied_completions = Vec::new();
        let mut seen_identities = BTreeMap::<
            (
                ResourceRequestId,
                ResourceGeneration,
                ResourceBranchEpoch,
                ResourceAttemptId,
            ),
            RawCompletionEnvelope,
        >::new();
        let mut duplicate_width = 0_u32;

        for raw in completions {
            let identity = (
                raw.request_id(),
                raw.generation(),
                raw.branch_epoch(),
                raw.attempt(),
            );
            if let Some(prior) = seen_identities.get(&identity) {
                duplicate_width = duplicate_width.saturating_add(1);
                telemetry.resource_completion_validation_count += 1;
                let class = if prior == &raw {
                    CompletionDenialClass::Duplicate
                } else {
                    CompletionDenialClass::Contradictory
                };
                let denied_node = self
                    .in_flight_by_request
                    .get(&raw.request_id())
                    .map(|in_flight| in_flight.node())
                    .or_else(|| {
                        self.retained_in_flight_history_by_request
                            .get(&raw.request_id())
                            .map(|retained| retained.node())
                    });
                let denied = self
                    .deny_completion(&raw, class, denied_node, telemetry, false)
                    .denied_completion()
                    .expect("batch duplicate denial should retain denied completion");
                denied_completions.push(denied);
                continue;
            }
            seen_identities.insert(identity, raw.clone());

            let report = self.admit_resource_completion_with_boundary(raw, telemetry, false);
            if let Some(admitted) = report.admitted_completion() {
                admitted_completions.push(admitted);
            }
            if let Some(denied) = report.denied_completion() {
                denied_completions.push(denied);
            }
        }

        telemetry.resource_completion_batch_admission_count += 1;
        let admitted_count = admitted_completions.len() as u32;
        let denied_count = denied_completions.len() as u32;
        let performance = Self::record_boundary_performance(
            telemetry,
            ResourceBoundaryPerformanceEnvelope::completion_batch_admission(
                input_width,
                admitted_count,
                denied_count,
            )
            .with_density_strategy(density_strategy),
        );
        ResourceCompletionBatchAdmissionReport::new(
            admitted_completions,
            denied_completions,
            input_width,
            duplicate_width,
            performance,
        )
    }

    pub fn compact_lifecycle_history(
        &mut self,
        max_reclaimed: u32,
        telemetry: &mut ResourceTelemetry,
    ) -> ResourceLifecycleRetentionCompactionReport {
        self.compact_lifecycle_history_with_budget(
            max_reclaimed,
            ResourceRetentionCompactionBudget::unbounded(),
            telemetry,
        )
    }

    fn retention_availability_class_for_lifecycle(
        descriptor: &LoweredResourceDescriptor,
        lifecycle: ResourceLifecycleClass,
    ) -> Option<ResourceRetainedHistoryAvailabilityClass> {
        match descriptor.retention_decision_plan().class() {
            crate::data::resource::ResourceRetentionDecisionClass::RetainAllTransitions => None,
            crate::data::resource::ResourceRetentionDecisionClass::TerminalSummariesOnly => {
                Some(ResourceRetainedHistoryAvailabilityClass::TerminalSummaryOnly)
            }
            crate::data::resource::ResourceRetentionDecisionClass::CompactSuperseded => {
                Some(ResourceRetainedHistoryAvailabilityClass::CompactSuperseded)
            }
            crate::data::resource::ResourceRetentionDecisionClass::CompactCancelled => {
                Some(ResourceRetainedHistoryAvailabilityClass::CompactCancelled)
            }
            crate::data::resource::ResourceRetentionDecisionClass::CompactTimedOut => {
                Some(ResourceRetainedHistoryAvailabilityClass::CompactTimedOut)
            }
        }
        .filter(|_| {
            descriptor
                .retention_decision_plan()
                .permits_compaction_for_lifecycle(lifecycle)
        })
    }

    fn retention_availability_from_in_flight(
        descriptor: &LoweredResourceDescriptor,
        in_flight: InFlightResourceRequest,
        class: ResourceRetainedHistoryAvailabilityClass,
    ) -> ResourceRetainedHistoryAvailability {
        ResourceRetainedHistoryAvailability::new(
            in_flight.handle(),
            in_flight.attempt(),
            in_flight.node(),
            in_flight.lifecycle(),
            class,
            descriptor.retention_decision_plan().descriptor_id(),
            descriptor.retention_decision_plan().class(),
            descriptor
                .retention_decision_plan()
                .decision_digest()
                .clone(),
        )
    }

    fn pruned_denied_completion_availability(
        denied: DeniedResourceCompletion,
    ) -> ResourceRetainedDeniedCompletionAvailability {
        ResourceRetainedDeniedCompletionAvailability::new(
            denied.denial_id(),
            denied.request_id(),
            denied.node(),
            denied.class(),
            ResourceRetainedDeniedCompletionAvailabilityClass::PrunedByRetainedDeniedCompletionLimit,
        )
    }

    fn retain_retry_lineage(&mut self, node: ResourceNodeId, scheduled: ScheduledResourceRetry) {
        let retained = RetainedResourceRetryLineage::from_scheduled(node, scheduled);
        self.retained_retry_lineage_by_ordinal
            .insert(retained.retry_ordinal(), retained);
    }

    pub fn compact_lifecycle_history_with_retained_limit(
        &mut self,
        max_reclaimed: u32,
        retained_history_limit: Option<u32>,
        telemetry: &mut ResourceTelemetry,
    ) -> ResourceLifecycleRetentionCompactionReport {
        let budget = retained_history_limit.map_or_else(
            ResourceRetentionCompactionBudget::unbounded,
            ResourceRetentionCompactionBudget::retained_history_limit_only,
        );
        self.compact_lifecycle_history_with_budget(max_reclaimed, budget, telemetry)
    }

    pub fn compact_lifecycle_history_with_budget(
        &mut self,
        max_reclaimed: u32,
        budget: ResourceRetentionCompactionBudget,
        telemetry: &mut ResourceTelemetry,
    ) -> ResourceLifecycleRetentionCompactionReport {
        let selected = self
            .terminal_in_flight_by_request
            .iter()
            .copied()
            .filter(|request_id| !self.pending_retry_by_request.contains_key(request_id))
            .filter(|request_id| {
                self.terminal_in_flight_record(*request_id)
                    .and_then(|request| request.managed_queue())
                    .is_none_or(ResourceManagedQueueState::is_empty)
            })
            .filter(|request_id| {
                let Some(in_flight) = self.terminal_in_flight_record(*request_id) else {
                    return false;
                };
                let Some(descriptor) = self.descriptor_for_node(in_flight.node()) else {
                    return false;
                };
                descriptor.retention_decision_plan().retains_rich_history()
                    || descriptor
                        .retention_decision_plan()
                        .permits_compaction_for_lifecycle(in_flight.lifecycle())
            })
            .take(max_reclaimed as usize)
            .collect::<Vec<_>>();
        let selected_terminal_count = selected.len() as u32;
        let mut reclaimed_in_flight_count = 0_u32;
        let mut retained_history_write_count = 0_u32;
        let mut retained_history_pruned_count = 0_u32;
        let mut retained_history_unavailable_count = 0_u32;
        let mut retained_denied_completion_pruned_count = 0_u32;
        let mut retained_retry_lineage_pruned_count = 0_u32;
        let mut compacted_terminal_summary_count = 0_u32;
        let mut compacted_superseded_count = 0_u32;
        let mut compacted_cancelled_count = 0_u32;
        let mut compacted_timed_out_count = 0_u32;

        for request_id in selected {
            self.terminal_in_flight_by_request.remove(&request_id);
            let Some(in_flight) = self
                .in_flight_by_request
                .remove(&request_id)
                .filter(|in_flight| in_flight.lifecycle().is_terminal())
            else {
                continue;
            };
            reclaimed_in_flight_count = reclaimed_in_flight_count.saturating_add(1);
            let Some(descriptor) = self.descriptor_for_node(in_flight.node()).cloned() else {
                continue;
            };
            if descriptor.retention_decision_plan().retains_rich_history() {
                self.retained_in_flight_history_by_request
                    .insert(request_id, in_flight);
                retained_history_write_count = retained_history_write_count.saturating_add(1);
                continue;
            }
            let Some(class) = Self::retention_availability_class_for_lifecycle(
                &descriptor,
                in_flight.lifecycle(),
            ) else {
                continue;
            };
            self.pruned_in_flight_history_by_request.insert(
                request_id,
                Self::retention_availability_from_in_flight(&descriptor, in_flight, class),
            );
            retained_history_unavailable_count =
                retained_history_unavailable_count.saturating_add(1);
            match class {
                ResourceRetainedHistoryAvailabilityClass::TerminalSummaryOnly => {
                    compacted_terminal_summary_count =
                        compacted_terminal_summary_count.saturating_add(1);
                }
                ResourceRetainedHistoryAvailabilityClass::CompactSuperseded => {
                    compacted_superseded_count = compacted_superseded_count.saturating_add(1);
                }
                ResourceRetainedHistoryAvailabilityClass::CompactCancelled => {
                    compacted_cancelled_count = compacted_cancelled_count.saturating_add(1);
                }
                ResourceRetainedHistoryAvailabilityClass::CompactTimedOut => {
                    compacted_timed_out_count = compacted_timed_out_count.saturating_add(1);
                }
                ResourceRetainedHistoryAvailabilityClass::PrunedByRetainedHistoryLimit => {}
            }
        }

        if let Some(retained_history_limit) = budget.retained_lifecycle_history_limit() {
            while self.retained_in_flight_history_by_request.len() > retained_history_limit as usize
            {
                let Some(request_id) = self
                    .retained_in_flight_history_by_request
                    .keys()
                    .next()
                    .copied()
                else {
                    break;
                };
                if let Some(pruned) = self
                    .retained_in_flight_history_by_request
                    .remove(&request_id)
                {
                    let descriptor = self
                        .descriptor_for_node(pruned.node())
                        .cloned()
                        .expect("retained history pruning requires descriptor for node");
                    self.pruned_in_flight_history_by_request.insert(
                        request_id,
                        Self::retention_availability_from_in_flight(
                            &descriptor,
                            pruned,
                            ResourceRetainedHistoryAvailabilityClass::PrunedByRetainedHistoryLimit,
                        ),
                    );
                    retained_history_unavailable_count =
                        retained_history_unavailable_count.saturating_add(1);
                }
                retained_history_pruned_count = retained_history_pruned_count.saturating_add(1);
            }
        }

        if let Some(retained_denied_completion_limit) = budget.retained_denied_completion_limit() {
            while self.denied_completions.len() > retained_denied_completion_limit as usize {
                let Some(denial_id) = self.denied_completions.keys().next().copied() else {
                    break;
                };
                if let Some(denied) = self.denied_completions.remove(&denial_id) {
                    self.pruned_denied_completions_by_id.insert(
                        denial_id,
                        Self::pruned_denied_completion_availability(denied),
                    );
                    if let Some(node) = denied.node() {
                        self.rebuild_latest_denied_completion_for_node(node);
                    }
                }
                retained_denied_completion_pruned_count =
                    retained_denied_completion_pruned_count.saturating_add(1);
            }
        }

        if let Some(retained_retry_lineage_limit) = budget.retained_retry_lineage_limit() {
            while self.retained_retry_lineage_by_ordinal.len()
                > retained_retry_lineage_limit as usize
            {
                let Some(retry_ordinal) = self
                    .retained_retry_lineage_by_ordinal
                    .keys()
                    .next()
                    .copied()
                else {
                    break;
                };
                if let Some(retained) = self
                    .retained_retry_lineage_by_ordinal
                    .remove(&retry_ordinal)
                {
                    self.pruned_retry_lineage_by_ordinal.insert(
                        retry_ordinal,
                        ResourceRetainedRetryLineageAvailability::from_retained(
                            retained,
                            ResourceRetainedRetryLineageAvailabilityClass::PrunedByRetainedRetryLineageLimit,
                        ),
                    );
                }
                retained_retry_lineage_pruned_count =
                    retained_retry_lineage_pruned_count.saturating_add(1);
            }
        }

        telemetry.resource_hot_in_flight_compaction_count += 1;
        telemetry.resource_in_flight_retired_record_count = telemetry
            .resource_in_flight_retired_record_count
            .saturating_add(selected_terminal_count as u64);
        telemetry.resource_in_flight_reclaimed_record_count = telemetry
            .resource_in_flight_reclaimed_record_count
            .saturating_add(reclaimed_in_flight_count as u64);
        telemetry.resource_retained_lifecycle_history_write_count = telemetry
            .resource_retained_lifecycle_history_write_count
            .saturating_add(retained_history_write_count as u64);
        telemetry.resource_retained_lifecycle_history_pruned_count = telemetry
            .resource_retained_lifecycle_history_pruned_count
            .saturating_add(retained_history_pruned_count as u64);
        telemetry.resource_retained_denied_completion_count = self.denied_completions.len() as u64;
        telemetry.resource_retained_retry_lineage_count =
            self.retained_retry_lineage_by_ordinal.len() as u64;
        telemetry.resource_retained_history_unavailable_count = telemetry
            .resource_retained_history_unavailable_count
            .saturating_add(retained_history_unavailable_count as u64);
        telemetry.resource_in_flight_request_count = self.in_flight_by_request.len() as u64;

        let retained_history_width = self.retained_in_flight_history_by_request.len() as u32;
        let retained_denied_completion_width = self.denied_completions.len() as u32;
        let retained_retry_lineage_width = self.retained_retry_lineage_by_ordinal.len() as u32;
        let hot_in_flight_width = self.in_flight_by_request.len() as u32;
        let retained_history_decision_digests = self
            .pruned_in_flight_history_by_request
            .values()
            .map(|availability| availability.retention_decision_digest().as_str().to_owned())
            .collect::<Vec<_>>();
        let retry_lineage_decision_digests = self
            .pruned_retry_lineage_by_ordinal
            .values()
            .map(|availability| availability.policy_decision_digest().as_str().to_owned())
            .collect::<Vec<_>>();
        let policy_provenance_digest =
            canonical_digest(&ResourceRetentionCompactionPolicyProvenanceDigestBasis {
                schema_version: "worth.resource.retention-compaction-policy-provenance.v1",
                retained_history_decision_digests: &retained_history_decision_digests,
                retry_lineage_decision_digests: &retry_lineage_decision_digests,
            });
        let performance = Self::record_boundary_performance(
            telemetry,
            ResourceBoundaryPerformanceEnvelope::lifecycle_retention_compaction(
                selected_terminal_count,
                reclaimed_in_flight_count,
                retained_history_write_count,
            ),
        );
        ResourceLifecycleRetentionCompactionReport::new(
            selected_terminal_count,
            reclaimed_in_flight_count,
            retained_history_write_count,
            retained_history_pruned_count,
            retained_history_unavailable_count,
            retained_denied_completion_pruned_count,
            retained_retry_lineage_pruned_count,
            retained_history_width,
            retained_denied_completion_width,
            retained_retry_lineage_width,
            hot_in_flight_width,
            compacted_terminal_summary_count,
            compacted_superseded_count,
            compacted_cancelled_count,
            compacted_timed_out_count,
            policy_provenance_digest,
            performance,
        )
    }

    pub fn stage_admitted_resource_completion(
        &mut self,
        admitted: AdmittedResourceCompletion,
        telemetry: &mut ResourceTelemetry,
    ) -> Result<ResourceCompletionStagingReport, crate::data::error::SignalError> {
        telemetry.resource_hot_in_flight_lookup_count += 1;
        let handle = admitted.handle();
        let Some(in_flight) = self.in_flight_by_request.get(&handle.request_id()) else {
            return Err(crate::data::error::SignalError::invalid_input(format!(
                "cannot stage resource completion for unknown request {}",
                handle.request_id().get()
            )));
        };
        if in_flight.handle() != handle
            || in_flight.status() != ResourceInFlightStatus::Active
            || in_flight.lifecycle() != ResourceLifecycleClass::Pending
        {
            return Err(crate::data::error::SignalError::invalid_input(format!(
                "cannot stage resource completion for non-active request {}",
                handle.request_id().get()
            )));
        }

        telemetry.resource_completion_staging_count += 1;
        let performance = Self::record_boundary_performance(
            telemetry,
            ResourceBoundaryPerformanceEnvelope::completion_staging(),
        );
        Ok(ResourceCompletionStagingReport::new(
            StagedResourceCompletionEffect::new(admitted),
            performance,
        ))
    }

    pub fn stage_denied_resource_completion(
        &mut self,
        denied: DeniedResourceCompletion,
        telemetry: &mut ResourceTelemetry,
    ) -> Result<ResourceCompletionDenialStagingReport, crate::data::error::SignalError> {
        telemetry.resource_hot_in_flight_lookup_count += 1;
        let Some(retained) = self.denied_completions.get(&denied.denial_id()) else {
            return Err(crate::data::error::SignalError::invalid_input(format!(
                "cannot stage unretained denied resource completion {}",
                denied.denial_id().get()
            )));
        };
        if *retained != denied {
            return Err(crate::data::error::SignalError::invalid_input(format!(
                "cannot stage mismatched denied resource completion {}",
                denied.denial_id().get()
            )));
        }

        telemetry.resource_completion_denial_staging_count += 1;
        let performance = Self::record_boundary_performance(
            telemetry,
            ResourceBoundaryPerformanceEnvelope::completion_denial_staging(),
        );
        Ok(ResourceCompletionDenialStagingReport::new(
            StagedDeniedResourceCompletionEffect::new(denied),
            performance,
        ))
    }

    pub fn rollback_staged_resource_completion(
        &mut self,
        staged: StagedResourceCompletionEffect,
        telemetry: &mut ResourceTelemetry,
    ) -> ResourceCompletionRollbackReport {
        telemetry.resource_completion_rollback_count += 1;
        let performance = Self::record_boundary_performance(
            telemetry,
            ResourceBoundaryPerformanceEnvelope::completion_rollback(1, 0),
        );
        ResourceCompletionRollbackReport::new(
            RolledBackResourceCompletionArtifact::admitted(staged),
            performance,
        )
    }

    pub fn rollback_staged_denied_resource_completion(
        &mut self,
        staged: StagedDeniedResourceCompletionEffect,
        telemetry: &mut ResourceTelemetry,
    ) -> ResourceCompletionRollbackReport {
        telemetry.resource_completion_rollback_count += 1;
        let performance = Self::record_boundary_performance(
            telemetry,
            ResourceBoundaryPerformanceEnvelope::completion_rollback(0, 1),
        );
        ResourceCompletionRollbackReport::new(
            RolledBackResourceCompletionArtifact::denied(staged),
            performance,
        )
    }

    pub fn commit_staged_resource_completion(
        &mut self,
        staged: StagedResourceCompletionEffect,
        telemetry: &mut ResourceTelemetry,
    ) -> Result<ResourceCompletionCommitReport, crate::data::error::SignalError> {
        telemetry.resource_hot_in_flight_lookup_count += 1;
        let admitted = staged.admitted_completion();
        let handle = admitted.handle();
        let Some(in_flight) = self.in_flight_by_request.get_mut(&handle.request_id()) else {
            return Err(crate::data::error::SignalError::invalid_input(format!(
                "cannot commit staged resource completion for unknown request {}",
                handle.request_id().get()
            )));
        };
        if in_flight.handle() != handle
            || in_flight.status() != ResourceInFlightStatus::Active
            || in_flight.lifecycle() != ResourceLifecycleClass::Pending
        {
            return Err(crate::data::error::SignalError::invalid_input(format!(
                "cannot commit staged resource completion for non-active request {}",
                handle.request_id().get()
            )));
        }
        let transition = admitted.lifecycle_transition();
        in_flight.fulfill(transition.ordinal());
        self.mark_terminal_in_flight(handle.request_id());
        if self
            .active_request_by_node
            .get(&admitted.node())
            .is_some_and(|active| *active == handle.request_id())
        {
            self.active_request_by_node.remove(&admitted.node());
        }
        self.stale_after_wake_by_node.remove(&admitted.node());
        let lifecycle = ResourceLifecycleSummary::new(
            admitted.node(),
            ResourceLifecycleClass::Fulfilled,
            ResourceOutputContinuity::OutputReplaced,
            transition.ordinal(),
        );
        self.lifecycle_by_node.insert(admitted.node(), lifecycle);
        self.clear_latest_denied_completion_for_node(admitted.node());
        self.retry_budget_ledger
            .clear_request_generation(handle.generation());
        let committed = CommittedResourceCompletionArtifact::new(staged, transition);

        telemetry.resource_completion_commit_count += 1;
        telemetry.resource_output_continuity_decision_count += 1;
        telemetry.resource_in_flight_request_count = self.in_flight_by_request.len() as u64;
        let performance = Self::record_boundary_performance(
            telemetry,
            ResourceBoundaryPerformanceEnvelope::completion_commit(1)
                .with_output_continuity_classification_width(1),
        );

        Ok(ResourceCompletionCommitReport::new(
            committed,
            lifecycle,
            transition,
            performance,
        ))
    }

    pub fn cancel_resource_request(
        &mut self,
        handle: ResourceRequestHandle,
        reason: ResourceCancellationReason,
        telemetry: &mut ResourceTelemetry,
    ) -> ResourceCancellationReport {
        telemetry.resource_hot_in_flight_lookup_count += 1;
        let request_id = handle.request_id();
        let Some(in_flight) = self.in_flight_by_request.get(&request_id).cloned() else {
            return self.deny_cancellation(
                request_id,
                ResourceCancellationDenialClass::UnknownOrStaleRequest,
                telemetry,
            );
        };

        if in_flight.handle() != handle {
            return self.deny_cancellation(
                request_id,
                ResourceCancellationDenialClass::UnknownOrStaleRequest,
                telemetry,
            );
        }

        if in_flight.status() != ResourceInFlightStatus::Active
            || in_flight.lifecycle() != ResourceLifecycleClass::Pending
        {
            return self.deny_cancellation(
                request_id,
                ResourceCancellationDenialClass::NonActiveRequest,
                telemetry,
            );
        }

        let applied = self
            .apply_resource_cancellation(request_id, reason, &mut BTreeSet::new(), telemetry)
            .expect("active cancellation should resolve through the runtime");
        let cancelled_width = 1u32.saturating_add(applied.propagated_dependents.len() as u32);
        let dependent_propagation = (!applied.propagated_dependents.is_empty()).then(|| {
            ResourceDependentCancellationPropagation::new(
                handle,
                applied.propagated_dependents.clone(),
            )
        });
        let cancellation_visibility_width = u32::from(
            applied.transition.output_continuity() != ResourceOutputContinuity::NoPriorOutput,
        ) + applied
            .propagated_dependents
            .iter()
            .filter(|cancelled| {
                cancelled.lifecycle_transition().output_continuity()
                    != ResourceOutputContinuity::NoPriorOutput
            })
            .count() as u32;
        telemetry.resource_in_flight_request_count = self.in_flight_by_request.len() as u64;
        let performance = Self::record_boundary_performance(
            telemetry,
            ResourceBoundaryPerformanceEnvelope::cancellation(cancelled_width, 0)
                .with_output_continuity_classification_width(cancellation_visibility_width),
        );

        ResourceCancellationReport::admitted(
            applied.cancelled,
            dependent_propagation,
            applied.lifecycle,
            applied.transition,
            performance,
        )
    }

    pub fn admit_resource_timeout(
        &mut self,
        handle: ResourceRequestHandle,
        ready_wake: ReadyTemporalWake,
        telemetry: &mut ResourceTelemetry,
    ) -> ResourceTimeoutReport {
        telemetry.resource_hot_in_flight_lookup_count += 1;
        let request_id = handle.request_id();
        let Some(in_flight) = self.in_flight_by_request.get(&request_id).cloned() else {
            return self.deny_timeout(
                request_id,
                ResourceTimeoutDenialClass::UnknownOrStaleRequest,
                telemetry,
            );
        };

        if in_flight.handle() != handle {
            return self.deny_timeout(
                request_id,
                ResourceTimeoutDenialClass::UnknownOrStaleRequest,
                telemetry,
            );
        }

        if in_flight.status() != ResourceInFlightStatus::Active
            || in_flight.lifecycle() != ResourceLifecycleClass::Pending
        {
            return self.deny_timeout(
                request_id,
                ResourceTimeoutDenialClass::NonActiveRequest,
                telemetry,
            );
        }

        let Some(timeout_wake_id) = in_flight.timeout_wake_id() else {
            return self.deny_timeout(
                request_id,
                ResourceTimeoutDenialClass::MissingTimeoutWake,
                telemetry,
            );
        };
        if timeout_wake_id != ready_wake.id() {
            return self.deny_timeout(
                request_id,
                ResourceTimeoutDenialClass::WakeMismatch,
                telemetry,
            );
        }
        let Some(timeout_duration) = in_flight.timeout_duration() else {
            return self.deny_timeout(
                request_id,
                ResourceTimeoutDenialClass::MissingTimeoutWake,
                telemetry,
            );
        };
        let timeout_decision_digest = in_flight.timeout_decision_digest().clone();
        let timeout_outcome_class = in_flight.timeout_outcome_class();
        let timeout_deadline_authority = in_flight.timeout_deadline_authority();
        let (output_continuity, terminal_visibility_classified) = self
            .classify_terminal_output_continuity_for_node(
                in_flight.node(),
                in_flight.descriptor_id(),
                ResourceTerminalVisibilityCause::Timeout,
                telemetry,
            );

        let lifecycle_ordinal = self.issue_lifecycle_ordinal();
        let timeout_ordinal = self.issue_timeout_ordinal();
        let transition = ResourceLifecycleTransition::new(
            in_flight.node(),
            ResourceLifecycleClass::Pending,
            ResourceLifecycleClass::TimedOut,
            ResourceLifecycleTransitionKind::RequestTimedOut,
            lifecycle_ordinal,
            output_continuity,
        );
        let timed_out = TimedOutResourceRequest::new(
            handle,
            timeout_ordinal,
            ready_wake,
            timeout_duration,
            timeout_outcome_class,
            timeout_deadline_authority,
            timeout_decision_digest,
            transition,
        );
        let lifecycle = ResourceLifecycleSummary::new(
            in_flight.node(),
            ResourceLifecycleClass::TimedOut,
            output_continuity,
            lifecycle_ordinal,
        );

        let in_flight_mut = self
            .in_flight_by_request
            .get_mut(&request_id)
            .expect("in-flight request was just resolved for timeout");
        in_flight_mut.timeout(lifecycle_ordinal);
        self.mark_terminal_in_flight(request_id);
        if self
            .active_request_by_node
            .get(&in_flight.node())
            .is_some_and(|active| *active == request_id)
        {
            self.active_request_by_node.remove(&in_flight.node());
        }
        self.lifecycle_by_node.insert(in_flight.node(), lifecycle);
        self.clear_latest_denied_completion_for_node(in_flight.node());

        telemetry.resource_timeout_admission_count += 1;
        telemetry.resource_in_flight_request_count = self.in_flight_by_request.len() as u64;
        telemetry.resource_timeout_temporal_wake_footprint = telemetry
            .resource_timeout_temporal_wake_footprint
            .saturating_add(1);
        let performance = Self::record_boundary_performance(
            telemetry,
            ResourceBoundaryPerformanceEnvelope::timeout_admission(1, 0, 1)
                .with_output_continuity_classification_width(u32::from(
                    terminal_visibility_classified,
                )),
        );

        ResourceTimeoutReport::admitted(timed_out, lifecycle, transition, performance)
    }

    fn deny_cancellation(
        &mut self,
        request_id: ResourceRequestId,
        class: ResourceCancellationDenialClass,
        telemetry: &mut ResourceTelemetry,
    ) -> ResourceCancellationReport {
        telemetry.resource_cancellation_denial_count += 1;
        match class {
            ResourceCancellationDenialClass::UnknownOrStaleRequest => {
                telemetry.resource_stale_cancellation_denial_count += 1
            }
            ResourceCancellationDenialClass::NonActiveRequest => {
                telemetry.resource_non_active_cancellation_denial_count += 1
            }
        }
        let performance = Self::record_boundary_performance(
            telemetry,
            ResourceBoundaryPerformanceEnvelope::cancellation(0, 1),
        );
        ResourceCancellationReport::denied(
            DeniedResourceCancellation::new(request_id, class),
            performance,
        )
    }

    fn deny_rejection(
        &mut self,
        request_id: ResourceRequestId,
        class: ResourceRejectionDenialClass,
        telemetry: &mut ResourceTelemetry,
    ) -> ResourceRejectionReport {
        telemetry.resource_rejection_denial_count += 1;
        match class {
            ResourceRejectionDenialClass::UnknownOrStaleRequest => {
                telemetry.resource_stale_rejection_denial_count += 1
            }
            ResourceRejectionDenialClass::NonActiveRequest => {
                telemetry.resource_non_active_rejection_denial_count += 1
            }
        }
        let performance = Self::record_boundary_performance(
            telemetry,
            ResourceBoundaryPerformanceEnvelope::rejection_admission(0, 1),
        );
        ResourceRejectionReport::denied(
            DeniedResourceRejection::new(request_id, class),
            performance,
        )
    }

    pub fn reject_resource_request(
        &mut self,
        handle: ResourceRequestHandle,
        reason: ResourceRejectionReason,
        telemetry: &mut ResourceTelemetry,
    ) -> ResourceRejectionReport {
        let request_id = handle.request_id();
        let Some(in_flight) = self.in_flight_by_request.get(&request_id).cloned() else {
            return self.deny_rejection(
                request_id,
                ResourceRejectionDenialClass::UnknownOrStaleRequest,
                telemetry,
            );
        };
        if in_flight.handle() != handle {
            return self.deny_rejection(
                request_id,
                ResourceRejectionDenialClass::UnknownOrStaleRequest,
                telemetry,
            );
        }
        if in_flight.status() != ResourceInFlightStatus::Active
            || in_flight.lifecycle() != ResourceLifecycleClass::Pending
        {
            return self.deny_rejection(
                request_id,
                ResourceRejectionDenialClass::NonActiveRequest,
                telemetry,
            );
        }

        let rejection_digest = ResourcePolicyDigest::new(format!(
            "resource-rejection:{}:{}",
            handle.request_id().get(),
            match reason {
                ResourceRejectionReason::HostFailure => "host-failure",
                ResourceRejectionReason::SemanticFailure => "semantic-failure",
            }
        ));
        let lifecycle_ordinal = self.issue_lifecycle_ordinal();
        let rejection_ordinal = self.issue_rejection_ordinal();
        let (output_continuity, terminal_visibility_classified) = self
            .classify_terminal_output_continuity_for_node(
                in_flight.node(),
                in_flight.descriptor_id(),
                ResourceTerminalVisibilityCause::Rejection,
                telemetry,
            );
        let transition = ResourceLifecycleTransition::new(
            in_flight.node(),
            ResourceLifecycleClass::Pending,
            ResourceLifecycleClass::Rejected,
            ResourceLifecycleTransitionKind::RequestRejected,
            lifecycle_ordinal,
            output_continuity,
        );
        let lifecycle = ResourceLifecycleSummary::new(
            in_flight.node(),
            ResourceLifecycleClass::Rejected,
            output_continuity,
            lifecycle_ordinal,
        );
        let in_flight_mut = self
            .in_flight_by_request
            .get_mut(&request_id)
            .expect("in-flight request was just resolved for rejection");
        in_flight_mut.reject(lifecycle_ordinal);
        self.mark_terminal_in_flight(request_id);
        if self
            .active_request_by_node
            .get(&in_flight.node())
            .is_some_and(|active| *active == request_id)
        {
            self.active_request_by_node.remove(&in_flight.node());
        }
        self.lifecycle_by_node.insert(in_flight.node(), lifecycle);
        self.clear_latest_denied_completion_for_node(in_flight.node());

        telemetry.resource_rejection_admission_count += 1;
        match reason {
            ResourceRejectionReason::HostFailure => {
                telemetry.resource_host_failure_rejection_count += 1
            }
            ResourceRejectionReason::SemanticFailure => {
                telemetry.resource_semantic_rejection_count += 1
            }
        }
        telemetry.resource_in_flight_request_count = self.in_flight_by_request.len() as u64;
        let performance = Self::record_boundary_performance(
            telemetry,
            ResourceBoundaryPerformanceEnvelope::rejection_admission(1, 0)
                .with_output_continuity_classification_width(u32::from(
                    terminal_visibility_classified,
                )),
        );
        ResourceRejectionReport::admitted(
            RejectedResourceRequest::new(
                handle,
                in_flight.node(),
                rejection_ordinal,
                reason,
                rejection_digest,
                transition,
            ),
            lifecycle,
            transition,
            performance,
        )
    }

    fn collect_active_timeout_wakes_for_cancellation_footprint(
        &self,
        request_id: ResourceRequestId,
        expected_handle: ResourceRequestHandle,
        visited_requests: &mut BTreeSet<ResourceRequestId>,
        collected_wakes: &mut BTreeSet<TemporalWakeId>,
    ) {
        if !visited_requests.insert(request_id) {
            return;
        }
        let Some(in_flight) = self.in_flight_by_request.get(&request_id) else {
            return;
        };
        if in_flight.handle() != expected_handle
            || in_flight.status() != ResourceInFlightStatus::Active
            || in_flight.lifecycle() != ResourceLifecycleClass::Pending
        {
            return;
        }
        if let Some(timeout_wake_id) = in_flight.timeout_wake_id() {
            collected_wakes.insert(timeout_wake_id);
        }
        let Some(descriptor) = self.descriptors.get(&in_flight.descriptor_id()) else {
            return;
        };
        for dependent_node in descriptor
            .cancellation_decision_plan()
            .declared_dependent_cancellation_nodes()
        {
            let Some(dependent_request_id) =
                self.active_request_by_node.get(dependent_node).copied()
            else {
                continue;
            };
            let Some(dependent_in_flight) = self.in_flight_by_request.get(&dependent_request_id)
            else {
                continue;
            };
            self.collect_active_timeout_wakes_for_cancellation_footprint(
                dependent_request_id,
                dependent_in_flight.handle(),
                visited_requests,
                collected_wakes,
            );
        }
    }

    fn apply_resource_cancellation(
        &mut self,
        request_id: ResourceRequestId,
        reason: ResourceCancellationReason,
        visited: &mut BTreeSet<ResourceRequestId>,
        telemetry: &mut ResourceTelemetry,
    ) -> Option<AppliedResourceCancellation> {
        if !visited.insert(request_id) {
            return None;
        }

        let in_flight = self.in_flight_by_request.get(&request_id)?.clone();
        if in_flight.status() != ResourceInFlightStatus::Active
            || in_flight.lifecycle() != ResourceLifecycleClass::Pending
        {
            return None;
        }

        let (
            cancellation_digest,
            requests_host_advisory,
            grace_period,
            declared_dependent_cancellation_nodes,
        ) = {
            let descriptor = self
                .descriptors
                .get(&in_flight.descriptor_id())
                .expect("in-flight cancellation must retain a declared descriptor");
            let plan = descriptor.cancellation_decision_plan();
            (
                plan.decision_digest().clone(),
                plan.requests_host_advisory(),
                plan.grace_period(),
                plan.declared_dependent_cancellation_nodes().to_vec(),
            )
        };

        let lifecycle_ordinal = self.issue_lifecycle_ordinal();
        let cancellation_ordinal = self.issue_cancellation_ordinal();
        let (output_continuity, _) = self.classify_terminal_output_continuity_for_node(
            in_flight.node(),
            in_flight.descriptor_id(),
            ResourceTerminalVisibilityCause::Cancellation,
            telemetry,
        );
        let transition = ResourceLifecycleTransition::new(
            in_flight.node(),
            ResourceLifecycleClass::Pending,
            ResourceLifecycleClass::Cancelled,
            ResourceLifecycleTransitionKind::RequestCancelled,
            lifecycle_ordinal,
            output_continuity,
        );
        let lifecycle = ResourceLifecycleSummary::new(
            in_flight.node(),
            ResourceLifecycleClass::Cancelled,
            output_continuity,
            lifecycle_ordinal,
        );

        let in_flight_mut = self
            .in_flight_by_request
            .get_mut(&request_id)
            .expect("in-flight request was just resolved for cancellation");
        in_flight_mut.cancel(lifecycle_ordinal);
        self.mark_terminal_in_flight(request_id);
        if self
            .active_request_by_node
            .get(&in_flight.node())
            .is_some_and(|active| *active == request_id)
        {
            self.active_request_by_node.remove(&in_flight.node());
        }
        self.lifecycle_by_node.insert(in_flight.node(), lifecycle);
        self.clear_latest_denied_completion_for_node(in_flight.node());
        self.retry_budget_ledger
            .clear_request_generation(in_flight.handle().generation());

        telemetry.resource_cancellation_policy_decision_count += 1;
        telemetry.resource_runtime_hard_cancellation_count += 1;
        telemetry.resource_cancellation_count += 1;

        let host_advisory = if requests_host_advisory {
            telemetry.resource_host_cancellation_advisory_count += 1;
            Some(ResourceHostCancellationAdvisory::requested(
                cancellation_digest.clone(),
            ))
        } else {
            None
        };
        let grace_window = grace_period.map(|duration| {
            telemetry.resource_cancellation_grace_period_count += 1;
            ResourceCancellationGraceWindow::new(duration)
        });

        let cancelled = CancelledResourceRequest::new(
            in_flight.handle(),
            cancellation_ordinal,
            reason,
            cancellation_digest,
            host_advisory,
            grace_window,
            transition,
        );

        let mut propagated_dependents = Vec::new();
        for dependent_node in declared_dependent_cancellation_nodes {
            let Some(dependent_request_id) =
                self.active_request_by_node.get(&dependent_node).copied()
            else {
                continue;
            };
            let Some(dependent_cancellation) = self.apply_resource_cancellation(
                dependent_request_id,
                ResourceCancellationReason::RuntimePolicy,
                visited,
                telemetry,
            ) else {
                continue;
            };
            telemetry.resource_dependent_cancellation_propagation_count += 1;
            propagated_dependents.push(dependent_cancellation.cancelled.clone());
            propagated_dependents.extend(dependent_cancellation.propagated_dependents);
        }

        Some(AppliedResourceCancellation {
            cancelled,
            lifecycle,
            transition,
            propagated_dependents,
        })
    }

    fn deny_timeout_heartbeat_extension(
        &mut self,
        request_id: ResourceRequestId,
        class: ResourceTimeoutHeartbeatExtensionDenialClass,
        telemetry: &mut ResourceTelemetry,
    ) -> ResourceTimeoutHeartbeatExtensionReport {
        telemetry.resource_timeout_heartbeat_extension_denial_count += 1;
        if matches!(
            class,
            ResourceTimeoutHeartbeatExtensionDenialClass::PolicyDoesNotAllowHeartbeatExtension
        ) {
            telemetry.resource_timeout_heartbeat_policy_denial_count += 1;
        }
        let performance = Self::record_boundary_performance(
            telemetry,
            ResourceBoundaryPerformanceEnvelope::timeout_heartbeat_extension(0, 1, 0),
        );
        ResourceTimeoutHeartbeatExtensionReport::denied(
            DeniedResourceTimeoutHeartbeatExtension::new(request_id, class),
            performance,
        )
    }

    pub fn deny_timeout_heartbeat_extension_for_report(
        &mut self,
        request_id: ResourceRequestId,
        class: ResourceTimeoutHeartbeatExtensionDenialClass,
        telemetry: &mut ResourceTelemetry,
    ) -> ResourceTimeoutHeartbeatExtensionReport {
        self.deny_timeout_heartbeat_extension(request_id, class, telemetry)
    }

    fn deny_timeout(
        &mut self,
        request_id: ResourceRequestId,
        class: ResourceTimeoutDenialClass,
        telemetry: &mut ResourceTelemetry,
    ) -> ResourceTimeoutReport {
        telemetry.resource_timeout_denial_count += 1;
        match class {
            ResourceTimeoutDenialClass::UnknownOrStaleRequest => {
                telemetry.resource_stale_timeout_denial_count += 1
            }
            ResourceTimeoutDenialClass::NonActiveRequest => {
                telemetry.resource_non_active_timeout_denial_count += 1
            }
            ResourceTimeoutDenialClass::MissingTimeoutWake => {
                telemetry.resource_missing_timeout_wake_denial_count += 1
            }
            ResourceTimeoutDenialClass::WakeMismatch => {
                telemetry.resource_timeout_wake_mismatch_denial_count += 1
            }
        }
        let performance = Self::record_boundary_performance(
            telemetry,
            ResourceBoundaryPerformanceEnvelope::timeout_admission(
                0,
                1,
                u32::from(matches!(class, ResourceTimeoutDenialClass::WakeMismatch)),
            ),
        );
        ResourceTimeoutReport::denied(DeniedResourceTimeout::new(request_id, class), performance)
    }

    fn deny_retry_schedule(
        &mut self,
        request_id: ResourceRequestId,
        class: ResourceRetryDenialClass,
        retry_decision_digest: ResourcePolicyDigest,
        retry_budget_charge: Option<ResourceRetryBudgetCharge>,
        telemetry: &mut ResourceTelemetry,
    ) -> ResourceRetryScheduleReport {
        self.record_retry_denial(class, telemetry);
        let performance = Self::record_boundary_performance(
            telemetry,
            ResourceBoundaryPerformanceEnvelope::retry_schedule(
                0,
                1,
                u32::from(retry_budget_charge.is_some()),
            ),
        );
        ResourceRetryScheduleReport::denied(
            DeniedResourceRetry::new(
                request_id,
                class,
                retry_decision_digest,
                retry_budget_charge.map(|charge| charge.scope()),
                retry_budget_charge.map(|charge| charge.limit()),
                retry_budget_charge.map(|charge| charge.spent_before()),
            ),
            performance,
        )
    }

    fn deny_retry_admission(
        &mut self,
        request_id: ResourceRequestId,
        class: ResourceRetryDenialClass,
        retry_decision_digest: ResourcePolicyDigest,
        telemetry: &mut ResourceTelemetry,
    ) -> ResourceRetryAdmissionReport {
        self.record_retry_denial(class, telemetry);
        let performance = Self::record_boundary_performance(
            telemetry,
            ResourceBoundaryPerformanceEnvelope::retry_admission(
                0,
                1,
                0,
                u32::from(matches!(class, ResourceRetryDenialClass::WakeMismatch)),
            ),
        );
        ResourceRetryAdmissionReport::denied(
            DeniedResourceRetry::new(request_id, class, retry_decision_digest, None, None, None),
            performance,
        )
    }

    fn record_retry_denial(
        &mut self,
        class: ResourceRetryDenialClass,
        telemetry: &mut ResourceTelemetry,
    ) {
        telemetry.resource_retry_denial_count += 1;
        match class {
            ResourceRetryDenialClass::UnknownOrStaleRequest
            | ResourceRetryDenialClass::MissingRetryBackoffWake => {
                telemetry.resource_stale_retry_denial_count += 1
            }
            ResourceRetryDenialClass::NonRetryableRequest => {
                telemetry.resource_non_retryable_denial_count += 1
            }
            ResourceRetryDenialClass::RetryPolicyDisabled => {
                telemetry.resource_retry_policy_disabled_denial_count += 1
            }
            ResourceRetryDenialClass::RetryAttemptLimitReached => {
                telemetry.resource_retry_attempt_limit_denial_count += 1
            }
            ResourceRetryDenialClass::RetryBudgetExhausted => {
                telemetry.resource_retry_budget_exhaustion_denial_count += 1
            }
            ResourceRetryDenialClass::RetryTimeoutWindowExhausted => {
                telemetry.resource_retry_timeout_window_exhaustion_denial_count += 1
            }
            ResourceRetryDenialClass::RetryAlreadyScheduled => {
                telemetry.resource_retry_already_scheduled_denial_count += 1
            }
            ResourceRetryDenialClass::WakeMismatch => {
                telemetry.resource_retry_wake_mismatch_denial_count += 1
            }
            ResourceRetryDenialClass::SupersededByNewerRequest => {
                telemetry.resource_retry_superseded_denial_count += 1
            }
        }
    }

    fn deny_revalidation(
        &mut self,
        intent: ResourceRevalidationIntent,
        class: ResourceRevalidationDenialClass,
        telemetry: &mut ResourceTelemetry,
    ) -> ResourceRevalidationReport {
        telemetry.resource_revalidation_denial_count += 1;
        match class {
            ResourceRevalidationDenialClass::UndeclaredResourceNode => {
                telemetry.resource_undeclared_owner_denial_count += 1
            }
            ResourceRevalidationDenialClass::ActiveRequestRequiresExpectedHandle => {
                telemetry.resource_revalidation_active_requires_expected_denial_count += 1
            }
            ResourceRevalidationDenialClass::ExpectedActiveRequestMismatch => {
                telemetry.resource_revalidation_expected_mismatch_denial_count += 1
            }
            ResourceRevalidationDenialClass::ForcedRevalidationPolicyDisabled => {
                telemetry.resource_forced_revalidation_policy_denial_count += 1
            }
            ResourceRevalidationDenialClass::ActiveHandleProofMismatch => {
                telemetry.resource_revalidation_active_handle_proof_mismatch_denial_count += 1
            }
            ResourceRevalidationDenialClass::DependencyChangeRevalidationPolicyDisabled => {
                telemetry.resource_revalidation_dependency_change_policy_denial_count += 1
            }
            ResourceRevalidationDenialClass::DependencyChangeProofMismatch => {
                telemetry.resource_revalidation_dependency_change_proof_mismatch_denial_count += 1
            }
            ResourceRevalidationDenialClass::ObserverDemandRevalidationPolicyDisabled => {
                telemetry.resource_revalidation_observer_demand_policy_denial_count += 1
            }
            ResourceRevalidationDenialClass::ObserverDemandProofMismatch => {
                telemetry.resource_revalidation_observer_demand_proof_mismatch_denial_count += 1
            }
            ResourceRevalidationDenialClass::TerminalStateRevalidationPolicyDisabled => {
                telemetry.resource_revalidation_terminal_state_policy_denial_count += 1
            }
            ResourceRevalidationDenialClass::TerminalStateProofMismatch => {
                telemetry.resource_revalidation_terminal_state_proof_mismatch_denial_count += 1
            }
            ResourceRevalidationDenialClass::FulfilledLifecycleRevalidationPolicyDisabled => {
                telemetry.resource_revalidation_fulfilled_lifecycle_policy_denial_count += 1
            }
            ResourceRevalidationDenialClass::FulfilledLifecycleProofMismatch => {
                telemetry.resource_revalidation_fulfilled_lifecycle_proof_mismatch_denial_count += 1
            }
            ResourceRevalidationDenialClass::StaleAfterRevalidationPolicyDisabled => {
                telemetry.resource_revalidation_stale_after_policy_denial_count += 1
            }
            ResourceRevalidationDenialClass::StaleAfterWakeMismatch => {
                telemetry.resource_revalidation_stale_after_wake_mismatch_denial_count += 1
            }
            ResourceRevalidationDenialClass::StaleAfterRequiresFulfilledLifecycle => {
                telemetry.resource_revalidation_stale_after_fulfilled_only_denial_count += 1
            }
        }
        let performance = Self::record_boundary_performance(
            telemetry,
            ResourceBoundaryPerformanceEnvelope::revalidation_admission(0, 1, 0, 0),
        );
        ResourceRevalidationReport::denied(
            DeniedResourceRevalidation::new(
                intent.node(),
                intent
                    .expected_active()
                    .map(ResourceRequestHandle::request_id),
                class,
            ),
            performance,
        )
    }

    fn admit_validated_completion(
        &mut self,
        validated: ValidatedCompletionEnvelope,
        in_flight: InFlightResourceRequest,
        telemetry: &mut ResourceTelemetry,
        count_scalar_boundary: bool,
    ) -> ResourceCompletionAdmissionReport {
        let lifecycle_ordinal = self.issue_lifecycle_ordinal();
        let completion_ordinal = self.issue_completion_ordinal();
        let transition = ResourceLifecycleTransition::new(
            in_flight.node(),
            ResourceLifecycleClass::Pending,
            ResourceLifecycleClass::Fulfilled,
            ResourceLifecycleTransitionKind::CompletionAdmitted,
            lifecycle_ordinal,
            ResourceOutputContinuity::OutputReplaced,
        );
        let admitted = AdmittedResourceCompletion::new(
            validated.handle(),
            validated.attempt(),
            in_flight.node(),
            in_flight.descriptor_id(),
            completion_ordinal,
            validated.payload_byte_len(),
            transition,
        );

        if count_scalar_boundary {
            telemetry.resource_completion_admission_count += 1;
        }
        let performance = ResourceBoundaryPerformanceEnvelope::completion_admission(1, 0, 1)
            .with_density_strategy(ResourceDensityStrategy::scalar_completion());
        let performance = if count_scalar_boundary {
            Self::record_boundary_performance(telemetry, performance)
        } else {
            performance
        };

        ResourceCompletionAdmissionReport::admitted(admitted, performance)
    }

    fn deny_completion(
        &mut self,
        raw: &RawCompletionEnvelope,
        class: CompletionDenialClass,
        node: Option<ResourceNodeId>,
        telemetry: &mut ResourceTelemetry,
        count_scalar_boundary: bool,
    ) -> ResourceCompletionAdmissionReport {
        let denial_id = self.issue_denial_id();
        let denied = DeniedResourceCompletion::new(denial_id, class, node, raw);
        self.denied_completions.insert(denial_id, denied);
        if let Some(node) = node {
            self.latest_denied_completion_by_node.insert(node, denied);
        }

        telemetry.resource_completion_denial_count += 1;
        match class {
            CompletionDenialClass::Stale => telemetry.resource_stale_completion_denial_count += 1,
            CompletionDenialClass::Superseded => {
                telemetry.resource_superseded_completion_denial_count += 1
            }
            CompletionDenialClass::Malformed => {
                telemetry.resource_malformed_completion_denial_count += 1
            }
            CompletionDenialClass::Partial => {
                telemetry.resource_partial_completion_denial_count += 1
            }
            CompletionDenialClass::Contradictory => {
                telemetry.resource_contradictory_completion_denial_count += 1
            }
            CompletionDenialClass::Duplicate => {
                telemetry.resource_duplicate_completion_denial_count += 1
            }
            CompletionDenialClass::UnknownRequest => {
                telemetry.resource_unknown_request_completion_denial_count += 1
            }
            CompletionDenialClass::RetainedHistoryUnavailable => {
                telemetry.resource_retained_history_unavailable_completion_denial_count += 1
            }
            CompletionDenialClass::Cancelled => {
                telemetry.resource_cancelled_completion_denial_count += 1
            }
            CompletionDenialClass::Rejected => {
                telemetry.resource_rejected_completion_denial_count += 1
            }
            CompletionDenialClass::TimedOut => {
                telemetry.resource_timed_out_completion_denial_count += 1
            }
            CompletionDenialClass::Retired | CompletionDenialClass::Impossible => {}
        }
        let performance = ResourceBoundaryPerformanceEnvelope::completion_admission(0, 1, 0)
            .with_density_strategy(ResourceDensityStrategy::scalar_completion());
        let performance = if count_scalar_boundary {
            Self::record_boundary_performance(telemetry, performance)
        } else {
            performance
        };

        ResourceCompletionAdmissionReport::denied(denied, performance)
    }

    fn retained_completion_denial_class(
        &self,
        raw: &RawCompletionEnvelope,
        retained: InFlightResourceRequest,
    ) -> CompletionDenialClass {
        let handle = retained.handle();
        if handle.request_id() != raw.request_id()
            || handle.generation() != raw.generation()
            || handle.branch_epoch() != raw.branch_epoch()
            || retained.attempt() != raw.attempt()
        {
            return CompletionDenialClass::Stale;
        }

        match retained.status() {
            ResourceInFlightStatus::Fulfilled => CompletionDenialClass::Retired,
            ResourceInFlightStatus::Rejected => CompletionDenialClass::Rejected,
            ResourceInFlightStatus::Superseded => CompletionDenialClass::Superseded,
            ResourceInFlightStatus::Cancelled => CompletionDenialClass::Cancelled,
            ResourceInFlightStatus::TimedOut => CompletionDenialClass::TimedOut,
            ResourceInFlightStatus::Active => CompletionDenialClass::Impossible,
        }
    }

    fn pruned_completion_denial_class(
        raw: &RawCompletionEnvelope,
        pruned: ResourceRetainedHistoryAvailability,
    ) -> CompletionDenialClass {
        let handle = pruned.handle();
        if handle.request_id() != raw.request_id()
            || handle.generation() != raw.generation()
            || handle.branch_epoch() != raw.branch_epoch()
            || pruned.attempt() != raw.attempt()
        {
            CompletionDenialClass::Stale
        } else if pruned.class()
            == ResourceRetainedHistoryAvailabilityClass::PrunedByRetainedHistoryLimit
        {
            CompletionDenialClass::RetainedHistoryUnavailable
        } else {
            Self::completion_denial_class_for_lifecycle(pruned.lifecycle())
        }
    }

    fn completion_denial_class_for_lifecycle(
        lifecycle: ResourceLifecycleClass,
    ) -> CompletionDenialClass {
        match lifecycle {
            ResourceLifecycleClass::Fulfilled => CompletionDenialClass::Retired,
            ResourceLifecycleClass::Rejected => CompletionDenialClass::Rejected,
            ResourceLifecycleClass::Cancelled => CompletionDenialClass::Cancelled,
            ResourceLifecycleClass::TimedOut => CompletionDenialClass::TimedOut,
            ResourceLifecycleClass::Superseded => CompletionDenialClass::Superseded,
            ResourceLifecycleClass::RetainedHistoryUnavailable
            | ResourceLifecycleClass::Unrequested
            | ResourceLifecycleClass::Pending
            | ResourceLifecycleClass::Stale
            | ResourceLifecycleClass::Disposed => CompletionDenialClass::RetainedHistoryUnavailable,
        }
    }

    fn supersede_active_request_for_node(
        &mut self,
        node: ResourceNodeId,
        replacing: ResourceRequestHandle,
        replacing_descriptor_id: ResourceDescriptorId,
        telemetry: &mut ResourceTelemetry,
    ) -> Option<ResourceSupersessionRecord> {
        let request_id = self.active_request_by_node.get(&node).copied()?;
        let (supersession_digest, permits_overlap_admission, requests_old_host_work_cancel) = {
            let plan = self
                .descriptors
                .get(&replacing_descriptor_id)?
                .supersession_decision_plan();
            (
                plan.decision_digest().clone(),
                plan.permits_overlapping_generation_admission(),
                plan.requests_old_host_work_advisory_cancel(),
            )
        };
        let ordinal = self.issue_lifecycle_ordinal();
        let supersession_ordinal = self.issue_supersession_ordinal();
        let (output_continuity, _) = self.classify_terminal_output_continuity_for_node(
            node,
            replacing_descriptor_id,
            ResourceTerminalVisibilityCause::Supersession,
            telemetry,
        );
        let in_flight = self.in_flight_by_request.get_mut(&request_id)?;
        let previous = in_flight.handle();
        in_flight.supersede(ordinal, replacing);
        self.mark_terminal_in_flight(request_id);
        telemetry.resource_supersession_policy_decision_count += 1;
        telemetry.resource_superseded_in_flight_count += 1;
        telemetry.resource_supersession_record_count += 1;
        telemetry.resource_supersession_lineage_width =
            telemetry.resource_supersession_lineage_width.max(2);
        let overlap_admission = if permits_overlap_admission {
            telemetry.resource_overlapping_generation_admission_count += 1;
            if requests_old_host_work_cancel {
                telemetry.resource_old_host_work_advisory_cancelled_count += 1;
            } else {
                telemetry.resource_old_host_work_retained_count += 1;
            }
            Some(ResourceOverlappingGenerationAdmission::new(
                previous,
                replacing,
                supersession_digest.clone(),
                requests_old_host_work_cancel.then(|| {
                    ResourceOldHostWorkCancellationAdvisory::requested(supersession_digest.clone())
                }),
            ))
        } else {
            None
        };
        Some(ResourceSupersessionRecord::new(
            supersession_ordinal,
            previous,
            replacing,
            supersession_digest,
            overlap_admission,
            ResourceLifecycleTransition::new(
                node,
                ResourceLifecycleClass::Pending,
                ResourceLifecycleClass::Superseded,
                ResourceLifecycleTransitionKind::RequestSuperseded,
                ordinal,
                output_continuity,
            ),
        ))
    }

    fn try_coalesce_equivalent_request_intent(
        &mut self,
        node: ResourceNodeId,
        descriptor_id: ResourceDescriptorId,
        request_intent_digest: &crate::data::resource::ResourceRequestIntentDigest,
        branch_id: SignalBranchId,
        generation_started_tick: crate::data::temporal::ClockTick,
        telemetry: &mut ResourceTelemetry,
    ) -> Option<ResourceRequestAdmissionReport> {
        let active_request_id = self.active_request_by_node.get(&node).copied()?;
        let active_in_flight = self.in_flight_by_request.get(&active_request_id)?.clone();
        if active_in_flight.status() != ResourceInFlightStatus::Active
            || active_in_flight.lifecycle() != ResourceLifecycleClass::Pending
        {
            return None;
        }
        let supersession_plan = self
            .descriptors
            .get(&descriptor_id)?
            .supersession_decision_plan()
            .clone();
        if !supersession_plan.permits_intent_equivalence_coalescing()
            || active_in_flight.request_intent_digest() != request_intent_digest
        {
            return None;
        }

        telemetry.resource_supersession_policy_decision_count += 1;
        telemetry.resource_intent_equivalence_coalescing_count += 1;

        let request_id = self.issue_request_id();
        let generation = self.issue_generation();
        let attempt = ResourceAttemptId::ZERO;
        let branch_epoch = ResourceBranchEpoch::new(branch_id, self.restore_epoch);
        let coalesced_request =
            AdmittedResourceRequest::new(request_id, generation, branch_epoch, attempt);
        let lifecycle_ordinal = self.issue_lifecycle_ordinal();
        let supersession_ordinal = self.issue_supersession_ordinal();
        let (output_continuity, terminal_visibility_classified) = self
            .classify_terminal_output_continuity_for_node(
                node,
                descriptor_id,
                ResourceTerminalVisibilityCause::Supersession,
                telemetry,
            );
        let coalesced_transition = ResourceLifecycleTransition::new(
            node,
            ResourceLifecycleClass::Pending,
            ResourceLifecycleClass::Superseded,
            ResourceLifecycleTransitionKind::RequestSuperseded,
            lifecycle_ordinal,
            output_continuity,
        );
        let mut coalesced_in_flight = InFlightResourceRequest::new(
            coalesced_request.handle(),
            node,
            descriptor_id,
            generation,
            attempt,
            request_intent_digest.clone(),
            generation_started_tick,
            lifecycle_ordinal,
            active_in_flight.timeout_duration(),
            active_in_flight.timeout_due_tick(),
            active_in_flight.timeout_outcome_class(),
            active_in_flight.timeout_deadline_authority(),
            active_in_flight.timeout_decision_digest().clone(),
        );
        coalesced_in_flight.supersede(lifecycle_ordinal, active_in_flight.handle());
        self.in_flight_by_request
            .insert(request_id, coalesced_in_flight);
        self.mark_terminal_in_flight(request_id);

        telemetry.resource_request_admission_count += 1;
        telemetry.resource_in_flight_request_count = self.in_flight_by_request.len() as u64;
        telemetry.resource_in_flight_frontier_width = telemetry
            .resource_in_flight_frontier_width
            .max(self.active_request_by_node.len() as u64);
        let density_strategy =
            ResourceDensityStrategy::request_pressure(self.in_flight_by_request.len() as u32);
        let performance = Self::record_boundary_performance(
            telemetry,
            ResourceBoundaryPerformanceEnvelope::request_admission(1, 0, 1)
                .with_density_strategy(density_strategy)
                .with_output_continuity_classification_width(u32::from(
                    terminal_visibility_classified,
                )),
        );
        let lifecycle = self
            .lifecycle_by_node
            .get(&node)
            .copied()
            .unwrap_or_else(|| {
                ResourceLifecycleSummary::new(
                    node,
                    ResourceLifecycleClass::Pending,
                    ResourceOutputContinuity::NoPriorOutput,
                    active_in_flight.lifecycle_ordinal(),
                )
            });
        let admitted_request = AdmittedResourceRequest::new(
            active_in_flight.handle().request_id(),
            active_in_flight.generation(),
            active_in_flight.handle().branch_epoch(),
            active_in_flight.attempt(),
        );

        Some(ResourceRequestAdmissionReport::new(
            admitted_request,
            lifecycle,
            coalesced_transition,
            None,
            Some(ResourceIntentEquivalenceCoalescing::new(
                supersession_ordinal,
                active_in_flight.handle(),
                coalesced_request,
                request_intent_digest.clone(),
                supersession_plan.decision_digest().clone(),
                coalesced_transition,
            )),
            performance,
        ))
    }

    fn issue_descriptor_id(&mut self) -> ResourceDescriptorId {
        let id = self.next_descriptor_id;
        self.next_descriptor_id = id.next();
        id
    }

    fn issue_request_id(&mut self) -> ResourceRequestId {
        let id = self.next_request_id;
        self.next_request_id = ResourceRequestId::new(id.get().saturating_add(1));
        id
    }

    fn issue_generation(&mut self) -> ResourceGeneration {
        self.next_generation =
            ResourceGeneration::new(self.next_generation.get().saturating_add(1));
        self.next_generation
    }

    fn issue_lifecycle_ordinal(&mut self) -> ResourceLifecycleOrdinal {
        self.next_lifecycle_ordinal =
            ResourceLifecycleOrdinal::new(self.next_lifecycle_ordinal.get().saturating_add(1));
        self.next_lifecycle_ordinal
    }

    fn issue_denial_id(&mut self) -> AsyncDenialId {
        let id = self.next_denial_id;
        self.next_denial_id = AsyncDenialId::new(id.get().saturating_add(1));
        id
    }

    fn issue_completion_ordinal(&mut self) -> ResourceCompletionOrdinal {
        self.next_completion_ordinal =
            ResourceCompletionOrdinal::new(self.next_completion_ordinal.get().saturating_add(1));
        self.next_completion_ordinal
    }

    fn issue_cancellation_ordinal(&mut self) -> ResourceCancellationOrdinal {
        self.next_cancellation_ordinal = ResourceCancellationOrdinal::new(
            self.next_cancellation_ordinal.get().saturating_add(1),
        );
        self.next_cancellation_ordinal
    }

    fn issue_timeout_ordinal(&mut self) -> ResourceTimeoutOrdinal {
        self.next_timeout_ordinal =
            ResourceTimeoutOrdinal::new(self.next_timeout_ordinal.get().saturating_add(1));
        self.next_timeout_ordinal
    }

    fn issue_rejection_ordinal(&mut self) -> ResourceRejectionOrdinal {
        self.next_rejection_ordinal =
            ResourceRejectionOrdinal::new(self.next_rejection_ordinal.get().saturating_add(1));
        self.next_rejection_ordinal
    }

    fn issue_supersession_ordinal(&mut self) -> ResourceSupersessionOrdinal {
        self.next_supersession_ordinal = ResourceSupersessionOrdinal::new(
            self.next_supersession_ordinal.get().saturating_add(1),
        );
        self.next_supersession_ordinal
    }

    fn issue_retry_ordinal(&mut self) -> ResourceRetryOrdinal {
        self.next_retry_ordinal =
            ResourceRetryOrdinal::new(self.next_retry_ordinal.get().saturating_add(1));
        self.next_retry_ordinal
    }
}

fn resource_policy_resolution_signal_error(
    err: ResourcePolicyResolutionError,
) -> crate::data::error::SignalError {
    match err {
        ResourcePolicyResolutionError::UnknownPolicy { kind, name } => {
            crate::data::error::SignalError::invalid_input(format!(
                "unknown resource policy '{}' for {:?}",
                name.as_str(),
                kind
            ))
        }
        ResourcePolicyResolutionError::MissingDescriptor { kind, name } => {
            crate::data::error::SignalError::invalid_input(format!(
                "missing resource policy descriptor '{}' for {:?}",
                name.as_str(),
                kind
            ))
        }
        ResourcePolicyResolutionError::RegistryDigestDrift { expected, actual } => {
            crate::data::error::SignalError::invalid_input(format!(
                "resource policy registry digest drift during freeze: expected '{}', got '{}'",
                expected.as_str(),
                actual.as_str()
            ))
        }
        ResourcePolicyResolutionError::IncompatibleDescriptor {
            kind,
            name,
            version,
            compatibility_posture,
        } => crate::data::error::SignalError::invalid_input(format!(
            "incompatible resource policy descriptor '{}' for {:?} at version {}.{} with posture {:?}",
            name.as_str(),
            kind,
            version.major(),
            version.minor(),
            compatibility_posture
        )),
        ResourcePolicyResolutionError::MalformedDescriptor { kind, name, reason } => {
            crate::data::error::SignalError::invalid_input(format!(
                "malformed resource policy descriptor '{}' for {:?}: {}",
                name.as_str(),
                kind,
                reason
            ))
        }
        ResourcePolicyResolutionError::UnsupportedExecutablePolicy { kind, name, reason } => {
            crate::data::error::SignalError::invalid_input(format!(
                "resource policy descriptor '{}' for {:?} is not executable in the first ship runtime: {}",
                name.as_str(),
                kind,
                reason
            ))
        }
    }
}
