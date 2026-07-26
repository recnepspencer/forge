use std::marker::PhantomData;

use super::core::WorthQueryConvergenceEpochCore;
use super::{
    WorthQueryBoundConvergenceReport, WorthQueryConvergenceEpochCounters,
    WorthQueryConvergenceIndeterminateCause, WorthQueryConvergenceTerminalKind,
    WorthQueryConvergenceTerminalState, WorthQueryRetainedConvergenceCandidateEvidence,
    WorthQueryWorkflowConvergenceTerminal,
};
use crate::domain_computation::{
    WorthQueryManagedRunCleanupDisposition, WorthQueryWorkflowRunCleanupFailure,
    WorthQueryWorkflowRunCleanupOutcome, WorthQueryWorkflowRunCleanupPending,
    WorthQueryWorkflowRunCleanupReceipt,
};

pub enum WorthQueryWorkflowConvergenceCleanupOutcome<State>
where
    State: WorthQueryConvergenceTerminalState,
{
    Complete(WorthQueryWorkflowConvergenceCleanupReceipt<State>),
    Pending(WorthQueryWorkflowConvergenceCleanupPending<State>),
    RecoveryRequired(WorthQueryWorkflowConvergenceCleanupFailure<State>),
}

impl<State> WorthQueryWorkflowConvergenceCleanupOutcome<State>
where
    State: WorthQueryConvergenceTerminalState,
{
    pub fn disposition(&self) -> WorthQueryManagedRunCleanupDisposition {
        match self {
            Self::Complete(receipt) => receipt.managed_receipt().disposition(),
            Self::Pending(_) => WorthQueryManagedRunCleanupDisposition::CleanupPending,
            Self::RecoveryRequired(_) => WorthQueryManagedRunCleanupDisposition::RecoveryRequired,
        }
    }

    pub fn counters(&self) -> &WorthQueryConvergenceEpochCounters {
        match self {
            Self::Complete(receipt) => receipt.counters(),
            Self::Pending(pending) => pending.counters(),
            Self::RecoveryRequired(failure) => failure.counters(),
        }
    }
}

pub struct WorthQueryWorkflowConvergenceCleanupReceipt<State>
where
    State: WorthQueryConvergenceTerminalState,
{
    core: WorthQueryConvergenceEpochCore,
    indeterminate_cause: Option<WorthQueryConvergenceIndeterminateCause>,
    managed: WorthQueryWorkflowRunCleanupReceipt,
    terminal_state: PhantomData<State>,
}

impl<State> WorthQueryWorkflowConvergenceCleanupReceipt<State>
where
    State: WorthQueryConvergenceTerminalState,
{
    pub fn identity(&self) -> &str {
        self.core.identity()
    }

    pub const fn kind(&self) -> WorthQueryConvergenceTerminalKind {
        State::KIND
    }

    pub fn counters(&self) -> &WorthQueryConvergenceEpochCounters {
        self.core.counters()
    }

    pub fn incumbents(&self) -> &[WorthQueryRetainedConvergenceCandidateEvidence] {
        self.core.incumbents()
    }

    pub fn latest_report(&self) -> Option<&WorthQueryBoundConvergenceReport> {
        self.core.latest_report()
    }

    pub fn indeterminate_cause(&self) -> Option<&WorthQueryConvergenceIndeterminateCause> {
        self.indeterminate_cause.as_ref()
    }

    pub fn managed_receipt(&self) -> &WorthQueryWorkflowRunCleanupReceipt {
        &self.managed
    }
}

pub struct WorthQueryWorkflowConvergenceCleanupPending<State>
where
    State: WorthQueryConvergenceTerminalState,
{
    core: WorthQueryConvergenceEpochCore,
    indeterminate_cause: Option<WorthQueryConvergenceIndeterminateCause>,
    pending: WorthQueryWorkflowRunCleanupPending,
    terminal_state: PhantomData<State>,
}

