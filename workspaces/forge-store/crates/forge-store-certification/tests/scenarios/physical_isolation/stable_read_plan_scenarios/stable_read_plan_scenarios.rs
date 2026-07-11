#![allow(dead_code)]

#[path = "../../recovery/counter_strength/compaction_interlock_trace.rs"]
mod compaction_interlock_trace;

use forge_store_physical_certification::{
    admit_physical_counter_evidence, lower_physical_simulation_plan,
    physical_isolation_stable_read_plan_fault_event, physical_scenario, CounterContractKind,
    CounterContractOracle, ExecutedPhysicalSimulationObservation, ExecutedTranscriptParts,
    FixtureCapabilityDeclaration, FixtureMutationBoundary, LargeStoreFixtureProfile, ObserverKind,
    OracleFamilyKind, PhysicalCertificationEvidenceBundle, PhysicalDriverKind,
    PhysicalExecutedCounterEvidence, PhysicalFaultEvent, PhysicalFaultEventKind,
    PhysicalFixtureBuilder, PhysicalInterleavingSchedule, PhysicalScenarioActor,
    PhysicalScenarioExpectation, PhysicalScenarioFault, PhysicalScenarioFaultKind,
    PhysicalScenarioIntent, PhysicalScenarioSchedule, PhysicalSimulationCapabilitySet,
    PhysicalSimulationObserver, PhysicalSimulationPlan, PhysicalSimulationProfile,
    PhysicalSimulationProfileSet, PhysicalSimulationScenarioFamily, PhysicalSimulationTranscript,
    ReusablePhysicalOracleFamily, ShortcutRejectionObservation, SimulationEvidencePolicy,
    SimulationPlanningContext, StateSpaceBudget, SupportedObserverSet, SupportedOracleFamilySet,
};
use forge_store_test_support::{
    admitted_developer_smoke_driver_contracts, developer_smoke_replay_seed,
    production_backed_physical_fixture_materialization, NativeStoreAspectFixture,
};

#[test]
fn stable_read_plan_scenarios_execute_through_simulation_harness_evidence_pipeline() {
    for lane in stable_read_plan_lanes() {
        let plan = lower_physical_simulation_plan(lane.scenario, complete_context()).unwrap();
        let evidence = executed_evidence_bundle(&plan, lane.expected_fault);
        let primary = evidence.primary();

        assert!(plan
            .drivers()
            .contains(PhysicalDriverKind::ProductionBoundaryYieldpoint));
        assert!(plan
            .drivers()
            .contains(PhysicalDriverKind::MemoryPressureBoundary));
        assert!(plan
            .observers()
            .contains(ObserverKind::IndependentPhysicalTrace));
        assert!(plan
            .oracle_families()
            .contains(OracleFamilyKind::TranscriptReplayEvidence));
        assert!(plan
            .counter_contracts()
            .contains(CounterContractKind::ProtectedReferences));
        assert!(plan
            .counter_contracts()
            .contains(CounterContractKind::EpochRetries));
        assert!(plan
            .counter_contracts()
            .contains(CounterContractKind::AllocationBytes));
        assert_eq!(
            primary.counter_row_count(),
            plan.counter_contracts().iter().count()
        );
        assert_eq!(
            primary.oracle_verdict_count(),
            plan.oracle_families().iter().count()
        );
        assert_fault_events_match_lane(evidence.replay().fault_events(), lane.expected_fault);
    }
}

struct StableReadPlanLane {
    scenario: forge_store_physical_certification::CertifiedPhysicalScenario,
    expected_fault: PhysicalScenarioFaultKind,
}

fn stable_read_plan_lanes() -> Vec<StableReadPlanLane> {
    vec![
        lane(
            "counter-contracts",
            PhysicalScenarioIntent::StableReadPlanCounterContracts,
            PhysicalScenarioFault::no_fault(),
            PhysicalScenarioExpectation::stable_read_plan_counter_contracts(),
        ),
        lane(
            "transcript-replay",
            PhysicalScenarioIntent::StableReadPlanTranscriptReplay,
            PhysicalScenarioFault::no_fault(),
            PhysicalScenarioExpectation::stable_read_plan_transcript_replay(),
        ),
        lane(
            "stale-generation",
            PhysicalScenarioIntent::StableReadPlanStaleGenerationMutant,
            PhysicalScenarioFault::stale_generation(),
            PhysicalScenarioExpectation::stable_read_plan_denial(),
        ),
        lane(
            "missing-release",
            PhysicalScenarioIntent::StableReadPlanMissingReleaseMutant,
            PhysicalScenarioFault::missing_read_plan_release(),
            PhysicalScenarioExpectation::stable_read_plan_denial(),
        ),
        lane(
            "execution-time-discovery",
            PhysicalScenarioIntent::StableReadPlanExecutionTimeDiscoveryMutant,
            PhysicalScenarioFault::execution_time_reference_discovery(),
            PhysicalScenarioExpectation::stable_read_plan_denial(),
        ),
        lane(
            "unbounded-footprint",
            PhysicalScenarioIntent::StableReadPlanUnboundedFootprintMutant,
            PhysicalScenarioFault::unbounded_read_plan_footprint(),
            PhysicalScenarioExpectation::stable_read_plan_denial(),
        ),
    ]
}

