use serde::{Deserialize, Serialize};

use crate::{
    MaintenanceCoalescingDecision, MaintenanceEscalationVerdict, MaintenanceExecutionStatus,
    MaintenanceExecutionTransition, MaintenanceForegroundImpact, MaintenanceLaneKey,
    MaintenancePlanFamily, MaintenanceReadmissionStatus, MaintenanceReservationTransition,
    MaintenanceResourceBudgetGrant, MaintenanceStarvationStatus,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MaintenanceExecutionRecord {
    pub artifact_id: String,
    pub family_version: u32,
    pub declaration_id: String,
    pub execution_status: MaintenanceExecutionStatus,
    #[serde(default)]
    pub lane_key: Option<MaintenanceLaneKey>,
    #[serde(default)]
    pub plan_family: Option<MaintenancePlanFamily>,
    pub last_completed_phase: Option<String>,
    #[serde(default)]
    pub pending_reason: Option<String>,
    pub durable_error_kind: Option<String>,
    pub durable_error_message: Option<String>,
    #[serde(default)]
    pub last_quantum_units: Option<u64>,
    #[serde(default)]
    pub reservation_transition: Option<MaintenanceReservationTransition>,
    #[serde(default)]
    pub execution_transition: Option<MaintenanceExecutionTransition>,
    #[serde(default)]
    pub restart_readmission_status: Option<MaintenanceReadmissionStatus>,
    #[serde(default = "MaintenanceForegroundImpact::none")]
    pub foreground_impact: MaintenanceForegroundImpact,
    #[serde(default)]
    pub coalescing_decision: Option<MaintenanceCoalescingDecision>,
    #[serde(default)]
    pub supersession_source: Option<String>,
    #[serde(default)]
    pub resource_budget_grant: Option<MaintenanceResourceBudgetGrant>,
    #[serde(default)]
    pub starvation_status: Option<MaintenanceStarvationStatus>,
    #[serde(default)]
    pub escalation_verdict: Option<MaintenanceEscalationVerdict>,
    #[serde(default)]
    pub explicit_global_scope_debt: bool,
    pub resume_count: u64,
}
