#![allow(dead_code)]

use forge_store_buffer_pool::{
    AllocationAdmission, AllocationRequest, AllocationScope, BufferPoolExecutedEvidenceSource,
};
use forge_store_io_scheduler::IoQueueExecutionRecorder;
use forge_store_physical_certification::{
    admit_physical_counter_evidence, lower_physical_simulation_plan, physical_scenario,
    CounterContractKind, CounterExpectationKind, CounterStrengthJustification,
    CounterStrengthPosture, ExecutedPhysicalSimulationObservation, ForbiddenShortcutSet,
    HostileCounterEvidenceRow, HostileResourceEnvelopeObservation, PhysicalCounterEvidenceReceipt,
    PhysicalCounterExecutionSources, PhysicalExecutedCounterEvidence, PhysicalInterleavingSchedule,
    PhysicalScenarioActor, PhysicalScenarioExpectation, PhysicalScenarioIntent,
    PhysicalScenarioSchedule, PhysicalSimulationCapabilitySet, PhysicalSimulationObserver,
    PhysicalSimulationPlan, PhysicalSimulationProfile, PhysicalSimulationProfileSet,
    PhysicalSimulationScenarioFamily, ReplaySeed, ShortcutRejectionObservation,
    SimulationEvidencePolicy, SimulationPlanningContext, StateSpaceBudget, SupportedObserverSet,
    SupportedOracleFamilySet,
};
use forge_store_test_support::{
    admitted_developer_smoke_driver_contracts, NativeStoreAspectFixture,
};

pub(crate) fn assert_counter(
    plan: &PhysicalSimulationPlan,
    kind: CounterContractKind,
    expectation: CounterExpectationKind,
    expected_value: Option<u64>,
    posture: CounterStrengthPosture,
    justification: CounterStrengthJustification,
) {
    let contract = plan
        .counter_contracts()
        .require(kind)
        .expect("lowered plan must require counter contract");

    assert_eq!(contract.expectation().kind(), expectation);
    assert_eq!(contract.expectation().value(), expected_value);
    assert_eq!(contract.posture(), posture);
    assert_eq!(contract.justification(), justification);
}

pub(crate) fn hostile_satisfied_rows(
    plan: &PhysicalSimulationPlan,
) -> Vec<HostileCounterEvidenceRow> {
    plan.counter_contracts()
        .iter()
        .map(|contract| {
            let observed_count = match contract.expectation().kind() {
                CounterExpectationKind::Zero => 0,
                CounterExpectationKind::Positive => 1,
                CounterExpectationKind::Exact => contract.expectation().value().unwrap(),
                CounterExpectationKind::Monotonic => 0,
                CounterExpectationKind::Bounded => 1,
                CounterExpectationKind::ProfileScoped => 1,
            };
            HostileCounterEvidenceRow::new(
                contract.kind(),
                contract.expectation().kind(),
                observed_count,
            )
        })
        .collect()
}

pub(crate) fn replace_row(
    rows: &mut Vec<HostileCounterEvidenceRow>,
    replacement: HostileCounterEvidenceRow,
) {
    let Some(row) = rows.iter_mut().find(|row| row.kind() == replacement.kind()) else {
        panic!("replacement row must target an existing row");
    };
    *row = replacement;
}

pub(crate) fn hostile_resource_observation_within_envelope(
    plan: &PhysicalSimulationPlan,
) -> HostileResourceEnvelopeObservation {
    let envelope = plan.resource_envelope();
    HostileResourceEnvelopeObservation::new(
        plan.profile(),
        envelope
            .allocation()
            .budget(AllocationScope::Foreground)
            .as_bytes(),
        envelope.resident_bytes().as_bytes(),
        u64::from(envelope.max_pinned_pages()),
        u64::from(envelope.max_dirty_pages()),
        u64::from(envelope.io_queue().max_queue_depth()),
        u64::from(envelope.io_queue().max_interference_events()),
    )
}

pub(crate) fn executed_counter_evidence(
    plan: &PhysicalSimulationPlan,
    trace: forge_store_physical_certification::ObservedPhysicalTrace,
) -> PhysicalExecutedCounterEvidence {
    let schedule = PhysicalInterleavingSchedule::from_lowered_plan(
        plan,
        ReplaySeed::required(Some(8)).unwrap(),
        StateSpaceBudget::bounded_steps(8).unwrap(),
    )
    .unwrap();
    let sources = PhysicalCounterExecutionSources::admit_for_plan(
        plan,
        &schedule,
        &trace,
        buffer_pool_evidence(plan),
        io_queue_evidence(plan),
    )
    .unwrap();
    PhysicalExecutedCounterEvidence::from_execution_sources(plan, sources).unwrap()
}

