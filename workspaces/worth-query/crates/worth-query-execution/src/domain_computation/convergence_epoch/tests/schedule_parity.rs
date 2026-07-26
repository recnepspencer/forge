use super::fixture::{
    direct_admission_fixture, workflow_admission_fixture, workflow_epoch_fixture,
    DirectAdmissionFixture, FixtureDisposition, WorkflowAdmissionFixture, WORKFLOW_STAGE,
};
use super::terminal_fixture::{converged_terminal, workflow_converged_terminal};
use crate::domain_computation::{
    WorthQueryConverged, WorthQueryDirectConvergenceIterationOutcome,
    WorthQueryDirectConvergenceReadmissionOutcome, WorthQueryDirectConvergenceYieldOutcome,
    WorthQueryDirectGraphStepOutcome, WorthQueryGraphProviderCallKind,
    WorthQueryManagedGraphCallRequest, WorthQueryWorkflowConvergenceCleanupOutcome,
    WorthQueryWorkflowConvergenceIterationOutcome, WorthQueryWorkflowConvergenceReadmissionOutcome,
    WorthQueryWorkflowConvergenceTerminal, WorthQueryWorkflowConvergenceYieldOutcome,
    WorthQueryWorkflowGraphStepOutcome,
};

#[test]
fn same_runtime_yield_and_resume_preserve_the_semantic_convergence_result() {
    let ordinary = converged_terminal();
    let DirectAdmissionFixture {
        runtime,
        operation,
        alternate_basis_operation: _,
        contract,
        managed,
        graph,
        bridge,
    } = direct_admission_fixture(FixtureDisposition::YieldThenConverged);
    let epoch = match runtime.admit_direct_convergence_epoch(&operation, contract, managed, graph) {
        Ok(epoch) => epoch.start(),
        Err(_) => panic!("yield convergence authorities must admit"),
    };
    let started = match epoch.begin_iteration(WorthQueryManagedGraphCallRequest::new(
        WorthQueryGraphProviderCallKind::Observe,
        "yielded-convergence-iteration",
    )) {
        Ok(started) => started,
        Err(_) => panic!("yield convergence iteration must start"),
    };
    let (pending, active) = started.into_parts();
    let paused = match active.advance() {
        WorthQueryDirectGraphStepOutcome::Continue(paused) => paused,
        _ => panic!("yield provider must expose its installed safe point"),
    };
    let yielded = match pending.admit_yield_outcome(paused.yield_run()) {
        WorthQueryDirectConvergenceYieldOutcome::Yielded(yielded) => yielded,
        _ => panic!("eligible convergence iteration must preserve yielded authority"),
    };
    assert_eq!(yielded.yielded_run().checkpoint().retained_bytes(), 1);
    let resumed = match yielded.readmit_same_runtime(&runtime, &bridge) {
        WorthQueryDirectConvergenceReadmissionOutcome::Readmitted(started) => started,
        _ => panic!("same-runtime convergence readmission must restore the provider"),
    };
    let (pending, active) = resumed.into_parts();
    let completion = match active.advance() {
        WorthQueryDirectGraphStepOutcome::Completed(completion) => completion,
        _ => panic!("restored convergence provider must complete"),
    };
    let outcome = match pending.admit_completion(completion) {
        Ok(outcome) => outcome,
        Err(_) => panic!("restored completion must rejoin its epoch"),
    };
    let resumed = match outcome {
        WorthQueryDirectConvergenceIterationOutcome::Converged(terminal) => terminal,
        _ => panic!("restored installed comparator must converge"),
    };

    assert_eq!(ordinary.kind(), resumed.kind());
    assert_eq!(
        ordinary.latest_report().unwrap().decision(),
        resumed.latest_report().unwrap().decision()
    );
    assert_eq!(
        ordinary.latest_report().unwrap().domain_work(),
        resumed.latest_report().unwrap().domain_work()
    );
    assert_eq!(
        ordinary.incumbents()[0].occurrence_identity(),
        resumed.incumbents()[0].occurrence_identity()
    );
    assert_eq!(
        ordinary.incumbents()[0].state_identity(),
        resumed.incumbents()[0].state_identity()
    );
    assert_eq!(resumed.counters().yield_count(), 1);
    assert_eq!(resumed.counters().resume_count(), 1);
    assert_eq!(resumed.counters().iteration_count(), 1);
    assert_eq!(resumed.counters().provider_work_unit_count(), 2);
    assert!(!ordinary
        .managed_terminal()
        .provider_work()
        .checkpoint_available());
    assert!(!resumed
        .managed_terminal()
        .provider_work()
        .checkpoint_available());
    assert_eq!(
        resumed
            .managed_terminal()
            .provider_work()
            .checkpoint_available_observation_count(),
        1
    );
    assert_eq!(
        resumed.managed_terminal().provider_work().retained_bytes(),
        0
    );
    assert_eq!(
        resumed
            .managed_terminal()
            .provider_work()
            .retained_artifact_count(),
        0
    );
    if ordinary.cleanup().is_err() || resumed.cleanup().is_err() {
        panic!("both schedule variants must retain cleanup authority");
    }
}

