use super::interruption_classification::consumer_terminal_kind;
use super::workflow_graph_execution::WorthQueryActiveWorkflowGraphExecution;
use super::workflow_graph_step_outcome::WorthQueryWorkflowGraphStepOutcome;
use super::WorthQueryManagedRunTerminalKind;
use crate::domain_computation::provider_session::graph_provider::bounded_step::WorthQueryGraphProviderStepCompletion;
use crate::domain_computation::{WorthQueryGraphProviderStepReport, WorthQueryGraphReadMaterial};

pub(super) struct WorthQueryPendingWorkflowGraphQueueState {
    width: u64,
    depth: u64,
    capacity: u64,
}

impl WorthQueryPendingWorkflowGraphQueueState {
    pub(super) const fn new(width: u64, depth: u64, capacity: u64) -> Self {
        Self {
            width,
            depth,
            capacity,
        }
    }
}

pub struct WorthQueryPendingWorkflowGraphChunk {
    active: WorthQueryActiveWorkflowGraphExecution,
    report: WorthQueryGraphProviderStepReport,
    material: WorthQueryGraphReadMaterial,
    queue: WorthQueryPendingWorkflowGraphQueueState,
    retained_bytes: usize,
}

impl WorthQueryPendingWorkflowGraphChunk {
    pub(super) fn new(
        active: WorthQueryActiveWorkflowGraphExecution,
        report: WorthQueryGraphProviderStepReport,
        material: WorthQueryGraphReadMaterial,
        queue: WorthQueryPendingWorkflowGraphQueueState,
        retained_bytes: usize,
    ) -> Self {
        Self {
            active,
            report,
            material,
            queue,
            retained_bytes,
        }
    }

    pub fn chunk(&self) -> &WorthQueryGraphReadMaterial {
        &self.material
    }

    pub const fn queue_depth(&self) -> u64 {
        self.queue.depth
    }

    pub const fn queue_capacity(&self) -> u64 {
        self.queue.capacity
    }

    pub fn request_cancellation(
        &self,
        reason: worth_runtime_bridge::facade::BridgeManagedExecutionCancellationReason,
    ) -> Result<
        worth_runtime_bridge::facade::BridgeManagedExecutionCancellation,
        worth_runtime_bridge::facade::BridgeManagedExecutionInterruptionFailure,
    > {
        self.active.request_cancellation(reason)
    }

    pub fn acknowledge(mut self) -> WorthQueryWorkflowGraphStepOutcome {
        let before = match self.active.observe_safe_point() {
            Ok(observation) => observation,
            Err(_) => {
                return self
                    .active
                    .abandoned_terminal(WorthQueryManagedRunTerminalKind::Failed)
            }
        };
        let terminal = consumer_terminal_kind(&before);
        let mutation = match self
            .active
            .running
            .bridge_basis_mut()
            .dequeue_managed_queue(self.queue.width)
        {
            Ok(mutation) => mutation,
            Err(_) => {
                let _ = self.active.release_pending_chunk(self.retained_bytes);
                return self
                    .active
                    .abandoned_terminal(WorthQueryManagedRunTerminalKind::Failed);
            }
        };
        self.active
            .running
            .provider_work_mut()
            .record_queue_mutation(mutation.counters());
        if !self.active.release_pending_chunk(self.retained_bytes) {
            return self
                .active
                .abandoned_terminal(WorthQueryManagedRunTerminalKind::Failed);
        }
        if let Some(kind) = terminal {
            return self.active.interrupted_terminal(kind);
        }
        match self.report.completion() {
            WorthQueryGraphProviderStepCompletion::Continue => {
                self.active.continue_after_safe_point()
            }
            WorthQueryGraphProviderStepCompletion::Complete => {
                self.active.finish_completion(&self.report)
            }
            WorthQueryGraphProviderStepCompletion::Failed => self
                .active
                .abandoned_terminal(WorthQueryManagedRunTerminalKind::Failed),
        }
    }
}
