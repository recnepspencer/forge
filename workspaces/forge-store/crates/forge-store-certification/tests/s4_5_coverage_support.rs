#![allow(dead_code)]

#[path = "s4_5_counter_strength/support.rs"]
mod counter_support;

use forge_store_physical_certification::{
    CertifiedPhysicalScenario, CounterContractOracle, CoverageGapDenial,
    DetachedSimulationReplayParts, ExecutedTranscriptParts, FaultDeliveryAttempt,
    FixtureCapabilityDeclaration, FixtureMutationBoundary, LargeStoreFixtureProfile,
    NoJsonAuthorityOracle, NoPrivateMutationOracle, ObservedPhysicalTrace, OracleFamilyKind,
    PhysicalCertificationEvidenceBundle, PhysicalFixtureBuilder, PhysicalInterleavingSchedule,
    PhysicalMutationCoverageEvidence, PhysicalProofOracleVerdict, PhysicalSimulationCapabilitySet,
    PhysicalSimulationPlan, PhysicalSimulationProfile, PhysicalSimulationProfileSet,
    ProductionBackedPhysicalFixture, ReusablePhysicalOracleFamily, Roadmap2CoverageRegistry,
    Roadmap2HarnessSequence, ShortcutRejectionObservationKind, SimulationEvidencePolicy,
    SimulationPlanDenial, SimulationPlanningContext, SimulationReplayBundle, StateSpaceBudget,
    SupportedOracleFamilySet, SupportedPhysicalDriverSet,
};
use forge_store_test_support::{
    admitted_developer_smoke_driver_contracts, developer_smoke_replay_seed,
    production_backed_physical_fixture_materialization,
};

pub(crate) fn lowered_plan() -> PhysicalSimulationPlan {
    counter_support::lower_s5_shortcut_plan_for_profile(PhysicalSimulationProfile::DeveloperSmoke)
}

pub(crate) fn lowered_ci_plan() -> PhysicalSimulationPlan {
    counter_support::lower_s5_shortcut_plan_for_profile(PhysicalSimulationProfile::CiCertification)
}

pub(crate) fn shortcut_plan() -> PhysicalSimulationPlan {
    counter_support::lower_shortcut_plan()
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
    replay_bundle_with_trace(plan, counter_support::shortcut_trace(plan))
}

pub(crate) fn replay_bundle_without_mutation_denial(
    plan: &PhysicalSimulationPlan,
) -> SimulationReplayBundle {
    replay_bundle_with_trace(plan, counter_support::json_shortcut_trace(plan))
}

fn replay_bundle_with_trace(
    plan: &PhysicalSimulationPlan,
    trace: ObservedPhysicalTrace,
) -> SimulationReplayBundle {
    let transcript =
        forge_store_physical_certification::PhysicalSimulationTranscript::from_executed_parts(
            executed_parts_with_trace(plan, trace)
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

pub(crate) fn complete_registry(
    plan: &PhysicalSimulationPlan,
    replay: &SimulationReplayBundle,
) -> Roadmap2CoverageRegistry {
    Roadmap2CoverageRegistry::for_sequence(Roadmap2HarnessSequence::S45)
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
    PhysicalMutationCoverageEvidence::from_replay_private_mutation_denial(
        Roadmap2HarnessSequence::S45,
        replay,
        FaultDeliveryAttempt::private_mutation(),
    )
    .unwrap()
}

pub(crate) fn mutation_evidence_denial(replay: &SimulationReplayBundle) -> CoverageGapDenial {
    PhysicalMutationCoverageEvidence::from_replay_private_mutation_denial(
        Roadmap2HarnessSequence::S45,
        replay,
        FaultDeliveryAttempt::private_mutation(),
    )
    .unwrap_err()
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
    executed_parts_with_trace(plan, trace)
}

fn executed_parts_with_trace(
    plan: &PhysicalSimulationPlan,
    trace: ObservedPhysicalTrace,
) -> ExecutedTranscriptParts {
    let counter_receipt = counter_support::counter_receipt(plan, trace.clone());
    let s5_verdict = if plan
        .oracle_families()
        .contains(OracleFamilyKind::S5ReadinessShape)
    {
        Some(s5_readiness_verdict(plan, &trace))
    } else {
        None
    };
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
        schedule(plan),
        &production_fixture(),
        trace,
        counter_receipt,
    )
    .unwrap();
    if let Some(verdict) = s5_verdict {
        parts = parts.with_oracle_verdict(verdict);
    }
    if let Some(verdict) = shortcut_rejection_verdict {
        parts = parts.with_oracle_verdict(verdict);
    }
    parts
}

fn s5_readiness_verdict(
    plan: &PhysicalSimulationPlan,
    trace: &ObservedPhysicalTrace,
) -> PhysicalProofOracleVerdict {
    ReusablePhysicalOracleFamily::s5_readiness_shape()
        .oracle(CounterContractOracle)
        .judge(plan, trace)
        .unwrap()
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

fn complete_context_for_profile(profile: PhysicalSimulationProfile) -> SimulationPlanningContext {
    SimulationPlanningContext::for_profile(profile)
        .with_supported_profiles(PhysicalSimulationProfileSet::all())
        .with_capabilities(PhysicalSimulationCapabilitySet::s5_readiness_shape_probe())
        .with_driver_contracts(admitted_developer_smoke_driver_contracts().unwrap())
        .with_supported_drivers(SupportedPhysicalDriverSet::all_for_developer_smoke())
        .with_supported_observers(
            forge_store_physical_certification::SupportedObserverSet::all_for_developer_smoke(),
        )
        .with_supported_oracle_families(SupportedOracleFamilySet::all_for_developer_smoke())
        .with_evidence_policy(SimulationEvidencePolicy::minimal_replayable())
        .with_forbidden_shortcuts(
            forge_store_physical_certification::ForbiddenShortcutSet::roadmap2_baseline(),
        )
}
