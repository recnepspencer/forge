use crate::{
    failure::{StoreError, StoreErrorKind},
    maintenance::{
        AdmittedMaintenanceWork, BackgroundPacedMaintenancePlan, DeferredMaintenancePlan,
        EscalatedMaintenancePlan, ForegroundReservationFamily, ForegroundReservationWitness,
        ForegroundReservedMaintenancePlan, MaintenanceCoalescingDecision,
        MaintenanceDebtPressureClass, MaintenanceDeclaration, MaintenanceEscalationDecision,
        MaintenanceEscalationVerdict, MaintenanceExecutionStatus, MaintenanceLaneKey,
        MaintenancePlanFamily, MaintenanceReservationFamily, MaintenanceStarvationStatus,
        MaintenanceWorkDescriptor,
    },
};

use super::super::summaries::{budget_fits, SchedulerAdmissionContext};
use super::budget::deterministic_budget_grant;
use super::decision::{LoweredMaintenancePlan, PlanningDecision, ResumedExecutionState};

pub(crate) fn lower_maintenance_plan(
    admitted_work: &AdmittedMaintenanceWork,
    allow_resume: bool,
    current_status: MaintenanceExecutionStatus,
    resumed_execution: Option<&ResumedExecutionState>,
    context: &SchedulerAdmissionContext,
) -> Result<PlanningDecision, StoreError> {
    let lowering_input = validate_descriptor_and_context(admitted_work, context);

    if let Some(resumed_decision) = resume_existing_plan(
        &lowering_input,
        allow_resume,
        current_status,
        resumed_execution,
    )? {
        return Ok(resumed_decision);
    }

    if let Some(coalescing_decision) = resolve_freshness_and_coalescing(&lowering_input) {
        return Ok(coalescing_decision);
    }

    let budget_resolution = resolve_budget_starvation_and_escalation(&lowering_input);
    let lowered_plan = select_lowered_plan(&lowering_input, &budget_resolution)?;

    Ok(PlanningDecision::new(
        lowered_plan,
        lowering_input.lane_key,
        MaintenanceCoalescingDecision::NotCoalesced,
        None,
        Some(budget_resolution.grant),
        budget_resolution.starvation_status,
        budget_resolution.escalation_verdict,
        budget_resolution.explicit_global_scope_debt,
    ))
}

struct MaintenanceLoweringInput<'a> {
    declaration: MaintenanceDeclaration,
    descriptor: MaintenanceWorkDescriptor,
    lane_key: MaintenanceLaneKey,
    context: &'a SchedulerAdmissionContext,
}

fn validate_descriptor_and_context<'a>(
    admitted_work: &AdmittedMaintenanceWork,
    context: &'a SchedulerAdmissionContext,
) -> MaintenanceLoweringInput<'a> {
    let descriptor = admitted_work.descriptor().clone();
    MaintenanceLoweringInput {
        declaration: admitted_work.declaration().clone(),
        lane_key: descriptor.lane_key(),
        descriptor,
        context,
    }
}

