use super::*;

struct TwoStepProvider;

struct TwoStepExecution {
    phase: u8,
    retained: Option<WorthQueryGraphProviderRetainedMemory>,
}

impl WorthQueryGraphProviderExecution for TwoStepExecution {
    fn advance(
        &mut self,
        step: &mut WorthQueryGraphProviderStep,
    ) -> Result<WorthQueryGraphProviderStepDisposition, WorthQueryGraphProviderFailure> {
        match self.phase {
            0 => {
                self.phase = 1;
                for _ in 0..3 {
                    step.perform_work_unit(|| Ok(()))?;
                }
                step.with_scratch_bytes(8, |_| Ok(()))?;
                self.retained = Some(step.retain_bytes(4).map_err(step_failure)?);
                Ok(WorthQueryGraphProviderStepDisposition::continue_work())
            }
            1 => {
                self.phase = 2;
                drop(self.retained.take());
                for _ in 0..2 {
                    step.perform_work_unit(|| Ok(()))?;
                }
                step.emit_projection_chunk(graph_material())
                    .map_err(step_failure)?;
                WorthQueryGraphProviderStepDisposition::complete("two-step-provider")
                    .map_err(WorthQueryGraphProviderFailure::new)
            }
            _ => Err(WorthQueryGraphProviderFailure::new(
                "completed provider advanced again",
            )),
        }
    }

    fn dispose(&mut self) -> Result<(), WorthQueryGraphProviderFailure> {
        Ok(())
    }
}

impl WorthQueryGraphParticipationProvider<ManagedGraph> for TwoStepProvider {
    type Execution = TwoStepExecution;

    fn execution_resource_support(
        &self,
    ) -> worth_query_admission::facade::resource_admission::WorthQueryExecutionResourceSupport {
        crate::domain_computation::provider_session::execution_resource_support(
            "managed-two-step",
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
            TwoStepExecution {
                phase: 0,
                retained: None,
            },
        )
    }
}

struct FailingProvider;

struct FailingExecution {
    retained: Option<WorthQueryGraphProviderRetainedMemory>,
}

impl WorthQueryGraphProviderExecution for FailingExecution {
    fn advance(
        &mut self,
        step: &mut WorthQueryGraphProviderStep,
    ) -> Result<WorthQueryGraphProviderStepDisposition, WorthQueryGraphProviderFailure> {
        for _ in 0..2 {
            step.perform_work_unit(|| Ok(()))?;
        }
        self.retained = Some(step.retain_bytes(4).map_err(step_failure)?);
        Err(WorthQueryGraphProviderFailure::new(
            "provider failed after governed work",
        ))
    }

    fn dispose(&mut self) -> Result<(), WorthQueryGraphProviderFailure> {
        Ok(())
    }
}

impl WorthQueryGraphParticipationProvider<ManagedGraph> for FailingProvider {
    type Execution = FailingExecution;

    fn execution_resource_support(
        &self,
    ) -> worth_query_admission::facade::resource_admission::WorthQueryExecutionResourceSupport {
        crate::domain_computation::provider_session::execution_resource_support(
            "managed-failing",
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
        admit_provider_execution(start, FailingExecution { retained: None })
    }
}

#[derive(Clone, Copy)]
enum StartBehavior {
    Retain,
    Deny,
    Panic,
}

struct StartProvider(StartBehavior);

struct StartExecution {
    _retained: WorthQueryGraphProviderRetainedMemory,
}

impl WorthQueryGraphProviderExecution for StartExecution {
    fn advance(
        &mut self,
        step: &mut WorthQueryGraphProviderStep,
    ) -> Result<WorthQueryGraphProviderStepDisposition, WorthQueryGraphProviderFailure> {
        step.perform_work_unit(|| Ok(()))?;
        WorthQueryGraphProviderStepDisposition::complete("start-retained")
            .map_err(WorthQueryGraphProviderFailure::new)
    }

    fn dispose(&mut self) -> Result<(), WorthQueryGraphProviderFailure> {
        Ok(())
    }
}

impl WorthQueryGraphParticipationProvider<ManagedGraph> for StartProvider {
    type Execution = StartExecution;

