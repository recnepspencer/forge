use crate::access_shape::{S8AccessShape, S8AccessStaleDisposition};
use crate::planning::S8SelectedAccessPlan;
use worth_proof::raw::{
    CheckedReadmitLoweredForExecutionReadyTransition, ContextualTransition,
    LoweredReadmissionReadiness, TransitionOutcome,
};

use super::denial::{S8AccessLoweringDeferred, S8AccessLoweringDenied};
use super::executed_evidence::S8ExecutedAccessReceipt;
use super::freshness::{
    readiness_authority, readmission_authority, S8ExecutionReadinessAuthority,
    S8ReadmissionAuthority,
};
use super::admitted_counters::S8AdmittedExecutedCounters;
use super::freshness::{S8ExecutionReadmissionWitness, S8ExecutionRebindWitness};
use super::lowered_plan::{
    S8AccessLoweringBasis, S8LoweredAccessPayload, S8LoweredAccessReceipt,
    S8RebindRequiredAccessReceipt, S8StaleLoweredAccessReceipt,
};
use super::path_kind::S8AccessPathKind;
use super::ready_plan::S8ExecutionReadyAccessReceipt;
use super::{counter_witness, S8ExecutedCounterWitness};

#[derive(Debug, PartialEq, Eq)]
pub enum S8AccessLoweringOutcome {
    Lowered(S8LoweredAccessReceipt),
    Ready(S8ExecutionReadyAccessReceipt),
    Executed(S8ExecutedAccessReceipt),
    Stale(S8StaleLoweredAccessReceipt),
    RebindRequired(S8RebindRequiredAccessReceipt),
    Readmitted(S8ExecutionReadyAccessReceipt),
    Denied(S8AccessLoweringDenied),
    Deferred(S8AccessLoweringDeferred),
}

