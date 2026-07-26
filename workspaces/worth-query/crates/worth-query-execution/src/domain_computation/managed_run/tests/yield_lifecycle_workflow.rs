use std::collections::BTreeMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use worth_runtime_bridge::facade::{
    BridgeExecutionBasisSignalTerminal, BridgeExecutionBasisTerminalDisposition,
};

use super::yield_fixture::YieldProvider;
use super::*;

#[test]
fn workflow_yield_retains_operation_and_stage_capacity_until_cleanup() {
    let installer = WorthQueryExecutionRuntimeInstaller::new();
    let provider_anchor = Arc::new(
        crate::domain_computation::provider_session::graph_provider::bounded_step::provider_anchor::WorthQueryGraphProviderAnchor::install::<ManagedGraph, _>(
            YieldProvider::installed(7),
        ),
    );
    let provider_support = provider_anchor.resource_support().clone();
    let graph = super::workflow_provider_steps::installed_graph(
        &installer,
        "workflow-yield-graph",
        provider_anchor,
    );
    let runtime = super::workflow_provider_steps::installed_runtime(installer, "workflow yield");
    let operation_resources =
        crate::domain_computation::provider_session::admitted_yield_plan("workflow-yield", 8);
    let stage_resources =
        admitted_plan_with_graph_support("workflow-yield:stage", 8, graph.role(), provider_support);
    let resources = WorthQueryAdmittedWorkflowResourcePlan::assemble(
        operation_resources,
        BTreeMap::from([("stage".to_owned(), stage_resources)]),
    );
    let operation = workflow_authority_with_stage_graph(
        &runtime,
        &resources,
        "stage",
        &graph,
        WorthQueryOperationGraphAccess::Observe,
    );
    let running =
        super::workflow_provider_steps::admitted_workflow(&runtime, &operation, resources);
    let logical_run_identity = running.logical_run_identity().to_owned();
    let attempt_identity = running.identity().to_owned();
    let active = running
        .begin_stage_graph_execution(
            "stage",
            &graph,
            WorthQueryManagedGraphCallRequest::new(
                WorthQueryGraphProviderCallKind::Observe,
                "workflow-yield",
            ),
        )
        .unwrap();
    let paused = match active.advance() {
        WorthQueryWorkflowGraphStepOutcome::Continue(paused) => paused,
        _ => panic!("workflow provider did not pause"),
    };

    let yielded = match paused.yield_run() {
        crate::domain_computation::WorthQueryWorkflowYieldOutcome::Yielded(yielded) => yielded,
        _ => panic!("eligible workflow did not yield"),
    };
    assert_eq!(yielded.logical_run_identity(), logical_run_identity);
    assert_eq!(yielded.yielded_attempt_identity(), attempt_identity);
    assert_eq!(yielded.checkpoint().retained_bytes(), 7);
    assert_eq!(yielded.retained_capacity_reservation_count(), 3);
    assert_eq!(
        yielded.bridge().disposition(),
        BridgeExecutionBasisTerminalDisposition::Yielded
    );
    assert_eq!(
        yielded.bridge().signal_terminal(),
        BridgeExecutionBasisSignalTerminal::Cancelled
    );
    assert!(yielded.bridge().signal_transition_performed());
    assert_eq!(yielded.provider_work().interrupted_call_count(), 1);
    assert_eq!(yielded.artifact_evidence().retained_artifact_count(), 0);

    let cleanup = match yielded.cleanup() {
        crate::domain_computation::WorthQueryWorkflowYieldCleanupOutcome::Complete(receipt) => {
            receipt
        }
        crate::domain_computation::WorthQueryWorkflowYieldCleanupOutcome::Pending(_) => {
            panic!("artifact-free yielded workflow reported pending cleanup")
        }
        crate::domain_computation::WorthQueryWorkflowYieldCleanupOutcome::RecoveryRequired(_) => {
            panic!("ordinary checkpoint release unexpectedly required recovery")
        }
    };
    assert_eq!(cleanup.logical_run_identity(), logical_run_identity);
    assert!(cleanup.relational().released());
    assert_eq!(cleanup.attempt().capacity().released_reservation_count(), 3);
    assert_eq!(cleanup.checkpoint().retained_bytes(), 7);
}

