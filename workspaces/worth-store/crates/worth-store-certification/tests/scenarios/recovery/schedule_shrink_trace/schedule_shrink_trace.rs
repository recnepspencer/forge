use worth_store_physical_certification::{
    lower_physical_simulation_plan, physical_scenario, CounterMismatchSummary,
    ForbiddenShortcutSet, OracleVerdictKind, OracleVerdictSummary, PhysicalFaultLocus,
    PhysicalScenarioActor, PhysicalScenarioExpectation, PhysicalScenarioIntent,
    PhysicalScenarioSchedule, PhysicalSimulationCapabilitySet, PhysicalSimulationProfile,
    PhysicalSimulationProfileSet, PhysicalSimulationScenarioFamily, ScheduleFailureClass,
    ScheduleFailureSignature, ScheduleReplayDenial, ScheduleShrinkTrace, SimulationEvidencePolicy,
    SimulationPlanningContext, SupportedObserverSet, SupportedOracleFamilySet,
};
use worth_store_test_support::{
    admitted_developer_smoke_driver_contracts, deterministic_developer_smoke_schedule,
    NativeStoreAspectFixture,
};

#[test]
fn shrink_trace_preserves_failure_evidence() {
    let plan =
        lower_physical_simulation_plan(physical_isolation_scenario(), complete_context()).unwrap();
    let schedule = deterministic_developer_smoke_schedule(&plan).unwrap();
    let proving_step = schedule.actor_steps()[0].clone();
    let proving_actor_id = proving_step.actor_id().to_owned();
    let fault_locus = PhysicalFaultLocus::from_actor_step(&proving_step);
    let failure = ScheduleFailureSignature::new(
        ScheduleFailureClass::CounterMismatch,
        fault_locus,
        CounterMismatchSummary::new("actor-step-exact"),
        OracleVerdictSummary::violated("s5-readiness-shape"),
    );
    let observed_failure = failure.clone();

    let shrink = ScheduleShrinkTrace::shrink_reproducing_failure(
        failure,
        schedule.actor_steps().iter().cloned(),
        |steps| {
            steps
                .iter()
                .any(|step| {
                    step.actor_id() == proving_actor_id
                        && step.yieldpoint() == "root-publication-before-observe"
                })
                .then(|| observed_failure.clone())
        },
    )
    .unwrap();

    assert_eq!(
        shrink.failure_class(),
        ScheduleFailureClass::CounterMismatch
    );
    assert_eq!(
        shrink.fault_locus().yieldpoint(),
        "root-publication-before-observe"
    );
    assert_eq!(shrink.fault_locus().actor_id(), proving_actor_id);
    assert_eq!(
        shrink.counter_mismatch().counter_contract(),
        "actor-step-exact"
    );
    assert_eq!(
        shrink.oracle_verdict().oracle_family(),
        "s5-readiness-shape"
    );
    assert_eq!(
        shrink.oracle_verdict().verdict(),
        OracleVerdictKind::Violated
    );
    assert_eq!(shrink.minimized_steps().len(), 1);
    assert_eq!(shrink.minimized_steps()[0].actor_id(), proving_actor_id);
    assert_eq!(
        shrink.minimized_steps()[0].yieldpoint(),
        "root-publication-before-observe"
    );
}

