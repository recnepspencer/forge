use std::collections::BTreeMap;

use crate::{
    backend::records::{
        MaintenanceDebtSummaryRecord, MaintenanceLocalitySummaryRecord,
        MaintenanceQueueSummaryRecord, MaintenanceReservationSummaryRecord,
        MaintenanceResourceBudgetSummaryRecord, StoreState,
    },
    maintenance::{
        CpuBudgetUnits, IoBudgetUnits, MaintenanceCoalescingDecision, MaintenanceDebtPressureClass,
        MaintenanceDebtSummary, MaintenanceDescriptorDemand, MaintenanceExecutionStatus,
        MaintenanceLaneKey, MaintenanceLocalityScope, MaintenanceLocalitySummary,
        MaintenanceQueueSummary, MaintenanceReservationFamily, MaintenanceReservationSummary,
        MaintenanceResourceBudgetSummary, MaintenanceStarvationStatus, MemoryBudgetUnits,
        PublicationSlotBudget,
    },
};

const AVAILABLE_IO_UNITS: u64 = 8;
const AVAILABLE_CPU_UNITS: u64 = 6;
const AVAILABLE_MEMORY_UNITS: u64 = 4;
const AVAILABLE_PUBLICATION_UNITS: u64 = 2;
const AVAILABLE_FOREGROUND_LATENCY_GUARD_UNITS: u64 = 2;
const STARVATION_DEFERRED_THRESHOLD: u64 = 2;

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

