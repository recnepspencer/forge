#![cfg_attr(feature = "certification-test-support", allow(dead_code))]

mod boundary_fact;
mod sample;

use forge_foundational::{BoundaryArtifactField, BoundaryArtifactId, BoundaryArtifactLocator};
use forge_store_buffer_pool::{
    AllocationAdmission, AllocationRequest, AllocationScope, BufferPoolExecutedEvidenceSource,
};
use forge_store_io_scheduler::IoQueueExecutionRecorder;

use crate::s6_io_pressure_execution::materialize_s6_pressure_observation;
use crate::{
    admit_physical_counter_evidence, lower_physical_simulation_plan, physical_scenario,
    AdmittedDriverContractSet, ExecutedTranscriptParts, ExpectedFaultLocalization,
    FixtureCapabilityDeclaration, FixtureMutationBoundary, ForbiddenShortcutSet,
    LargeStoreFixtureProfile, PhysicalArtifactFaultLocus, PhysicalCounterEvidenceReceipt,
    PhysicalCounterExecutionSources, PhysicalExecutedCounterEvidence, PhysicalFaultEvent,
    PhysicalInterleavingSchedule, PhysicalScenarioActor, PhysicalScenarioExpectation,
    PhysicalScenarioFault, PhysicalScenarioIntent, PhysicalScenarioSchedule,
    PhysicalSimulationCapabilitySet, PhysicalSimulationDriver, PhysicalSimulationObserver,
    PhysicalSimulationPlan, PhysicalSimulationProfile, PhysicalSimulationProfileSet,
    PhysicalSimulationScenarioFamily, ProductionBackedFixtureMaterialization,
    ProductionBoundaryDriverTrace, ReplaySeed, ReusablePhysicalOracleFamily, S6IoPressureFaultKind,
    S6IoPressureHarnessScenario, ShortcutRejectionObservation, SimulationEvidencePolicy,
    SimulationPlanningContext, SimulationReplayBundle, StateSpaceBudget, SupportedObserverSet,
    SupportedOracleFamilySet, SupportedPhysicalDriverSet,
};

use self::boundary_fact::boundary_fact;
use self::sample::sample_for_profile;
pub(crate) use self::sample::S6PressureExecutionSample;

pub fn replay_bundle_for(
    scenario: S6IoPressureHarnessScenario,
    profile: PhysicalSimulationProfile,
) -> SimulationReplayBundle {
    replay_bundle_with_sample(scenario, profile, sample_for_profile(profile))
}

pub(crate) fn replay_bundle_with_delivered_fault(
    scenario: S6IoPressureHarnessScenario,
    profile: PhysicalSimulationProfile,
    delivered_fault_kind: S6IoPressureFaultKind,
) -> SimulationReplayBundle {
    replay_bundle_with_fault_event(
        scenario,
        profile,
        io_pressure_fault_event(delivered_fault_kind),
    )
}

pub(crate) fn replay_bundle_with_fault_event(
    scenario: S6IoPressureHarnessScenario,
    profile: PhysicalSimulationProfile,
    fault_event: PhysicalFaultEvent,
) -> SimulationReplayBundle {
    replay_bundle_from_parts(
        scenario,
        profile,
        sample_for_profile(profile),
        fault_event,
        None,
    )
}

pub(crate) fn replay_bundle_with_sample(
    scenario: S6IoPressureHarnessScenario,
    profile: PhysicalSimulationProfile,
    sample: S6PressureExecutionSample,
) -> SimulationReplayBundle {
    let fault_event = io_pressure_fault_event(scenario.fault_kind());
    replay_bundle_from_parts(scenario, profile, sample, fault_event, None)
}

pub(crate) fn replay_bundle_with_shortcut_observation(
    scenario: S6IoPressureHarnessScenario,
    profile: PhysicalSimulationProfile,
    shortcut: ShortcutRejectionObservation,
) -> SimulationReplayBundle {
    let fault_event = io_pressure_fault_event(scenario.fault_kind());
    replay_bundle_from_parts(
        scenario,
        profile,
        sample_for_profile(profile),
        fault_event,
        Some(shortcut),
    )
}

