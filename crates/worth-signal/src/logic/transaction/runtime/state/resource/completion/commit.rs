use super::super::ResourceRuntimeState;
use crate::data::resource::*;
use crate::data::telemetry::ResourceTelemetry;

impl ResourceRuntimeState {
    pub fn commit_staged_resource_completion(
        &mut self,
        staged: StagedResourceCompletionEffect,
        mut telemetry: Option<&mut ResourceTelemetry>,
    ) -> Result<ResourceCompletionCommitReport, crate::data::error::SignalError> {
        if let Some(telemetry) = telemetry.as_deref_mut() {
            telemetry.resource_hot_in_flight_lookup_count += 1;
        }
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

        if let Some(telemetry) = telemetry.as_deref_mut() {
            telemetry.resource_completion_commit_count += 1;
            telemetry.resource_output_continuity_decision_count += 1;
            telemetry.resource_in_flight_request_count = self.in_flight_by_request.len() as u64;
        }
        let envelope = ResourceBoundaryPerformanceEnvelope::completion_commit(1)
            .with_output_continuity_classification_width(1);
        let performance = telemetry
            .as_deref_mut()
            .map(|telemetry| Self::record_boundary_performance(telemetry, envelope))
            .unwrap_or(envelope);

        Ok(ResourceCompletionCommitReport::new(
            committed,
            lifecycle,
            transition,
            performance,
        ))
    }
}