#[test]
fn workflow_suspension_failure_returns_terminalized_release_authority() {
    let installer = WorthQueryExecutionRuntimeInstaller::new();
    let provider_anchor = Arc::new(
        crate::domain_computation::provider_session::graph_provider::bounded_step::provider_anchor::WorthQueryGraphProviderAnchor::install::<ManagedGraph, _>(
            YieldProvider::suspension_failure(),
        ),
    );
    let provider_support = provider_anchor.resource_support().clone();
    let graph = super::workflow_provider_steps::installed_graph(
        &installer,
        "workflow-yield-failure-graph",
        provider_anchor,
    );
    let runtime =
        super::workflow_provider_steps::installed_runtime(installer, "workflow yield failure");
    let operation_resources = crate::domain_computation::provider_session::admitted_yield_plan(
        "workflow-yield-failure",
        8,
    );
    let stage_resources = admitted_plan_with_graph_support(
        "workflow-yield-failure:stage",
        8,
        graph.role(),
        provider_support,
    );
    let resources = WorthQueryAdmittedWorkflowResourcePlan::assemble(
        operation_resources,
        BTreeMap::from([("stage".to_owned(), stage_resources)]),
    );
    let operation = workflow_authority_with_stage_graph(
        &runtime,
        &resources,
        "stage",
        &graph,
        WorthQueryOperationGraphAccess::Observe,
    );
    let running =
        super::workflow_provider_steps::admitted_workflow(&runtime, &operation, resources);
    let active = running
        .begin_stage_graph_execution(
            "stage",
            &graph,
            WorthQueryManagedGraphCallRequest::new(
                WorthQueryGraphProviderCallKind::Observe,
                "workflow-yield-failure",
            ),
        )
        .unwrap();
    let paused = match active.advance() {
        WorthQueryWorkflowGraphStepOutcome::Continue(paused) => paused,
        _ => panic!("workflow provider did not pause"),
    };
    let recovery = match paused.yield_run() {
        crate::domain_computation::WorthQueryWorkflowYieldOutcome::RecoveryRequired(recovery) => {
            recovery
        }
        _ => panic!("workflow suspension failure did not preserve recovery authority"),
    };
    assert!(!recovery.running_attempt_recoverable());
    assert_eq!(
        recovery.kind(),
        crate::domain_computation::WorthQueryYieldRecoveryKind::ProviderCheckpointSuspension(
            crate::domain_computation::WorthQueryProviderCheckpointSuspensionFailureKind::
                ProviderRejected,
        )
    );
    let release = match recovery.release_terminalized() {
        Ok(crate::domain_computation::WorthQueryWorkflowYieldRecoveryReleaseOutcome::Complete(
            release,
        )) => release,
        Ok(crate::domain_computation::WorthQueryWorkflowYieldRecoveryReleaseOutcome::Pending(
            _,
        )) => {
            panic!("artifact-free workflow recovery reported pending cleanup")
        }
        Err(_) => panic!("artifact-free workflow recovery did not release"),
    };
    assert_eq!(
        release.bridge().signal_terminal(),
        BridgeExecutionBasisSignalTerminal::Cancelled
    );
    assert!(release.relational().released());
    assert_eq!(release.attempt().capacity().released_reservation_count(), 3);
}

