use worth_runtime_bridge::facade::BridgeManagedQueueFailureKind;

use super::interruption_classification::producer_terminal_kind;
use super::managed_graph_execution::{
    WorthQueryManagedGraphExecution, WorthQueryManagedProviderStep,
};
use super::workflow_graph_chunk::{
    WorthQueryPendingWorkflowGraphChunk, WorthQueryPendingWorkflowGraphQueueState,
};
use super::workflow_graph_step_outcome::{
    terminal_outcome, WorthQueryCompletedWorkflowGraphExecution,
    WorthQueryPausedWorkflowGraphExecution, WorthQueryWorkflowGraphStepOutcome,
};
use super::{WorthQueryManagedRunTerminalKind, WorthQueryRunningWorkflowRun};
use crate::domain_computation::provider_session::graph_provider::bounded_step::WorthQueryGraphProviderStepCompletion;
use crate::domain_computation::{WorthQueryGraphProviderStepReport, WorthQueryGraphReadMaterial};

pub struct WorthQueryActiveWorkflowGraphExecution {
    pub(super) running: WorthQueryRunningWorkflowRun,
    pub(super) execution: WorthQueryManagedGraphExecution,
}

impl WorthQueryActiveWorkflowGraphExecution {
    pub(super) fn new(
        running: WorthQueryRunningWorkflowRun,
        execution: WorthQueryManagedGraphExecution,
    ) -> Self {
        Self { running, execution }
    }

    pub fn run_identity(&self) -> &str {
        self.running.identity()
    }

    pub fn logical_run_identity(&self) -> &str {
        self.running.logical_run_identity()
    }

    pub fn resource_attempt_identity(&self) -> &str {
        self.running.resource_attempt.attempt_identity().as_str()
    }

    pub fn provider_session_identity(&self) -> &str {
        self.running.resource_attempt.provider_session().identity()
    }

    pub fn bridge_basis_identity(&self) -> &str {
        self.running.bridge_basis.identity().as_str()
    }

    pub fn bridge_request_identity(&self) -> &str {
        self.running.bridge_basis.request().digest()
    }

    pub fn provider_call_identity(&self) -> &str {
        self.execution.provider_call_identity()
    }

    pub fn retained_capacity_reservation_count(&self) -> usize {
        self.running
            .resource_attempt
            .retained_capacity_reservation_count()
    }

    pub fn artifact_evidence(
        &self,
    ) -> crate::domain_computation::artifact_owner::WorthQueryWorkflowArtifactRegistryEvidence {
        self.running.artifacts.registry().evidence()
    }

