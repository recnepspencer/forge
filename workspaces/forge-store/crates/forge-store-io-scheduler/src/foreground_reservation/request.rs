use crate::{
    IoSchedulerBackendCapabilityAdmission, IoSchedulerIsolationAdmission,
    IoSchedulerSecurityScopeAdmission,
};

use super::{
    ForegroundArbitrationDeclaration, ForegroundLaneDeclaration,
    ForegroundReservationCapacityAdmission,
};

#[derive(Debug)]
pub struct ForegroundReservationAdmissionRequest<'a> {
    lane: ForegroundLaneDeclaration,
    backend: &'a IoSchedulerBackendCapabilityAdmission,
    stable_readiness: &'a IoSchedulerIsolationAdmission,
    security_scope: &'a IoSchedulerSecurityScopeAdmission,
    arbitration: ForegroundArbitrationDeclaration,
    capacity_admission: &'a ForegroundReservationCapacityAdmission,
}

impl<'a> ForegroundReservationAdmissionRequest<'a> {
    pub const fn new(
        lane: ForegroundLaneDeclaration,
        backend: &'a IoSchedulerBackendCapabilityAdmission,
        stable_readiness: &'a IoSchedulerIsolationAdmission,
        security_scope: &'a IoSchedulerSecurityScopeAdmission,
        arbitration: ForegroundArbitrationDeclaration,
        capacity_admission: &'a ForegroundReservationCapacityAdmission,
    ) -> Self {
        Self {
            lane,
            backend,
            stable_readiness,
            security_scope,
            arbitration,
            capacity_admission,
        }
    }

    pub const fn lane(&self) -> ForegroundLaneDeclaration {
        self.lane
    }

    pub const fn backend(&self) -> &IoSchedulerBackendCapabilityAdmission {
        self.backend
    }

    pub const fn stable_readiness(&self) -> &IoSchedulerIsolationAdmission {
        self.stable_readiness
    }

    pub const fn security_scope(&self) -> &IoSchedulerSecurityScopeAdmission {
        self.security_scope
    }

    pub const fn arbitration(&self) -> ForegroundArbitrationDeclaration {
        self.arbitration
    }

    pub const fn capacity_admission(&self) -> &ForegroundReservationCapacityAdmission {
        self.capacity_admission
    }
}

pub const fn reject_raw_lane_label_as_foreground_reservation(
) -> Result<(), super::ForegroundReservationAdmissionDenial> {
    Err(super::ForegroundReservationAdmissionDenial::RawLaneLabelCannotReserve)
}

pub const fn reject_semantic_priority_as_foreground_reservation(
) -> Result<(), super::ForegroundReservationAdmissionDenial> {
    Err(super::ForegroundReservationAdmissionDenial::SemanticPriorityCannotReserve)
}

pub const fn reject_copied_physical_isolation_counters_as_foreground_reservation(
) -> Result<(), super::ForegroundReservationAdmissionDenial> {
    Err(super::ForegroundReservationAdmissionDenial::CopiedIsolationCountersCannotReserve)
}

pub const fn reject_copied_security_scope_fields_as_foreground_reservation(
) -> Result<(), super::ForegroundReservationAdmissionDenial> {
    Err(super::ForegroundReservationAdmissionDenial::CopiedSecurityScopeFieldsCannotReserve)
}

pub const fn reject_terminal_projection_as_foreground_reservation(
) -> Result<(), super::ForegroundReservationAdmissionDenial> {
    Err(super::ForegroundReservationAdmissionDenial::TerminalProjectionCannotReserve)
}