fn replay_bundle_from_parts(
    scenario: S6IoPressureHarnessScenario,
    profile: PhysicalSimulationProfile,
    sample: S6PressureExecutionSample,
    fault_event: PhysicalFaultEvent,
    shortcut: Option<ShortcutRejectionObservation>,
) -> SimulationReplayBundle {
    let plan = lower_s6_pressure_plan(scenario.clone(), profile);
    let schedule = schedule_for_plan(&plan);
    let base_trace = pressure_trace(&plan, None, shortcut);
    let counter_receipt = counter_receipt(&plan, &schedule, &base_trace, sample);
    let observation =
        materialize_s6_pressure_observation(&plan, &fault_event, &counter_receipt, &scenario)
            .unwrap_or_else(|_| {
                let canonical_fault = io_pressure_fault_event(scenario.fault_kind());
                materialize_s6_pressure_observation(
                    &plan,
                    &canonical_fault,
                    &counter_receipt,
                    &scenario,
                )
                .unwrap()
            });
    let trace = pressure_trace(&plan, Some(observation), shortcut);
    let mut parts =
        ExecutedTranscriptParts::new(&plan, schedule, &pressure_fixture(), trace, counter_receipt)
            .unwrap()
            .with_faults([fault_event]);
    let verdict = ReusablePhysicalOracleFamily::s6_io_pressure_simulation()
        .oracle(crate::S6IoPressureSimulationOracle)
        .judge(parts.plan(), parts.trace())
        .unwrap();
    parts = parts.with_oracle_verdict(verdict);
    parts = parts.with_transcript_replay_verdict().unwrap();
    let transcript = crate::PhysicalSimulationTranscript::from_executed_parts(parts).unwrap();
    crate::DetachedSimulationReplayParts::from_transcript(&transcript)
        .admit_replay_bundle()
        .unwrap()
}

pub(crate) fn s6_oracle_denial_without_pressure_observation(
    scenario: S6IoPressureHarnessScenario,
) -> crate::OracleDenial {
    let plan = lower_s6_pressure_plan(scenario, PhysicalSimulationProfile::DeveloperSmoke);
    let trace = pressure_trace(&plan, None, None);
    ReusablePhysicalOracleFamily::s6_io_pressure_simulation()
        .oracle(crate::S6IoPressureSimulationOracle)
        .judge(&plan, &trace)
        .unwrap_err()
}

pub(crate) fn lower_s6_pressure_plan(
    scenario: S6IoPressureHarnessScenario,
    profile: PhysicalSimulationProfile,
) -> PhysicalSimulationPlan {
    lower_physical_simulation_plan(
        certified_s6_pressure_scenario(scenario),
        planning_context(profile),
    )
    .unwrap()
}

pub(crate) fn schedule_for_plan(plan: &PhysicalSimulationPlan) -> PhysicalInterleavingSchedule {
    PhysicalInterleavingSchedule::from_lowered_plan(
        plan,
        ReplaySeed::from_u64(9),
        StateSpaceBudget::bounded_steps(8).unwrap(),
    )
    .unwrap()
}

pub(crate) fn counter_receipt(
    plan: &PhysicalSimulationPlan,
    schedule: &PhysicalInterleavingSchedule,
    trace: &crate::ObservedPhysicalTrace,
    sample: S6PressureExecutionSample,
) -> PhysicalCounterEvidenceReceipt {
    let sources = PhysicalCounterExecutionSources::admit_for_plan(
        plan,
        schedule,
        trace,
        buffer_pool_evidence(plan, sample),
        io_queue_evidence(plan, sample),
    )
    .unwrap();
    let evidence = PhysicalExecutedCounterEvidence::from_execution_sources(plan, sources).unwrap();
    admit_physical_counter_evidence(plan, evidence).unwrap()
}

pub(crate) fn scenario_fault(fault_kind: S6IoPressureFaultKind) -> PhysicalScenarioFault {
    match fault_kind {
        S6IoPressureFaultKind::BackendLatencyInjection => {
            PhysicalScenarioFault::s6_backend_latency_injection()
        }
        S6IoPressureFaultKind::QueueDepthSaturation => {
            PhysicalScenarioFault::s6_queue_depth_saturation()
        }
        S6IoPressureFaultKind::BandwidthThrottle => PhysicalScenarioFault::s6_bandwidth_throttle(),
        S6IoPressureFaultKind::DelayedSync => PhysicalScenarioFault::s6_delayed_sync(),
        S6IoPressureFaultKind::PageCachePressure => PhysicalScenarioFault::s6_page_cache_pressure(),
        S6IoPressureFaultKind::BackgroundPacingLateYield => {
            PhysicalScenarioFault::s6_background_pacing_late_yield()
        }
    }
}

pub(crate) fn fault_phase_for(
    fault_kind: S6IoPressureFaultKind,
) -> crate::PhysicalScenarioFaultKind {
    scenario_fault(fault_kind).kind()
}

fn certified_s6_pressure_scenario(
    scenario: S6IoPressureHarnessScenario,
) -> crate::CertifiedPhysicalScenario {
    physical_scenario("store.physical.s6.io-pressure")
        .family(PhysicalSimulationScenarioFamily::S6IoPressureHarness)
        .intent(PhysicalScenarioIntent::S6IoPressureSimulation)
        .fixture(boundary_fact("store.s6.io-pressure.fixture", "fixture"))
        .actor(PhysicalScenarioActor::foreground_reader("reader"))
        .actor(PhysicalScenarioActor::scrub_driver("repair-scan"))
        .fault(scenario_fault(scenario.fault_kind()))
        .schedule(PhysicalScenarioSchedule::named_boundary_yieldpoint(
            "io-pressure-boundary",
        ))
        .expectation(PhysicalScenarioExpectation::s6_io_pressure_simulation())
        .certify_definition()
        .unwrap()
}

