use super::yield_fixture::YieldProvider;
use super::*;

#[test]
fn failed_workflow_yield_reports_pending_artifacts_and_preserves_retry_authority() {
    let disposals = Arc::new(AtomicUsize::new(0));
    let world = terminalized_recovery_world(
        "workflow-yield-recovery-artifact-graph",
        RecoveryArtifactResource(Arc::clone(&disposals)),
    );
    let TerminalizedRecoveryWorld {
        running,
        graph,
        handle,
    } = world;
    let borrowed = handle
        .borrow("failed-yield cleanup ownership probe")
        .expect("installed artifact contract should admit a shared borrow");
    let recovery = terminalize_recovery_world(running, &graph);
    let initial_artifacts = recovery
        .artifact_evidence()
        .expect("terminalized workflow recovery reports artifact ownership");
    assert_eq!(initial_artifacts.produced_artifact_count(), 1);
    assert_eq!(initial_artifacts.retained_artifact_count(), 1);
    assert_eq!(initial_artifacts.retained_bytes(), 64);

    let pending = match recovery.release_terminalized() {
        Ok(crate::domain_computation::WorthQueryWorkflowYieldRecoveryReleaseOutcome::Pending(
            pending,
        )) => pending,
        Ok(crate::domain_computation::WorthQueryWorkflowYieldRecoveryReleaseOutcome::Complete(
            _,
        )) => {
            panic!("outstanding artifact borrow allowed complete failed-yield cleanup")
        }
        Ok(
            crate::domain_computation::WorthQueryWorkflowYieldRecoveryReleaseOutcome::
                RecoveryRequired(_),
        ) => panic!("outstanding artifact borrow skipped the pending cleanup phase"),
        Err(_) => panic!("terminalized failed yield was misclassified as running"),
    };
    assert_eq!(pending.pending_artifact_owner_count(), 1);
    assert_eq!(pending.artifact_evidence().retained_bytes(), 64);
    assert_eq!(
        pending.recovery().kind(),
        crate::domain_computation::WorthQueryYieldRecoveryKind::ProviderCheckpointSuspension(
            crate::domain_computation::WorthQueryProviderCheckpointSuspensionFailureKind::
                ProviderRejected,
        )
    );
    assert_eq!(disposals.load(Ordering::Acquire), 0);

    drop(borrowed);
    assert_eq!(disposals.load(Ordering::Acquire), 1);
    drop(handle);
    let release = match pending.retry() {
        Ok(crate::domain_computation::WorthQueryWorkflowYieldRecoveryReleaseOutcome::Complete(
            release,
        )) => release,
        Ok(crate::domain_computation::WorthQueryWorkflowYieldRecoveryReleaseOutcome::Pending(
            _,
        )) => {
            panic!("released artifact owner kept failed-yield cleanup pending")
        }
        Ok(
            crate::domain_computation::WorthQueryWorkflowYieldRecoveryReleaseOutcome::
                RecoveryRequired(_),
        ) => panic!("successful artifact release required recovery"),
        Err(_) => panic!("pending failed-yield cleanup lost terminalized retry authority"),
    };
    assert_eq!(release.artifact_evidence().disposed_artifact_count(), 1);
    assert_eq!(
        release
            .artifact_evidence()
            .provider_release_complete_count(),
        1
    );
    assert!(release.relational().released());
    assert_eq!(release.attempt().capacity().released_reservation_count(), 3);
}

#[test]
fn terminalized_workflow_yield_types_double_artifact_release_panic_as_recovery() {
    let disposal_attempts = Arc::new(AtomicUsize::new(0));
    let destructor_attempts = Arc::new(AtomicUsize::new(0));
    let world = terminalized_recovery_world(
        "workflow-yield-recovery-double-panic",
        DoublePanickingRecoveryArtifactResource {
            disposal_attempts: Arc::clone(&disposal_attempts),
            destructor_attempts: Arc::clone(&destructor_attempts),
        },
    );
    let TerminalizedRecoveryWorld {
        running,
        graph,
        handle,
    } = world;
    let recovery = terminalize_recovery_world(running, &graph);
    let release = match recovery.release_terminalized() {
        Ok(
            crate::domain_computation::WorthQueryWorkflowYieldRecoveryReleaseOutcome::
                RecoveryRequired(release),
        ) => release,
        Ok(crate::domain_computation::WorthQueryWorkflowYieldRecoveryReleaseOutcome::Complete(
            _,
        )) => panic!("double artifact release panic was reported as complete"),
        Ok(crate::domain_computation::WorthQueryWorkflowYieldRecoveryReleaseOutcome::Pending(
            _,
        )) => panic!("artifact without a surviving borrow remained pending"),
        Err(_) => panic!("terminalized failed yield lost release authority"),
    };
    assert_eq!(
        release
            .recovery_evidence()
            .provider_checkpoint_failure()
            .expect("primary suspension failure evidence must survive cleanup")
            .kind(),
        crate::domain_computation::WorthQueryProviderCheckpointSuspensionFailureKind::
            ProviderRejected,
    );
    assert_eq!(
        release
            .artifact_evidence()
            .provider_release_recovery_required_count(),
        1
    );
    assert_eq!(disposal_attempts.load(Ordering::Acquire), 1);
    assert_eq!(destructor_attempts.load(Ordering::Acquire), 1);
    assert!(release.relational().released());
    assert_eq!(release.attempt().capacity().released_reservation_count(), 3);
    let artifact_release = match handle.owner_snapshot().provider_release() {
        crate::domain_computation::WorthQueryArtifactProviderReleasePosture::RecoveryRequired(
            evidence,
        ) => evidence,
        posture => panic!("terminalized artifact release reported {posture:?}"),
    };
    assert_eq!(
        artifact_release.disposal(),
        crate::domain_computation::WorthQueryArtifactProviderDisposalDisposition::Panicked
    );
    assert_eq!(
        artifact_release.destructor(),
        crate::domain_computation::WorthQueryArtifactProviderDestructorDisposition::Panicked
    );
}

