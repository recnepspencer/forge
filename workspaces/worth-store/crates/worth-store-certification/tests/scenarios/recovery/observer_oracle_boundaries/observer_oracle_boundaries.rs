#[path = "../../../support/recovery/independent_verifier_observation.rs"]
mod independent_verifier_observation;

use worth_store_test_support::harness::recovery::compaction_observation as compaction_interlock_trace;

use worth_store_physical_certification::{
    expected_error_text_oracle_attempt, fixture_label_oracle_attempt, log_only_oracle_attempt,
    lower_physical_simulation_plan, physical_scenario, same_run_self_comparison_oracle_attempt,
    test_support_oracle_verdict_attempt, BlockedReclaimUntilReleaseOracle, ForbiddenShortcutSet,
    IndependentVerifierAgreementOracle, IndependentVerifierObservationKind, NoMixedRootOracle,
    ObservationDenial, ObserverKind, OfflineVerifierBoundarySeam, OracleDenial, OracleFamilyKind,
    PhysicalOracleNonClaim, PhysicalProofOracleKind, PhysicalProofOracleVerdictKind,
    PhysicalScenarioActor, PhysicalScenarioExpectation, PhysicalScenarioIntent,
    PhysicalScenarioSchedule, PhysicalSimulationCapabilitySet, PhysicalSimulationObserver,
    PhysicalSimulationPlan, PhysicalSimulationProfile, PhysicalSimulationProfileSet,
    PhysicalSimulationScenarioFamily, ReusablePhysicalOracleFamily, SimulationEvidencePolicy,
    SimulationPlanningContext, SupportedObserverSet, SupportedOracleFamilySet,
};
use worth_store_test_support::{
    admitted_developer_smoke_driver_contracts, NativeStoreAspectFixture,
};

#[test]
fn observer_collects_facts_and_certification_oracle_judges_verdict() {
    let plan = lower_physical_isolation_plan();
    let trace = PhysicalSimulationObserver::independent_physical_trace()
        .observe_plan(&plan)
        .unwrap()
        .with_runtime_trace(developer_smoke_production_trace())
        .with_compaction_interlock_observation(
            compaction_interlock_trace::store_compaction_observation(),
        )
        .complete()
        .unwrap();

    let verdict = ReusablePhysicalOracleFamily::physical_isolation_readiness_shape()
        .oracle(NoMixedRootOracle)
        .judge(&plan, &trace)
        .unwrap();

    assert_eq!(trace.observer(), ObserverKind::IndependentPhysicalTrace);
    assert_eq!(trace.scenario_identity(), plan.scenario_identity());
    assert_eq!(trace.plan_identity(), plan.identity());
    assert_eq!(
        verdict.family(),
        OracleFamilyKind::PhysicalIsolationReadinessShape
    );
    assert_eq!(verdict.oracle(), PhysicalProofOracleKind::NoMixedRoot);
    assert_eq!(verdict.kind(), PhysicalProofOracleVerdictKind::Satisfied);
    assert!(verdict.basis().runtime_trace_present());
    assert!(!verdict.basis().independent_verifier_present());
    assert_eq!(
        verdict.non_claims(),
        &[PhysicalOracleNonClaim::PhysicalIsolationCorrectness]
    );
}

#[test]
fn independent_verifier_oracle_requires_independent_observation() {
    let plan = lower_physical_isolation_plan();
    let trace = PhysicalSimulationObserver::independent_physical_trace()
        .observe_plan(&plan)
        .unwrap()
        .with_runtime_trace(developer_smoke_production_trace())
        .with_compaction_interlock_observation(
            compaction_interlock_trace::store_compaction_observation(),
        )
        .complete()
        .unwrap();

    let denial = ReusablePhysicalOracleFamily::physical_isolation_readiness_shape()
        .oracle(IndependentVerifierAgreementOracle)
        .judge(&plan, &trace)
        .expect_err("same-run runtime trace alone must not satisfy verifier oracle");

    assert_eq!(denial, OracleDenial::MissingIndependentVerifierObservation);

    let verified_trace = PhysicalSimulationObserver::independent_physical_trace()
        .observe_plan(&plan)
        .unwrap()
        .with_runtime_trace(developer_smoke_production_trace())
        .with_compaction_interlock_observation(
            compaction_interlock_trace::store_compaction_observation(),
        )
        .with_independent_verifier_observation(
            independent_verifier_observation::observed_runtime_comparison(
                independent_verifier_observation::RuntimeComparisonFixture::Equivalent,
            ),
        )
        .complete()
        .unwrap();

    let verdict = ReusablePhysicalOracleFamily::physical_isolation_readiness_shape()
        .oracle(IndependentVerifierAgreementOracle)
        .judge(&plan, &verified_trace)
        .unwrap();

    assert!(verdict.basis().independent_verifier_present());
    assert_eq!(
        verdict
            .basis()
            .independent_verifier()
            .map(|observation| observation.kind()),
        Some(IndependentVerifierObservationKind::Agreement)
    );
    assert_eq!(
        verdict.oracle(),
        PhysicalProofOracleKind::IndependentVerifierAgreement
    );
}