pub(crate) fn counter_receipt(
    plan: &PhysicalSimulationPlan,
    trace: forge_store_physical_certification::ObservedPhysicalTrace,
) -> PhysicalCounterEvidenceReceipt {
    admit_physical_counter_evidence(plan, executed_counter_evidence(plan, trace)).unwrap()
}

pub(crate) fn execution_sources_for_plan(
    plan: &PhysicalSimulationPlan,
    trace: forge_store_physical_certification::ObservedPhysicalTrace,
) -> Result<
    PhysicalCounterExecutionSources,
    forge_store_physical_certification::CounterMismatchEvidence,
> {
    let schedule = PhysicalInterleavingSchedule::from_lowered_plan(
        plan,
        ReplaySeed::required(Some(8)).unwrap(),
        StateSpaceBudget::bounded_steps(8).unwrap(),
    )
    .unwrap();
    execution_sources_with_schedule(plan, &schedule, trace)
}

pub(crate) fn execution_sources_with_schedule(
    plan: &PhysicalSimulationPlan,
    schedule: &PhysicalInterleavingSchedule,
    trace: forge_store_physical_certification::ObservedPhysicalTrace,
) -> Result<
    PhysicalCounterExecutionSources,
    forge_store_physical_certification::CounterMismatchEvidence,
> {
    PhysicalCounterExecutionSources::admit_for_plan(
        plan,
        schedule,
        &trace,
        buffer_pool_evidence(plan),
        io_queue_evidence(plan),
    )
}

pub(crate) fn observed_trace(
    plan: &PhysicalSimulationPlan,
) -> forge_store_physical_certification::ObservedPhysicalTrace {
    let execution = ExecutedPhysicalSimulationObservation::from_executed_plan(plan).unwrap();
    PhysicalSimulationObserver::independent_physical_trace()
        .observe_executed_plan(plan, &execution)
        .unwrap()
        .complete()
        .unwrap()
}

pub(crate) fn shortcut_trace(
    plan: &PhysicalSimulationPlan,
) -> forge_store_physical_certification::ObservedPhysicalTrace {
    let execution = ExecutedPhysicalSimulationObservation::from_executed_plan(plan).unwrap();
    PhysicalSimulationObserver::shortcut_rejection()
        .observe_executed_plan(plan, &execution)
        .unwrap()
        .with_shortcut_rejection_observation(ShortcutRejectionObservation::private_mutation_denied())
        .complete()
        .unwrap()
}

pub(crate) fn json_shortcut_trace(
    plan: &PhysicalSimulationPlan,
) -> forge_store_physical_certification::ObservedPhysicalTrace {
    let execution = ExecutedPhysicalSimulationObservation::from_executed_plan(plan).unwrap();
    PhysicalSimulationObserver::shortcut_rejection()
        .observe_executed_plan(plan, &execution)
        .unwrap()
        .with_shortcut_rejection_observation(ShortcutRejectionObservation::json_authority_denied())
        .complete()
        .unwrap()
}

pub(crate) fn lower_s5_plan() -> PhysicalSimulationPlan {
    lower_s5_plan_for_profile(PhysicalSimulationProfile::DeveloperSmoke)
}

pub(crate) fn lower_s5_plan_for_profile(
    profile: PhysicalSimulationProfile,
) -> PhysicalSimulationPlan {
    lower_physical_simulation_plan(s5_scenario(), complete_context_for_profile(profile)).unwrap()
}

pub(crate) fn lower_shortcut_plan() -> PhysicalSimulationPlan {
    lower_physical_simulation_plan(shortcut_scenario(), complete_context()).unwrap()
}

pub(crate) fn lower_s5_shortcut_plan() -> PhysicalSimulationPlan {
    lower_physical_simulation_plan(s5_shortcut_scenario(), complete_context()).unwrap()
}

pub(crate) fn lower_s5_shortcut_plan_for_profile(
    profile: PhysicalSimulationProfile,
) -> PhysicalSimulationPlan {
    lower_physical_simulation_plan(
        s5_shortcut_scenario(),
        complete_context_for_profile(profile),
    )
    .unwrap()
}

fn buffer_pool_evidence(plan: &PhysicalSimulationPlan) -> BufferPoolExecutedEvidenceSource {
    let mut allocation =
        AllocationAdmission::from_declaration(plan.resource_envelope().allocation());
    let grant = allocation
        .admit(AllocationRequest::copied_payload(AllocationScope::Foreground, 64).unwrap())
        .unwrap();
    allocation.record_allocation(grant).unwrap();
    BufferPoolExecutedEvidenceSource::from_allocation_execution(&allocation).unwrap()
}