struct TerminalizedRecoveryWorld {
    running: crate::domain_computation::WorthQueryRunningWorkflowRun,
    graph: WorthQueryInstalledGraphParticipationAuthority,
    handle: crate::domain_computation::WorthQueryMoveOnlyArtifactHandle,
}

fn terminalized_recovery_world<R>(label: &str, resource: R) -> TerminalizedRecoveryWorld
where
    R: WorthQueryArtifactProviderResource,
{
    let installer = WorthQueryExecutionRuntimeInstaller::new();
    let provider_anchor = Arc::new(
        crate::domain_computation::provider_session::graph_provider::bounded_step::provider_anchor::WorthQueryGraphProviderAnchor::install::<ManagedGraph, _>(
            YieldProvider::suspension_failure(),
        ),
    );
    let provider_support = provider_anchor.resource_support().clone();
    let graph = super::workflow_provider_steps::installed_graph(
        &installer,
        &format!("{label}:graph"),
        provider_anchor,
    );
    let runtime =
        super::workflow_provider_steps::installed_runtime(installer, &format!("{label}:runtime"));
    let operation_resources =
        crate::domain_computation::provider_session::admitted_yield_plan(label, 8);
    let stage_resources = admitted_plan_with_graph_support(
        &format!("{label}:producer"),
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
        .expect("producer role should validate")
        .expect("producer output contract should install");
    let admission =
        crate::domain_computation::artifact_owner::WorthQueryArtifactProductionAuthority::admit(
            &production,
            WorthQueryArtifactProductionEvidence::new(
                format!("{label}:provenance"),
                format!("{label}:dependency"),
            ),
        );
    let handle =
        crate::domain_computation::artifact_owner::WorthQueryArtifactProductionAuthority::register_exact(
            &production,
            admission,
            resource,
        )
        .expect("recovery artifact should register before yield begins");
    TerminalizedRecoveryWorld {
        running,
        graph,
        handle,
    }
}

fn terminalize_recovery_world(
    running: crate::domain_computation::WorthQueryRunningWorkflowRun,
    graph: &WorthQueryInstalledGraphParticipationAuthority,
) -> crate::domain_computation::WorthQueryWorkflowYieldRecoveryRequired {
    let active = running
        .begin_stage_graph_execution(
            "producer",
            graph,
            WorthQueryManagedGraphCallRequest::new(
                WorthQueryGraphProviderCallKind::Observe,
                "workflow-yield-recovery",
            ),
        )
        .expect("recovery provider should begin");
    let paused = match active.advance() {
        WorthQueryWorkflowGraphStepOutcome::Continue(paused) => paused,
        _ => panic!("recovery provider did not reach its safe point"),
    };
    let recovery = match paused.yield_run() {
        crate::domain_computation::WorthQueryWorkflowYieldOutcome::RecoveryRequired(recovery) => {
            recovery
        }
        _ => panic!("provider suspension failure did not return recovery authority"),
    };
    recovery
}

struct RecoveryArtifactResource(Arc<AtomicUsize>);

impl WorthQueryArtifactProviderResource for RecoveryArtifactResource {
    const PROVIDER_FAMILY: &'static str = "WORTH.tests.affinity.provider";

    fn canonical_semantic_projection(&self) -> Vec<u8> {
        b"yield-recovery-artifact".to_vec()
    }

    fn retained_bytes(&self) -> usize {
        64
    }

    fn dispose(&mut self) {
        self.0.fetch_add(1, Ordering::AcqRel);
    }
}

struct DoublePanickingRecoveryArtifactResource {
    disposal_attempts: Arc<AtomicUsize>,
    destructor_attempts: Arc<AtomicUsize>,
}

impl WorthQueryArtifactProviderResource for DoublePanickingRecoveryArtifactResource {
    const PROVIDER_FAMILY: &'static str = "WORTH.tests.affinity.provider";

    fn canonical_semantic_projection(&self) -> Vec<u8> {
        b"terminalized-yield-double-panic".to_vec()
    }

    fn retained_bytes(&self) -> usize {
        64
    }

    fn dispose(&mut self) {
        self.disposal_attempts.fetch_add(1, Ordering::AcqRel);
        panic!("terminalized artifact provider disposal panicked")
    }
}

impl Drop for DoublePanickingRecoveryArtifactResource {
    fn drop(&mut self) {
        self.destructor_attempts.fetch_add(1, Ordering::AcqRel);
        panic!("terminalized artifact provider destructor panicked")
    }
}
