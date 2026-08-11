use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MaintenanceExecutionStatus {
    Declared,
    Admitted,
    Reserved,
    Deferred,
    Started,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MaintenanceReadmissionStatus {
    PendingRecoveredReadmission,
    ReadmittedRecoveredWork,
    RejectedStaleRecoveredWork,
    RejectedSupersededRecoveredWork,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MaintenanceForegroundImpact {
    borrowed_foreground_reservation: bool,
    foreground_wait_required: bool,
    cutover_dependency_required: bool,
}

impl MaintenanceForegroundImpact {
    pub const fn none() -> Self {
        Self {
            borrowed_foreground_reservation: false,
            foreground_wait_required: false,
            cutover_dependency_required: false,
        }
    }

    pub(crate) fn escalated() -> Self {
        Self {
            borrowed_foreground_reservation: true,
            foreground_wait_required: true,
            cutover_dependency_required: true,
        }
    }

    pub fn borrowed_foreground_reservation(&self) -> bool {
        self.borrowed_foreground_reservation
    }

    pub fn foreground_wait_required(&self) -> bool {
        self.foreground_wait_required
    }

    pub fn cutover_dependency_required(&self) -> bool {
        self.cutover_dependency_required
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ForegroundReservationClass {
    Read,
    Write,
    Continuation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ForegroundInterferencePosture {
    StayedIsolated,
    ObservedMaintenanceNoWait,
    WaitedOnMaintenance,
    BroadenedByMaintenance,
    ReservationViolation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ForegroundWaitDependency {
    MaintenanceReservationRelease,
    MaintenancePublication,
    MaintenanceCutover,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ForegroundBroadeningCause {
    MaintenanceBlockedIsolatedPath,
    GlobalDebtPromotion,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ForegroundIsolationViolation {
    SharedReservationConflict,
    IllegalForegroundBorrow,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ForegroundIsolationOutcome {
    reservation_class: ForegroundReservationClass,
    posture: ForegroundInterferencePosture,
    wait_dependency: Option<ForegroundWaitDependency>,
    broadening_cause: Option<ForegroundBroadeningCause>,
    violation: Option<ForegroundIsolationViolation>,
}

impl ForegroundIsolationOutcome {
    pub fn stayed_isolated(reservation_class: ForegroundReservationClass) -> Self {
        Self {
            reservation_class,
            posture: ForegroundInterferencePosture::StayedIsolated,
            wait_dependency: None,
            broadening_cause: None,
            violation: None,
        }
    }

    pub fn observed_maintenance(reservation_class: ForegroundReservationClass) -> Self {
        Self {
            reservation_class,
            posture: ForegroundInterferencePosture::ObservedMaintenanceNoWait,
            wait_dependency: None,
            broadening_cause: None,
            violation: None,
        }
    }

    pub fn waited(
        reservation_class: ForegroundReservationClass,
        wait_dependency: ForegroundWaitDependency,
    ) -> Self {
        Self {
            reservation_class,
            posture: ForegroundInterferencePosture::WaitedOnMaintenance,
            wait_dependency: Some(wait_dependency),
            broadening_cause: None,
            violation: None,
        }
    }

    pub fn broadened(
        reservation_class: ForegroundReservationClass,
        broadening_cause: ForegroundBroadeningCause,
    ) -> Self {
        Self {
            reservation_class,
            posture: ForegroundInterferencePosture::BroadenedByMaintenance,
            wait_dependency: None,
            broadening_cause: Some(broadening_cause),
            violation: None,
        }
    }

    pub fn violated(
        reservation_class: ForegroundReservationClass,
        violation: ForegroundIsolationViolation,
    ) -> Self {
        Self {
            reservation_class,
            posture: ForegroundInterferencePosture::ReservationViolation,
            wait_dependency: None,
            broadening_cause: None,
            violation: Some(violation),
        }
    }

    pub fn reservation_class(&self) -> ForegroundReservationClass {
        self.reservation_class
    }

    pub fn posture(&self) -> ForegroundInterferencePosture {
        self.posture
    }

    pub fn wait_dependency(&self) -> Option<ForegroundWaitDependency> {
        self.wait_dependency
    }

    pub fn broadening_cause(&self) -> Option<ForegroundBroadeningCause> {
        self.broadening_cause
    }

    pub fn violation(&self) -> Option<ForegroundIsolationViolation> {
        self.violation
    }

    pub fn maintenance_interference(&self) -> bool {
        !matches!(self.posture, ForegroundInterferencePosture::StayedIsolated)
    }
}
