use super::PhysicalMutationObservation;

#[derive(Default)]
pub(in crate::physical_runtime) struct PhysicalMutationObservationCounters {
    started: u64,
    completed: u64,
    proven_no_effect: u64,
    indeterminate: u64,
    completed_unobserved: u64,
    worker_panics: u64,
    cancellation_accepted: u64,
    cancellation_effectful: u64,
    cancellation_terminal: u64,
    cancellation_stale: u64,
    cancellation_runtime_closing: u64,
}

#[derive(Clone, Copy)]
pub(in crate::physical_runtime) enum PhysicalMutationTerminalClass {
    Completed,
    ProvenNoEffect,
    Indeterminate,
}

#[derive(Clone, Copy)]
pub(in crate::physical_runtime) enum PhysicalMutationCancellationClass {
    Accepted,
    Effectful,
    Terminal,
    Stale,
    RuntimeClosing,
}

impl PhysicalMutationObservationCounters {
    pub(in crate::physical_runtime) fn record_started(&mut self) {
        self.started = self.started.saturating_add(1);
    }

    pub(in crate::physical_runtime) fn record_terminal(
        &mut self,
        class: PhysicalMutationTerminalClass,
        panicked: bool,
    ) {
        match class {
            PhysicalMutationTerminalClass::Completed => {
                self.completed = self.completed.saturating_add(1)
            }
            PhysicalMutationTerminalClass::ProvenNoEffect => {
                self.proven_no_effect = self.proven_no_effect.saturating_add(1)
            }
            PhysicalMutationTerminalClass::Indeterminate => {
                self.indeterminate = self.indeterminate.saturating_add(1)
            }
        }
        if panicked {
            self.worker_panics = self.worker_panics.saturating_add(1);
        }
    }

    pub(in crate::physical_runtime) fn record_completed_unobserved(
        &mut self,
        _event: crate::physical_runtime::CompletedUnobservedPhysicalMutation,
    ) {
        self.completed_unobserved = self.completed_unobserved.saturating_add(1);
    }

    pub(in crate::physical_runtime) fn record_cancellation(
        &mut self,
        class: PhysicalMutationCancellationClass,
    ) {
        let counter = match class {
            PhysicalMutationCancellationClass::Accepted => &mut self.cancellation_accepted,
            PhysicalMutationCancellationClass::Effectful => &mut self.cancellation_effectful,
            PhysicalMutationCancellationClass::Terminal => &mut self.cancellation_terminal,
            PhysicalMutationCancellationClass::Stale => &mut self.cancellation_stale,
            PhysicalMutationCancellationClass::RuntimeClosing => {
                &mut self.cancellation_runtime_closing
            }
        };
        *counter = counter.saturating_add(1);
    }

    pub(in crate::physical_runtime) const fn snapshot(&self) -> PhysicalMutationObservation {
        PhysicalMutationObservation::new([
            self.started,
            self.completed,
            self.proven_no_effect,
            self.indeterminate,
            self.completed_unobserved,
            self.worker_panics,
            self.cancellation_accepted,
            self.cancellation_effectful,
            self.cancellation_terminal,
            self.cancellation_stale,
            self.cancellation_runtime_closing,
        ])
    }
}
