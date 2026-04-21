use crate::{
    failure::{StoreError, StoreErrorKind},
    maintenance::{
        AdmittedMaintenanceWork, BackgroundPacedMaintenancePlan, DeferredMaintenancePlan,
        EscalatedMaintenancePlan, ForegroundReservationFamily, ForegroundReservationWitness,
        ForegroundReservedMaintenancePlan, MaintenanceCoalescingDecision,
        MaintenanceDebtPressureClass, MaintenanceDeclaration, MaintenanceEscalationDecision,
        MaintenanceEscalationVerdict, MaintenanceExecutionStatus, MaintenanceLaneKey,
        MaintenancePlanFamily, MaintenanceQuantum, MaintenanceReservationFamily,
        MaintenanceResourceBudgetGrant, MaintenanceStarvationStatus, PacingWindow,
        ReservedMaintenanceWork,
    },
};

use super::summaries::{budget_fits, SchedulerAdmissionContext};

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
    fn new(
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
    fn family(&self) -> MaintenancePlanFamily {
        match self {
            Self::ForegroundReserved(_) => MaintenancePlanFamily::ForegroundReserved,
            Self::BackgroundPaced(_) => MaintenancePlanFamily::BackgroundPaced,
            Self::Escalated(_) => MaintenancePlanFamily::Escalated,
            Self::Deferred(_) => MaintenancePlanFamily::Deferred,
            Self::Cancelled { .. } => MaintenancePlanFamily::Cancelled,
        }
    }

    fn reason(&self) -> Option<&str> {
        match self {
            Self::Deferred(plan) => Some(plan.reason()),
            Self::Cancelled { reason } => Some(reason),
            _ => None,
        }
    }

    fn quantum_units(&self) -> Option<u64> {
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

    fn escalation_decision(&self) -> Option<MaintenanceEscalationDecision> {
        match self {
            Self::ForegroundReserved(_) | Self::BackgroundPaced(_) => {
                Some(MaintenanceEscalationDecision::StayBackground)
            }
            Self::Escalated(plan) => Some(plan.escalation_decision()),
            Self::Deferred(_) | Self::Cancelled { .. } => None,
        }
    }

    fn into_reserved_work(
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

pub(crate) fn lower_maintenance_plan(
    admitted_work: &AdmittedMaintenanceWork,
    allow_resume: bool,
    current_status: MaintenanceExecutionStatus,
    resumed_execution: Option<&ResumedExecutionState>,
    context: &SchedulerAdmissionContext,
) -> Result<PlanningDecision, StoreError> {
    let declaration = admitted_work.declaration().clone();
    let descriptor = admitted_work.descriptor().clone();
    let lane_key = descriptor.lane_key();

    if allow_resume && matches!(current_status, MaintenanceExecutionStatus::Started) {
        let resumed = resumed_execution.ok_or_else(|| {
            StoreError::new(
                StoreErrorKind::MaintenanceLifecycleViolation,
                "started maintenance cannot resume without persisted plan family and budget grant",
            )
        })?;
        let quantum_budget_receipt = resumed
            .resource_budget_grant
            .clone()
            .into_quantum_budget_receipt();
        let lowered_plan = match resumed.plan_family {
            MaintenancePlanFamily::ForegroundReserved => {
                let witness = ForegroundReservationWitness::new(
                    ForegroundReservationFamily::Read,
                    descriptor.locality_scope().clone(),
                );
                LoweredMaintenancePlan::ForegroundReserved(ForegroundReservedMaintenancePlan::new(
                    descriptor,
                    witness,
                    quantum_budget_receipt,
                ))
            }
            MaintenancePlanFamily::BackgroundPaced => LoweredMaintenancePlan::BackgroundPaced(
                BackgroundPacedMaintenancePlan::new(descriptor, quantum_budget_receipt),
            ),
            MaintenancePlanFamily::Escalated => {
                let witness = ForegroundReservationWitness::new(
                    ForegroundReservationFamily::Read,
                    descriptor.locality_scope().clone(),
                );
                LoweredMaintenancePlan::Escalated(EscalatedMaintenancePlan::new(
                    descriptor,
                    MaintenanceEscalationDecision::EscalateWithForegroundImpact,
                    Some(witness),
                    quantum_budget_receipt,
                ))
            }
            other => {
                return Err(StoreError::new(
                    StoreErrorKind::MaintenanceLifecycleViolation,
                    format!(
                        "started maintenance cannot resume from persisted plan family {:?}",
                        other
                    ),
                ))
            }
        };
        return Ok(PlanningDecision::new(
            lowered_plan,
            lane_key,
            MaintenanceCoalescingDecision::NotCoalesced,
            None,
            Some(resumed.resource_budget_grant.clone()),
            resumed.starvation_status,
            resumed.escalation_verdict,
            resumed.explicit_global_scope_debt,
        ));
    }

    if descriptor.freshness_window().value() == 0 {
        return Ok(PlanningDecision::new(
            LoweredMaintenancePlan::Cancelled {
                reason: "maintenance descriptor is stale and must be cancelled before execution"
                    .to_string(),
            },
            lane_key,
            MaintenanceCoalescingDecision::CancelledAsSuperseded,
            Some("descriptor freshness window reached zero".to_string()),
            None,
            context.debt_summary.starvation_status(),
            MaintenanceEscalationVerdict::NoEscalation,
            context.debt_summary.explicit_global_scope_debt(),
        ));
    }

    if let Some(max_epoch) = context
        .lane_summary
        .max_supersession_epoch_for(descriptor.equivalence_key())
    {
        if max_epoch > descriptor.supersession_epoch().value() {
            return Ok(PlanningDecision::new(
                LoweredMaintenancePlan::Cancelled {
                    reason: "maintenance work was cancelled because a newer supersession epoch already owns this lane".to_string(),
                },
                lane_key,
                MaintenanceCoalescingDecision::CancelledAsSuperseded,
                Some(format!(
                    "superseded by newer lane member at epoch {max_epoch}"
                )),
                None,
                context.debt_summary.starvation_status(),
                MaintenanceEscalationVerdict::NoEscalation,
                context.debt_summary.explicit_global_scope_debt(),
            ));
        }
    }

    if context
        .lane_summary
        .equivalence_member_count(descriptor.equivalence_key())
        > 1
    {
        let leader_identity = context
            .lane_summary
            .leader_identity_for(descriptor.equivalence_key())
            .unwrap_or(descriptor.work_identity().as_str());
        if leader_identity != descriptor.work_identity().as_str() {
            return Ok(PlanningDecision::new(
                LoweredMaintenancePlan::Cancelled {
                    reason: format!(
                        "maintenance work coalesced behind equivalent lane leader `{leader_identity}`"
                    ),
                },
                lane_key,
                MaintenanceCoalescingDecision::CoalescedWithEquivalentLaneMember,
                Some(format!("coalesced with `{leader_identity}`")),
                None,
                context.debt_summary.starvation_status(),
                MaintenanceEscalationVerdict::NoEscalation,
                context.debt_summary.explicit_global_scope_debt(),
            ));
        }
    }

    let grant = deterministic_budget_grant(&descriptor);
    let fits_budget = budget_fits(descriptor.demand(), &context.resource_budget_summary);
    let starvation_status = if !fits_budget && context.lane_summary.deferred_count() + 1 >= 2 {
        MaintenanceStarvationStatus::DeferredLanePressure
    } else {
        context.debt_summary.starvation_status()
    };
    let should_force_escalation = !fits_budget
        && matches!(
            context.debt_summary.pressure_class(),
            MaintenanceDebtPressureClass::Elevated
        )
        && matches!(
            descriptor.execution_posture(),
            crate::MaintenanceExecutionPosture::ForegroundAware
        );

    if !fits_budget
        && !should_force_escalation
        && !matches!(
            descriptor.escalation_decision(),
            MaintenanceEscalationDecision::EscalateWithForegroundImpact
        )
    {
        return Ok(PlanningDecision::new(
            LoweredMaintenancePlan::Deferred(DeferredMaintenancePlan::new(
                descriptor,
                "maintenance work was deferred because one or more budget dimensions could not be reserved",
            )),
            lane_key,
            MaintenanceCoalescingDecision::NotCoalesced,
            None,
            None,
            starvation_status,
            MaintenanceEscalationVerdict::DeferredForBudgetPressure,
            context.debt_summary.explicit_global_scope_debt(),
        ));
    }

    let explicit_global_scope_debt = context.debt_summary.explicit_global_scope_debt()
        || matches!(
            descriptor.locality_scope(),
            crate::MaintenanceLocalityScope::StoreGlobalLocalityScope
        ) && matches!(
            descriptor.escalation_decision(),
            MaintenanceEscalationDecision::EscalateWithForegroundImpact
        );

    let escalation_decision = if should_force_escalation {
        MaintenanceEscalationDecision::EscalateWithForegroundImpact
    } else {
        descriptor.escalation_decision()
    };
    let escalation_verdict = if should_force_escalation {
        MaintenanceEscalationVerdict::EscalatedForDebtPressure
    } else if !fits_budget {
        MaintenanceEscalationVerdict::DeferredForBudgetPressure
    } else {
        MaintenanceEscalationVerdict::NoEscalation
    };

    let lowered_plan = match escalation_decision {
        MaintenanceEscalationDecision::StayBackground
        | MaintenanceEscalationDecision::PaceUpWithinBackgroundBudget => {
            match descriptor.reservation_family() {
                MaintenanceReservationFamily::Foreground(_) => {
                    let witness = ForegroundReservationWitness::new(
                        ForegroundReservationFamily::Read,
                        descriptor.locality_scope().clone(),
                    );
                    LoweredMaintenancePlan::ForegroundReserved(
                        ForegroundReservedMaintenancePlan::new(
                            descriptor,
                            witness,
                            grant.clone().into_quantum_budget_receipt(),
                        ),
                    )
                }
                MaintenanceReservationFamily::Background(_) => {
                    LoweredMaintenancePlan::BackgroundPaced(BackgroundPacedMaintenancePlan::new(
                        descriptor,
                        grant.clone().into_quantum_budget_receipt(),
                    ))
                }
            }
        }
        MaintenanceEscalationDecision::EscalateWithForegroundImpact => {
            let witness = ForegroundReservationWitness::new(
                ForegroundReservationFamily::Read,
                descriptor.locality_scope().clone(),
            );
            LoweredMaintenancePlan::Escalated(EscalatedMaintenancePlan::new(
                descriptor,
                MaintenanceEscalationDecision::EscalateWithForegroundImpact,
                Some(witness),
                grant.clone().into_quantum_budget_receipt(),
            ))
        }
        MaintenanceEscalationDecision::DeferWithOperatorSignal => {
            LoweredMaintenancePlan::Deferred(DeferredMaintenancePlan::new(
                descriptor,
                "maintenance work was deferred pending an operator-visible signal",
            ))
        }
        MaintenanceEscalationDecision::RejectNewDerivedWork => {
            if matches!(declaration, MaintenanceDeclaration::Rebuild { .. })
                || matches!(declaration, MaintenanceDeclaration::Reclaim { .. })
            {
                LoweredMaintenancePlan::Cancelled {
                    reason:
                        "new derived maintenance work was rejected by the active escalation policy"
                            .to_string(),
                }
            } else {
                return Err(StoreError::new(
                    StoreErrorKind::MaintenanceLifecycleViolation,
                    "reject-new-derived-work escalation was applied to non-derived maintenance",
                ));
            }
        }
    };

    Ok(PlanningDecision::new(
        lowered_plan,
        lane_key,
        MaintenanceCoalescingDecision::NotCoalesced,
        None,
        Some(grant),
        starvation_status,
        escalation_verdict,
        explicit_global_scope_debt,
    ))
}

fn deterministic_budget_grant(
    descriptor: &crate::MaintenanceWorkDescriptor,
) -> MaintenanceResourceBudgetGrant {
    let demand = descriptor.demand();
    let cap = match descriptor.work_class() {
        crate::MaintenanceWorkClass::RetentionAudit => 1,
        crate::MaintenanceWorkClass::CompactionMaintenance => 3,
        crate::MaintenanceWorkClass::DerivedArtifactReclaim
        | crate::MaintenanceWorkClass::AuthoritativeReclaim => 2,
        crate::MaintenanceWorkClass::RetainedRangeRebuild => 2,
        _ => 1,
    };
    let quantum_units = demand
        .predicted_io()
        .units()
        .max(demand.predicted_cpu().units())
        .max(demand.predicted_memory().units())
        .max(demand.predicted_publication().units().max(1))
        .min(cap);
    let pacing_units = quantum_units.max(demand.foreground_latency_guard().units());
    MaintenanceResourceBudgetGrant::new(
        demand.predicted_io(),
        demand.predicted_cpu(),
        demand.predicted_memory(),
        demand.predicted_publication(),
        demand.foreground_latency_guard(),
        MaintenanceQuantum::new(quantum_units),
        PacingWindow::new(pacing_units),
    )
}