impl S8AccessLoweringOutcome {
    pub fn spent_cost_receipt(&self) -> super::attempt_cost::S8AccessAttemptCostReceipt {
        match self {
            Self::Lowered(lowered) => super::attempt_cost::S8AccessAttemptCostReceipt::NoExecutionCountersSpent {
                fingerprint: lowered.basis().fingerprint(),
                path_kind: lowered.path_kind(),
            },
            Self::Ready(ready) | Self::Readmitted(ready) => super::attempt_cost::S8AccessAttemptCostReceipt::NoExecutionCountersSpent {
                fingerprint: ready.basis().fingerprint(),
                path_kind: ready.path_kind(),
            },
            Self::Executed(executed) => executed.spent_cost_receipt(),
            Self::Stale(stale) => super::attempt_cost::S8AccessAttemptCostReceipt::NoExecutionCountersSpent {
                fingerprint: stale.basis().fingerprint(),
                path_kind: stale.path_kind(),
            },
            Self::RebindRequired(rebind) => super::attempt_cost::S8AccessAttemptCostReceipt::NoExecutionCountersSpent {
                fingerprint: rebind.basis().fingerprint(),
                path_kind: rebind.path_kind(),
            },
            Self::Denied(denial) => denial.spent_cost_receipt(),
            Self::Deferred(reason) => reason.spent_cost_receipt(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AccessLoweringFacade;

impl AccessLoweringFacade {
    pub fn lower_selected(&self, selected: S8SelectedAccessPlan) -> S8AccessLoweringOutcome {
        S8AccessLoweringOutcome::Lowered(S8LoweredAccessReceipt::lower(
            selected,
            path_kind_for(selected),
        ))
    }

    pub fn admit_ready(&self, lowered: S8LoweredAccessReceipt) -> S8AccessLoweringOutcome {
        let selected = lowered.selected();
        if selected.selected_family()
            == crate::strategy::S8LayoutStrategyFamily::BaselineLsmWriteOptimized
            && selected.access_shape().shape() == S8AccessShape::PointLookup
        {
            return S8AccessLoweringOutcome::Deferred(
                S8AccessLoweringDeferred::RuntimeLeaseRequired {
                    basis: lowered.basis(),
                },
            );
        }

        match selected.access_shape().stale_disposition() {
            S8AccessStaleDisposition::ExactOnly => {
                S8AccessLoweringOutcome::Ready(S8ExecutionReadyAccessReceipt::admit(lowered))
            }
            S8AccessStaleDisposition::ExplicitDegradedFallback => {
                S8AccessLoweringOutcome::Stale(lowered.bridge_to_stale())
            }
            S8AccessStaleDisposition::RebindBeforeExecution => {
                S8AccessLoweringOutcome::RebindRequired(lowered.bridge_to_rebind())
            }
        }
    }

    pub fn admit_executed_counters<W: S8ExecutedCounterWitness>(
        &self,
        ready: &S8ExecutionReadyAccessReceipt,
        witness: &W,
    ) -> Result<S8AdmittedExecutedCounters, S8AccessLoweringDenied> {
        let expected_plan_binding = ready.selected().budget_receipt().plan_binding();
        counter_witness::admit_execution_witness(ready.basis(), expected_plan_binding, witness)
            .map(S8AdmittedExecutedCounters::new)
            .map_err(|observed| {
                if witness.plan_binding() != expected_plan_binding {
                    return S8AccessLoweringDenied::ExecutedCounterWitnessPlanBindingMismatch {
                        expected: ready.basis(),
                        expected_plan_binding,
                        actual_plan_binding: witness.plan_binding(),
                        observed,
                    };
                }
                S8AccessLoweringDenied::ExecutedCounterWitnessPathMismatch {
                expected: ready.basis(),
                actual_path_kind: observed.basis().path_kind(),
                observed,
                }
            })
    }

    pub fn execute_ready(
        &self,
        ready: S8ExecutionReadyAccessReceipt,
        observed: S8AdmittedExecutedCounters,
    ) -> S8AccessLoweringOutcome {
        if observed.basis() != ready.basis() {
            return S8AccessLoweringOutcome::Denied(
                S8AccessLoweringDenied::ObservedCounterBasisMismatch {
                    expected: ready.basis(),
                    actual: observed.basis(),
                    observed,
                },
            );
        }
        S8AccessLoweringOutcome::Executed(S8ExecutedAccessReceipt::observe(ready, observed))
    }

    pub fn require_rebind(&self, lowered: S8LoweredAccessReceipt) -> S8AccessLoweringOutcome {
        S8AccessLoweringOutcome::RebindRequired(lowered.bridge_to_rebind())
    }

    pub fn rebind_for_execution(
        &self,
        rebind: S8RebindRequiredAccessReceipt,
        witness: S8ExecutionRebindWitness,
    ) -> S8AccessLoweringOutcome {
        if rebind.basis() != witness.basis() {
            return S8AccessLoweringOutcome::Denied(
                S8AccessLoweringDenied::RebindWitnessMismatch {
                    basis: rebind.basis(),
                    expected: rebind.basis(),
                    actual: witness.basis(),
                },
            );
        }
        let expected_coverage = rebind
            .selected()
            .access_shape()
            .coverage()
            .expect("rebind-required lowered access retains declared coverage")
            .require_exact()
            .expect("rebind-required lowered access retains exact declared coverage");
        if expected_coverage != witness.coverage() {
            return S8AccessLoweringOutcome::Denied(
                S8AccessLoweringDenied::CurrentCoverageMismatch {
                    basis: rebind.basis(),
                    expected: expected_coverage,
                    actual: witness.coverage(),
                },
            );
        }
        S8AccessLoweringOutcome::Lowered(rebind.rebind())
    }

    pub fn readmit_stale(
        &self,
        stale: S8StaleLoweredAccessReceipt,
        witness: S8ExecutionReadmissionWitness,
    ) -> S8AccessLoweringOutcome {
        if stale.basis() != witness.basis() {
            return S8AccessLoweringOutcome::Denied(
                S8AccessLoweringDenied::ReadmissionWitnessMismatch {
                    basis: stale.basis(),
                    expected: stale.basis(),
                    actual: witness.basis(),
                },
            );
        }
        let expected_planned = stale.selected().planned_counter_envelope().lookup();
        if expected_planned != witness.planned() {
            return S8AccessLoweringOutcome::Denied(
                S8AccessLoweringDenied::ReadmissionPlannedCountersMismatch {
                    basis: stale.basis(),
                    expected: expected_planned,
                    actual: witness.planned(),
                },
            );
        }
        let expected_coverage = stale
            .selected()
            .access_shape()
            .coverage()
            .expect("stale lowered access retains declared coverage")
            .require_exact()
            .expect("stale lowered access retains exact declared coverage");
        if expected_coverage != witness.coverage() {
            return S8AccessLoweringOutcome::Denied(
                S8AccessLoweringDenied::CurrentCoverageMismatch {
                    basis: stale.basis(),
                    expected: expected_coverage,
                    actual: witness.coverage(),
                },
            );
        }

        let ready = CheckedReadmitLoweredForExecutionReadyTransition.transition(
            stale.recipe(),
            LoweredReadmissionReadiness::<
                S8LoweredAccessPayload,
                S8AccessLoweringBasis,
                S8AccessLoweringBasis,
                S8ReadmissionAuthority,
                &'static str,
                S8ExecutionReadinessAuthority,
                S8AccessLoweringDenied,
                S8AccessLoweringDeferred,
                S8AccessLoweringDeferred,
            >::ready(worth_proof::raw::LoweredReadmissionContext::new(
                stale.basis(),
                readmission_authority(),
                "readmitted-ready",
                readiness_authority(),
            )),
        );

        let ready = match ready {
            TransitionOutcome::Success(ready) => ready,
            TransitionOutcome::Denied(denial) => {
                return S8AccessLoweringOutcome::Denied(denial);
            }
            TransitionOutcome::Deferred(reason) => {
                return S8AccessLoweringOutcome::Deferred(reason);
            }
            TransitionOutcome::Stale(_) => {
                return S8AccessLoweringOutcome::Stale(stale);
            }
            TransitionOutcome::RebindRequired(_) | TransitionOutcome::Failed(_) => {
                unreachable!("checked lowered readmission cannot rebind or fail")
            }
        };

        S8AccessLoweringOutcome::Readmitted(S8ExecutionReadyAccessReceipt::from_recipe(ready))
    }
}

const fn path_kind_for(selected: S8SelectedAccessPlan) -> S8AccessPathKind {
    match selected.selected_family() {
        crate::strategy::S8LayoutStrategyFamily::BaselineBTreeRange => {
            S8AccessPathKind::BaselineBTreeRead(selected.access_shape().detail())
        }
        crate::strategy::S8LayoutStrategyFamily::BaselineLsmWriteOptimized => {
            S8AccessPathKind::BaselineLsmRead(selected.access_shape().detail())
        }
        crate::strategy::S8LayoutStrategyFamily::ExactScan => {
            S8AccessPathKind::ExactDegradedScan(selected.access_shape().detail())
        }
        _ => S8AccessPathKind::ExactForegroundRead(selected.access_shape().detail()),
    }
}

pub const fn access_lowering() -> AccessLoweringFacade {
    AccessLoweringFacade
}
