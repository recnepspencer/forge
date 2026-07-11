use crate::access::planning::S8SelectedAccessPlan;
use crate::access::shape::{S8AccessShape, S8AccessStaleDisposition};
use forge_proof::raw::{
    CheckedReadmitLoweredForExecutionReadyTransition, ContextualTransition,
    LoweredReadmissionReadiness, TransitionOutcome,
};

use super::admitted_counters::S8AdmittedExecutedCounters;
use super::denial::{S8AccessLoweringDeferred, S8AccessLoweringDenied};
use super::executed_evidence::S8ExecutedAccessReceipt;
use super::freshness::{
    readiness_authority, readmission_authority, S8ExecutionReadinessAuthority,
    S8ReadmissionAuthority,
};
use super::freshness::{S8ExecutionReadmissionWitness, S8ExecutionRebindWitness};
use super::lowered_plan::{
    S8AccessLoweringBasis, S8LoweredAccessPayload, S8LoweredAccessReceipt,
    S8RebindRequiredAccessReceipt, S8StaleLoweredAccessReceipt,
};
use super::path_kind::S8AccessPathKind;
use super::ready_plan::S8ExecutionReadyAccessReceipt;
use super::S8AccessLoweringOutcome;
use super::{counter_witness, S8ExecutedCounterWitness};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AccessLoweringFacade;

impl AccessLoweringFacade {
    pub fn lower_selected(&self, selected: S8SelectedAccessPlan) -> S8AccessLoweringOutcome {
        S8AccessLoweringOutcome::lower(S8LoweredAccessReceipt::lower(
            selected,
            path_kind_for(selected),
        ))
    }

    pub fn admit_ready(
        &self,
        lowered: S8LoweredAccessReceipt,
    ) -> super::S8ExecutionReadinessOutcome {
        let selected = lowered.selected();
        if selected.selected_family()
            == crate::strategy::S8LayoutStrategyFamily::BaselineLsmWriteOptimized
            && selected.access_shape().shape() == S8AccessShape::PointLookup
        {
            return super::S8ExecutionReadinessOutcome::deferred(
                S8AccessLoweringDeferred::RuntimeLeaseRequired {
                    basis: lowered.basis(),
                },
            );
        }

        match selected.access_shape().stale_disposition() {
            S8AccessStaleDisposition::ExactOnly => super::S8ExecutionReadinessOutcome::ready(
                S8ExecutionReadyAccessReceipt::admit(lowered),
            ),
            S8AccessStaleDisposition::ExplicitDegradedFallback => {
                super::S8ExecutionReadinessOutcome::stale(lowered.bridge_to_stale())
            }
            S8AccessStaleDisposition::RebindBeforeExecution => {
                super::S8ExecutionReadinessOutcome::rebind_required(lowered.bridge_to_rebind())
            }
        }
    }

    pub fn admit_executed_counters<W: S8ExecutedCounterWitness>(
        &self,
        ready: &S8ExecutionReadyAccessReceipt,
        witness: &W,
    ) -> super::S8ExecutedCounterAdmissionOutcome {
        let expected_plan_binding = ready.selected().budget_receipt().plan_binding();
        let result =
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
                });
        super::S8ExecutedCounterAdmissionOutcome::issue(
            ready.path_kind().is_degraded_exact_scan(),
            result,
        )
    }

    pub fn execute_ready(
        &self,
        ready: S8ExecutionReadyAccessReceipt,
        observed: S8AdmittedExecutedCounters,
    ) -> super::S8ExecutedEvidenceOutcome {
        if observed.basis() != ready.basis() {
            return super::S8ExecutedEvidenceOutcome::denied(
                ready.path_kind().is_degraded_exact_scan(),
                S8AccessLoweringDenied::ObservedCounterBasisMismatch {
                    expected: ready.basis(),
                    actual: observed.basis(),
                    observed,
                },
            );
        }
        super::S8ExecutedEvidenceOutcome::executed(S8ExecutedAccessReceipt::observe(
            ready, observed,
        ))
    }

    pub fn require_rebind(
        &self,
        lowered: S8LoweredAccessReceipt,
    ) -> super::S8StaleReadmissionOutcome {
        super::S8StaleReadmissionOutcome::required(lowered.bridge_to_rebind())
    }

    pub fn rebind_for_execution(
        &self,
        rebind: S8RebindRequiredAccessReceipt,
        witness: S8ExecutionRebindWitness,
    ) -> super::S8StaleReadmissionOutcome {
        if rebind.basis() != witness.basis() {
            return super::S8StaleReadmissionOutcome::denied(
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
            return super::S8StaleReadmissionOutcome::denied(
                S8AccessLoweringDenied::RebindCurrentCoverageMismatch {
                    basis: rebind.basis(),
                    expected: expected_coverage,
                    actual: witness.coverage(),
                },
            );
        }
        super::S8StaleReadmissionOutcome::rebound(rebind.rebind())
    }

    pub fn readmit_stale(
        &self,
        stale: S8StaleLoweredAccessReceipt,
        witness: S8ExecutionReadmissionWitness,
    ) -> super::S8StaleReadmissionOutcome {
        if stale.basis() != witness.basis() {
            return super::S8StaleReadmissionOutcome::denied(
                S8AccessLoweringDenied::ReadmissionWitnessMismatch {
                    basis: stale.basis(),
                    expected: stale.basis(),
                    actual: witness.basis(),
                },
            );
        }
        let expected_planned = stale.selected().planned_counter_envelope().lookup();
        if expected_planned != witness.planned() {
            return super::S8StaleReadmissionOutcome::denied(
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
            return super::S8StaleReadmissionOutcome::denied(
                S8AccessLoweringDenied::ReadmissionCurrentCoverageMismatch {
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
            >::ready(forge_proof::raw::LoweredReadmissionContext::new(
                stale.basis(),
                readmission_authority(),
                "readmitted-ready",
                readiness_authority(),
            )),
        );

        let ready = match ready {
            TransitionOutcome::Success(ready) => ready,
            TransitionOutcome::Denied(denial) => {
                return super::S8StaleReadmissionOutcome::denied(denial);
            }
            TransitionOutcome::Deferred(reason) => {
                return super::S8StaleReadmissionOutcome::deferred(reason);
            }
            TransitionOutcome::Stale(_) => {
                return super::S8StaleReadmissionOutcome::still_stale(stale);
            }
            TransitionOutcome::RebindRequired(_) | TransitionOutcome::Failed(_) => {
                unreachable!("checked lowered readmission cannot rebind or fail")
            }
        };

        super::S8StaleReadmissionOutcome::readmitted(S8ExecutionReadyAccessReceipt::from_recipe(
            ready,
        ))
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
