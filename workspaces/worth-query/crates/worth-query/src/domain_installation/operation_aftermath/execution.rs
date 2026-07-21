use std::marker::PhantomData;

use crate::basis_lifecycle::BasisOperationLane;
use crate::runtime::WorthQueryWorkspace;
use worth_proof::TransitionOutcome;

use super::{
    exact_inverse_effect_scope_matches_original, mint_aftermath_relation,
    WorthQueryAdmittedAftermath, WorthQueryAftermathExecutionDenial,
    WorthQueryAftermathExecutionDenialKind, WorthQueryAftermathKind,
    WorthQueryAftermathRelationReceipt, WorthQueryCompensationCapability,
    WorthQueryExactInverseCapability,
};
use crate::domain_installation::{
    WorthQueryCompletedWorkflowTrace, WorthQueryExecutableDomainOperation,
    WorthQueryWorkflowOperation, WorthQueryWorkflowReexecutionStop,
};

type OriginalMarker<OO, OF, OL> = fn() -> (OO, OF, OL);

pub struct WorthQueryExecutedWorkflowAftermath<D, OO, OF, OL, CO, CF, CL>
where
    OL: BasisOperationLane,
    CL: BasisOperationLane,
{
    trace: WorthQueryCompletedWorkflowTrace<D, CO, CF, CL>,
    relation: WorthQueryAftermathRelationReceipt,
    _original: PhantomData<OriginalMarker<OO, OF, OL>>,
}

impl<D, OO, OF, OL, CO, CF, CL> WorthQueryExecutedWorkflowAftermath<D, OO, OF, OL, CO, CF, CL>
where
    OL: BasisOperationLane,
    CL: BasisOperationLane,
{
    pub fn trace(&self) -> &WorthQueryCompletedWorkflowTrace<D, CO, CF, CL> {
        &self.trace
    }
    pub fn relation(&self) -> &WorthQueryAftermathRelationReceipt {
        &self.relation
    }
    pub fn into_trace(self) -> WorthQueryCompletedWorkflowTrace<D, CO, CF, CL> {
        self.trace
    }
}

pub type WorthQueryWorkflowAftermathOutcome<D, OO, OF, OL, CO, CF, CL> = TransitionOutcome<
    WorthQueryExecutedWorkflowAftermath<D, OO, OF, OL, CO, CF, CL>,
    WorthQueryWorkflowReexecutionStop,
    WorthQueryWorkflowReexecutionStop,
    WorthQueryWorkflowReexecutionStop,
    WorthQueryWorkflowReexecutionStop,
    WorthQueryWorkflowReexecutionStop,
>;

macro_rules! aftermath_execution_methods {
    ($capability:ident) => {
        impl<D: 'static, OO, OF, OL, CO: 'static, CF: 'static, CL>
            $capability<D, OO, OF, OL, CO, CF, CL>
        where
            OL: BasisOperationLane,
            CL: BasisOperationLane,
            CO: WorthQueryExecutableDomainOperation<D, CF, Execution = WorthQueryWorkflowOperation>,
        {
            pub fn execute_workflow(
                self,
                workspace: &mut WorthQueryWorkspace,
            ) -> WorthQueryWorkflowAftermathOutcome<D, OO, OF, OL, CO, CF, CL> {
                execute_workflow(self.admitted, workspace)
            }
        }
    };
}

aftermath_execution_methods!(WorthQueryExactInverseCapability);
aftermath_execution_methods!(WorthQueryCompensationCapability);

