use super::*;

const UNRELATED_WIDTH: usize = 12;

struct CostSlopeProvider {
    advances: Arc<AtomicUsize>,
}

struct CostSlopeExecution {
    advances: Arc<AtomicUsize>,
}

impl WorthQueryGraphProviderExecution for CostSlopeExecution {
    fn advance(
        &mut self,
        step: &mut WorthQueryGraphProviderStep,
    ) -> Result<WorthQueryGraphProviderStepDisposition, WorthQueryGraphProviderFailure> {
        self.advances.fetch_add(1, Ordering::Relaxed);
        step.perform_work_unit(|| Ok(()))?;
        step.emit_projection_chunk(graph_material())
            .map_err(step_failure)?;
        WorthQueryGraphProviderStepDisposition::complete("cost-slope")
            .map_err(WorthQueryGraphProviderFailure::new)
    }

    fn dispose(&mut self) -> Result<(), WorthQueryGraphProviderFailure> {
        Ok(())
    }
}

impl WorthQueryGraphParticipationProvider<ManagedGraph> for CostSlopeProvider {
    type Execution = CostSlopeExecution;

    fn execution_resource_support(
        &self,
    ) -> worth_query_admission::facade::resource_admission::WorthQueryExecutionResourceSupport {
        crate::domain_computation::provider_session::execution_resource_support(
            "cost-slope-provider",
            8,
        )
    }

    fn begin(
        &self,
        _call: &WorthQueryGraphProviderCall,
        _start: &mut WorthQueryGraphProviderExecutionStart,
    ) -> Result<Self::Execution, WorthQueryGraphProviderFailure> {
        Ok(CostSlopeExecution {
            advances: Arc::clone(&self.advances),
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct StepCostEvidence {
    provider_steps: usize,
    safe_point_lookups: usize,
    pressure_classifications: usize,
    output_capacity_classifications: usize,
    queue_lookups: usize,
    queue_mutations: usize,
    retained_bytes: usize,
}

#[test]
fn one_provider_step_has_constant_work_under_unrelated_authority_width() {
    let baseline = execute_target(0);
    let wide = execute_target(UNRELATED_WIDTH);
    assert_eq!(baseline, wide);
    assert_eq!(
        baseline,
        StepCostEvidence {
            provider_steps: 1,
            safe_point_lookups: 3,
            pressure_classifications: 3,
            output_capacity_classifications: 1,
            queue_lookups: 2,
            queue_mutations: 2,
            retained_bytes: 0,
        }
    );
}

#[test]
fn one_provider_step_allocation_count_is_width_invariant() {
    let output = std::process::Command::new(std::env::current_exe().unwrap())
        .arg("isolated_provider_step_allocation_slope_probe")
        .env("WORTH_QUERY_STEP_ALLOCATION_PROBE", "1")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "provider-step allocation probe failed:\n{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );
}

#[test]
fn isolated_provider_step_allocation_slope_probe() {
    if std::env::var_os("WORTH_QUERY_STEP_ALLOCATION_PROBE").is_none() {
        return;
    }
    let baseline = measured_target(0);
    let wide = measured_target(UNRELATED_WIDTH);
    assert_eq!(baseline.allocations, wide.allocations);
    assert_eq!(baseline.reallocations, wide.reallocations);
}

fn execute_target(unrelated_width: usize) -> StepCostEvidence {
    let (active, unrelated, advances) = prepared_target(unrelated_width);
    let completion = complete_target(active);
    assert_eq!(advances.load(Ordering::Relaxed), 1);
    let terminal = completion.into_running().completed().unwrap();
    let work = terminal.provider_work();
    let evidence = StepCostEvidence {
        provider_steps: work.provider_step_attempt_count(),
        safe_point_lookups: work.safe_point_request_lookup_count(),
        pressure_classifications: work.pressure_classification_count(),
        output_capacity_classifications: work.output_capacity_classification_count(),
        queue_lookups: work.queue_request_lookup_count(),
        queue_mutations: work.queue_state_mutation_count(),
        retained_bytes: work.retained_bytes(),
    };
    terminal.cleanup().expect("cost target should clean up");
    drop(unrelated);
    evidence
}

fn measured_target(unrelated_width: usize) -> stats_alloc::Stats {
    let (active, unrelated, _) = prepared_target(unrelated_width);
    let region = stats_alloc::Region::new(&stats_alloc::INSTRUMENTED_SYSTEM);
    let completion = complete_target(active);
    let stats = region.change();
    completion
        .into_running()
        .completed()
        .unwrap()
        .cleanup()
        .expect("measured target should clean up");
    drop(unrelated);
    stats
}

fn prepared_target(
    unrelated_width: usize,
) -> (
    crate::domain_computation::WorthQueryActiveDirectGraphExecution,
    Vec<(
        crate::domain_computation::WorthQueryRunningWorkflowRun,
        crate::domain_computation::WorthQueryMoveOnlyArtifactHandle,
    )>,
    Arc<AtomicUsize>,
) {
    let disposed = Arc::new(AtomicUsize::new(0));
    let unrelated = (0..unrelated_width)
        .map(|index| super::cost_bound::unrelated_artifact_run(index, Arc::clone(&disposed)))
        .collect();
    let advances = Arc::new(AtomicUsize::new(0));
    let (running, graph) = managed_graph_run_with_provider(
        WorthQueryOperationGraphAccess::Project,
        CostSlopeProvider {
            advances: Arc::clone(&advances),
        },
    );
    let active = running
        .begin_graph_execution(
            &graph,
            WorthQueryManagedGraphCallRequest::new(
                WorthQueryGraphProviderCallKind::Project,
                "cost-slope",
            ),
        )
        .expect("cost-slope provider should start");
    (active, unrelated, advances)
}

fn complete_target(
    active: crate::domain_computation::WorthQueryActiveDirectGraphExecution,
) -> crate::domain_computation::WorthQueryCompletedDirectGraphExecution {
    let pending = match active.advance() {
        WorthQueryDirectGraphStepOutcome::ChunkReady(pending) => pending,
        _ => panic!("cost-slope provider did not expose its chunk"),
    };
    match pending.acknowledge() {
        WorthQueryDirectGraphStepOutcome::Completed(completion) => completion,
        _ => panic!("cost-slope provider did not complete after drain"),
    }
}

fn step_failure(
    denial: crate::domain_computation::WorthQueryGraphProviderStepDenial,
) -> WorthQueryGraphProviderFailure {
    WorthQueryGraphProviderFailure::new(denial.detail())
}