    pub fn artifacts(&self) -> super::WorthQueryManagedWorkflowArtifactAuthority<'_> {
        self.running.artifacts()
    }

    pub fn request_cancellation(
        &self,
        reason: worth_runtime_bridge::facade::BridgeManagedExecutionCancellationReason,
    ) -> Result<
        worth_runtime_bridge::facade::BridgeManagedExecutionCancellation,
        worth_runtime_bridge::facade::BridgeManagedExecutionInterruptionFailure,
    > {
        self.running.bridge_basis().request_cancellation(reason)
    }

    pub fn admit_ready_timeout(
        &self,
    ) -> Result<
        worth_runtime_bridge::facade::BridgeManagedExecutionTimeout,
        worth_runtime_bridge::facade::BridgeManagedExecutionInterruptionFailure,
    > {
        self.running.bridge_basis().admit_ready_timeout()
    }

    pub fn reject_execution(
        &self,
        reason: worth_runtime_bridge::facade::BridgeManagedExecutionRejectionReason,
    ) -> Result<
        worth_runtime_bridge::facade::BridgeManagedExecutionRejection,
        worth_runtime_bridge::facade::BridgeManagedExecutionInterruptionFailure,
    > {
        self.running.bridge_basis().reject_execution(reason)
    }

    pub fn advance(mut self) -> WorthQueryWorkflowGraphStepOutcome {
        let before = match self.observe_safe_point() {
            Ok(observation) => observation,
            Err(_) => return self.abandoned_terminal(WorthQueryManagedRunTerminalKind::Failed),
        };
        let admission = self.execution.admit_provider_step(before);
        self.running
            .provider_work_mut()
            .record_provider_step_admission(admission.counters());
        let admitted = match admission {
            super::provider_step_admission::WorthQueryProviderStepAdmissionOutcome::Admitted(
                admitted,
            ) => admitted,
            super::provider_step_admission::WorthQueryProviderStepAdmissionOutcome::Denied(
                denied,
            ) => return self.interrupted_terminal(denied.terminal()),
        };

        self.running
            .provider_work_mut()
            .record_provider_step_attempt();
        match self.execution.advance_provider(admitted) {
            WorthQueryManagedProviderStep::Failed(evidence) => {
                let mut report = evidence.into_report();
                self.record_report(&mut report);
                self.abandoned_terminal(WorthQueryManagedRunTerminalKind::Failed)
            }
            WorthQueryManagedProviderStep::Continue(evidence)
            | WorthQueryManagedProviderStep::Complete(evidence) => {
                self.admit_provider_step(evidence.into_report())
            }
        }
    }

    fn admit_provider_step(
        mut self,
        mut report: WorthQueryGraphProviderStepReport,
    ) -> WorthQueryWorkflowGraphStepOutcome {
        self.record_report(&mut report);
        if let Some(material) = report.take_projection() {
            return self.admit_pending_chunk(report, material);
        }
        match report.completion() {
            WorthQueryGraphProviderStepCompletion::Continue => self.continue_after_safe_point(),
            WorthQueryGraphProviderStepCompletion::Complete => self.finish_completion(&report),
            WorthQueryGraphProviderStepCompletion::Failed => {
                self.abandoned_terminal(WorthQueryManagedRunTerminalKind::Failed)
            }
        }
    }

    fn admit_pending_chunk(
        mut self,
        report: WorthQueryGraphProviderStepReport,
        material: WorthQueryGraphReadMaterial,
    ) -> WorthQueryWorkflowGraphStepOutcome {
        let width = u64::try_from(material.rows().len()).unwrap_or(u64::MAX);
        let retained_bytes = material.owned_allocation_capacity_bytes();
        let mutation = match self.running.bridge_basis_mut().enqueue_managed_queue(width) {
            Ok(mutation) => mutation,
            Err(failure) => {
                if !self.release_pending_chunk(retained_bytes) {
                    return self.abandoned_terminal(WorthQueryManagedRunTerminalKind::Failed);
                }
                return if failure.kind() == BridgeManagedQueueFailureKind::SignalQueueMutationDenied
                {
                    self.interrupted_terminal(WorthQueryManagedRunTerminalKind::Exhausted)
                } else {
                    self.abandoned_terminal(WorthQueryManagedRunTerminalKind::Failed)
                };
            }
        };
        self.running
            .provider_work_mut()
            .record_queue_mutation(mutation.counters());
        self.execution.admit_projection_chunk(&material);
        let queue = WorthQueryPendingWorkflowGraphQueueState::new(
            width,
            mutation.queue_depth(),
            mutation.queue_capacity(),
        );
        WorthQueryWorkflowGraphStepOutcome::ChunkReady(WorthQueryPendingWorkflowGraphChunk::new(
            self,
            report,
            material,
            queue,
            retained_bytes,
        ))
    }

    pub(super) fn finish_completion(
        mut self,
        report: &WorthQueryGraphProviderStepReport,
    ) -> WorthQueryWorkflowGraphStepOutcome {
        let receipt = match self.execution.seal_completion(report) {
            Ok(receipt) => receipt,
            Err(()) => return self.abandoned_terminal(WorthQueryManagedRunTerminalKind::Failed),
        };
        self.running.provider_work_mut().complete_step_call();
        let after = match self.observe_safe_point() {
            Ok(observation) => observation,
            Err(_) => return self.settled_terminal(WorthQueryManagedRunTerminalKind::Failed),
        };
        if let Some(kind) = producer_terminal_kind(&after) {
            return self.settled_terminal(kind);
        }
        let release = self.execution.release_provider_execution();
        self.running
            .provider_work_mut()
            .record_provider_execution_release(&release);
        if release.recovery_required() {
            return terminal_outcome(
                self.running
                    .terminal(WorthQueryManagedRunTerminalKind::Failed),
                WorthQueryManagedRunTerminalKind::Failed,
            );
        }
        WorthQueryWorkflowGraphStepOutcome::Completed(
            WorthQueryCompletedWorkflowGraphExecution::new(self.running, receipt),
        )
    }

    pub(super) fn continue_after_safe_point(mut self) -> WorthQueryWorkflowGraphStepOutcome {
        let after = match self.observe_safe_point() {
            Ok(observation) => observation,
            Err(_) => return self.abandoned_terminal(WorthQueryManagedRunTerminalKind::Failed),
        };
        match producer_terminal_kind(&after) {
            Some(kind) => self.interrupted_terminal(kind),
            None => {
                let safe_point = self.execution.yield_safe_point(after);
                WorthQueryWorkflowGraphStepOutcome::Continue(
                    WorthQueryPausedWorkflowGraphExecution {
                        active: self,
                        safe_point,
                    },
                )
            }
        }
    }

    fn record_report(&mut self, report: &mut WorthQueryGraphProviderStepReport) {
        self.running.provider_work_mut().admit_step(report);
        self.execution.admit_report(report);
    }

    pub(super) fn observe_safe_point(
        &mut self,
    ) -> Result<
        super::WorthQueryManagedSafePointObservation,
        super::WorthQueryManagedSafePointFailure,
    > {
        let observation = self.running.observe_safe_point()?;
        self.running
            .provider_work_mut()
            .record_safe_point(&observation);
        Ok(observation)
    }

    pub(super) fn release_pending_chunk(&mut self, retained_bytes: usize) -> bool {
        self.execution.release_projection_chunk(retained_bytes)
            && self
                .running
                .provider_work_mut()
                .release_retained_bytes(retained_bytes)
    }

    pub(super) fn interrupted_terminal(
        mut self,
        kind: WorthQueryManagedRunTerminalKind,
    ) -> WorthQueryWorkflowGraphStepOutcome {
        self.running.provider_work_mut().interrupt_step_call();
        self.into_terminal_outcome(kind)
    }

    pub(super) fn abandoned_terminal(
        mut self,
        kind: WorthQueryManagedRunTerminalKind,
    ) -> WorthQueryWorkflowGraphStepOutcome {
        self.running.provider_work_mut().abandon();
        self.into_terminal_outcome(kind)
    }

    fn settled_terminal(
        self,
        kind: WorthQueryManagedRunTerminalKind,
    ) -> WorthQueryWorkflowGraphStepOutcome {
        self.into_terminal_outcome(kind)
    }

    fn into_terminal_outcome(
        mut self,
        kind: WorthQueryManagedRunTerminalKind,
    ) -> WorthQueryWorkflowGraphStepOutcome {
        let release = self.execution.release_provider_execution();
        self.running
            .provider_work_mut()
            .record_provider_execution_release(&release);
        terminal_outcome(self.running.terminal(kind), kind)
    }
}
