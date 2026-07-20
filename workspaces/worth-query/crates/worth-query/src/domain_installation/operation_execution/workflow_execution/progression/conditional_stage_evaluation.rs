use crate::basis_lifecycle::BasisOperationLane;

pub(super) enum ConditionalStageStop {
    Deferred(Vec<crate::domain_installation::WorthQueryConditionalProvenance>),
    Denied(super::WorthQueryWorkflowAdvanceDenialKind),
}

pub(super) fn evaluate<D: 'static, O: 'static, F: 'static, L: BasisOperationLane>(
    bound: &crate::domain_installation::WorthQueryBoundDomainOperation<D, O, F, L>,
    workspace: &mut crate::runtime::WorthQueryWorkspace,
    snapshot: &crate::memory_workspace::WorthQuerySnapshotIdentity,
    stage_identity: &str,
    run_identity: &str,
    attempt: u64,
    run_counters: &mut super::WorthQueryWorkflowRunCounters,
) -> Result<Vec<crate::domain_installation::WorthQueryConditionalProvenance>, ConditionalStageStop>
{
    let execution_identity = format!(
        "{}:{}:{}",
        bound.binding_identity(),
        run_identity,
        stage_identity
    );
    let mut counters = crate::domain_installation::WorthQueryOperationExecutionCounters::default();
    let outcome = crate::domain_installation::evaluate_bound_conditionals(
        bound,
        workspace,
        snapshot,
        &execution_identity,
        Some(stage_identity),
        Some(run_identity),
        attempt,
        &mut counters,
    );
    super::workflow_conditional_counters::add_conditional_counters(run_counters, counters);
    match outcome {
        Ok(conditional) => Ok(conditional),
        Err(crate::domain_installation::WorthQueryConditionalEvaluationStop::Deferred(
            conditional,
        )) => Err(ConditionalStageStop::Deferred(conditional)),
        Err(crate::domain_installation::WorthQueryConditionalEvaluationStop::Failed {
            kind,
            ..
        }) => Err(ConditionalStageStop::Denied(
            super::WorthQueryWorkflowAdvanceDenialKind::ConditionalExecution(kind),
        )),
        Err(crate::domain_installation::WorthQueryConditionalEvaluationStop::Reentry(denial)) => {
            Err(ConditionalStageStop::Denied(
                super::WorthQueryWorkflowAdvanceDenialKind::ConditionalReentry(denial),
            ))
        }
    }
}
