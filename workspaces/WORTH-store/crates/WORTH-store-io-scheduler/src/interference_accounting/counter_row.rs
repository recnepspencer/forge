use worth_store_budgets::CounterEvidenceStrength;

use crate::QueueWorkClass;

use super::InterferenceAttribution;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InterferenceCounterName {
    QueueSubmittedUnits,
    QueueAdmittedUnits,
    QueueDeniedUnits,
    QueuePeakDepth,
    QueueGroupedWrites,
    QueueReadAheadUnits,
    QueueWriteBackUnits,
    QueueBackpressureEvents,
    QueueForegroundWaitEvents,
    QueueMechanicalRetries,
    QueuePartialReadEvents,
    QueueShortWriteEvents,
    QueueViolationEvents,
    FlushDelayEvents,
    SyncDebtUnits,
    PageCacheWaitEvents,
    WorkerHandoffWaitEvents,
    BackendContradictionEvents,
    EnvelopeExceededEvents,
    PolicyDebtEvents,
    BackgroundYieldEvents,
    BackgroundDebtUnits,
    BackgroundViolationEvents,
}

impl InterferenceCounterName {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::QueueSubmittedUnits => "queue.submitted-units",
            Self::QueueAdmittedUnits => "queue.admitted-units",
            Self::QueueDeniedUnits => "queue.denied-units",
            Self::QueuePeakDepth => "queue.peak-depth",
            Self::QueueGroupedWrites => "queue.grouped-writes",
            Self::QueueReadAheadUnits => "queue.read-ahead-units",
            Self::QueueWriteBackUnits => "queue.write-back-units",
            Self::QueueBackpressureEvents => "queue.backpressure-events",
            Self::QueueForegroundWaitEvents => "queue.foreground-wait-events",
            Self::QueueMechanicalRetries => "queue.mechanical-retries",
            Self::QueuePartialReadEvents => "queue.partial-read-events",
            Self::QueueShortWriteEvents => "queue.short-write-events",
            Self::QueueViolationEvents => "queue.violation-events",
            Self::FlushDelayEvents => "flush.delay-events",
            Self::SyncDebtUnits => "sync.debt-units",
            Self::PageCacheWaitEvents => "page-cache.wait-events",
            Self::WorkerHandoffWaitEvents => "worker.handoff-wait-events",
            Self::BackendContradictionEvents => "backend.contradiction-events",
            Self::EnvelopeExceededEvents => "latency-envelope.exceeded-events",
            Self::PolicyDebtEvents => "policy.debt-events",
            Self::BackgroundYieldEvents => "background.yield-events",
            Self::BackgroundDebtUnits => "background.debt-units",
            Self::BackgroundViolationEvents => "background.violation-events",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct InterferenceCounterRow {
    name: InterferenceCounterName,
    value: u64,
    strength: CounterEvidenceStrength,
    profile_scope: &'static str,
    lane: QueueWorkClass,
    attribution: Option<InterferenceAttribution>,
}

impl InterferenceCounterRow {
    pub const fn new(
        name: InterferenceCounterName,
        value: u64,
        strength: CounterEvidenceStrength,
        profile_scope: &'static str,
        lane: QueueWorkClass,
        attribution: Option<InterferenceAttribution>,
    ) -> Self {
        Self {
            name,
            value,
            strength,
            profile_scope,
            lane,
            attribution,
        }
    }

    pub const fn name(self) -> InterferenceCounterName {
        self.name
    }

    pub const fn value(self) -> u64 {
        self.value
    }

    pub const fn strength(self) -> CounterEvidenceStrength {
        self.strength
    }

    pub const fn profile_scope(self) -> &'static str {
        self.profile_scope
    }

    pub const fn lane(self) -> QueueWorkClass {
        self.lane
    }

    pub const fn attribution(self) -> Option<InterferenceAttribution> {
        self.attribution
    }
}