fn pressure_trace(
    plan: &PhysicalSimulationPlan,
    observation: Option<crate::S6IoPressureOracleObservation>,
    shortcut: Option<ShortcutRejectionObservation>,
) -> crate::ObservedPhysicalTrace {
    let mut builder = PhysicalSimulationObserver::independent_physical_trace()
        .observe_plan(plan)
        .unwrap()
        .with_runtime_trace(production_trace(plan));
    if let Some(observation) = observation {
        builder = builder.with_s6_io_pressure_observation(observation);
    }
    if let Some(shortcut) = shortcut {
        builder = builder.with_shortcut_rejection_observation(shortcut);
    }
    builder.complete().unwrap()
}

fn pressure_fixture() -> crate::ProductionBackedPhysicalFixture {
    crate::PhysicalFixtureBuilder::production_backed("s6-io-pressure")
        .materialize_with(
            ProductionBackedFixtureMaterialization::build_profile(
                LargeStoreFixtureProfile::ForegroundUnderBackgroundIo,
                11,
            )
            .unwrap(),
        )
        .capability(FixtureCapabilityDeclaration::for_mutation_boundary(
            FixtureMutationBoundary::PageImage,
        ))
        .and_reopen_through_physical_authority()
        .unwrap()
}

fn buffer_pool_evidence(
    plan: &PhysicalSimulationPlan,
    sample: S6PressureExecutionSample,
) -> BufferPoolExecutedEvidenceSource {
    let mut allocation =
        AllocationAdmission::from_declaration(plan.resource_envelope().allocation());
    let grant = allocation
        .admit(
            AllocationRequest::copied_payload(AllocationScope::Foreground, sample.allocation_bytes)
                .unwrap(),
        )
        .unwrap();
    allocation.record_allocation(grant).unwrap();
    BufferPoolExecutedEvidenceSource::from_allocation_execution(&allocation).unwrap()
}

fn io_queue_evidence(
    plan: &PhysicalSimulationPlan,
    sample: S6PressureExecutionSample,
) -> forge_store_io_scheduler::IoQueueExecutedEvidenceSource {
    let mut recorder = IoQueueExecutionRecorder::from_envelope(plan.resource_envelope().io_queue());
    recorder.observe_queue_depth(sample.queue_depth).unwrap();
    for _ in 0..sample.interference_events {
        recorder.record_interference_event().unwrap();
    }
    recorder.executed_evidence().unwrap()
}

fn planning_context(profile: PhysicalSimulationProfile) -> SimulationPlanningContext {
    SimulationPlanningContext::for_profile(profile)
        .with_supported_profiles(PhysicalSimulationProfileSet::all())
        .with_capabilities(PhysicalSimulationCapabilitySet::all_for_developer_smoke())
        .with_driver_contracts(AdmittedDriverContractSet::developer_smoke().unwrap())
        .with_supported_drivers(SupportedPhysicalDriverSet::all_for_developer_smoke())
        .with_supported_observers(SupportedObserverSet::all_for_developer_smoke())
        .with_supported_oracle_families(SupportedOracleFamilySet::all_for_developer_smoke())
        .with_evidence_policy(SimulationEvidencePolicy::minimal_replayable())
        .with_forbidden_shortcuts(ForbiddenShortcutSet::roadmap2_baseline())
}

fn production_trace(plan: &PhysicalSimulationPlan) -> ProductionBoundaryDriverTrace {
    plan.driver_contracts()
        .iter()
        .find_map(PhysicalSimulationDriver::production_boundary_trace)
        .unwrap()
}

pub(crate) fn io_pressure_fault_event(fault_kind: S6IoPressureFaultKind) -> PhysicalFaultEvent {
    PhysicalFaultEvent::s6_io_pressure_stall(
        fault_kind,
        PhysicalArtifactFaultLocus::root_pointer(
            BoundaryArtifactLocator::new(BoundaryArtifactId::new(1), BoundaryArtifactField::Basis),
            ExpectedFaultLocalization::ProductionDriverBoundary,
        ),
    )
    .unwrap()
}

pub(crate) fn mislocalized_io_pressure_fault_event(
    fault_kind: S6IoPressureFaultKind,
) -> PhysicalFaultEvent {
    PhysicalFaultEvent::s6_io_pressure_stall(
        fault_kind,
        PhysicalArtifactFaultLocus::root_pointer(
            BoundaryArtifactLocator::new(BoundaryArtifactId::new(2), BoundaryArtifactField::Basis),
            ExpectedFaultLocalization::PhysicalIntegrityBoundary,
        ),
    )
    .unwrap()
}