fn lane(
    suffix: &str,
    intent: PhysicalScenarioIntent,
    fault: PhysicalScenarioFault,
    expectation: PhysicalScenarioExpectation,
) -> StableReadPlanLane {
    let fixture = NativeStoreAspectFixture::segment_header(suffix, 7);
    let expected_fault = fault.kind();
    let scenario = physical_scenario(format!("store.physical.s5.stable-read-plan.{suffix}"))
        .family(PhysicalSimulationScenarioFamily::PhysicalIsolationStableReadPlanAdmission)
        .intent(intent)
        .fixture(fixture.boundary_fact().clone())
        .actor(PhysicalScenarioActor::foreground_reader("reader"))
        .actor(PhysicalScenarioActor::maintenance_reclaimer("reclaimer"))
        .fault(fault)
        .schedule(PhysicalScenarioSchedule::named_boundary_yieldpoint(
            "root-publication-before-observe",
        ))
        .expectation(expectation)
        .certify_definition()
        .unwrap();
    StableReadPlanLane {
        scenario,
        expected_fault,
    }
}

fn complete_context() -> SimulationPlanningContext {
    SimulationPlanningContext::for_profile(PhysicalSimulationProfile::DeveloperSmoke)
        .with_supported_profiles(PhysicalSimulationProfileSet::all())
        .with_capabilities(PhysicalSimulationCapabilitySet::physical_isolation_readiness_shape_probe())
        .with_driver_contracts(admitted_developer_smoke_driver_contracts().unwrap())
        .with_supported_observers(SupportedObserverSet::all_for_developer_smoke())
        .with_supported_oracle_families(SupportedOracleFamilySet::all_for_developer_smoke())
        .with_evidence_policy(SimulationEvidencePolicy::minimal_replayable())
        .with_forbidden_shortcuts(
            forge_store_physical_certification::ForbiddenShortcutSet::physical_certification_baseline(),
        )
}

fn executed_evidence_bundle(
    plan: &PhysicalSimulationPlan,
    expected_fault: PhysicalScenarioFaultKind,
) -> PhysicalCertificationEvidenceBundle {
    let schedule = PhysicalInterleavingSchedule::from_lowered_plan(
        plan,
        developer_smoke_replay_seed(),
        StateSpaceBudget::bounded_steps(8).unwrap(),
    )
    .unwrap();
    let trace = executed_trace(plan);
    let sources =
        forge_store_physical_certification::PhysicalCounterExecutionSources::admit_for_plan(
            plan,
            &schedule,
            &trace,
            buffer_pool_evidence(plan),
            io_queue_evidence(plan),
        )
        .unwrap();
    let executed_counters =
        PhysicalExecutedCounterEvidence::from_execution_sources(plan, sources).unwrap();
    let counter_receipt = admit_physical_counter_evidence(plan, executed_counters).unwrap();
    let counter_verdict = ReusablePhysicalOracleFamily::physical_isolation_readiness_shape()
        .oracle(CounterContractOracle)
        .judge(plan, &trace)
        .unwrap();
    let parts = ExecutedTranscriptParts::new(
        plan,
        schedule,
        &production_fixture(),
        trace,
        counter_receipt,
    )
    .unwrap()
    .with_faults(physical_isolation_fault_events(expected_fault))
    .with_oracle_verdict(counter_verdict)
    .with_transcript_replay_verdict()
    .unwrap();
    let transcript = PhysicalSimulationTranscript::from_executed_parts(parts).unwrap();
    let replay =
        forge_store_physical_certification::DetachedSimulationReplayParts::from_transcript(
            &transcript,
        )
        .admit_replay_bundle()
        .unwrap();
    PhysicalCertificationEvidenceBundle::from_replay_bundle(replay).unwrap()
}

fn physical_isolation_fault_events(expected_fault: PhysicalScenarioFaultKind) -> Vec<PhysicalFaultEvent> {
    physical_isolation_stable_read_plan_fault_event(expected_fault)
        .unwrap()
        .into_iter()
        .collect()
}

fn assert_fault_events_match_lane(
    events: &[PhysicalFaultEvent],
    expected_fault: PhysicalScenarioFaultKind,
) {
    let expected = expected_fault_kind(expected_fault);
    match expected {
        Some(kind) => {
            assert_eq!(events.len(), 1);
            assert_eq!(events[0].kind(), kind);
        }
        None => assert!(events.is_empty()),
    }
}

fn expected_fault_kind(fault: PhysicalScenarioFaultKind) -> Option<PhysicalFaultEventKind> {
    physical_isolation_stable_read_plan_fault_event(fault)
        .unwrap()
        .map(|event| event.kind())
}

fn executed_trace(
    plan: &PhysicalSimulationPlan,
) -> forge_store_physical_certification::ObservedPhysicalTrace {
    let execution = ExecutedPhysicalSimulationObservation::from_executed_plan(plan).unwrap();
    PhysicalSimulationObserver::independent_physical_trace()
        .observe_executed_plan(plan, &execution)
        .unwrap()
        .with_compaction_interlock_observation(
            compaction_interlock_trace::store_compaction_observation(),
        )
        .with_shortcut_rejection_observation(ShortcutRejectionObservation::private_mutation_denied())
        .complete()
        .unwrap()
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

fn production_fixture() -> forge_store_physical_certification::ProductionBackedPhysicalFixture {
    PhysicalFixtureBuilder::production_backed("phase5-stable-read-plan-scenarios")
        .materialize_with(
            production_backed_physical_fixture_materialization(
                LargeStoreFixtureProfile::StoreLargerThanMemory,
                15,
            )
            .unwrap(),
        )
        .capability(FixtureCapabilityDeclaration::for_mutation_boundary(
            FixtureMutationBoundary::Manifest,
        ))
        .and_reopen_through_physical_authority()
        .unwrap()
}
