use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use super::*;
use crate::domain_computation::{
    WorthQueryProviderExecutionPlanView, WorthQueryProviderSessionDenialKind,
    WorthQueryProviderSessionFailure, WorthQueryProviderSessionLifecycle,
    WorthQueryProviderSessionProtocolCounters, WorthQueryProviderSessionProtocolStage,
    WorthQueryProviderSessionRecoveryPosture, WorthQueryProviderSessionToken,
    WorthQueryProviderSessionTokenAdmission, WorthQueryProviderSessionView,
    WorthQuerySessionCommitOrAbortOutcome,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum SessionFailurePoint {
    None,
    ReadmissionRejection,
    ReadmissionPanic,
    PreparationPanic,
    StagedPreparationRejection,
    CommitPanic,
    AbortRejection,
}

#[derive(Default)]
pub(super) struct SessionCallCounts {
    readmissions: AtomicUsize,
    preparations: AtomicUsize,
    staged_preparations: AtomicUsize,
    commits: AtomicUsize,
    pub(super) aborts: AtomicUsize,
}

struct SessionProtocolProvider {
    failure: SessionFailurePoint,
    calls: Arc<SessionCallCounts>,
}

struct UnusedSessionExecution;

impl WorthQueryGraphProviderExecution for UnusedSessionExecution {
    fn advance(
        &mut self,
        _step: &mut WorthQueryGraphProviderStep,
    ) -> Result<WorthQueryGraphProviderStepDisposition, WorthQueryGraphProviderFailure> {
        unreachable!("session protocol tests do not enter the legacy graph callback")
    }

    fn dispose(&mut self) -> Result<(), WorthQueryGraphProviderFailure> {
        Ok(())
    }
}

impl WorthQueryGraphParticipationProvider<ManagedGraph> for SessionProtocolProvider {
    type Execution = UnusedSessionExecution;

    fn execution_resource_support(
        &self,
    ) -> worth_query_admission::facade::resource_admission::WorthQueryExecutionResourceSupport {
        crate::domain_computation::provider_session::execution_resource_support(
            "session-protocol",
            8,
        )
    }

    fn begin(
        &self,
        _call: &WorthQueryGraphProviderCall,
        _start: &mut WorthQueryGraphProviderExecutionStart,
    ) -> Result<
        WorthQueryCooperativeGraphProviderExecution<Self::Execution>,
        WorthQueryGraphProviderFailure,
    > {
        unreachable!("sealed session protocol must not route through the one-shot callback")
    }
}

impl WorthQueryProviderSessionLifecycle for SessionProtocolProvider {
    fn readmit_provider_plan(
        &self,
        plan: &WorthQueryProviderExecutionPlanView<'_>,
        admission: WorthQueryProviderSessionTokenAdmission,
    ) -> Result<WorthQueryProviderSessionToken, WorthQueryProviderSessionFailure> {
        self.calls.readmissions.fetch_add(1, Ordering::AcqRel);
        assert_eq!(plan.contract().provider_role(), "managed-graph");
        match self.failure {
            SessionFailurePoint::ReadmissionRejection => {
                Err(provider_rejection("readmission rejected"))
            }
            SessionFailurePoint::ReadmissionPanic => panic!("readmission panic"),
            _ => admission.admit("physical-session"),
        }
    }

    fn prepare_provider_session(
        &self,
        _session: &WorthQueryProviderSessionView<'_>,
    ) -> Result<(), WorthQueryProviderSessionFailure> {
        self.calls.preparations.fetch_add(1, Ordering::AcqRel);
        if self.failure == SessionFailurePoint::PreparationPanic {
            panic!("preparation panic");
        }
        Ok(())
    }

    fn prepare_staged_session(
        &self,
        _session: &WorthQueryProviderSessionView<'_>,
    ) -> Result<(), WorthQueryProviderSessionFailure> {
        self.calls
            .staged_preparations
            .fetch_add(1, Ordering::AcqRel);
        if self.failure == SessionFailurePoint::StagedPreparationRejection {
            return Err(provider_rejection("staged preparation rejected"));
        }
        Ok(())
    }

    fn commit_prepared_session(
        &self,
        _session: &WorthQueryProviderSessionView<'_>,
    ) -> Result<String, WorthQueryProviderSessionFailure> {
        self.calls.commits.fetch_add(1, Ordering::AcqRel);
        if self.failure == SessionFailurePoint::CommitPanic {
            panic!("commit panic");
        }
        Ok("provider-commit".to_owned())
    }

    fn abort_provider_session(
        &self,
        _session: &WorthQueryProviderSessionView<'_>,
    ) -> Result<String, WorthQueryProviderSessionFailure> {
        self.calls.aborts.fetch_add(1, Ordering::AcqRel);
        if self.failure == SessionFailurePoint::AbortRejection {
            return Err(provider_rejection("abort rejected"));
        }
        Ok("provider-abort".to_owned())
    }
}

#[test]
fn sealed_plan_prepares_session_binds_work_and_aborts() {
    let calls = Arc::new(SessionCallCounts::default());
    let (mut running, graph) = session_run(SessionFailurePoint::None, Arc::clone(&calls), true);
    let expected_run = running.identity().to_owned();
    let expected_basis = running
        .provider_plan_bridge_basis()
        .identity()
        .as_str()
        .to_owned();
    let expected_snapshot = running.execution_snapshot_reference();
    {
        let plan = running
            .admit_provider_execution_plan(&graph)
            .expect("exact managed authorities should admit a provider plan");
        assert_eq!(plan.contract().provider_role(), "managed-graph");
        assert_eq!(plan.contract().managed_run_identity(), expected_run);
        assert_eq!(plan.contract().execution_basis_identity(), expected_basis);
        assert_eq!(plan.contract().snapshot_identity(), expected_snapshot);
        assert_eq!(
            plan.contract().graph_authority_identity(),
            graph.authority_identity()
        );
        assert_eq!(plan.contract().effect_closure(), ["mutation"]);
        assert_eq!(plan.counters().authority_checks(), 1);
        let readmitted = plan.readmit().expect("provider should readmit the plan");
        let prepared = readmitted
            .prepare()
            .expect("provider should prepare the session");
        let staged = prepared.bind_reads_and_effects();
        assert_eq!(
            staged.read_authority().token_identity(),
            staged.effect_authority().token_identity()
        );
        assert_eq!(staged.counters().provider_calls(), 2);
        let outcome = staged.abort();
        assert_eq!(
            outcome.recovery_posture(),
            WorthQueryProviderSessionRecoveryPosture::Closed
        );
        assert!(matches!(
            outcome,
            WorthQuerySessionCommitOrAbortOutcome::Aborted { .. }
        ));
    }
    assert_eq!(calls.readmissions.load(Ordering::Acquire), 1);
    assert_eq!(calls.preparations.load(Ordering::Acquire), 1);
    assert_eq!(calls.aborts.load(Ordering::Acquire), 1);
    cleanup(running);
}

#[test]
fn legacy_provider_is_denied_before_any_session_protocol_call() {
    let (mut running, graph) =
        managed_graph_run_with_provider(WorthQueryOperationGraphAccess::Observe, GraphOnlyProvider);
    let failure = running
        .admit_provider_execution_plan(&graph)
        .expect_err("one-shot-only provider must not enter the session lane");
    assert_eq!(
        failure.kind(),
        WorthQueryProviderSessionDenialKind::SessionProtocolUnsupported
    );
    cleanup(running);
}

#[test]
fn failures_are_localized_to_the_exact_protocol_stage() {
    assert_failure_stage(
        SessionFailurePoint::ReadmissionRejection,
        WorthQueryProviderSessionProtocolStage::PlanReadmission,
    );
    assert_failure_stage(
        SessionFailurePoint::ReadmissionPanic,
        WorthQueryProviderSessionProtocolStage::PlanReadmission,
    );
    assert_failure_stage(
        SessionFailurePoint::PreparationPanic,
        WorthQueryProviderSessionProtocolStage::SessionPreparation,
    );
}

#[test]
fn staged_prepare_commit_and_abort_failures_carry_recovery_posture() {
    let (mut prepare_run, prepare_graph) = session_run(
        SessionFailurePoint::StagedPreparationRejection,
        Arc::new(SessionCallCounts::default()),
        true,
    );
    let staged = staged_session(&mut prepare_run, &prepare_graph);
    let failure = staged
        .prepare_for_commit()
        .expect_err("staged preparation rejection must not mint prepare outcome");
    assert_eq!(
        failure.stage(),
        WorthQueryProviderSessionProtocolStage::StagedPreparation
    );
    assert_eq!(
        failure.recovery_posture(),
        WorthQueryProviderSessionRecoveryPosture::Closed
    );
    cleanup(prepare_run);

    let (mut commit_run, commit_graph) = session_run(
        SessionFailurePoint::CommitPanic,
        Arc::new(SessionCallCounts::default()),
        true,
    );
    let prepared = staged_session(&mut commit_run, &commit_graph)
        .prepare_for_commit()
        .expect("staged preparation should succeed");
    let outcome = prepared.commit();
    assert_eq!(
        outcome.recovery_posture(),
        WorthQueryProviderSessionRecoveryPosture::RecoveryRequired
    );
    assert_eq!(
        outcome.failure().unwrap().stage(),
        WorthQueryProviderSessionProtocolStage::Commit
    );
    cleanup(commit_run);

    let (mut abort_run, abort_graph) = session_run(
        SessionFailurePoint::AbortRejection,
        Arc::new(SessionCallCounts::default()),
        true,
    );
    let outcome = staged_session(&mut abort_run, &abort_graph).abort();
    assert_eq!(
        outcome.recovery_posture(),
        WorthQueryProviderSessionRecoveryPosture::RecoveryRequired
    );
    assert_eq!(
        outcome.failure().unwrap().stage(),
        WorthQueryProviderSessionProtocolStage::Abort
    );
    cleanup(abort_run);
}

struct GraphOnlyProvider;

impl WorthQueryGraphParticipationProvider<ManagedGraph> for GraphOnlyProvider {
    type Execution = UnusedSessionExecution;

    fn execution_resource_support(
        &self,
    ) -> worth_query_admission::facade::resource_admission::WorthQueryExecutionResourceSupport {
        crate::domain_computation::provider_session::execution_resource_support("graph-only", 8)
    }

    fn begin(
        &self,
        _call: &WorthQueryGraphProviderCall,
        _start: &mut WorthQueryGraphProviderExecutionStart,
    ) -> Result<
        WorthQueryCooperativeGraphProviderExecution<Self::Execution>,
        WorthQueryGraphProviderFailure,
    > {
        unreachable!("unsupported session provider must deny before provider work")
    }
}

pub(super) fn session_run(
    failure: SessionFailurePoint,
    calls: Arc<SessionCallCounts>,
    touch: bool,
) -> (
    WorthQueryRunningDirectRun,
    WorthQueryInstalledGraphParticipationAuthority,
) {
    managed_session_graph_run_with_provider(
        WorthQueryOperationGraphAccess::Observe,
        SessionProtocolProvider { failure, calls },
        touch,
    )
}

fn staged_session<'run>(
    running: &'run mut WorthQueryRunningDirectRun,
    graph: &WorthQueryInstalledGraphParticipationAuthority,
) -> crate::domain_computation::WorthQuerySessionBoundReadsAndEffects<'run> {
    running
        .admit_provider_execution_plan(graph)
        .expect("plan admission should succeed")
        .readmit()
        .expect("plan readmission should succeed")
        .prepare()
        .expect("session preparation should succeed")
        .bind_reads_and_effects()
}

