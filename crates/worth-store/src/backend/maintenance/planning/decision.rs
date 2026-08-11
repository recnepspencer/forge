use crate::maintenance::{
    AdmittedMaintenanceWork, BackgroundPacedMaintenancePlan, DeferredMaintenancePlan,
    EscalatedMaintenancePlan, ForegroundReservedMaintenancePlan, MaintenanceCoalescingDecision,
    MaintenanceEscalationDecision, MaintenanceEscalationVerdict, MaintenanceLaneKey,
    MaintenancePlanFamily, MaintenanceResourceBudgetGrant, MaintenanceStarvationStatus,
    ReservedMaintenanceWork,
};

#[derive(Debug, Clone)]
pub(crate) struct ResumedExecutionState {
    pub(crate) plan_family: MaintenancePlanFamily,
    pub(crate) resource_budget_grant: MaintenanceResourceBudgetGrant,
    pub(crate) starvation_status: MaintenanceStarvationStatus,
    pub(crate) escalation_verdict: MaintenanceEscalationVerdict,
    pub(crate) explicit_global_scope_debt: bool,
}

#[derive(Debug, Clone)]
pub(crate) enum LoweredMaintenancePlan {
    ForegroundReserved(ForegroundReservedMaintenancePlan),
    BackgroundPaced(BackgroundPacedMaintenancePlan),
    Escalated(EscalatedMaintenancePlan),
    Deferred(DeferredMaintenancePlan),
    Cancelled { reason: String },
}

#[derive(Debug, Clone)]
pub(crate) struct PlanningDecision {
    lowered_plan: LoweredMaintenancePlan,
    lane_key: MaintenanceLaneKey,
    coalescing_decision: MaintenanceCoalescingDecision,
    supersession_source: Option<String>,
    resource_budget_grant: Option<MaintenanceResourceBudgetGrant>,
    starvation_status: MaintenanceStarvationStatus,
    escalation_verdict: MaintenanceEscalationVerdict,
    explicit_global_scope_debt: bool,
}

impl PlanningDecision {
    pub(super) fn new(
        lowered_plan: LoweredMaintenancePlan,
        lane_key: MaintenanceLaneKey,
        coalescing_decision: MaintenanceCoalescingDecision,
        supersession_source: Option<String>,
        resource_budget_grant: Option<MaintenanceResourceBudgetGrant>,
        starvation_status: MaintenanceStarvationStatus,
        escalation_verdict: MaintenanceEscalationVerdict,
        explicit_global_scope_debt: bool,
    ) -> Self {
        Self {
            lowered_plan,
            lane_key,
            coalescing_decision,
            supersession_source,
            resource_budget_grant,
            starvation_status,
            escalation_verdict,
            explicit_global_scope_debt,
        }
    }

    pub(crate) fn family(&self) -> MaintenancePlanFamily {
        self.lowered_plan.family()
    }

    pub(crate) fn reason(&self) -> Option<&str> {
        self.lowered_plan.reason()
    }

    pub(crate) fn quantum_units(&self) -> Option<u64> {
        self.lowered_plan.quantum_units()
    }

    pub(crate) fn into_reserved_work(
        self,
        admitted_work: AdmittedMaintenanceWork,
    ) -> Option<ReservedMaintenanceWork> {
        self.lowered_plan.into_reserved_work(admitted_work)
    }

    pub(crate) fn lane_key(&self) -> &MaintenanceLaneKey {
        &self.lane_key
    }

    pub(crate) fn coalescing_decision(&self) -> MaintenanceCoalescingDecision {
        self.coalescing_decision
    }

    pub(crate) fn supersession_source(&self) -> Option<&str> {
        self.supersession_source.as_deref()
    }

    pub(crate) fn resource_budget_grant(&self) -> Option<&MaintenanceResourceBudgetGrant> {
        self.resource_budget_grant.as_ref()
    }

    pub(crate) fn starvation_status(&self) -> MaintenanceStarvationStatus {
        self.starvation_status
    }

    pub(crate) fn escalation_verdict(&self) -> MaintenanceEscalationVerdict {
        self.escalation_verdict
    }

    pub(crate) fn explicit_global_scope_debt(&self) -> bool {
        self.explicit_global_scope_debt
    }
}

impl LoweredMaintenancePlan {
    pub(super) fn family(&self) -> MaintenancePlanFamily {
        match self {
            Self::ForegroundReserved(_) => MaintenancePlanFamily::ForegroundReserved,
            Self::BackgroundPaced(_) => MaintenancePlanFamily::BackgroundPaced,
            Self::Escalated(_) => MaintenancePlanFamily::Escalated,
            Self::Deferred(_) => MaintenancePlanFamily::Deferred,
            Self::Cancelled { .. } => MaintenancePlanFamily::Cancelled,
        }
    }

    pub(super) fn reason(&self) -> Option<&str> {
        match self {
            Self::Deferred(plan) => Some(plan.reason()),
            Self::Cancelled { reason } => Some(reason),
            _ => None,
        }
    }

    pub(super) fn quantum_units(&self) -> Option<u64> {
        match self {
            Self::ForegroundReserved(plan) => {
                Some(plan.quantum_budget_receipt().maintenance_quantum().units())
            }
            Self::BackgroundPaced(plan) => {
                Some(plan.quantum_budget_receipt().maintenance_quantum().units())
            }
            Self::Escalated(plan) => {
                Some(plan.quantum_budget_receipt().maintenance_quantum().units())
            }
            Self::Deferred(_) | Self::Cancelled { .. } => None,
        }
    }

    pub(super) fn escalation_decision(&self) -> Option<MaintenanceEscalationDecision> {
        match self {
            Self::ForegroundReserved(_) | Self::BackgroundPaced(_) => {
                Some(MaintenanceEscalationDecision::StayBackground)
            }
            Self::Escalated(plan) => Some(plan.escalation_decision()),
            Self::Deferred(_) | Self::Cancelled { .. } => None,
        }
    }

    pub(super) fn into_reserved_work(
        self,
        admitted_work: AdmittedMaintenanceWork,
    ) -> Option<ReservedMaintenanceWork> {
        let escalation_decision = self.escalation_decision()?;
        let quantum_budget_receipt = match self {
            Self::ForegroundReserved(plan) => plan.quantum_budget_receipt().clone(),
            Self::BackgroundPaced(plan) => plan.quantum_budget_receipt().clone(),
            Self::Escalated(plan) => plan.quantum_budget_receipt().clone(),
            Self::Deferred(_) | Self::Cancelled { .. } => return None,
        };
        Some(ReservedMaintenanceWork::new(
            admitted_work,
            quantum_budget_receipt,
            escalation_decision,
        ))
    }
}