#[test]
fn workflow_yield_and_resume_preserve_the_semantic_convergence_result() {
    let ordinary = workflow_converged_terminal();
    let WorkflowAdmissionFixture {
        runtime,
        operation,
        contract,
        managed,
        graph,
        bridge,
    } = workflow_admission_fixture(FixtureDisposition::YieldThenConverged);
    let admitted =
        match runtime.admit_workflow_convergence_epoch(&operation, contract, managed, graph) {
            Ok(epoch) => epoch,
            Err(_) => panic!("yield-capable workflow authorities must admit"),
        };
    let epoch = match admitted.start() {
        Ok(epoch) => epoch,
        Err(_) => panic!("yield-capable workflow epoch must start"),
    };
    let started = match epoch.begin_stage_iteration(
        WORKFLOW_STAGE,
        WorthQueryManagedGraphCallRequest::new(
            WorthQueryGraphProviderCallKind::Observe,
            "yielded-workflow-convergence-iteration",
        ),
    ) {
        Ok(started) => started,
        Err(_) => panic!("yield-capable workflow iteration must start"),
    };
    let (pending, active) = started.into_parts();
    let paused = match active.advance() {
        WorthQueryWorkflowGraphStepOutcome::Continue(paused) => paused,
        _ => panic!("workflow provider must expose its installed safe point"),
    };
    let yielded = match pending.admit_yield_outcome(paused.yield_run()) {
        WorthQueryWorkflowConvergenceYieldOutcome::Yielded(yielded) => yielded,
        _ => panic!("eligible workflow iteration must preserve yielded authority"),
    };
    assert_eq!(yielded.yielded_run().checkpoint().retained_bytes(), 1);
    let resumed = match yielded.readmit_same_runtime(&runtime, &bridge) {
        WorthQueryWorkflowConvergenceReadmissionOutcome::Readmitted(started) => started,
        _ => panic!("same-runtime workflow readmission must restore the provider"),
    };
    let (pending, active) = resumed.into_parts();
    let completion = match active.advance() {
        WorthQueryWorkflowGraphStepOutcome::Completed(completion) => completion,
        _ => panic!("restored workflow provider must complete"),
    };
    let outcome = match pending.admit_completion(completion) {
        Ok(outcome) => outcome,
        Err(_) => panic!("restored workflow completion must rejoin its epoch"),
    };
    let resumed = match outcome {
        WorthQueryWorkflowConvergenceIterationOutcome::Converged(terminal) => terminal,
        _ => panic!("restored installed workflow comparator must converge"),
    };

    assert_eq!(ordinary.kind(), resumed.kind());
    assert_eq!(
        ordinary.latest_report().unwrap().decision(),
        resumed.latest_report().unwrap().decision()
    );
    assert_eq!(
        ordinary.latest_report().unwrap().domain_work(),
        resumed.latest_report().unwrap().domain_work()
    );
    assert_eq!(
        ordinary.incumbents()[0].occurrence_identity(),
        resumed.incumbents()[0].occurrence_identity()
    );
    assert_eq!(resumed.counters().yield_count(), 1);
    assert_eq!(resumed.counters().resume_count(), 1);
    assert_eq!(resumed.counters().iteration_count(), 1);
    assert_eq!(resumed.counters().provider_work_unit_count(), 2);
    assert!(!ordinary
        .managed_terminal()
        .provider_work()
        .checkpoint_available());
    assert!(!resumed
        .managed_terminal()
        .provider_work()
        .checkpoint_available());
    assert_eq!(
        resumed
            .managed_terminal()
            .provider_work()
            .checkpoint_available_observation_count(),
        1
    );
    assert_eq!(
        resumed.managed_terminal().provider_work().retained_bytes(),
        0
    );
    assert_eq!(
        resumed
            .managed_terminal()
            .provider_work()
            .retained_artifact_count(),
        0
    );
    let ordinary_cleanup = ordinary.cleanup();
    let resumed_cleanup = resumed.cleanup();
    assert!(matches!(
        ordinary_cleanup,
        WorthQueryWorkflowConvergenceCleanupOutcome::Complete(_)
    ));
    assert!(matches!(
        resumed_cleanup,
        WorthQueryWorkflowConvergenceCleanupOutcome::Complete(_)
    ));
}

