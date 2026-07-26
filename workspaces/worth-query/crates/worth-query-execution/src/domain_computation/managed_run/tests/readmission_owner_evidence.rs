use super::yield_fixture::YieldProvider;
use super::*;
use crate::domain_computation::{
    WorthQueryDirectGraphStepOutcome, WorthQueryDirectReadmissionDenialKind,
    WorthQueryDirectReadmissionOutcome, WorthQueryReadmissionEvidence,
    WorthQueryWorkflowGraphStepOutcome, WorthQueryWorkflowReadmissionDenialKind,
    WorthQueryWorkflowReadmissionOutcome,
};

#[test]
fn successful_direct_readmission_carries_exact_query_and_bridge_work() {
    let (yielded, bridge, runtime) = super::readmission_direct::yielded_direct();
    let readmitted = match yielded.readmit_same_runtime(&runtime, &bridge) {
        WorthQueryDirectReadmissionOutcome::Readmitted(readmitted) => readmitted,
        _ => panic!("owner runtime must readmit the direct yielded authority"),
    };

    assert_committed_owner_work(readmitted.readmission_evidence(), 0);
    let terminal = match readmitted.into_active().abandon() {
        WorthQueryDirectGraphStepOutcome::Failed(terminal) => terminal,
        _ => panic!("readmitted direct execution must terminalize"),
    };
    terminal
        .cleanup()
        .expect("readmitted direct execution must release its authorities");
}

#[test]
fn successful_workflow_readmission_carries_exact_query_and_bridge_work() {
    let (yielded, bridge, runtime, old_producer) =
        super::readmission_workflow::yielded_workflow(YieldProvider::installed(7));
    let readmitted = match yielded.readmit_same_runtime(&runtime, &bridge) {
        WorthQueryWorkflowReadmissionOutcome::Readmitted(readmitted) => readmitted,
        _ => panic!("owner runtime must readmit the workflow yielded authority"),
    };

    assert_committed_owner_work(readmitted.readmission_evidence(), 1);
    drop(old_producer);
    let terminal = match readmitted.into_active().abandon() {
        WorthQueryWorkflowGraphStepOutcome::Failed(terminal) => terminal,
        _ => panic!("readmitted workflow execution must terminalize"),
    };
    assert!(matches!(
        terminal.cleanup(),
        WorthQueryWorkflowRunCleanupOutcome::Complete(_)
    ));
}

#[test]
fn foreign_direct_call_denies_before_fresh_query_or_bridge_authority() {
    let (mut yielded, bridge, runtime) = super::readmission_direct::yielded_direct();
    let (mut foreign_yielded, _foreign_bridge, _foreign_runtime) =
        super::readmission_direct::yielded_direct_for_binding("foreign-managed-graph-binding");
    std::mem::swap(
        &mut yielded.execution.call,
        &mut foreign_yielded.execution.call,
    );

    let denied = match yielded.readmit_same_runtime(&runtime, &bridge) {
        WorthQueryDirectReadmissionOutcome::Denied(denied) => denied,
        _ => panic!("foreign provider call must deny during effect-free preflight"),
    };
    assert_eq!(
        denied.kind(),
        WorthQueryDirectReadmissionDenialKind::ProviderCallBindingDenied
    );
    assert_zero_fresh_work(denied.readmission_evidence());
    complete_direct_yield_cleanup(denied.into_yielded());
    complete_direct_yield_cleanup(foreign_yielded);
}

