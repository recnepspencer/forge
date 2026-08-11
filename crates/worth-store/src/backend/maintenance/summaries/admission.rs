use std::collections::BTreeMap;

use crate::{
    backend::records::StoreState,
    maintenance::{
        CpuBudgetUnits, IoBudgetUnits, MaintenanceDebtPressureClass, MaintenanceDebtSummary,
        MaintenanceDescriptorDemand, MaintenanceLaneKey, MaintenanceQueueSummary,
        MaintenanceQueueSummaryBasis, MaintenanceResourceBudgetSummary,
        MaintenanceResourceBudgetSummaryBasis, MaintenanceStarvationStatus, MemoryBudgetUnits,
        PublicationSlotBudget,
    },
};

const AVAILABLE_IO_UNITS: u64 = 8;
const AVAILABLE_CPU_UNITS: u64 = 6;
const AVAILABLE_MEMORY_UNITS: u64 = 4;
const AVAILABLE_PUBLICATION_UNITS: u64 = 2;
const AVAILABLE_FOREGROUND_LATENCY_GUARD_UNITS: u64 = 2;

#[derive(Debug, Clone)]
pub(crate) struct SchedulerAdmissionContext {
    pub(crate) lane_summary: MaintenanceQueueSummary,
    pub(crate) resource_budget_summary: MaintenanceResourceBudgetSummary,
    pub(crate) debt_summary: MaintenanceDebtSummary,
}

pub(crate) fn scheduler_admission_context(
    state: &StoreState,
    lane_key: &MaintenanceLaneKey,
) -> SchedulerAdmissionContext {
    SchedulerAdmissionContext {
        lane_summary: state
            .maintenance_queue_summary_records
            .get(&lane_key.artifact_id())
            .map(|record| record.summary.clone())
            .unwrap_or_else(|| empty_lane_summary(lane_key.clone())),
        resource_budget_summary: state
            .maintenance_resource_budget_summary_records
            .get("maintenance-resource-budget")
            .map(|record| record.summary.clone())
            .unwrap_or_else(default_resource_budget_summary),
        debt_summary: state
            .maintenance_debt_summary_records
            .get(&lane_key.artifact_id())
            .map(|record| record.summary.clone())
            .unwrap_or_else(|| {
                MaintenanceDebtSummary::new(
                    None,
                    lane_key.locality_scope().clone(),
                    MaintenanceDebtPressureClass::None,
                    MaintenanceStarvationStatus::NotStarved,
                    false,
                )
            }),
    }
}

pub(crate) fn default_resource_budget_summary() -> MaintenanceResourceBudgetSummary {
    MaintenanceResourceBudgetSummary::new(MaintenanceResourceBudgetSummaryBasis {
        available_io: IoBudgetUnits::new(AVAILABLE_IO_UNITS),
        reserved_io: IoBudgetUnits::new(0),
        available_cpu: CpuBudgetUnits::new(AVAILABLE_CPU_UNITS),
        reserved_cpu: CpuBudgetUnits::new(0),
        available_memory: MemoryBudgetUnits::new(AVAILABLE_MEMORY_UNITS),
        reserved_memory: MemoryBudgetUnits::new(0),
        available_publication: PublicationSlotBudget::new(AVAILABLE_PUBLICATION_UNITS),
        reserved_publication: PublicationSlotBudget::new(0),
        available_foreground_latency_guard: crate::ForegroundLatencyGuard::new(
            AVAILABLE_FOREGROUND_LATENCY_GUARD_UNITS,
        ),
        reserved_foreground_latency_guard: crate::ForegroundLatencyGuard::new(0),
    })
}

pub(crate) fn budget_fits(
    demand: &MaintenanceDescriptorDemand,
    summary: &MaintenanceResourceBudgetSummary,
) -> bool {
    demand.predicted_io().units() + summary.reserved_io().units() <= summary.available_io().units()
        && demand.predicted_cpu().units() + summary.reserved_cpu().units()
            <= summary.available_cpu().units()
        && demand.predicted_memory().units() + summary.reserved_memory().units()
            <= summary.available_memory().units()
        && demand.predicted_publication().units() + summary.reserved_publication().units()
            <= summary.available_publication().units()
        && demand.foreground_latency_guard().units()
            + summary.reserved_foreground_latency_guard().units()
            <= summary.available_foreground_latency_guard().units()
}

pub(super) fn empty_lane_summary(lane_key: MaintenanceLaneKey) -> MaintenanceQueueSummary {
    MaintenanceQueueSummary::new(MaintenanceQueueSummaryBasis {
        lane_key,
        admitted_count: 0,
        reserved_count: 0,
        deferred_count: 0,
        active_quantum_count: 0,
        coalesced_count: 0,
        cancelled_superseded_count: 0,
        equivalence_member_counts: BTreeMap::new(),
        equivalence_leader_identities: BTreeMap::new(),
        max_supersession_epoch_by_equivalence: BTreeMap::new(),
    })
}
