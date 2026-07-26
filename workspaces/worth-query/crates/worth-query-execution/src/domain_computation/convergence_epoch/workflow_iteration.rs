use std::sync::Arc;

use worth_query_installation::facade::WorthQueryInstalledGraphParticipationAuthority;

use super::core::WorthQueryConvergenceEpochCore;
use super::domain_assessment_transition::assess_domain_report;
use super::report_admission::{
    admit_assessed_domain_report, WorthQueryConvergenceReportAdmissionFailure,
};
use super::terminal_outcome::{semantic_terminal_kind, workflow_terminal_outcome};
use super::{
    WorthQueryCancelled, WorthQueryConverged, WorthQueryConvergenceDisposition,
    WorthQueryConvergenceDomainProvider, WorthQueryConvergenceEpochDenial,
    WorthQueryConvergenceEpochDenialKind as Kind, WorthQueryConvergenceIndeterminateCause,
    WorthQueryConvergenceIterationStartFailureKind as StartFailureKind,
    WorthQueryConvergenceTerminalKind, WorthQueryExhausted, WorthQueryFeasibleIncumbent,
    WorthQueryIndeterminate, WorthQueryIteratingWorkflowConvergenceEpoch, WorthQueryOscillating,
    WorthQueryStableWithoutProof, WorthQueryWorkflowConvergenceTerminal,
};
use crate::domain_computation::{
    WorthQueryActiveWorkflowGraphExecution, WorthQueryCompletedWorkflowGraphExecution,
    WorthQueryManagedGraphCallRequest, WorthQueryManagedRunTerminalKind,
    WorthQueryWorkflowRunTerminal,
};

pub struct WorthQueryStartedWorkflowConvergenceIteration {
    pub(super) pending: WorthQueryPendingWorkflowConvergenceIteration,
    pub(super) execution: WorthQueryActiveWorkflowGraphExecution,
}

impl WorthQueryStartedWorkflowConvergenceIteration {
    pub fn into_parts(
        self,
    ) -> (
        WorthQueryPendingWorkflowConvergenceIteration,
        WorthQueryActiveWorkflowGraphExecution,
    ) {
        (self.pending, self.execution)
    }
}

pub struct WorthQueryPendingWorkflowConvergenceIteration {
    pub(super) core: WorthQueryConvergenceEpochCore,
    pub(super) expected_run_identity: Arc<str>,
    pub(super) stage_identity: Arc<str>,
    pub(super) graph: WorthQueryInstalledGraphParticipationAuthority,
    pub(super) provider: Arc<dyn WorthQueryConvergenceDomainProvider>,
}

pub struct WorthQueryWorkflowConvergenceIterationStartRejection {
    denial: WorthQueryConvergenceEpochDenial,
    epoch: WorthQueryIteratingWorkflowConvergenceEpoch,
}

pub struct WorthQueryWorkflowConvergenceIterationStartTermination {
    denial: WorthQueryConvergenceEpochDenial,
    outcome: WorthQueryWorkflowConvergenceIterationOutcome,
}

impl WorthQueryWorkflowConvergenceIterationStartRejection {
    pub fn denial(&self) -> &WorthQueryConvergenceEpochDenial {
        &self.denial
    }

    pub fn into_epoch(self) -> WorthQueryIteratingWorkflowConvergenceEpoch {
        self.epoch
    }

    pub fn terminate(self) -> WorthQueryWorkflowConvergenceIterationStartTermination {
        let terminal_kind = if self.denial.kind() == Kind::IterationBudgetExhausted {
            WorthQueryConvergenceTerminalKind::Exhausted
        } else {
            WorthQueryConvergenceTerminalKind::Indeterminate
        };
        let managed_kind = if terminal_kind == WorthQueryConvergenceTerminalKind::Exhausted {
            WorthQueryManagedRunTerminalKind::Exhausted
        } else {
            WorthQueryManagedRunTerminalKind::Failed
        };
        let indeterminate_cause =
            (terminal_kind == WorthQueryConvergenceTerminalKind::Indeterminate).then(|| {
                WorthQueryConvergenceIndeterminateCause::EpochProgression(self.denial.clone())
            });
        let WorthQueryIteratingWorkflowConvergenceEpoch {
            core, managed_run, ..
        } = self.epoch;
        WorthQueryWorkflowConvergenceIterationStartTermination {
            denial: self.denial,
            outcome: workflow_terminal_outcome(
                core,
                managed_run.terminate_for_convergence(managed_kind),
                terminal_kind,
                indeterminate_cause,
            ),
        }
    }
}

impl WorthQueryWorkflowConvergenceIterationStartTermination {
    pub fn denial(&self) -> &WorthQueryConvergenceEpochDenial {
        &self.denial
    }

    pub fn outcome(&self) -> &WorthQueryWorkflowConvergenceIterationOutcome {
        &self.outcome
    }

