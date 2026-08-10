use serde::{Deserialize, Serialize};

use super::budgets::{
    CpuBudgetUnits, ForegroundLatencyGuard, IoBudgetUnits, MemoryBudgetUnits, PublicationSlotBudget,
};

use super::classes::{MaintenanceDebtFamily, MaintenanceReservationFamily};

use super::identities::{
    MaintenanceDebtPressureClass, MaintenanceEquivalenceKey, MaintenanceLaneKey,
    MaintenanceLocalityScope, MaintenanceStarvationStatus,
};

#[derive(Debug, Clone)]
pub(crate) struct MaintenanceQueueSummaryBasis {
    pub(crate) lane_key: MaintenanceLaneKey,
    pub(crate) admitted_count: u64,
    pub(crate) reserved_count: u64,
    pub(crate) deferred_count: u64,
    pub(crate) active_quantum_count: u64,
    pub(crate) coalesced_count: u64,
    pub(crate) cancelled_superseded_count: u64,
    pub(crate) equivalence_member_counts: std::collections::BTreeMap<String, u64>,
    pub(crate) equivalence_leader_identities: std::collections::BTreeMap<String, String>,
    pub(crate) max_supersession_epoch_by_equivalence: std::collections::BTreeMap<String, u64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MaintenanceQueueSummary {
    lane_key: MaintenanceLaneKey,
    admitted_count: u64,
    reserved_count: u64,
    deferred_count: u64,
    active_quantum_count: u64,
    coalesced_count: u64,
    cancelled_superseded_count: u64,
    equivalence_member_counts: std::collections::BTreeMap<String, u64>,
    equivalence_leader_identities: std::collections::BTreeMap<String, String>,
    max_supersession_epoch_by_equivalence: std::collections::BTreeMap<String, u64>,
}

impl MaintenanceQueueSummary {
    pub(crate) fn new(basis: MaintenanceQueueSummaryBasis) -> Self {
        Self {
            lane_key: basis.lane_key,
            admitted_count: basis.admitted_count,
            reserved_count: basis.reserved_count,
            deferred_count: basis.deferred_count,
            active_quantum_count: basis.active_quantum_count,
            coalesced_count: basis.coalesced_count,
            cancelled_superseded_count: basis.cancelled_superseded_count,
            equivalence_member_counts: basis.equivalence_member_counts,
            equivalence_leader_identities: basis.equivalence_leader_identities,
            max_supersession_epoch_by_equivalence: basis.max_supersession_epoch_by_equivalence,
        }
    }

    pub fn lane_key(&self) -> &MaintenanceLaneKey {
        &self.lane_key
    }

    pub fn admitted_count(&self) -> u64 {
        self.admitted_count
    }

    pub fn reserved_count(&self) -> u64 {
        self.reserved_count
    }

    pub fn deferred_count(&self) -> u64 {
        self.deferred_count
    }

    pub fn active_quantum_count(&self) -> u64 {
        self.active_quantum_count
    }

    pub fn coalesced_count(&self) -> u64 {
        self.coalesced_count
    }

    pub fn cancelled_superseded_count(&self) -> u64 {
        self.cancelled_superseded_count
    }

    pub fn equivalence_member_count(&self, key: &MaintenanceEquivalenceKey) -> u64 {
        self.equivalence_member_counts
            .get(key.as_str())
            .copied()
            .unwrap_or(0)
    }

    pub fn leader_identity_for(&self, key: &MaintenanceEquivalenceKey) -> Option<&str> {
        self.equivalence_leader_identities
            .get(key.as_str())
            .map(String::as_str)
    }

