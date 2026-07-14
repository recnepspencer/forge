use worth_store_io_scheduler::foreground_reservation::{
    ForegroundIoLaneKind, ForegroundLatencyEnvelope, ForegroundReservationCounterSnapshot,
    ForegroundReservationReceipt, ForegroundReservationState,
};
use worth_store_physical_backend::{BackendTargetProfile, CapabilityEvidenceClass};
use worth_store_security::StoreSecurityScopeIdentity;

use worth_store_io_scheduler::IoSchedulerBackendCapabilityRequirement;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum S6ForegroundReservationCertificationDenial {
    ReceiptMismatch,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct S6ForegroundReservationCertificationEvidence {
    state: ForegroundReservationState,
    lane: ForegroundIoLaneKind,
    backend_requirement: IoSchedulerBackendCapabilityRequirement,
    backend_profile: BackendTargetProfile,
    backend_evidence_class: CapabilityEvidenceClass,
    envelope: ForegroundLatencyEnvelope,
    counters: ForegroundReservationCounterSnapshot,
    security_scope_identity: StoreSecurityScopeIdentity,
}

impl S6ForegroundReservationCertificationEvidence {
    pub fn from_reservation_receipt(
        receipt: ForegroundReservationReceipt,
        expected: ForegroundReservationReceipt,
    ) -> Result<Self, S6ForegroundReservationCertificationDenial> {
        if receipt != expected {
            return Err(S6ForegroundReservationCertificationDenial::ReceiptMismatch);
        }
        Ok(Self {
            state: receipt.state(),
            lane: receipt.lane(),
            backend_requirement: receipt.backend_requirement(),
            backend_profile: receipt.backend_profile(),
            backend_evidence_class: receipt.backend_evidence_class(),
            envelope: receipt.envelope(),
            counters: receipt.counters(),
            security_scope_identity: receipt.security_scope_identity(),
        })
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

    pub const fn counters(self) -> ForegroundReservationCounterSnapshot {
        self.counters
    }

    pub const fn security_scope_identity(self) -> StoreSecurityScopeIdentity {
        self.security_scope_identity
    }
}

pub fn certify_io_qos_foreground_reservation(
    receipt: ForegroundReservationReceipt,
    expected: ForegroundReservationReceipt,
) -> Result<S6ForegroundReservationCertificationEvidence, S6ForegroundReservationCertificationDenial>
{
    S6ForegroundReservationCertificationEvidence::from_reservation_receipt(receipt, expected)
}