fn execute_workflow<D: 'static, OO, OF, OL, CO, CF: 'static, CL>(
    admitted: WorthQueryAdmittedAftermath<D, OO, OF, OL, CO, CF, CL>,
    workspace: &mut WorthQueryWorkspace,
) -> WorthQueryWorkflowAftermathOutcome<D, OO, OF, OL, CO, CF, CL>
where
    OL: BasisOperationLane,
    CL: BasisOperationLane,
    CO: WorthQueryExecutableDomainOperation<D, CF, Execution = WorthQueryWorkflowOperation>
        + 'static,
{
    let Some(executor) = admitted.candidate.workflow_executor().cloned() else {
        return TransitionOutcome::Denied(WorthQueryWorkflowReexecutionStop::Aftermath(
            WorthQueryAftermathExecutionDenial::before_execution(
                WorthQueryAftermathExecutionDenialKind::DomainPlanUnavailable,
            ),
        ));
    };
    let Some(intent) = executor.prepare_aftermath_intent(&admitted.original_evidence) else {
        return TransitionOutcome::Denied(WorthQueryWorkflowReexecutionStop::Aftermath(
            WorthQueryAftermathExecutionDenial::before_execution(
                WorthQueryAftermathExecutionDenialKind::DomainPlanUnavailable,
            ),
        ));
    };
    let WorthQueryAdmittedAftermath {
        candidate,
        mut counters,
        proof,
        original_trace_identity,
        kind,
        postcondition,
        original_evidence,
        ..
    } = admitted;
    match candidate.reexecute(intent, workspace) {
        TransitionOutcome::Success(trace) => {
            let candidate_effects = trace
                .stage_receipts()
                .iter()
                .flat_map(|stage| stage.effect_evidence().iter().cloned())
                .collect::<Vec<_>>();
            if kind == WorthQueryAftermathKind::ExactInverse {
                let (scope_matches, checks) = exact_inverse_effect_scope_matches_original(
                    &original_evidence,
                    &candidate_effects,
                );
                counters.candidate_effect_receipt_checks += checks;
                if !scope_matches {
                    return TransitionOutcome::Failed(
                        WorthQueryWorkflowReexecutionStop::Aftermath(
                            WorthQueryAftermathExecutionDenial::after_execution(
                                WorthQueryAftermathExecutionDenialKind::ExactInverseScopeMismatch,
                                trace.identity(),
                                &candidate_effects,
                                kind,
                            ),
                        ),
                    );
                }
            }
            counters.postcondition_verification_checks += 1;
            if !executor.verify_aftermath_postcondition(&original_evidence, &trace.semantics()) {
                return TransitionOutcome::Failed(WorthQueryWorkflowReexecutionStop::Aftermath(
                    WorthQueryAftermathExecutionDenial::after_execution(
                        WorthQueryAftermathExecutionDenialKind::PostconditionNotEstablished,
                        trace.identity(),
                        &candidate_effects,
                        kind,
                    ),
                ));
            }
            let relation = mint_aftermath_relation(
                proof,
                &original_trace_identity,
                kind,
                &postcondition,
                trace.identity(),
                counters,
            );
            TransitionOutcome::Success(WorthQueryExecutedWorkflowAftermath {
                trace,
                relation,
                _original: PhantomData,
            })
        }
        TransitionOutcome::Denied(stop) => {
            TransitionOutcome::Denied(retain_aftermath_failure(stop, kind))
        }
        TransitionOutcome::Deferred(stop) => {
            TransitionOutcome::Deferred(retain_aftermath_failure(stop, kind))
        }
        TransitionOutcome::Stale(stop) => {
            TransitionOutcome::Stale(retain_aftermath_failure(stop, kind))
        }
        TransitionOutcome::RebindRequired(stop) => {
            TransitionOutcome::RebindRequired(retain_aftermath_failure(stop, kind))
        }
        TransitionOutcome::Failed(stop) => {
            TransitionOutcome::Failed(retain_aftermath_failure(stop, kind))
        }
    }
}

fn retain_aftermath_failure(
    stop: WorthQueryWorkflowReexecutionStop,
    attempted_kind: WorthQueryAftermathKind,
) -> WorthQueryWorkflowReexecutionStop {
    if stop.executed_effects().is_empty() {
        stop
    } else {
        WorthQueryWorkflowReexecutionStop::Aftermath(
            WorthQueryAftermathExecutionDenial::from_candidate_stop(stop, attempted_kind),
        )
    }
}
