use super::*;

struct MultiChunkProvider {
    advances: Arc<AtomicUsize>,
}

struct MultiChunkExecution {
    advances: Arc<AtomicUsize>,
}

impl WorthQueryGraphProviderExecution for MultiChunkExecution {
    fn advance(
        &mut self,
        step: &mut WorthQueryGraphProviderStep,
    ) -> Result<WorthQueryGraphProviderStepDisposition, WorthQueryGraphProviderFailure> {
        let ordinal = self.advances.fetch_add(1, Ordering::Relaxed);
        step.perform_work_unit(|| Ok(()))?;
        step.emit_projection_chunk(graph_material())
            .map_err(step_failure)?;
        if ordinal == 0 {
            Ok(WorthQueryGraphProviderStepDisposition::continue_work())
        } else {
            WorthQueryGraphProviderStepDisposition::complete("multi-chunk")
                .map_err(WorthQueryGraphProviderFailure::new)
        }
    }

    fn dispose(&mut self) -> Result<(), WorthQueryGraphProviderFailure> {
        Ok(())
    }
}

impl WorthQueryGraphParticipationProvider<ManagedGraph> for MultiChunkProvider {
    type Execution = MultiChunkExecution;

    fn execution_resource_support(
        &self,
    ) -> worth_query_admission::facade::resource_admission::WorthQueryExecutionResourceSupport {
        crate::domain_computation::provider_session::execution_resource_support("multi-chunk", 8)
    }

    fn begin(
        &self,
        _call: &WorthQueryGraphProviderCall,
        _start: &mut WorthQueryGraphProviderExecutionStart,
    ) -> Result<Self::Execution, WorthQueryGraphProviderFailure> {
        Ok(MultiChunkExecution {
            advances: Arc::clone(&self.advances),
        })
    }
}

#[test]
fn each_streamed_chunk_requires_consumption_before_the_next_provider_step() {
    let advances = Arc::new(AtomicUsize::new(0));
    let active = start_projection(MultiChunkProvider {
        advances: Arc::clone(&advances),
    });
    let first = expect_chunk(active.advance());
    assert_eq!(first.queue_depth(), 1);
    assert_eq!(first.queue_capacity(), 8);
    assert_eq!(advances.load(Ordering::Relaxed), 1);

    let active = match first.acknowledge() {
        WorthQueryDirectGraphStepOutcome::Continue(active) => active,
        _ => panic!("consuming the first chunk did not release the next step"),
    };
    let second = expect_chunk(active.advance());
    assert_eq!(advances.load(Ordering::Relaxed), 2);
    let completion = match second.acknowledge() {
        WorthQueryDirectGraphStepOutcome::Completed(completion) => completion,
        _ => panic!("consuming the final chunk did not complete"),
    };
    let stream = completion
        .receipt()
        .graph_read_stream_evidence()
        .expect("managed projection should seal stream evidence");
    assert_eq!(stream.chunk_count(), 2);
    assert_eq!(stream.row_count(), 2);
}

#[test]
fn stalled_consumer_cancellation_releases_the_chunk_before_terminal_cleanup() {
    let advances = Arc::new(AtomicUsize::new(0));
    let pending = expect_chunk(
        start_projection(MultiChunkProvider {
            advances: Arc::clone(&advances),
        })
        .advance(),
    );
    pending
        .request_cancellation(
            worth_runtime_bridge::facade::BridgeManagedExecutionCancellationReason::HostRequested,
        )
        .expect("pending chunk should retain the exact Signal request");
    let terminal = match pending.acknowledge() {
        WorthQueryDirectGraphStepOutcome::Cancelled(terminal) => terminal,
        _ => panic!("cancelled stalled consumer did not derive cancellation"),
    };
    assert_eq!(advances.load(Ordering::Relaxed), 1);
    assert_eq!(terminal.provider_work().interrupted_call_count(), 1);
    let cleanup = terminal
        .cleanup()
        .expect("cancelled stream should clean up");
    assert_eq!(
        cleanup.bridge().signal_terminal(),
        BridgeExecutionBasisSignalTerminal::Cancelled
    );
}

