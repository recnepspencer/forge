#![allow(dead_code)]

#[path = "../compaction_mutation_support/compaction_mutation_support.rs"]
mod compaction_mutation_support;
#[path = "../../../s4_5_coverage_support/context.rs"]
mod context;
#[path = "../counter_strength/support.rs"]
mod counter_support;

use context::complete_context_for_profile;
use forge_store_physical_certification::{
    BlockedReclaimUntilReleaseOracle, CertifiedPhysicalScenario, CounterContractOracle,
    CoverageGapDenial, DetachedSimulationReplayParts, ExecutedTranscriptParts,
    FaultDeliveryAttempt, FixtureCapabilityDeclaration, FixtureMutationBoundary,
    HarnessCoverageStage, LargeStoreFixtureProfile, NoJsonAuthorityOracle, NoMixedRootOracle,
    NoPrivateMutationOracle, ObservedPhysicalTrace, OldReaderSeesOldRootOracle, OracleFamilyKind,
    PhysicalCertificationEvidenceBundle, PhysicalCoverageRegistry, PhysicalFixtureBuilder,
    PhysicalInterleavingSchedule, PhysicalIsolationCompactionMutationKind,
    PhysicalIsolationCompactionMutationObservationSet,
    PhysicalIsolationCompactionMutationReplayBinding,
    PhysicalIsolationCompactionMutationScheduledLaneOutput, PhysicalMutationCoverageEvidence,
    PhysicalProofOracleVerdict, PhysicalSimulationCapabilitySet, PhysicalSimulationPlan,
    PhysicalSimulationProfile, PhysicalSimulationProfileSet, PostSwapReaderSeesNewRootOracle,
    ProductionBackedPhysicalFixture, ReusablePhysicalOracleFamily,
    ShortcutRejectionObservationKind, SimulationEvidencePolicy, SimulationPlanDenial,
    SimulationPlanningContext, SimulationReplayBundle, StateSpaceBudget, SupportedOracleFamilySet,
    SupportedPhysicalDriverSet,
};
use forge_store_test_support::{
    admitted_developer_smoke_driver_contracts, developer_smoke_replay_seed,
    production_backed_physical_fixture_materialization,
};

type ScheduledCompactionMutationLanes = Vec<PhysicalIsolationCompactionMutationScheduledLaneOutput>;

pub(crate) fn lowered_plan() -> PhysicalSimulationPlan {
    forge_store_physical_certification::lower_physical_simulation_plan(
        scenario(),
        complete_context_for_profile(PhysicalSimulationProfile::DeveloperSmoke),
    )
    .unwrap()
}

pub(crate) fn lowered_ci_plan() -> PhysicalSimulationPlan {
    forge_store_physical_certification::lower_physical_simulation_plan(
        scenario(),
        complete_context_for_profile(PhysicalSimulationProfile::CiCertification),
    )
    .unwrap()
}

pub(crate) fn shortcut_plan() -> PhysicalSimulationPlan {
    forge_store_physical_certification::lower_physical_simulation_plan(
        shortcut_scenario(),
        complete_context_for_profile(PhysicalSimulationProfile::DeveloperSmoke),
    )
    .unwrap()
}

pub(crate) fn ci_plan_without_supported_driver(
    driver: forge_store_physical_certification::PhysicalDriverKind,
) -> Result<PhysicalSimulationPlan, SimulationPlanDenial> {
    forge_store_physical_certification::lower_physical_simulation_plan(
        scenario(),
        complete_context_for_profile(PhysicalSimulationProfile::CiCertification)
            .with_driver_contracts(
                admitted_developer_smoke_driver_contracts()
                    .unwrap()
                    .without(driver),
            ),
    )
}

pub(crate) fn ci_plan_without_supported_oracle(
    oracle_family: OracleFamilyKind,
) -> Result<PhysicalSimulationPlan, SimulationPlanDenial> {
    forge_store_physical_certification::lower_physical_simulation_plan(
        scenario(),
        complete_context_for_profile(PhysicalSimulationProfile::CiCertification)
            .with_supported_oracle_families(
                SupportedOracleFamilySet::all_for_developer_smoke().without(oracle_family),
            ),
    )
}

