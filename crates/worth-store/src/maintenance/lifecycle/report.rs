use serde::Serialize;

use super::super::{
    FreshnessWindow, MaintenanceCoalescingDecision, MaintenanceDebtFamily,
    MaintenanceDeclarationClass, MaintenanceDeclarationId, MaintenanceEscalationDecision,
    MaintenanceEscalationVerdict, MaintenanceExecutionPosture, MaintenanceLaneKey,
    MaintenanceLocalityScope, MaintenancePlanFamily, MaintenanceReservationFamily,
    MaintenanceResourceBudgetGrant, MaintenanceStarvationStatus, MaintenanceWorkClass,
    PlanGeneration, SupersessionEpoch,
};

use super::status::{
    MaintenanceExecutionStatus, MaintenanceForegroundImpact, MaintenanceReadmissionStatus,
};

use super::transitions::{MaintenanceExecutionTransition, MaintenanceReservationTransition};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct MaintenanceStatusReport {
    declaration_id: MaintenanceDeclarationId,
    declaration_class: MaintenanceDeclarationClass,
    work_class: MaintenanceWorkClass,
    execution_posture: MaintenanceExecutionPosture,
    locality_scope: MaintenanceLocalityScope,
    lane_key: MaintenanceLaneKey,
    reservation_family: MaintenanceReservationFamily,
    plan_generation: PlanGeneration,
    supersession_epoch: SupersessionEpoch,
    freshness_window: FreshnessWindow,
    debt_family: Option<MaintenanceDebtFamily>,
    escalation_decision: MaintenanceEscalationDecision,
    tier_work_container_class: Option<crate::TierWorkContainerClass>,
    recovered_from_restart: bool,
    restart_readmission_status: Option<MaintenanceReadmissionStatus>,
    reservation_transition: Option<MaintenanceReservationTransition>,
    execution_transition: Option<MaintenanceExecutionTransition>,
    foreground_impact: MaintenanceForegroundImpact,
    coalescing_decision: Option<MaintenanceCoalescingDecision>,
    supersession_source: Option<String>,
    resource_budget_grant: Option<MaintenanceResourceBudgetGrant>,
    starvation_status: Option<MaintenanceStarvationStatus>,
    escalation_verdict: Option<MaintenanceEscalationVerdict>,
    explicit_global_scope_debt: bool,
    plan_family: Option<MaintenancePlanFamily>,
    pending_reason: Option<String>,
    execution_status: MaintenanceExecutionStatus,
    last_completed_phase: Option<String>,
    durable_error_kind: Option<String>,
    debt_link_artifact_id: Option<String>,
}

#[derive(Debug, Clone)]
pub(crate) struct MaintenanceStatusReportBasis {
    pub(crate) declaration_id: MaintenanceDeclarationId,
    pub(crate) declaration_class: MaintenanceDeclarationClass,
    pub(crate) work_class: MaintenanceWorkClass,
    pub(crate) execution_posture: MaintenanceExecutionPosture,
    pub(crate) locality_scope: MaintenanceLocalityScope,
    pub(crate) lane_key: MaintenanceLaneKey,
    pub(crate) reservation_family: MaintenanceReservationFamily,
    pub(crate) plan_generation: PlanGeneration,
    pub(crate) supersession_epoch: SupersessionEpoch,
    pub(crate) freshness_window: FreshnessWindow,
    pub(crate) debt_family: Option<MaintenanceDebtFamily>,
    pub(crate) escalation_decision: MaintenanceEscalationDecision,
    pub(crate) tier_work_container_class: Option<crate::TierWorkContainerClass>,
    pub(crate) recovered_from_restart: bool,
    pub(crate) restart_readmission_status: Option<MaintenanceReadmissionStatus>,
    pub(crate) reservation_transition: Option<MaintenanceReservationTransition>,
    pub(crate) execution_transition: Option<MaintenanceExecutionTransition>,
    pub(crate) foreground_impact: MaintenanceForegroundImpact,
    pub(crate) coalescing_decision: Option<MaintenanceCoalescingDecision>,
    pub(crate) supersession_source: Option<String>,
    pub(crate) resource_budget_grant: Option<MaintenanceResourceBudgetGrant>,
    pub(crate) starvation_status: Option<MaintenanceStarvationStatus>,
    pub(crate) escalation_verdict: Option<MaintenanceEscalationVerdict>,
    pub(crate) explicit_global_scope_debt: bool,
    pub(crate) plan_family: Option<MaintenancePlanFamily>,
    pub(crate) pending_reason: Option<String>,
    pub(crate) execution_status: MaintenanceExecutionStatus,
    pub(crate) last_completed_phase: Option<String>,
    pub(crate) durable_error_kind: Option<String>,
    pub(crate) debt_link_artifact_id: Option<String>,
}

