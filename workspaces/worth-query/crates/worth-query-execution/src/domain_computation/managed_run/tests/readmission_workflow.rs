use std::collections::BTreeMap;
use std::sync::Arc;

use super::yield_fixture::YieldProvider;
use super::*;

struct ReadmissionArtifact;

impl WorthQueryArtifactProviderResource for ReadmissionArtifact {
    const PROVIDER_FAMILY: &'static str = "WORTH.tests.affinity.provider";

    fn canonical_semantic_projection(&self) -> Vec<u8> {
        b"workflow-readmission-artifact".to_vec()
    }

    fn retained_bytes(&self) -> usize {
        9
    }

    fn dispose(&mut self) {}
}

pub(super) fn yielded_workflow(
    provider: YieldProvider,
) -> (
    crate::domain_computation::WorthQueryYieldedWorkflowRun,
    RuntimeBridge,
    WorthQueryExecutionRuntime,
    Arc<crate::domain_computation::WorthQueryArtifactProductionAuthority>,
) {
    let installer = WorthQueryExecutionRuntimeInstaller::new();
    let provider_anchor = Arc::new(
        crate::domain_computation::provider_session::graph_provider::bounded_step::provider_anchor::WorthQueryGraphProviderAnchor::install::<ManagedGraph, _>(
            provider,
        ),
    );
    let provider_support = provider_anchor.resource_support().clone();
    let graph = super::workflow_provider_steps::installed_graph(
        &installer,
        "workflow-readmission-graph",
        provider_anchor,
    );
    let runtime =
        super::workflow_provider_steps::installed_runtime(installer, "workflow readmission");
    let operation_resources =
        crate::domain_computation::provider_session::admitted_yield_plan("workflow-readmission", 8);
    let stage_resources = admitted_plan_with_graph_support(
        "workflow-readmission:producer",
        8,
        graph.role(),
        provider_support,
    );
    let resources = WorthQueryAdmittedWorkflowResourcePlan::assemble(
        operation_resources,
        BTreeMap::from([("producer".to_owned(), stage_resources)]),
    );
    let output =
        crate::domain_computation::artifact_owner::installed_artifact_contract_for_managed_run();
    let operation = workflow_authority_with_stage_graph_and_output_artifact(
        &runtime,
        &resources,
        "producer",
        &graph,
        WorthQueryOperationGraphAccess::Observe,
        output,
    );
    let attempt = runtime
        .start_workflow_resource_attempt(&operation, resources)
        .expect("workflow resources should reserve");
    let lower = causal_fixture::managed_admission_context();
    let running = runtime
        .managed_run_admission(&lower.bridge, &lower.relational)
        .admit_workflow(&operation, attempt, lower.read_request())
        .expect("workflow should admit")
        .start()
        .expect("workflow should start");
    let old_producer = running
        .artifacts()
        .production_authority("producer")
        .expect("producer stage should validate")
        .expect("producer stage should own output authority");
    let active = running
        .begin_stage_graph_execution(
            "producer",
            &graph,
            WorthQueryManagedGraphCallRequest::new(
                WorthQueryGraphProviderCallKind::Observe,
                "workflow-readmission",
            ),
        )
        .expect("workflow provider should begin");
    let paused = match active.advance() {
        WorthQueryWorkflowGraphStepOutcome::Continue(paused) => paused,
        _ => panic!("workflow provider did not pause"),
    };
    let yielded = match paused.yield_run() {
        crate::domain_computation::WorthQueryWorkflowYieldOutcome::Yielded(yielded) => yielded,
        _ => panic!("eligible workflow did not yield"),
    };
    (yielded, lower.bridge, runtime, old_producer)
}