pub(crate) fn shortcut_scenario() -> CertifiedPhysicalScenario {
    counter_support::shortcut_scenario()
}

pub(crate) fn scenario() -> CertifiedPhysicalScenario {
    counter_support::s5_shortcut_scenario()
}

pub(crate) fn replay_bundle(plan: &PhysicalSimulationPlan) -> SimulationReplayBundle {
    replay_bundle_with_trace_builder(plan, shortcut_trace_with_complete_compaction_mutations)
}

pub(crate) fn replay_bundle_without_compaction_mutations(
    plan: &PhysicalSimulationPlan,
) -> SimulationReplayBundle {
    replay_bundle_with_trace_builder(plan, |plan, _schedule| {
        counter_support::shortcut_trace(plan)
    })
}

pub(crate) fn replay_bundle_without_mutation_denial(
    plan: &PhysicalSimulationPlan,
) -> SimulationReplayBundle {
    replay_bundle_with_trace_builder(plan, |plan, _schedule| {
        counter_support::json_shortcut_trace(plan)
    })
}

pub(crate) fn replay_bundle_with_unscheduled_compaction_mutations(
    plan: &PhysicalSimulationPlan,
) -> Result<SimulationReplayBundle, CoverageGapDenial> {
    let other_plan = shortcut_plan();
    let other_schedule = schedule(&other_plan);
    let lanes = compaction_mutation_support::complete_scheduled_compaction_mutation_lanes(
        &other_plan,
        &other_schedule,
    )?;
    let binding = PhysicalIsolationCompactionMutationReplayBinding::from_plan_and_schedule(
        plan,
        &other_schedule,
    )?;
    let observation =
        PhysicalIsolationCompactionMutationObservationSet::from_scheduled_lanes(binding, lanes)?;
    Ok(replay_bundle_with_trace_builder(plan, |plan, _schedule| {
        counter_support::shortcut_trace(plan).with_scheduled_compaction_mutation_lanes(observation)
    }))
}

fn replay_bundle_with_trace_builder(
    plan: &PhysicalSimulationPlan,
    trace_builder: impl FnOnce(
        &PhysicalSimulationPlan,
        &PhysicalInterleavingSchedule,
    ) -> ObservedPhysicalTrace,
) -> SimulationReplayBundle {
    let schedule = schedule(plan);
    let trace = trace_builder(plan, &schedule);
    let transcript =
        forge_store_physical_certification::PhysicalSimulationTranscript::from_executed_parts(
            executed_parts_with_schedule_and_trace(plan, schedule, trace)
                .with_transcript_replay_verdict()
                .unwrap(),
        )
        .unwrap();
    let detached = DetachedSimulationReplayParts::from_transcript(&transcript);
    drop(transcript);
    detached.admit_replay_bundle().unwrap()
}

pub(crate) fn evidence_bundle(
    plan: &PhysicalSimulationPlan,
) -> PhysicalCertificationEvidenceBundle {
    PhysicalCertificationEvidenceBundle::from_replay_bundle(replay_bundle(plan)).unwrap()
}

pub(crate) fn evidence_bundle_without_compaction_mutations(
    plan: &PhysicalSimulationPlan,
) -> PhysicalCertificationEvidenceBundle {
    PhysicalCertificationEvidenceBundle::from_replay_bundle(
        replay_bundle_without_compaction_mutations(plan),
    )
    .unwrap()
}

pub(crate) fn complete_registry(
    plan: &PhysicalSimulationPlan,
    replay: &SimulationReplayBundle,
) -> PhysicalCoverageRegistry {
    PhysicalCoverageRegistry::for_sequence(HarnessCoverageStage::SimulationAdmission)
        .register_scenario(&scenario())
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
        .register_mutation_result(&mutation_evidence(replay))
        .unwrap()
}

