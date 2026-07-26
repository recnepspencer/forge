use super::fixture::{
    direct_admission_fixture, workflow_admission_fixture, DirectAdmissionFixture,
    FixtureDisposition, WorkflowAdmissionFixture, WORKFLOW_STAGE,
};
use crate::domain_computation::{
    WorthQueryDirectConvergenceIterationOutcome, WorthQueryDirectConvergenceReadmissionOutcome,
    WorthQueryDirectConvergenceYieldOutcome, WorthQueryDirectGraphStepOutcome,
    WorthQueryDirectReadmissionDenialKind, WorthQueryExecutionRuntime,
    WorthQueryExecutionRuntimeInstaller, WorthQueryGraphProviderCallKind,
    WorthQueryManagedGraphCallRequest, WorthQueryWorkflowConvergenceCleanupOutcome,
    WorthQueryWorkflowConvergenceIterationOutcome, WorthQueryWorkflowConvergenceReadmissionOutcome,
    WorthQueryWorkflowConvergenceYieldOutcome, WorthQueryWorkflowGraphStepOutcome,
    WorthQueryWorkflowReadmissionDenialKind,
};

#[test]
fn denied_direct_readmission_rejoins_without_recounting_the_yield() {
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
        Err(_) => panic!("direct convergence authorities must admit"),
    };
    let started = match epoch.begin_iteration(call("direct-readmission-denial")) {
        Ok(started) => started,
        Err(_) => panic!("direct convergence iteration must start"),
    };
    let (pending, active) = started.into_parts();
    let paused = match active.advance() {
        WorthQueryDirectGraphStepOutcome::Continue(paused) => paused,
        _ => panic!("direct provider must expose its yield safe point"),
    };
    let yielded = match pending.admit_yield_outcome(paused.yield_run()) {
        WorthQueryDirectConvergenceYieldOutcome::Yielded(yielded) => yielded,
        _ => panic!("direct convergence yield must admit"),
    };
    let denied = match yielded.readmit_same_runtime(&foreign_runtime(), &bridge) {
        WorthQueryDirectConvergenceReadmissionOutcome::Denied(denied) => denied,
        _ => panic!("foreign Query runtime must deny direct convergence readmission"),
    };
    assert_eq!(
        denied.managed_denial().kind(),
        WorthQueryDirectReadmissionDenialKind::ForeignQueryRuntime
    );
    assert!(denied.readmission_evidence().bridge_counters().is_none());
    let started = match denied
        .into_yielded()
        .readmit_same_runtime(&runtime, &bridge)
    {
        WorthQueryDirectConvergenceReadmissionOutcome::Readmitted(readmitted) => {
            assert_eq!(
                readmitted
                    .readmission_evidence()
                    .bridge_counters()
                    .expect("owner readmission must carry Bridge evidence")
                    .commit_count(),
                1
            );
            readmitted.into_started()
        }
        _ => panic!("owning Query runtime must readmit the retained direct convergence authority"),
    };
    let (pending, active) = started.into_parts();
    let completion = match active.advance() {
        WorthQueryDirectGraphStepOutcome::Completed(completion) => completion,
        _ => panic!("readmitted direct provider must complete"),
    };
    let terminal = match pending.admit_completion(completion) {
        Ok(WorthQueryDirectConvergenceIterationOutcome::Converged(terminal)) => terminal,
        _ => panic!("readmitted direct convergence iteration must converge"),
    };
    assert_eq!(terminal.counters().yield_count(), 1);
    assert_eq!(terminal.counters().readmission_count(), 1);
    assert_eq!(terminal.counters().iteration_count(), 1);
    if terminal.cleanup().is_err() {
        panic!("readmitted direct convergence terminal must retain cleanup authority");
    }
}

#[test]
fn denied_workflow_readmission_rejoins_without_recounting_the_yield() {
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
            Ok(admitted) => admitted,
            Err(_) => panic!("workflow convergence authorities must admit"),
        };
    let epoch = match admitted.start() {
        Ok(epoch) => epoch,
        Err(_) => panic!("workflow convergence epoch must start"),
    };
    let started =
        match epoch.begin_stage_iteration(WORKFLOW_STAGE, call("workflow-readmission-denial")) {
            Ok(started) => started,
            Err(_) => panic!("workflow convergence iteration must start"),
        };
    let (pending, active) = started.into_parts();
    let paused = match active.advance() {
        WorthQueryWorkflowGraphStepOutcome::Continue(paused) => paused,
        _ => panic!("workflow provider must expose its yield safe point"),
    };
    let yielded = match pending.admit_yield_outcome(paused.yield_run()) {
        WorthQueryWorkflowConvergenceYieldOutcome::Yielded(yielded) => yielded,
        _ => panic!("workflow convergence yield must admit"),
    };
    let denied = match yielded.readmit_same_runtime(&foreign_runtime(), &bridge) {
        WorthQueryWorkflowConvergenceReadmissionOutcome::Denied(denied) => denied,
        _ => panic!("foreign Query runtime must deny workflow convergence readmission"),
    };
    assert_eq!(
        denied.managed_denial().kind(),
        WorthQueryWorkflowReadmissionDenialKind::ForeignQueryRuntime
    );
    assert!(denied.readmission_evidence().bridge_counters().is_none());
    let started = match denied
        .into_yielded()
        .readmit_same_runtime(&runtime, &bridge)
    {
        WorthQueryWorkflowConvergenceReadmissionOutcome::Readmitted(readmitted) => {
            assert_eq!(
                readmitted
                    .readmission_evidence()
                    .bridge_counters()
                    .expect("owner workflow readmission must carry Bridge evidence")
                    .commit_count(),
                1
            );
            readmitted.into_started()
        }
        _ => {
            panic!("owning Query runtime must readmit the retained workflow convergence authority")
        }
    };
    let (pending, active) = started.into_parts();
    let completion = match active.advance() {
        WorthQueryWorkflowGraphStepOutcome::Completed(completion) => completion,
        _ => panic!("readmitted workflow provider must complete"),
    };
    let terminal = match pending.admit_completion(completion) {
        Ok(WorthQueryWorkflowConvergenceIterationOutcome::Converged(terminal)) => terminal,
        _ => panic!("readmitted workflow convergence iteration must converge"),
    };
    assert_eq!(terminal.counters().yield_count(), 1);
    assert_eq!(terminal.counters().readmission_count(), 1);
    assert_eq!(terminal.counters().iteration_count(), 1);
    assert!(matches!(
        terminal.cleanup(),
        WorthQueryWorkflowConvergenceCleanupOutcome::Complete(_)
    ));
}

fn foreign_runtime() -> WorthQueryExecutionRuntime {
    WorthQueryExecutionRuntimeInstaller::new()
        .install(
            worth_query_installation::facade::WorthQueryInstallationGeneration::initial(),
            std::iter::empty(),
        )
        .expect("foreign Query runtime must install")
        .into_parts()
        .0
}

fn call(identity: &str) -> WorthQueryManagedGraphCallRequest {
    WorthQueryManagedGraphCallRequest::new(WorthQueryGraphProviderCallKind::Observe, identity)
}