#[test]
fn workflow_readmission_rolls_generation_and_preserves_occurrence_state() {
    let (yielded, bridge, runtime, old_producer) = yielded_workflow(YieldProvider::installed(7));
    let logical = yielded.logical_run_identity().to_owned();
    let old_managed_attempt = yielded.yielded_attempt_identity().to_owned();
    let old_resource_attempt = yielded.resource_attempt_identity().to_owned();
    let old_provider_session = yielded.provider_session_identity().to_owned();
    let old_bridge_request = yielded.bridge_request_identity().to_owned();
    let old_artifacts = yielded.artifact_evidence();
    let old_provider_work = yielded.provider_work();
    let reservations = yielded.retained_capacity_reservation_count();
    let active = match yielded.readmit_same_runtime(&runtime, &bridge) {
        crate::domain_computation::WorthQueryWorkflowReadmissionOutcome::Readmitted(active) => {
            active
        }
        _ => panic!("same-runtime workflow readmission should succeed"),
    };
    assert_eq!(active.logical_run_identity(), logical);
    assert_ne!(active.run_identity(), old_managed_attempt);
    assert_ne!(active.resource_attempt_identity(), old_resource_attempt);
    assert_ne!(active.provider_session_identity(), old_provider_session);
    assert_ne!(active.bridge_request_identity(), old_bridge_request);
    assert_eq!(active.retained_capacity_reservation_count(), reservations);
    assert_eq!(reservations, 3);
    let fresh_artifacts = active.artifact_evidence();
    assert_eq!(
        fresh_artifacts.production_generation(),
        old_artifacts.production_generation() + 1
    );
    assert_eq!(
        (
            fresh_artifacts.produced_artifact_count(),
            fresh_artifacts.retained_artifact_count(),
            fresh_artifacts.disposed_artifact_count(),
            fresh_artifacts.retained_bytes(),
        ),
        (
            old_artifacts.produced_artifact_count(),
            old_artifacts.retained_artifact_count(),
            old_artifacts.disposed_artifact_count(),
            old_artifacts.retained_bytes(),
        )
    );

    let old_admission = crate::domain_computation::WorthQueryArtifactProductionAuthority::admit(
        &old_producer,
        WorthQueryArtifactProductionEvidence::new("old-generation", "readmission"),
    );
    let old_denial =
        crate::domain_computation::WorthQueryArtifactProductionAuthority::register_exact(
            &old_producer,
            old_admission,
            ReadmissionArtifact,
        )
        .expect_err("pre-yield producer must remain stale");
    assert_eq!(
        old_denial.kind(),
        crate::domain_computation::WorthQueryArtifactDenialKind::StaleLifecycleGeneration
    );
    let fresh_producer = active
        .artifacts()
        .production_authority("producer")
        .expect("fresh producer stage should validate")
        .expect("fresh generation should mint production authority");
    let fresh_admission = crate::domain_computation::WorthQueryArtifactProductionAuthority::admit(
        &fresh_producer,
        WorthQueryArtifactProductionEvidence::new("fresh-generation", "readmission"),
    );
    let fresh_handle =
        crate::domain_computation::WorthQueryArtifactProductionAuthority::register_exact(
            &fresh_producer,
            fresh_admission,
            ReadmissionArtifact,
        )
        .expect("fresh producer should register");
    drop(fresh_handle);

    let completion = match active.advance() {
        WorthQueryWorkflowGraphStepOutcome::Completed(completion) => completion,
        _ => panic!("restored workflow provider did not complete"),
    };
    let terminal = completion.into_running().completed().unwrap();
    assert_eq!(terminal.logical_run_identity(), logical);
    assert_eq!(terminal.provider_work().completed_work_units(), 4);
    assert_eq!(
        terminal.provider_work().produced_artifact_count(),
        old_provider_work.produced_artifact_count()
    );
    match terminal.cleanup() {
        WorthQueryWorkflowRunCleanupOutcome::Complete(_) => {}
        _ => panic!("readmitted workflow did not clean up"),
    }
}

