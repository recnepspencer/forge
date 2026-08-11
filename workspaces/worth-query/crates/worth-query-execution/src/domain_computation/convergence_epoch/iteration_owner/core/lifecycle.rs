mod counters;
mod event;

pub use counters::WorthQueryConvergenceEpochCounters;
pub(in crate::domain_computation::convergence_epoch) use event::{
    WorthQueryConvergenceAdmissionStartEvent, WorthQueryConvergenceLifecycleEvent,
};

pub(in crate::domain_computation::convergence_epoch) struct WorthQueryConvergenceEpochLifecycle {
    counters: WorthQueryConvergenceEpochCounters,
}

impl WorthQueryConvergenceEpochLifecycle {
    pub(in crate::domain_computation::convergence_epoch) fn begin<E>(event: E) -> Self
    where
        E: WorthQueryConvergenceAdmissionStartEvent,
    {
        let mut lifecycle = Self {
            counters: WorthQueryConvergenceEpochCounters::empty(),
        };
        lifecycle.record(event);
        lifecycle
    }

    pub(in crate::domain_computation::convergence_epoch) fn record<E>(&mut self, event: E)
    where
        E: WorthQueryConvergenceLifecycleEvent,
    {
        event.apply(&mut self.counters);
    }

    pub(in crate::domain_computation::convergence_epoch) fn counters(
        &self,
    ) -> &WorthQueryConvergenceEpochCounters {
        &self.counters
    }

    pub(in crate::domain_computation::convergence_epoch) fn into_counters(
        self,
    ) -> WorthQueryConvergenceEpochCounters {
        self.counters
    }
}
