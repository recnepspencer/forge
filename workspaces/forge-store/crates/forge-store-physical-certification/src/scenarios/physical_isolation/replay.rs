use crate::{
    admit_physical_counter_evidence, CertifiedPhysicalScenario, ExecutedTranscriptParts,
    GeneratedCoverageMatrix, HarnessCoverageStage, ObservedPhysicalTrace, PhysicalCoverageRegistry,
    PhysicalExecutedCounterEvidence, PhysicalFaultEvent, PhysicalInterleavingSchedule,
    PhysicalIsolationInterleavingOracle, PhysicalIsolationMutationEvidence,
    PhysicalProofOracleVerdict, PhysicalSimulationPlan, PhysicalSimulationTranscript,
    ProductionBackedPhysicalFixture, ReusablePhysicalOracleFamily, SimulationReplayBundle,
};

pub fn assemble_physical_isolation_replay_bundle(
    plan: &PhysicalSimulationPlan,
    schedule: PhysicalInterleavingSchedule,
    fixture: &ProductionBackedPhysicalFixture,
    trace: ObservedPhysicalTrace,
    expected_fault: crate::PhysicalScenarioFaultKind,
) -> SimulationReplayBundle {
    let sources = crate::PhysicalCounterExecutionSources::admit_for_plan(
        plan,
        &schedule,
        &trace,
        buffer_pool_evidence(plan),
        io_queue_evidence(plan),
    )
    .unwrap();
    let executed = PhysicalExecutedCounterEvidence::from_execution_sources(plan, sources).unwrap();
    let counter_receipt = admit_physical_counter_evidence(plan, executed).unwrap();
    let parts =
        ExecutedTranscriptParts::new(plan, schedule, fixture, trace.clone(), counter_receipt)
            .unwrap()
            .with_faults(physical_isolation_fault_events(expected_fault))
            .with_oracle_verdict(physical_isolation_verdict(plan, &trace))
            .with_transcript_replay_verdict()
            .unwrap();
    let transcript = PhysicalSimulationTranscript::from_executed_parts(parts).unwrap();
    crate::DetachedSimulationReplayParts::from_transcript(&transcript)
        .admit_replay_bundle()
        .unwrap()
}

pub fn physical_isolation_coverage_matrix(
    scenario: &CertifiedPhysicalScenario,
    plan: &PhysicalSimulationPlan,
    replay: &SimulationReplayBundle,
    mutation: &PhysicalIsolationMutationEvidence,
) -> GeneratedCoverageMatrix {
    PhysicalCoverageRegistry::for_sequence(HarnessCoverageStage::SimulationAdmission)
        .register_scenario(scenario)
        .unwrap()
        .register_plan(plan)
        .unwrap()
        .register_schedule(replay.schedule())
        .unwrap()
        .register_actor_set()
        .unwrap()
        .register_driver_contracts(plan.driver_contracts())
        .unwrap()
        .register_oracle_verdicts(replay.oracle_verdicts())
        .unwrap()
        .register_counter_receipt(replay.counter_receipt())
        .unwrap()
        .register_transcript(replay)
        .unwrap()
        .register_mutation_result(mutation.physical())
        .unwrap()
        .generate_matrix()
        .unwrap()
}

fn physical_isolation_verdict(
    plan: &PhysicalSimulationPlan,
    trace: &ObservedPhysicalTrace,
) -> PhysicalProofOracleVerdict {
    ReusablePhysicalOracleFamily::physical_isolation_interleaving()
        .oracle(PhysicalIsolationInterleavingOracle)
        .judge(plan, trace)
        .unwrap()
}

fn physical_isolation_fault_events(expected_fault: crate::PhysicalScenarioFaultKind) -> Vec<PhysicalFaultEvent> {
    crate::physical_isolation_stable_read_plan_fault_event(expected_fault)
        .unwrap()
        .into_iter()
        .collect()
}

fn buffer_pool_evidence(
    plan: &PhysicalSimulationPlan,
) -> forge_store_buffer_pool::BufferPoolExecutedEvidenceSource {
    let mut allocation = forge_store_buffer_pool::AllocationAdmission::from_declaration(
        plan.resource_envelope().allocation(),
    );
    let grant = allocation
        .admit(
            forge_store_buffer_pool::AllocationRequest::copied_payload(
                forge_store_buffer_pool::AllocationScope::Foreground,
                64,
            )
            .unwrap(),
        )
        .unwrap();
    allocation.record_allocation(grant).unwrap();
    forge_store_buffer_pool::BufferPoolExecutedEvidenceSource::from_allocation_execution(
        &allocation,
    )
    .unwrap()
}

fn io_queue_evidence(
    plan: &PhysicalSimulationPlan,
) -> forge_store_io_scheduler::IoQueueExecutedEvidenceSource {
    let mut recorder = forge_store_io_scheduler::IoQueueExecutionRecorder::from_envelope(
        plan.resource_envelope().io_queue(),
    );
    recorder.observe_queue_depth(1).unwrap();
    recorder.executed_evidence().unwrap()
}