impl MaintenanceStatusReport {
    pub(crate) fn new(basis: MaintenanceStatusReportBasis) -> Self {
        Self {
            declaration_id: basis.declaration_id,
            declaration_class: basis.declaration_class,
            work_class: basis.work_class,
            execution_posture: basis.execution_posture,
            locality_scope: basis.locality_scope,
            lane_key: basis.lane_key,
            reservation_family: basis.reservation_family,
            plan_generation: basis.plan_generation,
            supersession_epoch: basis.supersession_epoch,
            freshness_window: basis.freshness_window,
            debt_family: basis.debt_family,
            escalation_decision: basis.escalation_decision,
            tier_work_container_class: basis.tier_work_container_class,
            recovered_from_restart: basis.recovered_from_restart,
            restart_readmission_status: basis.restart_readmission_status,
            reservation_transition: basis.reservation_transition,
            execution_transition: basis.execution_transition,
            foreground_impact: basis.foreground_impact,
            coalescing_decision: basis.coalescing_decision,
            supersession_source: basis.supersession_source,
            resource_budget_grant: basis.resource_budget_grant,
            starvation_status: basis.starvation_status,
            escalation_verdict: basis.escalation_verdict,
            explicit_global_scope_debt: basis.explicit_global_scope_debt,
            plan_family: basis.plan_family,
            pending_reason: basis.pending_reason,
            execution_status: basis.execution_status,
            last_completed_phase: basis.last_completed_phase,
            durable_error_kind: basis.durable_error_kind,
            debt_link_artifact_id: basis.debt_link_artifact_id,
        }
    }

    pub fn declaration_id(&self) -> &MaintenanceDeclarationId {
        &self.declaration_id
    }

    pub fn declaration_class(&self) -> MaintenanceDeclarationClass {
        self.declaration_class
    }

    pub fn work_class(&self) -> MaintenanceWorkClass {
        self.work_class
    }

    pub fn execution_posture(&self) -> MaintenanceExecutionPosture {
        self.execution_posture
    }

    pub fn locality_scope(&self) -> &MaintenanceLocalityScope {
        &self.locality_scope
    }

    pub fn lane_key(&self) -> &MaintenanceLaneKey {
        &self.lane_key
    }

    pub fn reservation_family(&self) -> MaintenanceReservationFamily {
        self.reservation_family
    }

    pub fn plan_generation(&self) -> PlanGeneration {
        self.plan_generation
    }

    pub fn supersession_epoch(&self) -> SupersessionEpoch {
        self.supersession_epoch
    }

    pub fn freshness_window(&self) -> FreshnessWindow {
        self.freshness_window
    }

    pub fn debt_family(&self) -> Option<MaintenanceDebtFamily> {
        self.debt_family
    }

    pub fn escalation_decision(&self) -> MaintenanceEscalationDecision {
        self.escalation_decision
    }

    pub fn tier_work_container_class(&self) -> Option<crate::TierWorkContainerClass> {
        self.tier_work_container_class
    }

    pub fn recovered_from_restart(&self) -> bool {
        self.recovered_from_restart
    }

    pub fn restart_readmission_status(&self) -> Option<MaintenanceReadmissionStatus> {
        self.restart_readmission_status
    }

    pub fn reservation_transition(&self) -> Option<&MaintenanceReservationTransition> {
        self.reservation_transition.as_ref()
    }

    pub fn execution_transition(&self) -> Option<&MaintenanceExecutionTransition> {
        self.execution_transition.as_ref()
    }

    pub fn foreground_impact(&self) -> &MaintenanceForegroundImpact {
        &self.foreground_impact
    }

    pub fn coalescing_decision(&self) -> Option<MaintenanceCoalescingDecision> {
        self.coalescing_decision
    }

    pub fn supersession_source(&self) -> Option<&str> {
        self.supersession_source.as_deref()
    }

    pub fn resource_budget_grant(&self) -> Option<&MaintenanceResourceBudgetGrant> {
        self.resource_budget_grant.as_ref()
    }

    pub fn starvation_status(&self) -> Option<MaintenanceStarvationStatus> {
        self.starvation_status
    }

    pub fn escalation_verdict(&self) -> Option<MaintenanceEscalationVerdict> {
        self.escalation_verdict
    }

    pub fn explicit_global_scope_debt(&self) -> bool {
        self.explicit_global_scope_debt
    }

    pub fn plan_family(&self) -> Option<MaintenancePlanFamily> {
        self.plan_family
    }

    pub fn pending_reason(&self) -> Option<&str> {
        self.pending_reason.as_deref()
    }

    pub fn execution_status(&self) -> MaintenanceExecutionStatus {
        self.execution_status
    }

    pub fn last_completed_phase(&self) -> Option<&str> {
        self.last_completed_phase.as_deref()
    }

    pub fn durable_error_kind(&self) -> Option<&str> {
        self.durable_error_kind.as_deref()
    }

    pub fn debt_link_artifact_id(&self) -> Option<&str> {
        self.debt_link_artifact_id.as_deref()
    }
}
