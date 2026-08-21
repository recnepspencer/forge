use crate::data::error::SignalError;
use crate::data::handle::NodeId;
use crate::diagnostics::policy::DiagnosticsAvailability;
use crate::facade::{
    SignalObservationCompletion, SignalObservationSession, SignalSnapshotV1, SnapshotRestoreIntent,
};
use crate::logic::explain::NodeExplanation;

impl super::CompiledFinancialLocalityWorld {
    pub(crate) fn capture_runtime_snapshot(&mut self) -> Result<SignalSnapshotV1, SignalError> {
        self.runtime.capture_snapshot()
    }

    pub(crate) fn restore_runtime_snapshot(
        &mut self,
        snapshot: &SignalSnapshotV1,
    ) -> Result<(), SignalError> {
        self.runtime.restore_snapshot(snapshot)
    }

    pub(crate) fn restore_runtime_snapshot_keeping_destination_policy(
        &mut self,
        snapshot: &SignalSnapshotV1,
    ) -> Result<(), SignalError> {
        self.runtime.restore_snapshot_with_intent(
            snapshot,
            SnapshotRestoreIntent::restore_runtime_truth_with_active_policy(),
        )
    }

    pub(crate) fn last_observation_completion(&self) -> Option<SignalObservationCompletion> {
        self.runtime.graph().last_observation_completion()
    }

    pub(crate) fn materialize_explanation(
        &self,
        node: NodeId,
    ) -> Result<(Option<NodeExplanation>, DiagnosticsAvailability), SignalError> {
        self.runtime.graph().materialize_explanation_artifact(node)
    }

    pub(crate) fn begin_runtime_observation(
        &mut self,
    ) -> Result<SignalObservationSession, SignalError> {
        self.runtime
            .begin_observation_session(crate::facade::SignalObservationRequest::operation())
            .map_err(|denial| SignalError::invalid_input(denial.to_string()))
    }

    pub(crate) fn finish_runtime_observation(
        &self,
        session: &SignalObservationSession,
    ) -> Result<crate::data::proof::SignalInvalidationExecutionReceipt, SignalError> {
        self.runtime.finish_observation_session(session)
    }

    pub(crate) fn first_dependent_node(&self) -> Result<NodeId, SignalError> {
        self.locality_definition()
            .outputs()
            .iter()
            .find(|output| !output.subscriptions.is_empty())
            .map(|output| self.handles[&output.id])
            .ok_or_else(|| SignalError::internal("restore world lacks a dependency target"))
    }
}

impl super::super::CompiledFinancialWorld {
    pub(crate) fn locality_capture_runtime_snapshot(
        &mut self,
    ) -> Result<SignalSnapshotV1, SignalError> {
        self.locality_mut().capture_runtime_snapshot()
    }

    pub(crate) fn locality_restore_runtime_snapshot(
        &mut self,
        snapshot: &SignalSnapshotV1,
    ) -> Result<(), SignalError> {
        self.locality_mut().restore_runtime_snapshot(snapshot)
    }

    pub(crate) fn locality_restore_runtime_snapshot_keeping_destination_policy(
        &mut self,
        snapshot: &SignalSnapshotV1,
    ) -> Result<(), SignalError> {
        self.locality_mut()
            .restore_runtime_snapshot_keeping_destination_policy(snapshot)
    }

    pub(crate) fn locality_last_observation_completion(
        &self,
    ) -> Option<SignalObservationCompletion> {
        self.locality().last_observation_completion()
    }

    pub(crate) fn locality_materialize_explanation(
        &self,
        node: NodeId,
    ) -> Result<(Option<NodeExplanation>, DiagnosticsAvailability), SignalError> {
        self.locality().materialize_explanation(node)
    }

    pub(crate) fn locality_committed_value_list(&self) -> Result<Vec<i64>, SignalError> {
        let mut values = self
            .locality()
            .committed_financial_values()?
            .into_iter()
            .collect::<Vec<_>>();
        values.sort_by_key(|(id, _)| id.ordinal());
        Ok(values.into_iter().map(|(_, value)| value).collect())
    }

    pub(crate) fn locality_first_dependent_node(&self) -> Result<NodeId, SignalError> {
        self.locality().first_dependent_node()
    }

    pub(crate) fn locality_begin_runtime_observation(
        &mut self,
    ) -> Result<SignalObservationSession, SignalError> {
        self.locality_mut().begin_runtime_observation()
    }

    pub(crate) fn locality_finish_runtime_observation(
        &self,
        session: &SignalObservationSession,
    ) -> Result<crate::data::proof::SignalInvalidationExecutionReceipt, SignalError> {
        self.locality().finish_runtime_observation(session)
    }
}