impl<State> WorthQueryWorkflowConvergenceCleanupPending<State>
where
    State: WorthQueryConvergenceTerminalState,
{
    pub fn identity(&self) -> &str {
        self.core.identity()
    }

    pub const fn kind(&self) -> WorthQueryConvergenceTerminalKind {
        State::KIND
    }

    pub fn counters(&self) -> &WorthQueryConvergenceEpochCounters {
        self.core.counters()
    }

    pub fn incumbents(&self) -> &[WorthQueryRetainedConvergenceCandidateEvidence] {
        self.core.incumbents()
    }

    pub fn latest_report(&self) -> Option<&WorthQueryBoundConvergenceReport> {
        self.core.latest_report()
    }

    pub fn indeterminate_cause(&self) -> Option<&WorthQueryConvergenceIndeterminateCause> {
        self.indeterminate_cause.as_ref()
    }

    pub fn managed_pending(&self) -> &WorthQueryWorkflowRunCleanupPending {
        &self.pending
    }

    pub fn retry(self) -> WorthQueryWorkflowConvergenceCleanupOutcome<State> {
        let Self {
            mut core,
            indeterminate_cause,
            pending,
            terminal_state,
        } = self;
        core.counters_mut().cleaned_up();
        admit_workflow_cleanup_outcome(core, indeterminate_cause, terminal_state, pending.retry())
    }
}

pub struct WorthQueryWorkflowConvergenceCleanupFailure<State>
where
    State: WorthQueryConvergenceTerminalState,
{
    core: WorthQueryConvergenceEpochCore,
    indeterminate_cause: Option<WorthQueryConvergenceIndeterminateCause>,
    failure: WorthQueryWorkflowRunCleanupFailure,
    terminal_state: PhantomData<State>,
}

impl<State> WorthQueryWorkflowConvergenceCleanupFailure<State>
where
    State: WorthQueryConvergenceTerminalState,
{
    pub fn identity(&self) -> &str {
        self.core.identity()
    }

    pub const fn kind(&self) -> WorthQueryConvergenceTerminalKind {
        State::KIND
    }

    pub fn counters(&self) -> &WorthQueryConvergenceEpochCounters {
        self.core.counters()
    }

    pub fn incumbents(&self) -> &[WorthQueryRetainedConvergenceCandidateEvidence] {
        self.core.incumbents()
    }

    pub fn latest_report(&self) -> Option<&WorthQueryBoundConvergenceReport> {
        self.core.latest_report()
    }

    pub fn indeterminate_cause(&self) -> Option<&WorthQueryConvergenceIndeterminateCause> {
        self.indeterminate_cause.as_ref()
    }

    pub fn managed_failure(&self) -> &WorthQueryWorkflowRunCleanupFailure {
        &self.failure
    }

    pub fn retry(self) -> WorthQueryWorkflowConvergenceCleanupOutcome<State> {
        let Self {
            mut core,
            indeterminate_cause,
            failure,
            terminal_state,
        } = self;
        core.counters_mut().cleaned_up();
        admit_workflow_cleanup_outcome(core, indeterminate_cause, terminal_state, failure.retry())
    }
}

impl<State> WorthQueryWorkflowConvergenceTerminal<State>
where
    State: WorthQueryConvergenceTerminalState,
{
    pub fn cleanup(self) -> WorthQueryWorkflowConvergenceCleanupOutcome<State> {
        let WorthQueryWorkflowConvergenceTerminal {
            mut core,
            managed,
            indeterminate_cause,
            terminal_state,
        } = self;
        core.counters_mut().cleaned_up();
        admit_workflow_cleanup_outcome(core, indeterminate_cause, terminal_state, managed.cleanup())
    }
}

fn admit_workflow_cleanup_outcome<State>(
    core: WorthQueryConvergenceEpochCore,
    indeterminate_cause: Option<WorthQueryConvergenceIndeterminateCause>,
    terminal_state: PhantomData<State>,
    outcome: WorthQueryWorkflowRunCleanupOutcome,
) -> WorthQueryWorkflowConvergenceCleanupOutcome<State>
where
    State: WorthQueryConvergenceTerminalState,
{
    match outcome {
        WorthQueryWorkflowRunCleanupOutcome::Complete(managed) => {
            WorthQueryWorkflowConvergenceCleanupOutcome::Complete(
                WorthQueryWorkflowConvergenceCleanupReceipt {
                    core,
                    indeterminate_cause,
                    managed,
                    terminal_state,
                },
            )
        }
        WorthQueryWorkflowRunCleanupOutcome::Pending(pending) => {
            WorthQueryWorkflowConvergenceCleanupOutcome::Pending(
                WorthQueryWorkflowConvergenceCleanupPending {
                    core,
                    indeterminate_cause,
                    pending,
                    terminal_state,
                },
            )
        }
        WorthQueryWorkflowRunCleanupOutcome::RecoveryRequired(failure) => {
            WorthQueryWorkflowConvergenceCleanupOutcome::RecoveryRequired(
                WorthQueryWorkflowConvergenceCleanupFailure {
                    core,
                    indeterminate_cause,
                    failure,
                    terminal_state,
                },
            )
        }
    }
}