#[test]
fn exact_capacity_chunk_drains_before_completion() {
    let advances = Arc::new(AtomicUsize::new(0));
    let (running, graph) = managed_graph_run_with_provider(
        WorthQueryOperationGraphAccess::Project,
        WideChunkProvider {
            advances: Arc::clone(&advances),
        },
    );
    let active = running
        .begin_graph_execution(
            &graph,
            WorthQueryManagedGraphCallRequest::new(
                WorthQueryGraphProviderCallKind::Project,
                "exact-capacity",
            ),
        )
        .expect("wide provider should start");
    let pending = expect_chunk(active.advance());
    assert_eq!(pending.queue_depth(), 8);
    assert_eq!(pending.queue_capacity(), 8);
    let completion = match pending.acknowledge() {
        WorthQueryDirectGraphStepOutcome::Completed(completion) => completion,
        _ => panic!("an admitted exact-capacity chunk could not drain"),
    };
    assert_eq!(advances.load(Ordering::Relaxed), 1);
    assert_eq!(
        completion
            .receipt()
            .graph_read_stream_evidence()
            .expect("exact-capacity projection should seal stream evidence")
            .row_count(),
        8
    );
    let terminal = completion.into_running().completed().unwrap();
    assert_eq!(terminal.provider_work().retained_bytes(), 0);
    terminal
        .cleanup()
        .expect("completed stream should clean up");
}

struct WideChunkProvider {
    advances: Arc<AtomicUsize>,
}

struct WideChunkExecution {
    advances: Arc<AtomicUsize>,
}

impl WorthQueryGraphProviderExecution for WideChunkExecution {
    fn advance(
        &mut self,
        step: &mut WorthQueryGraphProviderStep,
    ) -> Result<WorthQueryGraphProviderStepDisposition, WorthQueryGraphProviderFailure> {
        self.advances.fetch_add(1, Ordering::Relaxed);
        step.perform_work_unit(|| Ok(()))?;
        step.emit_projection_chunk(graph_material_rows(8))
            .map_err(step_failure)?;
        WorthQueryGraphProviderStepDisposition::complete("wide-chunk")
            .map_err(WorthQueryGraphProviderFailure::new)
    }

    fn dispose(&mut self) -> Result<(), WorthQueryGraphProviderFailure> {
        Ok(())
    }
}

impl WorthQueryGraphParticipationProvider<ManagedGraph> for WideChunkProvider {
    type Execution = WideChunkExecution;

    fn execution_resource_support(
        &self,
    ) -> worth_query_admission::facade::resource_admission::WorthQueryExecutionResourceSupport {
        crate::domain_computation::provider_session::execution_resource_support("wide-chunk", 8)
    }

    fn begin(
        &self,
        _call: &WorthQueryGraphProviderCall,
        _start: &mut WorthQueryGraphProviderExecutionStart,
    ) -> Result<Self::Execution, WorthQueryGraphProviderFailure> {
        Ok(WideChunkExecution {
            advances: Arc::clone(&self.advances),
        })
    }
}

#[derive(Clone, Copy)]
enum HostilePort {
    Effect,
    Output,
    Scratch,
    Retained,
    Checkpoint,
    NoProgress,
}

struct HostileProvider(HostilePort);

struct HostileExecution(HostilePort);

impl WorthQueryGraphProviderExecution for HostileExecution {
    fn advance(
        &mut self,
        step: &mut WorthQueryGraphProviderStep,
    ) -> Result<WorthQueryGraphProviderStepDisposition, WorthQueryGraphProviderFailure> {
        match self.0 {
            HostilePort::Effect => {
                let _ = step.apply_effect(|| Ok(()));
            }
            HostilePort::Output => {
                step.perform_work_unit(|| Ok(()))?;
                let _ = step.emit_projection_chunk(graph_material_rows(9));
            }
            HostilePort::Scratch => {
                step.perform_work_unit(|| Ok(()))?;
                let _ = step.with_scratch_bytes(9, |_| Ok(()));
            }
            HostilePort::Retained => {
                step.perform_work_unit(|| Ok(()))?;
                let _ = step.retain_bytes(4_097);
            }
            HostilePort::Checkpoint => {
                step.perform_work_unit(|| Ok(()))?;
                step.record_checkpoint_available().map_err(step_failure)?;
                let _ = step.record_checkpoint_available();
            }
            HostilePort::NoProgress => {}
        }
        WorthQueryGraphProviderStepDisposition::complete("hostile")
            .map_err(WorthQueryGraphProviderFailure::new)
    }