    pub fn into_parts(
        self,
    ) -> (
        WorthQueryConvergenceEpochDenial,
        WorthQueryWorkflowConvergenceIterationOutcome,
    ) {
        (self.denial, self.outcome)
    }
}

pub struct WorthQueryWorkflowConvergenceCompletionRejection {
    denial: WorthQueryConvergenceEpochDenial,
    pending: WorthQueryPendingWorkflowConvergenceIteration,
    completion: WorthQueryCompletedWorkflowGraphExecution,
}

impl WorthQueryWorkflowConvergenceCompletionRejection {
    pub fn denial(&self) -> &WorthQueryConvergenceEpochDenial {
        &self.denial
    }

    pub fn into_parts(
        self,
    ) -> (
        WorthQueryPendingWorkflowConvergenceIteration,
        WorthQueryCompletedWorkflowGraphExecution,
    ) {
        (self.pending, self.completion)
    }
}

pub enum WorthQueryWorkflowConvergenceIterationOutcome {
    Continue(WorthQueryIteratingWorkflowConvergenceEpoch),
    Converged(WorthQueryWorkflowConvergenceTerminal<WorthQueryConverged>),
    StableWithoutProof(WorthQueryWorkflowConvergenceTerminal<WorthQueryStableWithoutProof>),
    FeasibleIncumbent(WorthQueryWorkflowConvergenceTerminal<WorthQueryFeasibleIncumbent>),
    Oscillating(WorthQueryWorkflowConvergenceTerminal<WorthQueryOscillating>),
    Exhausted(WorthQueryWorkflowConvergenceTerminal<WorthQueryExhausted>),
    Cancelled(WorthQueryWorkflowConvergenceTerminal<WorthQueryCancelled>),
    Indeterminate(WorthQueryWorkflowConvergenceTerminal<WorthQueryIndeterminate>),
}

impl WorthQueryIteratingWorkflowConvergenceEpoch {
    pub fn begin_stage_iteration(
        self,
        stage_identity: &str,
        request: WorthQueryManagedGraphCallRequest,
    ) -> Result<
        WorthQueryStartedWorkflowConvergenceIteration,
        WorthQueryWorkflowConvergenceIterationStartRejection,
    > {
        if self.core.counters().iteration_count() >= self.core.contract().iteration_bound() {
            return Err(start_denied(
                self,
                Kind::IterationBudgetExhausted,
                "convergence iteration budget is already exhausted",
            ));
        }
        if self.core.contract().evidence_stage_identity() != Some(stage_identity) {
            return Err(start_denied(
                self,
                Kind::WorkflowEvidenceStageMismatch,
                "workflow convergence iteration must use the installed evidence stage sealed by admission",
            ));
        }
        let WorthQueryIteratingWorkflowConvergenceEpoch {
            mut core,
            managed_run,
            graph,
            provider,
        } = self;
        let expected_run_identity = Arc::from(managed_run.identity());
        let execution =
            match managed_run.begin_stage_graph_execution(stage_identity, &graph, request) {
                Ok(execution) => execution,
                Err(failure) => {
                    let detail = Arc::from(failure.detail());
                    let kind = failure.kind();
                    return Err(start_denied_owned(
                        WorthQueryIteratingWorkflowConvergenceEpoch {
                            core,
                            managed_run: failure.into_running(),
                            graph,
                            provider,
                        },
                        Kind::ManagedIterationStart(StartFailureKind::Workflow(kind)),
                        detail,
                    ));
                }
            };
        core.counters_mut().began_iteration();
        Ok(WorthQueryStartedWorkflowConvergenceIteration {
            pending: WorthQueryPendingWorkflowConvergenceIteration {
                core,
                expected_run_identity,
                stage_identity: Arc::from(stage_identity),
                graph,
                provider,
            },
            execution,
        })
    }
}

