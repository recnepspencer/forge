use super::{
    ForegroundIoLaneKind, ForegroundLatencyEnvelope, ForegroundReservationCounterSnapshot,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForegroundReservationViolationCause {
    EnvelopeExceeded {
        allowed_interference_events: u64,
        observed_interference_events: u64,
    },
    CapacityConsumedWithoutPermit,
    SecurityScopeDrift,
    BackendRebindObservedAfterAdmission,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ReservationViolatedWithCause {
    lane: ForegroundIoLaneKind,
    envelope: ForegroundLatencyEnvelope,
    counters: ForegroundReservationCounterSnapshot,
    cause: ForegroundReservationViolationCause,
}

impl ReservationViolatedWithCause {
    pub(crate) const fn new(
        lane: ForegroundIoLaneKind,
        envelope: ForegroundLatencyEnvelope,
        counters: ForegroundReservationCounterSnapshot,
        cause: ForegroundReservationViolationCause,
    ) -> Self {
        Self {
            lane,
            envelope,
            counters,
            cause,
        }
    }

    pub const fn lane(self) -> ForegroundIoLaneKind {
        self.lane
    }

    pub const fn envelope(self) -> ForegroundLatencyEnvelope {
        self.envelope
    }

    pub const fn counters(self) -> ForegroundReservationCounterSnapshot {
        self.counters
    }

    pub const fn cause(self) -> ForegroundReservationViolationCause {
        self.cause
    }
}
