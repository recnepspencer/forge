use std::collections::BTreeMap;

use serde::Serialize;

use crate::data::resource::{
    AdmittedResourceCompletion, AdmittedResourceRequest, AdmittedResourceRetry,
    AdmittedResourceRevalidation, AsyncDenialId, CancelledResourceRequest,
    CommittedResourceCompletionArtifact, CompletionDenialClass, DeniedResourceCancellation,
    DeniedResourceCompletion, DeniedResourceRetry, DeniedResourceRevalidation,
    DeniedResourceTimeout, FrozenResourcePolicyRegistry, InFlightResourceRequest,
    LoweredResourceDescriptor, RawCompletionEnvelope, ResourceAttemptId,
    ResourceBoundaryPerformanceEnvelope, ResourceBranchEpoch, ResourceBranchRestoreReport,
    ResourceCancellationDenialClass, ResourceCancellationOrdinal, ResourceCancellationReason,
    ResourceCancellationReport, ResourceCompletionAdmissionReport,
    ResourceCompletionBatchAdmissionReport, ResourceCompletionCommitReport,
    ResourceCompletionDenialStagingReport, ResourceCompletionOrdinal,
    ResourceCompletionRollbackReport, ResourceCompletionStagingReport, ResourceDeclarationReport,
    ResourceDescriptorId, ResourceDescriptorVersion, ResourceGeneration, ResourceInFlightStatus,
    ResourceLifecycleClass, ResourceLifecycleOrdinal, ResourceLifecycleSummary,
    ResourceLifecycleTransition, ResourceLifecycleTransitionKind, ResourceNodeDeclaration,
    ResourceNodeId, ResourceOutputContinuity, ResourcePolicyResolutionError,
    ResourceReplayReconstructionReport, ResourceRequestAdmissionReport, ResourceRequestHandle,
    ResourceRequestId, ResourceRequestIntent, ResourceResolvedPolicyBundle,
    ResourceRetryAdmissionReport, ResourceRetryDenialClass, ResourceRetryOrdinal,
    ResourceRetryPolicyDeclaration, ResourceRetryReason, ResourceRetryScheduleReport,
    ResourceRevalidationDenialClass, ResourceRevalidationIntent, ResourceRevalidationReport,
    ResourceRuntimeSummary, ResourceRuntimeSummaryReadReport, ResourceSupersessionOrdinal,
    ResourceSupersessionRecord, ResourceTimeoutDenialClass, ResourceTimeoutOrdinal,
    ResourceTimeoutReport, RolledBackResourceCompletionArtifact, ScheduledResourceRetry,
    StagedDeniedResourceCompletionEffect, StagedResourceCompletionEffect, TimedOutResourceRequest,
    ValidatedCompletionEnvelope,
};
use crate::data::telemetry::ResourceTelemetry;
use crate::data::temporal::{ReadyTemporalWake, TemporalWakeId};
use crate::state::SignalBranchId;

use super::merge::canonical_digest;

const RESOURCE_REPLAY_RECONSTRUCTION_SCHEMA_VERSION: &str =
    "forge.resource.replay-reconstruction.v1";

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
    next_supersession_ordinal: ResourceSupersessionOrdinal,
    next_retry_ordinal: ResourceRetryOrdinal,
    restore_epoch: u64,
    policy_registry: FrozenResourcePolicyRegistry,
    descriptors: BTreeMap<ResourceDescriptorId, LoweredResourceDescriptor>,
    descriptors_by_node: BTreeMap<ResourceNodeId, ResourceDescriptorId>,
    lifecycle_by_node: BTreeMap<ResourceNodeId, ResourceLifecycleSummary>,
    in_flight_by_request: BTreeMap<ResourceRequestId, InFlightResourceRequest>,
    active_request_by_node: BTreeMap<ResourceNodeId, ResourceRequestId>,
    pending_retry_by_request: BTreeMap<ResourceRequestId, ScheduledResourceRetry>,
    pending_retry_by_wake: BTreeMap<TemporalWakeId, ResourceRequestId>,
    denied_completions: BTreeMap<AsyncDenialId, DeniedResourceCompletion>,
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
            next_supersession_ordinal: ResourceSupersessionOrdinal::ZERO,
            next_retry_ordinal: ResourceRetryOrdinal::ZERO,
            restore_epoch: 0,
            policy_registry: FrozenResourcePolicyRegistry::built_in(),
            descriptors: BTreeMap::new(),
            descriptors_by_node: BTreeMap::new(),
            lifecycle_by_node: BTreeMap::new(),
            in_flight_by_request: BTreeMap::new(),
            active_request_by_node: BTreeMap::new(),
            pending_retry_by_request: BTreeMap::new(),
            pending_retry_by_wake: BTreeMap::new(),
            denied_completions: BTreeMap::new(),
            latest_branch_restore_report: None,
        }
    }
}

