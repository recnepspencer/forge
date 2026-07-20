use super::compaction_observation as compaction_interlock_trace;

use crate::{admitted_developer_smoke_driver_contracts, NativeStoreAspectFixture};
use worth_store_buffer_pool::{
    AllocationAdmission, AllocationRequest, AllocationScope, BufferPoolExecutedEvidenceSource,
};
use worth_store_io_scheduler::IoQueueExecutionRecorder;
use worth_store_physical_certification::{
    admit_physical_counter_evidence, lower_physical_simulation_plan, physical_scenario,
    CompactionInterlockObservation, CounterContractKind, CounterExpectationKind,
    CounterStrengthJustification, CounterStrengthPosture, ForbiddenShortcutSet,
    HostileCounterEvidenceRow, HostileResourceEnvelopeObservation, PhysicalCounterEvidenceReceipt,
    PhysicalCounterExecutionSources, PhysicalExecutedCounterEvidence, PhysicalInterleavingSchedule,
    PhysicalScenarioActor, PhysicalScenarioExpectation, PhysicalScenarioIntent,
    PhysicalScenarioSchedule, PhysicalSimulationBoundaryObservation,
    PhysicalSimulationCapabilitySet, PhysicalSimulationObserver, PhysicalSimulationPlan,
    PhysicalSimulationProfile, PhysicalSimulationProfileSet, PhysicalSimulationScenarioFamily,
    ReplaySeed, ShortcutRejectionObservation, SimulationEvidencePolicy, SimulationPlanningContext,
    StateSpaceBudget, SupportedObserverSet, SupportedOracleFamilySet,
};

