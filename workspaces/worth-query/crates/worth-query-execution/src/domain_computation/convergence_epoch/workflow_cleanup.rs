use std::marker::PhantomData;
use std::sync::Arc;

use super::core::WorthQueryConvergenceEpochCore;
use super::{
    WorthQueryBoundConvergenceReport, WorthQueryConvergenceEpochCounters,
    WorthQueryConvergenceTerminalKind, WorthQueryConvergenceTerminalState,
    WorthQueryRetainedConvergenceCandidateEvidence, WorthQueryWorkflowConvergenceTerminal,
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
    domain_failure: Option<Arc<str>>,
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

    pub fn domain_failure(&self) -> Option<&str> {
        self.domain_failure.as_deref()
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
    domain_failure: Option<Arc<str>>,
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

    pub fn domain_failure(&self) -> Option<&str> {
        self.domain_failure.as_deref()
    }

    pub fn managed_pending(&self) -> &WorthQueryWorkflowRunCleanupPending {
        &self.pending
    }

    pub fn retry(self) -> WorthQueryWorkflowConvergenceCleanupOutcome<State> {
        let Self {
            mut core,
            domain_failure,
            pending,
            terminal_state,
        } = self;
        core.counters_mut().cleaned_up();
        admit_workflow_cleanup_outcome(core, domain_failure, terminal_state, pending.retry())
    }
}

pub struct WorthQueryWorkflowConvergenceCleanupFailure<State>
where
    State: WorthQueryConvergenceTerminalState,
{
    core: WorthQueryConvergenceEpochCore,
    domain_failure: Option<Arc<str>>,
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

    pub fn domain_failure(&self) -> Option<&str> {
        self.domain_failure.as_deref()
    }

    pub fn managed_failure(&self) -> &WorthQueryWorkflowRunCleanupFailure {
        &self.failure
    }

    pub fn retry(self) -> WorthQueryWorkflowConvergenceCleanupOutcome<State> {
        let Self {
            mut core,
            domain_failure,
            failure,
            terminal_state,
        } = self;
        core.counters_mut().cleaned_up();
        admit_workflow_cleanup_outcome(core, domain_failure, terminal_state, failure.retry())
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
            domain_failure,
            terminal_state,
        } = self;
        core.counters_mut().cleaned_up();
        admit_workflow_cleanup_outcome(core, domain_failure, terminal_state, managed.cleanup())
    }
}

fn admit_workflow_cleanup_outcome<State>(
    core: WorthQueryConvergenceEpochCore,
    domain_failure: Option<Arc<str>>,
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
                    domain_failure,
                    managed,
                    terminal_state,
                },
            )
        }
        WorthQueryWorkflowRunCleanupOutcome::Pending(pending) => {
            WorthQueryWorkflowConvergenceCleanupOutcome::Pending(
                WorthQueryWorkflowConvergenceCleanupPending {
                    core,
                    domain_failure,
                    pending,
                    terminal_state,
                },
            )
        }
        WorthQueryWorkflowRunCleanupOutcome::RecoveryRequired(failure) => {
            WorthQueryWorkflowConvergenceCleanupOutcome::RecoveryRequired(
                WorthQueryWorkflowConvergenceCleanupFailure {
                    core,
                    domain_failure,
                    failure,
                    terminal_state,
                },
            )
        }
    }
}
