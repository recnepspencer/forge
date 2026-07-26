use super::workflow_graph_chunk::WorthQueryPendingWorkflowGraphChunk;
use super::workflow_graph_execution::WorthQueryActiveWorkflowGraphExecution;
use super::{
    WorthQueryManagedRunTerminalKind, WorthQueryRunningWorkflowRun, WorthQueryWorkflowRunTerminal,
};
use crate::domain_computation::WorthQueryBoundGraphExecutionReceipt;

#[must_use = "paused graph execution must be advanced, yielded, or explicitly abandoned"]
pub struct WorthQueryPausedWorkflowGraphExecution {
    pub(super) active: WorthQueryActiveWorkflowGraphExecution,
    pub(super) safe_point: super::yield_eligibility::WorthQueryManagedYieldSafePoint,
}

impl WorthQueryPausedWorkflowGraphExecution {
    pub fn run_identity(&self) -> &str {
        self.active.run_identity()
    }

    pub fn advance(self) -> WorthQueryWorkflowGraphStepOutcome {
        self.active.advance()
    }

    pub fn yield_run(self) -> super::WorthQueryWorkflowYieldOutcome {
        super::workflow_yield_transition::yield_workflow_run(self)
    }

    pub fn abandon(self) -> WorthQueryWorkflowGraphStepOutcome {
        self.active.abandon()
    }
}

pub struct WorthQueryCompletedWorkflowGraphExecution {
    running: WorthQueryRunningWorkflowRun,
    receipt: WorthQueryBoundGraphExecutionReceipt,
}

impl WorthQueryCompletedWorkflowGraphExecution {
    pub(super) fn new(
        running: WorthQueryRunningWorkflowRun,
        receipt: WorthQueryBoundGraphExecutionReceipt,
    ) -> Self {
        Self { running, receipt }
    }

    pub fn run_identity(&self) -> &str {
        self.running.identity()
    }

    pub fn receipt(&self) -> &WorthQueryBoundGraphExecutionReceipt {
        &self.receipt
    }

    pub fn into_running(self) -> WorthQueryRunningWorkflowRun {
        self.running
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        WorthQueryRunningWorkflowRun,
        WorthQueryBoundGraphExecutionReceipt,
    ) {
        (self.running, self.receipt)
    }

    pub(crate) fn bind_convergence_candidate_evidence(
        &self,
        stage_identity: &str,
        output_occurrence_identity: &str,
    ) -> Result<
        crate::domain_computation::WorthQueryDomainEvidenceExecutionBinding,
        crate::domain_computation::WorthQueryDomainEvidenceBindingDenial,
    > {
        self.running
            .bind_convergence_candidate_evidence(stage_identity, output_occurrence_identity)
    }
}

pub enum WorthQueryWorkflowGraphStepOutcome {
    Continue(WorthQueryPausedWorkflowGraphExecution),
    ChunkReady(WorthQueryPendingWorkflowGraphChunk),
    Completed(WorthQueryCompletedWorkflowGraphExecution),
    Cancelled(WorthQueryWorkflowRunTerminal),
    TimedOut(WorthQueryWorkflowRunTerminal),
    Exhausted(WorthQueryWorkflowRunTerminal),
    Degraded(WorthQueryWorkflowRunTerminal),
    Failed(WorthQueryWorkflowRunTerminal),
}

pub(super) fn terminal_outcome(
    terminal: WorthQueryWorkflowRunTerminal,
    kind: WorthQueryManagedRunTerminalKind,
) -> WorthQueryWorkflowGraphStepOutcome {
    match kind {
        WorthQueryManagedRunTerminalKind::Cancelled => {
            WorthQueryWorkflowGraphStepOutcome::Cancelled(terminal)
        }
        WorthQueryManagedRunTerminalKind::TimedOut => {
            WorthQueryWorkflowGraphStepOutcome::TimedOut(terminal)
        }
        WorthQueryManagedRunTerminalKind::Exhausted => {
            WorthQueryWorkflowGraphStepOutcome::Exhausted(terminal)
        }
        WorthQueryManagedRunTerminalKind::Degraded => {
            WorthQueryWorkflowGraphStepOutcome::Degraded(terminal)
        }
        WorthQueryManagedRunTerminalKind::Failed => {
            WorthQueryWorkflowGraphStepOutcome::Failed(terminal)
        }
        WorthQueryManagedRunTerminalKind::Completed => {
            unreachable!("provider completion returns a completion authority")
        }
    }
}