#[derive(Debug, Serialize)]
struct ResourceReplayLifecycleDigestBasis<'a> {
    schema_version: &'static str,
    lifecycle_summaries: &'a [ResourceLifecycleSummary],
}

#[derive(Debug, Serialize)]
struct ResourceReplayDescriptorDigestBasis<'a> {
    schema_version: &'static str,
    descriptors: &'a [LoweredResourceDescriptor],
}

#[derive(Debug, Serialize)]
struct ResourceReplayDenialDigestBasis<'a> {
    schema_version: &'static str,
    denied_completions: &'a [DeniedResourceCompletion],
}

#[derive(Debug, Serialize)]
struct ResourceReplayInFlightDigestBasis<'a> {
    schema_version: &'static str,
    in_flight_requests: &'a [InFlightResourceRequest],
}

#[derive(Debug, Serialize)]
struct ResourceReplayDigestBasis<'a> {
    schema_version: &'static str,
    descriptor_digest: &'a str,
    lifecycle_digest: &'a str,
    denied_completion_digest: &'a str,
    in_flight_digest: &'a str,
    retained_history_unavailable_count: u32,
}

impl ResourceRuntimeState {
    pub fn summary(&self) -> ResourceRuntimeSummary {
        ResourceRuntimeSummary::new(
            self.descriptors.len(),
            self.descriptors_by_node.len(),
            self.in_flight_by_request.len(),
            self.active_request_by_node.len(),
            self.denied_completions.len(),
            self.next_descriptor_id,
        )
    }

    pub fn summary_read_report(
        &self,
        telemetry: &mut ResourceTelemetry,
    ) -> ResourceRuntimeSummaryReadReport {
        telemetry.resource_retained_summary_read_count += 1;
        telemetry.resource_boundary_performance_envelope_count += 1;
        ResourceRuntimeSummaryReadReport::new(
            self.summary(),
            ResourceBoundaryPerformanceEnvelope::summary_read(),
        )
    }

    pub fn descriptor_for_node(&self, node: ResourceNodeId) -> Option<&LoweredResourceDescriptor> {
        self.descriptors_by_node
            .get(&node)
            .and_then(|descriptor_id| self.descriptors.get(descriptor_id))
    }

    pub fn latest_branch_restore_report(&self) -> Option<ResourceBranchRestoreReport> {
        self.latest_branch_restore_report
    }

    pub fn replay_reconstruction_width(&self) -> u32 {
        let width = self
            .descriptors
            .len()
            .saturating_add(self.lifecycle_by_node.len())
            .saturating_add(self.denied_completions.len())
            .saturating_add(self.in_flight_by_request.len());
        width.min(u32::MAX as usize) as u32
    }

