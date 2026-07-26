use std::collections::BTreeSet;

use super::*;
use crate::facade::provider_session::WorthQueryProviderWorkReport;

#[derive(Clone, Copy)]
struct RepeatedYieldProvider {
    step_count: u8,
}

struct RepeatedYieldExecution {
    next_step: u8,
    step_count: u8,
    retained: Option<WorthQueryGraphProviderRetainedMemory>,
}

struct RepeatedYieldCheckpoint {
    next_step: u8,
    step_count: u8,
    retained: WorthQueryGraphProviderRetainedMemory,
}

impl WorthQueryGraphParticipationProvider<ManagedGraph> for RepeatedYieldProvider {
    type Execution = RepeatedYieldExecution;

    fn execution_resource_support(
        &self,
    ) -> worth_query_admission::facade::resource_admission::WorthQueryExecutionResourceSupport {
        crate::domain_computation::provider_session::execution_resource_support_with_yield(
            "repeated-yield-parity",
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
            RepeatedYieldExecution {
                next_step: 0,
                step_count: self.step_count,
                retained: None,
            },
        )
    }
}

impl WorthQueryGraphProviderExecution for RepeatedYieldExecution {
    fn advance(
        &mut self,
        step: &mut WorthQueryGraphProviderStep,
    ) -> Result<WorthQueryGraphProviderStepDisposition, WorthQueryGraphProviderFailure> {
        step.perform_work_unit(|| Ok(()))?;
        self.next_step = self.next_step.saturating_add(1);
        if self.next_step < self.step_count {
            if self.retained.is_none() {
                self.retained = Some(step.retain_bytes(1).map_err(step_failure)?);
            }
            step.record_checkpoint_available().map_err(step_failure)?;
            return Ok(WorthQueryGraphProviderStepDisposition::continue_work());
        }
        drop(self.retained.take());
        WorthQueryGraphProviderStepDisposition::complete("repeated-yield-parity-complete")
            .map_err(WorthQueryGraphProviderFailure::new)
    }

    fn suspend(
        &mut self,
    ) -> Result<
        Box<dyn crate::domain_computation::WorthQueryGraphProviderCheckpoint>,
        WorthQueryGraphProviderFailure,
    > {
        Ok(Box::new(RepeatedYieldCheckpoint {
            next_step: self.next_step,
            step_count: self.step_count,
            retained: self
                .retained
                .take()
                .expect("checkpointable execution retains governed memory"),
        }))
    }

    fn dispose(&mut self) -> Result<(), WorthQueryGraphProviderFailure> {
        Ok(())
    }
}

impl crate::domain_computation::WorthQueryGraphProviderCheckpoint for RepeatedYieldCheckpoint {
    fn retained_bytes(&self) -> u64 {
        u64::try_from(self.retained.len()).unwrap()
    }

    fn restore(
        &self,
        _call: &WorthQueryGraphProviderCall,
        memory: &mut WorthQueryGraphProviderRestoreMemory,
    ) -> Result<
        WorthQueryCooperativeGraphProviderExecution<Box<dyn WorthQueryGraphProviderExecution>>,
        WorthQueryGraphProviderFailure,
    > {
        let execution = Box::new(RepeatedYieldExecution {
            next_step: self.next_step,
            step_count: self.step_count,
            retained: Some(memory.rebind(&self.retained).map_err(step_failure)?),
        }) as Box<dyn WorthQueryGraphProviderExecution>;
        admit_restored_provider_execution(memory, execution)
    }
}

#[test]
fn repeated_readmission_matches_uninterrupted_semantics_and_structural_evidence() {
    let uninterrupted = execute_repeated_yield_world(false);
    let resumed = execute_repeated_yield_world(true);

    assert_eq!(resumed.readmission_count, 2);
    assert_eq!(resumed.unique_managed_attempt_count, 3);
    assert_eq!(resumed.unique_resource_attempt_count, 3);
    assert_eq!(resumed.unique_bridge_request_count, 3);
    assert_eq!(resumed.capacity_reservation_count, 2);
    assert_eq!(resumed.provider_receipt, uninterrupted.provider_receipt);
    assert_eq!(resumed.work_report, uninterrupted.work_report);
    assert_eq!(resumed.provider_work, uninterrupted.provider_work);
    assert_eq!(resumed.run_counters, uninterrupted.run_counters);
}

struct ParityEvidence {
    provider_receipt: String,
    work_report: WorthQueryProviderWorkReport,
    provider_work: ProviderWorkEvidence,
    run_counters: crate::domain_computation::WorthQueryManagedRunCounters,
    readmission_count: usize,
    unique_managed_attempt_count: usize,
    unique_resource_attempt_count: usize,
    unique_bridge_request_count: usize,
    capacity_reservation_count: usize,
}

