use serde::{Deserialize, Serialize};

use super::super::{MaintenanceDeclaration, MaintenanceWorkDescriptor};

use super::budgets::{
    CpuBudgetUnits, ForegroundLatencyGuard, IoBudgetUnits, MaintenanceQuantum, MemoryBudgetUnits,
    PacingWindow, PublicationSlotBudget,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DiscoveredMaintenanceWork {
    declaration: MaintenanceDeclaration,
    descriptor: MaintenanceWorkDescriptor,
}

impl DiscoveredMaintenanceWork {
    pub(crate) fn new(
        declaration: MaintenanceDeclaration,
        descriptor: MaintenanceWorkDescriptor,
    ) -> Self {
        Self {
            declaration,
            descriptor,
        }
    }

    pub fn declaration(&self) -> &MaintenanceDeclaration {
        &self.declaration
    }

    pub fn descriptor(&self) -> &MaintenanceWorkDescriptor {
        &self.descriptor
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AdmittedMaintenanceWork {
    declaration: MaintenanceDeclaration,
    descriptor: MaintenanceWorkDescriptor,
}

impl AdmittedMaintenanceWork {
    pub(crate) fn new(
        declaration: MaintenanceDeclaration,
        descriptor: MaintenanceWorkDescriptor,
    ) -> Self {
        Self {
            declaration,
            descriptor,
        }
    }

    pub fn declaration(&self) -> &MaintenanceDeclaration {
        &self.declaration
    }

    pub fn descriptor(&self) -> &MaintenanceWorkDescriptor {
        &self.descriptor
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct QuantumBudgetReceipt {
    maintenance_quantum: MaintenanceQuantum,
    pacing_window: PacingWindow,
}

impl QuantumBudgetReceipt {
    pub(crate) fn new(
        maintenance_quantum: MaintenanceQuantum,
        pacing_window: PacingWindow,
    ) -> Self {
        Self {
            maintenance_quantum,
            pacing_window,
        }
    }

    pub fn maintenance_quantum(&self) -> MaintenanceQuantum {
        self.maintenance_quantum
    }

    pub fn pacing_window(&self) -> PacingWindow {
        self.pacing_window
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MaintenanceResourceBudgetGrant {
    granted_io: IoBudgetUnits,
    granted_cpu: CpuBudgetUnits,
    granted_memory: MemoryBudgetUnits,
    granted_publication: PublicationSlotBudget,
    granted_foreground_latency_guard: ForegroundLatencyGuard,
    maintenance_quantum: MaintenanceQuantum,
    pacing_window: PacingWindow,
}

impl MaintenanceResourceBudgetGrant {
    pub(crate) fn new(
        granted_io: IoBudgetUnits,
        granted_cpu: CpuBudgetUnits,
        granted_memory: MemoryBudgetUnits,
        granted_publication: PublicationSlotBudget,
        granted_foreground_latency_guard: ForegroundLatencyGuard,
        maintenance_quantum: MaintenanceQuantum,
        pacing_window: PacingWindow,
    ) -> Self {
        Self {
            granted_io,
            granted_cpu,
            granted_memory,
            granted_publication,
            granted_foreground_latency_guard,
            maintenance_quantum,
            pacing_window,
        }
    }

    pub fn granted_io(&self) -> IoBudgetUnits {
        self.granted_io
    }

    pub fn granted_cpu(&self) -> CpuBudgetUnits {
        self.granted_cpu
    }

    pub fn granted_memory(&self) -> MemoryBudgetUnits {
        self.granted_memory
    }

    pub fn granted_publication(&self) -> PublicationSlotBudget {
        self.granted_publication
    }

    pub fn granted_foreground_latency_guard(&self) -> ForegroundLatencyGuard {
        self.granted_foreground_latency_guard
    }

    pub fn maintenance_quantum(&self) -> MaintenanceQuantum {
        self.maintenance_quantum
    }

    pub fn pacing_window(&self) -> PacingWindow {
        self.pacing_window
    }

    pub fn into_quantum_budget_receipt(self) -> QuantumBudgetReceipt {
        QuantumBudgetReceipt::new(self.maintenance_quantum, self.pacing_window)
    }
}
