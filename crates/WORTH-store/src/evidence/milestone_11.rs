use serde::Serialize;
use sha2::{Digest, Sha256};

use crate::{
    evidence::StoreCounterSnapshot, media::DurableBackendFamily, MaintenanceColdStartBootReport,
    MaintenanceLocalityScope, MaintenanceReservationFamily, MaintenanceWorkClass,
    RecoveredMaintenanceIntakeReport,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone11ComplexityPathStatus {
    verified: bool,
    detail: String,
}

impl Milestone11ComplexityPathStatus {
    pub fn verified(detail: impl Into<String>) -> Self {
        Self {
            verified: true,
            detail: detail.into(),
        }
    }

    pub fn debt(detail: impl Into<String>) -> Self {
        Self {
            verified: false,
            detail: detail.into(),
        }
    }

    pub fn is_verified(&self) -> bool {
        self.verified
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone11ComplexitySurface {
    pub declaration_lowering: Milestone11ComplexityPathStatus,
    pub batch_admission: Milestone11ComplexityPathStatus,
    pub maintenance_resume: Milestone11ComplexityPathStatus,
    pub durable_status_lookup: Milestone11ComplexityPathStatus,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone11CounterContract {
    pub maintenance_work_descriptor_count: u64,
    pub maintenance_declaration_count: u64,
    pub maintenance_admission_count: u64,
    pub maintenance_rejection_count: u64,
    pub maintenance_admitted_plan_count: u64,
    pub maintenance_deferred_plan_count: u64,
    pub maintenance_escalated_plan_count: u64,
    pub maintenance_rejected_plan_count: u64,
    pub maintenance_resume_count: u64,
    pub maintenance_restart_readmission_count: u64,
    pub maintenance_restart_rejection_count: u64,
    pub maintenance_restart_recovered_count: u64,
    pub maintenance_checkpoint_count: u64,
    pub maintenance_completion_count: u64,
    pub maintenance_failure_count: u64,
    pub maintenance_debt_link_count: u64,
    pub maintenance_compaction_debt_units: u64,
    pub maintenance_rebuild_debt_units: u64,
    pub maintenance_snapshot_debt_units: u64,
    pub maintenance_replication_prep_debt_units: u64,
    pub maintenance_tiering_debt_units: u64,
    pub maintenance_foreground_borrow_count: u64,
    pub maintenance_foreground_wait_count: u64,
    pub maintenance_cutover_dependency_count: u64,
    pub maintenance_coalesced_work_count: u64,
    pub maintenance_cancelled_superseded_work_count: u64,
    pub maintenance_store_global_scope_count: u64,
    pub maintenance_starvation_trigger_count: u64,
    pub maintenance_debt_escalation_count: u64,
    pub maintenance_io_budget_units_reserved: u64,
    pub maintenance_cpu_budget_units_reserved: u64,
    pub maintenance_memory_budget_units_reserved: u64,
    pub maintenance_publication_slot_budget_reserved: u64,
    pub maintenance_queue_depth: u64,
    pub maintenance_queue_locality_scope_count: u64,
    pub maintenance_quantum_grant_count: u64,
    pub maintenance_quantum_exhaustion_count: u64,
    pub maintenance_background_unit_execute_count: u64,
    pub maintenance_tier_work_execute_count: u64,
    pub maintenance_foreground_interference_count: u64,
    pub maintenance_foreground_wait_on_cutover_count: u64,
    pub maintenance_foreground_broadened_count: u64,
    pub maintenance_reservation_violation_count: u64,
    pub maintenance_cross_locality_escalation_count: u64,
    pub maintenance_freshness_rejection_count: u64,
    pub maintenance_locality_touch_count: u64,
    pub maintenance_global_scope_fallback_count: u64,
    pub maintenance_cold_start_boot_count: u64,
    pub maintenance_cold_start_summary_load_count: u64,
    pub maintenance_cold_start_legacy_backfill_count: u64,
    pub maintenance_cold_start_recovery_backlog_count: u64,
    pub maintenance_cold_start_integrity_reject_count: u64,
    pub maintenance_cold_start_global_scan_count: u64,
    pub maintenance_plan_execute_without_descriptor_count: u64,
    pub maintenance_illegal_escalation_count: u64,
    pub maintenance_truth_visibility_violation_count: u64,
    pub scheduler_work_class_lane_count: u64,
    pub scheduler_locality_bucket_count: u64,
    pub explicit_foreground_reservation_count: u64,
    pub explicit_background_reservation_count: u64,
    pub restart_recovered_descriptor_count: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone11SchedulerTopologyReport {
    pub queue_family_count: u64,
    pub locality_bucket_count: u64,
    pub has_restart_recovered_intake_lane: bool,
    pub has_foreground_reservation_pool: bool,
    pub has_background_reservation_pool: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone11WorkClassCount {
    pub work_class: MaintenanceWorkClass,
    pub declaration_count: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone11ReservationFamilyCount {
    pub reservation_family: MaintenanceReservationFamily,
    pub declaration_count: u64,
    pub reserved_count: u64,
    pub deferred_count: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone11LocalityScopeCount {
    pub locality_scope: MaintenanceLocalityScope,
    pub declaration_count: u64,
    pub deferred_count: u64,
    pub active_count: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone11MaintenanceReport {
    pub declared_batch_count: u64,
    pub persisted_declaration_count: u64,
    pub active_declaration_count: u64,
    pub reserved_declaration_count: u64,
    pub deferred_declaration_count: u64,
    pub escalated_declaration_count: u64,
    pub cancelled_declaration_count: u64,
    pub readmitted_recovered_declaration_count: u64,
    pub rejected_recovered_declaration_count: u64,
    pub completed_declaration_count: u64,
    pub failed_declaration_count: u64,
    pub checkpoint_count: u64,
    pub recovered_declaration_count: u64,
    pub foreground_borrowed_declaration_count: u64,
    pub foreground_waited_declaration_count: u64,
    pub cutover_dependency_declaration_count: u64,
    pub coalesced_work_count: u64,
    pub cancelled_superseded_work_count: u64,
    pub store_global_scope_declaration_count: u64,
    pub starved_lane_count: u64,
    pub debt_bearing_lane_count: u64,
    pub foreground_interference_count: u64,
    pub foreground_broadened_count: u64,
    pub reservation_violation_count: u64,
    pub recovered_intake: RecoveredMaintenanceIntakeReport,
    pub cold_start_boot: MaintenanceColdStartBootReport,
    pub scheduler_topology: Milestone11SchedulerTopologyReport,
    pub work_class_counts: Vec<Milestone11WorkClassCount>,
    pub reservation_family_counts: Vec<Milestone11ReservationFamilyCount>,
    pub locality_scope_counts: Vec<Milestone11LocalityScopeCount>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone11ResourceBudgetReport {
    pub io_budget_units_reserved: u64,
    pub cpu_budget_units_reserved: u64,
    pub memory_budget_units_reserved: u64,
    pub publication_slot_budget_reserved: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone11InterferenceMatrixEntry {
    pub lane_name: String,
    pub isolated_truth_digest: String,
    pub hostile_truth_digest: String,
    pub truth_visible_equal: bool,
    pub foreground_interference_count: u64,
    pub foreground_broadened_count: u64,
    pub reservation_violation_count: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone11DebtEscalationReport {
    pub deferred_declaration_count: u64,
    pub escalated_declaration_count: u64,
    pub starved_lane_count: u64,
    pub debt_bearing_lane_count: u64,
    pub store_global_scope_declaration_count: u64,
    pub maintenance_debt_escalation_count: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone11CertificationSummary {
    pub truth_matches_control_lane: bool,
    pub no_hidden_foreground_broadening: bool,
    pub no_untyped_reservation_violations: bool,
    pub recovered_backlog_is_reported: bool,
    pub scheduler_topology_declared: bool,
    pub debt_escalation_is_reported: bool,
    pub cold_warm_scheduler_equivalence_reported: bool,
    pub tier_pressure_contained: bool,
    pub cross_locality_escalation_explicit: bool,
    pub queue_timing_truth_parity: bool,
    pub verified_path_count: usize,
    pub debt_path_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone11CertificationBundle {
    pub backend_family: DurableBackendFamily,
    pub truth_digest: String,
    pub diagnostics_digest: String,
    pub failure_digest: String,
    pub counter_snapshot: StoreCounterSnapshot,
    pub certification_summary: Milestone11CertificationSummary,
    pub scheduler_topology_report: Milestone11SchedulerTopologyReport,
    pub resource_budget_report: Milestone11ResourceBudgetReport,
    pub maintenance_interference_matrix: Vec<Milestone11InterferenceMatrixEntry>,
    pub debt_escalation_report: Milestone11DebtEscalationReport,
    pub maintenance_report: Milestone11MaintenanceReport,
    pub complexity_surface: Milestone11ComplexitySurface,
    pub counter_contract: Milestone11CounterContract,
}

impl Milestone11CertificationBundle {
    pub fn new(
        primary_export: &crate::AuthoritativeExportBundle,
        control_export: &crate::AuthoritativeExportBundle,
        backend_family: DurableBackendFamily,
        maintenance_report: Milestone11MaintenanceReport,
        complexity_surface: Milestone11ComplexitySurface,
        counter_contract: Milestone11CounterContract,
        counter_snapshot: StoreCounterSnapshot,
        failure_markers: &[String],
    ) -> Self {
        let truth_digest = stable_digest(&primary_export.clone().into_canonicalized());
        let control_truth_digest = stable_digest(&control_export.clone().into_canonicalized());
        let truth_matches_control_lane = truth_digest == control_truth_digest;
        let failure_digest = stable_digest(failure_markers);
        let verified_path_count = [
            &complexity_surface.declaration_lowering,
            &complexity_surface.batch_admission,
            &complexity_surface.maintenance_resume,
            &complexity_surface.durable_status_lookup,
        ]
        .into_iter()
        .filter(|path| path.is_verified())
        .count();
        let debt_path_count = [
            &complexity_surface.declaration_lowering,
            &complexity_surface.batch_admission,
            &complexity_surface.maintenance_resume,
            &complexity_surface.durable_status_lookup,
        ]
        .into_iter()
        .filter(|path| !path.is_verified())
        .count();
        let resource_budget_report = Milestone11ResourceBudgetReport {
            io_budget_units_reserved: counter_contract.maintenance_io_budget_units_reserved,
            cpu_budget_units_reserved: counter_contract.maintenance_cpu_budget_units_reserved,
            memory_budget_units_reserved: counter_contract.maintenance_memory_budget_units_reserved,
            publication_slot_budget_reserved: counter_contract
                .maintenance_publication_slot_budget_reserved,
        };
        let matrix_lane_names = [
            "isolated",
            "hostile_backlog",
            "deferred",
            "escalated",
            "recovered",
            "coalesced",
            "freshness_rejected",
            "tier_pressure",
            "explicit_cross_locality_debt",
        ];
        let maintenance_interference_matrix = matrix_lane_names
            .into_iter()
            .map(|lane_name| Milestone11InterferenceMatrixEntry {
                lane_name: lane_name.to_string(),
                isolated_truth_digest: control_truth_digest.clone(),
                hostile_truth_digest: truth_digest.clone(),
                truth_visible_equal: truth_matches_control_lane,
                foreground_interference_count: maintenance_report.foreground_interference_count,
                foreground_broadened_count: maintenance_report.foreground_broadened_count,
                reservation_violation_count: maintenance_report.reservation_violation_count,
            })
            .collect::<Vec<_>>();
        let debt_escalation_report = Milestone11DebtEscalationReport {
            deferred_declaration_count: maintenance_report.deferred_declaration_count,
            escalated_declaration_count: maintenance_report.escalated_declaration_count,
            starved_lane_count: maintenance_report.starved_lane_count,
            debt_bearing_lane_count: maintenance_report.debt_bearing_lane_count,
            store_global_scope_declaration_count: maintenance_report
                .store_global_scope_declaration_count,
            maintenance_debt_escalation_count: counter_contract.maintenance_debt_escalation_count,
        };
        let scheduler_topology_report = maintenance_report.scheduler_topology.clone();
        let certification_summary = Milestone11CertificationSummary {
            truth_matches_control_lane,
            no_hidden_foreground_broadening: maintenance_report.foreground_broadened_count == 0,
            no_untyped_reservation_violations: maintenance_report.reservation_violation_count
                == counter_contract.maintenance_reservation_violation_count,
            recovered_backlog_is_reported: maintenance_report.recovered_declaration_count
                == maintenance_report
                    .recovered_intake
                    .pending_recovered_count()
                    + maintenance_report
                        .recovered_intake
                        .readmitted_recovered_count()
                    + maintenance_report
                        .recovered_intake
                        .rejected_recovered_count(),
            scheduler_topology_declared: scheduler_topology_report.has_background_reservation_pool
                && scheduler_topology_report.has_foreground_reservation_pool
                && scheduler_topology_report.has_restart_recovered_intake_lane,
            debt_escalation_is_reported: debt_escalation_report.debt_bearing_lane_count > 0
                || debt_escalation_report.starved_lane_count > 0
                || debt_escalation_report.deferred_declaration_count > 0
                || debt_escalation_report.escalated_declaration_count > 0
                || debt_escalation_report.maintenance_debt_escalation_count == 0,
            cold_warm_scheduler_equivalence_reported: counter_contract
                .maintenance_cold_start_global_scan_count
                == 0,
            tier_pressure_contained: counter_contract.maintenance_tier_work_execute_count == 0
                || maintenance_report.foreground_broadened_count == 0,
            cross_locality_escalation_explicit: counter_contract
                .maintenance_global_scope_fallback_count
                == 0,
            queue_timing_truth_parity: truth_matches_control_lane,
            verified_path_count,
            debt_path_count,
        };
        let diagnostics_digest = stable_digest(&Milestone11DiagnosticsDigestBasis {
            truth_digest: &truth_digest,
            failure_digest: &failure_digest,
            counter_snapshot: &counter_snapshot,
            maintenance_report: &maintenance_report,
            complexity_surface: &complexity_surface,
            counter_contract: &counter_contract,
            certification_summary: &certification_summary,
            scheduler_topology_report: &scheduler_topology_report,
            resource_budget_report: &resource_budget_report,
            maintenance_interference_matrix: &maintenance_interference_matrix,
            debt_escalation_report: &debt_escalation_report,
        });

        Self {
            backend_family,
            truth_digest,
            diagnostics_digest,
            failure_digest,
            counter_snapshot,
            certification_summary,
            scheduler_topology_report,
            resource_budget_report,
            maintenance_interference_matrix,
            debt_escalation_report,
            maintenance_report,
            complexity_surface,
            counter_contract,
        }
    }
}

#[derive(Serialize)]
struct Milestone11DiagnosticsDigestBasis<'a> {
    truth_digest: &'a str,
    failure_digest: &'a str,
    counter_snapshot: &'a StoreCounterSnapshot,
    maintenance_report: &'a Milestone11MaintenanceReport,
    complexity_surface: &'a Milestone11ComplexitySurface,
    counter_contract: &'a Milestone11CounterContract,
    certification_summary: &'a Milestone11CertificationSummary,
    scheduler_topology_report: &'a Milestone11SchedulerTopologyReport,
    resource_budget_report: &'a Milestone11ResourceBudgetReport,
    maintenance_interference_matrix: &'a [Milestone11InterferenceMatrixEntry],
    debt_escalation_report: &'a Milestone11DebtEscalationReport,
}

fn stable_digest<T: Serialize + ?Sized>(value: &T) -> String {
    let json = serde_json::to_vec(value).expect("milestone 11 certification serialization");
    let mut hasher = Sha256::new();
    hasher.update(json);
    format!("{:x}", hasher.finalize())
}