    fn execution_resource_support(
        &self,
    ) -> worth_query_admission::facade::resource_admission::WorthQueryExecutionResourceSupport {
        crate::domain_computation::provider_session::execution_resource_support(
            "managed-provider-start",
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
        match self.0 {
            StartBehavior::Retain => {
                let execution = StartExecution {
                    _retained: start.retain_bytes(4).map_err(step_failure)?,
                };
                admit_provider_execution(start, execution)
            }
            StartBehavior::Deny => {
                let _ = start.retain_bytes(4_097);
                Err(WorthQueryGraphProviderFailure::new(
                    "provider returned after ignoring start denial",
                ))
            }
            StartBehavior::Panic => {
                let _retained = start.retain_bytes(4).map_err(step_failure)?;
                panic!("provider construction panicked after governed retention")
            }
        }
    }
}

#[test]
fn bounded_provider_steps_stream_exact_terminal_work_evidence() {
    let (running, graph) =
        managed_graph_run_with_provider(WorthQueryOperationGraphAccess::Project, TwoStepProvider);
    let active = running
        .begin_graph_execution(
            &graph,
            WorthQueryManagedGraphCallRequest::new(
                WorthQueryGraphProviderCallKind::Project,
                "managed-project",
            ),
        )
        .expect("installed provider anchor should start exact execution");
    let active = match active.advance() {
        WorthQueryDirectGraphStepOutcome::Continue(active) => active,
        _ => panic!("first bounded step did not continue"),
    };
    let pending = match active.advance() {
        WorthQueryDirectGraphStepOutcome::ChunkReady(pending) => pending,
        _ => panic!("second bounded step did not expose its bounded result chunk"),
    };
    assert_eq!(pending.chunk().rows().len(), 1);
    let completion = match pending.acknowledge() {
        WorthQueryDirectGraphStepOutcome::Completed(completion) => completion,
        _ => panic!("acknowledged final chunk did not complete"),
    };
    let stream = completion
        .receipt()
        .graph_read_stream_evidence()
        .expect("managed projection completion should carry stream evidence");
    assert_eq!(stream.chunk_count(), 1);
    assert_eq!(stream.row_count(), 1);
    assert!(completion.receipt().graph_read_product().is_none());
    assert_eq!(completion.receipt().work_report().completed_work_units(), 5);
    assert_eq!(completion.receipt().work_report().scratch_bytes(), 8);
    assert_eq!(completion.receipt().work_report().retained_bytes(), 0);

    let terminal = completion
        .into_running()
        .completed()
        .expect("Query-settled provider work should permit run completion");
    let work = terminal.provider_work().clone();
    assert_eq!(
        work.session_disposition(),
        WorthQueryManagedProviderSessionDisposition::ReceiptsAdmitted
    );
    assert_eq!(work.issued_call_count(), 1);
    assert_eq!(work.admitted_receipt_count(), 1);
    assert_eq!(work.completed_work_units(), 5);
    assert_eq!(work.peak_scratch_bytes(), 8);
    assert_eq!(work.retained_bytes(), 0);
    assert!(work.peak_retained_bytes() >= 4);
    assert_eq!(work.output_capacity_classification_count(), 2);
    let last_safe_point = work
        .last_safe_point()
        .expect("terminal work retains the exact last Signal safe point");
    assert_eq!(
        last_safe_point.signal_state(),
        worth_runtime_bridge::facade::BridgeExecutionSafePointSignalState::Active
    );
    assert_eq!(last_safe_point.queue_depth(), 0);
    assert_eq!(last_safe_point.queue_capacity(), 8);
    let cleanup = terminal.cleanup().expect("settled provider work cleans up");
    assert_eq!(
        cleanup.disposition(),
        WorthQueryManagedRunCleanupDisposition::CleanupComplete
    );
}

#[test]
fn provider_failure_preserves_governed_work_and_recovery_authority() {
    let (running, graph) =
        managed_graph_run_with_provider(WorthQueryOperationGraphAccess::Observe, FailingProvider);
    let active = running
        .begin_graph_execution(
            &graph,
            WorthQueryManagedGraphCallRequest::new(
                WorthQueryGraphProviderCallKind::Observe,
                "failing-observe",
            ),
        )
        .expect("failing provider should start before its bounded work");
    let terminal = match active.advance() {
        WorthQueryDirectGraphStepOutcome::Failed(terminal) => terminal,
        _ => panic!("provider failure did not terminalize the managed run"),
    };
    assert_eq!(terminal.provider_work().completed_work_units(), 2);
    assert_eq!(terminal.provider_work().retained_bytes(), 0);
    assert_eq!(terminal.provider_work().peak_retained_bytes(), 4);
    assert_eq!(terminal.provider_work().abandoned_call_count(), 1);
    let cleanup = terminal
        .cleanup()
        .expect("failed provider run retains cleanup authority");
    assert_eq!(
        cleanup.disposition(),
        WorthQueryManagedRunCleanupDisposition::RecoveryRequired
    );
}

#[test]
fn governed_provider_start_classifies_denial_and_panic_before_execution() {
    let (running, graph) = managed_graph_run_with_provider(
        WorthQueryOperationGraphAccess::Observe,
        StartProvider(StartBehavior::Deny),
    );
    let denial = match running.begin_graph_execution(
        &graph,
        WorthQueryManagedGraphCallRequest::new(
            WorthQueryGraphProviderCallKind::Observe,
            "start-denial",
        ),
    ) {
        Ok(_) => panic!("ignored start-memory denial admitted provider construction"),
        Err(denial) => denial,
    };
    assert_eq!(
        denial.kind(),
        crate::domain_computation::WorthQueryDirectGraphExecutionStartFailureKind::
            ProviderStartContractDenied
    );
    assert_eq!(denial.provider_retained_bytes(), 0);
    assert_eq!(denial.provider_retained_allocation_count(), 0);

    let (running, graph) = managed_graph_run_with_provider(
        WorthQueryOperationGraphAccess::Observe,
        StartProvider(StartBehavior::Panic),
    );
    let panic = match running.begin_graph_execution(
        &graph,
        WorthQueryManagedGraphCallRequest::new(
            WorthQueryGraphProviderCallKind::Observe,
            "start-panic",
        ),
    ) {
        Ok(_) => panic!("provider construction panic escaped typed failure"),
        Err(panic) => panic,
    };
    assert_eq!(
        panic.kind(),
        crate::domain_computation::WorthQueryDirectGraphExecutionStartFailureKind::
            ProviderStartPanicked
    );
    assert_eq!(panic.provider_retained_bytes(), 0);
    assert_eq!(panic.provider_retained_allocation_count(), 0);
}

#[test]
fn active_abandonment_releases_start_retention_and_preserves_peak_evidence() {
    let (running, graph) = managed_graph_run_with_provider(
        WorthQueryOperationGraphAccess::Observe,
        StartProvider(StartBehavior::Retain),
    );
    let active = running
        .begin_graph_execution(
            &graph,
            WorthQueryManagedGraphCallRequest::new(
                WorthQueryGraphProviderCallKind::Observe,
                "abandon-start-retention",
            ),
        )
        .expect("governed start retention should admit within the installed ceiling");
    let terminal = match active.abandon() {
        WorthQueryDirectGraphStepOutcome::Failed(terminal) => terminal,
        _ => panic!("explicit active abandonment must produce a failed terminal"),
    };
    assert_eq!(terminal.provider_work().retained_bytes(), 0);
    assert_eq!(terminal.provider_work().peak_retained_bytes(), 4);
    assert_eq!(terminal.provider_work().abandoned_call_count(), 1);
    assert_eq!(
        terminal
            .provider_work()
            .provider_execution_release()
            .release_count(),
        1
    );
    let cleanup = terminal
        .cleanup()
        .expect("explicit abandonment preserves cleanup authority");
    assert_eq!(
        cleanup.disposition(),
        WorthQueryManagedRunCleanupDisposition::RecoveryRequired
    );
}

fn step_failure(
    denial: crate::domain_computation::WorthQueryGraphProviderStepDenial,
) -> WorthQueryGraphProviderFailure {
    WorthQueryGraphProviderFailure::new(denial.detail())
}