fn resume_existing_plan(
    input: &MaintenanceLoweringInput<'_>,
    allow_resume: bool,
    current_status: MaintenanceExecutionStatus,
    resumed_execution: Option<&ResumedExecutionState>,
) -> Result<Option<PlanningDecision>, StoreError> {
    if !allow_resume || !matches!(current_status, MaintenanceExecutionStatus::Started) {
        return Ok(None);
    }

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
                input.descriptor.locality_scope().clone(),
            );
            LoweredMaintenancePlan::ForegroundReserved(ForegroundReservedMaintenancePlan::new(
                input.descriptor.clone(),
                witness,
                quantum_budget_receipt,
            ))
        }
        MaintenancePlanFamily::BackgroundPaced => LoweredMaintenancePlan::BackgroundPaced(
            BackgroundPacedMaintenancePlan::new(input.descriptor.clone(), quantum_budget_receipt),
        ),
        MaintenancePlanFamily::Escalated => {
            let witness = ForegroundReservationWitness::new(
                ForegroundReservationFamily::Read,
                input.descriptor.locality_scope().clone(),
            );
            LoweredMaintenancePlan::Escalated(EscalatedMaintenancePlan::new(
                input.descriptor.clone(),
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

    Ok(Some(PlanningDecision::new(
        lowered_plan,
        input.lane_key.clone(),
        MaintenanceCoalescingDecision::NotCoalesced,
        None,
        Some(resumed.resource_budget_grant.clone()),
        resumed.starvation_status,
        resumed.escalation_verdict,
        resumed.explicit_global_scope_debt,
    )))
}

enum FreshnessAndCoalescingOutcome {
    Continue,
    Cancelled {
        decision: MaintenanceCoalescingDecision,
        reason: String,
        supersession_source: String,
    },
}

fn resolve_freshness_and_coalescing(
    input: &MaintenanceLoweringInput<'_>,
) -> Option<PlanningDecision> {
    let outcome = if input.descriptor.freshness_window().value() == 0 {
        FreshnessAndCoalescingOutcome::Cancelled {
            decision: MaintenanceCoalescingDecision::CancelledAsSuperseded,
            reason: "maintenance descriptor is stale and must be cancelled before execution"
                .to_string(),
            supersession_source: "descriptor freshness window reached zero".to_string(),
        }
    } else {
        if let Some(max_epoch) = input
            .context
            .lane_summary
            .max_supersession_epoch_for(input.descriptor.equivalence_key())
            .filter(|max_epoch| *max_epoch > input.descriptor.supersession_epoch().value())
        {
            FreshnessAndCoalescingOutcome::Cancelled {
                decision: MaintenanceCoalescingDecision::CancelledAsSuperseded,
                reason: "maintenance work was cancelled because a newer supersession epoch already owns this lane".to_string(),
                supersession_source: format!("superseded by newer lane member at epoch {max_epoch}"),
            }
        } else {
            let equivalent_member_count = input
                .context
                .lane_summary
                .equivalence_member_count(input.descriptor.equivalence_key());
            let leader_identity = input
                .context
                .lane_summary
                .leader_identity_for(input.descriptor.equivalence_key())
                .unwrap_or(input.descriptor.work_identity().as_str());
            if equivalent_member_count > 1
                && leader_identity != input.descriptor.work_identity().as_str()
            {
                FreshnessAndCoalescingOutcome::Cancelled {
                    decision: MaintenanceCoalescingDecision::CoalescedWithEquivalentLaneMember,
                    reason: format!(
                        "maintenance work coalesced behind equivalent lane leader `{leader_identity}`"
                    ),
                    supersession_source: format!("coalesced with `{leader_identity}`"),
                }
            } else {
                FreshnessAndCoalescingOutcome::Continue
            }
        }
    };

    match outcome {
        FreshnessAndCoalescingOutcome::Continue => None,
        FreshnessAndCoalescingOutcome::Cancelled {
            decision,
            reason,
            supersession_source,
        } => Some(PlanningDecision::new(
            LoweredMaintenancePlan::Cancelled { reason },
            input.lane_key.clone(),
            decision,
            Some(supersession_source),
            None,
            input.context.debt_summary.starvation_status(),
            MaintenanceEscalationVerdict::NoEscalation,
            input.context.debt_summary.explicit_global_scope_debt(),
        )),
    }
}

struct BudgetStarvationEscalationResolution {
    grant: crate::MaintenanceResourceBudgetGrant,
    fits_budget: bool,
    starvation_status: MaintenanceStarvationStatus,
    should_force_escalation: bool,
    escalation_decision: MaintenanceEscalationDecision,
    escalation_verdict: MaintenanceEscalationVerdict,
    explicit_global_scope_debt: bool,
}

fn resolve_budget_starvation_and_escalation(
    input: &MaintenanceLoweringInput<'_>,
) -> BudgetStarvationEscalationResolution {
    let grant = deterministic_budget_grant(&input.descriptor);
    let fits_budget = budget_fits(
        input.descriptor.demand(),
        &input.context.resource_budget_summary,
    );
    let starvation_status = if !fits_budget && input.context.lane_summary.deferred_count() + 1 >= 2
    {
        MaintenanceStarvationStatus::DeferredLanePressure
    } else {
        input.context.debt_summary.starvation_status()
    };
    let should_force_escalation = !fits_budget
        && matches!(
            input.context.debt_summary.pressure_class(),
            MaintenanceDebtPressureClass::Elevated
        )
        && matches!(
            input.descriptor.execution_posture(),
            crate::MaintenanceExecutionPosture::ForegroundAware
        );
    let explicit_global_scope_debt = input.context.debt_summary.explicit_global_scope_debt()
        || (matches!(
            input.descriptor.locality_scope(),
            crate::MaintenanceLocalityScope::StoreGlobalLocalityScope
        ) && matches!(
            input.descriptor.escalation_decision(),
            MaintenanceEscalationDecision::EscalateWithForegroundImpact
        ));
    let escalation_decision = if should_force_escalation {
        MaintenanceEscalationDecision::EscalateWithForegroundImpact
    } else {
        input.descriptor.escalation_decision()
    };
    let escalation_verdict = if should_force_escalation {
        MaintenanceEscalationVerdict::EscalatedForDebtPressure
    } else if !fits_budget {
        MaintenanceEscalationVerdict::DeferredForBudgetPressure
    } else {
        MaintenanceEscalationVerdict::NoEscalation
    };

    BudgetStarvationEscalationResolution {
        grant,
        fits_budget,
        starvation_status,
        should_force_escalation,
        escalation_decision,
        escalation_verdict,
        explicit_global_scope_debt,
    }
}

fn select_lowered_plan(
    input: &MaintenanceLoweringInput<'_>,
    resolution: &BudgetStarvationEscalationResolution,
) -> Result<LoweredMaintenancePlan, StoreError> {
    if !resolution.fits_budget
        && !resolution.should_force_escalation
        && !matches!(
            input.descriptor.escalation_decision(),
            MaintenanceEscalationDecision::EscalateWithForegroundImpact
        )
    {
        return Ok(LoweredMaintenancePlan::Deferred(DeferredMaintenancePlan::new(
            input.descriptor.clone(),
            "maintenance work was deferred because one or more budget dimensions could not be reserved",
        )));
    }

    match resolution.escalation_decision {
        MaintenanceEscalationDecision::StayBackground
        | MaintenanceEscalationDecision::PaceUpWithinBackgroundBudget => {
            Ok(match input.descriptor.reservation_family() {
                MaintenanceReservationFamily::Foreground(_) => {
                    let witness = ForegroundReservationWitness::new(
                        ForegroundReservationFamily::Read,
                        input.descriptor.locality_scope().clone(),
                    );
                    LoweredMaintenancePlan::ForegroundReserved(
                        ForegroundReservedMaintenancePlan::new(
                            input.descriptor.clone(),
                            witness,
                            resolution.grant.clone().into_quantum_budget_receipt(),
                        ),
                    )
                }
                MaintenanceReservationFamily::Background(_) => {
                    LoweredMaintenancePlan::BackgroundPaced(BackgroundPacedMaintenancePlan::new(
                        input.descriptor.clone(),
                        resolution.grant.clone().into_quantum_budget_receipt(),
                    ))
                }
            })
        }
        MaintenanceEscalationDecision::EscalateWithForegroundImpact => {
            let witness = ForegroundReservationWitness::new(
                ForegroundReservationFamily::Read,
                input.descriptor.locality_scope().clone(),
            );
            Ok(LoweredMaintenancePlan::Escalated(
                EscalatedMaintenancePlan::new(
                    input.descriptor.clone(),
                    MaintenanceEscalationDecision::EscalateWithForegroundImpact,
                    Some(witness),
                    resolution.grant.clone().into_quantum_budget_receipt(),
                ),
            ))
        }
        MaintenanceEscalationDecision::DeferWithOperatorSignal => Ok(
            LoweredMaintenancePlan::Deferred(DeferredMaintenancePlan::new(
                input.descriptor.clone(),
                "maintenance work was deferred pending an operator-visible signal",
            )),
        ),
        MaintenanceEscalationDecision::RejectNewDerivedWork => {
            if matches!(input.declaration, MaintenanceDeclaration::Rebuild { .. })
                || matches!(input.declaration, MaintenanceDeclaration::Reclaim { .. })
            {
                Ok(LoweredMaintenancePlan::Cancelled {
                    reason:
                        "new derived maintenance work was rejected by the active escalation policy"
                            .to_string(),
                })
            } else {
                Err(StoreError::new(
                    StoreErrorKind::MaintenanceLifecycleViolation,
                    "reject-new-derived-work escalation was applied to non-derived maintenance",
                ))
            }
        }
    }
}
