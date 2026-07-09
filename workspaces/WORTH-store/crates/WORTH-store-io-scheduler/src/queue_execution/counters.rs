use super::completion::QueueBackendCompletionEvidence;
use super::QueueBackpressureCause;

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
    #[allow(clippy::too_many_arguments)]
    pub(crate) const fn executed(
        submitted_units: u64,
        admitted_units: u64,
        peak_queue_depth: u32,
        grouped_writes: u32,
        read_ahead_units: u64,
        write_back_units: u64,
        mechanical_retries: u64,
        partial_read_events: u64,
        short_write_events: u64,
    ) -> Self {
        Self {
            submitted_units,
            admitted_units,
            denied_units: 0,
            peak_queue_depth,
            grouped_writes,
            read_ahead_units,
            write_back_units,
            backpressure_events: 0,
            foreground_wait_events: 0,
            mechanical_retries,
            partial_read_events,
            short_write_events,
            violation_events: 0,
            backpressure_cause: None,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) const fn backpressured(
        submitted_units: u64,
        admitted_units: u64,
        peak_queue_depth: u32,
        grouped_writes: u32,
        read_ahead_units: u64,
        write_back_units: u64,
        mechanical_retries: u64,
        partial_read_events: u64,
        short_write_events: u64,
        cause: QueueBackpressureCause,
        foreground_wait_events: u64,
    ) -> Self {
        Self {
            submitted_units,
            admitted_units,
            denied_units: submitted_units.saturating_sub(admitted_units),
            peak_queue_depth,
            grouped_writes,
            read_ahead_units,
            write_back_units,
            backpressure_events: 1,
            foreground_wait_events,
            mechanical_retries,
            partial_read_events,
            short_write_events,
            violation_events: 0,
            backpressure_cause: Some(cause),
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) const fn denied(
        submitted_units: u64,
        admitted_units: u64,
        grouped_writes: u32,
        read_ahead_units: u64,
        write_back_units: u64,
        mechanical_retries: u64,
        partial_read_events: u64,
        short_write_events: u64,
        cause: QueueBackpressureCause,
    ) -> Self {
        Self {
            submitted_units,
            admitted_units,
            denied_units: submitted_units.saturating_sub(admitted_units),
            peak_queue_depth: 0,
            grouped_writes,
            read_ahead_units,
            write_back_units,
            backpressure_events: 0,
            foreground_wait_events: 0,
            mechanical_retries,
            partial_read_events,
            short_write_events,
            violation_events: 0,
            backpressure_cause: Some(cause),
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) const fn violation_observed(
        submitted_units: u64,
        admitted_units: u64,
        peak_queue_depth: u32,
        grouped_writes: u32,
        read_ahead_units: u64,
        write_back_units: u64,
        mechanical_retries: u64,
        partial_read_events: u64,
        short_write_events: u64,
        foreground_wait_events: u64,
        backpressure_cause: Option<QueueBackpressureCause>,
    ) -> Self {
        Self {
            submitted_units,
            admitted_units,
            denied_units: 0,
            peak_queue_depth,
            grouped_writes,
            read_ahead_units,
            write_back_units,
            backpressure_events: if backpressure_cause.is_some() { 1 } else { 0 },
            foreground_wait_events,
            mechanical_retries,
            partial_read_events,
            short_write_events,
            violation_events: 1,
            backpressure_cause,
        }
    }

    pub(crate) fn violation_from_completion(
        submitted_units: u64,
        admitted_units: u64,
        completion: QueueBackendCompletionEvidence,
        grouped_writes: u32,
        backpressure_cause: Option<QueueBackpressureCause>,
    ) -> Self {
        Self::violation_observed(
            submitted_units,
            admitted_units,
            completion.queue_depth_sample(),
            grouped_writes,
            completion.read_ahead_units(),
            completion.write_back_units(),
            completion.mechanical_retries(),
            completion.partial_read_events(),
            completion.short_write_events(),
            completion.foreground_wait_events(),
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