pub(crate) fn refresh_scheduler_summaries(state: &mut StoreState) {
    state.maintenance_queue_summary_records.clear();
    state.maintenance_locality_summary_records.clear();
    state.maintenance_reservation_summary_records.clear();
    state.maintenance_resource_budget_summary_records.clear();
    state.maintenance_debt_summary_records.clear();

    if state.maintenance_declaration_records.is_empty()
        && state.maintenance_execution_records.is_empty()
    {
        return;
    }

    let mut lane_accumulators = BTreeMap::<String, LaneAccumulator>::new();

    for declaration in state.maintenance_declaration_records.values() {
        let lane_key = declaration.work_descriptor.lane_key();
        let lane_id = lane_key.artifact_id();
        let execution = state
            .maintenance_execution_records
            .get(declaration.artifact_id.as_str());
        let accumulator = lane_accumulators
            .entry(lane_id)
            .or_insert_with(|| LaneAccumulator::new(lane_key.clone()));
        accumulator.admitted_count += 1;

        let equivalence_key = declaration
            .work_descriptor
            .equivalence_key()
            .as_str()
            .to_string();
        *accumulator
            .equivalence_member_counts
            .entry(equivalence_key.clone())
            .or_default() += 1;
        accumulator
            .max_supersession_epoch_by_equivalence
            .entry(equivalence_key.clone())
            .and_modify(|current| {
                *current = (*current).max(declaration.work_descriptor.supersession_epoch().value())
            })
            .or_insert(declaration.work_descriptor.supersession_epoch().value());
        accumulator
            .equivalence_leaders
            .entry(equivalence_key)
            .and_modify(|leader| {
                if declaration.created_order < leader.created_order {
                    *leader = EquivalenceLeader {
                        created_order: declaration.created_order,
                        work_identity: declaration
                            .work_descriptor
                            .work_identity()
                            .as_str()
                            .to_string(),
                    };
                }
            })
            .or_insert_with(|| EquivalenceLeader {
                created_order: declaration.created_order,
                work_identity: declaration
                    .work_descriptor
                    .work_identity()
                    .as_str()
                    .to_string(),
            });

        if let Some(execution) = execution {
            match execution.execution_status {
                MaintenanceExecutionStatus::Reserved => {
                    accumulator.reserved_count += 1;
                    accumulator.active_quantum_count += execution.last_quantum_units.unwrap_or(0);
                }
                MaintenanceExecutionStatus::Started => {
                    accumulator.active_quantum_count += execution.last_quantum_units.unwrap_or(0);
                }
                MaintenanceExecutionStatus::Deferred => {
                    accumulator.deferred_count += 1;
                }
                _ => {}
            }
            if matches!(
                execution.coalescing_decision,
                Some(MaintenanceCoalescingDecision::CoalescedWithEquivalentLaneMember)
            ) {
                accumulator.coalesced_count += 1;
            }
            if matches!(
                execution.coalescing_decision,
                Some(MaintenanceCoalescingDecision::CancelledAsSuperseded)
            ) || execution.supersession_source.is_some()
            {
                accumulator.cancelled_superseded_count += 1;
            }
        }
    }

    let mut locality_accumulators =
        BTreeMap::<MaintenanceLocalityScope, LocalityAccumulator>::new();
    let mut reservation_accumulators =
        BTreeMap::<MaintenanceReservationFamily, ReservationAccumulator>::new();
    let mut reserved_io = 0;
    let mut reserved_cpu = 0;
    let mut reserved_memory = 0;
    let mut reserved_publication = 0;
    let mut reserved_foreground_latency_guard = 0;

    for accumulator in lane_accumulators.into_values() {
        let lane_summary = accumulator.into_queue_summary();
        let lane_id = lane_summary.lane_key().artifact_id();
        let starvation_status = if lane_summary.deferred_count() >= STARVATION_DEFERRED_THRESHOLD {
            MaintenanceStarvationStatus::DeferredLanePressure
        } else {
            MaintenanceStarvationStatus::NotStarved
        };
        let debt_family = state
            .maintenance_declaration_records
            .values()
            .find(|record| record.work_descriptor.lane_key() == *lane_summary.lane_key())
            .and_then(|record| record.work_descriptor.debt_family());
        let pressure_class = if debt_family.is_none() {
            MaintenanceDebtPressureClass::None
        } else if matches!(
            starvation_status,
            MaintenanceStarvationStatus::DeferredLanePressure
        ) {
            MaintenanceDebtPressureClass::Elevated
        } else {
            MaintenanceDebtPressureClass::Active
        };
        let explicit_global_scope_debt = matches!(
            lane_summary.lane_key().locality_scope(),
            MaintenanceLocalityScope::StoreGlobalLocalityScope
        ) && (debt_family.is_some()
            || state.maintenance_execution_records.values().any(|record| {
                record.lane_key.as_ref() == Some(lane_summary.lane_key())
                    && record.explicit_global_scope_debt
            }));

        state.maintenance_queue_summary_records.insert(
            lane_id.clone(),
            MaintenanceQueueSummaryRecord {
                artifact_id: lane_id.clone(),
                family_version: 1,
                summary: lane_summary.clone(),
            },
        );
        state.maintenance_debt_summary_records.insert(
            lane_id.clone(),
            MaintenanceDebtSummaryRecord {
                artifact_id: lane_id.clone(),
                family_version: 1,
                lane_key: lane_summary.lane_key().clone(),
                summary: MaintenanceDebtSummary::new(
                    debt_family,
                    lane_summary.lane_key().locality_scope().clone(),
                    pressure_class,
                    starvation_status,
                    explicit_global_scope_debt,
                ),
            },
        );

        let locality = lane_summary.lane_key().locality_scope().clone();
        let locality_entry = locality_accumulators
            .entry(locality)
            .or_insert_with(LocalityAccumulator::default);
        locality_entry.lane_count += 1;
        locality_entry.admitted_count += lane_summary.admitted_count();
        locality_entry.deferred_count += lane_summary.deferred_count();
        locality_entry.active_count +=
            lane_summary.reserved_count() + lane_summary.active_quantum_count();

        let reservation_entry = reservation_accumulators
            .entry(lane_summary.lane_key().reservation_family())
            .or_insert_with(ReservationAccumulator::default);
        reservation_entry.lane_count += 1;
        reservation_entry.reserved_count += lane_summary.reserved_count();
        reservation_entry.deferred_count += lane_summary.deferred_count();
    }

    for execution in state.maintenance_execution_records.values() {
        if matches!(
            execution.execution_status,
            MaintenanceExecutionStatus::Reserved | MaintenanceExecutionStatus::Started
        ) {
            if let Some(grant) = &execution.resource_budget_grant {
                reserved_io += grant.granted_io().units();
                reserved_cpu += grant.granted_cpu().units();
                reserved_memory += grant.granted_memory().units();
                reserved_publication += grant.granted_publication().units();
                reserved_foreground_latency_guard +=
                    grant.granted_foreground_latency_guard().units();
            }
        }
    }

    for (scope, accumulator) in locality_accumulators {
        let artifact_id = locality_artifact_id(&scope);
        state.maintenance_locality_summary_records.insert(
            artifact_id.clone(),
            MaintenanceLocalitySummaryRecord {
                artifact_id,
                family_version: 1,
                summary: MaintenanceLocalitySummary::new(
                    scope,
                    accumulator.lane_count,
                    accumulator.admitted_count,
                    accumulator.deferred_count,
                    accumulator.active_count,
                ),
            },
        );
    }

    for (family, accumulator) in reservation_accumulators {
        let artifact_id = format!("reservation:{family:?}");
        state.maintenance_reservation_summary_records.insert(
            artifact_id.clone(),
            MaintenanceReservationSummaryRecord {
                artifact_id,
                family_version: 1,
                summary: MaintenanceReservationSummary::new(
                    family,
                    accumulator.lane_count,
                    accumulator.reserved_count,
                    accumulator.deferred_count,
                ),
            },
        );
    }

    state.maintenance_resource_budget_summary_records.insert(
        "maintenance-resource-budget".to_string(),
        MaintenanceResourceBudgetSummaryRecord {
            artifact_id: "maintenance-resource-budget".to_string(),
            family_version: 1,
            summary: MaintenanceResourceBudgetSummary::new(
                IoBudgetUnits::new(AVAILABLE_IO_UNITS),
                IoBudgetUnits::new(reserved_io),
                CpuBudgetUnits::new(AVAILABLE_CPU_UNITS),
                CpuBudgetUnits::new(reserved_cpu),
                MemoryBudgetUnits::new(AVAILABLE_MEMORY_UNITS),
                MemoryBudgetUnits::new(reserved_memory),
                PublicationSlotBudget::new(AVAILABLE_PUBLICATION_UNITS),
                PublicationSlotBudget::new(reserved_publication),
                crate::ForegroundLatencyGuard::new(AVAILABLE_FOREGROUND_LATENCY_GUARD_UNITS),
                crate::ForegroundLatencyGuard::new(reserved_foreground_latency_guard),
            ),
        },
    );
}

