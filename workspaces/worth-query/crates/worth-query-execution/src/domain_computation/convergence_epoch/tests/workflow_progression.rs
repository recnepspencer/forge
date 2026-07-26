use super::fixture::{workflow_epoch_fixture, FixtureDisposition, WORKFLOW_STAGE};
use crate::domain_computation::{
    WorthQueryConvergenceEpochDenialKind, WorthQueryConvergenceIterationStartFailureKind,
    WorthQueryConvergenceTerminalKind, WorthQueryGraphProviderCallKind,
    WorthQueryManagedGraphCallRequest, WorthQueryManagedRunTerminalKind,
    WorthQueryManagedStepContractDenialKind, WorthQueryWorkflowConvergenceCleanupOutcome,
    WorthQueryWorkflowConvergenceIterationOutcome,
    WorthQueryWorkflowGraphExecutionStartFailureKind, WorthQueryWorkflowGraphStepOutcome,
};

#[test]
fn real_installed_workflow_seals_its_evidence_stage_and_converges() {
    let epoch = workflow_epoch_fixture(FixtureDisposition::Converged);
    let rejection = match epoch.begin_stage_iteration(
        "foreign-stage",
        WorthQueryManagedGraphCallRequest::new(
            WorthQueryGraphProviderCallKind::Observe,
            "wrong-workflow-stage",
        ),
    ) {
        Ok(_) => panic!("a stage not sealed by convergence admission entered iteration"),
        Err(rejection) => rejection,
    };
    assert_eq!(
        rejection.denial().kind(),
        WorthQueryConvergenceEpochDenialKind::WorkflowEvidenceStageMismatch
    );
    let epoch = rejection.into_epoch();
    let started = match epoch.begin_stage_iteration(
        WORKFLOW_STAGE,
        WorthQueryManagedGraphCallRequest::new(
            WorthQueryGraphProviderCallKind::Observe,
            "workflow-convergence-iteration",
        ),
    ) {
        Ok(started) => started,
        Err(_) => panic!("the sealed workflow evidence stage must start iteration"),
    };
    let (pending, active) = started.into_parts();
    let completion = match active.advance() {
        WorthQueryWorkflowGraphStepOutcome::Completed(completion) => completion,
        _ => panic!("single-step workflow fixture provider must complete"),
    };
    let outcome = match pending.admit_completion(completion) {
        Ok(outcome) => outcome,
        Err(_) => panic!("exact workflow completion must rejoin the pending epoch"),
    };
    let terminal = match outcome {
        WorthQueryWorkflowConvergenceIterationOutcome::Converged(terminal) => terminal,
        _ => panic!("installed workflow convergence decision must remain distinct"),
    };
    assert_eq!(
        terminal.kind(),
        WorthQueryConvergenceTerminalKind::Converged
    );
    assert_eq!(terminal.incumbents().len(), 1);
    assert_eq!(
        terminal.incumbents()[0].domain_evidence().stage_identity(),
        Some(WORKFLOW_STAGE)
    );
    assert_eq!(
        terminal.incumbents()[0]
            .domain_evidence()
            .output_occurrence_identity(),
        "candidate-1"
    );
    let cleanup = terminal.cleanup();
    assert_eq!(cleanup.counters().iteration_count(), 1);
    assert_eq!(cleanup.counters().provider_work_unit_count(), 1);
    assert!(matches!(
        cleanup,
        WorthQueryWorkflowConvergenceCleanupOutcome::Complete(_)
    ));
}

#[test]
fn incompatible_stage_queue_contract_denies_before_iteration_and_terminates_cleanly() {
    let epoch = workflow_epoch_fixture(FixtureDisposition::StageQueueContractMismatch);
    let rejection = match epoch.begin_stage_iteration(
        WORKFLOW_STAGE,
        WorthQueryManagedGraphCallRequest::new(
            WorthQueryGraphProviderCallKind::Project,
            "stage-queue-contract-mismatch",
        ),
    ) {
        Ok(_) => panic!("a stage queue contract wider than Signal entered convergence iteration"),
        Err(rejection) => rejection,
    };
    assert_eq!(rejection.denial().counters().iteration_count(), 0);

    let (denial, outcome) = rejection.terminate().into_parts();
    assert_eq!(
        denial.kind(),
        WorthQueryConvergenceEpochDenialKind::ManagedIterationStart(
            WorthQueryConvergenceIterationStartFailureKind::Workflow(
                WorthQueryWorkflowGraphExecutionStartFailureKind::StepContract(
                    WorthQueryManagedStepContractDenialKind::QueueDepthExceeded,
                ),
            ),
        )
    );
    let terminal = match outcome {
        WorthQueryWorkflowConvergenceIterationOutcome::Indeterminate(terminal) => terminal,
        _ => panic!("an irrecoverable managed start denial must fail closed as indeterminate"),
    };
    assert_eq!(
        terminal.managed_terminal().kind(),
        WorthQueryManagedRunTerminalKind::Failed
    );
    assert_eq!(
        terminal
            .managed_terminal()
            .provider_work()
            .provider_step_attempt_count(),
        0
    );
    assert_eq!(
        terminal
            .managed_terminal()
            .provider_work()
            .completed_work_units(),
        0
    );
    let cleanup = terminal.cleanup();
    assert!(matches!(
        cleanup,
        WorthQueryWorkflowConvergenceCleanupOutcome::Complete(_)
    ));
}