pub(crate) fn mutation_evidence(
    replay: &SimulationReplayBundle,
) -> PhysicalMutationCoverageEvidence {
    PhysicalMutationCoverageEvidence::from_replay_private_and_compaction_mutation_denials(
        HarnessCoverageStage::SimulationAdmission,
        replay,
        FaultDeliveryAttempt::private_mutation(),
    )
    .unwrap()
}

pub(crate) fn mutation_evidence_denial(replay: &SimulationReplayBundle) -> CoverageGapDenial {
    PhysicalMutationCoverageEvidence::from_replay_private_mutation_denial(
        HarnessCoverageStage::SimulationAdmission,
        replay,
        FaultDeliveryAttempt::private_mutation(),
    )
    .unwrap_err()
}

pub(crate) fn compaction_mutation_evidence(
    replay: &SimulationReplayBundle,
) -> Result<PhysicalMutationCoverageEvidence, CoverageGapDenial> {
    PhysicalMutationCoverageEvidence::from_replay_private_and_compaction_mutation_denials(
        HarnessCoverageStage::SimulationAdmission,
        replay,
        FaultDeliveryAttempt::private_mutation(),
    )
}

pub(crate) fn complete_compaction_mutation_lanes(
    plan: &PhysicalSimulationPlan,
    schedule: &PhysicalInterleavingSchedule,
) -> Result<ScheduledCompactionMutationLanes, CoverageGapDenial> {
    compaction_mutation_support::complete_scheduled_compaction_mutation_lanes(plan, schedule)
}

pub(crate) fn compaction_mutation_lanes_without(
    plan: &PhysicalSimulationPlan,
    schedule: &PhysicalInterleavingSchedule,
    missing: PhysicalIsolationCompactionMutationKind,
) -> Result<ScheduledCompactionMutationLanes, CoverageGapDenial> {
    Ok(complete_compaction_mutation_lanes(plan, schedule)?
        .into_iter()
        .filter(|lane| lane.kind() != missing)
        .collect())
}

pub(crate) fn same_footprint_wrong_cutover_lanes(
    plan: &PhysicalSimulationPlan,
    schedule: &PhysicalInterleavingSchedule,
) -> Result<ScheduledCompactionMutationLanes, CoverageGapDenial> {
    compaction_mutation_support::same_footprint_wrong_cutover_lanes(plan, schedule)
}

pub(crate) fn detached_compaction_mutation_lanes(
    plan: &PhysicalSimulationPlan,
    schedule: &PhysicalInterleavingSchedule,
) -> Result<ScheduledCompactionMutationLanes, CoverageGapDenial> {
    compaction_mutation_support::detached_compaction_mutation_lanes(plan, schedule)
}

pub(crate) fn compaction_mutation_lane_observation_set(
    plan: &PhysicalSimulationPlan,
    schedule: &PhysicalInterleavingSchedule,
    lanes: impl IntoIterator<Item = PhysicalIsolationCompactionMutationScheduledLaneOutput>,
) -> Result<PhysicalIsolationCompactionMutationObservationSet, CoverageGapDenial> {
    let binding =
        PhysicalIsolationCompactionMutationReplayBinding::from_plan_and_schedule(plan, schedule)?;
    PhysicalIsolationCompactionMutationObservationSet::from_scheduled_lanes(binding, lanes)
}

pub(crate) fn schedule(plan: &PhysicalSimulationPlan) -> PhysicalInterleavingSchedule {
    PhysicalInterleavingSchedule::from_lowered_plan(
        plan,
        developer_smoke_replay_seed(),
        StateSpaceBudget::bounded_steps(8).unwrap(),
    )
    .unwrap()
}

fn executed_parts(plan: &PhysicalSimulationPlan) -> ExecutedTranscriptParts {
    let trace = counter_support::observed_trace(plan);
    executed_parts_with_schedule_and_trace(plan, schedule(plan), trace)
}

