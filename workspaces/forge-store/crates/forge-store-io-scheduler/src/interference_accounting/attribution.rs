use crate::{BackgroundDebtKind, QueueBackpressureCause};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InterferenceAttribution {
    Queueing,
    Backpressure(QueueBackpressureCause),
    ForegroundWait,
    FlushDelay,
    SyncDebt,
    PageCacheWait,
    WorkerHandoffWait,
    BackgroundYield,
    BackgroundDebt(BackgroundDebtKind),
    BackendLatencyInjection,
    BackendContradictedWitness,
    EnvelopeExceeded,
    PolicyDebt,
    ExecutionViolation,
}
