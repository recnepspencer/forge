use std::num::NonZeroU64;

use crate::physical_runtime::LifecycleGeneration;

use super::{cancellation::ManagedIntegrityScrubCancellation, close::ManagedIntegrityScrubClose};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum ManagedIntegrityScrubGate {
    Proceed,
    Paused,
    Cancelled,
    Closed,
    StaleRuntimeGeneration,
}

pub(super) struct ManagedIntegrityScrubLifecycle {
    generation: LifecycleGeneration,
    completed_in_slice: u64,
    yield_after_windows: Option<NonZeroU64>,
    cancellation: Option<ManagedIntegrityScrubCancellation>,
    close: Option<ManagedIntegrityScrubClose>,
}

impl ManagedIntegrityScrubLifecycle {
    pub(super) const fn new(
        generation: LifecycleGeneration,
        yield_after_windows: Option<NonZeroU64>,
    ) -> Self {
        Self {
            generation,
            completed_in_slice: 0,
            yield_after_windows,
            cancellation: None,
            close: None,
        }
    }

    pub(super) fn gate(
        &mut self,
        current_generation: LifecycleGeneration,
    ) -> ManagedIntegrityScrubGate {
        if current_generation != self.generation {
            return ManagedIntegrityScrubGate::StaleRuntimeGeneration;
        }
        if self.close.is_some() {
            return ManagedIntegrityScrubGate::Closed;
        }
        if self.cancellation.is_some() {
            return ManagedIntegrityScrubGate::Cancelled;
        }
        if self
            .yield_after_windows
            .is_some_and(|limit| self.completed_in_slice >= limit.get())
        {
            self.completed_in_slice = 0;
            return ManagedIntegrityScrubGate::Paused;
        }
        ManagedIntegrityScrubGate::Proceed
    }

    pub(super) fn record_completed_window(&mut self) {
        self.completed_in_slice += 1;
    }

    pub(super) fn cancel(&mut self) {
        self.cancellation = Some(ManagedIntegrityScrubCancellation::Requested);
    }

    pub(super) fn close(&mut self) {
        self.close = Some(ManagedIntegrityScrubClose::RuntimeClosing);
    }

    pub(super) const fn generation(&self) -> LifecycleGeneration {
        self.generation
    }
}

#[cfg(test)]
mod tests {
    use std::num::{NonZeroU64, NonZeroU64 as Generation};

    use super::*;

    fn generation(value: u64) -> LifecycleGeneration {
        LifecycleGeneration::from_reopened(Generation::new(value).unwrap())
    }

    #[test]
    fn cooperative_yield_pauses_once_then_resumes() {
        let current = generation(7);
        let mut lifecycle = ManagedIntegrityScrubLifecycle::new(current, Some(NonZeroU64::MIN));
        lifecycle.record_completed_window();
        assert_eq!(lifecycle.gate(current), ManagedIntegrityScrubGate::Paused);
        assert_eq!(lifecycle.gate(current), ManagedIntegrityScrubGate::Proceed);
    }

    #[test]
    fn cancellation_is_terminal_for_the_bound_generation() {
        let current = generation(8);
        let mut lifecycle = ManagedIntegrityScrubLifecycle::new(current, None);
        lifecycle.cancel();
        assert_eq!(
            lifecycle.gate(current),
            ManagedIntegrityScrubGate::Cancelled
        );
    }

    #[test]
    fn close_dominates_later_observation() {
        let current = generation(9);
        let mut lifecycle = ManagedIntegrityScrubLifecycle::new(current, None);
        lifecycle.close();
        assert_eq!(lifecycle.gate(current), ManagedIntegrityScrubGate::Closed);
    }

    #[test]
    fn runtime_generation_drift_is_rejected_before_progress() {
        let mut lifecycle = ManagedIntegrityScrubLifecycle::new(generation(10), None);
        assert_eq!(
            lifecycle.gate(generation(11)),
            ManagedIntegrityScrubGate::StaleRuntimeGeneration
        );
    }
}