fn shortcut_trace_with_complete_compaction_mutations(
    plan: &PhysicalSimulationPlan,
    schedule: &PhysicalInterleavingSchedule,
) -> ObservedPhysicalTrace {
    let Ok(binding) =
        PhysicalIsolationCompactionMutationReplayBinding::from_plan_and_schedule(plan, schedule)
    else {
        return counter_support::shortcut_trace(plan);
    };
    let lanes = complete_compaction_mutation_lanes(plan, schedule).unwrap();
    counter_support::shortcut_trace(plan).with_scheduled_compaction_mutation_lanes(
        PhysicalIsolationCompactionMutationObservationSet::from_scheduled_lanes(binding, lanes)
            .unwrap(),
    )
}

fn executed_parts_with_schedule_and_trace(
    plan: &PhysicalSimulationPlan,
    schedule: PhysicalInterleavingSchedule,
    trace: ObservedPhysicalTrace,
) -> ExecutedTranscriptParts {
    let counter_receipt = counter_support::counter_receipt(plan, trace.clone());
    let s5_verdicts = physical_isolation_readiness_verdicts(plan, &trace);
    let shortcut_rejection_verdict = if plan
        .oracle_families()
        .contains(OracleFamilyKind::ForbiddenShortcutRejection)
    {
        Some(shortcut_rejection_verdict(plan, &trace))
    } else {
        None
    };
    let mut parts = ExecutedTranscriptParts::new(
        plan,
        schedule,
        &production_fixture(),
        trace,
        counter_receipt,
    )
    .unwrap();
    for verdict in s5_verdicts {
        parts = parts.with_oracle_verdict(verdict);
    }
    if let Some(verdict) = shortcut_rejection_verdict {
        parts = parts.with_oracle_verdict(verdict);
    }
    parts
}

fn physical_isolation_readiness_verdicts(
    plan: &PhysicalSimulationPlan,
    trace: &ObservedPhysicalTrace,
) -> Vec<PhysicalProofOracleVerdict> {
    if !plan
        .oracle_families()
        .contains(OracleFamilyKind::PhysicalIsolationReadinessShape)
    {
        return Vec::new();
    }
    let family = ReusablePhysicalOracleFamily::physical_isolation_readiness_shape();
    vec![
        family
            .oracle(CounterContractOracle)
            .judge(plan, trace)
            .unwrap(),
        family.oracle(NoMixedRootOracle).judge(plan, trace).unwrap(),
        family
            .oracle(OldReaderSeesOldRootOracle)
            .judge(plan, trace)
            .unwrap(),
        family
            .oracle(PostSwapReaderSeesNewRootOracle)
            .judge(plan, trace)
            .unwrap(),
        family
            .oracle(BlockedReclaimUntilReleaseOracle)
            .judge(plan, trace)
            .unwrap(),
    ]
}

fn shortcut_rejection_verdict(
    plan: &PhysicalSimulationPlan,
    trace: &ObservedPhysicalTrace,
) -> PhysicalProofOracleVerdict {
    if trace.shortcut_rejections().iter().any(|observation| {
        observation.kind() == ShortcutRejectionObservationKind::PrivateMutationDenied
    }) {
        return ReusablePhysicalOracleFamily::forbidden_shortcut_rejection()
            .oracle(NoPrivateMutationOracle)
            .judge(plan, trace)
            .unwrap();
    }
    ReusablePhysicalOracleFamily::forbidden_shortcut_rejection()
        .oracle(NoJsonAuthorityOracle)
        .judge(plan, trace)
        .unwrap()
}

fn production_fixture() -> ProductionBackedPhysicalFixture {
    PhysicalFixtureBuilder::production_backed("phase11-coverage-fixture")
        .materialize_with(
            production_backed_physical_fixture_materialization(
                LargeStoreFixtureProfile::StoreLargerThanMemory,
                10,
            )
            .unwrap(),
        )
        .capability(FixtureCapabilityDeclaration::for_mutation_boundary(
            FixtureMutationBoundary::Manifest,
        ))
        .and_reopen_through_physical_authority()
        .unwrap()
}
