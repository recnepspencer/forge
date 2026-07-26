use std::marker::PhantomData;

use super::core::WorthQueryConvergenceEpochCore;
use super::{
    WorthQueryBoundConvergenceReport, WorthQueryConvergenceEpochCounters,
    WorthQueryConvergenceIndeterminateCause, WorthQueryConvergenceTerminalKind,
    WorthQueryConvergenceTerminalState, WorthQueryDirectConvergenceTerminal,
    WorthQueryRetainedConvergenceCandidateEvidence,
};
use crate::domain_computation::{
    WorthQueryDirectRunCleanupFailure, WorthQueryDirectRunCleanupReceipt,
};

impl<State> WorthQueryDirectConvergenceTerminal<State>
where
    State: WorthQueryConvergenceTerminalState,
{
    pub fn cleanup(
        self,
    ) -> Result<
        WorthQueryDirectConvergenceCleanupReceipt<State>,
        WorthQueryDirectConvergenceCleanupFailure<State>,
    > {
        let WorthQueryDirectConvergenceTerminal {
            mut core,
            managed,
            indeterminate_cause,
            terminal_state,
        } = self;
        core.counters_mut().cleaned_up();
        match managed.cleanup() {
            Ok(managed) => Ok(WorthQueryDirectConvergenceCleanupReceipt {
                core,
                indeterminate_cause,
                managed,
                terminal_state,
            }),
            Err(failure) => Err(WorthQueryDirectConvergenceCleanupFailure {
                core,
                indeterminate_cause,
                failure,
                terminal_state,
            }),
        }
    }
}

pub struct WorthQueryDirectConvergenceCleanupReceipt<State>
where
    State: WorthQueryConvergenceTerminalState,
{
    core: WorthQueryConvergenceEpochCore,
    indeterminate_cause: Option<WorthQueryConvergenceIndeterminateCause>,
    managed: WorthQueryDirectRunCleanupReceipt,
    terminal_state: PhantomData<State>,
}

impl<State> WorthQueryDirectConvergenceCleanupReceipt<State>
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

    pub fn managed_receipt(&self) -> &WorthQueryDirectRunCleanupReceipt {
        &self.managed
    }
}

pub struct WorthQueryDirectConvergenceCleanupFailure<State>
where
    State: WorthQueryConvergenceTerminalState,
{
    core: WorthQueryConvergenceEpochCore,
    indeterminate_cause: Option<WorthQueryConvergenceIndeterminateCause>,
    failure: WorthQueryDirectRunCleanupFailure,
    terminal_state: PhantomData<State>,
}

impl<State> WorthQueryDirectConvergenceCleanupFailure<State>
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

    pub fn managed_failure(&self) -> &WorthQueryDirectRunCleanupFailure {
        &self.failure
    }

    pub fn retry(self) -> Result<WorthQueryDirectConvergenceCleanupReceipt<State>, Self> {
        WorthQueryDirectConvergenceTerminal::<State>::new(
            self.core,
            self.failure.into_terminal(),
            self.indeterminate_cause,
        )
        .cleanup()
    }
}
