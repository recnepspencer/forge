use crate::{
    CpuBudgetUnits, ForegroundLatencyGuard, IoBudgetUnits, MemoryBudgetUnits, PublicationSlotBudget,
};

pub(super) fn fixture_budget_grant(quantum_units: u64) -> crate::MaintenanceResourceBudgetGrant {
    crate::MaintenanceResourceBudgetGrant::new(
        IoBudgetUnits::new(1),
        CpuBudgetUnits::new(1),
        MemoryBudgetUnits::new(1),
        PublicationSlotBudget::new(1),
        ForegroundLatencyGuard::new(1),
        crate::MaintenanceQuantum::new(quantum_units),
        crate::PacingWindow::new(quantum_units.max(1)),
    )
}