#[test]
fn shrink_trace_denies_when_minimization_erases_fault_locus() {
    let plan =
        lower_physical_simulation_plan(physical_isolation_scenario(), complete_context()).unwrap();
    let schedule = deterministic_developer_smoke_schedule(&plan).unwrap();
    let proving_step = schedule.actor_steps()[0].clone();
    let wrong_actor_same_yieldpoint = schedule
        .actor_steps()
        .iter()
        .find(|step| step.actor_id() != proving_step.actor_id())
        .cloned()
        .unwrap();
    let proving_actor_id = proving_step.actor_id().to_owned();
    let fault_locus = PhysicalFaultLocus::from_actor_step(&proving_step);
    let failure = ScheduleFailureSignature::new(
        ScheduleFailureClass::CounterMismatch,
        fault_locus,
        CounterMismatchSummary::new("actor-step-exact"),
        OracleVerdictSummary::violated("s5-readiness-shape"),
    );

    let denial = ScheduleShrinkTrace::shrink_reproducing_failure(
        failure.clone(),
        std::iter::once(wrong_actor_same_yieldpoint),
        |_| Some(failure.clone()),
    )
    .unwrap_err();

    assert_eq!(
        denial,
        ScheduleReplayDenial::ShrinkErasedFaultLocus {
            actor_id: proving_actor_id,
            yieldpoint: "root-publication-before-observe".to_owned(),
        }
    );
}

#[test]
fn shrink_trace_refuses_a_candidate_that_reproduces_a_different_failure() {
    let plan =
        lower_physical_simulation_plan(physical_isolation_scenario(), complete_context()).unwrap();
    let schedule = deterministic_developer_smoke_schedule(&plan).unwrap();
    let fault_locus = PhysicalFaultLocus::from_actor_step(&schedule.actor_steps()[0]);
    let original = ScheduleFailureSignature::new(
        ScheduleFailureClass::CounterMismatch,
        fault_locus.clone(),
        CounterMismatchSummary::new("actor-step-exact"),
        OracleVerdictSummary::violated("s5-readiness-shape"),
    );
    let different = ScheduleFailureSignature::new(
        ScheduleFailureClass::OracleViolation,
        fault_locus,
        CounterMismatchSummary::new("different-counter"),
        OracleVerdictSummary::violated("different-oracle"),
    );
    let full_width = schedule.actor_steps().len();
    let original_observation = original.clone();
    let shrink = ScheduleShrinkTrace::shrink_reproducing_failure(
        original,
        schedule.actor_steps().iter().cloned(),
        |steps| {
            Some(if steps.len() == full_width {
                original_observation.clone()
            } else {
                different.clone()
            })
        },
    )
    .unwrap();

    assert_eq!(shrink.minimized_steps().len(), full_width);
}

fn complete_context() -> SimulationPlanningContext {
    SimulationPlanningContext::for_profile(PhysicalSimulationProfile::DeveloperSmoke)
        .with_supported_profiles(PhysicalSimulationProfileSet::all())
        .with_capabilities(
            PhysicalSimulationCapabilitySet::physical_isolation_readiness_shape_probe(),
        )
        .with_driver_contracts(admitted_developer_smoke_driver_contracts().unwrap())
        .with_supported_observers(SupportedObserverSet::all_for_developer_smoke())
        .with_supported_oracle_families(SupportedOracleFamilySet::all_for_developer_smoke())
        .with_evidence_policy(SimulationEvidencePolicy::minimal_replayable())
        .with_forbidden_shortcuts(ForbiddenShortcutSet::physical_certification_baseline())
}

fn physical_isolation_scenario() -> worth_store_physical_certification::CertifiedPhysicalScenario {
    physical_scenario("store.physical.s5.schedule.shrink")
        .family(PhysicalSimulationScenarioFamily::PhysicalIsolationReadinessShapeProbe)
        .intent(PhysicalScenarioIntent::ProtectBeforeObserveShape)
        .fixture(
            NativeStoreAspectFixture::segment_header("schedule-shrink", 5)
                .boundary_fact()
                .clone(),
        )
        .actor(PhysicalScenarioActor::maintenance_reclaimer("reclaimer"))
        .actor(PhysicalScenarioActor::foreground_reader("reader"))
        .schedule(PhysicalScenarioSchedule::named_boundary_yieldpoint(
            "root-publication-before-observe",
        ))
        .expectation(PhysicalScenarioExpectation::non_claiming_physical_isolation_readiness_shape())
        .certify_definition()
        .unwrap()
}
