use crate::{
    IoSchedulerBackendCapabilityAdmission, IoSchedulerS6ReadinessAdmission,
    IoSchedulerS6SecurityScopeAdmission,
};

use super::{
    ForegroundArbitrationDeclaration, ForegroundLaneDeclaration,
    ForegroundReservationCapacityAdmission,
};

#[derive(Debug)]
pub struct ForegroundReservationAdmissionRequest<'a> {
    lane: ForegroundLaneDeclaration,
    backend: &'a IoSchedulerBackendCapabilityAdmission,
    stable_readiness: &'a IoSchedulerS6ReadinessAdmission,
    security_scope: &'a IoSchedulerS6SecurityScopeAdmission,
    arbitration: ForegroundArbitrationDeclaration,
    capacity_admission: &'a ForegroundReservationCapacityAdmission,
}

impl<'a> ForegroundReservationAdmissionRequest<'a> {
    pub const fn new(
        lane: ForegroundLaneDeclaration,
        backend: &'a IoSchedulerBackendCapabilityAdmission,
        stable_readiness: &'a IoSchedulerS6ReadinessAdmission,
        security_scope: &'a IoSchedulerS6SecurityScopeAdmission,
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

    pub const fn stable_readiness(&self) -> &IoSchedulerS6ReadinessAdmission {
        self.stable_readiness
    }

    pub const fn security_scope(&self) -> &IoSchedulerS6SecurityScopeAdmission {
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

pub const fn reject_copied_s5_counters_as_foreground_reservation(
) -> Result<(), super::ForegroundReservationAdmissionDenial> {
    Err(super::ForegroundReservationAdmissionDenial::CopiedS5CountersCannotReserve)
}

pub const fn reject_copied_security_scope_fields_as_foreground_reservation(
) -> Result<(), super::ForegroundReservationAdmissionDenial> {
    Err(super::ForegroundReservationAdmissionDenial::CopiedSecurityScopeFieldsCannotReserve)
}

pub const fn reject_terminal_projection_as_foreground_reservation(
) -> Result<(), super::ForegroundReservationAdmissionDenial> {
    Err(super::ForegroundReservationAdmissionDenial::TerminalProjectionCannotReserve)
}