#[test]
fn workflow_yield_cleanup_waits_for_retained_artifact_owners() {
    let installer = WorthQueryExecutionRuntimeInstaller::new();
    let provider_anchor = Arc::new(
        crate::domain_computation::provider_session::graph_provider::bounded_step::provider_anchor::WorthQueryGraphProviderAnchor::install::<ManagedGraph, _>(
            YieldProvider::installed(7),
        ),
    );
    let provider_support = provider_anchor.resource_support().clone();
    let graph = super::workflow_provider_steps::installed_graph(
        &installer,
        "workflow-yield-artifact-graph",
        provider_anchor,
    );
    let runtime =
        super::workflow_provider_steps::installed_runtime(installer, "workflow yield artifact");
    let operation_resources = crate::domain_computation::provider_session::admitted_yield_plan(
        "workflow-yield-artifact",
        8,
    );
    let stage_resources = admitted_plan_with_graph_support(
        "workflow-yield-artifact:producer",
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
    let running =
        super::workflow_provider_steps::admitted_workflow(&runtime, &operation, resources);
    let production = running
        .artifacts()
        .production_authority("producer")
        .unwrap()
        .expect("output artifact contract should install");
    let admission =
        crate::domain_computation::artifact_owner::WorthQueryArtifactProductionAuthority::admit(
            &production,
            WorthQueryArtifactProductionEvidence::new(
                "yield-artifact-provenance",
                "yield-artifact-dependency",
            ),
        );
    let disposed = Arc::new(AtomicUsize::new(0));
    let handle =
        crate::domain_computation::artifact_owner::WorthQueryArtifactProductionAuthority::register_exact(
            &production,
            admission,
            YieldArtifactResource(Arc::clone(&disposed)),
        )
        .expect("exact artifact authority should register");
    let borrowed = handle
        .borrow("yield cleanup ownership probe")
        .expect("installed artifact contract should allow read borrow");
    let active = running
        .begin_stage_graph_execution(
            "producer",
            &graph,
            WorthQueryManagedGraphCallRequest::new(
                WorthQueryGraphProviderCallKind::Observe,
                "workflow-yield-artifact",
            ),
        )
        .unwrap();
    let paused = match active.advance() {
        WorthQueryWorkflowGraphStepOutcome::Continue(paused) => paused,
        _ => panic!("workflow provider did not pause"),
    };
    let yielded = match paused.yield_run() {
        crate::domain_computation::WorthQueryWorkflowYieldOutcome::Yielded(yielded) => yielded,
        _ => panic!("artifact-owning workflow did not yield"),
    };
    assert_eq!(yielded.artifact_evidence().retained_artifact_count(), 1);
    let rejected_disposed = Arc::new(AtomicUsize::new(0));
    let rejected_admission =
        crate::domain_computation::artifact_owner::WorthQueryArtifactProductionAuthority::admit(
            &production,
            WorthQueryArtifactProductionEvidence::new(
                "post-yield-provenance",
                "post-yield-dependency",
            ),
        );
    let denial =
        match crate::domain_computation::artifact_owner::WorthQueryArtifactProductionAuthority::register_exact(
            &production,
            rejected_admission,
            YieldArtifactResource(Arc::clone(&rejected_disposed)),
        ) {
            Ok(_) => panic!("yielded workflow retained live artifact production authority"),
            Err(denial) => denial,
        };
    assert_eq!(
        denial.kind(),
        crate::domain_computation::WorthQueryArtifactDenialKind::ProductionClosed
    );
    assert_eq!(rejected_disposed.load(Ordering::Acquire), 1);
    let pending = match yielded.cleanup() {
        crate::domain_computation::WorthQueryWorkflowYieldCleanupOutcome::Pending(pending) => {
            pending
        }
        crate::domain_computation::WorthQueryWorkflowYieldCleanupOutcome::Complete(_) => {
            panic!("live artifact owner allowed yielded cleanup to release lower authority")
        }
        crate::domain_computation::WorthQueryWorkflowYieldCleanupOutcome::RecoveryRequired(_) => {
            panic!("ordinary checkpoint release unexpectedly required recovery")
        }
    };
    assert_eq!(pending.artifact_evidence().retained_artifact_count(), 1);
    assert_eq!(disposed.load(Ordering::Acquire), 0);

    drop(borrowed);
    assert_eq!(disposed.load(Ordering::Acquire), 1);
    drop(handle);
    let cleanup = match pending.retry() {
        crate::domain_computation::WorthQueryWorkflowYieldCleanupOutcome::Complete(cleanup) => {
            cleanup
        }
        crate::domain_computation::WorthQueryWorkflowYieldCleanupOutcome::Pending(_) => {
            panic!("released artifact owners kept yielded cleanup pending")
        }
        crate::domain_computation::WorthQueryWorkflowYieldCleanupOutcome::RecoveryRequired(_) => {
            panic!("ordinary checkpoint release unexpectedly required recovery")
        }
    };
    assert_eq!(cleanup.artifact_evidence().disposed_artifact_count(), 1);
    assert!(cleanup.relational().released());
    assert_eq!(cleanup.attempt().capacity().released_reservation_count(), 3);
}

struct YieldArtifactResource(Arc<AtomicUsize>);

impl WorthQueryArtifactProviderResource for YieldArtifactResource {
    const PROVIDER_FAMILY: &'static str = "WORTH.tests.affinity.provider";

    fn canonical_semantic_projection(&self) -> Vec<u8> {
        b"yield-artifact".to_vec()
    }

    fn retained_bytes(&self) -> usize {
        64
    }

    fn dispose(&mut self) {
        self.0.fetch_add(1, Ordering::AcqRel);
    }
}