fn io_queue_evidence(
    plan: &PhysicalSimulationPlan,
) -> forge_store_io_scheduler::IoQueueExecutedEvidenceSource {
    let mut recorder = IoQueueExecutionRecorder::from_envelope(plan.resource_envelope().io_queue());
    recorder.observe_queue_depth(1).unwrap();
    recorder.executed_evidence().unwrap()
}

fn complete_context() -> SimulationPlanningContext {
    complete_context_for_profile(PhysicalSimulationProfile::DeveloperSmoke)
}

fn complete_context_for_profile(profile: PhysicalSimulationProfile) -> SimulationPlanningContext {
    SimulationPlanningContext::for_profile(profile)
        .with_supported_profiles(PhysicalSimulationProfileSet::all())
        .with_capabilities(PhysicalSimulationCapabilitySet::s5_readiness_shape_probe())
        .with_driver_contracts(admitted_developer_smoke_driver_contracts().unwrap())
        .with_supported_observers(SupportedObserverSet::all_for_developer_smoke())
        .with_supported_oracle_families(SupportedOracleFamilySet::all_for_developer_smoke())
        .with_evidence_policy(SimulationEvidencePolicy::minimal_replayable())
        .with_forbidden_shortcuts(ForbiddenShortcutSet::roadmap2_baseline())
}

pub(crate) fn s5_scenario() -> forge_store_physical_certification::CertifiedPhysicalScenario {
    named_s5_scenario("store.physical.s45.phase8.counter-strength")
}

fn named_s5_scenario(name: &str) -> forge_store_physical_certification::CertifiedPhysicalScenario {
    physical_scenario(name)
        .family(PhysicalSimulationScenarioFamily::S5ReadinessShapeProbe)
        .intent(PhysicalScenarioIntent::ProtectBeforeObserveShape)
        .fixture(
            NativeStoreAspectFixture::segment_header("phase8-strength", 8)
                .boundary_fact()
                .clone(),
        )
        .actor(PhysicalScenarioActor::maintenance_reclaimer("reclaimer"))
        .actor(PhysicalScenarioActor::foreground_reader("reader"))
        .schedule(PhysicalScenarioSchedule::named_boundary_yieldpoint(
            "root-publication-before-observe",
        ))
        .expectation(PhysicalScenarioExpectation::non_claiming_s5_readiness_shape())
        .certify_definition()
        .unwrap()
}

pub(crate) fn shortcut_scenario() -> forge_store_physical_certification::CertifiedPhysicalScenario {
    physical_scenario("store.physical.s45.phase8.shortcut-counter-strength")
        .family(PhysicalSimulationScenarioFamily::ShortcutRejectionDogfood)
        .intent(PhysicalScenarioIntent::ForbiddenShortcutRejectionShape)
        .fixture(
            NativeStoreAspectFixture::segment_header("phase8-shortcut", 8)
                .boundary_fact()
                .clone(),
        )
        .actor(PhysicalScenarioActor::shortcut_rejection_probe("probe"))
        .schedule(PhysicalScenarioSchedule::named_boundary_yieldpoint(
            "shortcut-rejection-boundary",
        ))
        .expectation(PhysicalScenarioExpectation::shortcut_rejection_dogfood())
        .certify_definition()
        .unwrap()
}

pub(crate) fn s5_shortcut_scenario() -> forge_store_physical_certification::CertifiedPhysicalScenario
{
    physical_scenario("store.physical.s45.phase8.executed-shortcut-counter")
        .family(PhysicalSimulationScenarioFamily::S5ReadinessShapeProbe)
        .intent(PhysicalScenarioIntent::ProtectBeforeObserveShape)
        .fixture(
            NativeStoreAspectFixture::segment_header("phase8-executed-shortcut", 8)
                .boundary_fact()
                .clone(),
        )
        .actor(PhysicalScenarioActor::maintenance_reclaimer("reclaimer"))
        .actor(PhysicalScenarioActor::foreground_reader("reader"))
        .actor(PhysicalScenarioActor::shortcut_rejection_probe("probe"))
        .schedule(PhysicalScenarioSchedule::named_boundary_yieldpoint(
            "root-publication-before-observe",
        ))
        .expectation(
            PhysicalScenarioExpectation::non_claiming_s5_readiness_with_shortcut_rejection(),
        )
        .certify_definition()
        .unwrap()
}