#[test]
fn workflow_call_without_stage_denies_before_fresh_query_or_bridge_authority() {
    let (mut yielded, bridge, runtime, old_producer) =
        super::readmission_workflow::yielded_workflow(YieldProvider::installed(7));
    let (mut direct_yielded, _direct_bridge, _direct_runtime) =
        super::readmission_direct::yielded_direct();
    std::mem::swap(
        &mut yielded.execution.call,
        &mut direct_yielded.execution.call,
    );

    let denied = match yielded.readmit_same_runtime(&runtime, &bridge) {
        WorthQueryWorkflowReadmissionOutcome::Denied(denied) => denied,
        _ => panic!("workflow call without a stage must deny during effect-free preflight"),
    };
    assert_eq!(
        denied.kind(),
        WorthQueryWorkflowReadmissionDenialKind::WorkflowStageResourcesUnavailable
    );
    assert_zero_fresh_work(denied.readmission_evidence());
    drop(old_producer);
    complete_workflow_yield_cleanup(denied.into_yielded());
    complete_direct_yield_cleanup(direct_yielded);
}

#[test]
fn workflow_call_for_unreserved_stage_denies_before_fresh_query_or_bridge_authority() {
    let (mut yielded, bridge, runtime, old_producer) =
        super::readmission_workflow::yielded_workflow(YieldProvider::installed(7));
    let (mut alternate_yielded, _alternate_bridge, _alternate_runtime, alternate_producer) =
        super::readmission_workflow::yielded_workflow_for_stage(
            YieldProvider::installed(7),
            "alternate-stage",
        );
    std::mem::swap(
        &mut yielded.execution.call,
        &mut alternate_yielded.execution.call,
    );

    let denied = match yielded.readmit_same_runtime(&runtime, &bridge) {
        WorthQueryWorkflowReadmissionOutcome::Denied(denied) => denied,
        _ => panic!("unreserved workflow stage must deny during effect-free preflight"),
    };
    assert_eq!(
        denied.kind(),
        WorthQueryWorkflowReadmissionDenialKind::WorkflowStageResourcesUnavailable
    );
    assert_zero_fresh_work(denied.readmission_evidence());
    drop((old_producer, alternate_producer));
    complete_workflow_yield_cleanup(denied.into_yielded());
    complete_workflow_yield_cleanup(alternate_yielded);
}

fn assert_committed_owner_work(evidence: WorthQueryReadmissionEvidence, artifact_attempts: usize) {
    let query = evidence.query_counters();
    assert_eq!(query.preflight_check_count(), 1);
    assert_eq!(query.fresh_resource_attempt_count(), 1);
    assert_eq!(query.bridge_readmission_attempt_count(), 1);
    assert_eq!(query.provider_restore_attempt_count(), 1);
    assert_eq!(query.artifact_generation_attempt_count(), artifact_attempts);
    assert_eq!(query.artifact_generation_commit_count(), artifact_attempts);
    assert_eq!(query.committed_attempt_count(), 1);

    let bridge = evidence
        .bridge_counters()
        .expect("successful readmission must carry Bridge owner evidence");
    assert_eq!(bridge.preflight_check_count(), 1);
    assert_eq!(bridge.reservation_check_count(), 1);
    assert_eq!(bridge.signal_attempt_admission_count(), 1);
    assert_eq!(bridge.signal_attempt_check_count(), 1);
    assert_eq!(bridge.signal_queue_binding_count(), 1);
    assert_eq!(bridge.abort_count(), 0);
    assert_eq!(bridge.commit_count(), 1);
}

fn assert_zero_fresh_work(evidence: WorthQueryReadmissionEvidence) {
    let query = evidence.query_counters();
    assert_eq!(query.preflight_check_count(), 1);
    assert_eq!(query.fresh_resource_attempt_count(), 0);
    assert_eq!(query.bridge_readmission_attempt_count(), 0);
    assert_eq!(query.provider_restore_attempt_count(), 0);
    assert_eq!(query.artifact_generation_attempt_count(), 0);
    assert_eq!(query.artifact_generation_commit_count(), 0);
    assert_eq!(query.committed_attempt_count(), 0);
    assert!(evidence.bridge_counters().is_none());
}

fn complete_workflow_yield_cleanup(
    yielded: crate::domain_computation::WorthQueryYieldedWorkflowRun,
) {
    assert!(matches!(
        yielded.cleanup(),
        crate::domain_computation::WorthQueryWorkflowYieldCleanupOutcome::Complete(_)
    ));
}
