use worth_store_physical_backend::{BackendTargetProfile, CapabilityEvidenceClass};
use worth_store_security::StoreSecurityScopeIdentity;

use crate::IoSchedulerBackendCapabilityRequirement;

use super::{
    ForegroundArbitrationDeclaration, ForegroundFairnessClass, ForegroundIoLaneKind,
    ForegroundLatencyEnvelope, ForegroundReservationAdmissionDenial,
    ForegroundReservationCounterSnapshot, ForegroundReservationViolationCause,
    ReservationViolatedWithCause,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForegroundReservationState {
    ReservationAdmitted,
    ReservationHeld,
    ReservationAdmissionDenied,
    ReservationStaleRebindRequired,
    ReservationViolatedWithCause,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ForegroundReservationReceipt {
    state: ForegroundReservationState,
    lane: ForegroundIoLaneKind,
    backend_requirement: IoSchedulerBackendCapabilityRequirement,
    backend_profile: BackendTargetProfile,
    backend_evidence_class: CapabilityEvidenceClass,
    envelope: ForegroundLatencyEnvelope,
    arbitration: ForegroundArbitrationDeclaration,
    counters: ForegroundReservationCounterSnapshot,
    security_scope_identity: StoreSecurityScopeIdentity,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ForegroundReservationHeld {
    lane: ForegroundIoLaneKind,
    envelope: ForegroundLatencyEnvelope,
    counters: ForegroundReservationCounterSnapshot,
    reason: ForegroundReservationAdmissionDenial,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ForegroundReservationDenied {
    lane: ForegroundIoLaneKind,
    counters: ForegroundReservationCounterSnapshot,
    denial: ForegroundReservationAdmissionDenial,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ForegroundReservationStaleRebindRequired {
    lane: ForegroundIoLaneKind,
    counters: ForegroundReservationCounterSnapshot,
    denial: ForegroundReservationAdmissionDenial,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForegroundReservationAdmissionOutcome {
    Admitted(ForegroundReservationReceipt),
    Held(ForegroundReservationHeld),
    Denied(ForegroundReservationDenied),
    StaleRebindRequired(ForegroundReservationStaleRebindRequired),
}

impl ForegroundReservationReceipt {
    pub(crate) const fn admitted(
        lane: ForegroundIoLaneKind,
        backend_requirement: IoSchedulerBackendCapabilityRequirement,
        backend_profile: BackendTargetProfile,
        backend_evidence_class: CapabilityEvidenceClass,
        envelope: ForegroundLatencyEnvelope,
        arbitration: ForegroundArbitrationDeclaration,
        counters: ForegroundReservationCounterSnapshot,
        security_scope_identity: StoreSecurityScopeIdentity,
    ) -> Self {
        Self {
            state: ForegroundReservationState::ReservationAdmitted,
            lane,
            backend_requirement,
            backend_profile,
            backend_evidence_class,
            envelope,
            arbitration,
            counters,
            security_scope_identity,
        }
    }

    pub const fn state(self) -> ForegroundReservationState {
        self.state
    }

    pub const fn lane(self) -> ForegroundIoLaneKind {
        self.lane
    }

    pub const fn backend_requirement(self) -> IoSchedulerBackendCapabilityRequirement {
        self.backend_requirement
    }

    pub const fn backend_profile(self) -> BackendTargetProfile {
        self.backend_profile
    }

    pub const fn backend_evidence_class(self) -> CapabilityEvidenceClass {
        self.backend_evidence_class
    }

    pub const fn envelope(self) -> ForegroundLatencyEnvelope {
        self.envelope
    }

    pub const fn arbitration(self) -> ForegroundArbitrationDeclaration {
        self.arbitration
    }

    pub const fn fairness_class(self) -> ForegroundFairnessClass {
        self.arbitration.fairness_class()
    }

    pub const fn counters(self) -> ForegroundReservationCounterSnapshot {
        self.counters
    }

    pub const fn security_scope_identity(self) -> StoreSecurityScopeIdentity {
        self.security_scope_identity
    }

    pub const fn execution_ready(self) -> Self {
        self
    }

    pub const fn observe_interference(
        self,
        observed_interference_events: u64,
    ) -> Result<Self, ReservationViolatedWithCause> {
        match self.envelope.max_interference_events() {
            Some(allowed_interference_events)
                if observed_interference_events > allowed_interference_events =>
            {
                Err(ReservationViolatedWithCause::new(
                    self.lane,
                    self.envelope,
                    self.counters,
                    ForegroundReservationViolationCause::EnvelopeExceeded {
                        allowed_interference_events,
                        observed_interference_events,
                    },
                ))
            }
            _ => Ok(self),
        }
    }
}

impl ForegroundReservationHeld {
    pub(crate) const fn new(
        lane: ForegroundIoLaneKind,
        envelope: ForegroundLatencyEnvelope,
        counters: ForegroundReservationCounterSnapshot,
        reason: ForegroundReservationAdmissionDenial,
    ) -> Self {
        Self {
            lane,
            envelope,
            counters,
            reason,
        }
    }

    pub const fn state(self) -> ForegroundReservationState {
        ForegroundReservationState::ReservationHeld
    }

    pub const fn lane(self) -> ForegroundIoLaneKind {
        self.lane
    }

    pub const fn reason(self) -> ForegroundReservationAdmissionDenial {
        self.reason
    }
}

impl ForegroundReservationDenied {
    pub(crate) const fn new(
        lane: ForegroundIoLaneKind,
        counters: ForegroundReservationCounterSnapshot,
        denial: ForegroundReservationAdmissionDenial,
    ) -> Self {
        Self {
            lane,
            counters,
            denial,
        }
    }

    pub const fn state(self) -> ForegroundReservationState {
        ForegroundReservationState::ReservationAdmissionDenied
    }

    pub const fn lane(self) -> ForegroundIoLaneKind {
        self.lane
    }

    pub const fn denial(self) -> ForegroundReservationAdmissionDenial {
        self.denial
    }

    pub const fn counters(self) -> ForegroundReservationCounterSnapshot {
        self.counters
    }
}

impl ForegroundReservationStaleRebindRequired {
    pub(crate) const fn new(
        lane: ForegroundIoLaneKind,
        counters: ForegroundReservationCounterSnapshot,
        denial: ForegroundReservationAdmissionDenial,
    ) -> Self {
        Self {
            lane,
            counters,
            denial,
        }
    }

    pub const fn state(self) -> ForegroundReservationState {
        ForegroundReservationState::ReservationStaleRebindRequired
    }

    pub const fn lane(self) -> ForegroundIoLaneKind {
        self.lane
    }

    pub const fn counters(self) -> ForegroundReservationCounterSnapshot {
        self.counters
    }

    pub const fn denial(self) -> ForegroundReservationAdmissionDenial {
        self.denial
    }
}

impl ForegroundReservationAdmissionOutcome {
    pub fn into_result(
        self,
    ) -> Result<ForegroundReservationReceipt, ForegroundReservationAdmissionDenial> {
        match self {
            Self::Admitted(receipt) => Ok(receipt),
            Self::Held(held) => Err(held.reason()),
            Self::Denied(denied) => Err(denied.denial()),
            Self::StaleRebindRequired(stale) => Err(stale.denial()),
        }
    }

    pub const fn state(&self) -> ForegroundReservationState {
        match self {
            Self::Admitted(_) => ForegroundReservationState::ReservationAdmitted,
            Self::Held(_) => ForegroundReservationState::ReservationHeld,
            Self::Denied(_) => ForegroundReservationState::ReservationAdmissionDenied,
            Self::StaleRebindRequired(_) => {
                ForegroundReservationState::ReservationStaleRebindRequired
            }
        }
    }
}
