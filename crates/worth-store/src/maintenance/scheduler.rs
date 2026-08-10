#![allow(dead_code)]

mod admission;
mod budgets;
mod classes;
mod declaration;
mod descriptor;
mod identities;
mod plans;
mod summaries;

pub use admission::{
    AdmittedMaintenanceWork, DiscoveredMaintenanceWork, MaintenanceResourceBudgetGrant,
    QuantumBudgetReceipt,
};
pub use budgets::{
    CpuBudgetUnits, ForegroundLatencyGuard, FreshnessWindow, IoBudgetUnits,
    MaintenanceDescriptorDemand, MaintenanceQuantum, MemoryBudgetUnits, PacingWindow,
    PlanGeneration, PublicationSlotBudget, SupersessionEpoch,
};
pub use classes::{
    BackgroundReservationFamily, ForegroundReservationFamily, MaintenanceDebtFamily,
    MaintenanceEscalationDecision, MaintenanceExecutionPosture, MaintenanceFailureKind,
    MaintenancePlanFamily, MaintenanceReservationFamily, MaintenanceWorkClass,
    TierWorkContainerClass,
};
pub use descriptor::MaintenanceWorkDescriptor;
pub(crate) use descriptor::MaintenanceWorkDescriptorBasis;
pub use identities::{
    LocalityScopeToken, MaintenanceCoalescingDecision, MaintenanceDebtPressureClass,
    MaintenanceEquivalenceKey, MaintenanceEscalationVerdict, MaintenanceLaneKey,
    MaintenanceLocalityScope, MaintenanceStarvationStatus, MaintenanceWorkIdentity,
};
pub use plans::{
    BackgroundPacedMaintenancePlan, CancelledMaintenanceWork, DeferredMaintenancePlan,
    EscalatedMaintenancePlan, ExecutingMaintenanceWork, ForegroundReservationWitness,
    ForegroundReservedMaintenancePlan, RecoveredMaintenanceDescriptor, ReservedMaintenanceWork,
    RestartMaintenanceAdmission, SupersededMaintenanceWitness,
};
pub use summaries::{
    MaintenanceDebtSummary, MaintenanceLocalitySummary, MaintenanceQueueSummary,
    MaintenanceReservationSummary, MaintenanceResourceBudgetSummary,
};
pub(crate) use summaries::{MaintenanceQueueSummaryBasis, MaintenanceResourceBudgetSummaryBasis};
