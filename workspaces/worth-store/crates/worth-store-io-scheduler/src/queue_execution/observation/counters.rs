use super::super::execution::completion::QueueBackendCompletionEvidence;
use super::{QueueBackpressureCause, QueueExecutionObservation};
use worth_store_physical_backend::BackendQueueExecutionCompletion;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct QueueExecutionCounterBasis {
    units: QueueExecutionUnitCounts,
    observed: QueueObservedCounters,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct QueueExecutionUnitCounts {
    pub(crate) submitted: u64,
    pub(crate) admitted: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct QueueObservedCounters {
    peak_queue_depth: u32,
    grouped_writes: u32,
    read_ahead_units: u64,
    write_back_units: u64,
    foreground_wait_events: u64,
    mechanical_retries: u64,
    partial_read_events: u64,
    short_write_events: u64,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct QueueExecutionCounterSnapshot {
    submitted_units: u64,
    admitted_units: u64,
    denied_units: u64,
    peak_queue_depth: u32,
    grouped_writes: u32,
    read_ahead_units: u64,
    write_back_units: u64,
    backpressure_events: u64,
    foreground_wait_events: u64,
    mechanical_retries: u64,
    partial_read_events: u64,
    short_write_events: u64,
    violation_events: u64,
    backpressure_cause: Option<QueueBackpressureCause>,
}

impl QueueExecutionCounterSnapshot {
    pub(crate) const fn executed(basis: QueueExecutionCounterBasis) -> Self {
        Self {
            submitted_units: basis.units.submitted,
            admitted_units: basis.units.admitted,
            denied_units: 0,
            peak_queue_depth: basis.observed.peak_queue_depth,
            grouped_writes: basis.observed.grouped_writes,
            read_ahead_units: basis.observed.read_ahead_units,
            write_back_units: basis.observed.write_back_units,
            backpressure_events: 0,
            foreground_wait_events: 0,
            mechanical_retries: basis.observed.mechanical_retries,
            partial_read_events: basis.observed.partial_read_events,
            short_write_events: basis.observed.short_write_events,
            violation_events: 0,
            backpressure_cause: None,
        }
    }

    pub(crate) const fn backpressured(
        basis: QueueExecutionCounterBasis,
        cause: QueueBackpressureCause,
    ) -> Self {
        Self {
            submitted_units: basis.units.submitted,
            admitted_units: basis.units.admitted,
            denied_units: basis.units.submitted.saturating_sub(basis.units.admitted),
            peak_queue_depth: basis.observed.peak_queue_depth,
            grouped_writes: basis.observed.grouped_writes,
            read_ahead_units: basis.observed.read_ahead_units,
            write_back_units: basis.observed.write_back_units,
            backpressure_events: 1,
            foreground_wait_events: basis.observed.foreground_wait_events,
            mechanical_retries: basis.observed.mechanical_retries,
            partial_read_events: basis.observed.partial_read_events,
            short_write_events: basis.observed.short_write_events,
            violation_events: 0,
            backpressure_cause: Some(cause),
        }
    }

    pub(crate) const fn denied(
        basis: QueueExecutionCounterBasis,
        cause: QueueBackpressureCause,
    ) -> Self {
        Self {
            submitted_units: basis.units.submitted,
            admitted_units: basis.units.admitted,
            denied_units: basis.units.submitted.saturating_sub(basis.units.admitted),
            peak_queue_depth: 0,
            grouped_writes: basis.observed.grouped_writes,
            read_ahead_units: basis.observed.read_ahead_units,
            write_back_units: basis.observed.write_back_units,
            backpressure_events: 0,
            foreground_wait_events: 0,
            mechanical_retries: basis.observed.mechanical_retries,
            partial_read_events: basis.observed.partial_read_events,
            short_write_events: basis.observed.short_write_events,
            violation_events: 0,
            backpressure_cause: Some(cause),
        }
    }

    pub(crate) const fn violation_observed(
        basis: QueueExecutionCounterBasis,
        backpressure_cause: Option<QueueBackpressureCause>,
    ) -> Self {
        Self {
            submitted_units: basis.units.submitted,
            admitted_units: basis.units.admitted,
            denied_units: 0,
            peak_queue_depth: basis.observed.peak_queue_depth,
            grouped_writes: basis.observed.grouped_writes,
            read_ahead_units: basis.observed.read_ahead_units,
            write_back_units: basis.observed.write_back_units,
            backpressure_events: if backpressure_cause.is_some() { 1 } else { 0 },
            foreground_wait_events: basis.observed.foreground_wait_events,
            mechanical_retries: basis.observed.mechanical_retries,
            partial_read_events: basis.observed.partial_read_events,
            short_write_events: basis.observed.short_write_events,
            violation_events: 1,
            backpressure_cause,
        }
    }

    pub(crate) fn violation_from_completion(
        units: QueueExecutionUnitCounts,
        completion: &QueueBackendCompletionEvidence,
        grouped_writes: u32,
        backpressure_cause: Option<QueueBackpressureCause>,
    ) -> Self {
        Self::violation_observed(
            QueueExecutionCounterBasis {
                units,
                observed: QueueObservedCounters {
                    peak_queue_depth: completion.queue_depth_sample(),
                    grouped_writes,
                    read_ahead_units: completion.read_ahead_units(),
                    write_back_units: completion.write_back_units(),
                    foreground_wait_events: completion.foreground_wait_events(),
                    mechanical_retries: completion.mechanical_retries(),
                    partial_read_events: completion.partial_read_events(),
                    short_write_events: completion.short_write_events(),
                },
            },
            backpressure_cause,
        )
    }

    pub(crate) fn violation_from_backend_completion(
        units: QueueExecutionUnitCounts,
        completion: BackendQueueExecutionCompletion,
        backpressure_cause: Option<QueueBackpressureCause>,
    ) -> Self {
        Self::violation_observed(
            QueueExecutionCounterBasis {
                units,
                observed: QueueObservedCounters {
                    peak_queue_depth: completion.queue_depth_sample(),
                    grouped_writes: completion.grouped_writes(),
                    read_ahead_units: completion.read_ahead_units(),
                    write_back_units: completion.write_back_units(),
                    foreground_wait_events: completion.foreground_wait_events(),
                    mechanical_retries: completion.mechanical_retries(),
                    partial_read_events: completion.partial_read_events(),
                    short_write_events: completion.short_write_events(),
                },
            },
            backpressure_cause,
        )
    }

    pub const fn submitted_units(self) -> u64 {
        self.submitted_units
    }
    pub const fn admitted_units(self) -> u64 {
        self.admitted_units
    }
    pub const fn denied_units(self) -> u64 {
        self.denied_units
    }
    pub const fn peak_queue_depth(self) -> u32 {
        self.peak_queue_depth
    }
    pub const fn grouped_writes(self) -> u32 {
        self.grouped_writes
    }
    pub const fn read_ahead_units(self) -> u64 {
        self.read_ahead_units
    }
    pub const fn write_back_units(self) -> u64 {
        self.write_back_units
    }
    pub const fn backpressure_events(self) -> u64 {
        self.backpressure_events
    }
    pub const fn foreground_wait_events(self) -> u64 {
        self.foreground_wait_events
    }
    pub const fn mechanical_retries(self) -> u64 {
        self.mechanical_retries
    }
    pub const fn partial_read_events(self) -> u64 {
        self.partial_read_events
    }
    pub const fn short_write_events(self) -> u64 {
        self.short_write_events
    }
    pub const fn violation_events(self) -> u64 {
        self.violation_events
    }
    pub const fn backpressure_cause(self) -> Option<QueueBackpressureCause> {
        self.backpressure_cause
    }
}

impl QueueExecutionCounterBasis {
    pub(crate) const fn from_observation(
        units: QueueExecutionUnitCounts,
        observation: QueueExecutionObservation,
    ) -> Self {
        Self {
            units,
            observed: QueueObservedCounters {
                peak_queue_depth: observation.queue_depth_sample,
                grouped_writes: observation.grouped_writes,
                read_ahead_units: observation.read_ahead_units,
                write_back_units: observation.write_back_units,
                foreground_wait_events: observation.foreground_wait_events,
                mechanical_retries: observation.mechanical_retries,
                partial_read_events: observation.partial_read_events,
                short_write_events: observation.short_write_events,
            },
        }
    }
}

impl QueueExecutionUnitCounts {
    pub(crate) const fn all_admitted(units: u64) -> Self {
        Self {
            submitted: units,
            admitted: units,
        }
    }
}
