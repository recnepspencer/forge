use super::{OperationalExecutionPolicy, OperationalSessionRecoveryHandle};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperationalInterruptionReason {
    Cancelled,
    DeadlineReached,
    ClientDisconnected,
    ProcessCrashed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OperationalSessionInterruption {
    reason: OperationalInterruptionReason,
    recovery: OperationalSessionRecoveryHandle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperationalSessionAdmissionDenial {
    DeadlineAlreadyReached,
    ResidentBudgetExceeded,
    InFlightIoBudgetExceeded,
}

impl OperationalSessionInterruption {
    /// Interruption is representable only with a durable recovery handle. A
    /// dropped live task therefore cannot masquerade as governed cancellation.
    pub const fn from_durable_recovery(
        reason: OperationalInterruptionReason,
        recovery: OperationalSessionRecoveryHandle,
    ) -> Self {
        Self { reason, recovery }
    }

    pub const fn reason(self) -> OperationalInterruptionReason {
        self.reason
    }
    pub const fn recovery(self) -> OperationalSessionRecoveryHandle {
        self.recovery
    }
}

pub fn admit_operational_session(
    policy: OperationalExecutionPolicy,
    now_tick: u64,
    required_resident_bytes: u64,
    required_in_flight_io: u64,
) -> Result<(), OperationalSessionAdmissionDenial> {
    if policy
        .deadline_tick()
        .is_some_and(|deadline| now_tick >= deadline)
    {
        return Err(OperationalSessionAdmissionDenial::DeadlineAlreadyReached);
    }
    if required_resident_bytes > policy.maximum_resident_bytes() {
        return Err(OperationalSessionAdmissionDenial::ResidentBudgetExceeded);
    }
    if required_in_flight_io > policy.maximum_in_flight_io() {
        return Err(OperationalSessionAdmissionDenial::InFlightIoBudgetExceeded);
    }
    Ok(())
}
