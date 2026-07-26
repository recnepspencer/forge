use std::sync::Mutex;

use super::*;

struct EscapingMemoryProvider {
    retained: Arc<Mutex<Vec<WorthQueryGraphProviderRetainedMemory>>>,
}

struct EscapingMemoryExecution {
    retained: Arc<Mutex<Vec<WorthQueryGraphProviderRetainedMemory>>>,
}

impl WorthQueryGraphProviderExecution for EscapingMemoryExecution {
    fn advance(
        &mut self,
        step: &mut WorthQueryGraphProviderStep,
    ) -> Result<WorthQueryGraphProviderStepDisposition, WorthQueryGraphProviderFailure> {
        step.perform_work_unit(|| Ok(()))?;
        self.retained
            .lock()
            .expect("retained-memory fixture lock should remain available")
            .push(step.retain_bytes(32).map_err(step_failure)?);
        WorthQueryGraphProviderStepDisposition::complete("escaping-memory")
            .map_err(WorthQueryGraphProviderFailure::new)
    }

    fn dispose(&mut self) -> Result<(), WorthQueryGraphProviderFailure> {
        Ok(())
    }
}

impl WorthQueryGraphParticipationProvider<ManagedGraph> for EscapingMemoryProvider {
    type Execution = EscapingMemoryExecution;

    fn execution_resource_support(
        &self,
    ) -> worth_query_admission::facade::resource_admission::WorthQueryExecutionResourceSupport {
        crate::domain_computation::provider_session::execution_resource_support(
            "escaping-provider-memory",
            8,
        )
    }

    fn begin(
        &self,
        _call: &WorthQueryGraphProviderCall,
        start: &mut WorthQueryGraphProviderExecutionStart,
    ) -> Result<
        WorthQueryCooperativeGraphProviderExecution<Self::Execution>,
        WorthQueryGraphProviderFailure,
    > {
        admit_provider_execution(
            start,
            EscapingMemoryExecution {
                retained: Arc::clone(&self.retained),
            },
        )
    }
}

struct EscapingStartProvider {
    retained: Arc<Mutex<Vec<WorthQueryGraphProviderRetainedMemory>>>,
}

struct NeverStartedExecution;

impl WorthQueryGraphProviderExecution for NeverStartedExecution {
    fn advance(
        &mut self,
        _step: &mut WorthQueryGraphProviderStep,
    ) -> Result<WorthQueryGraphProviderStepDisposition, WorthQueryGraphProviderFailure> {
        Err(WorthQueryGraphProviderFailure::new(
            "rejected provider must never advance",
        ))
    }

    fn dispose(&mut self) -> Result<(), WorthQueryGraphProviderFailure> {
        Ok(())
    }
}

impl WorthQueryGraphParticipationProvider<ManagedGraph> for EscapingStartProvider {
    type Execution = NeverStartedExecution;

    fn execution_resource_support(
        &self,
    ) -> worth_query_admission::facade::resource_admission::WorthQueryExecutionResourceSupport {
        crate::domain_computation::provider_session::execution_resource_support(
            "escaping-start-memory",
            8,
        )
    }

    fn begin(
        &self,
        _call: &WorthQueryGraphProviderCall,
        start: &mut WorthQueryGraphProviderExecutionStart,
    ) -> Result<
        WorthQueryCooperativeGraphProviderExecution<Self::Execution>,
        WorthQueryGraphProviderFailure,
    > {
        self.retained
            .lock()
            .expect("start-memory fixture lock should remain available")
            .push(start.retain_bytes(24).map_err(step_failure)?);
        Err(WorthQueryGraphProviderFailure::new(
            "provider rejected after escaping governed start memory",
        ))
    }
}

