use crate::{
    backend::records::{MaintenanceResourceBudgetSummaryRecord, StoreState},
    maintenance::{
        CpuBudgetUnits, IoBudgetUnits, MaintenanceResourceBudgetSummary,
        MaintenanceResourceBudgetSummaryBasis, MemoryBudgetUnits, PublicationSlotBudget,
    },
};

use super::budget_accumulation::ReservedResourceTotals;

const AVAILABLE_IO_UNITS: u64 = 8;
const AVAILABLE_CPU_UNITS: u64 = 6;
const AVAILABLE_MEMORY_UNITS: u64 = 4;
const AVAILABLE_PUBLICATION_UNITS: u64 = 2;
const AVAILABLE_FOREGROUND_LATENCY_GUARD_UNITS: u64 = 2;

pub(super) fn publish_resource_budget_summary(
    state: &mut StoreState,
    totals: ReservedResourceTotals,
) {
    let artifact_id = "maintenance-resource-budget".to_string();
    state.maintenance_resource_budget_summary_records.insert(
        artifact_id.clone(),
        MaintenanceResourceBudgetSummaryRecord {
            artifact_id,
            family_version: 1,
            summary: MaintenanceResourceBudgetSummary::new(MaintenanceResourceBudgetSummaryBasis {
                available_io: IoBudgetUnits::new(AVAILABLE_IO_UNITS),
                reserved_io: IoBudgetUnits::new(totals.io),
                available_cpu: CpuBudgetUnits::new(AVAILABLE_CPU_UNITS),
                reserved_cpu: CpuBudgetUnits::new(totals.cpu),
                available_memory: MemoryBudgetUnits::new(AVAILABLE_MEMORY_UNITS),
                reserved_memory: MemoryBudgetUnits::new(totals.memory),
                available_publication: PublicationSlotBudget::new(AVAILABLE_PUBLICATION_UNITS),
                reserved_publication: PublicationSlotBudget::new(totals.publication),
                available_foreground_latency_guard: crate::ForegroundLatencyGuard::new(
                    AVAILABLE_FOREGROUND_LATENCY_GUARD_UNITS,
                ),
                reserved_foreground_latency_guard: crate::ForegroundLatencyGuard::new(
                    totals.foreground_latency_guard,
                ),
            }),
        },
    );
}