    fn dispose(&mut self) -> Result<(), WorthQueryGraphProviderFailure> {
        Ok(())
    }
}

impl WorthQueryGraphParticipationProvider<ManagedGraph> for HostileProvider {
    type Execution = HostileExecution;

    fn execution_resource_support(
        &self,
    ) -> worth_query_admission::facade::resource_admission::WorthQueryExecutionResourceSupport {
        crate::domain_computation::provider_session::execution_resource_support("hostile-port", 8)
    }

    fn begin(
        &self,
        _call: &WorthQueryGraphProviderCall,
        _start: &mut WorthQueryGraphProviderExecutionStart,
    ) -> Result<Self::Execution, WorthQueryGraphProviderFailure> {
        Ok(HostileExecution(self.0))
    }
}

#[test]
fn ignored_governed_denials_and_zero_progress_completion_cannot_advance() {
    for (port, expected_denial) in [
        (
            HostilePort::Effect,
            WorthQueryGraphProviderStepDenialKind::UnexpectedEffect,
        ),
        (
            HostilePort::Output,
            WorthQueryGraphProviderStepDenialKind::ChunkWidthExceeded,
        ),
        (
            HostilePort::Scratch,
            WorthQueryGraphProviderStepDenialKind::ScratchBudgetExceeded,
        ),
        (
            HostilePort::Retained,
            WorthQueryGraphProviderStepDenialKind::RetainedBudgetExceeded,
        ),
        (
            HostilePort::Checkpoint,
            WorthQueryGraphProviderStepDenialKind::MultipleCheckpoints,
        ),
        (
            HostilePort::NoProgress,
            WorthQueryGraphProviderStepDenialKind::NoProgress,
        ),
    ] {
        let (access, kind) = if matches!(port, HostilePort::Output) {
            (
                WorthQueryOperationGraphAccess::Project,
                WorthQueryGraphProviderCallKind::Project,
            )
        } else {
            (
                WorthQueryOperationGraphAccess::Observe,
                WorthQueryGraphProviderCallKind::Observe,
            )
        };
        let (running, graph) = managed_graph_run_with_provider(access, HostileProvider(port));
        let active = running
            .begin_graph_execution(
                &graph,
                WorthQueryManagedGraphCallRequest::new(kind, "hostile-port"),
            )
            .expect("hostile fixture should reach its governed step");
        let terminal = match active.advance() {
            WorthQueryDirectGraphStepOutcome::Failed(terminal) => terminal,
            _ => panic!("hostile governed port advanced the managed lane"),
        };
        assert_eq!(terminal.provider_work().abandoned_call_count(), 1);
        let failure = terminal
            .provider_work()
            .last_step_failure()
            .expect("failed provider step should retain its exact cause");
        assert_eq!(
            failure.invocation(),
            WorthQueryGraphProviderStepInvocationDisposition::Returned
        );
        assert_eq!(failure.governed_denial_kind(), Some(expected_denial));
        terminal
            .cleanup()
            .expect("failed hostile step should clean up");
    }
}

fn start_projection(
    provider: MultiChunkProvider,
) -> crate::domain_computation::WorthQueryActiveDirectGraphExecution {
    let (running, graph) =
        managed_graph_run_with_provider(WorthQueryOperationGraphAccess::Project, provider);
    running
        .begin_graph_execution(
            &graph,
            WorthQueryManagedGraphCallRequest::new(
                WorthQueryGraphProviderCallKind::Project,
                "streamed-project",
            ),
        )
        .expect("streaming provider should start")
}

fn expect_chunk(
    outcome: WorthQueryDirectGraphStepOutcome,
) -> crate::domain_computation::WorthQueryPendingDirectGraphChunk {
    match outcome {
        WorthQueryDirectGraphStepOutcome::ChunkReady(pending) => pending,
        _ => panic!("provider did not expose a bounded pending chunk"),
    }
}

fn step_failure(
    denial: crate::domain_computation::WorthQueryGraphProviderStepDenial,
) -> WorthQueryGraphProviderFailure {
    WorthQueryGraphProviderFailure::new(denial.detail())
}
