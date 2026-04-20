use crate::{
    failure::{StoreError, StoreErrorKind},
    maintenance::{
        AdmittedMaintenanceWork, BackgroundPacedMaintenancePlan, DeferredMaintenancePlan,
        EscalatedMaintenancePlan, ForegroundReservationFamily, ForegroundReservationWitness,
        ForegroundReservedMaintenancePlan, MaintenanceDeclaration, MaintenanceEscalationDecision,
        MaintenanceExecutionStatus, MaintenancePlanFamily, MaintenanceQuantum,
        MaintenanceReservationFamily, PacingWindow, QuantumBudgetReceipt,
        ReservedMaintenanceWork,
    },
};

#[derive(Debug, Clone)]
pub(crate) enum LoweredMaintenancePlan {
    ForegroundReserved(ForegroundReservedMaintenancePlan),
    BackgroundPaced(BackgroundPacedMaintenancePlan),
    Escalated(EscalatedMaintenancePlan),
    Deferred(DeferredMaintenancePlan),
    Cancelled { reason: String },
}

impl LoweredMaintenancePlan {
    pub(crate) fn family(&self) -> MaintenancePlanFamily {
        match self {
            Self::ForegroundReserved(_) => MaintenancePlanFamily::ForegroundReserved,
            Self::BackgroundPaced(_) => MaintenancePlanFamily::BackgroundPaced,
            Self::Escalated(_) => MaintenancePlanFamily::Escalated,
            Self::Deferred(_) => MaintenancePlanFamily::Deferred,
            Self::Cancelled { .. } => MaintenancePlanFamily::Cancelled,
        }
    }

    pub(crate) fn reason(&self) -> Option<&str> {
        match self {
            Self::Deferred(plan) => Some(plan.reason()),
            Self::Cancelled { reason } => Some(reason),
            _ => None,
        }
    }

    pub(crate) fn quantum_units(&self) -> Option<u64> {
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

    pub(crate) fn escalation_decision(&self) -> Option<MaintenanceEscalationDecision> {
        match self {
            Self::ForegroundReserved(_) | Self::BackgroundPaced(_) => {
                Some(MaintenanceEscalationDecision::StayBackground)
            }
            Self::Escalated(plan) => Some(plan.escalation_decision()),
            Self::Deferred(_) | Self::Cancelled { .. } => None,
        }
    }

    pub(crate) fn into_reserved_work(
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
) -> Result<LoweredMaintenancePlan, StoreError> {
    let declaration = admitted_work.declaration().clone();
    let descriptor = admitted_work.descriptor().clone();

    if allow_resume && matches!(current_status, MaintenanceExecutionStatus::Started) {
        return Ok(LoweredMaintenancePlan::BackgroundPaced(
            BackgroundPacedMaintenancePlan::new(
                descriptor,
                QuantumBudgetReceipt::new(MaintenanceQuantum::new(1), PacingWindow::new(1)),
            ),
        ));
    }

    if descriptor.freshness_window().value() == 0 {
        return Ok(LoweredMaintenancePlan::Cancelled {
            reason: "maintenance descriptor is stale and must be cancelled before execution"
                .to_string(),
        });
    }

    let quantum_budget_receipt =
        QuantumBudgetReceipt::new(MaintenanceQuantum::new(1), PacingWindow::new(1));

    match descriptor.escalation_decision() {
        MaintenanceEscalationDecision::StayBackground
        | MaintenanceEscalationDecision::PaceUpWithinBackgroundBudget => {
            match descriptor.reservation_family() {
                MaintenanceReservationFamily::Foreground(_) => {
                    let witness = ForegroundReservationWitness::new(
                        ForegroundReservationFamily::Read,
                        descriptor.locality_scope().clone(),
                    );
                    Ok(LoweredMaintenancePlan::ForegroundReserved(
                        ForegroundReservedMaintenancePlan::new(
                            descriptor,
                            witness,
                            quantum_budget_receipt,
                        ),
                    ))
                }
                MaintenanceReservationFamily::Background(_) => Ok(
                    LoweredMaintenancePlan::BackgroundPaced(
                        BackgroundPacedMaintenancePlan::new(descriptor, quantum_budget_receipt),
                    ),
                ),
            }
        }
        MaintenanceEscalationDecision::EscalateWithForegroundImpact => {
            let witness = ForegroundReservationWitness::new(
                ForegroundReservationFamily::Read,
                descriptor.locality_scope().clone(),
            );
            Ok(LoweredMaintenancePlan::Escalated(
                EscalatedMaintenancePlan::new(
                    descriptor,
                    MaintenanceEscalationDecision::EscalateWithForegroundImpact,
                    Some(witness),
                    quantum_budget_receipt,
                ),
            ))
        }
        MaintenanceEscalationDecision::DeferWithOperatorSignal => Ok(
            LoweredMaintenancePlan::Deferred(DeferredMaintenancePlan::new(
                descriptor,
                "maintenance work was deferred pending an operator-visible signal",
            )),
        ),
        MaintenanceEscalationDecision::RejectNewDerivedWork => {
            if matches!(declaration, MaintenanceDeclaration::Rebuild { .. })
                || matches!(declaration, MaintenanceDeclaration::Reclaim { .. })
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
