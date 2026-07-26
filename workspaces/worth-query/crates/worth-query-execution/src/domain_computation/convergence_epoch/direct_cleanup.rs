use std::marker::PhantomData;

use super::core::WorthQueryConvergenceEpochCore;
use super::{
    WorthQueryBoundConvergenceReport, WorthQueryConvergenceEpochCounters,
    WorthQueryConvergenceTerminalKind, WorthQueryConvergenceTerminalState,
    WorthQueryDirectConvergenceTerminal, WorthQueryRetainedConvergenceCandidateEvidence,
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
            domain_failure,
            terminal_state,
        } = self;
        core.counters_mut().cleaned_up();
        match managed.cleanup() {
            Ok(managed) => Ok(WorthQueryDirectConvergenceCleanupReceipt {
                core,
                domain_failure,
                managed,
                terminal_state,
            }),
            Err(failure) => Err(WorthQueryDirectConvergenceCleanupFailure {
                core,
                domain_failure,
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
    domain_failure: Option<std::sync::Arc<str>>,
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

    pub fn domain_failure(&self) -> Option<&str> {
        self.domain_failure.as_deref()
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
    domain_failure: Option<std::sync::Arc<str>>,
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

    pub fn domain_failure(&self) -> Option<&str> {
        self.domain_failure.as_deref()
    }

    pub fn managed_failure(&self) -> &WorthQueryDirectRunCleanupFailure {
        &self.failure
    }

    pub fn retry(self) -> Result<WorthQueryDirectConvergenceCleanupReceipt<State>, Self> {
        WorthQueryDirectConvergenceTerminal::<State>::new(
            self.core,
            self.failure.into_terminal(),
            self.domain_failure,
        )
        .cleanup()
    }
}