#[test]
fn workflow_provider_restore_denial_keeps_frozen_generation_retryable() {
    let (yielded, bridge, runtime, _producer) =
        yielded_workflow(YieldProvider::checkpoint_restore_failure(7));
    let checkpoint = yielded.checkpoint().identity().to_owned();
    let generation = yielded.artifact_evidence().production_generation();
    let denied = match yielded.readmit_same_runtime(&runtime, &bridge) {
        crate::domain_computation::WorthQueryWorkflowReadmissionOutcome::Denied(denied) => denied,
        _ => panic!("ordinary workflow restore failure should deny"),
    };
    assert_eq!(
        denied.kind(),
        crate::domain_computation::WorthQueryWorkflowReadmissionDenialKind::ProviderRestoreDenied
    );
    assert_eq!(denied.counters().artifact_generation_attempt_count(), 0);
    let yielded = denied.into_yielded();
    assert_eq!(yielded.checkpoint().identity(), checkpoint);
    assert_eq!(
        yielded.artifact_evidence().production_generation(),
        generation
    );
    match yielded.cleanup() {
        crate::domain_computation::WorthQueryWorkflowYieldCleanupOutcome::Complete(_) => {}
        _ => panic!("denied workflow should retain complete cleanup authority"),
    }
}

#[test]
fn workflow_restore_panic_can_recover_only_through_the_retained_yielded_authority() {
    let (yielded, bridge, runtime, _producer) =
        yielded_workflow(YieldProvider::checkpoint_restore_panic(7));
    let checkpoint = yielded.checkpoint().identity().to_owned();
    let generation = yielded.artifact_evidence().production_generation();
    let recovery = match yielded.readmit_same_runtime(&runtime, &bridge) {
        crate::domain_computation::WorthQueryWorkflowReadmissionOutcome::RecoveryRequired(
            recovery,
        ) => recovery,
        _ => panic!("workflow provider restore panic should require recovery"),
    };
    assert!(recovery.checkpoint_authority_retained());
    assert!(!recovery.replacement_execution_active());
    let yielded = match recovery.retry_to_yielded() {
        Ok(
            crate::domain_computation::WorthQueryWorkflowReadmissionRecoveryRetryOutcome::Yielded(
                yielded,
            ),
        ) => yielded,
        _ => panic!("retry-safe workflow recovery should return the yielded authority"),
    };
    assert_eq!(yielded.checkpoint().identity(), checkpoint);
    assert_eq!(
        yielded.artifact_evidence().production_generation(),
        generation
    );
    match yielded.cleanup() {
        crate::domain_computation::WorthQueryWorkflowYieldCleanupOutcome::Complete(_) => {}
        _ => panic!("recovered workflow yielded authority should clean up"),
    }
}

#[test]
fn workflow_generation_mismatch_denies_before_fresh_authority_and_rolls_back_cleanly() {
    let (yielded, bridge, runtime, _producer) = yielded_workflow(YieldProvider::installed(7));
    let generation = yielded.artifact_evidence().production_generation();
    let pending = yielded
        .artifacts
        .registry()
        .prepare_next_generation()
        .expect("test delta should place the registry in a pending generation");
    let denied = match yielded.readmit_same_runtime(&runtime, &bridge) {
        crate::domain_computation::WorthQueryWorkflowReadmissionOutcome::Denied(denied) => denied,
        _ => panic!("non-frozen yielded generation should deny during preflight"),
    };
    assert_eq!(
        denied.kind(),
        crate::domain_computation::WorthQueryWorkflowReadmissionDenialKind::ArtifactGenerationMismatch
    );
    assert_eq!(denied.counters().fresh_resource_attempt_count(), 0);
    assert_eq!(denied.counters().bridge_readmission_attempt_count(), 0);
    assert_eq!(denied.counters().provider_restore_attempt_count(), 0);
    drop(pending);
    let yielded = denied.into_yielded();
    assert_eq!(
        yielded.artifact_evidence().production_generation(),
        generation
    );
    let active = match yielded.readmit_same_runtime(&runtime, &bridge) {
        crate::domain_computation::WorthQueryWorkflowReadmissionOutcome::Readmitted(active) => {
            active
        }
        _ => panic!("generation rollback should restore exact retry authority"),
    };
    let completion = match active.advance() {
        WorthQueryWorkflowGraphStepOutcome::Completed(completion) => completion,
        _ => panic!("restored workflow provider should complete"),
    };
    match completion.into_running().completed().unwrap().cleanup() {
        WorthQueryWorkflowRunCleanupOutcome::Complete(_) => {}
        _ => panic!("retried workflow should clean up"),
    }
}