#[test]
fn multiple_provider_calls_retain_every_live_arena_until_cleanup() {
    let retained = Arc::new(Mutex::new(Vec::new()));
    let (running, graph) = managed_graph_run_with_provider(
        WorthQueryOperationGraphAccess::Observe,
        EscapingMemoryProvider {
            retained: Arc::clone(&retained),
        },
    );
    let running = complete_call(running, &graph, "first-memory-call");
    let running = complete_call(running, &graph, "second-memory-call");
    let terminal = running
        .completed()
        .expect("receipt-bound calls should complete logically");

    assert!(terminal.provider_work().provider_retained_bytes() >= 64);
    let first_pending = terminal
        .cleanup()
        .expect_err("two escaped arenas must keep cleanup pending");
    assert_eq!(
        first_pending.failure_kind(),
        WorthQueryManagedRunCleanupFailureKind::ProviderMemoryRetained
    );
    assert_eq!(
        first_pending.disposition(),
        WorthQueryManagedRunCleanupDisposition::CleanupPending
    );
    let first_bytes = first_pending.provider_retained_bytes();

    let first = retained
        .lock()
        .expect("retained-memory fixture lock should remain available")
        .remove(0);
    drop(first);
    let second_pending = first_pending
        .retry()
        .expect_err("the second escaped arena must remain pending");
    assert!(second_pending.provider_retained_bytes() < first_bytes);
    retained
        .lock()
        .expect("retained-memory fixture lock should remain available")
        .clear();

    let cleanup = second_pending
        .retry()
        .expect("released arenas should permit exact cleanup");
    assert_eq!(cleanup.provider_work().provider_retained_bytes(), 0);
    assert!(cleanup.provider_work().peak_retained_bytes() >= first_bytes);
    assert_eq!(cleanup.attempt().capacity().released_reservation_count(), 2);
}

#[test]
fn provider_start_failure_preserves_escaped_arena_recovery() {
    let retained = Arc::new(Mutex::new(Vec::new()));
    let (running, graph) = managed_graph_run_with_provider(
        WorthQueryOperationGraphAccess::Observe,
        EscapingStartProvider {
            retained: Arc::clone(&retained),
        },
    );
    let failure = match running.begin_graph_execution(
        &graph,
        WorthQueryManagedGraphCallRequest::new(
            WorthQueryGraphProviderCallKind::Observe,
            "escaping-start-memory",
        ),
    ) {
        Err(failure) => failure,
        Ok(_) => panic!("provider start should reject"),
    };
    assert_eq!(
        failure.kind(),
        crate::domain_computation::WorthQueryDirectGraphExecutionStartFailureKind::ProviderStartMemoryLeaked
    );
    assert!(failure.provider_retained_bytes() >= 24);
    let terminal = failure
        .into_running()
        .terminate_for_convergence(WorthQueryManagedRunTerminalKind::Failed);
    let pending = terminal
        .cleanup()
        .expect_err("escaped start memory must keep cleanup pending");
    assert_eq!(
        pending.failure_kind(),
        WorthQueryManagedRunCleanupFailureKind::ProviderMemoryRetained
    );

    retained
        .lock()
        .expect("start-memory fixture lock should remain available")
        .clear();
    let cleanup = pending
        .retry()
        .expect("released start memory should permit physical cleanup");
    assert_eq!(cleanup.provider_work().provider_retained_bytes(), 0);
    assert_eq!(cleanup.attempt().capacity().released_reservation_count(), 2);
}

fn complete_call(
    running: WorthQueryRunningDirectRun,
    graph: &WorthQueryInstalledGraphParticipationAuthority,
    scope: &str,
) -> WorthQueryRunningDirectRun {
    let active = running
        .begin_graph_execution(
            graph,
            WorthQueryManagedGraphCallRequest::new(WorthQueryGraphProviderCallKind::Observe, scope),
        )
        .expect("provider call should start");
    match active.advance() {
        WorthQueryDirectGraphStepOutcome::Completed(completion) => completion.into_running(),
        _ => panic!("escaping-memory provider did not complete"),
    }
}

fn step_failure(
    denial: crate::domain_computation::WorthQueryGraphProviderStepDenial,
) -> WorthQueryGraphProviderFailure {
    WorthQueryGraphProviderFailure::new(denial.detail())
}