#[derive(Debug, Eq, PartialEq)]
struct ProviderWorkEvidence {
    completed_work_units: u64,
    applied_effect_count: u64,
    produced_artifact_count: usize,
    retained_artifact_count: usize,
    disposed_artifact_count: usize,
    peak_scratch_bytes: usize,
    retained_bytes: usize,
    provider_step_attempt_count: usize,
    safe_point_request_lookup_count: usize,
    pressure_classification_count: usize,
    output_capacity_classification_count: usize,
    queue_state_mutation_count: usize,
}

fn execute_repeated_yield_world(resume_every_safe_point: bool) -> ParityEvidence {
    let (running, graph, bridge, runtime) = managed_graph_run_with_provider_and_runtime(
        WorthQueryOperationGraphAccess::Observe,
        RepeatedYieldProvider { step_count: 3 },
    );
    let active = running
        .begin_graph_execution(
            &graph,
            WorthQueryManagedGraphCallRequest::new(
                WorthQueryGraphProviderCallKind::Observe,
                "repeated-yield-parity",
            ),
        )
        .expect("parity provider should begin");
    let mut managed_attempts = BTreeSet::from([active.run_identity().to_owned()]);
    let mut resource_attempts = BTreeSet::from([active.resource_attempt_identity().to_owned()]);
    let mut bridge_requests = BTreeSet::from([active.bridge_request_identity().to_owned()]);
    let capacity_reservation_count = active.retained_capacity_reservation_count();
    let mut readmission_count = 0;
    let mut outcome = active.advance();

    loop {
        outcome = match outcome {
            WorthQueryDirectGraphStepOutcome::Continue(paused) if resume_every_safe_point => {
                let yielded = match paused.yield_run() {
                    crate::domain_computation::WorthQueryDirectYieldOutcome::Yielded(yielded) => {
                        yielded
                    }
                    _ => panic!("parity safe point should yield"),
                };
                let active = match yielded.readmit_same_runtime(&runtime, &bridge) {
                    crate::domain_computation::WorthQueryDirectReadmissionOutcome::Readmitted(
                        active,
                    ) => active,
                    _ => panic!("parity yielded run should readmit"),
                };
                readmission_count += 1;
                managed_attempts.insert(active.run_identity().to_owned());
                resource_attempts.insert(active.resource_attempt_identity().to_owned());
                bridge_requests.insert(active.bridge_request_identity().to_owned());
                assert_eq!(
                    active.retained_capacity_reservation_count(),
                    capacity_reservation_count
                );
                active.advance()
            }
            WorthQueryDirectGraphStepOutcome::Continue(paused) => paused.advance(),
            WorthQueryDirectGraphStepOutcome::Completed(completion) => {
                return parity_evidence(
                    completion,
                    managed_attempts,
                    resource_attempts,
                    bridge_requests,
                    readmission_count,
                    capacity_reservation_count,
                );
            }
            _ => panic!("parity provider should continue or complete"),
        };
    }
}

fn parity_evidence(
    completion: crate::domain_computation::WorthQueryCompletedDirectGraphExecution,
    managed_attempts: BTreeSet<String>,
    resource_attempts: BTreeSet<String>,
    bridge_requests: BTreeSet<String>,
    readmission_count: usize,
    capacity_reservation_count: usize,
) -> ParityEvidence {
    let provider_receipt = completion.receipt().provider_receipt().to_owned();
    let work_report = completion.receipt().work_report();
    let terminal = completion.into_running().completed().unwrap();
    let provider_work = terminal.provider_work();
    let evidence = ParityEvidence {
        provider_receipt,
        work_report,
        provider_work: ProviderWorkEvidence {
            completed_work_units: provider_work.completed_work_units(),
            applied_effect_count: provider_work.applied_effect_count(),
            produced_artifact_count: provider_work.produced_artifact_count(),
            retained_artifact_count: provider_work.retained_artifact_count(),
            disposed_artifact_count: provider_work.disposed_artifact_count(),
            peak_scratch_bytes: provider_work.peak_scratch_bytes(),
            retained_bytes: provider_work.retained_bytes(),
            provider_step_attempt_count: provider_work.provider_step_attempt_count(),
            safe_point_request_lookup_count: provider_work.safe_point_request_lookup_count(),
            pressure_classification_count: provider_work.pressure_classification_count(),
            output_capacity_classification_count: provider_work
                .output_capacity_classification_count(),
            queue_state_mutation_count: provider_work.queue_state_mutation_count(),
        },
        run_counters: terminal.counters().clone(),
        readmission_count,
        unique_managed_attempt_count: managed_attempts.len(),
        unique_resource_attempt_count: resource_attempts.len(),
        unique_bridge_request_count: bridge_requests.len(),
        capacity_reservation_count,
    };
    assert!(terminal.cleanup().is_ok());
    evidence
}

fn step_failure(
    denial: crate::domain_computation::WorthQueryGraphProviderStepDenial,
) -> WorthQueryGraphProviderFailure {
    WorthQueryGraphProviderFailure::new(denial.detail())
}