#[test]
fn independent_verifier_disagreement_is_typed_failed_verdict_evidence() {
    let plan = lower_physical_isolation_plan();
    let disputed_trace = PhysicalSimulationObserver::independent_physical_trace()
        .observe_plan(&plan)
        .unwrap()
        .with_runtime_trace(developer_smoke_production_trace())
        .with_compaction_interlock_observation(
            compaction_interlock_trace::store_compaction_observation(),
        )
        .with_independent_verifier_observation(
            independent_verifier_observation::observed_runtime_comparison(
                independent_verifier_observation::RuntimeComparisonFixture::ArtifactDigestMismatch,
            ),
        )
        .complete()
        .unwrap();

    let verdict = ReusablePhysicalOracleFamily::physical_isolation_readiness_shape()
        .oracle(IndependentVerifierAgreementOracle)
        .judge(&plan, &disputed_trace)
        .unwrap();

    let verifier_observation = verdict.basis().independent_verifier().unwrap();
    assert_eq!(verdict.kind(), PhysicalProofOracleVerdictKind::Failed);
    assert_eq!(
        verifier_observation.kind(),
        IndependentVerifierObservationKind::Disagreement
    );
    assert_eq!(
        verifier_observation.seam(),
        OfflineVerifierBoundarySeam::RuntimeVerifierComparison
    );
    assert_eq!(
        verdict.non_claims(),
        &[PhysicalOracleNonClaim::PhysicalIsolationCorrectness]
    );
}

#[test]
fn oracle_family_admission_is_plan_bound_not_fixture_label_bound() {
    let plan = lower_physical_isolation_plan();
    let trace = PhysicalSimulationObserver::independent_physical_trace()
        .observe_plan(&plan)
        .unwrap()
        .with_runtime_trace(developer_smoke_production_trace())
        .with_compaction_interlock_observation(
            compaction_interlock_trace::store_compaction_observation(),
        )
        .complete()
        .unwrap();

    let denial = ReusablePhysicalOracleFamily::recovery_dogfood()
        .oracle(NoMixedRootOracle)
        .judge(&plan, &trace)
        .expect_err("wrong reusable family cannot judge an S5 readiness oracle");

    assert_eq!(
        denial,
        OracleDenial::OracleFamilyNotRequired {
            family: OracleFamilyKind::RecoveryDogfood,
        }
    );
}

#[test]
fn fake_verdict_sources_are_explicitly_denied() {
    assert_eq!(
        test_support_oracle_verdict_attempt().unwrap_err(),
        OracleDenial::TestSupportOracleDenied
    );
    assert_eq!(
        log_only_oracle_attempt().unwrap_err(),
        OracleDenial::LogOnlyEvidenceDenied
    );
    assert_eq!(
        expected_error_text_oracle_attempt().unwrap_err(),
        OracleDenial::ExpectedErrorTextDenied
    );
    assert_eq!(
        same_run_self_comparison_oracle_attempt().unwrap_err(),
        OracleDenial::SameRunSelfComparisonDenied
    );
    assert_eq!(
        fixture_label_oracle_attempt().unwrap_err(),
        OracleDenial::FixtureLabelOracleDenied
    );
}

#[test]
fn physical_isolation_readiness_family_is_reusable_without_claiming_physical_isolation_correctness()
{
    let plan = lower_physical_isolation_plan();
    let trace = PhysicalSimulationObserver::independent_physical_trace()
        .observe_plan(&plan)
        .unwrap()
        .with_runtime_trace(developer_smoke_production_trace())
        .with_compaction_interlock_observation(
            compaction_interlock_trace::store_compaction_observation(),
        )
        .complete()
        .unwrap();

    let family = ReusablePhysicalOracleFamily::physical_isolation_readiness_shape();
    let mixed_root = family
        .oracle(NoMixedRootOracle)
        .judge(&plan, &trace)
        .unwrap();
    let blocked_reclaim = family
        .oracle(BlockedReclaimUntilReleaseOracle)
        .judge(&plan, &trace)
        .unwrap();

    for verdict in [&mixed_root, &blocked_reclaim] {
        assert_eq!(verdict.kind(), PhysicalProofOracleVerdictKind::Satisfied);
        assert_eq!(
            verdict.non_claims(),
            &[PhysicalOracleNonClaim::PhysicalIsolationCorrectness]
        );
    }
}

#[test]
fn observer_cannot_collect_for_unrequired_plan_observer() {
    let denial = PhysicalSimulationObserver::recovery_outcome()
        .observe_plan(&lower_physical_isolation_plan())
        .expect_err("S5 readiness plan does not authorize recovery observer");

    assert_eq!(
        denial,
        ObservationDenial::ObserverNotRequired {
            observer: ObserverKind::RecoveryOutcomeObserver,
        }
    );
}

fn lower_physical_isolation_plan() -> PhysicalSimulationPlan {
    lower_physical_simulation_plan(physical_isolation_scenario(), complete_context()).unwrap()
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
    physical_scenario("store.physical.s45.phase7.observer.oracle")
        .family(PhysicalSimulationScenarioFamily::PhysicalIsolationReadinessShapeProbe)
        .intent(PhysicalScenarioIntent::ProtectBeforeObserveShape)
        .fixture(
            NativeStoreAspectFixture::segment_header("phase7", 7)
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

fn developer_smoke_production_trace(
) -> worth_store_physical_certification::ProductionBoundaryDriverTrace {
    admitted_developer_smoke_driver_contracts()
        .unwrap()
        .iter()
        .find_map(|driver| driver.production_boundary_trace())
        .unwrap()
}