pub(crate) fn record_scheduler_boot_state(state: &mut StoreState) {
    let has_maintenance_state = !state.maintenance_declaration_records.is_empty()
        || !state.maintenance_execution_records.is_empty();
    let summaries_present = !state.maintenance_queue_summary_records.is_empty()
        || !state.maintenance_locality_summary_records.is_empty()
        || !state.maintenance_reservation_summary_records.is_empty()
        || !state.maintenance_resource_budget_summary_records.is_empty()
        || !state.maintenance_debt_summary_records.is_empty();
    state.maintenance_loaded_persisted_summaries_on_boot =
        has_maintenance_state && summaries_present;
    state.maintenance_used_legacy_summary_backfill_on_boot = false;
    state.maintenance_recovered_backlog_on_boot = state
        .maintenance_declaration_records
        .values()
        .filter(|record| record.work_descriptor.recovered_from_restart())
        .count() as u64;
}

pub(crate) fn backfill_scheduler_summaries_if_missing(state: &mut StoreState) {
    let has_maintenance_state = !state.maintenance_declaration_records.is_empty()
        || !state.maintenance_execution_records.is_empty();
    let summaries_missing = state.maintenance_queue_summary_records.is_empty()
        && state.maintenance_locality_summary_records.is_empty()
        && state.maintenance_reservation_summary_records.is_empty()
        && state.maintenance_resource_budget_summary_records.is_empty()
        && state.maintenance_debt_summary_records.is_empty();

    if has_maintenance_state && summaries_missing {
        refresh_scheduler_summaries(state);
        state.maintenance_used_legacy_summary_backfill_on_boot = true;
    }
}

