use forge_store_physical_backend::{BackendTargetProfile, CapabilityEvidenceClass};
use forge_store_security::StoreSecurityScopeIdentity;

use crate::foreground_reservation::{ForegroundIoLaneKind, ForegroundReservationReceipt};
use crate::{
    IoSchedulerBackendCapabilityAdmission, IoSchedulerBackendCapabilityRequirement,
    IoSchedulerIsolationAdmission, IoSchedulerIsolationCounterSnapshot,
};

use super::BackgroundIoPressureClass;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BackgroundPacingAdmissionBasis {
    class: BackgroundIoPressureClass,
    foreground_lane: ForegroundIoLaneKind,
    foreground_backend_requirement: IoSchedulerBackendCapabilityRequirement,
    foreground_backend_profile: BackendTargetProfile,
    foreground_backend_evidence_class: CapabilityEvidenceClass,
    backend_requirement: IoSchedulerBackendCapabilityRequirement,
    backend_profile: BackendTargetProfile,
    backend_evidence_class: CapabilityEvidenceClass,
    backend_security_scope_bound: bool,
    security_scope_identity: StoreSecurityScopeIdentity,
    readiness_counters: IoSchedulerIsolationCounterSnapshot,
}

impl BackgroundPacingAdmissionBasis {
    pub(crate) fn new(
        class: BackgroundIoPressureClass,
        foreground: &ForegroundReservationReceipt,
        backend: &IoSchedulerBackendCapabilityAdmission,
        readiness: &IoSchedulerIsolationAdmission,
        security_scope_identity: StoreSecurityScopeIdentity,
    ) -> Self {
        Self {
            class,
            foreground_lane: foreground.lane(),
            foreground_backend_requirement: foreground.backend_requirement(),
            foreground_backend_profile: foreground.backend_profile(),
            foreground_backend_evidence_class: foreground.backend_evidence_class(),
            backend_requirement: backend.requirement(),
            backend_profile: backend.profile(),
            backend_evidence_class: backend.evidence_class(),
            backend_security_scope_bound: backend.security_scope_bound(),
            security_scope_identity,
            readiness_counters: readiness.counters(),
        }
    }

    pub const fn class(self) -> BackgroundIoPressureClass {
        self.class
    }
    pub const fn foreground_lane(self) -> ForegroundIoLaneKind {
        self.foreground_lane
    }
    pub const fn foreground_backend_requirement(self) -> IoSchedulerBackendCapabilityRequirement {
        self.foreground_backend_requirement
    }
    pub const fn foreground_backend_profile(self) -> BackendTargetProfile {
        self.foreground_backend_profile
    }
    pub const fn foreground_backend_evidence_class(self) -> CapabilityEvidenceClass {
        self.foreground_backend_evidence_class
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
    pub const fn backend_security_scope_bound(self) -> bool {
        self.backend_security_scope_bound
    }
    pub const fn security_scope_identity(self) -> StoreSecurityScopeIdentity {
        self.security_scope_identity
    }
    pub const fn readiness_counters(self) -> IoSchedulerIsolationCounterSnapshot {
        self.readiness_counters
    }
}