    pub fn reconstruct_replay_summary(
        &self,
        telemetry: &mut ResourceTelemetry,
    ) -> ResourceReplayReconstructionReport {
        let descriptors = self.descriptors.values().cloned().collect::<Vec<_>>();
        let lifecycle_summaries = self.lifecycle_by_node.values().copied().collect::<Vec<_>>();
        let denied_completions = self
            .denied_completions
            .values()
            .copied()
            .collect::<Vec<_>>();
        let in_flight_requests = self
            .in_flight_by_request
            .values()
            .copied()
            .collect::<Vec<_>>();
        let retained_history_unavailable_count = lifecycle_summaries
            .iter()
            .filter(|summary| {
                summary.lifecycle() == ResourceLifecycleClass::RetainedHistoryUnavailable
            })
            .count() as u32;
        let lifecycle_summary_width = lifecycle_summaries.len() as u32;
        let descriptor_width = descriptors.len() as u32;
        let denied_completion_width = denied_completions.len() as u32;
        let in_flight_width = in_flight_requests.len() as u32;
        let descriptor_digest = canonical_digest(&ResourceReplayDescriptorDigestBasis {
            schema_version: RESOURCE_REPLAY_RECONSTRUCTION_SCHEMA_VERSION,
            descriptors: &descriptors,
        });
        let lifecycle_digest = canonical_digest(&ResourceReplayLifecycleDigestBasis {
            schema_version: RESOURCE_REPLAY_RECONSTRUCTION_SCHEMA_VERSION,
            lifecycle_summaries: &lifecycle_summaries,
        });
        let denied_completion_digest = canonical_digest(&ResourceReplayDenialDigestBasis {
            schema_version: RESOURCE_REPLAY_RECONSTRUCTION_SCHEMA_VERSION,
            denied_completions: &denied_completions,
        });
        let in_flight_digest = canonical_digest(&ResourceReplayInFlightDigestBasis {
            schema_version: RESOURCE_REPLAY_RECONSTRUCTION_SCHEMA_VERSION,
            in_flight_requests: &in_flight_requests,
        });
        let replay_digest = canonical_digest(&ResourceReplayDigestBasis {
            schema_version: RESOURCE_REPLAY_RECONSTRUCTION_SCHEMA_VERSION,
            descriptor_digest: &descriptor_digest,
            lifecycle_digest: &lifecycle_digest,
            denied_completion_digest: &denied_completion_digest,
            in_flight_digest: &in_flight_digest,
            retained_history_unavailable_count,
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
        telemetry.resource_boundary_performance_envelope_count += 1;

        ResourceReplayReconstructionReport::new(
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
            ResourceBoundaryPerformanceEnvelope::replay_reconstruction(
                descriptor_width,
                lifecycle_summary_width,
                denied_completion_width,
                in_flight_width,
                retained_history_unavailable_count,
            ),
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
        for scheduled in self.pending_retry_by_request.values_mut() {
            *scheduled =
                scheduled.with_previous(scheduled.previous().with_branch_epoch(branch_epoch));
        }
        telemetry.resource_branch_restore_count += 1;
        telemetry.resource_branch_restore_in_flight_width = telemetry
            .resource_branch_restore_in_flight_width
            .max(self.in_flight_by_request.len() as u64);
        let restored_in_flight_width = self.in_flight_by_request.len() as u32;
        let retained_summary_width =
            self.lifecycle_by_node
                .len()
                .saturating_add(self.denied_completions.len()) as u32;
        let broad_rebuild_denial_count = 1;
        telemetry.resource_branch_restore_retained_summary_width = telemetry
            .resource_branch_restore_retained_summary_width
            .max(retained_summary_width as u64);
        telemetry.resource_branch_restore_broad_rebuild_denial_count = telemetry
            .resource_branch_restore_broad_rebuild_denial_count
            .saturating_add(broad_rebuild_denial_count as u64);
        telemetry.resource_boundary_performance_envelope_count += 1;
        let report = ResourceBranchRestoreReport::new(
            restored_in_flight_width,
            retained_summary_width,
            broad_rebuild_denial_count,
            ResourceBoundaryPerformanceEnvelope::branch_restore(
                restored_in_flight_width,
                retained_summary_width,
                broad_rebuild_denial_count,
            ),
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

    pub fn active_timeout_wake_for_node(&self, node: ResourceNodeId) -> Option<TemporalWakeId> {
        let request_id = self.active_request_by_node.get(&node)?;
        self.in_flight_by_request
            .get(request_id)
            .filter(|in_flight| in_flight.status() == ResourceInFlightStatus::Active)
            .filter(|in_flight| in_flight.lifecycle() == ResourceLifecycleClass::Pending)
            .and_then(|in_flight| in_flight.timeout_wake_id())
    }

    pub fn attach_timeout_wake(
        &mut self,
        handle: ResourceRequestHandle,
        wake_id: TemporalWakeId,
        telemetry: &mut ResourceTelemetry,
    ) -> Result<(), crate::data::error::SignalError> {
        telemetry.resource_hot_in_flight_lookup_count += 1;
        let Some(in_flight) = self.in_flight_by_request.get_mut(&handle.request_id()) else {
            return Err(crate::data::error::SignalError::internal(format!(
                "cannot attach timeout wake {} to unknown resource request {}",
                wake_id.get(),
                handle.request_id().get()
            )));
        };
        if in_flight.handle() != handle {
            return Err(crate::data::error::SignalError::internal(format!(
                "cannot attach timeout wake {} to stale resource request {}",
                wake_id.get(),
                handle.request_id().get()
            )));
        }
        in_flight.attach_timeout_wake(wake_id);
        telemetry.resource_timeout_temporal_wake_footprint = telemetry
            .resource_timeout_temporal_wake_footprint
            .saturating_add(1);
        Ok(())
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
        let resolved_policy_bundle = match ResourceResolvedPolicyBundle::from_declaration(
            &declaration,
            &self.policy_registry,
        ) {
            Ok(bundle) => bundle,
            Err(err) => {
                telemetry.resource_policy_resolution_denial_count += 1;
                return Err(resource_policy_resolution_signal_error(err));
            }
        };
        let descriptor_id = self.issue_descriptor_id();
        let descriptor = LoweredResourceDescriptor::from_declaration(
            descriptor_id,
            ResourceDescriptorVersion::INITIAL,
            &declaration,
            resolved_policy_bundle,
        );
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
        telemetry.resource_boundary_performance_envelope_count += 1;

        Ok(ResourceDeclarationReport::new(
            descriptor_id,
            lifecycle,
            transition,
            ResourceBoundaryPerformanceEnvelope::declaration_lowering(1),
        ))
    }

    pub fn admit_resource_request(
        &mut self,
        intent: ResourceRequestIntent,
        branch_id: SignalBranchId,
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
        let supersession = self.supersede_active_request_for_node(node, handle, telemetry);
        let ordinal = self.issue_lifecycle_ordinal();
        let lifecycle = ResourceLifecycleSummary::new(
            node,
            ResourceLifecycleClass::Pending,
            ResourceOutputContinuity::NoPriorOutput,
            ordinal,
        );
        let transition = ResourceLifecycleTransition::new(
            node,
            from,
            ResourceLifecycleClass::Pending,
            ResourceLifecycleTransitionKind::RequestAdmitted,
            ordinal,
            ResourceOutputContinuity::NoPriorOutput,
        );
        let in_flight =
            InFlightResourceRequest::new(handle, node, descriptor_id, generation, attempt, ordinal);
        self.in_flight_by_request.insert(request_id, in_flight);
        self.active_request_by_node.insert(node, request_id);
        self.lifecycle_by_node.insert(node, lifecycle);

        telemetry.resource_request_admission_count += 1;
        telemetry.resource_in_flight_request_count = self.in_flight_by_request.len() as u64;
        telemetry.resource_in_flight_frontier_width = telemetry
            .resource_in_flight_frontier_width
            .max(self.active_request_by_node.len() as u64);
        telemetry.resource_boundary_performance_envelope_count += 1;

        let lifecycle_transition_count = if supersession.is_some() { 2 } else { 1 };

        Ok(ResourceRequestAdmissionReport::new(
            admitted,
            lifecycle,
            transition,
            supersession,
            ResourceBoundaryPerformanceEnvelope::request_admission(
                1,
                0,
                lifecycle_transition_count,
            ),
        ))
    }

    pub fn retry_backoff_delay_for_handle(
        &self,
        handle: ResourceRequestHandle,
    ) -> Result<crate::data::temporal::TemporalDuration, ResourceRetryDenialClass> {
        let in_flight = self
            .in_flight_by_request
            .get(&handle.request_id())
            .copied()
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
        match descriptor.retry_policy() {
            ResourceRetryPolicyDeclaration::Disabled => {
                Err(ResourceRetryDenialClass::RetryPolicyDisabled)
            }
            ResourceRetryPolicyDeclaration::RuntimeBackoff { delay } => Ok(*delay),
            ResourceRetryPolicyDeclaration::Named { .. } => {
                Err(ResourceRetryDenialClass::RetryPolicyDisabled)
            }
        }
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

    pub fn deny_resource_retry_schedule(
        &mut self,
        handle: ResourceRequestHandle,
        class: ResourceRetryDenialClass,
        telemetry: &mut ResourceTelemetry,
    ) -> ResourceRetryScheduleReport {
        self.deny_retry_schedule(handle.request_id(), class, telemetry)
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
                .copied()
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

    pub fn admit_resource_revalidation(
        &mut self,
        intent: ResourceRevalidationIntent,
        branch_id: SignalBranchId,
        telemetry: &mut ResourceTelemetry,
    ) -> ResourceRevalidationReport {
        if let Some(class) = self.validate_resource_revalidation_intent(intent) {
            return self.deny_revalidation(intent, class, telemetry);
        }

        let expected_active = intent.expected_active();
        let request_report = match self.admit_resource_request(
            ResourceRequestIntent::new(intent.node()),
            branch_id,
            telemetry,
        ) {
            Ok(report) => report,
            Err(_) => {
                return self.deny_revalidation(
                    intent,
                    ResourceRevalidationDenialClass::UndeclaredResourceNode,
                    telemetry,
                )
            }
        };
        let admitted_request = request_report.admitted_request();
        let supersession_record = request_report.supersession_record();
        let lifecycle = request_report.lifecycle();
        let transition = request_report.transition();
        let lifecycle_transition_count = request_report.performance().lifecycle_transition_count();

        telemetry.resource_revalidation_admission_count += 1;
        telemetry.resource_boundary_performance_envelope_count += 1;

        ResourceRevalidationReport::admitted(
            AdmittedResourceRevalidation::new(
                admitted_request,
                expected_active,
                supersession_record,
            ),
            lifecycle,
            transition,
            ResourceBoundaryPerformanceEnvelope::revalidation_admission(
                1,
                0,
                lifecycle_transition_count,
                0,
            ),
        )
    }

    pub fn schedule_resource_retry(
        &mut self,
        handle: ResourceRequestHandle,
        reason: ResourceRetryReason,
        backoff_wake_id: TemporalWakeId,
        telemetry: &mut ResourceTelemetry,
    ) -> ResourceRetryScheduleReport {
        telemetry.resource_hot_in_flight_lookup_count += 1;
        let request_id = handle.request_id();
        let Some(in_flight) = self.in_flight_by_request.get(&request_id).copied() else {
            return self.deny_retry_schedule(
                request_id,
                ResourceRetryDenialClass::UnknownOrStaleRequest,
                telemetry,
            );
        };

        if in_flight.handle() != handle {
            return self.deny_retry_schedule(
                request_id,
                ResourceRetryDenialClass::UnknownOrStaleRequest,
                telemetry,
            );
        }
        if in_flight.status() != ResourceInFlightStatus::TimedOut
            || in_flight.lifecycle() != ResourceLifecycleClass::TimedOut
        {
            return self.deny_retry_schedule(
                request_id,
                ResourceRetryDenialClass::NonRetryableRequest,
                telemetry,
            );
        }
        if self.pending_retry_by_request.contains_key(&request_id) {
            return self.deny_retry_schedule(
                request_id,
                ResourceRetryDenialClass::RetryAlreadyScheduled,
                telemetry,
            );
        }
        let Some(descriptor) = self.descriptors.get(&in_flight.descriptor_id()) else {
            return self.deny_retry_schedule(
                request_id,
                ResourceRetryDenialClass::UnknownOrStaleRequest,
                telemetry,
            );
        };
        if matches!(
            descriptor.retry_policy(),
            ResourceRetryPolicyDeclaration::Disabled | ResourceRetryPolicyDeclaration::Named { .. }
        ) {
            return self.deny_retry_schedule(
                request_id,
                ResourceRetryDenialClass::RetryPolicyDisabled,
                telemetry,
            );
        }

        let scheduled = ScheduledResourceRetry::new(
            handle,
            self.issue_retry_ordinal(),
            reason,
            in_flight.attempt().next(),
            backoff_wake_id,
        );
        self.pending_retry_by_request.insert(request_id, scheduled);
        self.pending_retry_by_wake
            .insert(backoff_wake_id, request_id);
        telemetry.resource_retry_schedule_count += 1;
        telemetry.resource_retry_temporal_wake_footprint = telemetry
            .resource_retry_temporal_wake_footprint
            .saturating_add(1);
        telemetry.resource_boundary_performance_envelope_count += 1;

        ResourceRetryScheduleReport::admitted(
            scheduled,
            ResourceBoundaryPerformanceEnvelope::retry_schedule(1, 0),
        )
    }

    pub fn admit_scheduled_resource_retry(
        &mut self,
        handle: ResourceRequestHandle,
        ready_wake: ReadyTemporalWake,
        branch_id: SignalBranchId,
        telemetry: &mut ResourceTelemetry,
    ) -> ResourceRetryAdmissionReport {
        telemetry.resource_hot_in_flight_lookup_count += 1;
        let request_id = handle.request_id();
        let Some(scheduled) = self.pending_retry_by_request.get(&request_id).copied() else {
            return self.deny_retry_admission(
                request_id,
                ResourceRetryDenialClass::MissingRetryBackoffWake,
                telemetry,
            );
        };
        if scheduled.previous() != handle {
            return self.deny_retry_admission(
                request_id,
                ResourceRetryDenialClass::UnknownOrStaleRequest,
                telemetry,
            );
        }
        if scheduled.backoff_wake_id() != ready_wake.id() {
            return self.deny_retry_admission(
                request_id,
                ResourceRetryDenialClass::WakeMismatch,
                telemetry,
            );
        }

        let Some(previous) = self.in_flight_by_request.get(&request_id).copied() else {
            return self.deny_retry_admission(
                request_id,
                ResourceRetryDenialClass::UnknownOrStaleRequest,
                telemetry,
            );
        };
        if previous.handle() != handle {
            return self.deny_retry_admission(
                request_id,
                ResourceRetryDenialClass::UnknownOrStaleRequest,
                telemetry,
            );
        }
        if self
            .active_request_by_node
            .get(&previous.node())
            .is_some_and(|active| *active != request_id)
        {
            return self.deny_retry_admission(
                request_id,
                ResourceRetryDenialClass::SupersededByNewerRequest,
                telemetry,
            );
        }

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
        let lifecycle = ResourceLifecycleSummary::new(
            previous.node(),
            ResourceLifecycleClass::Pending,
            ResourceOutputContinuity::NoPriorOutput,
            ordinal,
        );
        let transition = ResourceLifecycleTransition::new(
            previous.node(),
            ResourceLifecycleClass::TimedOut,
            ResourceLifecycleClass::Pending,
            ResourceLifecycleTransitionKind::RequestAdmitted,
            ordinal,
            ResourceOutputContinuity::NoPriorOutput,
        );
        let in_flight = InFlightResourceRequest::new(
            handle,
            previous.node(),
            previous.descriptor_id(),
            previous.generation(),
            scheduled.next_attempt(),
            ordinal,
        );
        self.pending_retry_by_request.remove(&request_id);
        self.pending_retry_by_wake.remove(&ready_wake.id());
        self.in_flight_by_request
            .insert(retry_request_id, in_flight);
        self.active_request_by_node
            .insert(previous.node(), retry_request_id);
        self.lifecycle_by_node.insert(previous.node(), lifecycle);

        telemetry.resource_retry_admission_count += 1;
        telemetry.resource_boundary_performance_envelope_count += 1;
        telemetry.resource_in_flight_request_count = self.in_flight_by_request.len() as u64;
        telemetry.resource_in_flight_frontier_width = telemetry
            .resource_in_flight_frontier_width
            .max(self.active_request_by_node.len() as u64);
        telemetry.resource_retry_temporal_wake_footprint = telemetry
            .resource_retry_temporal_wake_footprint
            .saturating_add(1);

        ResourceRetryAdmissionReport::admitted(
            AdmittedResourceRetry::new(scheduled, admitted, ready_wake),
            lifecycle,
            transition,
            ResourceBoundaryPerformanceEnvelope::retry_admission(1, 0, 1, 1),
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

        let Some(in_flight) = self.in_flight_by_request.get(&raw.request_id()).copied() else {
            return self.deny_completion(
                &raw,
                CompletionDenialClass::UnknownRequest,
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
                telemetry,
                count_scalar_boundary,
            );
        }

        if in_flight.status() == ResourceInFlightStatus::Superseded {
            return self.deny_completion(
                &raw,
                CompletionDenialClass::Superseded,
                telemetry,
                count_scalar_boundary,
            );
        }

        if in_flight.status() == ResourceInFlightStatus::Cancelled {
            return self.deny_completion(
                &raw,
                CompletionDenialClass::Cancelled,
                telemetry,
                count_scalar_boundary,
            );
        }

        if in_flight.status() == ResourceInFlightStatus::TimedOut {
            return self.deny_completion(
                &raw,
                CompletionDenialClass::TimedOut,
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
                telemetry,
                count_scalar_boundary,
            );
        }

        let Some(descriptor) = self.descriptors.get(&in_flight.descriptor_id()) else {
            return self.deny_completion(
                &raw,
                CompletionDenialClass::Impossible,
                telemetry,
                count_scalar_boundary,
            );
        };

        if descriptor.payload_contract_digest() != raw.payload_contract_digest() {
            return self.deny_completion(
                &raw,
                CompletionDenialClass::Malformed,
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
                telemetry,
                count_scalar_boundary,
            );
        }

        if in_flight.lifecycle() != ResourceLifecycleClass::Pending {
            return self.deny_completion(
                &raw,
                CompletionDenialClass::Impossible,
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
                let denied = self
                    .deny_completion(&raw, class, telemetry, false)
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
        telemetry.resource_boundary_performance_envelope_count += 1;
        let admitted_count = admitted_completions.len() as u32;
        let denied_count = denied_completions.len() as u32;
        ResourceCompletionBatchAdmissionReport::new(
            admitted_completions,
            denied_completions,
            input_width,
            duplicate_width,
            ResourceBoundaryPerformanceEnvelope::completion_batch_admission(
                input_width,
                admitted_count,
                denied_count,
            ),
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
        telemetry.resource_boundary_performance_envelope_count += 1;
        Ok(ResourceCompletionStagingReport::new(
            StagedResourceCompletionEffect::new(admitted),
            ResourceBoundaryPerformanceEnvelope::completion_staging(),
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
        telemetry.resource_boundary_performance_envelope_count += 1;
        Ok(ResourceCompletionDenialStagingReport::new(
            StagedDeniedResourceCompletionEffect::new(denied),
            ResourceBoundaryPerformanceEnvelope::completion_denial_staging(),
        ))
    }

    pub fn rollback_staged_resource_completion(
        &mut self,
        staged: StagedResourceCompletionEffect,
        telemetry: &mut ResourceTelemetry,
    ) -> ResourceCompletionRollbackReport {
        telemetry.resource_completion_rollback_count += 1;
        telemetry.resource_boundary_performance_envelope_count += 1;
        ResourceCompletionRollbackReport::new(
            RolledBackResourceCompletionArtifact::admitted(staged),
            ResourceBoundaryPerformanceEnvelope::completion_rollback(1, 0),
        )
    }

    pub fn rollback_staged_denied_resource_completion(
        &mut self,
        staged: StagedDeniedResourceCompletionEffect,
        telemetry: &mut ResourceTelemetry,
    ) -> ResourceCompletionRollbackReport {
        telemetry.resource_completion_rollback_count += 1;
        telemetry.resource_boundary_performance_envelope_count += 1;
        ResourceCompletionRollbackReport::new(
            RolledBackResourceCompletionArtifact::denied(staged),
            ResourceBoundaryPerformanceEnvelope::completion_rollback(0, 1),
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
        if self
            .active_request_by_node
            .get(&admitted.node())
            .is_some_and(|active| *active == handle.request_id())
        {
            self.active_request_by_node.remove(&admitted.node());
        }
        let lifecycle = ResourceLifecycleSummary::new(
            admitted.node(),
            ResourceLifecycleClass::Fulfilled,
            ResourceOutputContinuity::OutputReplaced,
            transition.ordinal(),
        );
        self.lifecycle_by_node.insert(admitted.node(), lifecycle);
        let committed = CommittedResourceCompletionArtifact::new(staged, transition);

        telemetry.resource_completion_commit_count += 1;
        telemetry.resource_boundary_performance_envelope_count += 1;
        telemetry.resource_in_flight_request_count = self.in_flight_by_request.len() as u64;

        Ok(ResourceCompletionCommitReport::new(
            committed,
            lifecycle,
            transition,
            ResourceBoundaryPerformanceEnvelope::completion_commit(1),
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
        let Some(in_flight) = self.in_flight_by_request.get(&request_id).copied() else {
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

        let lifecycle_ordinal = self.issue_lifecycle_ordinal();
        let cancellation_ordinal = self.issue_cancellation_ordinal();
        let transition = ResourceLifecycleTransition::new(
            in_flight.node(),
            ResourceLifecycleClass::Pending,
            ResourceLifecycleClass::Cancelled,
            ResourceLifecycleTransitionKind::RequestCancelled,
            lifecycle_ordinal,
            ResourceOutputContinuity::NoPriorOutput,
        );
        let cancelled =
            CancelledResourceRequest::new(handle, cancellation_ordinal, reason, transition);
        let lifecycle = ResourceLifecycleSummary::new(
            in_flight.node(),
            ResourceLifecycleClass::Cancelled,
            ResourceOutputContinuity::NoPriorOutput,
            lifecycle_ordinal,
        );

        let in_flight_mut = self
            .in_flight_by_request
            .get_mut(&request_id)
            .expect("in-flight request was just resolved for cancellation");
        in_flight_mut.cancel(lifecycle_ordinal);
        if self
            .active_request_by_node
            .get(&in_flight.node())
            .is_some_and(|active| *active == request_id)
        {
            self.active_request_by_node.remove(&in_flight.node());
        }
        self.lifecycle_by_node.insert(in_flight.node(), lifecycle);

        telemetry.resource_cancellation_count += 1;
        telemetry.resource_boundary_performance_envelope_count += 1;
        telemetry.resource_in_flight_request_count = self.in_flight_by_request.len() as u64;

        ResourceCancellationReport::admitted(
            cancelled,
            lifecycle,
            transition,
            ResourceBoundaryPerformanceEnvelope::cancellation(1, 0),
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
        let Some(in_flight) = self.in_flight_by_request.get(&request_id).copied() else {
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

        let lifecycle_ordinal = self.issue_lifecycle_ordinal();
        let timeout_ordinal = self.issue_timeout_ordinal();
        let transition = ResourceLifecycleTransition::new(
            in_flight.node(),
            ResourceLifecycleClass::Pending,
            ResourceLifecycleClass::TimedOut,
            ResourceLifecycleTransitionKind::RequestTimedOut,
            lifecycle_ordinal,
            ResourceOutputContinuity::NoPriorOutput,
        );
        let timed_out =
            TimedOutResourceRequest::new(handle, timeout_ordinal, ready_wake, transition);
        let lifecycle = ResourceLifecycleSummary::new(
            in_flight.node(),
            ResourceLifecycleClass::TimedOut,
            ResourceOutputContinuity::NoPriorOutput,
            lifecycle_ordinal,
        );

        let in_flight_mut = self
            .in_flight_by_request
            .get_mut(&request_id)
            .expect("in-flight request was just resolved for timeout");
        in_flight_mut.timeout(lifecycle_ordinal);
        if self
            .active_request_by_node
            .get(&in_flight.node())
            .is_some_and(|active| *active == request_id)
        {
            self.active_request_by_node.remove(&in_flight.node());
        }
        self.lifecycle_by_node.insert(in_flight.node(), lifecycle);

        telemetry.resource_timeout_admission_count += 1;
        telemetry.resource_boundary_performance_envelope_count += 1;
        telemetry.resource_in_flight_request_count = self.in_flight_by_request.len() as u64;
        telemetry.resource_timeout_temporal_wake_footprint = telemetry
            .resource_timeout_temporal_wake_footprint
            .saturating_add(1);

        ResourceTimeoutReport::admitted(
            timed_out,
            lifecycle,
            transition,
            ResourceBoundaryPerformanceEnvelope::timeout_admission(1, 0, 1),
        )
    }

    fn deny_cancellation(
        &mut self,
        request_id: ResourceRequestId,
        class: ResourceCancellationDenialClass,
        telemetry: &mut ResourceTelemetry,
    ) -> ResourceCancellationReport {
        telemetry.resource_cancellation_denial_count += 1;
        telemetry.resource_boundary_performance_envelope_count += 1;
        match class {
            ResourceCancellationDenialClass::UnknownOrStaleRequest => {
                telemetry.resource_stale_cancellation_denial_count += 1
            }
            ResourceCancellationDenialClass::NonActiveRequest => {
                telemetry.resource_non_active_cancellation_denial_count += 1
            }
        }
        ResourceCancellationReport::denied(
            DeniedResourceCancellation::new(request_id, class),
            ResourceBoundaryPerformanceEnvelope::cancellation(0, 1),
        )
    }

    fn deny_timeout(
        &mut self,
        request_id: ResourceRequestId,
        class: ResourceTimeoutDenialClass,
        telemetry: &mut ResourceTelemetry,
    ) -> ResourceTimeoutReport {
        telemetry.resource_timeout_denial_count += 1;
        telemetry.resource_boundary_performance_envelope_count += 1;
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
        ResourceTimeoutReport::denied(
            DeniedResourceTimeout::new(request_id, class),
            ResourceBoundaryPerformanceEnvelope::timeout_admission(
                0,
                1,
                u32::from(matches!(class, ResourceTimeoutDenialClass::WakeMismatch)),
            ),
        )
    }

    fn deny_retry_schedule(
        &mut self,
        request_id: ResourceRequestId,
        class: ResourceRetryDenialClass,
        telemetry: &mut ResourceTelemetry,
    ) -> ResourceRetryScheduleReport {
        self.record_retry_denial(class, telemetry);
        ResourceRetryScheduleReport::denied(
            DeniedResourceRetry::new(request_id, class),
            ResourceBoundaryPerformanceEnvelope::retry_schedule(0, 1),
        )
    }

    fn deny_retry_admission(
        &mut self,
        request_id: ResourceRequestId,
        class: ResourceRetryDenialClass,
        telemetry: &mut ResourceTelemetry,
    ) -> ResourceRetryAdmissionReport {
        self.record_retry_denial(class, telemetry);
        ResourceRetryAdmissionReport::denied(
            DeniedResourceRetry::new(request_id, class),
            ResourceBoundaryPerformanceEnvelope::retry_admission(
                0,
                1,
                0,
                u32::from(matches!(class, ResourceRetryDenialClass::WakeMismatch)),
            ),
        )
    }

    fn record_retry_denial(
        &mut self,
        class: ResourceRetryDenialClass,
        telemetry: &mut ResourceTelemetry,
    ) {
        telemetry.resource_retry_denial_count += 1;
        telemetry.resource_boundary_performance_envelope_count += 1;
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
        telemetry.resource_boundary_performance_envelope_count += 1;
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
        }
        ResourceRevalidationReport::denied(
            DeniedResourceRevalidation::new(
                intent.node(),
                intent
                    .expected_active()
                    .map(ResourceRequestHandle::request_id),
                class,
            ),
            ResourceBoundaryPerformanceEnvelope::revalidation_admission(0, 1, 0, 0),
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
            in_flight.node(),
            in_flight.descriptor_id(),
            completion_ordinal,
            validated.payload_byte_len(),
            transition,
        );

        if count_scalar_boundary {
            telemetry.resource_completion_admission_count += 1;
            telemetry.resource_boundary_performance_envelope_count += 1;
        }

        ResourceCompletionAdmissionReport::admitted(
            admitted,
            ResourceBoundaryPerformanceEnvelope::completion_admission(1, 0, 1),
        )
    }

    fn deny_completion(
        &mut self,
        raw: &RawCompletionEnvelope,
        class: CompletionDenialClass,
        telemetry: &mut ResourceTelemetry,
        count_scalar_boundary: bool,
    ) -> ResourceCompletionAdmissionReport {
        let denial_id = self.issue_denial_id();
        let denied = DeniedResourceCompletion::new(denial_id, class, raw);
        self.denied_completions.insert(denial_id, denied);

        telemetry.resource_completion_denial_count += 1;
        if count_scalar_boundary {
            telemetry.resource_boundary_performance_envelope_count += 1;
        }
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
            CompletionDenialClass::Cancelled => {
                telemetry.resource_cancelled_completion_denial_count += 1
            }
            CompletionDenialClass::TimedOut => {
                telemetry.resource_timed_out_completion_denial_count += 1
            }
            CompletionDenialClass::Retired | CompletionDenialClass::Impossible => {}
        }

        ResourceCompletionAdmissionReport::denied(
            denied,
            ResourceBoundaryPerformanceEnvelope::completion_admission(0, 1, 0),
        )
    }

    fn supersede_active_request_for_node(
        &mut self,
        node: ResourceNodeId,
        replacing: ResourceRequestHandle,
        telemetry: &mut ResourceTelemetry,
    ) -> Option<ResourceSupersessionRecord> {
        let request_id = self.active_request_by_node.get(&node).copied()?;
        let ordinal = self.issue_lifecycle_ordinal();
        let supersession_ordinal = self.issue_supersession_ordinal();
        let in_flight = self.in_flight_by_request.get_mut(&request_id)?;
        let previous = in_flight.handle();
        in_flight.supersede(ordinal, replacing);
        telemetry.resource_superseded_in_flight_count += 1;
        telemetry.resource_supersession_record_count += 1;
        telemetry.resource_supersession_lineage_width =
            telemetry.resource_supersession_lineage_width.max(2);
        Some(ResourceSupersessionRecord::new(
            supersession_ordinal,
            previous,
            replacing,
            ResourceLifecycleTransition::new(
                node,
                ResourceLifecycleClass::Pending,
                ResourceLifecycleClass::Superseded,
                ResourceLifecycleTransitionKind::RequestSuperseded,
                ordinal,
                ResourceOutputContinuity::NoPriorOutput,
            ),
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
    }
}