pub(crate) fn default_resource_budget_summary() -> MaintenanceResourceBudgetSummary {
    MaintenanceResourceBudgetSummary::new(
        IoBudgetUnits::new(AVAILABLE_IO_UNITS),
        IoBudgetUnits::new(0),
        CpuBudgetUnits::new(AVAILABLE_CPU_UNITS),
        CpuBudgetUnits::new(0),
        MemoryBudgetUnits::new(AVAILABLE_MEMORY_UNITS),
        MemoryBudgetUnits::new(0),
        PublicationSlotBudget::new(AVAILABLE_PUBLICATION_UNITS),
        PublicationSlotBudget::new(0),
        crate::ForegroundLatencyGuard::new(AVAILABLE_FOREGROUND_LATENCY_GUARD_UNITS),
        crate::ForegroundLatencyGuard::new(0),
    )
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

fn empty_lane_summary(lane_key: MaintenanceLaneKey) -> MaintenanceQueueSummary {
    MaintenanceQueueSummary::new(
        lane_key,
        0,
        0,
        0,
        0,
        0,
        0,
        BTreeMap::new(),
        BTreeMap::new(),
        BTreeMap::new(),
    )
}

fn locality_artifact_id(scope: &MaintenanceLocalityScope) -> String {
    match scope {
        MaintenanceLocalityScope::BranchLocalityScope { branch_label } => {
            format!("locality:branch:{branch_label}")
        }
        MaintenanceLocalityScope::ArtifactFamilyLocalityScope { family_label } => {
            format!("locality:family:{family_label}")
        }
        MaintenanceLocalityScope::TenantLocalityScope { tenant_label } => {
            format!("locality:tenant:{tenant_label}")
        }
        MaintenanceLocalityScope::StoreGlobalLocalityScope => "locality:store:global".to_string(),
    }
}

#[derive(Debug, Clone)]
struct EquivalenceLeader {
    created_order: u64,
    work_identity: String,
}

#[derive(Debug, Clone)]
struct LaneAccumulator {
    lane_key: MaintenanceLaneKey,
    admitted_count: u64,
    reserved_count: u64,
    deferred_count: u64,
    active_quantum_count: u64,
    coalesced_count: u64,
    cancelled_superseded_count: u64,
    equivalence_member_counts: BTreeMap<String, u64>,
    equivalence_leaders: BTreeMap<String, EquivalenceLeader>,
    max_supersession_epoch_by_equivalence: BTreeMap<String, u64>,
}

impl LaneAccumulator {
    fn new(lane_key: MaintenanceLaneKey) -> Self {
        Self {
            lane_key,
            admitted_count: 0,
            reserved_count: 0,
            deferred_count: 0,
            active_quantum_count: 0,
            coalesced_count: 0,
            cancelled_superseded_count: 0,
            equivalence_member_counts: BTreeMap::new(),
            equivalence_leaders: BTreeMap::new(),
            max_supersession_epoch_by_equivalence: BTreeMap::new(),
        }
    }

    fn into_queue_summary(self) -> MaintenanceQueueSummary {
        MaintenanceQueueSummary::new(
            self.lane_key,
            self.admitted_count,
            self.reserved_count,
            self.deferred_count,
            self.active_quantum_count,
            self.coalesced_count,
            self.cancelled_superseded_count,
            self.equivalence_member_counts,
            self.equivalence_leaders
                .into_iter()
                .map(|(key, leader)| (key, leader.work_identity))
                .collect(),
            self.max_supersession_epoch_by_equivalence,
        )
    }
}

#[derive(Debug, Default, Clone)]
struct LocalityAccumulator {
    lane_count: u64,
    admitted_count: u64,
    deferred_count: u64,
    active_count: u64,
}

#[derive(Debug, Default, Clone)]
struct ReservationAccumulator {
    lane_count: u64,
    reserved_count: u64,
    deferred_count: u64,
}
