use serde::{Deserialize, Serialize};

use super::super::MaintenancePlanFamily;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MaintenanceReservationTransition {
    plan_family: MaintenancePlanFamily,
    quantum_units: u64,
}

impl MaintenanceReservationTransition {
    pub(crate) fn new(plan_family: MaintenancePlanFamily, quantum_units: u64) -> Self {
        Self {
            plan_family,
            quantum_units,
        }
    }

    pub fn plan_family(&self) -> MaintenancePlanFamily {
        self.plan_family
    }

    pub fn quantum_units(&self) -> u64 {
        self.quantum_units
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MaintenanceExecutionTransition {
    resumed_from_started: bool,
    quantum_units: Option<u64>,
}

impl MaintenanceExecutionTransition {
    pub(crate) fn new(resumed_from_started: bool, quantum_units: Option<u64>) -> Self {
        Self {
            resumed_from_started,
            quantum_units,
        }
    }

    pub fn resumed_from_started(&self) -> bool {
        self.resumed_from_started
    }

    pub fn quantum_units(&self) -> Option<u64> {
        self.quantum_units
    }
}