fn assert_failure_stage(
    point: SessionFailurePoint,
    expected: WorthQueryProviderSessionProtocolStage,
) {
    let (mut running, graph) = session_run(point, Arc::new(SessionCallCounts::default()), false);
    let plan = running
        .admit_provider_execution_plan(&graph)
        .expect("failure fixture plan should admit");
    let failure = match point {
        SessionFailurePoint::ReadmissionRejection | SessionFailurePoint::ReadmissionPanic => {
            plan.readmit().expect_err("readmission should fail")
        }
        SessionFailurePoint::PreparationPanic => plan
            .readmit()
            .expect("readmission should succeed")
            .prepare()
            .expect_err("preparation should fail"),
        _ => unreachable!("unsupported failure fixture"),
    };
    assert_eq!(failure.stage(), expected);
    let expected_posture = if point == SessionFailurePoint::ReadmissionPanic {
        WorthQueryProviderSessionRecoveryPosture::RecoveryRequired
    } else {
        WorthQueryProviderSessionRecoveryPosture::Closed
    };
    assert_eq!(failure.recovery_posture(), expected_posture);
    cleanup(running);
}

pub(super) fn cleanup(running: WorthQueryRunningDirectRun) {
    running
        .terminate_for_convergence(WorthQueryManagedRunTerminalKind::Failed)
        .cleanup()
        .expect("session protocol fixture cleanup should complete");
}

fn provider_rejection(detail: &'static str) -> WorthQueryProviderSessionFailure {
    WorthQueryProviderSessionFailure::new(
        WorthQueryProviderSessionDenialKind::ProviderRejected,
        WorthQueryProviderSessionProtocolStage::PlanReadmission,
        detail,
        WorthQueryProviderSessionProtocolCounters::default(),
    )
}