impl WorthQueryPendingWorkflowConvergenceIteration {
    pub fn admit_completion(
        mut self,
        completion: WorthQueryCompletedWorkflowGraphExecution,
    ) -> Result<
        WorthQueryWorkflowConvergenceIterationOutcome,
        WorthQueryWorkflowConvergenceCompletionRejection,
    > {
        if completion.run_identity() != self.expected_run_identity.as_ref() {
            let denial = WorthQueryConvergenceEpochDenial::new(
                Kind::IterationRunMismatch,
                "completed graph execution belongs to another managed workflow run",
                self.core.counters().clone(),
            );
            return Err(WorthQueryWorkflowConvergenceCompletionRejection {
                denial,
                pending: self,
                completion,
            });
        }
        let assessment = match assess_domain_report(
            &mut self.core,
            self.provider.as_ref(),
            completion.receipt(),
        ) {
            Ok(assessment) => assessment,
            Err(failure) => {
                let (running, _) = completion.into_parts();
                return Ok(indeterminate(
                    self.core,
                    running,
                    WorthQueryConvergenceIndeterminateCause::DomainInvocation(failure),
                ));
            }
        };
        let domain_evidence = match completion.bind_convergence_candidate_evidence(
            &self.stage_identity,
            assessment.decision().candidate_occurrence_identity(),
        ) {
            Ok(evidence) => evidence,
            Err(denial) => {
                let (running, _) = completion.into_parts();
                return Ok(indeterminate(
                    self.core,
                    running,
                    WorthQueryConvergenceIndeterminateCause::DomainEvidenceBinding(denial),
                ));
            }
        };
        let disposition = match admit_assessed_domain_report(
            &mut self.core,
            completion.receipt(),
            assessment,
            domain_evidence,
        ) {
            Ok(disposition) => disposition,
            Err(WorthQueryConvergenceReportAdmissionFailure::Epoch(denial)) => {
                let (running, _) = completion.into_parts();
                return Ok(indeterminate(
                    self.core,
                    running,
                    WorthQueryConvergenceIndeterminateCause::ReportAdmission(denial),
                ));
            }
        };
        let (running, _) = completion.into_parts();
        if disposition == WorthQueryConvergenceDisposition::Continue {
            if self.core.counters().iteration_count() >= self.core.contract().iteration_bound() {
                return Ok(workflow_terminal_outcome(
                    self.core,
                    running.terminate_for_convergence(WorthQueryManagedRunTerminalKind::Exhausted),
                    WorthQueryConvergenceTerminalKind::Exhausted,
                    None,
                ));
            }
            return Ok(WorthQueryWorkflowConvergenceIterationOutcome::Continue(
                WorthQueryIteratingWorkflowConvergenceEpoch {
                    core: self.core,
                    managed_run: running,
                    graph: self.graph,
                    provider: self.provider,
                },
            ));
        }
        let kind = semantic_terminal_kind(disposition);
        let managed = match running.completed() {
            Ok(terminal) => terminal,
            Err(rejection) => {
                let denial = rejection.denial().clone();
                return Ok(indeterminate(
                    self.core,
                    rejection.into_running(),
                    WorthQueryConvergenceIndeterminateCause::ManagedCompletion(denial),
                ));
            }
        };
        Ok(workflow_terminal_outcome(self.core, managed, kind, None))
    }

    pub fn admit_managed_terminal(
        mut self,
        terminal: WorthQueryWorkflowRunTerminal,
    ) -> Result<
        WorthQueryWorkflowConvergenceIterationOutcome,
        (
            WorthQueryPendingWorkflowConvergenceIteration,
            WorthQueryWorkflowRunTerminal,
        ),
    > {
        if terminal.identity() != self.expected_run_identity.as_ref() {
            return Err((self, terminal));
        }
        self.core
            .counters_mut()
            .reconciled_provider_work_total(terminal.provider_work().completed_work_units());
        let managed_kind = terminal.kind();
        let kind = match managed_kind {
            WorthQueryManagedRunTerminalKind::Cancelled => {
                WorthQueryConvergenceTerminalKind::Cancelled
            }
            WorthQueryManagedRunTerminalKind::Exhausted => {
                WorthQueryConvergenceTerminalKind::Exhausted
            }
            _ => WorthQueryConvergenceTerminalKind::Indeterminate,
        };
        let indeterminate_cause = (kind == WorthQueryConvergenceTerminalKind::Indeterminate)
            .then_some(WorthQueryConvergenceIndeterminateCause::ManagedTerminal(
                managed_kind,
            ));
        Ok(workflow_terminal_outcome(
            self.core,
            terminal,
            kind,
            indeterminate_cause,
        ))
    }
}

fn indeterminate(
    core: WorthQueryConvergenceEpochCore,
    running: crate::domain_computation::WorthQueryRunningWorkflowRun,
    cause: WorthQueryConvergenceIndeterminateCause,
) -> WorthQueryWorkflowConvergenceIterationOutcome {
    workflow_terminal_outcome(
        core,
        running.terminate_for_convergence(WorthQueryManagedRunTerminalKind::Failed),
        WorthQueryConvergenceTerminalKind::Indeterminate,
        Some(cause),
    )
}

fn start_denied(
    epoch: WorthQueryIteratingWorkflowConvergenceEpoch,
    kind: Kind,
    detail: &'static str,
) -> WorthQueryWorkflowConvergenceIterationStartRejection {
    start_denied_owned(epoch, kind, Arc::<str>::from(detail))
}

fn start_denied_owned(
    epoch: WorthQueryIteratingWorkflowConvergenceEpoch,
    kind: Kind,
    detail: Arc<str>,
) -> WorthQueryWorkflowConvergenceIterationStartRejection {
    WorthQueryWorkflowConvergenceIterationStartRejection {
        denial: WorthQueryConvergenceEpochDenial::new(kind, detail, epoch.core.counters().clone()),
        epoch,
    }
}