pub fn assert_counter(
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

pub fn hostile_satisfied_rows(plan: &PhysicalSimulationPlan) -> Vec<HostileCounterEvidenceRow> {
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

pub fn replace_row(rows: &mut [HostileCounterEvidenceRow], replacement: HostileCounterEvidenceRow) {
    let Some(row) = rows.iter_mut().find(|row| row.kind() == replacement.kind()) else {
        panic!("replacement row must target an existing row");
    };
    *row = replacement;
}

pub fn hostile_resource_observation_within_envelope(
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

pub fn executed_counter_evidence(
    plan: &PhysicalSimulationPlan,
    trace: worth_store_physical_certification::ObservedPhysicalTrace,
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

pub fn counter_receipt(
    plan: &PhysicalSimulationPlan,
    trace: worth_store_physical_certification::ObservedPhysicalTrace,
) -> PhysicalCounterEvidenceReceipt {
    admit_physical_counter_evidence(plan, executed_counter_evidence(plan, trace)).unwrap()
}

pub fn execution_sources_for_plan(
    plan: &PhysicalSimulationPlan,
    trace: worth_store_physical_certification::ObservedPhysicalTrace,
) -> Result<
    PhysicalCounterExecutionSources,
    worth_store_physical_certification::CounterMismatchEvidence,
> {
    let schedule = PhysicalInterleavingSchedule::from_lowered_plan(
        plan,
        ReplaySeed::required(Some(8)).unwrap(),
        StateSpaceBudget::bounded_steps(8).unwrap(),
    )
    .unwrap();
    execution_sources_with_schedule(plan, &schedule, trace)
}

pub fn execution_sources_with_schedule(
    plan: &PhysicalSimulationPlan,
    schedule: &PhysicalInterleavingSchedule,
    trace: worth_store_physical_certification::ObservedPhysicalTrace,
) -> Result<
    PhysicalCounterExecutionSources,
    worth_store_physical_certification::CounterMismatchEvidence,
> {
    PhysicalCounterExecutionSources::admit_for_plan(
        plan,
        schedule,
        &trace,
        buffer_pool_evidence(plan),
        io_queue_evidence(plan),
    )
}

pub fn observed_trace(
    plan: &PhysicalSimulationPlan,
) -> worth_store_physical_certification::ObservedPhysicalTrace {
    let execution =
        PhysicalSimulationBoundaryObservation::from_declared_driver_shape_probe(plan).unwrap();
    PhysicalSimulationObserver::independent_physical_trace()
        .observe_boundary_observation(plan, &execution)
        .unwrap()
        .with_compaction_interlock_observation(compaction_observation())
        .complete()
        .unwrap()
}

pub fn publication_only_trace(
    plan: &PhysicalSimulationPlan,
) -> worth_store_physical_certification::ObservedPhysicalTrace {
    let execution =
        PhysicalSimulationBoundaryObservation::from_declared_driver_shape_probe(plan).unwrap();
    PhysicalSimulationObserver::independent_physical_trace()
        .observe_boundary_observation(plan, &execution)
        .unwrap()
        .with_compaction_interlock_observation(
            compaction_interlock_trace::publication_only_compaction_observation(),
        )
        .complete()
        .unwrap()
}

pub fn shortcut_trace(
    plan: &PhysicalSimulationPlan,
) -> worth_store_physical_certification::ObservedPhysicalTrace {
    let execution =
        PhysicalSimulationBoundaryObservation::from_declared_driver_shape_probe(plan).unwrap();
    PhysicalSimulationObserver::shortcut_rejection()
        .observe_boundary_observation(plan, &execution)
        .unwrap()
        .with_compaction_interlock_observation(compaction_observation())
        .with_shortcut_rejection_observation(ShortcutRejectionObservation::private_mutation_denied())
        .complete()
        .unwrap()
}

pub fn json_shortcut_trace(
    plan: &PhysicalSimulationPlan,
) -> worth_store_physical_certification::ObservedPhysicalTrace {
    let execution =
        PhysicalSimulationBoundaryObservation::from_declared_driver_shape_probe(plan).unwrap();
    PhysicalSimulationObserver::shortcut_rejection()
        .observe_boundary_observation(plan, &execution)
        .unwrap()
        .with_compaction_interlock_observation(compaction_observation())
        .with_shortcut_rejection_observation(ShortcutRejectionObservation::json_authority_denied())
        .complete()
        .unwrap()
}

pub fn compaction_observation() -> CompactionInterlockObservation {
    compaction_interlock_trace::store_compaction_observation()
}

pub fn lower_physical_isolation_plan() -> PhysicalSimulationPlan {
    lower_physical_isolation_plan_for_profile(PhysicalSimulationProfile::DeveloperSmoke)
}

pub fn lower_physical_isolation_plan_for_profile(
    profile: PhysicalSimulationProfile,
) -> PhysicalSimulationPlan {
    lower_physical_simulation_plan(
        physical_isolation_scenario(),
        complete_context_for_profile(profile),
    )
    .unwrap()
}

pub fn lower_shortcut_plan() -> PhysicalSimulationPlan {
    lower_physical_simulation_plan(shortcut_scenario(), complete_context()).unwrap()
}

pub fn lower_physical_isolation_shortcut_plan() -> PhysicalSimulationPlan {
    lower_physical_simulation_plan(physical_isolation_shortcut_scenario(), complete_context())
        .unwrap()
}

pub fn lower_physical_isolation_shortcut_plan_for_profile(
    profile: PhysicalSimulationProfile,
) -> PhysicalSimulationPlan {
    lower_physical_simulation_plan(
        physical_isolation_shortcut_scenario(),
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
) -> worth_store_io_scheduler::IoQueueExecutedEvidenceSource {
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
        .with_capabilities(
            PhysicalSimulationCapabilitySet::physical_isolation_readiness_shape_probe(),
        )
        .with_driver_contracts(admitted_developer_smoke_driver_contracts().unwrap())
        .with_supported_observers(SupportedObserverSet::all_for_developer_smoke())
        .with_supported_oracle_families(SupportedOracleFamilySet::all_for_developer_smoke())
        .with_evidence_policy(SimulationEvidencePolicy::minimal_replayable())
        .with_forbidden_shortcuts(ForbiddenShortcutSet::physical_certification_baseline())
}

pub fn physical_isolation_scenario() -> worth_store_physical_certification::CertifiedPhysicalScenario
{
    named_physical_isolation_scenario("store.physical.s45.phase8.counter-strength")
}

fn named_physical_isolation_scenario(
    name: &str,
) -> worth_store_physical_certification::CertifiedPhysicalScenario {
    physical_scenario(name)
        .family(PhysicalSimulationScenarioFamily::PhysicalIsolationReadinessShapeProbe)
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
        .expectation(PhysicalScenarioExpectation::non_claiming_physical_isolation_readiness_shape())
        .certify_definition()
        .unwrap()
}

pub fn shortcut_scenario() -> worth_store_physical_certification::CertifiedPhysicalScenario {
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

pub fn physical_isolation_shortcut_scenario(
) -> worth_store_physical_certification::CertifiedPhysicalScenario {
    physical_scenario("store.physical.s45.phase8.executed-shortcut-counter")
        .family(PhysicalSimulationScenarioFamily::PhysicalIsolationReadinessShapeProbe)
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
            PhysicalScenarioExpectation::non_claiming_physical_isolation_readiness_with_shortcut_rejection(),
        )
        .certify_definition()
        .unwrap()
}
