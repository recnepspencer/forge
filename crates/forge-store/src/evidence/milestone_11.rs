use serde::Serialize;

use crate::{MaintenanceLocalityScope, MaintenanceReservationFamily, MaintenanceWorkClass};

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
    pub maintenance_declaration_count: u64,
    pub maintenance_admission_count: u64,
    pub maintenance_rejection_count: u64,
    pub maintenance_resume_count: u64,
    pub maintenance_restart_readmission_count: u64,
    pub maintenance_restart_rejection_count: u64,
    pub maintenance_checkpoint_count: u64,
    pub maintenance_completion_count: u64,
    pub maintenance_failure_count: u64,
    pub maintenance_debt_link_count: u64,
    pub maintenance_foreground_borrow_count: u64,
    pub maintenance_foreground_wait_count: u64,
    pub maintenance_cutover_dependency_count: u64,
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
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone11LocalityScopeCount {
    pub locality_scope: MaintenanceLocalityScope,
    pub declaration_count: u64,
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
    pub scheduler_topology: Milestone11SchedulerTopologyReport,
    pub work_class_counts: Vec<Milestone11WorkClassCount>,
    pub reservation_family_counts: Vec<Milestone11ReservationFamilyCount>,
    pub locality_scope_counts: Vec<Milestone11LocalityScopeCount>,
}