#[test]
fn admitted_chunk_schedule_preserves_the_semantic_convergence_result() {
    let ordinary = workflow_converged_terminal();
    for width in [1, 8] {
        let chunked = chunked_converged_terminal(width);
        assert_eq!(ordinary.kind(), chunked.kind());
        assert_eq!(
            ordinary.latest_report().unwrap().decision(),
            chunked.latest_report().unwrap().decision()
        );
        assert_eq!(
            ordinary.latest_report().unwrap().domain_work(),
            chunked.latest_report().unwrap().domain_work()
        );
        assert_eq!(
            ordinary.incumbents()[0].state_identity(),
            chunked.incumbents()[0].state_identity()
        );
        assert_eq!(chunked.counters().provider_work_unit_count(), 1);
        assert!(matches!(
            chunked.cleanup(),
            WorthQueryWorkflowConvergenceCleanupOutcome::Complete(_)
        ));
    }
    assert!(matches!(
        ordinary.cleanup(),
        WorthQueryWorkflowConvergenceCleanupOutcome::Complete(_)
    ));
}

fn chunked_converged_terminal(
    width: usize,
) -> WorthQueryWorkflowConvergenceTerminal<WorthQueryConverged> {
    let epoch = workflow_epoch_fixture(FixtureDisposition::ChunkedConverged(width));
    let started = match epoch.begin_stage_iteration(
        WORKFLOW_STAGE,
        WorthQueryManagedGraphCallRequest::new(
            WorthQueryGraphProviderCallKind::Project,
            format!("chunked-workflow-convergence-iteration-{width}"),
        ),
    ) {
        Ok(started) => started,
        Err(_) => panic!("chunked workflow iteration must start"),
    };
    let (pending, active) = started.into_parts();
    let chunk = match active.advance() {
        WorthQueryWorkflowGraphStepOutcome::ChunkReady(chunk) => chunk,
        _ => panic!("chunked workflow provider must expose its bounded projection"),
    };
    assert_eq!(
        chunk.queue_depth(),
        u64::try_from(width).expect("fixture chunk width must fit the queue counter")
    );
    assert_eq!(chunk.queue_capacity(), 8);
    let completion = match chunk.acknowledge() {
        WorthQueryWorkflowGraphStepOutcome::Completed(completion) => completion,
        _ => panic!("acknowledged exact-capacity chunk must complete"),
    };
    let outcome = match pending.admit_completion(completion) {
        Ok(outcome) => outcome,
        Err(_) => panic!("chunked workflow completion must rejoin its epoch"),
    };
    match outcome {
        WorthQueryWorkflowConvergenceIterationOutcome::Converged(terminal) => terminal,
        _ => panic!("chunk schedule must preserve convergence"),
    }
}