    pub fn max_supersession_epoch_for(&self, key: &MaintenanceEquivalenceKey) -> Option<u64> {
        self.max_supersession_epoch_by_equivalence
            .get(key.as_str())
            .copied()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MaintenanceLocalitySummary {
    locality_scope: MaintenanceLocalityScope,
    lane_count: u64,
    admitted_count: u64,
    deferred_count: u64,
    active_count: u64,
}

impl MaintenanceLocalitySummary {
    pub(crate) fn new(
        locality_scope: MaintenanceLocalityScope,
        lane_count: u64,
        admitted_count: u64,
        deferred_count: u64,
        active_count: u64,
    ) -> Self {
        Self {
            locality_scope,
            lane_count,
            admitted_count,
            deferred_count,
            active_count,
        }
    }

    pub fn locality_scope(&self) -> &MaintenanceLocalityScope {
        &self.locality_scope
    }

    pub fn lane_count(&self) -> u64 {
        self.lane_count
    }

    pub fn admitted_count(&self) -> u64 {
        self.admitted_count
    }

    pub fn deferred_count(&self) -> u64 {
        self.deferred_count
    }

    pub fn active_count(&self) -> u64 {
        self.active_count
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MaintenanceReservationSummary {
    reservation_family: MaintenanceReservationFamily,
    lane_count: u64,
    reserved_count: u64,
    deferred_count: u64,
}

impl MaintenanceReservationSummary {
    pub(crate) fn new(
        reservation_family: MaintenanceReservationFamily,
        lane_count: u64,
        reserved_count: u64,
        deferred_count: u64,
    ) -> Self {
        Self {
            reservation_family,
            lane_count,
            reserved_count,
            deferred_count,
        }
    }

    pub fn reservation_family(&self) -> MaintenanceReservationFamily {
        self.reservation_family
    }

    pub fn lane_count(&self) -> u64 {
        self.lane_count
    }

    pub fn reserved_count(&self) -> u64 {
        self.reserved_count
    }

    pub fn deferred_count(&self) -> u64 {
        self.deferred_count
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MaintenanceResourceBudgetSummary {
    available_io: IoBudgetUnits,
    reserved_io: IoBudgetUnits,
    available_cpu: CpuBudgetUnits,
    reserved_cpu: CpuBudgetUnits,
    available_memory: MemoryBudgetUnits,
    reserved_memory: MemoryBudgetUnits,
    available_publication: PublicationSlotBudget,
    reserved_publication: PublicationSlotBudget,
    available_foreground_latency_guard: ForegroundLatencyGuard,
    reserved_foreground_latency_guard: ForegroundLatencyGuard,
}

#[derive(Debug, Clone)]
pub(crate) struct MaintenanceResourceBudgetSummaryBasis {
    pub(crate) available_io: IoBudgetUnits,
    pub(crate) reserved_io: IoBudgetUnits,
    pub(crate) available_cpu: CpuBudgetUnits,
    pub(crate) reserved_cpu: CpuBudgetUnits,
    pub(crate) available_memory: MemoryBudgetUnits,
    pub(crate) reserved_memory: MemoryBudgetUnits,
    pub(crate) available_publication: PublicationSlotBudget,
    pub(crate) reserved_publication: PublicationSlotBudget,
    pub(crate) available_foreground_latency_guard: ForegroundLatencyGuard,
    pub(crate) reserved_foreground_latency_guard: ForegroundLatencyGuard,
}

impl MaintenanceResourceBudgetSummary {
    pub(crate) fn new(basis: MaintenanceResourceBudgetSummaryBasis) -> Self {
        Self {
            available_io: basis.available_io,
            reserved_io: basis.reserved_io,
            available_cpu: basis.available_cpu,
            reserved_cpu: basis.reserved_cpu,
            available_memory: basis.available_memory,
            reserved_memory: basis.reserved_memory,
            available_publication: basis.available_publication,
            reserved_publication: basis.reserved_publication,
            available_foreground_latency_guard: basis.available_foreground_latency_guard,
            reserved_foreground_latency_guard: basis.reserved_foreground_latency_guard,
        }
    }

    pub fn available_io(&self) -> IoBudgetUnits {
        self.available_io
    }

    pub fn reserved_io(&self) -> IoBudgetUnits {
        self.reserved_io
    }

    pub fn available_cpu(&self) -> CpuBudgetUnits {
        self.available_cpu
    }

    pub fn reserved_cpu(&self) -> CpuBudgetUnits {
        self.reserved_cpu
    }

    pub fn available_memory(&self) -> MemoryBudgetUnits {
        self.available_memory
    }

    pub fn reserved_memory(&self) -> MemoryBudgetUnits {
        self.reserved_memory
    }

    pub fn available_publication(&self) -> PublicationSlotBudget {
        self.available_publication
    }

    pub fn reserved_publication(&self) -> PublicationSlotBudget {
        self.reserved_publication
    }

    pub fn available_foreground_latency_guard(&self) -> ForegroundLatencyGuard {
        self.available_foreground_latency_guard
    }

    pub fn reserved_foreground_latency_guard(&self) -> ForegroundLatencyGuard {
        self.reserved_foreground_latency_guard
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MaintenanceDebtSummary {
    debt_family: Option<MaintenanceDebtFamily>,
    locality_scope: MaintenanceLocalityScope,
    pressure_class: MaintenanceDebtPressureClass,
    starvation_status: MaintenanceStarvationStatus,
    explicit_global_scope_debt: bool,
}

impl MaintenanceDebtSummary {
    pub(crate) fn new(
        debt_family: Option<MaintenanceDebtFamily>,
        locality_scope: MaintenanceLocalityScope,
        pressure_class: MaintenanceDebtPressureClass,
        starvation_status: MaintenanceStarvationStatus,
        explicit_global_scope_debt: bool,
    ) -> Self {
        Self {
            debt_family,
            locality_scope,
            pressure_class,
            starvation_status,
            explicit_global_scope_debt,
        }
    }

    pub fn pressure_class(&self) -> MaintenanceDebtPressureClass {
        self.pressure_class
    }

    pub fn starvation_status(&self) -> MaintenanceStarvationStatus {
        self.starvation_status
    }

    pub fn explicit_global_scope_debt(&self) -> bool {
        self.explicit_global_scope_debt
    }
}
