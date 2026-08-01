#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UiIntentExecutionShutdownReport {
    execution_entries_disposed: usize,
    reservation_backed_entries_disposed: usize,
    provider_cancellation_calls: usize,
    before_effect_disposals: usize,
    completed_outcomes_disposed: usize,
    partial_effect_disposals: usize,
    indeterminate_effect_disposals: usize,
    recovery_lanes_disposed: usize,
    consequence_pending_outcomes_disposed: usize,
    active_after: usize,
}

#[derive(Default)]
pub(crate) struct UiIntentExecutionShutdownCounts {
    pub(crate) execution_entries_disposed: usize,
    pub(crate) reservation_backed_entries_disposed: usize,
    pub(crate) provider_cancellation_calls: usize,
    pub(crate) before_effect_disposals: usize,
    pub(crate) completed_outcomes_disposed: usize,
    pub(crate) partial_effect_disposals: usize,
    pub(crate) indeterminate_effect_disposals: usize,
    pub(crate) recovery_lanes_disposed: usize,
    pub(crate) consequence_pending_outcomes_disposed: usize,
    pub(crate) active_after: usize,
}

impl UiIntentExecutionShutdownReport {
    pub(crate) const fn from_counts(counts: UiIntentExecutionShutdownCounts) -> Self {
        Self {
            execution_entries_disposed: counts.execution_entries_disposed,
            reservation_backed_entries_disposed: counts.reservation_backed_entries_disposed,
            provider_cancellation_calls: counts.provider_cancellation_calls,
            before_effect_disposals: counts.before_effect_disposals,
            completed_outcomes_disposed: counts.completed_outcomes_disposed,
            partial_effect_disposals: counts.partial_effect_disposals,
            indeterminate_effect_disposals: counts.indeterminate_effect_disposals,
            recovery_lanes_disposed: counts.recovery_lanes_disposed,
            consequence_pending_outcomes_disposed: counts.consequence_pending_outcomes_disposed,
            active_after: counts.active_after,
        }
    }

    pub const fn execution_entries_disposed(self) -> usize {
        self.execution_entries_disposed
    }

    pub const fn reservation_backed_entries_disposed(self) -> usize {
        self.reservation_backed_entries_disposed
    }

    pub const fn provider_cancellation_calls(self) -> usize {
        self.provider_cancellation_calls
    }

    pub const fn before_effect_disposals(self) -> usize {
        self.before_effect_disposals
    }

    pub const fn completed_outcomes_disposed(self) -> usize {
        self.completed_outcomes_disposed
    }

    pub const fn partial_effect_disposals(self) -> usize {
        self.partial_effect_disposals
    }

    pub const fn indeterminate_effect_disposals(self) -> usize {
        self.indeterminate_effect_disposals
    }

    pub const fn recovery_lanes_disposed(self) -> usize {
        self.recovery_lanes_disposed
    }

    pub const fn consequence_pending_outcomes_disposed(self) -> usize {
        self.consequence_pending_outcomes_disposed
    }

    pub const fn active_after(self) -> usize {
        self.active_after
    }
}
