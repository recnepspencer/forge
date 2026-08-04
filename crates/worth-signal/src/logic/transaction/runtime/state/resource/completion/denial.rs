use super::super::ResourceRuntimeState;
use crate::data::resource::*;
use crate::data::telemetry::ResourceTelemetry;

impl ResourceRuntimeState {
    pub(in crate::logic::transaction::runtime::state::resource) fn deny_completion(
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
    pub(in crate::logic::transaction::runtime::state::resource) fn retained_completion_denial_class(
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
    pub(in crate::logic::transaction::runtime::state::resource) fn pruned_completion_denial_class(
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
    pub(in crate::logic::transaction::runtime::state::resource) fn completion_denial_class_for_lifecycle(
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
}
